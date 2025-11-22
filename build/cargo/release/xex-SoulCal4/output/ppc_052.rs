pub fn sub_8242FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242FAF0 size=136
    let mut pc: u32 = 0x8242FAF0;
    'dispatch: loop {
        match pc {
            0x8242FAF0 => {
    //   block [0x8242FAF0..0x8242FB78)
	// 8242FAF0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8242FAF4: 3484FFFD  addic. r4, r4, -3
	ctx.xer.ca = (ctx.r[4].u32 > (!(-3 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -3;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8242FAF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8242FAFC: 4081004C  ble 0x8242fb48
	if !ctx.cr[0].gt {
	pc = 0x8242FB48; continue 'dispatch;
	}
	// 8242FB00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242FB04: 38CBC79C  addi r6, r11, -0x3864
	ctx.r[6].s64 = ctx.r[11].s64 + -14436;
	// 8242FB08: 7D691A14  add r11, r9, r3
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[3].u64;
	// 8242FB0C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8242FB10: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 8242FB14: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FB18: 8BEA0000  lbz r31, 0(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FB1C: 7CFF3851  subf. r7, r31, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[31].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8242FB20: 40820014  bne 0x8242fb34
	if !ctx.cr[0].eq {
	pc = 0x8242FB34; continue 'dispatch;
	}
	// 8242FB24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242FB28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8242FB2C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8242FB30: 409AFFE4  bne cr6, 0x8242fb14
	if !ctx.cr[6].eq {
	pc = 0x8242FB14; continue 'dispatch;
	}
	// 8242FB34: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242FB38: 41820024  beq 0x8242fb5c
	if ctx.cr[0].eq {
	pc = 0x8242FB5C; continue 'dispatch;
	}
	// 8242FB3C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8242FB40: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8242FB44: 4198FFC4  blt cr6, 0x8242fb08
	if ctx.cr[6].lt {
	pc = 0x8242FB08; continue 'dispatch;
	}
	// 8242FB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242FB4C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242FB50: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8242FB54: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8242FB58: 4E800020  blr
	return;
	// 8242FB5C: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 8242FB60: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8242FB64: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8242FB68: 4098FFE0  bge cr6, 0x8242fb48
	if !ctx.cr[6].lt {
	pc = 0x8242FB48; continue 'dispatch;
	}
	// 8242FB6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242FB70: B1250000  sth r9, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8242FB74: 4BFFFFE0  b 0x8242fb54
	pc = 0x8242FB54; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242FB78 size=156
    let mut pc: u32 = 0x8242FB78;
    'dispatch: loop {
        match pc {
            0x8242FB78 => {
    //   block [0x8242FB78..0x8242FC14)
	// 8242FB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242FB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242FB80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8242FB84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8242FB88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242FB8C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8242FB90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8242FB94: 419A0058  beq cr6, 0x8242fbec
	if ctx.cr[6].eq {
	pc = 0x8242FBEC; continue 'dispatch;
	}
	// 8242FB98: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8242FB9C: 419A0050  beq cr6, 0x8242fbec
	if ctx.cr[6].eq {
	pc = 0x8242FBEC; continue 'dispatch;
	}
	// 8242FBA0: 2B040118  cmplwi cr6, r4, 0x118
	ctx.cr[6].compare_u32(ctx.r[4].u32, 280 as u32, &mut ctx.xer);
	// 8242FBA4: 41980030  blt cr6, 0x8242fbd4
	if ctx.cr[6].lt {
	pc = 0x8242FBD4; continue 'dispatch;
	}
	// 8242FBA8: 39630003  addi r11, r3, 3
	ctx.r[11].s64 = ctx.r[3].s64 + 3;
	// 8242FBAC: 38A00118  li r5, 0x118
	ctx.r[5].s64 = 280;
	// 8242FBB0: 557F003A  rlwinm r31, r11, 0, 0, 0x1d
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8242FBB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242FBB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242FBBC: 48105615  bl 0x825351d0
	ctx.lr = 0x8242FBC0;
	sub_825351D0(ctx, base);
	// 8242FBC0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8242FBC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242FBC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242FBCC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8242FBD0: 4800002C  b 0x8242fbfc
	pc = 0x8242FBFC; continue 'dispatch;
	// 8242FBD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242FBD8: 386B3E70  addi r3, r11, 0x3e70
	ctx.r[3].s64 = ctx.r[11].s64 + 15984;
	// 8242FBDC: 4BFED02D  bl 0x8241cc08
	ctx.lr = 0x8242FBE0;
	sub_8241CC08(ctx, base);
	// 8242FBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8242FBE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242FBE8: 48000010  b 0x8242fbf8
	pc = 0x8242FBF8; continue 'dispatch;
	// 8242FBEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242FBF0: 386B3E48  addi r3, r11, 0x3e48
	ctx.r[3].s64 = ctx.r[11].s64 + 15944;
	// 8242FBF4: 4BFED015  bl 0x8241cc08
	ctx.lr = 0x8242FBF8;
	sub_8241CC08(ctx, base);
	// 8242FBF8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242FBFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8242FC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242FC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242FC08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8242FC0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8242FC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242FC18 size=44
    let mut pc: u32 = 0x8242FC18;
    'dispatch: loop {
        match pc {
            0x8242FC18 => {
    //   block [0x8242FC18..0x8242FC44)
	// 8242FC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242FC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242FC20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242FC24: 38A00118  li r5, 0x118
	ctx.r[5].s64 = 280;
	// 8242FC28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242FC2C: 481055A5  bl 0x825351d0
	ctx.lr = 0x8242FC30;
	sub_825351D0(ctx, base);
	// 8242FC30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242FC34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242FC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242FC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242FC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242FC48 size=68
    let mut pc: u32 = 0x8242FC48;
    'dispatch: loop {
        match pc {
            0x8242FC48 => {
    //   block [0x8242FC48..0x8242FC8C)
	// 8242FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242FC50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242FC54: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242FC58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242FC5C: 409A0018  bne cr6, 0x8242fc74
	if !ctx.cr[6].eq {
	pc = 0x8242FC74; continue 'dispatch;
	}
	// 8242FC60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242FC64: 386B3E98  addi r3, r11, 0x3e98
	ctx.r[3].s64 = ctx.r[11].s64 + 16024;
	// 8242FC68: 4BFECFA1  bl 0x8241cc08
	ctx.lr = 0x8242FC6C;
	sub_8241CC08(ctx, base);
	// 8242FC6C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242FC70: 4800000C  b 0x8242fc7c
	pc = 0x8242FC7C; continue 'dispatch;
	// 8242FC74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242FC78: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8242FC7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242FC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242FC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242FC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242FC90 size=72
    let mut pc: u32 = 0x8242FC90;
    'dispatch: loop {
        match pc {
            0x8242FC90 => {
    //   block [0x8242FC90..0x8242FCD8)
	// 8242FC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242FC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8242FC98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242FC9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8242FCA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242FCA4: 409A0018  bne cr6, 0x8242fcbc
	if !ctx.cr[6].eq {
	pc = 0x8242FCBC; continue 'dispatch;
	}
	// 8242FCA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242FCAC: 386B3EC0  addi r3, r11, 0x3ec0
	ctx.r[3].s64 = ctx.r[11].s64 + 16064;
	// 8242FCB0: 4BFECF59  bl 0x8241cc08
	ctx.lr = 0x8242FCB4;
	sub_8241CC08(ctx, base);
	// 8242FCB4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242FCB8: 48000010  b 0x8242fcc8
	pc = 0x8242FCC8; continue 'dispatch;
	// 8242FCBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242FCC0: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8242FCC4: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 8242FCC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8242FCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8242FCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8242FCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242FCD8 size=448
    let mut pc: u32 = 0x8242FCD8;
    'dispatch: loop {
        match pc {
            0x8242FCD8 => {
    //   block [0x8242FCD8..0x8242FE98)
	// 8242FCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242FCDC: 481053C1  bl 0x8253509c
	ctx.lr = 0x8242FCE0;
	sub_82535080(ctx, base);
	// 8242FCE0: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242FCE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242FCE8: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 8242FCEC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8242FCF0: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 8242FCF4: 839F0110  lwz r28, 0x110(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 8242FCF8: 2F1C0102  cmpwi cr6, r28, 0x102
	ctx.cr[6].compare_i32(ctx.r[28].s32, 258, &mut ctx.xer);
	// 8242FCFC: 41980018  blt cr6, 0x8242fd14
	if ctx.cr[6].lt {
	pc = 0x8242FD14; continue 'dispatch;
	}
	// 8242FD00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242FD04: 386B3EEC  addi r3, r11, 0x3eec
	ctx.r[3].s64 = ctx.r[11].s64 + 16108;
	// 8242FD08: 4BFECF01  bl 0x8241cc08
	ctx.lr = 0x8242FD0C;
	sub_8241CC08(ctx, base);
	// 8242FD0C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242FD10: 48000180  b 0x8242fe90
	pc = 0x8242FE90; continue 'dispatch;
	// 8242FD14: 7FDCEA14  add r30, r28, r29
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 8242FD18: 3AE0005C  li r23, 0x5c
	ctx.r[23].s64 = 92;
	// 8242FD1C: 3B00002A  li r24, 0x2a
	ctx.r[24].s64 = 42;
	// 8242FD20: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8242FD24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242FD28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8242FD2C: 7EFCE9AE  stbx r23, r28, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32), ctx.r[23].u8) };
	// 8242FD30: 9B1E0001  stb r24, 1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 8242FD34: 9B5E0002  stb r26, 2(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[26].u8 ) };
	// 8242FD38: 4BF9B7D9  bl 0x823cb510
	ctx.lr = 0x8242FD3C;
	sub_823CB510(ctx, base);
	// 8242FD3C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8242FD40: 2F16FFFF  cmpwi cr6, r22, -1
	ctx.cr[6].compare_i32(ctx.r[22].s32, -1, &mut ctx.xer);
	// 8242FD44: 419A0148  beq cr6, 0x8242fe8c
	if ctx.cr[6].eq {
	pc = 0x8242FE8C; continue 'dispatch;
	}
	// 8242FD48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242FD4C: 3B2B3EE8  addi r25, r11, 0x3ee8
	ctx.r[25].s64 = ctx.r[11].s64 + 16104;
	// 8242FD50: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8242FD54: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242FD58: 418200BC  beq 0x8242fe14
	if ctx.cr[0].eq {
	pc = 0x8242FE14; continue 'dispatch;
	}
	// 8242FD5C: 8961007C  lbz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8242FD60: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 8242FD64: 419A010C  beq cr6, 0x8242fe70
	if ctx.cr[6].eq {
	pc = 0x8242FE70; continue 'dispatch;
	}
	// 8242FD68: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 8242FD6C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8242FD70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242FD74: 48117235  bl 0x82546fa8
	ctx.lr = 0x8242FD78;
	sub_82546FA8(ctx, base);
	// 8242FD78: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8242FD7C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242FD80: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FD84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242FD88: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8242FD8C: 409AFFF4  bne cr6, 0x8242fd80
	if !ctx.cr[6].eq {
	pc = 0x8242FD80; continue 'dispatch;
	}
	// 8242FD90: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242FD94: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FD98: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8242FD9C: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 8242FDA0: 554A07BD  rlwinm. r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8242FDA4: 4182003C  beq 0x8242fde0
	if ctx.cr[0].eq {
	pc = 0x8242FDE0; continue 'dispatch;
	}
	// 8242FDA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242FDAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8242FDB0: 41820030  beq 0x8242fde0
	if ctx.cr[0].eq {
	pc = 0x8242FDE0; continue 'dispatch;
	}
	// 8242FDB4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242FDB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8242FDBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242FDC0: 4E800421  bctrl
	ctx.lr = 0x8242FDC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242FDC4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242FDC8: 418000C8  blt 0x8242fe90
	if ctx.cr[0].lt {
	pc = 0x8242FE90; continue 'dispatch;
	}
	// 8242FDCC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8242FDD0: 419A0010  beq cr6, 0x8242fde0
	if ctx.cr[6].eq {
	pc = 0x8242FDE0; continue 'dispatch;
	}
	// 8242FDD4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FDD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242FDDC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242FDE0: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8242FDE4: 419A001C  beq cr6, 0x8242fe00
	if ctx.cr[6].eq {
	pc = 0x8242FE00; continue 'dispatch;
	}
	// 8242FDE8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8242FDEC: 3895FFFF  addi r4, r21, -1
	ctx.r[4].s64 = ctx.r[21].s64 + -1;
	// 8242FDF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242FDF4: 4BFFFEE5  bl 0x8242fcd8
	ctx.lr = 0x8242FDF8;
	sub_8242FCD8(ctx, base);
	// 8242FDF8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242FDFC: 41800094  blt 0x8242fe90
	if ctx.cr[0].lt {
	pc = 0x8242FE90; continue 'dispatch;
	}
	// 8242FE00: 9AFE0000  stb r23, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[23].u8 ) };
	// 8242FE04: 9B1E0001  stb r24, 1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 8242FE08: 9B5E0002  stb r26, 2(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[26].u8 ) };
	// 8242FE0C: 939F0110  stw r28, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[28].u32 ) };
	// 8242FE10: 48000060  b 0x8242fe70
	pc = 0x8242FE70; continue 'dispatch;
	// 8242FE14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FE18: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8242FE1C: 41820054  beq 0x8242fe70
	if ctx.cr[0].eq {
	pc = 0x8242FE70; continue 'dispatch;
	}
	// 8242FE20: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242FE24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8242FE28: 419A0048  beq cr6, 0x8242fe70
	if ctx.cr[6].eq {
	pc = 0x8242FE70; continue 'dispatch;
	}
	// 8242FE2C: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 8242FE30: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8242FE34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242FE38: 48117171  bl 0x82546fa8
	ctx.lr = 0x8242FE3C;
	sub_82546FA8(ctx, base);
	// 8242FE3C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8242FE40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8242FE44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8242FE48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8242FE4C: 4E800421  bctrl
	ctx.lr = 0x8242FE50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8242FE50: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242FE54: 4180003C  blt 0x8242fe90
	if ctx.cr[0].lt {
	pc = 0x8242FE90; continue 'dispatch;
	}
	// 8242FE58: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8242FE5C: 9B5E0000  stb r26, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8242FE60: 419A0010  beq cr6, 0x8242fe70
	if ctx.cr[6].eq {
	pc = 0x8242FE70; continue 'dispatch;
	}
	// 8242FE64: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FE68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242FE6C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242FE70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8242FE74: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8242FE78: 4BF9B641  bl 0x823cb4b8
	ctx.lr = 0x8242FE7C;
	sub_823CB4B8(ctx, base);
	// 8242FE7C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8242FE80: 4082FED0  bne 0x8242fd50
	if !ctx.cr[0].eq {
	pc = 0x8242FD50; continue 'dispatch;
	}
	// 8242FE84: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8242FE88: 4BF90AD1  bl 0x823c0958
	ctx.lr = 0x8242FE8C;
	sub_823C0958(ctx, base);
	// 8242FE8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242FE90: 382101F0  addi r1, r1, 0x1f0
	ctx.r[1].s64 = ctx.r[1].s64 + 496;
	// 8242FE94: 48105258  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242FE98 size=184
    let mut pc: u32 = 0x8242FE98;
    'dispatch: loop {
        match pc {
            0x8242FE98 => {
    //   block [0x8242FE98..0x8242FF50)
	// 8242FE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242FE9C: 48105219  bl 0x825350b4
	ctx.lr = 0x8242FEA0;
	sub_82535080(ctx, base);
	// 8242FEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242FEA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8242FEA8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8242FEAC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8242FEB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8242FEB4: 419A0084  beq cr6, 0x8242ff38
	if ctx.cr[6].eq {
	pc = 0x8242FF38; continue 'dispatch;
	}
	// 8242FEB8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8242FEBC: 419A007C  beq cr6, 0x8242ff38
	if ctx.cr[6].eq {
	pc = 0x8242FF38; continue 'dispatch;
	}
	// 8242FEC0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8242FEC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8242FEC8: 419A0008  beq cr6, 0x8242fed0
	if ctx.cr[6].eq {
	pc = 0x8242FED0; continue 'dispatch;
	}
	// 8242FECC: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8242FED0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 8242FED4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8242FED8: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8242FEDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8242FEE0: 4BFF3649  bl 0x82423528
	ctx.lr = 0x8242FEE4;
	sub_82423528(ctx, base);
	// 8242FEE4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8242FEE8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242FEEC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FEF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8242FEF4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8242FEF8: 409AFFF4  bne cr6, 0x8242feec
	if !ctx.cr[6].eq {
	pc = 0x8242FEEC; continue 'dispatch;
	}
	// 8242FEFC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8242FF00: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8242FF04: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8242FF08: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8242FF0C: 892A000B  lbz r9, 0xb(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(11 as u32) ) } as u64;
	// 8242FF10: 2B09005C  cmplwi cr6, r9, 0x5c
	ctx.cr[6].compare_u32(ctx.r[9].u32, 92 as u32, &mut ctx.xer);
	// 8242FF14: 409A000C  bne cr6, 0x8242ff20
	if !ctx.cr[6].eq {
	pc = 0x8242FF20; continue 'dispatch;
	}
	// 8242FF18: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8242FF1C: 9B8A000B  stb r28, 0xb(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(11 as u32), ctx.r[28].u8 ) };
	// 8242FF20: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8242FF24: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 8242FF28: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8242FF2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242FF30: 4BFFFDA9  bl 0x8242fcd8
	ctx.lr = 0x8242FF34;
	sub_8242FCD8(ctx, base);
	// 8242FF34: 48000014  b 0x8242ff48
	pc = 0x8242FF48; continue 'dispatch;
	// 8242FF38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8242FF3C: 386B3F14  addi r3, r11, 0x3f14
	ctx.r[3].s64 = ctx.r[11].s64 + 16148;
	// 8242FF40: 4BFECCC9  bl 0x8241cc08
	ctx.lr = 0x8242FF44;
	sub_8241CC08(ctx, base);
	// 8242FF44: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8242FF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8242FF4C: 481051B8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8242FF50 size=20
    let mut pc: u32 = 0x8242FF50;
    'dispatch: loop {
        match pc {
            0x8242FF50 => {
    //   block [0x8242FF50..0x8242FF64)
	// 8242FF50: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242FF54: 38A00780  li r5, 0x780
	ctx.r[5].s64 = 1920;
	// 8242FF58: 386B45A0  addi r3, r11, 0x45a0
	ctx.r[3].s64 = ctx.r[11].s64 + 17824;
	// 8242FF5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242FF60: 48105270  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8242FF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8242FF68 size=168
    let mut pc: u32 = 0x8242FF68;
    'dispatch: loop {
        match pc {
            0x8242FF68 => {
    //   block [0x8242FF68..0x82430010)
	// 8242FF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8242FF6C: 48105151  bl 0x825350bc
	ctx.lr = 0x8242FF70;
	sub_82535080(ctx, base);
	// 8242FF70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8242FF74: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8242FF78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8242FF7C: 396B45A0  addi r11, r11, 0x45a0
	ctx.r[11].s64 = ctx.r[11].s64 + 17824;
	// 8242FF80: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8242FF84: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8242FF88: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8242FF8C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8242FF90: 419A0018  beq cr6, 0x8242ffa8
	if ctx.cr[6].eq {
	pc = 0x8242FFA8; continue 'dispatch;
	}
	// 8242FF94: 394A003C  addi r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 + 60;
	// 8242FF98: 392B0780  addi r9, r11, 0x780
	ctx.r[9].s64 = ctx.r[11].s64 + 1920;
	// 8242FF9C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8242FFA0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8242FFA4: 4198FFE4  blt cr6, 0x8242ff88
	if ctx.cr[6].lt {
	pc = 0x8242FF88; continue 'dispatch;
	}
	// 8242FFA8: 2F1E0020  cmpwi cr6, r30, 0x20
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32, &mut ctx.xer);
	// 8242FFAC: 409A000C  bne cr6, 0x8242ffb8
	if !ctx.cr[6].eq {
	pc = 0x8242FFB8; continue 'dispatch;
	}
	// 8242FFB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8242FFB4: 48000054  b 0x82430008
	pc = 0x82430008; continue 'dispatch;
	// 8242FFB8: 1D5E003C  mulli r10, r30, 0x3c
	ctx.r[10].s64 = ctx.r[30].s64 * 60;
	// 8242FFBC: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8242FFC0: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 8242FFC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8242FFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8242FFCC: 48105205  bl 0x825351d0
	ctx.lr = 0x8242FFD0;
	sub_825351D0(ctx, base);
	// 8242FFD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8242FFD4: 3C800000  lis r4, 0
	ctx.r[4].s64 = 0;
	// 8242FFD8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8242FFDC: 38DF0032  addi r6, r31, 0x32
	ctx.r[6].s64 = ctx.r[31].s64 + 50;
	// 8242FFE0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8242FFE4: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 8242FFE8: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8242FFEC: 6084AC44  ori r4, r4, 0xac44
	ctx.r[4].u64 = ctx.r[4].u64 | 44100;
	// 8242FFF0: 386001F4  li r3, 0x1f4
	ctx.r[3].s64 = 500;
	// 8242FFF4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8242FFF8: 4BFF7BB1  bl 0x82427ba8
	ctx.lr = 0x8242FFFC;
	sub_82427BA8(ctx, base);
	// 8242FFFC: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82430000: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82430004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82430008: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243000C: 48105100  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430010 size=20
    let mut pc: u32 = 0x82430010;
    'dispatch: loop {
        match pc {
            0x82430010 => {
    //   block [0x82430010..0x82430024)
	// 82430010: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82430014: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82430018: 38CB0032  addi r6, r11, 0x32
	ctx.r[6].s64 = ctx.r[11].s64 + 50;
	// 8243001C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82430020: 4BFF7B88  b 0x82427ba8
	sub_82427BA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430028 size=36
    let mut pc: u32 = 0x82430028;
    'dispatch: loop {
        match pc {
            0x82430028 => {
    //   block [0x82430028..0x8243004C)
	// 82430028: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243002C: B1630028  sth r11, 0x28(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u16 ) };
	// 82430030: A1650000  lhz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430034: B163002A  sth r11, 0x2a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(42 as u32), ctx.r[11].u16 ) };
	// 82430038: A1640002  lhz r11, 2(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 8243003C: B163002C  sth r11, 0x2c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u16 ) };
	// 82430040: A1650002  lhz r11, 2(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430044: B163002E  sth r11, 0x2e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(46 as u32), ctx.r[11].u16 ) };
	// 82430048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430050 size=36
    let mut pc: u32 = 0x82430050;
    'dispatch: loop {
        match pc {
            0x82430050 => {
    //   block [0x82430050..0x82430074)
	// 82430050: A1630028  lhz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82430054: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82430058: A163002A  lhz r11, 0x2a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(42 as u32) ) } as u64;
	// 8243005C: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82430060: A163002C  lhz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82430064: B1640002  sth r11, 2(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82430068: A163002E  lhz r11, 0x2e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(46 as u32) ) } as u64;
	// 8243006C: B1650002  sth r11, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82430070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430078 size=16
    let mut pc: u32 = 0x82430078;
    'dispatch: loop {
        match pc {
            0x82430078 => {
    //   block [0x82430078..0x82430088)
	// 82430078: B0830034  sth r4, 0x34(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[4].u16 ) };
	// 8243007C: B0A30036  sth r5, 0x36(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(54 as u32), ctx.r[5].u16 ) };
	// 82430080: B0C30038  sth r6, 0x38(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[6].u16 ) };
	// 82430084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430088 size=28
    let mut pc: u32 = 0x82430088;
    'dispatch: loop {
        match pc {
            0x82430088 => {
    //   block [0x82430088..0x824300A4)
	// 82430088: A1630034  lhz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8243008C: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82430090: A1630036  lhz r11, 0x36(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(54 as u32) ) } as u64;
	// 82430094: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82430098: A1630038  lhz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8243009C: B1660000  sth r11, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824300A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824300A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824300A8 size=8
    let mut pc: u32 = 0x824300A8;
    'dispatch: loop {
        match pc {
            0x824300A8 => {
    //   block [0x824300A8..0x824300B0)
	// 824300A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824300AC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824300B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824300B0 size=20
    let mut pc: u32 = 0x824300B0;
    'dispatch: loop {
        match pc {
            0x824300B0 => {
    //   block [0x824300B0..0x824300C4)
	// 824300B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824300B4: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 824300B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824300BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824300C0: 48105110  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824300C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824300C4 size=4
    let mut pc: u32 = 0x824300C4;
    'dispatch: loop {
        match pc {
            0x824300C4 => {
    //   block [0x824300C4..0x824300C8)
	// 824300C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824300C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824300C8 size=8
    let mut pc: u32 = 0x824300C8;
    'dispatch: loop {
        match pc {
            0x824300C8 => {
    //   block [0x824300C8..0x824300D0)
	// 824300C8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824300CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824300D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824300D0 size=48
    let mut pc: u32 = 0x824300D0;
    'dispatch: loop {
        match pc {
            0x824300D0 => {
    //   block [0x824300D0..0x82430100)
	// 824300D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824300D4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824300D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824300DC: 409A0024  bne cr6, 0x82430100
	if !ctx.cr[6].eq {
		sub_82430100(ctx, base);
		return;
	}
	// 824300E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824300E4: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 824300E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824300EC: 90AB001C  stw r5, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 824300F0: 90CB0020  stw r6, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 824300F4: 90EB0024  stw r7, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 824300F8: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824300FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430100 size=8
    let mut pc: u32 = 0x82430100;
    'dispatch: loop {
        match pc {
            0x82430100 => {
    //   block [0x82430100..0x82430108)
	// 82430100: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82430104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430108 size=48
    let mut pc: u32 = 0x82430108;
    'dispatch: loop {
        match pc {
            0x82430108 => {
    //   block [0x82430108..0x82430138)
	// 82430108: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243010C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82430110: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82430114: 409A0024  bne cr6, 0x82430138
	if !ctx.cr[6].eq {
		sub_82430138(ctx, base);
		return;
	}
	// 82430118: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8243011C: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82430120: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82430124: 90AB001C  stw r5, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82430128: 90CB0020  stw r6, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 8243012C: 90EB0024  stw r7, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82430130: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82430134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430138 size=8
    let mut pc: u32 = 0x82430138;
    'dispatch: loop {
        match pc {
            0x82430138 => {
    //   block [0x82430138..0x82430140)
	// 82430138: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243013C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430140 size=12
    let mut pc: u32 = 0x82430140;
    'dispatch: loop {
        match pc {
            0x82430140 => {
    //   block [0x82430140..0x8243014C)
	// 82430140: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82430144: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82430148: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243014C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243014C size=16
    let mut pc: u32 = 0x8243014C;
    'dispatch: loop {
        match pc {
            0x8243014C => {
    //   block [0x8243014C..0x8243015C)
	// 8243014C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82430150: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82430154: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82430158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430160 size=20
    let mut pc: u32 = 0x82430160;
    'dispatch: loop {
        match pc {
            0x82430160 => {
    //   block [0x82430160..0x82430174)
	// 82430160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82430164: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82430168: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8243016C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82430170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430178 size=12
    let mut pc: u32 = 0x82430178;
    'dispatch: loop {
        match pc {
            0x82430178 => {
    //   block [0x82430178..0x82430184)
	// 82430178: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243017C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82430180: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430184(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430184 size=12
    let mut pc: u32 = 0x82430184;
    'dispatch: loop {
        match pc {
            0x82430184 => {
    //   block [0x82430184..0x82430190)
	// 82430184: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82430188: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8243018C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82430190 size=204
    let mut pc: u32 = 0x82430190;
    'dispatch: loop {
        match pc {
            0x82430190 => {
    //   block [0x82430190..0x8243025C)
	// 82430190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82430194: 48104F29  bl 0x825350bc
	ctx.lr = 0x82430198;
	sub_82535080(ctx, base);
	// 82430198: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243019C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824301A0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824301A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824301A8: 409A000C  bne cr6, 0x824301b4
	if !ctx.cr[6].eq {
	pc = 0x824301B4; continue 'dispatch;
	}
	// 824301AC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 824301B0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824301B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824301B8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824301BC: 409A0098  bne cr6, 0x82430254
	if !ctx.cr[6].eq {
	pc = 0x82430254; continue 'dispatch;
	}
	// 824301C0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824301C4: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 824301C8: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824301CC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824301D0: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824301D4: A17F0038  lhz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 824301D8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824301DC: 409A0024  bne cr6, 0x82430200
	if !ctx.cr[6].eq {
	pc = 0x82430200; continue 'dispatch;
	}
	// 824301E0: 393F0034  addi r9, r31, 0x34
	ctx.r[9].s64 = ctx.r[31].s64 + 52;
	// 824301E4: A15F0036  lhz r10, 0x36(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 824301E8: A11F0032  lhz r8, 0x32(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 824301EC: A0FF0030  lhz r7, 0x30(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824301F0: B1610056  sth r11, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[11].u16 ) };
	// 824301F4: 480012BD  bl 0x824314b0
	ctx.lr = 0x824301F8;
	sub_824314B0(ctx, base);
	// 824301F8: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 824301FC: 48000050  b 0x8243024c
	pc = 0x8243024C; continue 'dispatch;
	// 82430200: A3BF0036  lhz r29, 0x36(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82430204: 3BDF0034  addi r30, r31, 0x34
	ctx.r[30].s64 = ctx.r[31].s64 + 52;
	// 82430208: 391F002C  addi r8, r31, 0x2c
	ctx.r[8].s64 = ctx.r[31].s64 + 44;
	// 8243020C: A15F0032  lhz r10, 0x32(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 82430210: A13F0030  lhz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82430214: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82430218: B1610066  sth r11, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[11].u16 ) };
	// 8243021C: B3A1005E  sth r29, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[29].u16 ) };
	// 82430220: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82430224: 48001945  bl 0x82431b68
	ctx.lr = 0x82430228;
	sub_82431B68(ctx, base);
	// 82430228: 7C6B0E70  srawi r11, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 8243022C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82430230: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82430234: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82430238: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8243023C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82430240: 409A000C  bne cr6, 0x8243024c
	if !ctx.cr[6].eq {
	pc = 0x8243024C; continue 'dispatch;
	}
	// 82430244: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82430248: 916A9B40  stw r11, -0x64c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25792 as u32), ctx.r[11].u32 ) };
	// 8243024C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82430250: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82430254: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82430258: 48104EB4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82430260 size=600
    let mut pc: u32 = 0x82430260;
    'dispatch: loop {
        match pc {
            0x82430260 => {
    //   block [0x82430260..0x824304B8)
	// 82430260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82430264: 48104E55  bl 0x825350b8
	ctx.lr = 0x82430268;
	sub_82535080(ctx, base);
	// 82430268: 89430003  lbz r10, 3(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 8243026C: 3D600064  lis r11, 0x64
	ctx.r[11].s64 = 6553600;
	// 82430270: 8BC30002  lbz r30, 2(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430274: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430278: 8BA30001  lbz r29, 1(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8243027C: 8B830000  lbz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430280: 617F732E  ori r31, r11, 0x732e
	ctx.r[31].u64 = ctx.r[11].u64 | 29486;
	// 82430284: 7D4AF378  or r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	// 82430288: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 8243028C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430290: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 82430294: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430298: 7D4AE378  or r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 8243029C: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824302A0: 419A001C  beq cr6, 0x824302bc
	if ctx.cr[6].eq {
	pc = 0x824302BC; continue 'dispatch;
	}
	// 824302A4: 3FE0646E  lis r31, 0x646e
	ctx.r[31].s64 = 1684930560;
	// 824302A8: 63FF732E  ori r31, r31, 0x732e
	ctx.r[31].u64 = ctx.r[31].u64 | 29486;
	// 824302AC: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824302B0: 419A000C  beq cr6, 0x824302bc
	if ctx.cr[6].eq {
	pc = 0x824302BC; continue 'dispatch;
	}
	// 824302B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824302B8: 480001FC  b 0x824304b4
	pc = 0x824304B4; continue 'dispatch;
	// 824302BC: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 824302C0: 8BEB0002  lbz r31, 2(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 824302C4: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 824302C8: 8BCB0001  lbz r30, 1(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824302CC: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824302D0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824302D4: 7D4AFB78  or r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 824302D8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824302DC: 7D4AF378  or r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	// 824302E0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824302E4: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 824302E8: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 824302EC: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 824302F0: 515F843E  rlwimi r31, r10, 0x10, 0x10, 0x1f
	ctx.r[31].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[31].u64 & 0xFFFFFFFFFFFF0000);
	// 824302F4: 515E801E  rlwimi r30, r10, 0x10, 0, 0xf
	ctx.r[30].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[30].u64 & 0xFFFFFFFF0000FFFF);
	// 824302F8: 57EAC43E  rlwinm r10, r31, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 824302FC: 57DF401E  rlwinm r31, r30, 8, 0, 0xf
	ctx.r[31].u64 = ctx.r[30].u32 as u64 & 0x00FFFFFFu64;
	// 82430300: 7D5EFB78  or r30, r10, r31
	ctx.r[30].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 82430304: 7F1E2000  cmpw cr6, r30, r4
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82430308: 4199FFAC  bgt cr6, 0x824302b4
	if ctx.cr[6].gt {
	pc = 0x824302B4; continue 'dispatch;
	}
	// 8243030C: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430310: 888B0002  lbz r4, 2(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430314: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430318: 8BEB0001  lbz r31, 1(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8243031C: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430320: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82430324: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 82430328: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243032C: 888B0003  lbz r4, 3(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430330: 7D4AFB78  or r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 82430334: 8BEB0002  lbz r31, 2(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430338: 5484403E  rotlwi r4, r4, 8
	ctx.r[4].u64 = ((ctx.r[4].u32).rotate_left(8)) as u64;
	// 8243033C: 8B8B0000  lbz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430340: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430344: 7C84FB78  or r4, r4, r31
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[31].u64;
	// 82430348: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 8243034C: 8BAB0001  lbz r29, 1(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430350: 5484402E  slwi r4, r4, 8
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82430354: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82430358: 7C84EB78  or r4, r4, r29
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[29].u64;
	// 8243035C: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82430360: 515F843E  rlwimi r31, r10, 0x10, 0x10, 0x1f
	ctx.r[31].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[31].u64 & 0xFFFFFFFFFFFF0000);
	// 82430364: 515D801E  rlwimi r29, r10, 0x10, 0, 0xf
	ctx.r[29].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[29].u64 & 0xFFFFFFFF0000FFFF);
	// 82430368: 5484402E  slwi r4, r4, 8
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8243036C: 57EAC43E  rlwinm r10, r31, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82430370: 57BF401E  rlwinm r31, r29, 8, 0, 0xf
	ctx.r[31].u64 = ctx.r[29].u32 as u64 & 0x00FFFFFFu64;
	// 82430374: 7C84E378  or r4, r4, r28
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[28].u64;
	// 82430378: 7D5FFB78  or r31, r10, r31
	ctx.r[31].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 8243037C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82430380: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82430384: 508A843E  rlwimi r10, r4, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82430388: 509D801E  rlwimi r29, r4, 0x10, 0, 0xf
	ctx.r[29].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[29].u64 & 0xFFFFFFFF0000FFFF);
	// 8243038C: 554AC43E  rlwinm r10, r10, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82430390: 57A4401E  rlwinm r4, r29, 8, 0, 0xf
	ctx.r[4].u64 = ctx.r[29].u32 as u64 & 0x00FFFFFFu64;
	// 82430394: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82430398: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 8243039C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 824303A0: 419A0028  beq cr6, 0x824303c8
	if ctx.cr[6].eq {
	pc = 0x824303C8; continue 'dispatch;
	}
	// 824303A4: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 824303A8: 419A0018  beq cr6, 0x824303c0
	if ctx.cr[6].eq {
	pc = 0x824303C0; continue 'dispatch;
	}
	// 824303AC: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 824303B0: 409AFF04  bne cr6, 0x824302b4
	if !ctx.cr[6].eq {
	pc = 0x824302B4; continue 'dispatch;
	}
	// 824303B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824303B8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 824303BC: 48000014  b 0x824303d0
	pc = 0x824303D0; continue 'dispatch;
	// 824303C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824303C4: 48000008  b 0x824303cc
	pc = 0x824303CC; continue 'dispatch;
	// 824303C8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 824303CC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824303D0: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824303D4: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824303D8: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 824303DC: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 824303E0: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 824303E4: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824303E8: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824303EC: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 824303F0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824303F4: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 824303F8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824303FC: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 82430400: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82430404: 7D44C670  srawi r4, r10, 0x18
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 24) as i64;
	// 82430408: 5147801E  rlwimi r7, r10, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 8243040C: 7D4A4670  srawi r10, r10, 8
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 8) as i64;
	// 82430410: 54E7002E  rlwinm r7, r7, 0, 0, 0x17
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82430414: 554A042E  rlwinm r10, r10, 0, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82430418: 50E4402E  rlwimi r4, r7, 8, 0, 0x17
	ctx.r[4].u64 = (((ctx.r[7].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[4].u64 & 0xFFFFFFFF000000FF);
	// 8243041C: 7C8A5378  or r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 | ctx.r[10].u64;
	// 82430420: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82430424: 894B0007  lbz r10, 7(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 82430428: 88EB0006  lbz r7, 6(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8243042C: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430430: 88AB0005  lbz r5, 5(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82430434: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82430438: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 8243043C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430440: 7D4A2B78  or r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 82430444: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430448: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8243044C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82430450: 7D67C670  srawi r7, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 82430454: 516A801E  rlwimi r10, r11, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82430458: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243045C: 554A002E  rlwinm r10, r10, 0, 0, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82430460: 5147402E  rlwimi r7, r10, 8, 0, 0x17
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[7].u64 & 0xFFFFFFFF000000FF);
	// 82430464: 7D6A4670  srawi r10, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 82430468: 554A042E  rlwinm r10, r10, 0, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8243046C: 7CEA5378  or r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 | ctx.r[10].u64;
	// 82430470: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82430474: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430478: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8243047C: 409A000C  bne cr6, 0x82430488
	if !ctx.cr[6].eq {
	pc = 0x82430488; continue 'dispatch;
	}
	// 82430480: 7D7F53D6  divw r11, r31, r10
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[10].s32;
	// 82430484: 48000028  b 0x824304ac
	pc = 0x824304AC; continue 'dispatch;
	// 82430488: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243048C: 419AFFF4  beq cr6, 0x82430480
	if ctx.cr[6].eq {
	pc = 0x82430480; continue 'dispatch;
	}
	// 82430490: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82430494: 409A0014  bne cr6, 0x824304a8
	if !ctx.cr[6].eq {
	pc = 0x824304A8; continue 'dispatch;
	}
	// 82430498: 7FEB0E70  srawi r11, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 8243049C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824304A0: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 824304A4: 48000008  b 0x824304ac
	pc = 0x824304AC; continue 'dispatch;
	// 824304A8: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 824304AC: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824304B0: 7C7E1A14  add r3, r30, r3
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 824304B4: 48104C54  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824304B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824304B8 size=116
    let mut pc: u32 = 0x824304B8;
    'dispatch: loop {
        match pc {
            0x824304B8 => {
    //   block [0x824304B8..0x8243052C)
	// 824304B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824304BC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824304C0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824304C4: 39293F40  addi r9, r9, 0x3f40
	ctx.r[9].s64 = ctx.r[9].s64 + 16192;
	// 824304C8: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 824304CC: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824304D0: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824304D4: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824304D8: 40820014  bne 0x824304ec
	if !ctx.cr[0].eq {
	pc = 0x824304EC; continue 'dispatch;
	}
	// 824304DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824304E0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824304E4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824304E8: 409AFFE4  bne cr6, 0x824304cc
	if !ctx.cr[6].eq {
	pc = 0x824304CC; continue 'dispatch;
	}
	// 824304EC: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824304F0: 4182003C  beq 0x8243052c
	if ctx.cr[0].eq {
		sub_8243052C(ctx, base);
		return;
	}
	// 824304F4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824304F8: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 824304FC: 394A3F3C  addi r10, r10, 0x3f3c
	ctx.r[10].s64 = ctx.r[10].s64 + 16188;
	// 82430500: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430504: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430508: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8243050C: 40820014  bne 0x82430520
	if !ctx.cr[0].eq {
	pc = 0x82430520; continue 'dispatch;
	}
	// 82430510: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82430514: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82430518: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8243051C: 409AFFE4  bne cr6, 0x82430500
	if !ctx.cr[6].eq {
	pc = 0x82430500; continue 'dispatch;
	}
	// 82430520: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82430524: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82430528: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243052C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243052C size=8
    let mut pc: u32 = 0x8243052C;
    'dispatch: loop {
        match pc {
            0x8243052C => {
    //   block [0x8243052C..0x82430534)
	// 8243052C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82430530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82430538 size=240
    let mut pc: u32 = 0x82430538;
    'dispatch: loop {
        match pc {
            0x82430538 => {
    //   block [0x82430538..0x82430628)
	// 82430538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243053C: 48104B71  bl 0x825350ac
	ctx.lr = 0x82430540;
	sub_82535080(ctx, base);
	// 82430540: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82430544: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82430548: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243054C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82430550: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82430554: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82430558: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8243055C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82430560: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 82430564: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82430568: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 8243056C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82430570: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82430574: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 82430578: 40980010  bge cr6, 0x82430588
	if !ctx.cr[6].lt {
	pc = 0x82430588; continue 'dispatch;
	}
	// 8243057C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82430580: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82430584: 4800009C  b 0x82430620
	pc = 0x82430620; continue 'dispatch;
	// 82430588: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 8243058C: 81210104  lwz r9, 0x104(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82430590: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82430594: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82430598: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243059C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824305A0: 4BFFFCC1  bl 0x82430260
	ctx.lr = 0x824305A4;
	sub_82430260(ctx, base);
	// 824305A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824305A8: 4082000C  bne 0x824305b4
	if !ctx.cr[0].eq {
	pc = 0x824305B4; continue 'dispatch;
	}
	// 824305AC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824305B0: 48000070  b 0x82430620
	pc = 0x82430620; continue 'dispatch;
	// 824305B4: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 824305B8: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824305BC: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824305C0: 4081FFEC  ble 0x824305ac
	if !ctx.cr[0].gt {
	pc = 0x824305AC; continue 'dispatch;
	}
	// 824305C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824305C8: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824305CC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824305D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824305D4: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824305D8: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824305DC: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824305E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824305E4: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824305E8: 814100F4  lwz r10, 0xf4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 824305EC: 993D0000  stb r9, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 824305F0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824305F4: 98FB0000  stb r7, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 824305F8: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824305FC: 893C0000  lbz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430600: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82430604: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82430608: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8243060C: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 82430610: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82430614: 995A0000  stb r10, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82430618: 814100FC  lwz r10, 0xfc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 8243061C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82430620: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82430624: 48104AD8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82430628 size=236
    let mut pc: u32 = 0x82430628;
    'dispatch: loop {
        match pc {
            0x82430628 => {
    //   block [0x82430628..0x82430714)
	// 82430628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243062C: 48104A8D  bl 0x825350b8
	ctx.lr = 0x82430630;
	sub_82535080(ctx, base);
	// 82430630: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82430634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82430638: 39210074  addi r9, r1, 0x74
	ctx.r[9].s64 = ctx.r[1].s64 + 116;
	// 8243063C: 391F0018  addi r8, r31, 0x18
	ctx.r[8].s64 = ctx.r[31].s64 + 24;
	// 82430640: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82430644: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 82430648: 3BDF000E  addi r30, r31, 0xe
	ctx.r[30].s64 = ctx.r[31].s64 + 14;
	// 8243064C: 3BBF000F  addi r29, r31, 0xf
	ctx.r[29].s64 = ctx.r[31].s64 + 15;
	// 82430650: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82430654: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82430658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8243065C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82430660: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82430664: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 82430668: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8243066C: 38FF000D  addi r7, r31, 0xd
	ctx.r[7].s64 = ctx.r[31].s64 + 13;
	// 82430670: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 82430674: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82430678: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8243067C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82430680: 4BFFFEB9  bl 0x82430538
	ctx.lr = 0x82430684;
	sub_82430538(ctx, base);
	// 82430684: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82430688: 4080000C  bge 0x82430694
	if !ctx.cr[0].lt {
	pc = 0x82430694; continue 'dispatch;
	}
	// 8243068C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82430690: 4800007C  b 0x8243070c
	pc = 0x8243070C; continue 'dispatch;
	// 82430694: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243069C: 893D0000  lbz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824306A0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824306A4: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824306A8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824306AC: 80FF003C  lwz r7, 0x3c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 824306B0: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 824306B4: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824306B8: 80BF0044  lwz r5, 0x44(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 824306BC: 83C10074  lwz r30, 0x74(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 824306C0: A8610070  lha r3, 0x70(r1)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as i16) as i64;
	// 824306C4: B17F001C  sth r11, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 824306C8: B17F0026  sth r11, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 824306CC: B17F0024  sth r11, 0x24(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 824306D0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 824306D4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824306D8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824306DC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824306E0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824306E4: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824306E8: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 824306EC: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 824306F0: 90FF005C  stw r7, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 824306F4: 90DF0060  stw r6, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 824306F8: 90BF0064  stw r5, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 824306FC: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82430700: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82430704: B09F0098  sth r4, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[4].u16 ) };
	// 82430708: B3DF009C  sth r30, 0x9c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u16 ) };
	// 8243070C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82430710: 481049F8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82430718 size=392
    let mut pc: u32 = 0x82430718;
    'dispatch: loop {
        match pc {
            0x82430718 => {
    //   block [0x82430718..0x824308A0)
	// 82430718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243071C: 481049A1  bl 0x825350bc
	ctx.lr = 0x82430720;
	sub_82535080(ctx, base);
	// 82430720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82430724: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82430728: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 8243072C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82430730: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430734: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82430738: 409A0134  bne cr6, 0x8243086c
	if !ctx.cr[6].eq {
	pc = 0x8243086C; continue 'dispatch;
	}
	// 8243073C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82430740: 4BFFF989  bl 0x824300c8
	ctx.lr = 0x82430744;
	sub_824300C8(ctx, base);
	// 82430744: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82430748: 40820124  bne 0x8243086c
	if !ctx.cr[0].eq {
	pc = 0x8243086C; continue 'dispatch;
	}
	// 8243074C: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82430750: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 82430754: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 82430758: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8243075C: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 82430760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82430764: 4E800421  bctrl
	ctx.lr = 0x82430768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82430768: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243076C: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82430770: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82430774: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82430778: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243077C: 40990008  ble cr6, 0x82430784
	if !ctx.cr[6].gt {
	pc = 0x82430784; continue 'dispatch;
	}
	// 82430780: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82430784: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82430788: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243078C: 40990008  ble cr6, 0x82430794
	if !ctx.cr[6].gt {
	pc = 0x82430794; continue 'dispatch;
	}
	// 82430790: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82430794: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82430798: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8243079C: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 824307A0: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824307A4: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 824307A8: 409A006C  bne cr6, 0x82430814
	if !ctx.cr[6].eq {
	pc = 0x82430814; continue 'dispatch;
	}
	// 824307AC: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824307B0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824307B4: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 824307B8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824307BC: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824307C0: 4099008C  ble cr6, 0x8243084c
	if !ctx.cr[6].gt {
	pc = 0x8243084C; continue 'dispatch;
	}
	// 824307C4: 7CCA4050  subf r6, r10, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 824307C8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 824307CC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 824307D0: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824307D4: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824307D8: 54E5403E  rotlwi r5, r7, 8
	ctx.r[5].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 824307DC: 54E7C23E  srwi r7, r7, 8
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824307E0: 54A5043E  clrlwi r5, r5, 0x10
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 824307E4: 7CA73B78  or r7, r5, r7
	ctx.r[7].u64 = ctx.r[5].u64 | ctx.r[7].u64;
	// 824307E8: 7CE6532E  sthx r7, r6, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u16) };
	// 824307EC: A0EB0002  lhz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 824307F0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824307F4: 54E5403E  rotlwi r5, r7, 8
	ctx.r[5].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 824307F8: 54E7C23E  srwi r7, r7, 8
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824307FC: 54A5043E  clrlwi r5, r5, 0x10
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 82430800: 7CA73B78  or r7, r5, r7
	ctx.r[7].u64 = ctx.r[5].u64 | ctx.r[7].u64;
	// 82430804: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 82430808: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8243080C: 4082FFC4  bne 0x824307d0
	if !ctx.cr[0].eq {
	pc = 0x824307D0; continue 'dispatch;
	}
	// 82430810: 4800003C  b 0x8243084c
	pc = 0x8243084C; continue 'dispatch;
	// 82430814: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82430818: 40990034  ble cr6, 0x8243084c
	if !ctx.cr[6].gt {
	pc = 0x8243084C; continue 'dispatch;
	}
	// 8243081C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82430820: 7CE8E850  subf r7, r8, r29
	ctx.r[7].s64 = ctx.r[29].s64 - ctx.r[8].s64;
	// 82430824: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82430828: 7D075A2E  lhzx r8, r7, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8243082C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82430830: 5506403E  rotlwi r6, r8, 8
	ctx.r[6].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 82430834: 5508C23E  srwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82430838: 54C6043E  clrlwi r6, r6, 0x10
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	// 8243083C: 7CC84378  or r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 82430840: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82430844: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82430848: 4082FFE0  bne 0x82430828
	if !ctx.cr[0].eq {
	pc = 0x82430828; continue 'dispatch;
	}
	// 8243084C: 897E000E  lbz r11, 0xe(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 82430850: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82430854: 913E0090  stw r9, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 82430858: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8243085C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82430860: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82430864: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82430868: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8243086C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82430870: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82430874: 409A0024  bne cr6, 0x82430898
	if !ctx.cr[6].eq {
	pc = 0x82430898; continue 'dispatch;
	}
	// 82430878: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8243087C: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 82430880: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82430884: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82430888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243088C: 4E800421  bctrl
	ctx.lr = 0x82430890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82430890: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82430894: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82430898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243089C: 48104870  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824308A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824308A0 size=380
    let mut pc: u32 = 0x824308A0;
    'dispatch: loop {
        match pc {
            0x824308A0 => {
    //   block [0x824308A0..0x82430A1C)
	// 824308A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824308A4: 48104819  bl 0x825350bc
	ctx.lr = 0x824308A8;
	sub_82535080(ctx, base);
	// 824308A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824308AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824308B0: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 824308B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824308B8: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824308BC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824308C0: 409A0128  bne cr6, 0x824309e8
	if !ctx.cr[6].eq {
	pc = 0x824309E8; continue 'dispatch;
	}
	// 824308C4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824308C8: 4BFFF801  bl 0x824300c8
	ctx.lr = 0x824308CC;
	sub_824300C8(ctx, base);
	// 824308CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824308D0: 40820118  bne 0x824309e8
	if !ctx.cr[0].eq {
	pc = 0x824309E8; continue 'dispatch;
	}
	// 824308D4: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 824308D8: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 824308DC: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 824308E0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824308E4: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 824308E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824308EC: 4E800421  bctrl
	ctx.lr = 0x824308F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824308F0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824308F4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824308F8: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824308FC: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82430900: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82430904: 40990008  ble cr6, 0x8243090c
	if !ctx.cr[6].gt {
	pc = 0x8243090C; continue 'dispatch;
	}
	// 82430908: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8243090C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82430910: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82430914: 40990008  ble cr6, 0x8243091c
	if !ctx.cr[6].gt {
	pc = 0x8243091C; continue 'dispatch;
	}
	// 82430918: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8243091C: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 82430920: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82430924: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82430928: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 8243092C: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82430930: 409A0064  bne cr6, 0x82430994
	if !ctx.cr[6].eq {
	pc = 0x82430994; continue 'dispatch;
	}
	// 82430934: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82430938: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243093C: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82430940: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82430944: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82430948: 40990084  ble cr6, 0x824309cc
	if !ctx.cr[6].gt {
	pc = 0x824309CC; continue 'dispatch;
	}
	// 8243094C: 3D008273  lis r8, -0x7d8d
	ctx.r[8].s64 = -2106392576;
	// 82430950: 7CDD4850  subf r6, r29, r9
	ctx.r[6].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 82430954: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82430958: 7CFD3850  subf r7, r29, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[29].s64;
	// 8243095C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82430960: 39083F30  addi r8, r8, 0x3f30
	ctx.r[8].s64 = ctx.r[8].s64 + 16176;
	// 82430964: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430968: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8243096C: 54A5083E  rotlwi r5, r5, 1
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(1)) as u64;
	// 82430970: 7CA5422E  lhzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82430974: 7CA75B2E  sthx r5, r7, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u16) };
	// 82430978: 88AB0001  lbz r5, 1(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8243097C: 54A5083E  rotlwi r5, r5, 1
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(1)) as u64;
	// 82430980: 7CA5422E  lhzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82430984: 7CA65B2E  sthx r5, r6, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u16) };
	// 82430988: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8243098C: 4082FFD8  bne 0x82430964
	if !ctx.cr[0].eq {
	pc = 0x82430964; continue 'dispatch;
	}
	// 82430990: 4800003C  b 0x824309cc
	pc = 0x824309CC; continue 'dispatch;
	// 82430994: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82430998: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243099C: 40990030  ble cr6, 0x824309cc
	if !ctx.cr[6].gt {
	pc = 0x824309CC; continue 'dispatch;
	}
	// 824309A0: 3D008273  lis r8, -0x7d8d
	ctx.r[8].s64 = -2106392576;
	// 824309A4: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 824309A8: 39083F30  addi r8, r8, 0x3f30
	ctx.r[8].s64 = ctx.r[8].s64 + 16176;
	// 824309AC: 7CEBE8AE  lbzx r7, r11, r29
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824309B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824309B4: 54E7083E  rotlwi r7, r7, 1
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(1)) as u64;
	// 824309B8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824309BC: 7CE7422E  lhzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824309C0: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824309C4: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 824309C8: 4198FFE4  blt cr6, 0x824309ac
	if ctx.cr[6].lt {
	pc = 0x824309AC; continue 'dispatch;
	}
	// 824309CC: 897E000E  lbz r11, 0xe(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 824309D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 824309D4: 915E0090  stw r10, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 824309D8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824309DC: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 824309E0: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824309E4: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 824309E8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824309EC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824309F0: 409A0024  bne cr6, 0x82430a14
	if !ctx.cr[6].eq {
	pc = 0x82430A14; continue 'dispatch;
	}
	// 824309F4: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824309F8: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 824309FC: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82430A00: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82430A04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82430A08: 4E800421  bctrl
	ctx.lr = 0x82430A0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82430A0C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82430A10: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82430A14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82430A18: 481046F4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430A20 size=16
    let mut pc: u32 = 0x82430A20;
    'dispatch: loop {
        match pc {
            0x82430A20 => {
    //   block [0x82430A20..0x82430A30)
	// 82430A20: A963009C  lha r11, 0x9c(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as i16) as i64;
	// 82430A24: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82430A28: 409A0008  bne cr6, 0x82430a30
	if !ctx.cr[6].eq {
		sub_82430A30(ctx, base);
		return;
	}
	// 82430A2C: 4BFFFE74  b 0x824308a0
	sub_824308A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430A30 size=12
    let mut pc: u32 = 0x82430A30;
    'dispatch: loop {
        match pc {
            0x82430A30 => {
    //   block [0x82430A30..0x82430A3C)
	// 82430A30: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82430A34: 409A0008  bne cr6, 0x82430a3c
	if !ctx.cr[6].eq {
		sub_82430A3C(ctx, base);
		return;
	}
	// 82430A38: 480006D8  b 0x82431110
	sub_82431110(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430A3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430A3C size=4
    let mut pc: u32 = 0x82430A3C;
    'dispatch: loop {
        match pc {
            0x82430A3C => {
    //   block [0x82430A3C..0x82430A40)
	// 82430A3C: 4BFFFCDC  b 0x82430718
	sub_82430718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430A40 size=780
    let mut pc: u32 = 0x82430A40;
    'dispatch: loop {
        match pc {
            0x82430A40 => {
    //   block [0x82430A40..0x82430D4C)
	// 82430A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82430A44: 4810466D  bl 0x825350b0
	ctx.lr = 0x82430A48;
	sub_82535080(ctx, base);
	// 82430A48: 89430003  lbz r10, 3(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430A4C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82430A50: 89230002  lbz r9, 2(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430A54: 3D004D52  lis r8, 0x4d52
	ctx.r[8].s64 = 1297219584;
	// 82430A58: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430A5C: 8BA30001  lbz r29, 1(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430A60: 8B830000  lbz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430A64: 611E4F46  ori r30, r8, 0x4f46
	ctx.r[30].u64 = ctx.r[8].u64 | 20294;
	// 82430A68: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82430A6C: 8B6B0000  lbz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430A70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82430A74: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430A78: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82430A7C: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 82430A80: 8BAB0002  lbz r29, 2(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430A84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82430A88: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430A8C: 7D49E378  or r9, r10, r28
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 82430A90: 8B8B0001  lbz r28, 1(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430A94: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430A98: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82430A9C: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82430AA0: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430AA4: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 82430AA8: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430AAC: 8BCB0002  lbz r30, 2(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430AB0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430AB4: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82430AB8: 8BAB0001  lbz r29, 1(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430ABC: 7D4AE378  or r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 82430AC0: 8B8B0000  lbz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430AC4: 7D29F378  or r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[30].u64;
	// 82430AC8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430ACC: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82430AD0: 7D4ADB78  or r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[27].u64;
	// 82430AD4: 7D29EB78  or r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[29].u64;
	// 82430AD8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82430ADC: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82430AE0: 7D29E378  or r9, r9, r28
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[28].u64;
	// 82430AE4: 409A0260  bne cr6, 0x82430d44
	if !ctx.cr[6].eq {
	pc = 0x82430D44; continue 'dispatch;
	}
	// 82430AE8: 3FC04646  lis r30, 0x4646
	ctx.r[30].s64 = 1178992640;
	// 82430AEC: 63DE4941  ori r30, r30, 0x4941
	ctx.r[30].u64 = ctx.r[30].u64 | 18753;
	// 82430AF0: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82430AF4: 409A0250  bne cr6, 0x82430d44
	if !ctx.cr[6].eq {
	pc = 0x82430D44; continue 'dispatch;
	}
	// 82430AF8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82430AFC: 7D5E4670  srawi r30, r10, 8
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[10].s32 >> 8) as i64;
	// 82430B00: 5149801E  rlwimi r9, r10, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82430B04: 513E401E  rlwimi r30, r9, 8, 0, 0xf
	ctx.r[30].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFF0000) | (ctx.r[30].u64 & 0xFFFFFFFF0000FFFF);
	// 82430B08: 7D49C670  srawi r9, r10, 0x18
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 24) as i64;
	// 82430B0C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82430B10: 512A063E  rlwimi r10, r9, 0, 0x18, 0x1f
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000000000FF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF00);
	// 82430B14: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82430B18: 3BCAFFFC  addi r30, r10, -4
	ctx.r[30].s64 = ctx.r[10].s64 + -4;
	// 82430B1C: 4800021C  b 0x82430d38
	pc = 0x82430D38; continue 'dispatch;
	// 82430B20: 3D20444E  lis r9, 0x444e
	ctx.r[9].s64 = 1145962496;
	// 82430B24: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430B28: 8B8B0001  lbz r28, 1(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430B2C: 613D5353  ori r29, r9, 0x5353
	ctx.r[29].u64 = ctx.r[9].u64 | 21331;
	// 82430B30: 892B0002  lbz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430B34: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430B38: 8B6B0000  lbz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430B3C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82430B40: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82430B44: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430B48: 7D4AE378  or r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 82430B4C: 8B8B0002  lbz r28, 2(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430B50: 8B4B0000  lbz r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430B54: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430B58: 7D49DB78  or r9, r10, r27
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[27].u64;
	// 82430B5C: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430B60: 8B6B0001  lbz r27, 1(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430B64: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82430B68: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430B6C: 7F09E800  cmpw cr6, r9, r29
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82430B70: 7D4AE378  or r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 82430B74: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430B78: 7D4ADB78  or r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[27].u64;
	// 82430B7C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430B80: 7D4AD378  or r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[26].u64;
	// 82430B84: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82430B88: 7D5C4670  srawi r28, r10, 8
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[10].s32 >> 8) as i64;
	// 82430B8C: 515D801E  rlwimi r29, r10, 0x10, 0, 0xf
	ctx.r[29].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[29].u64 & 0xFFFFFFFF0000FFFF);
	// 82430B90: 53BC401E  rlwimi r28, r29, 8, 0, 0xf
	ctx.r[28].u64 = (((ctx.r[29].u32).rotate_left(8) as u64) & 0x00000000FFFF0000) | (ctx.r[28].u64 & 0xFFFFFFFF0000FFFF);
	// 82430B94: 7D5DC670  srawi r29, r10, 0x18
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[10].s32 >> 24) as i64;
	// 82430B98: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82430B9C: 53AA063E  rlwimi r10, r29, 0, 0x18, 0x1f
	ctx.r[10].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000000000FF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF00);
	// 82430BA0: 419A0138  beq cr6, 0x82430cd8
	if ctx.cr[6].eq {
	pc = 0x82430CD8; continue 'dispatch;
	}
	// 82430BA4: 3FA04D4D  lis r29, 0x4d4d
	ctx.r[29].s64 = 1296891904;
	// 82430BA8: 63BD4F43  ori r29, r29, 0x4f43
	ctx.r[29].u64 = ctx.r[29].u64 | 20291;
	// 82430BAC: 7F09E800  cmpw cr6, r9, r29
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82430BB0: 419A0014  beq cr6, 0x82430bc4
	if ctx.cr[6].eq {
	pc = 0x82430BC4; continue 'dispatch;
	}
	// 82430BB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82430BB8: 554A003C  rlwinm r10, r10, 0, 0, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82430BBC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82430BC0: 48000178  b 0x82430d38
	pc = 0x82430D38; continue 'dispatch;
	// 82430BC4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82430BC8: 409A0170  bne cr6, 0x82430d38
	if !ctx.cr[6].eq {
	pc = 0x82430D38; continue 'dispatch;
	}
	// 82430BCC: 2F0A0012  cmpwi cr6, r10, 0x12
	ctx.cr[6].compare_i32(ctx.r[10].s32, 18, &mut ctx.xer);
	// 82430BD0: 41980174  blt cr6, 0x82430d44
	if ctx.cr[6].lt {
	pc = 0x82430D44; continue 'dispatch;
	}
	// 82430BD4: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430BD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82430BDC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430BE0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82430BE4: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430BE8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82430BEC: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82430BF0: 7D494670  srawi r9, r10, 8
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 8) as i64;
	// 82430BF4: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 82430BF8: 552A043E  clrlwi r10, r9, 0x10
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82430BFC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82430C00: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430C04: 892B0002  lbz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430C08: 555D403E  rotlwi r29, r10, 8
	ctx.r[29].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430C0C: 8B8B0001  lbz r28, 1(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430C10: 8B6B0000  lbz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430C14: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82430C18: 7FAB4B78  or r11, r29, r9
	ctx.r[11].u64 = ctx.r[29].u64 | ctx.r[9].u64;
	// 82430C1C: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82430C20: 7D6BE378  or r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 82430C24: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82430C28: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 82430C2C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82430C30: 7D7DC670  srawi r29, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 82430C34: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82430C38: 7D6B4670  srawi r11, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 82430C3C: 5529002E  rlwinm r9, r9, 0, 0, 0x17
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82430C40: 556B042E  rlwinm r11, r11, 0, 0x10, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82430C44: 513D402E  rlwimi r29, r9, 8, 0, 0x17
	ctx.r[29].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[29].u64 & 0xFFFFFFFF000000FF);
	// 82430C48: 7FAB5B78  or r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 | ctx.r[11].u64;
	// 82430C4C: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82430C50: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 82430C54: 892A0001  lbz r9, 1(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430C58: 8BAA0000  lbz r29, 0(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430C5C: 552A403E  rotlwi r10, r9, 8
	ctx.r[10].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82430C60: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 82430C64: 7D494670  srawi r9, r10, 8
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 8) as i64;
	// 82430C68: 5149442E  rlwimi r9, r10, 8, 0x10, 0x17
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF00FF);
	// 82430C6C: 552A043E  clrlwi r10, r9, 0x10
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82430C70: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82430C74: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430C78: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430C7C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82430C80: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430C84: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82430C88: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430C8C: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430C90: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82430C94: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82430C98: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82430C9C: 7D29EB78  or r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[29].u64;
	// 82430CA0: 555D422E  rlwinm r29, r10, 8, 8, 0x17
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82430CA4: 554AC63E  rlwinm r10, r10, 0x18, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82430CA8: 57BD043E  clrlwi r29, r29, 0x10
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 82430CAC: 7FAA5378  or r10, r29, r10
	ctx.r[10].u64 = ctx.r[29].u64 | ctx.r[10].u64;
	// 82430CB0: 214A400E  subfic r10, r10, 0x400e
	ctx.xer.ca = ctx.r[10].u32 <= 16398 as u32;
	ctx.r[10].s64 = (16398 as i64) - ctx.r[10].s64;
	// 82430CB4: 555D043E  clrlwi r29, r10, 0x10
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82430CB8: 552A043E  clrlwi r10, r9, 0x10
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82430CBC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82430CC0: 554AC23E  srwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430CC4: 5529442E  rlwinm r9, r9, 8, 0x10, 0x17
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82430CC8: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82430CCC: 7D4AEC30  srw r10, r10, r29
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	// 82430CD0: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82430CD4: 48000060  b 0x82430d34
	pc = 0x82430D34; continue 'dispatch;
	// 82430CD8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82430CDC: 409A005C  bne cr6, 0x82430d38
	if !ctx.cr[6].eq {
	pc = 0x82430D38; continue 'dispatch;
	}
	// 82430CE0: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82430CE4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82430CE8: 892B0002  lbz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82430CEC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82430CF0: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82430CF4: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82430CF8: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430CFC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82430D00: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82430D04: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430D08: 7D4A1B78  or r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[3].u64;
	// 82430D0C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82430D10: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 82430D14: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82430D18: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82430D1C: 5149843E  rlwimi r9, r10, 0x10, 0x10, 0x1f
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF0000);
	// 82430D20: 5143801E  rlwimi r3, r10, 0x10, 0, 0xf
	ctx.r[3].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[3].u64 & 0xFFFFFFFF0000FFFF);
	// 82430D24: 552AC43E  rlwinm r10, r9, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82430D28: 5469401E  rlwinm r9, r3, 8, 0, 0xf
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x00FFFFFFu64;
	// 82430D2C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82430D30: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82430D34: 409A0014  bne cr6, 0x82430d48
	if !ctx.cr[6].eq {
	pc = 0x82430D48; continue 'dispatch;
	}
	// 82430D38: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82430D3C: 4198FDE4  blt cr6, 0x82430b20
	if ctx.cr[6].lt {
	pc = 0x82430B20; continue 'dispatch;
	}
	// 82430D40: 48000008  b 0x82430d48
	pc = 0x82430D48; continue 'dispatch;
	// 82430D44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82430D48: 481043B8  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430D50 size=116
    let mut pc: u32 = 0x82430D50;
    'dispatch: loop {
        match pc {
            0x82430D50 => {
    //   block [0x82430D50..0x82430DC4)
	// 82430D50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82430D54: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82430D58: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82430D5C: 394A3F50  addi r10, r10, 0x3f50
	ctx.r[10].s64 = ctx.r[10].s64 + 16208;
	// 82430D60: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430D64: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430D68: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82430D6C: 40820014  bne 0x82430d80
	if !ctx.cr[0].eq {
	pc = 0x82430D80; continue 'dispatch;
	}
	// 82430D70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82430D74: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82430D78: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82430D7C: 409AFFE4  bne cr6, 0x82430d60
	if !ctx.cr[6].eq {
	pc = 0x82430D60; continue 'dispatch;
	}
	// 82430D80: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82430D84: 40820040  bne 0x82430dc4
	if !ctx.cr[0].eq {
		sub_82430DC4(ctx, base);
		return;
	}
	// 82430D88: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82430D8C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82430D90: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82430D94: 394A3F48  addi r10, r10, 0x3f48
	ctx.r[10].s64 = ctx.r[10].s64 + 16200;
	// 82430D98: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430D9C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430DA0: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82430DA4: 40820014  bne 0x82430db8
	if !ctx.cr[0].eq {
	pc = 0x82430DB8; continue 'dispatch;
	}
	// 82430DA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82430DAC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82430DB0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82430DB4: 409AFFE4  bne cr6, 0x82430d98
	if !ctx.cr[6].eq {
	pc = 0x82430D98; continue 'dispatch;
	}
	// 82430DB8: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82430DBC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82430DC0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430DC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82430DC4 size=8
    let mut pc: u32 = 0x82430DC4;
    'dispatch: loop {
        match pc {
            0x82430DC4 => {
    //   block [0x82430DC4..0x82430DCC)
	// 82430DC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82430DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82430DD0 size=236
    let mut pc: u32 = 0x82430DD0;
    'dispatch: loop {
        match pc {
            0x82430DD0 => {
    //   block [0x82430DD0..0x82430EBC)
	// 82430DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82430DD4: 481042D9  bl 0x825350ac
	ctx.lr = 0x82430DD8;
	sub_82535080(ctx, base);
	// 82430DD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82430DDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82430DE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82430DE4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82430DE8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82430DEC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82430DF0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82430DF4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82430DF8: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 82430DFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82430E00: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82430E04: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82430E08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82430E0C: 2F041000  cmpwi cr6, r4, 0x1000
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4096, &mut ctx.xer);
	// 82430E10: 40980010  bge cr6, 0x82430e20
	if !ctx.cr[6].lt {
	pc = 0x82430E20; continue 'dispatch;
	}
	// 82430E14: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82430E18: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82430E1C: 48000098  b 0x82430eb4
	pc = 0x82430EB4; continue 'dispatch;
	// 82430E20: 38E1005C  addi r7, r1, 0x5c
	ctx.r[7].s64 = ctx.r[1].s64 + 92;
	// 82430E24: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82430E28: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82430E2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82430E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82430E34: 4BFFFC0D  bl 0x82430a40
	ctx.lr = 0x82430E38;
	sub_82430A40(ctx, base);
	// 82430E38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82430E3C: 4082000C  bne 0x82430e48
	if !ctx.cr[0].eq {
	pc = 0x82430E48; continue 'dispatch;
	}
	// 82430E40: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82430E44: 48000070  b 0x82430eb4
	pc = 0x82430EB4; continue 'dispatch;
	// 82430E48: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82430E4C: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82430E50: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82430E54: 4081FFEC  ble 0x82430e40
	if !ctx.cr[0].gt {
	pc = 0x82430E40; continue 'dispatch;
	}
	// 82430E58: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82430E5C: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82430E60: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82430E64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82430E68: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82430E6C: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82430E70: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82430E74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82430E78: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82430E7C: 814100F4  lwz r10, 0xf4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 82430E80: 993D0000  stb r9, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82430E84: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82430E88: 98FB0000  stb r7, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82430E8C: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430E90: 893C0000  lbz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430E94: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82430E98: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82430E9C: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82430EA0: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 82430EA4: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82430EA8: 995A0000  stb r10, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82430EAC: 814100FC  lwz r10, 0xfc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 82430EB0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82430EB4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82430EB8: 48104244  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82430EC0 size=248
    let mut pc: u32 = 0x82430EC0;
    'dispatch: loop {
        match pc {
            0x82430EC0 => {
    //   block [0x82430EC0..0x82430FB8)
	// 82430EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82430EC4: 481041ED  bl 0x825350b0
	ctx.lr = 0x82430EC8;
	sub_82535080(ctx, base);
	// 82430EC8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82430ECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82430ED0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82430ED4: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82430ED8: 3B7F0010  addi r27, r31, 0x10
	ctx.r[27].s64 = ctx.r[31].s64 + 16;
	// 82430EDC: 3BDF000E  addi r30, r31, 0xe
	ctx.r[30].s64 = ctx.r[31].s64 + 14;
	// 82430EE0: 3BBF000F  addi r29, r31, 0xf
	ctx.r[29].s64 = ctx.r[31].s64 + 15;
	// 82430EE4: 3B9F000D  addi r28, r31, 0xd
	ctx.r[28].s64 = ctx.r[31].s64 + 13;
	// 82430EE8: B35F0002  sth r26, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 82430EEC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82430EF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82430EF4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82430EF8: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82430EFC: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 82430F00: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 82430F04: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82430F08: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82430F0C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82430F10: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82430F14: 4BFFFEBD  bl 0x82430dd0
	ctx.lr = 0x82430F18;
	sub_82430DD0(ctx, base);
	// 82430F18: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82430F1C: 4080000C  bge 0x82430f28
	if !ctx.cr[0].lt {
	pc = 0x82430F28; continue 'dispatch;
	}
	// 82430F20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82430F24: 4800008C  b 0x82430fb0
	pc = 0x82430FB0; continue 'dispatch;
	// 82430F28: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430F2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82430F30: 893D0000  lbz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430F34: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82430F38: 811B0000  lwz r8, 0(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430F3C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82430F40: 80FF003C  lwz r7, 0x3c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82430F44: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82430F48: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82430F4C: 80BF0044  lwz r5, 0x44(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82430F50: 889C0000  lbz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430F54: B17F001C  sth r11, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 82430F58: B17F0026  sth r11, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 82430F5C: 2B040008  cmplwi cr6, r4, 8
	ctx.cr[6].compare_u32(ctx.r[4].u32, 8 as u32, &mut ctx.xer);
	// 82430F60: B17F0024  sth r11, 0x24(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 82430F64: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82430F68: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82430F6C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82430F70: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82430F74: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82430F78: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82430F7C: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82430F80: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82430F84: 90FF005C  stw r7, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82430F88: 90DF0060  stw r6, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82430F8C: 90BF0064  stw r5, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82430F90: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82430F94: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82430F98: B07F0098  sth r3, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[3].u16 ) };
	// 82430F9C: 409A000C  bne cr6, 0x82430fa8
	if !ctx.cr[6].eq {
	pc = 0x82430FA8; continue 'dispatch;
	}
	// 82430FA0: B35F009C  sth r26, 0x9c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[26].u16 ) };
	// 82430FA4: 48000008  b 0x82430fac
	pc = 0x82430FAC; continue 'dispatch;
	// 82430FA8: B17F009C  sth r11, 0x9c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u16 ) };
	// 82430FAC: A8610060  lha r3, 0x60(r1)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as i16) as i64;
	// 82430FB0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82430FB4: 4810414C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82430FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82430FB8 size=344
    let mut pc: u32 = 0x82430FB8;
    'dispatch: loop {
        match pc {
            0x82430FB8 => {
    //   block [0x82430FB8..0x82431110)
	// 82430FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82430FBC: 48104101  bl 0x825350bc
	ctx.lr = 0x82430FC0;
	sub_82535080(ctx, base);
	// 82430FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82430FC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82430FC8: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 82430FCC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82430FD0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82430FD4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82430FD8: 409A0104  bne cr6, 0x824310dc
	if !ctx.cr[6].eq {
	pc = 0x824310DC; continue 'dispatch;
	}
	// 82430FDC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82430FE0: 4BFFF0E9  bl 0x824300c8
	ctx.lr = 0x82430FE4;
	sub_824300C8(ctx, base);
	// 82430FE4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82430FE8: 408200F4  bne 0x824310dc
	if !ctx.cr[0].eq {
	pc = 0x824310DC; continue 'dispatch;
	}
	// 82430FEC: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82430FF0: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 82430FF4: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 82430FF8: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82430FFC: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 82431000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82431004: 4E800421  bctrl
	ctx.lr = 0x82431008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82431008: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243100C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82431010: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82431014: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82431018: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8243101C: 40990008  ble cr6, 0x82431024
	if !ctx.cr[6].gt {
	pc = 0x82431024; continue 'dispatch;
	}
	// 82431020: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82431024: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82431028: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8243102C: 40990008  ble cr6, 0x82431034
	if !ctx.cr[6].gt {
	pc = 0x82431034; continue 'dispatch;
	}
	// 82431030: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82431034: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82431038: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8243103C: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 82431040: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82431044: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 82431048: 409A004C  bne cr6, 0x82431094
	if !ctx.cr[6].eq {
	pc = 0x82431094; continue 'dispatch;
	}
	// 8243104C: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82431050: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82431054: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82431058: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243105C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82431060: 4099005C  ble cr6, 0x824310bc
	if !ctx.cr[6].gt {
	pc = 0x824310BC; continue 'dispatch;
	}
	// 82431064: 7CE94050  subf r7, r9, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82431068: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8243106C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82431070: A0CA0000  lhz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82431074: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82431078: 7CC74B2E  sthx r6, r7, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u16) };
	// 8243107C: A0CA0002  lhz r6, 2(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82431080: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82431084: B0C90000  sth r6, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 82431088: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8243108C: 4082FFE4  bne 0x82431070
	if !ctx.cr[0].eq {
	pc = 0x82431070; continue 'dispatch;
	}
	// 82431090: 4800002C  b 0x824310bc
	pc = 0x824310BC; continue 'dispatch;
	// 82431094: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82431098: 40990024  ble cr6, 0x824310bc
	if !ctx.cr[6].gt {
	pc = 0x824310BC; continue 'dispatch;
	}
	// 8243109C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 824310A0: 7D08E850  subf r8, r8, r29
	ctx.r[8].s64 = ctx.r[29].s64 - ctx.r[8].s64;
	// 824310A4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 824310A8: 7CE8522E  lhzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824310AC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824310B0: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 824310B4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 824310B8: 4082FFF0  bne 0x824310a8
	if !ctx.cr[0].eq {
	pc = 0x824310A8; continue 'dispatch;
	}
	// 824310BC: 895E000E  lbz r10, 0xe(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 824310C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 824310C4: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 824310C8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824310CC: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 824310D0: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824310D4: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824310D8: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 824310DC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824310E0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824310E4: 409A0024  bne cr6, 0x82431108
	if !ctx.cr[6].eq {
	pc = 0x82431108; continue 'dispatch;
	}
	// 824310E8: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824310EC: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 824310F0: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 824310F4: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 824310F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824310FC: 4E800421  bctrl
	ctx.lr = 0x82431100;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82431100: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82431104: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82431108: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243110C: 48104000  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431110 size=352
    let mut pc: u32 = 0x82431110;
    'dispatch: loop {
        match pc {
            0x82431110 => {
    //   block [0x82431110..0x82431270)
	// 82431110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431114: 48103FA9  bl 0x825350bc
	ctx.lr = 0x82431118;
	sub_82535080(ctx, base);
	// 82431118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243111C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82431120: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 82431124: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82431128: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243112C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82431130: 409A010C  bne cr6, 0x8243123c
	if !ctx.cr[6].eq {
	pc = 0x8243123C; continue 'dispatch;
	}
	// 82431134: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82431138: 4BFFEF91  bl 0x824300c8
	ctx.lr = 0x8243113C;
	sub_824300C8(ctx, base);
	// 8243113C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82431140: 408200FC  bne 0x8243123c
	if !ctx.cr[0].eq {
	pc = 0x8243123C; continue 'dispatch;
	}
	// 82431144: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82431148: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 8243114C: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 82431150: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82431154: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 82431158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243115C: 4E800421  bctrl
	ctx.lr = 0x82431160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82431160: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82431164: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82431168: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243116C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82431170: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82431174: 40990008  ble cr6, 0x8243117c
	if !ctx.cr[6].gt {
	pc = 0x8243117C; continue 'dispatch;
	}
	// 82431178: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8243117C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82431180: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82431184: 40990008  ble cr6, 0x8243118c
	if !ctx.cr[6].gt {
	pc = 0x8243118C; continue 'dispatch;
	}
	// 82431188: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8243118C: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 82431190: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82431194: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82431198: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 8243119C: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 824311A0: 409A0054  bne cr6, 0x824311f4
	if !ctx.cr[6].eq {
	pc = 0x824311F4; continue 'dispatch;
	}
	// 824311A4: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824311A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824311AC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824311B0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824311B4: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824311B8: 40990068  ble cr6, 0x82431220
	if !ctx.cr[6].gt {
	pc = 0x82431220; continue 'dispatch;
	}
	// 824311BC: 7D1D3850  subf r8, r29, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[29].s64;
	// 824311C0: 7CFD4850  subf r7, r29, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 824311C4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 824311C8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 824311CC: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824311D0: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824311D4: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 824311D8: 7CC8532E  sthx r6, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[6].u16) };
	// 824311DC: 88CA0001  lbz r6, 1(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 824311E0: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 824311E4: 7CC7532E  sthx r6, r7, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[6].u16) };
	// 824311E8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 824311EC: 4082FFE0  bne 0x824311cc
	if !ctx.cr[0].eq {
	pc = 0x824311CC; continue 'dispatch;
	}
	// 824311F0: 48000030  b 0x82431220
	pc = 0x82431220; continue 'dispatch;
	// 824311F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824311F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824311FC: 40990024  ble cr6, 0x82431220
	if !ctx.cr[6].gt {
	pc = 0x82431220; continue 'dispatch;
	}
	// 82431200: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82431204: 7D0AE8AE  lbzx r8, r10, r29
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82431208: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8243120C: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 82431210: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82431214: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82431218: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8243121C: 4198FFE8  blt cr6, 0x82431204
	if ctx.cr[6].lt {
	pc = 0x82431204; continue 'dispatch;
	}
	// 82431220: 895E000E  lbz r10, 0xe(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 82431224: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82431228: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8243122C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82431230: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82431234: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82431238: 915E0094  stw r10, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 8243123C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82431240: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82431244: 409A0024  bne cr6, 0x82431268
	if !ctx.cr[6].eq {
	pc = 0x82431268; continue 'dispatch;
	}
	// 82431248: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8243124C: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 82431250: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82431254: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82431258: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243125C: 4E800421  bctrl
	ctx.lr = 0x82431260;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82431260: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82431264: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82431268: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243126C: 48103EA0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431270 size=16
    let mut pc: u32 = 0x82431270;
    'dispatch: loop {
        match pc {
            0x82431270 => {
    //   block [0x82431270..0x82431280)
	// 82431270: A163009C  lhz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82431274: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82431278: 409A0008  bne cr6, 0x82431280
	if !ctx.cr[6].eq {
		sub_82431280(ctx, base);
		return;
	}
	// 8243127C: 4BFFFE94  b 0x82431110
	sub_82431110(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431280 size=4
    let mut pc: u32 = 0x82431280;
    'dispatch: loop {
        match pc {
            0x82431280 => {
    //   block [0x82431280..0x82431284)
	// 82431280: 4BFFFD38  b 0x82430fb8
	sub_82430FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82431288 size=272
    let mut pc: u32 = 0x82431288;
    'dispatch: loop {
        match pc {
            0x82431288 => {
    //   block [0x82431288..0x82431398)
	// 82431288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243128C: 48103E31  bl 0x825350bc
	ctx.lr = 0x82431290;
	sub_82535080(ctx, base);
	// 82431290: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82431294: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 82431298: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8243129C: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 824312A0: 888B0007  lbz r4, 7(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 824312A4: 5484203E  rotlwi r4, r4, 4
	ctx.r[4].u64 = ((ctx.r[4].u32).rotate_left(4)) as u64;
	// 824312A8: B0850000  sth r4, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 824312AC: 88AB0009  lbz r5, 9(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 824312B0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824312B4: 54A507BE  clrlwi r5, r5, 0x1e
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 824312B8: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 824312BC: 98A90000  stb r5, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 824312C0: A0AB002A  lhz r5, 0x2a(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(42 as u32) ) } as u64;
	// 824312C4: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 824312C8: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824312CC: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824312D0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 824312D4: 41980064  blt cr6, 0x82431338
	if ctx.cr[6].lt {
	pc = 0x82431338; continue 'dispatch;
	}
	// 824312D8: 419A0038  beq cr6, 0x82431310
	if ctx.cr[6].eq {
	pc = 0x82431310; continue 'dispatch;
	}
	// 824312DC: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 824312E0: 4098008C  bge cr6, 0x8243136c
	if !ctx.cr[6].lt {
	pc = 0x8243136C; continue 'dispatch;
	}
	// 824312E4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 824312E8: 99470000  stb r10, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824312EC: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824312F0: 99480000  stb r10, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824312F4: 93C50000  stw r30, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824312F8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824312FC: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82431300: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82431304: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82431308: B3CA0000  sth r30, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 8243130C: 48000060  b 0x8243136c
	pc = 0x8243136C; continue 'dispatch;
	// 82431310: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82431314: 99470000  stb r10, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82431318: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243131C: 99480000  stb r10, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82431320: 93E50000  stw r31, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82431324: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82431328: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243132C: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82431330: B3EA0000  sth r31, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 82431334: 48000038  b 0x8243136c
	pc = 0x8243136C; continue 'dispatch;
	// 82431338: 9BA70000  stb r29, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8243133C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82431340: 89290000  lbz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82431344: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82431348: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8243134C: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82431350: 93E50000  stw r31, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82431354: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82431358: 7D290E70  srawi r9, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 8243135C: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82431360: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82431364: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82431368: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8243136C: 9BC80000  stb r30, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82431370: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82431374: 93E50000  stw r31, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82431378: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243137C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82431380: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82431384: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82431388: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243138C: 9BA70000  stb r29, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82431390: 99460000  stb r10, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82431394: 48103D78  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431398 size=224
    let mut pc: u32 = 0x82431398;
    'dispatch: loop {
        match pc {
            0x82431398 => {
    //   block [0x82431398..0x82431478)
	// 82431398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243139C: 48103D1D  bl 0x825350b8
	ctx.lr = 0x824313A0;
	sub_82535080(ctx, base);
	// 824313A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824313A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824313A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824313AC: 393F009C  addi r9, r31, 0x9c
	ctx.r[9].s64 = ctx.r[31].s64 + 156;
	// 824313B0: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 824313B4: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 824313B8: 3BDF000E  addi r30, r31, 0xe
	ctx.r[30].s64 = ctx.r[31].s64 + 14;
	// 824313BC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824313C0: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 824313C4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824313C8: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 824313CC: 391F000F  addi r8, r31, 0xf
	ctx.r[8].s64 = ctx.r[31].s64 + 15;
	// 824313D0: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 824313D4: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 824313D8: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 824313DC: 38FF000D  addi r7, r31, 0xd
	ctx.r[7].s64 = ctx.r[31].s64 + 13;
	// 824313E0: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 824313E4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 824313E8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 824313EC: 4BFFFE9D  bl 0x82431288
	ctx.lr = 0x824313F0;
	sub_82431288(ctx, base);
	// 824313F0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824313F4: 4080000C  bge 0x82431400
	if !ctx.cr[0].lt {
	pc = 0x82431400; continue 'dispatch;
	}
	// 824313F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824313FC: 48000074  b 0x82431470
	pc = 0x82431470; continue 'dispatch;
	// 82431400: 89280000  lbz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82431404: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82431408: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243140C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82431410: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82431414: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82431418: 80FF003C  lwz r7, 0x3c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8243141C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82431420: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82431424: 80BF0044  lwz r5, 0x44(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82431428: A8610070  lha r3, 0x70(r1)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as i16) as i64;
	// 8243142C: B17F001C  sth r11, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 82431430: B17F0026  sth r11, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 82431434: B17F0024  sth r11, 0x24(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 82431438: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8243143C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82431440: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82431444: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82431448: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8243144C: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82431450: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82431454: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82431458: 90FF005C  stw r7, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8243145C: 90DF0060  stw r6, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82431460: 90BF0064  stw r5, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82431464: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82431468: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 8243146C: B09F0098  sth r4, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[4].u16 ) };
	// 82431470: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82431474: 48103C94  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431478 size=56
    let mut pc: u32 = 0x82431478;
    'dispatch: loop {
        match pc {
            0x82431478 => {
    //   block [0x82431478..0x824314B0)
	// 82431478: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243147C: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 82431480: 396B3F58  addi r11, r11, 0x3f58
	ctx.r[11].s64 = ctx.r[11].s64 + 16216;
	// 82431484: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82431488: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243148C: 7D485051  subf. r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82431490: 40820014  bne 0x824314a4
	if !ctx.cr[0].eq {
	pc = 0x824314A4; continue 'dispatch;
	}
	// 82431494: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82431498: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243149C: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824314A0: 409AFFE4  bne cr6, 0x82431484
	if !ctx.cr[6].eq {
	pc = 0x82431484; continue 'dispatch;
	}
	// 824314A4: 7D4B0034  cntlzw r11, r10
	ctx.r[11].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 824314A8: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824314AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824314B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824314B0 size=340
    let mut pc: u32 = 0x824314B0;
    'dispatch: loop {
        match pc {
            0x824314B0 => {
    //   block [0x824314B0..0x82431604)
	// 824314B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824314B4: 48103BE9  bl 0x8253509c
	ctx.lr = 0x824314B8;
	sub_82535080(ctx, base);
	// 824314B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824314BC: ABA60000  lha r29, 0(r6)
	ctx.r[29].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 824314C0: AAE60002  lha r23, 2(r6)
	ctx.r[23].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 824314C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824314C8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824314CC: 40990128  ble cr6, 0x824315f4
	if !ctx.cr[6].gt {
	pc = 0x824315F4; continue 'dispatch;
	}
	// 824314D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824314D4: 3AAB3F60  addi r21, r11, 0x3f60
	ctx.r[21].s64 = ctx.r[11].s64 + 16224;
	// 824314D8: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824314DC: 557E0021  rlwinm. r30, r11, 0, 0, 0x10
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824314E0: 40820120  bne 0x82431600
	if !ctx.cr[0].eq {
	pc = 0x82431600; continue 'dispatch;
	}
	// 824314E4: ABC90000  lha r30, 0(r9)
	ctx.r[30].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 824314E8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824314EC: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 824314F0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 824314F4: 7FCB5A78  xor r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 ^ ctx.r[11].u64;
	// 824314F8: 7D5E0734  extsh r30, r10
	ctx.r[30].s64 = ctx.r[10].s16 as i64;
	// 824314FC: 556B04FE  clrlwi r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 82431500: 7FDCF1D6  mullw r30, r28, r30
	ctx.r[30].s64 = (ctx.r[28].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82431504: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82431508: 7D1A0734  extsh r26, r8
	ctx.r[26].s64 = ctx.r[8].s16 as i64;
	// 8243150C: AB810056  lha r28, 0x56(r1)
	ctx.r[28].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as i16) as i64;
	// 82431510: 7CF90734  extsh r25, r7
	ctx.r[25].s64 = ctx.r[7].s16 as i64;
	// 82431514: 7FDEE214  add r30, r30, r28
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 82431518: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 8243151C: 7D780734  extsh r24, r11
	ctx.r[24].s64 = ctx.r[11].s16 as i64;
	// 82431520: 57DE047E  clrlwi r30, r30, 0x11
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0x00007FFFu64;
	// 82431524: B3C90000  sth r30, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82431528: 7F9AB9D6  mullw r28, r26, r23
	ctx.r[28].s64 = (ctx.r[26].s32 as i64) * (ctx.r[23].s32 as i64);
	// 8243152C: 8BDF0000  lbz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82431530: 7D79E9D6  mullw r11, r25, r29
	ctx.r[11].s64 = (ctx.r[25].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82431534: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82431538: 7FDE0774  extsb r30, r30
	ctx.r[30].s64 = ctx.r[30].s8 as i64;
	// 8243153C: 7D6B6670  srawi r11, r11, 0xc
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 12) as i64;
	// 82431540: 7FDC2670  srawi r28, r30, 4
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[30].s32 >> 4) as i64;
	// 82431544: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82431548: 7F9CC1D6  mullw r28, r28, r24
	ctx.r[28].s64 = (ctx.r[28].s32 as i64) * (ctx.r[24].s32 as i64);
	// 8243154C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82431550: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82431554: 4199000C  bgt cr6, 0x82431560
	if ctx.cr[6].gt {
	pc = 0x82431560; continue 'dispatch;
	}
	// 82431558: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 8243155C: 40980020  bge cr6, 0x8243157c
	if !ctx.cr[6].lt {
	pc = 0x8243157C; continue 'dispatch;
	}
	// 82431560: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 82431564: 4098000C  bge cr6, 0x82431570
	if !ctx.cr[6].lt {
	pc = 0x82431570; continue 'dispatch;
	}
	// 82431568: 39608000  li r11, -0x8000
	ctx.r[11].s64 = -32768;
	// 8243156C: 48000010  b 0x8243157c
	pc = 0x8243157C; continue 'dispatch;
	// 82431570: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82431574: 40990008  ble cr6, 0x8243157c
	if !ctx.cr[6].gt {
	pc = 0x8243157C; continue 'dispatch;
	}
	// 82431578: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 8243157C: 57DE16BA  rlwinm r30, r30, 2, 0x1a, 0x1d
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0x3FFFFFFFu64;
	// 82431580: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82431584: 3B650002  addi r27, r5, 2
	ctx.r[27].s64 = ctx.r[5].s64 + 2;
	// 82431588: 7F9959D6  mullw r28, r25, r11
	ctx.r[28].s64 = (ctx.r[25].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8243158C: 7FDEA82E  lwzx r30, r30, r21
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82431590: 7CBAE9D6  mullw r5, r26, r29
	ctx.r[5].s64 = (ctx.r[26].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82431594: 7CBC2A14  add r5, r28, r5
	ctx.r[5].u64 = ctx.r[28].u64 + ctx.r[5].u64;
	// 82431598: 7FDEC1D6  mullw r30, r30, r24
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[24].s32 as i64);
	// 8243159C: 7CA56670  srawi r5, r5, 0xc
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 12) as i64;
	// 824315A0: 7CA5F214  add r5, r5, r30
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[30].u64;
	// 824315A4: 2F057FFF  cmpwi cr6, r5, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[5].s32, 32767, &mut ctx.xer);
	// 824315A8: 4199000C  bgt cr6, 0x824315b4
	if ctx.cr[6].gt {
	pc = 0x824315B4; continue 'dispatch;
	}
	// 824315AC: 2F058000  cmpwi cr6, r5, -0x8000
	ctx.cr[6].compare_i32(ctx.r[5].s32, -32768, &mut ctx.xer);
	// 824315B0: 40980020  bge cr6, 0x824315d0
	if !ctx.cr[6].lt {
	pc = 0x824315D0; continue 'dispatch;
	}
	// 824315B4: 2F058000  cmpwi cr6, r5, -0x8000
	ctx.cr[6].compare_i32(ctx.r[5].s32, -32768, &mut ctx.xer);
	// 824315B8: 4098000C  bge cr6, 0x824315c4
	if !ctx.cr[6].lt {
	pc = 0x824315C4; continue 'dispatch;
	}
	// 824315BC: 38A08000  li r5, -0x8000
	ctx.r[5].s64 = -32768;
	// 824315C0: 48000010  b 0x824315d0
	pc = 0x824315D0; continue 'dispatch;
	// 824315C4: 2F057FFF  cmpwi cr6, r5, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[5].s32, 32767, &mut ctx.xer);
	// 824315C8: 40990008  ble cr6, 0x824315d0
	if !ctx.cr[6].gt {
	pc = 0x824315D0; continue 'dispatch;
	}
	// 824315CC: 38A07FFF  li r5, 0x7fff
	ctx.r[5].s64 = 32767;
	// 824315D0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824315D4: B0BB0000  sth r5, 0(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 824315D8: 36D6FFFF  addic. r22, r22, -1
	ctx.xer.ca = (ctx.r[22].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 824315DC: 7D775B78  mr r23, r11
	ctx.r[23].u64 = ctx.r[11].u64;
	// 824315E0: 38BB0002  addi r5, r27, 2
	ctx.r[5].s64 = ctx.r[27].s64 + 2;
	// 824315E4: 4082FF44  bne 0x82431528
	if !ctx.cr[0].eq {
	pc = 0x82431528; continue 'dispatch;
	}
	// 824315E8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 824315EC: 7F032000  cmpw cr6, r3, r4
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[4].s32, &mut ctx.xer);
	// 824315F0: 4198FEE8  blt cr6, 0x824314d8
	if ctx.cr[6].lt {
	pc = 0x824314D8; continue 'dispatch;
	}
	// 824315F4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824315F8: B3A60000  sth r29, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 824315FC: B2E60002  sth r23, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[23].u16 ) };
	// 82431600: 48103AEC  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82431608 size=760
    let mut pc: u32 = 0x82431608;
    'dispatch: loop {
        match pc {
            0x82431608 => {
    //   block [0x82431608..0x82431900)
	// 82431608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243160C: 48103A75  bl 0x82535080
	ctx.lr = 0x82431610;
	sub_82535080(ctx, base);
	// 82431610: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82431614: AAA60002  lha r21, 2(r6)
	ctx.r[21].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 82431618: 7C8B0E70  srawi r11, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 8243161C: AAE80000  lha r23, 0(r8)
	ctx.r[23].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82431620: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82431624: A8660000  lha r3, 0(r6)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82431628: 7DCB0195  addze. r14, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[14].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[14].s32, 0, &mut ctx.xer);
	// 8243162C: AAC80002  lha r22, 2(r8)
	ctx.r[22].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 82431630: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82431634: 9321FF60  stw r25, -0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.r[25].u32 ) };
	// 82431638: 408102A8  ble 0x824318e0
	if !ctx.cr[0].gt {
	pc = 0x824318E0; continue 'dispatch;
	}
	// 8243163C: 3FE08201  lis r31, -0x7dff
	ctx.r[31].s64 = -2113863680;
	// 82431640: 82210054  lwz r17, 0x54(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82431644: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 82431648: 39FF3F60  addi r15, r31, 0x3f60
	ctx.r[15].s64 = ctx.r[31].s64 + 16224;
	// 8243164C: 48000008  b 0x82431654
	pc = 0x82431654; continue 'dispatch;
	// 82431650: 8321FF60  lwz r25, -0xa0(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) } as u64;
	// 82431654: A3FE0000  lhz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82431658: 57FD0021  rlwinm. r29, r31, 0, 0, 0x10
	ctx.r[29].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8243165C: 4082029C  bne 0x824318f8
	if !ctx.cr[0].eq {
	pc = 0x824318F8; continue 'dispatch;
	}
	// 82431660: AB710000  lha r27, 0(r17)
	ctx.r[27].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82431664: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 82431668: ABA1005E  lha r29, 0x5e(r1)
	ctx.r[29].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as i16) as i64;
	// 8243166C: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 82431670: AB810066  lha r28, 0x66(r1)
	ctx.r[28].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as i16) as i64;
	// 82431674: 7F5FFA78  xor r31, r26, r31
	ctx.r[31].u64 = ctx.r[26].u64 ^ ctx.r[31].u64;
	// 82431678: 57FF04FE  clrlwi r31, r31, 0x13
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0x00001FFFu64;
	// 8243167C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82431680: 7FFA0734  extsh r26, r31
	ctx.r[26].s64 = ctx.r[31].s16 as i64;
	// 82431684: 7FFBE9D6  mullw r31, r27, r29
	ctx.r[31].s64 = (ctx.r[27].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82431688: 7FFFE214  add r31, r31, r28
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 8243168C: 57FF047E  clrlwi r31, r31, 0x11
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0x00007FFFu64;
	// 82431690: B3F10000  sth r31, 0(r17)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 82431694: A37E0012  lhz r27, 0x12(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(18 as u32) ) } as u64;
	// 82431698: 57780021  rlwinm. r24, r27, 0, 0, 0x10
	ctx.r[24].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8243169C: 4082025C  bne 0x824318f8
	if !ctx.cr[0].eq {
	pc = 0x824318F8; continue 'dispatch;
	}
	// 824316A0: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 824316A4: 3A9E0002  addi r20, r30, 2
	ctx.r[20].s64 = ctx.r[30].s64 + 2;
	// 824316A8: 7FDFE9D6  mullw r30, r31, r29
	ctx.r[30].s64 = (ctx.r[31].s32 as i64) * (ctx.r[29].s32 as i64);
	// 824316AC: 7F790734  extsh r25, r27
	ctx.r[25].s64 = ctx.r[27].s16 as i64;
	// 824316B0: 7FDEE214  add r30, r30, r28
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 824316B4: 7F39FA78  xor r25, r25, r31
	ctx.r[25].u64 = ctx.r[25].u64 ^ ctx.r[31].u64;
	// 824316B8: 57DE047E  clrlwi r30, r30, 0x11
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0x00007FFFu64;
	// 824316BC: 573F04FE  clrlwi r31, r25, 0x13
	ctx.r[31].u64 = ctx.r[25].u32 as u64 & 0x00001FFFu64;
	// 824316C0: 7F530734  extsh r19, r26
	ctx.r[19].s64 = ctx.r[26].s16 as i64;
	// 824316C4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824316C8: 7D5B0734  extsh r27, r10
	ctx.r[27].s64 = ctx.r[10].s16 as i64;
	// 824316CC: 7D3A0734  extsh r26, r9
	ctx.r[26].s64 = ctx.r[9].s16 as i64;
	// 824316D0: B3D10000  sth r30, 0(r17)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 824316D4: 3A000010  li r16, 0x10
	ctx.r[16].s64 = 16;
	// 824316D8: 7FF20734  extsh r18, r31
	ctx.r[18].s64 = ctx.r[31].s16 as i64;
	// 824316DC: 8BD40012  lbz r30, 0x12(r20)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[20].u32.wrapping_add(18 as u32) ) } as u64;
	// 824316E0: 7FFA59D6  mullw r31, r26, r11
	ctx.r[31].s64 = (ctx.r[26].s32 as i64) * (ctx.r[11].s32 as i64);
	// 824316E4: 7FD90774  extsb r25, r30
	ctx.r[25].s64 = ctx.r[30].s8 as i64;
	// 824316E8: 8BB40000  lbz r29, 0(r20)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 824316EC: 7FDBA9D6  mullw r30, r27, r21
	ctx.r[30].s64 = (ctx.r[27].s32 as i64) * (ctx.r[21].s32 as i64);
	// 824316F0: 7FFFF214  add r31, r31, r30
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 824316F4: 7FB80774  extsb r24, r29
	ctx.r[24].s64 = ctx.r[29].s8 as i64;
	// 824316F8: 7F3C2670  srawi r28, r25, 4
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[25].s32 >> 4) as i64;
	// 824316FC: 7FFF6670  srawi r31, r31, 0xc
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 12) as i64;
	// 82431700: 7F1E2670  srawi r30, r24, 4
	ctx.xer.ca = (ctx.r[24].s32 < 0) && ((ctx.r[24].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[24].s32 >> 4) as i64;
	// 82431704: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 82431708: 7FDE99D6  mullw r30, r30, r19
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[19].s32 as i64);
	// 8243170C: 7FFFF214  add r31, r31, r30
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82431710: 2F1F7FFF  cmpwi cr6, r31, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[31].s32, 32767, &mut ctx.xer);
	// 82431714: 4199000C  bgt cr6, 0x82431720
	if ctx.cr[6].gt {
	pc = 0x82431720; continue 'dispatch;
	}
	// 82431718: 2F1F8000  cmpwi cr6, r31, -0x8000
	ctx.cr[6].compare_i32(ctx.r[31].s32, -32768, &mut ctx.xer);
	// 8243171C: 40980020  bge cr6, 0x8243173c
	if !ctx.cr[6].lt {
	pc = 0x8243173C; continue 'dispatch;
	}
	// 82431720: 2F1F8000  cmpwi cr6, r31, -0x8000
	ctx.cr[6].compare_i32(ctx.r[31].s32, -32768, &mut ctx.xer);
	// 82431724: 4098000C  bge cr6, 0x82431730
	if !ctx.cr[6].lt {
	pc = 0x82431730; continue 'dispatch;
	}
	// 82431728: 3BE08000  li r31, -0x8000
	ctx.r[31].s64 = -32768;
	// 8243172C: 48000010  b 0x8243173c
	pc = 0x8243173C; continue 'dispatch;
	// 82431730: 2F1F7FFF  cmpwi cr6, r31, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[31].s32, 32767, &mut ctx.xer);
	// 82431734: 40990008  ble cr6, 0x8243173c
	if !ctx.cr[6].gt {
	pc = 0x8243173C; continue 'dispatch;
	}
	// 82431738: 3BE07FFF  li r31, 0x7fff
	ctx.r[31].s64 = 32767;
	// 8243173C: 7FBBB1D6  mullw r29, r27, r22
	ctx.r[29].s64 = (ctx.r[27].s32 as i64) * (ctx.r[22].s32 as i64);
	// 82431740: 7FDAB9D6  mullw r30, r26, r23
	ctx.r[30].s64 = (ctx.r[26].s32 as i64) * (ctx.r[23].s32 as i64);
	// 82431744: 7FDEEA14  add r30, r30, r29
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82431748: 7FB2E1D6  mullw r29, r18, r28
	ctx.r[29].s64 = (ctx.r[18].s32 as i64) * (ctx.r[28].s32 as i64);
	// 8243174C: 7FDE6670  srawi r30, r30, 0xc
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 12) as i64;
	// 82431750: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82431754: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82431758: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 8243175C: 4199000C  bgt cr6, 0x82431768
	if ctx.cr[6].gt {
	pc = 0x82431768; continue 'dispatch;
	}
	// 82431760: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 82431764: 40980020  bge cr6, 0x82431784
	if !ctx.cr[6].lt {
	pc = 0x82431784; continue 'dispatch;
	}
	// 82431768: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 8243176C: 4098000C  bge cr6, 0x82431778
	if !ctx.cr[6].lt {
	pc = 0x82431778; continue 'dispatch;
	}
	// 82431770: 39608000  li r11, -0x8000
	ctx.r[11].s64 = -32768;
	// 82431774: 48000010  b 0x82431784
	pc = 0x82431784; continue 'dispatch;
	// 82431778: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 8243177C: 40990008  ble cr6, 0x82431784
	if !ctx.cr[6].gt {
	pc = 0x82431784; continue 'dispatch;
	}
	// 82431780: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 82431784: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82431788: 7D765B78  mr r22, r11
	ctx.r[22].u64 = ctx.r[11].u64;
	// 8243178C: 1FDE0007  mulli r30, r30, 7
	ctx.r[30].s64 = ctx.r[30].s64 * 7;
	// 82431790: 7D7E1BD6  divw r11, r30, r3
	ctx.r[11].s32 = ctx.r[30].s32 / ctx.r[3].s32;
	// 82431794: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82431798: 4199000C  bgt cr6, 0x824317a4
	if ctx.cr[6].gt {
	pc = 0x824317A4; continue 'dispatch;
	}
	// 8243179C: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 824317A0: 40980020  bge cr6, 0x824317c0
	if !ctx.cr[6].lt {
	pc = 0x824317C0; continue 'dispatch;
	}
	// 824317A4: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 824317A8: 4098000C  bge cr6, 0x824317b4
	if !ctx.cr[6].lt {
	pc = 0x824317B4; continue 'dispatch;
	}
	// 824317AC: 39608000  li r11, -0x8000
	ctx.r[11].s64 = -32768;
	// 824317B0: 48000010  b 0x824317c0
	pc = 0x824317C0; continue 'dispatch;
	// 824317B4: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 824317B8: 40990008  ble cr6, 0x824317c0
	if !ctx.cr[6].gt {
	pc = 0x824317C0; continue 'dispatch;
	}
	// 824317BC: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 824317C0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824317C4: 571816BA  rlwinm r24, r24, 2, 0x1a, 0x1d
	ctx.r[24].u64 = ctx.r[24].u32 as u64 & 0x3FFFFFFFu64;
	// 824317C8: 7FBBE1D6  mullw r29, r27, r28
	ctx.r[29].s64 = (ctx.r[27].s32 as i64) * (ctx.r[28].s32 as i64);
	// 824317CC: B1670000  sth r11, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824317D0: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824317D4: 7D78782E  lwzx r11, r24, r15
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 824317D8: 7FDAF9D6  mullw r30, r26, r31
	ctx.r[30].s64 = (ctx.r[26].s32 as i64) * (ctx.r[31].s32 as i64);
	// 824317DC: 7FDEEA14  add r30, r30, r29
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 824317E0: 573D16BA  rlwinm r29, r25, 2, 0x1a, 0x1d
	ctx.r[29].u64 = ctx.r[25].u32 as u64 & 0x3FFFFFFFu64;
	// 824317E4: 7D6B99D6  mullw r11, r11, r19
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[19].s32 as i64);
	// 824317E8: 7FBD782E  lwzx r29, r29, r15
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 824317EC: 7FDE6670  srawi r30, r30, 0xc
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 12) as i64;
	// 824317F0: 3B870002  addi r28, r7, 2
	ctx.r[28].s64 = ctx.r[7].s64 + 2;
	// 824317F4: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 824317F8: 3B250002  addi r25, r5, 2
	ctx.r[25].s64 = ctx.r[5].s64 + 2;
	// 824317FC: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82431800: 4199000C  bgt cr6, 0x8243180c
	if ctx.cr[6].gt {
	pc = 0x8243180C; continue 'dispatch;
	}
	// 82431804: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 82431808: 40980020  bge cr6, 0x82431828
	if !ctx.cr[6].lt {
	pc = 0x82431828; continue 'dispatch;
	}
	// 8243180C: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 82431810: 4098000C  bge cr6, 0x8243181c
	if !ctx.cr[6].lt {
	pc = 0x8243181C; continue 'dispatch;
	}
	// 82431814: 39608000  li r11, -0x8000
	ctx.r[11].s64 = -32768;
	// 82431818: 48000010  b 0x82431828
	pc = 0x82431828; continue 'dispatch;
	// 8243181C: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82431820: 40990008  ble cr6, 0x82431828
	if !ctx.cr[6].gt {
	pc = 0x82431828; continue 'dispatch;
	}
	// 82431824: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 82431828: 7CBBB9D6  mullw r5, r27, r23
	ctx.r[5].s64 = (ctx.r[27].s32 as i64) * (ctx.r[23].s32 as i64);
	// 8243182C: 7CFAB1D6  mullw r7, r26, r22
	ctx.r[7].s64 = (ctx.r[26].s32 as i64) * (ctx.r[22].s32 as i64);
	// 82431830: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 82431834: 7CB2E9D6  mullw r5, r18, r29
	ctx.r[5].s64 = (ctx.r[18].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82431838: 7CE76670  srawi r7, r7, 0xc
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 12) as i64;
	// 8243183C: 7FF5FB78  mr r21, r31
	ctx.r[21].u64 = ctx.r[31].u64;
	// 82431840: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 82431844: 2F077FFF  cmpwi cr6, r7, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[7].s32, 32767, &mut ctx.xer);
	// 82431848: 4199000C  bgt cr6, 0x82431854
	if ctx.cr[6].gt {
	pc = 0x82431854; continue 'dispatch;
	}
	// 8243184C: 2F078000  cmpwi cr6, r7, -0x8000
	ctx.cr[6].compare_i32(ctx.r[7].s32, -32768, &mut ctx.xer);
	// 82431850: 40980020  bge cr6, 0x82431870
	if !ctx.cr[6].lt {
	pc = 0x82431870; continue 'dispatch;
	}
	// 82431854: 2F078000  cmpwi cr6, r7, -0x8000
	ctx.cr[6].compare_i32(ctx.r[7].s32, -32768, &mut ctx.xer);
	// 82431858: 4098000C  bge cr6, 0x82431864
	if !ctx.cr[6].lt {
	pc = 0x82431864; continue 'dispatch;
	}
	// 8243185C: 38E08000  li r7, -0x8000
	ctx.r[7].s64 = -32768;
	// 82431860: 48000010  b 0x82431870
	pc = 0x82431870; continue 'dispatch;
	// 82431864: 2F077FFF  cmpwi cr6, r7, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[7].s32, 32767, &mut ctx.xer);
	// 82431868: 40990008  ble cr6, 0x82431870
	if !ctx.cr[6].gt {
	pc = 0x82431870; continue 'dispatch;
	}
	// 8243186C: 38E07FFF  li r7, 0x7fff
	ctx.r[7].s64 = 32767;
	// 82431870: 7CA75A14  add r5, r7, r11
	ctx.r[5].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82431874: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82431878: 1CA50007  mulli r5, r5, 7
	ctx.r[5].s64 = ctx.r[5].s64 * 7;
	// 8243187C: 7CA51BD6  divw r5, r5, r3
	ctx.r[5].s32 = ctx.r[5].s32 / ctx.r[3].s32;
	// 82431880: 2F057FFF  cmpwi cr6, r5, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[5].s32, 32767, &mut ctx.xer);
	// 82431884: 4199000C  bgt cr6, 0x82431890
	if ctx.cr[6].gt {
	pc = 0x82431890; continue 'dispatch;
	}
	// 82431888: 2F058000  cmpwi cr6, r5, -0x8000
	ctx.cr[6].compare_i32(ctx.r[5].s32, -32768, &mut ctx.xer);
	// 8243188C: 40980020  bge cr6, 0x824318ac
	if !ctx.cr[6].lt {
	pc = 0x824318AC; continue 'dispatch;
	}
	// 82431890: 2F058000  cmpwi cr6, r5, -0x8000
	ctx.cr[6].compare_i32(ctx.r[5].s32, -32768, &mut ctx.xer);
	// 82431894: 4098000C  bge cr6, 0x824318a0
	if !ctx.cr[6].lt {
	pc = 0x824318A0; continue 'dispatch;
	}
	// 82431898: 38A08000  li r5, -0x8000
	ctx.r[5].s64 = -32768;
	// 8243189C: 48000010  b 0x824318ac
	pc = 0x824318AC; continue 'dispatch;
	// 824318A0: 2F057FFF  cmpwi cr6, r5, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[5].s32, 32767, &mut ctx.xer);
	// 824318A4: 40990008  ble cr6, 0x824318ac
	if !ctx.cr[6].gt {
	pc = 0x824318AC; continue 'dispatch;
	}
	// 824318A8: 38A07FFF  li r5, 0x7fff
	ctx.r[5].s64 = 32767;
	// 824318AC: 7CA50734  extsh r5, r5
	ctx.r[5].s64 = ctx.r[5].s16 as i64;
	// 824318B0: 3610FFFF  addic. r16, r16, -1
	ctx.xer.ca = (ctx.r[16].u32 > (!(-1 as u32)));
	ctx.r[16].s64 = ctx.r[16].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[16].s32, 0, &mut ctx.xer);
	// 824318B4: 38FC0002  addi r7, r28, 2
	ctx.r[7].s64 = ctx.r[28].s64 + 2;
	// 824318B8: B0BC0000  sth r5, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 824318BC: B0B90000  sth r5, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 824318C0: 38B90002  addi r5, r25, 2
	ctx.r[5].s64 = ctx.r[25].s64 + 2;
	// 824318C4: 4082FE18  bne 0x824316dc
	if !ctx.cr[0].eq {
	pc = 0x824316DC; continue 'dispatch;
	}
	// 824318C8: 83E1FF60  lwz r31, -0xa0(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) } as u64;
	// 824318CC: 3BD40012  addi r30, r20, 0x12
	ctx.r[30].s64 = ctx.r[20].s64 + 18;
	// 824318D0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824318D4: 7F1F7000  cmpw cr6, r31, r14
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[14].s32, &mut ctx.xer);
	// 824318D8: 93E1FF60  stw r31, -0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.r[31].u32 ) };
	// 824318DC: 4198FD74  blt cr6, 0x82431650
	if ctx.cr[6].lt {
	pc = 0x82431650; continue 'dispatch;
	}
	// 824318E0: B1660000  sth r11, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824318E4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824318E8: B2A60002  sth r21, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[21].u16 ) };
	// 824318EC: B2E80000  sth r23, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[23].u16 ) };
	// 824318F0: B2C80002  sth r22, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[22].u16 ) };
	// 824318F4: 481037DC  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 824318F8: 5723083C  slwi r3, r25, 1
	ctx.r[3].u32 = ctx.r[25].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824318FC: 4BFFFFF8  b 0x824318f4
	pc = 0x824318F4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82431900 size=616
    let mut pc: u32 = 0x82431900;
    'dispatch: loop {
        match pc {
            0x82431900 => {
    //   block [0x82431900..0x82431B68)
	// 82431900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431904: 48103781  bl 0x82535084
	ctx.lr = 0x82431908;
	sub_82535080(ctx, base);
	// 82431908: 7C8B0E70  srawi r11, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 8243190C: ABE60000  lha r31, 0(r6)
	ctx.r[31].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82431910: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82431914: ABC60002  lha r30, 2(r6)
	ctx.r[30].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 82431918: 7CF43B78  mr r20, r7
	ctx.r[20].u64 = ctx.r[7].u64;
	// 8243191C: AAA80002  lha r21, 2(r8)
	ctx.r[21].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 82431920: A8E80000  lha r7, 0(r8)
	ctx.r[7].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82431924: 7DEB0195  addze. r15, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[15].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[15].s32, 0, &mut ctx.xer);
	// 82431928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243192C: 4081021C  ble 0x82431b48
	if !ctx.cr[0].gt {
	pc = 0x82431B48; continue 'dispatch;
	}
	// 82431930: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82431934: 82210054  lwz r17, 0x54(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82431938: 3A0B3F60  addi r16, r11, 0x3f60
	ctx.r[16].s64 = ctx.r[11].s64 + 16224;
	// 8243193C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82431940: 557C0021  rlwinm. r28, r11, 0, 0, 0x10
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82431944: 4082021C  bne 0x82431b60
	if !ctx.cr[0].eq {
	pc = 0x82431B60; continue 'dispatch;
	}
	// 82431948: AB910000  lha r28, 0(r17)
	ctx.r[28].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8243194C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82431950: AB61005E  lha r27, 0x5e(r1)
	ctx.r[27].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as i16) as i64;
	// 82431954: 7F99E378  mr r25, r28
	ctx.r[25].u64 = ctx.r[28].u64;
	// 82431958: AB410066  lha r26, 0x66(r1)
	ctx.r[26].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as i16) as i64;
	// 8243195C: 7F2B5A78  xor r11, r25, r11
	ctx.r[11].u64 = ctx.r[25].u64 ^ ctx.r[11].u64;
	// 82431960: 556B04FE  clrlwi r11, r11, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 82431964: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82431968: 7D790734  extsh r25, r11
	ctx.r[25].s64 = ctx.r[11].s16 as i64;
	// 8243196C: 7D7CD9D6  mullw r11, r28, r27
	ctx.r[11].s64 = (ctx.r[28].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82431970: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82431974: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 82431978: B1710000  sth r11, 0(r17)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8243197C: A39D0012  lhz r28, 0x12(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(18 as u32) ) } as u64;
	// 82431980: 57980021  rlwinm. r24, r28, 0, 0, 0x10
	ctx.r[24].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82431984: 408201DC  bne 0x82431b60
	if !ctx.cr[0].eq {
	pc = 0x82431B60; continue 'dispatch;
	}
	// 82431988: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8243198C: 7F980734  extsh r24, r28
	ctx.r[24].s64 = ctx.r[28].s16 as i64;
	// 82431990: 7F6BD9D6  mullw r27, r11, r27
	ctx.r[27].s64 = (ctx.r[11].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82431994: 7F360734  extsh r22, r25
	ctx.r[22].s64 = ctx.r[25].s16 as i64;
	// 82431998: 7F195A78  xor r25, r24, r11
	ctx.r[25].u64 = ctx.r[24].u64 ^ ctx.r[11].u64;
	// 8243199C: 7F7BD214  add r27, r27, r26
	ctx.r[27].u64 = ctx.r[27].u64 + ctx.r[26].u64;
	// 824319A0: 572B04FE  clrlwi r11, r25, 0x13
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x00001FFFu64;
	// 824319A4: 577B047E  clrlwi r27, r27, 0x11
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0x00007FFFu64;
	// 824319A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824319AC: 3AFD0002  addi r23, r29, 2
	ctx.r[23].s64 = ctx.r[29].s64 + 2;
	// 824319B0: 7D5D0734  extsh r29, r10
	ctx.r[29].s64 = ctx.r[10].s16 as i64;
	// 824319B4: 7D3C0734  extsh r28, r9
	ctx.r[28].s64 = ctx.r[9].s16 as i64;
	// 824319B8: 3A400010  li r18, 0x10
	ctx.r[18].s64 = 16;
	// 824319BC: B3710000  sth r27, 0(r17)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 824319C0: 7D730734  extsh r19, r11
	ctx.r[19].s64 = ctx.r[11].s16 as i64;
	// 824319C4: 8B770012  lbz r27, 0x12(r23)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(18 as u32) ) } as u64;
	// 824319C8: 7FDDF1D6  mullw r30, r29, r30
	ctx.r[30].s64 = (ctx.r[29].s32 as i64) * (ctx.r[30].s32 as i64);
	// 824319CC: 7D7CF9D6  mullw r11, r28, r31
	ctx.r[11].s64 = (ctx.r[28].s32 as i64) * (ctx.r[31].s32 as i64);
	// 824319D0: 8B570000  lbz r26, 0(r23)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 824319D4: 7F790774  extsb r25, r27
	ctx.r[25].s64 = ctx.r[27].s8 as i64;
	// 824319D8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824319DC: 7F580774  extsb r24, r26
	ctx.r[24].s64 = ctx.r[26].s8 as i64;
	// 824319E0: 7F3A2670  srawi r26, r25, 4
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[25].s32 >> 4) as i64;
	// 824319E4: 7D6B6670  srawi r11, r11, 0xc
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 12) as i64;
	// 824319E8: 7F1E2670  srawi r30, r24, 4
	ctx.xer.ca = (ctx.r[24].s32 < 0) && ((ctx.r[24].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[24].s32 >> 4) as i64;
	// 824319EC: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 824319F0: 7FDEB1D6  mullw r30, r30, r22
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[22].s32 as i64);
	// 824319F4: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824319F8: 2F1E7FFF  cmpwi cr6, r30, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32767, &mut ctx.xer);
	// 824319FC: 4199000C  bgt cr6, 0x82431a08
	if ctx.cr[6].gt {
	pc = 0x82431A08; continue 'dispatch;
	}
	// 82431A00: 2F1E8000  cmpwi cr6, r30, -0x8000
	ctx.cr[6].compare_i32(ctx.r[30].s32, -32768, &mut ctx.xer);
	// 82431A04: 40980020  bge cr6, 0x82431a24
	if !ctx.cr[6].lt {
	pc = 0x82431A24; continue 'dispatch;
	}
	// 82431A08: 2F1E8000  cmpwi cr6, r30, -0x8000
	ctx.cr[6].compare_i32(ctx.r[30].s32, -32768, &mut ctx.xer);
	// 82431A0C: 4098000C  bge cr6, 0x82431a18
	if !ctx.cr[6].lt {
	pc = 0x82431A18; continue 'dispatch;
	}
	// 82431A10: 3BC08000  li r30, -0x8000
	ctx.r[30].s64 = -32768;
	// 82431A14: 48000010  b 0x82431a24
	pc = 0x82431A24; continue 'dispatch;
	// 82431A18: 2F1E7FFF  cmpwi cr6, r30, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32767, &mut ctx.xer);
	// 82431A1C: 40990008  ble cr6, 0x82431a24
	if !ctx.cr[6].gt {
	pc = 0x82431A24; continue 'dispatch;
	}
	// 82431A20: 3BC07FFF  li r30, 0x7fff
	ctx.r[30].s64 = 32767;
	// 82431A24: 7F7DA9D6  mullw r27, r29, r21
	ctx.r[27].s64 = (ctx.r[29].s32 as i64) * (ctx.r[21].s32 as i64);
	// 82431A28: 7D7C39D6  mullw r11, r28, r7
	ctx.r[11].s64 = (ctx.r[28].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82431A2C: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82431A30: 7F53D1D6  mullw r26, r19, r26
	ctx.r[26].s64 = (ctx.r[19].s32 as i64) * (ctx.r[26].s32 as i64);
	// 82431A34: 7D6B6670  srawi r11, r11, 0xc
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 12) as i64;
	// 82431A38: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82431A3C: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82431A40: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82431A44: 4199000C  bgt cr6, 0x82431a50
	if ctx.cr[6].gt {
	pc = 0x82431A50; continue 'dispatch;
	}
	// 82431A48: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 82431A4C: 40980020  bge cr6, 0x82431a6c
	if !ctx.cr[6].lt {
	pc = 0x82431A6C; continue 'dispatch;
	}
	// 82431A50: 2F0B8000  cmpwi cr6, r11, -0x8000
	ctx.cr[6].compare_i32(ctx.r[11].s32, -32768, &mut ctx.xer);
	// 82431A54: 4098000C  bge cr6, 0x82431a60
	if !ctx.cr[6].lt {
	pc = 0x82431A60; continue 'dispatch;
	}
	// 82431A58: 39608000  li r11, -0x8000
	ctx.r[11].s64 = -32768;
	// 82431A5C: 48000010  b 0x82431a6c
	pc = 0x82431A6C; continue 'dispatch;
	// 82431A60: 2F0B7FFF  cmpwi cr6, r11, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32767, &mut ctx.xer);
	// 82431A64: 40990008  ble cr6, 0x82431a6c
	if !ctx.cr[6].gt {
	pc = 0x82431A6C; continue 'dispatch;
	}
	// 82431A68: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 82431A6C: 571816BA  rlwinm r24, r24, 2, 0x1a, 0x1d
	ctx.r[24].u64 = ctx.r[24].u32 as u64 & 0x3FFFFFFFu64;
	// 82431A70: B3C50000  sth r30, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82431A74: B1740000  sth r11, 0(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82431A78: 7F7DD9D6  mullw r27, r29, r27
	ctx.r[27].s64 = (ctx.r[29].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82431A7C: 7F58802E  lwzx r26, r24, r16
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82431A80: 7FFCF1D6  mullw r31, r28, r30
	ctx.r[31].s64 = (ctx.r[28].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82431A84: 7FFFDA14  add r31, r31, r27
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 82431A88: 7F7AB1D6  mullw r27, r26, r22
	ctx.r[27].s64 = (ctx.r[26].s32 as i64) * (ctx.r[22].s32 as i64);
	// 82431A8C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82431A90: 572716BA  rlwinm r7, r25, 2, 0x1a, 0x1d
	ctx.r[7].u64 = ctx.r[25].u32 as u64 & 0x3FFFFFFFu64;
	// 82431A94: 7FFF6670  srawi r31, r31, 0xc
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 12) as i64;
	// 82431A98: 3B250002  addi r25, r5, 2
	ctx.r[25].s64 = ctx.r[5].s64 + 2;
	// 82431A9C: 7FFFDA14  add r31, r31, r27
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 82431AA0: 3B140002  addi r24, r20, 2
	ctx.r[24].s64 = ctx.r[20].s64 + 2;
	// 82431AA4: 7CE7802E  lwzx r7, r7, r16
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82431AA8: 2F1F7FFF  cmpwi cr6, r31, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[31].s32, 32767, &mut ctx.xer);
	// 82431AAC: 4199000C  bgt cr6, 0x82431ab8
	if ctx.cr[6].gt {
	pc = 0x82431AB8; continue 'dispatch;
	}
	// 82431AB0: 2F1F8000  cmpwi cr6, r31, -0x8000
	ctx.cr[6].compare_i32(ctx.r[31].s32, -32768, &mut ctx.xer);
	// 82431AB4: 40980020  bge cr6, 0x82431ad4
	if !ctx.cr[6].lt {
	pc = 0x82431AD4; continue 'dispatch;
	}
	// 82431AB8: 2F1F8000  cmpwi cr6, r31, -0x8000
	ctx.cr[6].compare_i32(ctx.r[31].s32, -32768, &mut ctx.xer);
	// 82431ABC: 4098000C  bge cr6, 0x82431ac8
	if !ctx.cr[6].lt {
	pc = 0x82431AC8; continue 'dispatch;
	}
	// 82431AC0: 3BE08000  li r31, -0x8000
	ctx.r[31].s64 = -32768;
	// 82431AC4: 48000010  b 0x82431ad4
	pc = 0x82431AD4; continue 'dispatch;
	// 82431AC8: 2F1F7FFF  cmpwi cr6, r31, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[31].s32, 32767, &mut ctx.xer);
	// 82431ACC: 40990008  ble cr6, 0x82431ad4
	if !ctx.cr[6].gt {
	pc = 0x82431AD4; continue 'dispatch;
	}
	// 82431AD0: 3BE07FFF  li r31, 0x7fff
	ctx.r[31].s64 = 32767;
	// 82431AD4: 7CBC59D6  mullw r5, r28, r11
	ctx.r[5].s64 = (ctx.r[28].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82431AD8: 7F7DD1D6  mullw r27, r29, r26
	ctx.r[27].s64 = (ctx.r[29].s32 as i64) * (ctx.r[26].s32 as i64);
	// 82431ADC: 7F65DA14  add r27, r5, r27
	ctx.r[27].u64 = ctx.r[5].u64 + ctx.r[27].u64;
	// 82431AE0: 7CB339D6  mullw r5, r19, r7
	ctx.r[5].s64 = (ctx.r[19].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82431AE4: 7F676670  srawi r7, r27, 0xc
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[27].s32 >> 12) as i64;
	// 82431AE8: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 82431AEC: 2F077FFF  cmpwi cr6, r7, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[7].s32, 32767, &mut ctx.xer);
	// 82431AF0: 4199000C  bgt cr6, 0x82431afc
	if ctx.cr[6].gt {
	pc = 0x82431AFC; continue 'dispatch;
	}
	// 82431AF4: 2F078000  cmpwi cr6, r7, -0x8000
	ctx.cr[6].compare_i32(ctx.r[7].s32, -32768, &mut ctx.xer);
	// 82431AF8: 40980020  bge cr6, 0x82431b18
	if !ctx.cr[6].lt {
	pc = 0x82431B18; continue 'dispatch;
	}
	// 82431AFC: 2F078000  cmpwi cr6, r7, -0x8000
	ctx.cr[6].compare_i32(ctx.r[7].s32, -32768, &mut ctx.xer);
	// 82431B00: 4098000C  bge cr6, 0x82431b0c
	if !ctx.cr[6].lt {
	pc = 0x82431B0C; continue 'dispatch;
	}
	// 82431B04: 38E08000  li r7, -0x8000
	ctx.r[7].s64 = -32768;
	// 82431B08: 48000010  b 0x82431b18
	pc = 0x82431B18; continue 'dispatch;
	// 82431B0C: 2F077FFF  cmpwi cr6, r7, 0x7fff
	ctx.cr[6].compare_i32(ctx.r[7].s32, 32767, &mut ctx.xer);
	// 82431B10: 40990008  ble cr6, 0x82431b18
	if !ctx.cr[6].gt {
	pc = 0x82431B18; continue 'dispatch;
	}
	// 82431B14: 38E07FFF  li r7, 0x7fff
	ctx.r[7].s64 = 32767;
	// 82431B18: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82431B1C: B3F90000  sth r31, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 82431B20: 3652FFFF  addic. r18, r18, -1
	ctx.xer.ca = (ctx.r[18].u32 > (!(-1 as u32)));
	ctx.r[18].s64 = ctx.r[18].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 82431B24: 7D755B78  mr r21, r11
	ctx.r[21].u64 = ctx.r[11].u64;
	// 82431B28: 38B90002  addi r5, r25, 2
	ctx.r[5].s64 = ctx.r[25].s64 + 2;
	// 82431B2C: 3A980002  addi r20, r24, 2
	ctx.r[20].s64 = ctx.r[24].s64 + 2;
	// 82431B30: B3780000  sth r27, 0(r24)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 82431B34: 4082FE90  bne 0x824319c4
	if !ctx.cr[0].eq {
	pc = 0x824319C4; continue 'dispatch;
	}
	// 82431B38: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82431B3C: 3BB70012  addi r29, r23, 0x12
	ctx.r[29].s64 = ctx.r[23].s64 + 18;
	// 82431B40: 7F037800  cmpw cr6, r3, r15
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[15].s32, &mut ctx.xer);
	// 82431B44: 4198FDF8  blt cr6, 0x8243193c
	if ctx.cr[6].lt {
	pc = 0x8243193C; continue 'dispatch;
	}
	// 82431B48: B3E60000  sth r31, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 82431B4C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82431B50: B3C60002  sth r30, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[30].u16 ) };
	// 82431B54: B0E80000  sth r7, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 82431B58: B2A80002  sth r21, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[21].u16 ) };
	// 82431B5C: 48103578  b 0x825350d4
	sub_825350D0(ctx, base);
	return;
	// 82431B60: 5463083C  slwi r3, r3, 1
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82431B64: 4BFFFFF8  b 0x82431b5c
	pc = 0x82431B5C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431B68 size=100
    let mut pc: u32 = 0x82431B68;
    'dispatch: loop {
        match pc {
            0x82431B68 => {
    //   block [0x82431B68..0x82431BCC)
	// 82431B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82431B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431B74: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82431B78: 816B9B44  lwz r11, -0x64bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25788 as u32) ) } as u64;
	// 82431B7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82431B80: A16100D6  lhz r11, 0xd6(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(214 as u32) ) } as u64;
	// 82431B84: 409A0020  bne cr6, 0x82431ba4
	if !ctx.cr[6].eq {
	pc = 0x82431BA4; continue 'dispatch;
	}
	// 82431B88: B1610066  sth r11, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[11].u16 ) };
	// 82431B8C: A16100CE  lhz r11, 0xce(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(206 as u32) ) } as u64;
	// 82431B90: B161005E  sth r11, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[11].u16 ) };
	// 82431B94: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82431B98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82431B9C: 4BFFFD65  bl 0x82431900
	ctx.lr = 0x82431BA0;
	sub_82431900(ctx, base);
	// 82431BA0: 4800001C  b 0x82431bbc
	pc = 0x82431BBC; continue 'dispatch;
	// 82431BA4: B1610066  sth r11, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[11].u16 ) };
	// 82431BA8: A16100CE  lhz r11, 0xce(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(206 as u32) ) } as u64;
	// 82431BAC: B161005E  sth r11, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[11].u16 ) };
	// 82431BB0: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82431BB4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82431BB8: 4BFFFA51  bl 0x82431608
	ctx.lr = 0x82431BBC;
	sub_82431608(ctx, base);
	// 82431BBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82431BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431BD0 size=8
    let mut pc: u32 = 0x82431BD0;
    'dispatch: loop {
        match pc {
            0x82431BD0 => {
    //   block [0x82431BD0..0x82431BD8)
	// 82431BD0: 3860201F  li r3, 0x201f
	ctx.r[3].s64 = 8223;
	// 82431BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431BD8 size=4
    let mut pc: u32 = 0x82431BD8;
    'dispatch: loop {
        match pc {
            0x82431BD8 => {
    //   block [0x82431BD8..0x82431BDC)
	// 82431BD8: 480057D8  b 0x824373b0
	sub_824373B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431BE0 size=4
    let mut pc: u32 = 0x82431BE0;
    'dispatch: loop {
        match pc {
            0x82431BE0 => {
    //   block [0x82431BE0..0x82431BE4)
	// 82431BE0: 480056C8  b 0x824372a8
	sub_824372A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431BE8 size=8
    let mut pc: u32 = 0x82431BE8;
    'dispatch: loop {
        match pc {
            0x82431BE8 => {
    //   block [0x82431BE8..0x82431BF0)
	// 82431BE8: 806300B8  lwz r3, 0xb8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 82431BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82431BF0 size=48
    let mut pc: u32 = 0x82431BF0;
    'dispatch: loop {
        match pc {
            0x82431BF0 => {
    //   block [0x82431BF0..0x82431C20)
	// 82431BF0: 7C8B1E70  srawi r11, r4, 3
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 3) as i64;
	// 82431BF4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82431BF8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82431BFC: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82431C00: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82431C04: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82431C08: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82431C0C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82431C10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82431C14: 814B00D0  lwz r10, 0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82431C18: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82431C1C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431C20 size=8
    let mut pc: u32 = 0x82431C20;
    'dispatch: loop {
        match pc {
            0x82431C20 => {
    //   block [0x82431C20..0x82431C28)
	// 82431C20: 806B00E0  lwz r3, 0xe0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(224 as u32) ) } as u64;
	// 82431C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82431C28 size=48
    let mut pc: u32 = 0x82431C28;
    'dispatch: loop {
        match pc {
            0x82431C28 => {
    //   block [0x82431C28..0x82431C58)
	// 82431C28: 7C8B1E70  srawi r11, r4, 3
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 3) as i64;
	// 82431C2C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82431C30: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82431C34: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82431C38: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82431C3C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82431C40: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82431C44: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82431C48: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 82431C4C: 814B00D0  lwz r10, 0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82431C50: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82431C54: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431C58 size=8
    let mut pc: u32 = 0x82431C58;
    'dispatch: loop {
        match pc {
            0x82431C58 => {
    //   block [0x82431C58..0x82431C60)
	// 82431C58: 806B00E8  lwz r3, 0xe8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(232 as u32) ) } as u64;
	// 82431C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431C60 size=108
    let mut pc: u32 = 0x82431C60;
    'dispatch: loop {
        match pc {
            0x82431C60 => {
    //   block [0x82431C60..0x82431CCC)
	// 82431C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82431C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431C6C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82431C70: 419A0048  beq cr6, 0x82431cb8
	if ctx.cr[6].eq {
	pc = 0x82431CB8; continue 'dispatch;
	}
	// 82431C74: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82431C78: 419A002C  beq cr6, 0x82431ca4
	if ctx.cr[6].eq {
	pc = 0x82431CA4; continue 'dispatch;
	}
	// 82431C7C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82431C80: 419A0010  beq cr6, 0x82431c90
	if ctx.cr[6].eq {
	pc = 0x82431C90; continue 'dispatch;
	}
	// 82431C84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82431C88: 386B47E0  addi r3, r11, 0x47e0
	ctx.r[3].s64 = ctx.r[11].s64 + 18400;
	// 82431C8C: 4800543D  bl 0x824370c8
	ctx.lr = 0x82431C90;
	sub_824370C8(ctx, base);
	// 82431C90: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82431C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431CA0: 4E800020  blr
	return;
	// 82431CA4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82431CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431CB4: 4E800020  blr
	return;
	// 82431CB8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82431CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431CD0 size=12
    let mut pc: u32 = 0x82431CD0;
    'dispatch: loop {
        match pc {
            0x82431CD0 => {
    //   block [0x82431CD0..0x82431CDC)
	// 82431CD0: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82431CD4: 9164004C  stw r11, 0x4c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82431CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431CE0 size=16
    let mut pc: u32 = 0x82431CE0;
    'dispatch: loop {
        match pc {
            0x82431CE0 => {
    //   block [0x82431CE0..0x82431CF0)
	// 82431CE0: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82431CE4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82431CE8: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82431CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431CF0 size=84
    let mut pc: u32 = 0x82431CF0;
    'dispatch: loop {
        match pc {
            0x82431CF0 => {
    //   block [0x82431CF0..0x82431D44)
	// 82431CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82431CF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82431CFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431D00: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82431D04: 806300B8  lwz r3, 0xb8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 82431D08: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82431D0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82431D10: 480058E1  bl 0x824375f0
	ctx.lr = 0x82431D14;
	sub_824375F0(ctx, base);
	// 82431D14: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82431D18: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82431D1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82431D20: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82431D24: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82431D28: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82431D2C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82431D30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82431D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431D48 size=156
    let mut pc: u32 = 0x82431D48;
    'dispatch: loop {
        match pc {
            0x82431D48 => {
    //   block [0x82431D48..0x82431DE4)
	// 82431D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82431D50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82431D54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431D58: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82431D5C: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 82431D60: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82431D64: 419A0068  beq cr6, 0x82431dcc
	if ctx.cr[6].eq {
	pc = 0x82431DCC; continue 'dispatch;
	}
	// 82431D68: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82431D6C: 419A0048  beq cr6, 0x82431db4
	if ctx.cr[6].eq {
	pc = 0x82431DB4; continue 'dispatch;
	}
	// 82431D70: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82431D74: 419A0028  beq cr6, 0x82431d9c
	if ctx.cr[6].eq {
	pc = 0x82431D9C; continue 'dispatch;
	}
	// 82431D78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82431D7C: 386B4808  addi r3, r11, 0x4808
	ctx.r[3].s64 = ctx.r[11].s64 + 18440;
	// 82431D80: 48005349  bl 0x824370c8
	ctx.lr = 0x82431D84;
	sub_824370C8(ctx, base);
	// 82431D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82431D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431D94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431D98: 4E800020  blr
	return;
	// 82431D9C: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82431DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431DB0: 4E800020  blr
	return;
	// 82431DB4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82431DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431DC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431DC8: 4E800020  blr
	return;
	// 82431DCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82431DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431DDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431DE8 size=156
    let mut pc: u32 = 0x82431DE8;
    'dispatch: loop {
        match pc {
            0x82431DE8 => {
    //   block [0x82431DE8..0x82431E84)
	// 82431DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82431DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82431DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431DF8: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82431DFC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82431E00: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82431E04: 419A0068  beq cr6, 0x82431e6c
	if ctx.cr[6].eq {
	pc = 0x82431E6C; continue 'dispatch;
	}
	// 82431E08: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82431E0C: 419A0048  beq cr6, 0x82431e54
	if ctx.cr[6].eq {
	pc = 0x82431E54; continue 'dispatch;
	}
	// 82431E10: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82431E14: 419A0028  beq cr6, 0x82431e3c
	if ctx.cr[6].eq {
	pc = 0x82431E3C; continue 'dispatch;
	}
	// 82431E18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82431E1C: 386B4830  addi r3, r11, 0x4830
	ctx.r[3].s64 = ctx.r[11].s64 + 18480;
	// 82431E20: 480052A9  bl 0x824370c8
	ctx.lr = 0x82431E24;
	sub_824370C8(ctx, base);
	// 82431E24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82431E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431E38: 4E800020  blr
	return;
	// 82431E3C: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82431E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431E4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431E50: 4E800020  blr
	return;
	// 82431E54: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82431E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431E68: 4E800020  blr
	return;
	// 82431E6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82431E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431E7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431E88 size=116
    let mut pc: u32 = 0x82431E88;
    'dispatch: loop {
        match pc {
            0x82431E88 => {
    //   block [0x82431E88..0x82431EFC)
	// 82431E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82431E90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82431E94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431E98: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82431E9C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82431EA0: 41980044  blt cr6, 0x82431ee4
	if ctx.cr[6].lt {
	pc = 0x82431EE4; continue 'dispatch;
	}
	// 82431EA4: 419A0028  beq cr6, 0x82431ecc
	if ctx.cr[6].eq {
	pc = 0x82431ECC; continue 'dispatch;
	}
	// 82431EA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82431EAC: 386B4854  addi r3, r11, 0x4854
	ctx.r[3].s64 = ctx.r[11].s64 + 18516;
	// 82431EB0: 48005219  bl 0x824370c8
	ctx.lr = 0x82431EB4;
	sub_824370C8(ctx, base);
	// 82431EB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82431EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431EC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431EC8: 4E800020  blr
	return;
	// 82431ECC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82431ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431EDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431EE0: 4E800020  blr
	return;
	// 82431EE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82431EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431EF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82431EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431F00 size=36
    let mut pc: u32 = 0x82431F00;
    'dispatch: loop {
        match pc {
            0x82431F00 => {
    //   block [0x82431F00..0x82431F24)
	// 82431F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82431F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431F0C: 4BFFFCDD  bl 0x82431be8
	ctx.lr = 0x82431F10;
	sub_82431BE8(ctx, base);
	// 82431F10: 4801FDB9  bl 0x82451cc8
	ctx.lr = 0x82431F14;
	sub_82451CC8(ctx, base);
	// 82431F14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82431F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82431F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82431F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431F28 size=104
    let mut pc: u32 = 0x82431F28;
    'dispatch: loop {
        match pc {
            0x82431F28 => {
    //   block [0x82431F28..0x82431F90)
	// 82431F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431F2C: 4810318D  bl 0x825350b8
	ctx.lr = 0x82431F30;
	sub_82535080(ctx, base);
	// 82431F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431F34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82431F38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82431F3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82431F40: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82431F44: 4800111D  bl 0x82433060
	ctx.lr = 0x82431F48;
	sub_82433060(ctx, base);
	// 82431F48: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82431F4C: 419A0018  beq cr6, 0x82431f64
	if ctx.cr[6].eq {
	pc = 0x82431F64; continue 'dispatch;
	}
	// 82431F50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82431F54: 386B4874  addi r3, r11, 0x4874
	ctx.r[3].s64 = ctx.r[11].s64 + 18548;
	// 82431F58: 48005171  bl 0x824370c8
	ctx.lr = 0x82431F5C;
	sub_824370C8(ctx, base);
	// 82431F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82431F60: 481031A8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82431F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82431F68: 4BFFFC81  bl 0x82431be8
	ctx.lr = 0x82431F6C;
	sub_82431BE8(ctx, base);
	// 82431F6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82431F70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82431F74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82431F78: 480055C9  bl 0x82437540
	ctx.lr = 0x82431F7C;
	sub_82437540(ctx, base);
	// 82431F7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82431F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82431F84: 480055CD  bl 0x82437550
	ctx.lr = 0x82431F88;
	sub_82437550(ctx, base);
	// 82431F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82431F8C: 4810317C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431F90 size=24
    let mut pc: u32 = 0x82431F90;
    'dispatch: loop {
        match pc {
            0x82431F90 => {
    //   block [0x82431F90..0x82431FA8)
	// 82431F90: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82431F94: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82431F98: 419A0010  beq cr6, 0x82431fa8
	if ctx.cr[6].eq {
		sub_82431FA8(ctx, base);
		return;
	}
	// 82431F9C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82431FA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82431FA4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431FA8 size=8
    let mut pc: u32 = 0x82431FA8;
    'dispatch: loop {
        match pc {
            0x82431FA8 => {
    //   block [0x82431FA8..0x82431FB0)
	// 82431FA8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82431FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431FB0 size=24
    let mut pc: u32 = 0x82431FB0;
    'dispatch: loop {
        match pc {
            0x82431FB0 => {
    //   block [0x82431FB0..0x82431FC8)
	// 82431FB0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82431FB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82431FB8: 419A0010  beq cr6, 0x82431fc8
	if ctx.cr[6].eq {
		sub_82431FC8(ctx, base);
		return;
	}
	// 82431FBC: 2F0B0101  cmpwi cr6, r11, 0x101
	ctx.cr[6].compare_i32(ctx.r[11].s32, 257, &mut ctx.xer);
	// 82431FC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82431FC4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82431FC8 size=8
    let mut pc: u32 = 0x82431FC8;
    'dispatch: loop {
        match pc {
            0x82431FC8 => {
    //   block [0x82431FC8..0x82431FD0)
	// 82431FC8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82431FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82431FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82431FD0 size=104
    let mut pc: u32 = 0x82431FD0;
    'dispatch: loop {
        match pc {
            0x82431FD0 => {
    //   block [0x82431FD0..0x82432038)
	// 82431FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82431FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82431FD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82431FDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82431FE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82431FE4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82431FE8: 4BFFFFC9  bl 0x82431fb0
	ctx.lr = 0x82431FEC;
	sub_82431FB0(ctx, base);
	// 82431FEC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82431FF0: 409A0030  bne cr6, 0x82432020
	if !ctx.cr[6].eq {
	pc = 0x82432020; continue 'dispatch;
	}
	// 82431FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82431FF8: 809F0418  lwz r4, 0x418(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1048 as u32) ) } as u64;
	// 82431FFC: 807F0414  lwz r3, 0x414(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1044 as u32) ) } as u64;
	// 82432000: 4BFF1449  bl 0x82423448
	ctx.lr = 0x82432004;
	sub_82423448(ctx, base);
	// 82432004: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432008: 409A001C  bne cr6, 0x82432024
	if !ctx.cr[6].eq {
	pc = 0x82432024; continue 'dispatch;
	}
	// 8243200C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432010: 386B48A8  addi r3, r11, 0x48a8
	ctx.r[3].s64 = ctx.r[11].s64 + 18600;
	// 82432014: 480050B5  bl 0x824370c8
	ctx.lr = 0x82432018;
	sub_824370C8(ctx, base);
	// 82432018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243201C: 4800438D  bl 0x824363a8
	ctx.lr = 0x82432020;
	sub_824363A8(ctx, base);
	// 82432020: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243202C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432038 size=12
    let mut pc: u32 = 0x82432038;
    'dispatch: loop {
        match pc {
            0x82432038 => {
    //   block [0x82432038..0x82432044)
	// 82432038: 81630410  lwz r11, 0x410(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1040 as u32) ) } as u64;
	// 8243203C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82432040: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432044(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432044 size=20
    let mut pc: u32 = 0x82432044;
    'dispatch: loop {
        match pc {
            0x82432044 => {
    //   block [0x82432044..0x82432058)
	// 82432044: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82432048: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243204C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82432050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82432054: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432058 size=4
    let mut pc: u32 = 0x82432058;
    'dispatch: loop {
        match pc {
            0x82432058 => {
    //   block [0x82432058..0x8243205C)
	// 82432058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82432060 size=100
    let mut pc: u32 = 0x82432060;
    'dispatch: loop {
        match pc {
            0x82432060 => {
    //   block [0x82432060..0x824320C4)
	// 82432060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243206C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82432070: 4BFFFF21  bl 0x82431f90
	ctx.lr = 0x82432074;
	sub_82431F90(ctx, base);
	// 82432074: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82432078: 409A0018  bne cr6, 0x82432090
	if !ctx.cr[6].eq {
	pc = 0x82432090; continue 'dispatch;
	}
	// 8243207C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243208C: 4E800020  blr
	return;
	// 82432090: 80AA0410  lwz r5, 0x410(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1040 as u32) ) } as u64;
	// 82432094: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82432098: 419AFFE4  beq cr6, 0x8243207c
	if ctx.cr[6].eq {
	pc = 0x8243207C; continue 'dispatch;
	}
	// 8243209C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824320A0: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 824320A4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824320A8: 48005679  bl 0x82437720
	ctx.lr = 0x824320AC;
	sub_82437720(ctx, base);
	// 824320AC: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 824320B0: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 824320B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824320B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824320BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824320C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824320C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824320C8 size=28
    let mut pc: u32 = 0x824320C8;
    'dispatch: loop {
        match pc {
            0x824320C8 => {
    //   block [0x824320C8..0x824320E4)
	// 824320C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824320CC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 824320D0: 91630428  stw r11, 0x428(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1064 as u32), ctx.r[11].u32 ) };
	// 824320D4: 9163042C  stw r11, 0x42c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1068 as u32), ctx.r[11].u32 ) };
	// 824320D8: 91630430  stw r11, 0x430(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1072 as u32), ctx.r[11].u32 ) };
	// 824320DC: 91430424  stw r10, 0x424(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1060 as u32), ctx.r[10].u32 ) };
	// 824320E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824320E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824320E8 size=12
    let mut pc: u32 = 0x824320E8;
    'dispatch: loop {
        match pc {
            0x824320E8 => {
    //   block [0x824320E8..0x824320F4)
	// 824320E8: 80630410  lwz r3, 0x410(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1040 as u32) ) } as u64;
	// 824320EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824320F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824320F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824320F4 size=16
    let mut pc: u32 = 0x824320F4;
    'dispatch: loop {
        match pc {
            0x824320F4 => {
    //   block [0x824320F4..0x82432104)
	// 824320F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824320F8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824320FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82432100: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432104(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432104 size=4
    let mut pc: u32 = 0x82432104;
    'dispatch: loop {
        match pc {
            0x82432104 => {
    //   block [0x82432104..0x82432108)
	// 82432104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432108 size=128
    let mut pc: u32 = 0x82432108;
    'dispatch: loop {
        match pc {
            0x82432108 => {
    //   block [0x82432108..0x82432188)
	// 82432108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243210C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432114: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432118: 8163042C  lwz r11, 0x42c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1068 as u32) ) } as u64;
	// 8243211C: 83E300B8  lwz r31, 0xb8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 82432120: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82432124: 409A0010  bne cr6, 0x82432134
	if !ctx.cr[6].eq {
	pc = 0x82432134; continue 'dispatch;
	}
	// 82432128: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8243212C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82432130: 4800003C  b 0x8243216c
	pc = 0x8243216C; continue 'dispatch;
	// 82432134: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82432138: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8243213C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82432140: 38AA48DC  addi r5, r10, 0x48dc
	ctx.r[5].s64 = ctx.r[10].s64 + 18652;
	// 82432144: 81430430  lwz r10, 0x430(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1072 as u32) ) } as u64;
	// 82432148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243214C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82432150: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82432154: 388A48D4  addi r4, r10, 0x48d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18644;
	// 82432158: 4BFF6EB9  bl 0x82429010
	ctx.lr = 0x8243215C;
	sub_82429010(ctx, base);
	// 8243215C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432160: 419AFFC8  beq cr6, 0x82432128
	if ctx.cr[6].eq {
	pc = 0x82432128; continue 'dispatch;
	}
	// 82432164: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82432168: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243216C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432170: 480053E9  bl 0x82437558
	ctx.lr = 0x82432174;
	sub_82437558(ctx, base);
	// 82432174: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243217C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82432188 size=104
    let mut pc: u32 = 0x82432188;
    'dispatch: loop {
        match pc {
            0x82432188 => {
    //   block [0x82432188..0x824321F0)
	// 82432188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432194: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82432198: 4BFFFDF9  bl 0x82431f90
	ctx.lr = 0x8243219C;
	sub_82431F90(ctx, base);
	// 8243219C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824321A0: 409A0018  bne cr6, 0x824321b8
	if !ctx.cr[6].eq {
	pc = 0x824321B8; continue 'dispatch;
	}
	// 824321A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824321A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824321AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824321B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824321B4: 4E800020  blr
	return;
	// 824321B8: 816A0410  lwz r11, 0x410(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1040 as u32) ) } as u64;
	// 824321BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824321C0: 419AFFE4  beq cr6, 0x824321a4
	if ctx.cr[6].eq {
	pc = 0x824321A4; continue 'dispatch;
	}
	// 824321C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824321C8: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 824321CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824321D0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824321D4: 4800554D  bl 0x82437720
	ctx.lr = 0x824321D8;
	sub_82437720(ctx, base);
	// 824321D8: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 824321DC: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 824321E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824321E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824321E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824321EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824321F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824321F0 size=48
    let mut pc: u32 = 0x824321F0;
    'dispatch: loop {
        match pc {
            0x824321F0 => {
    //   block [0x824321F0..0x82432220)
	// 824321F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824321F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824321F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824321FC: 480052DD  bl 0x824374d8
	ctx.lr = 0x82432200;
	sub_824374D8(ctx, base);
	// 82432200: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82432204: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82432208: 386B6798  addi r3, r11, 0x6798
	ctx.r[3].s64 = ctx.r[11].s64 + 26520;
	// 8243220C: 48004FA5  bl 0x824371b0
	ctx.lr = 0x82432210;
	sub_824371B0(ctx, base);
	// 82432210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243221C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432220 size=96
    let mut pc: u32 = 0x82432220;
    'dispatch: loop {
        match pc {
            0x82432220 => {
    //   block [0x82432220..0x82432280)
	// 82432220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243222C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82432230: 81690058  lwz r11, 0x58(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(88 as u32) ) } as u64;
	// 82432234: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82432238: 409A0028  bne cr6, 0x82432260
	if !ctx.cr[6].eq {
	pc = 0x82432260; continue 'dispatch;
	}
	// 8243223C: 80840030  lwz r4, 0x30(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82432240: 4BFFF9E9  bl 0x82431c28
	ctx.lr = 0x82432244;
	sub_82431C28(ctx, base);
	// 82432244: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82432248: 419A000C  beq cr6, 0x82432254
	if ctx.cr[6].eq {
	pc = 0x82432254; continue 'dispatch;
	}
	// 8243224C: 9069005C  stw r3, 0x5c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82432250: 4800000C  b 0x8243225c
	pc = 0x8243225C; continue 'dispatch;
	// 82432254: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 82432258: 9169005C  stw r11, 0x5c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8243225C: 8169005C  lwz r11, 0x5c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(92 as u32) ) } as u64;
	// 82432260: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82432264: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82432268: 4BFFF981  bl 0x82431be8
	ctx.lr = 0x8243226C;
	sub_82431BE8(ctx, base);
	// 8243226C: 4801FA5D  bl 0x82451cc8
	ctx.lr = 0x82432270;
	sub_82451CC8(ctx, base);
	// 82432270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243227C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432280 size=88
    let mut pc: u32 = 0x82432280;
    'dispatch: loop {
        match pc {
            0x82432280 => {
    //   block [0x82432280..0x824322D8)
	// 82432280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243228C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82432290: 80890030  lwz r4, 0x30(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 82432294: 4BFFF95D  bl 0x82431bf0
	ctx.lr = 0x82432298;
	sub_82431BF0(ctx, base);
	// 82432298: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 8243229C: 81690098  lwz r11, 0x98(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(152 as u32) ) } as u64;
	// 824322A0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 824322A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824322A8: 5543DFFE  rlwinm r3, r10, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 824322AC: 419A001C  beq cr6, 0x824322c8
	if ctx.cr[6].eq {
	pc = 0x824322C8; continue 'dispatch;
	}
	// 824322B0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824322B4: 8089009C  lwz r4, 0x9c(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(156 as u32) ) } as u64;
	// 824322B8: 48006259  bl 0x82438510
	ctx.lr = 0x824322BC;
	sub_82438510(ctx, base);
	// 824322BC: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 824322C0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824322C4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824322C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824322CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824322D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824322D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824322D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824322D8 size=176
    let mut pc: u32 = 0x824322D8;
    'dispatch: loop {
        match pc {
            0x824322D8 => {
    //   block [0x824322D8..0x82432388)
	// 824322D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824322DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824322E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824322E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824322E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824322EC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824322F0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824322F4: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824322F8: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824322FC: 909F0044  stw r4, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[4].u32 ) };
	// 82432300: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82432304: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82432308: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8243230C: 419A0018  beq cr6, 0x82432324
	if ctx.cr[6].eq {
	pc = 0x82432324; continue 'dispatch;
	}
	// 82432310: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82432314: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82432318: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8243231C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82432320: 4800004C  b 0x8243236c
	pc = 0x8243236C; continue 'dispatch;
	// 82432324: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82432328: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243232C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82432330: 48002651  bl 0x82434980
	ctx.lr = 0x82432334;
	sub_82434980(ctx, base);
	// 82432334: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82432338: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8243233C: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82432340: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82432344: 4BFFF99D  bl 0x82431ce0
	ctx.lr = 0x82432348;
	sub_82431CE0(ctx, base);
	// 82432348: 7FCB0E70  srawi r11, r30, 1
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 1) as i64;
	// 8243234C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82432350: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82432354: 7CCB0194  addze r6, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[6].s64 = tmp.s64;
	// 82432358: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243235C: 4BFFF985  bl 0x82431ce0
	ctx.lr = 0x82432360;
	sub_82431CE0(ctx, base);
	// 82432360: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82432364: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82432368: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8243236C: 4BFFF975  bl 0x82431ce0
	ctx.lr = 0x82432370;
	sub_82431CE0(ctx, base);
	// 82432370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243237C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82432380: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432388 size=124
    let mut pc: u32 = 0x82432388;
    'dispatch: loop {
        match pc {
            0x82432388 => {
    //   block [0x82432388..0x82432404)
	// 82432388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243238C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82432394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243239C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824323A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824323A4: 4BFFF9A5  bl 0x82431d48
	ctx.lr = 0x824323A8;
	sub_82431D48(ctx, base);
	// 824323A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824323AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824323B0: 917E0060  stw r11, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824323B4: 4BFFFA35  bl 0x82431de8
	ctx.lr = 0x824323B8;
	sub_82431DE8(ctx, base);
	// 824323B8: 907E0064  stw r3, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 824323BC: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 824323C0: 917E0068  stw r11, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824323C4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 824323C8: 917E006C  stw r11, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 824323CC: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 824323D0: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 824323D4: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 824323D8: 4BFFFAB1  bl 0x82431e88
	ctx.lr = 0x824323DC;
	sub_82431E88(ctx, base);
	// 824323DC: 907E0074  stw r3, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 824323E0: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 824323E4: 4BFFFAA5  bl 0x82431e88
	ctx.lr = 0x824323E8;
	sub_82431E88(ctx, base);
	// 824323E8: 907E0078  stw r3, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 824323EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824323F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824323F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824323F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824323FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432408 size=8
    let mut pc: u32 = 0x82432408;
    'dispatch: loop {
        match pc {
            0x82432408 => {
    //   block [0x82432408..0x82432410)
	// 82432408: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8243240C: 4BFFFB1C  b 0x82431f28
	sub_82431F28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432410 size=328
    let mut pc: u32 = 0x82432410;
    'dispatch: loop {
        match pc {
            0x82432410 => {
    //   block [0x82432410..0x82432558)
	// 82432410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243241C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432428: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243242C: 83DF0410  lwz r30, 0x410(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1040 as u32) ) } as u64;
	// 82432430: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82432434: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82432438: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243243C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82432440: 4E800421  bctrl
	ctx.lr = 0x82432444;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82432444: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82432448: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243244C: 419A0034  beq cr6, 0x82432480
	if ctx.cr[6].eq {
	pc = 0x82432480; continue 'dispatch;
	}
	// 82432450: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82432454: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82432458: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8243245C: 38AA48EC  addi r5, r10, 0x48ec
	ctx.r[5].s64 = ctx.r[10].s64 + 18668;
	// 82432460: 815F0414  lwz r10, 0x414(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1044 as u32) ) } as u64;
	// 82432464: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82432468: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8243246C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82432470: 388A48E4  addi r4, r10, 0x48e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18660;
	// 82432474: 4BFF6B9D  bl 0x82429010
	ctx.lr = 0x82432478;
	sub_82429010(ctx, base);
	// 82432478: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243247C: 409A001C  bne cr6, 0x82432498
	if !ctx.cr[6].eq {
	pc = 0x82432498; continue 'dispatch;
	}
	// 82432480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82432484: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82432488: 917F042C  stw r11, 0x42c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1068 as u32), ctx.r[11].u32 ) };
	// 8243248C: 917F0430  stw r11, 0x430(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1072 as u32), ctx.r[11].u32 ) };
	// 82432490: 915F0428  stw r10, 0x428(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1064 as u32), ctx.r[10].u32 ) };
	// 82432494: 480000AC  b 0x82432540
	pc = 0x82432540; continue 'dispatch;
	// 82432498: 807F041C  lwz r3, 0x41c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1052 as u32) ) } as u64;
	// 8243249C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824324A0: 419A0080  beq cr6, 0x82432520
	if ctx.cr[6].eq {
	pc = 0x82432520; continue 'dispatch;
	}
	// 824324A4: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824324A8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824324AC: 481026A5  bl 0x82534b50
	ctx.lr = 0x824324B0;
	sub_82534B50(ctx, base);
	// 824324B0: 817F041C  lwz r11, 0x41c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1052 as u32) ) } as u64;
	// 824324B4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824324B8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824324BC: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 824324C0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824324C4: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 824324C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824324CC: 917F042C  stw r11, 0x42c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1068 as u32), ctx.r[11].u32 ) };
	// 824324D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824324D4: 915F0430  stw r10, 0x430(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1072 as u32), ctx.r[10].u32 ) };
	// 824324D8: 913F0428  stw r9, 0x428(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1064 as u32), ctx.r[9].u32 ) };
	// 824324DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824324E0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824324E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824324E8: 4E800421  bctrl
	ctx.lr = 0x824324EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824324EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824324F0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824324F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824324F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824324FC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82432500: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82432504: 4E800421  bctrl
	ctx.lr = 0x82432508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82432508: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243250C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82432510: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82432514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82432518: 4E800421  bctrl
	ctx.lr = 0x8243251C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243251C: 48000024  b 0x82432540
	pc = 0x82432540; continue 'dispatch;
	// 82432520: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82432524: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82432528: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243252C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432530: 917F042C  stw r11, 0x42c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1068 as u32), ctx.r[11].u32 ) };
	// 82432534: 915F0430  stw r10, 0x430(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1072 as u32), ctx.r[10].u32 ) };
	// 82432538: 913F0428  stw r9, 0x428(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1064 as u32), ctx.r[9].u32 ) };
	// 8243253C: 4BFFFC4D  bl 0x82432188
	ctx.lr = 0x82432540;
	sub_82432188(ctx, base);
	// 82432540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243254C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82432550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432558 size=172
    let mut pc: u32 = 0x82432558;
    'dispatch: loop {
        match pc {
            0x82432558 => {
    //   block [0x82432558..0x82432604)
	// 82432558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243255C: 48102B61  bl 0x825350bc
	ctx.lr = 0x82432560;
	sub_82535080(ctx, base);
	// 82432560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432564: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82432568: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243256C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82432570: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82432574: 4BFFF6ED  bl 0x82431c60
	ctx.lr = 0x82432578;
	sub_82431C60(ctx, base);
	// 82432578: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243257C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82432580: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82432584: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82432588: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243258C: 4BFFFD4D  bl 0x824322d8
	ctx.lr = 0x82432590;
	sub_824322D8(ctx, base);
	// 82432590: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82432594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432598: 4BFFF739  bl 0x82431cd0
	ctx.lr = 0x8243259C;
	sub_82431CD0(ctx, base);
	// 8243259C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824325A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824325A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824325A8: 4BFFF749  bl 0x82431cf0
	ctx.lr = 0x824325AC;
	sub_82431CF0(ctx, base);
	// 824325AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824325B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824325B4: 4BFFFDD5  bl 0x82432388
	ctx.lr = 0x824325B8;
	sub_82432388(ctx, base);
	// 824325B8: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 824325BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824325C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824325C4: 917E0088  stw r11, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 824325C8: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 824325CC: 917E008C  stw r11, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 824325D0: 4BFFFCB1  bl 0x82432280
	ctx.lr = 0x824325D4;
	sub_82432280(ctx, base);
	// 824325D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824325D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824325DC: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 824325E0: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824325E4: 4BFFF645  bl 0x82431c28
	ctx.lr = 0x824325E8;
	sub_82431C28(ctx, base);
	// 824325E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824325EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824325F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824325F4: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 824325F8: 4BFFFC29  bl 0x82432220
	ctx.lr = 0x824325FC;
	sub_82432220(ctx, base);
	// 824325FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432600: 48102B0C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432608 size=64
    let mut pc: u32 = 0x82432608;
    'dispatch: loop {
        match pc {
            0x82432608 => {
    //   block [0x82432608..0x82432648)
	// 82432608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243260C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432610: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432614: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432618: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243261C: 817F0410  lwz r11, 0x410(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1040 as u32) ) } as u64;
	// 82432620: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82432624: 419A0010  beq cr6, 0x82432634
	if ctx.cr[6].eq {
	pc = 0x82432634; continue 'dispatch;
	}
	// 82432628: 4BFFFDE9  bl 0x82432410
	ctx.lr = 0x8243262C;
	sub_82432410(ctx, base);
	// 8243262C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432630: 4BFFFAD9  bl 0x82432108
	ctx.lr = 0x82432634;
	sub_82432108(ctx, base);
	// 82432634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243263C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432648 size=72
    let mut pc: u32 = 0x82432648;
    'dispatch: loop {
        match pc {
            0x82432648 => {
    //   block [0x82432648..0x82432690)
	// 82432648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243264C: 48102A71  bl 0x825350bc
	ctx.lr = 0x82432650;
	sub_82535080(ctx, base);
	// 82432650: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432654: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82432658: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8243265C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82432660: 4BFFF589  bl 0x82431be8
	ctx.lr = 0x82432664;
	sub_82431BE8(ctx, base);
	// 82432664: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82432668: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243266C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82432670: 4BFFFEE9  bl 0x82432558
	ctx.lr = 0x82432674;
	sub_82432558(ctx, base);
	// 82432674: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82432678: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243267C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82432680: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82432684: 48004F9D  bl 0x82437620
	ctx.lr = 0x82432688;
	sub_82437620(ctx, base);
	// 82432688: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8243268C: 48102A80  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432690 size=88
    let mut pc: u32 = 0x82432690;
    'dispatch: loop {
        match pc {
            0x82432690 => {
    //   block [0x82432690..0x824326E8)
	// 82432690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243269C: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 824326A0: 48006AE9  bl 0x82439188
	ctx.lr = 0x824326A4;
	sub_82439188(ctx, base);
	// 824326A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824326A8: 419A002C  beq cr6, 0x824326d4
	if ctx.cr[6].eq {
	pc = 0x824326D4; continue 'dispatch;
	}
	// 824326AC: 3860FEC9  li r3, -0x137
	ctx.r[3].s64 = -311;
	// 824326B0: 48004319  bl 0x824369c8
	ctx.lr = 0x824326B4;
	sub_824369C8(ctx, base);
	// 824326B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824326B8: 386B48F4  addi r3, r11, 0x48f4
	ctx.r[3].s64 = ctx.r[11].s64 + 18676;
	// 824326BC: 48004A0D  bl 0x824370c8
	ctx.lr = 0x824326C0;
	sub_824370C8(ctx, base);
	// 824326C0: 3860FEC9  li r3, -0x137
	ctx.r[3].s64 = -311;
	// 824326C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824326C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824326CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824326D0: 4E800020  blr
	return;
	// 824326D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824326D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824326DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824326E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824326E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824326E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824326E8 size=88
    let mut pc: u32 = 0x824326E8;
    'dispatch: loop {
        match pc {
            0x824326E8 => {
    //   block [0x824326E8..0x82432740)
	// 824326E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824326EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824326F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824326F4: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 824326F8: 48007EC9  bl 0x8243a5c0
	ctx.lr = 0x824326FC;
	sub_8243A5C0(ctx, base);
	// 824326FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432700: 419A002C  beq cr6, 0x8243272c
	if ctx.cr[6].eq {
	pc = 0x8243272C; continue 'dispatch;
	}
	// 82432704: 3860FECD  li r3, -0x133
	ctx.r[3].s64 = -307;
	// 82432708: 480042C1  bl 0x824369c8
	ctx.lr = 0x8243270C;
	sub_824369C8(ctx, base);
	// 8243270C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432710: 386B4910  addi r3, r11, 0x4910
	ctx.r[3].s64 = ctx.r[11].s64 + 18704;
	// 82432714: 480049B5  bl 0x824370c8
	ctx.lr = 0x82432718;
	sub_824370C8(ctx, base);
	// 82432718: 3860FECD  li r3, -0x133
	ctx.r[3].s64 = -307;
	// 8243271C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432728: 4E800020  blr
	return;
	// 8243272C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243273C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432740 size=56
    let mut pc: u32 = 0x82432740;
    'dispatch: loop {
        match pc {
            0x82432740 => {
    //   block [0x82432740..0x82432778)
	// 82432740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243274C: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432750: 480077A9  bl 0x82439ef8
	ctx.lr = 0x82432754;
	sub_82439EF8(ctx, base);
	// 82432754: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432758: 419A0010  beq cr6, 0x82432768
	if ctx.cr[6].eq {
	pc = 0x82432768; continue 'dispatch;
	}
	// 8243275C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432760: 386B492C  addi r3, r11, 0x492c
	ctx.r[3].s64 = ctx.r[11].s64 + 18732;
	// 82432764: 48004965  bl 0x824370c8
	ctx.lr = 0x82432768;
	sub_824370C8(ctx, base);
	// 82432768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243276C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432778 size=16
    let mut pc: u32 = 0x82432778;
    'dispatch: loop {
        match pc {
            0x82432778 => {
    //   block [0x82432778..0x82432788)
	// 82432778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243277C: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82432780: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82432784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432788 size=208
    let mut pc: u32 = 0x82432788;
    'dispatch: loop {
        match pc {
            0x82432788 => {
    //   block [0x82432788..0x82432858)
	// 82432788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243278C: 48102931  bl 0x825350bc
	ctx.lr = 0x82432790;
	sub_82535080(ctx, base);
	// 82432790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432798: 83BF0048  lwz r29, 0x48(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243279C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824327A0: 419A00B0  beq cr6, 0x82432850
	if ctx.cr[6].eq {
	pc = 0x82432850; continue 'dispatch;
	}
	// 824327A4: 48009BED  bl 0x8243c390
	ctx.lr = 0x824327A8;
	sub_8243C390(ctx, base);
	// 824327A8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824327AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824327B0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 824327B4: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 824327B8: 48008AA1  bl 0x8243b258
	ctx.lr = 0x824327BC;
	sub_8243B258(ctx, base);
	// 824327BC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824327C0: 419A0018  beq cr6, 0x824327d8
	if ctx.cr[6].eq {
	pc = 0x824327D8; continue 'dispatch;
	}
	// 824327C4: 3860FECC  li r3, -0x134
	ctx.r[3].s64 = -308;
	// 824327C8: 48004201  bl 0x824369c8
	ctx.lr = 0x824327CC;
	sub_824369C8(ctx, base);
	// 824327CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824327D0: 386B4954  addi r3, r11, 0x4954
	ctx.r[3].s64 = ctx.r[11].s64 + 18772;
	// 824327D4: 480048F5  bl 0x824370c8
	ctx.lr = 0x824327D8;
	sub_824370C8(ctx, base);
	// 824327D8: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 824327DC: 93BF0048  stw r29, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[29].u32 ) };
	// 824327E0: 48008E79  bl 0x8243b658
	ctx.lr = 0x824327E4;
	sub_8243B658(ctx, base);
	// 824327E4: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 824327E8: 48008E71  bl 0x8243b658
	ctx.lr = 0x824327EC;
	sub_8243B658(ctx, base);
	// 824327EC: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824327F0: 93DF0568  stw r30, 0x568(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1384 as u32), ctx.r[30].u32 ) };
	// 824327F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824327F8: 419A0008  beq cr6, 0x82432800
	if ctx.cr[6].eq {
	pc = 0x82432800; continue 'dispatch;
	}
	// 824327FC: 48008BE5  bl 0x8243b3e0
	ctx.lr = 0x82432800;
	sub_8243B3E0(ctx, base);
	// 82432800: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82432804: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432808: 419A0008  beq cr6, 0x82432810
	if ctx.cr[6].eq {
	pc = 0x82432810; continue 'dispatch;
	}
	// 8243280C: 4BFF5235  bl 0x82427a40
	ctx.lr = 0x82432810;
	sub_82427A40(ctx, base);
	// 82432810: 817F0578  lwz r11, 0x578(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1400 as u32) ) } as u64;
	// 82432814: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82432818: 409A0020  bne cr6, 0x82432838
	if !ctx.cr[6].eq {
	pc = 0x82432838; continue 'dispatch;
	}
	// 8243281C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82432820: 93DF0580  stw r30, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[30].u32 ) };
	// 82432824: 93DF057C  stw r30, 0x57c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1404 as u32), ctx.r[30].u32 ) };
	// 82432828: 93DF0574  stw r30, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[30].u32 ) };
	// 8243282C: 93DF0598  stw r30, 0x598(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1432 as u32), ctx.r[30].u32 ) };
	// 82432830: 917F0594  stw r11, 0x594(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 82432834: 4800000C  b 0x82432840
	pc = 0x82432840; continue 'dispatch;
	// 82432838: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8243283C: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	// 82432840: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82432844: 93DF05AC  stw r30, 0x5ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1452 as u32), ctx.r[30].u32 ) };
	// 82432848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243284C: 48000EFD  bl 0x82433748
	ctx.lr = 0x82432850;
	sub_82433748(ctx, base);
	// 82432850: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432854: 481028B8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432858 size=120
    let mut pc: u32 = 0x82432858;
    'dispatch: loop {
        match pc {
            0x82432858 => {
    //   block [0x82432858..0x824328D0)
	// 82432858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243285C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432868: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243286C: 480007F5  bl 0x82433060
	ctx.lr = 0x82432870;
	sub_82433060(ctx, base);
	// 82432870: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82432874: 419A0024  beq cr6, 0x82432898
	if ctx.cr[6].eq {
	pc = 0x82432898; continue 'dispatch;
	}
	// 82432878: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243287C: 386B4974  addi r3, r11, 0x4974
	ctx.r[3].s64 = ctx.r[11].s64 + 18804;
	// 82432880: 48004849  bl 0x824370c8
	ctx.lr = 0x82432884;
	sub_824370C8(ctx, base);
	// 82432884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243288C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432894: 4E800020  blr
	return;
	// 82432898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243289C: 4BFFFEED  bl 0x82432788
	ctx.lr = 0x824328A0;
	sub_82432788(ctx, base);
	// 824328A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824328A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824328A8: 48000EB9  bl 0x82433760
	ctx.lr = 0x824328AC;
	sub_82433760(ctx, base);
	// 824328AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824328B0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824328B4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824328B8: 4BFF5189  bl 0x82427a40
	ctx.lr = 0x824328BC;
	sub_82427A40(ctx, base);
	// 824328BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824328C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824328C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824328C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824328CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824328D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824328D0 size=24
    let mut pc: u32 = 0x824328D0;
    'dispatch: loop {
        match pc {
            0x824328D0 => {
    //   block [0x824328D0..0x824328E8)
	// 824328D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824328D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824328D8: 814B01F0  lwz r10, 0x1f0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(496 as u32) ) } as u64;
	// 824328DC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824328E0: 419A0008  beq cr6, 0x824328e8
	if ctx.cr[6].eq {
		sub_824328E8(ctx, base);
		return;
	}
	// 824328E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824328E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824328E8 size=52
    let mut pc: u32 = 0x824328E8;
    'dispatch: loop {
        match pc {
            0x824328E8 => {
    //   block [0x824328E8..0x8243291C)
	// 824328E8: 814B0518  lwz r10, 0x518(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1304 as u32) ) } as u64;
	// 824328EC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824328F0: 409A0020  bne cr6, 0x82432910
	if !ctx.cr[6].eq {
	pc = 0x82432910; continue 'dispatch;
	}
	// 824328F4: 814B0534  lwz r10, 0x534(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1332 as u32) ) } as u64;
	// 824328F8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824328FC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82432900: 814A01F4  lwz r10, 0x1f4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(500 as u32) ) } as u64;
	// 82432904: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82432908: 409A0008  bne cr6, 0x82432910
	if !ctx.cr[6].eq {
	pc = 0x82432910; continue 'dispatch;
	}
	// 8243290C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432910: 814B053C  lwz r10, 0x53c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1340 as u32) ) } as u64;
	// 82432914: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82432918: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243291C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243291C size=24
    let mut pc: u32 = 0x8243291C;
    'dispatch: loop {
        match pc {
            0x8243291C => {
    //   block [0x8243291C..0x82432934)
	// 8243291C: 814B0558  lwz r10, 0x558(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1368 as u32) ) } as u64;
	// 82432920: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82432924: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82432928: 816B01F4  lwz r11, 0x1f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 8243292C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82432930: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432934 size=8
    let mut pc: u32 = 0x82432934;
    'dispatch: loop {
        match pc {
            0x82432934 => {
    //   block [0x82432934..0x8243293C)
	// 82432934: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432940 size=284
    let mut pc: u32 = 0x82432940;
    'dispatch: loop {
        match pc {
            0x82432940 => {
    //   block [0x82432940..0x82432A5C)
	// 82432940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432944: 48102779  bl 0x825350bc
	ctx.lr = 0x82432948;
	sub_82535080(ctx, base);
	// 82432948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243294C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432950: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82432954: 4800070D  bl 0x82433060
	ctx.lr = 0x82432958;
	sub_82433060(ctx, base);
	// 82432958: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243295C: 419A0018  beq cr6, 0x82432974
	if ctx.cr[6].eq {
	pc = 0x82432974; continue 'dispatch;
	}
	// 82432960: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432964: 386B49C0  addi r3, r11, 0x49c0
	ctx.r[3].s64 = ctx.r[11].s64 + 18880;
	// 82432968: 48004761  bl 0x824370c8
	ctx.lr = 0x8243296C;
	sub_824370C8(ctx, base);
	// 8243296C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432970: 4810279C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82432974: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 82432978: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243297C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82432980: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82432984: 409A000C  bne cr6, 0x82432990
	if !ctx.cr[6].eq {
	pc = 0x82432990; continue 'dispatch;
	}
	// 82432988: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8243298C: 419A00C8  beq cr6, 0x82432a54
	if ctx.cr[6].eq {
	pc = 0x82432A54; continue 'dispatch;
	}
	// 82432990: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82432994: 409A001C  bne cr6, 0x824329b0
	if !ctx.cr[6].eq {
	pc = 0x824329B0; continue 'dispatch;
	}
	// 82432998: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8243299C: 409A0014  bne cr6, 0x824329b0
	if !ctx.cr[6].eq {
	pc = 0x824329B0; continue 'dispatch;
	}
	// 824329A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824329A4: 4BFFFF2D  bl 0x824328d0
	ctx.lr = 0x824329A8;
	sub_824328D0(ctx, base);
	// 824329A8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824329AC: 409A00A8  bne cr6, 0x82432a54
	if !ctx.cr[6].eq {
	pc = 0x82432A54; continue 'dispatch;
	}
	// 824329B0: 48003F29  bl 0x824368d8
	ctx.lr = 0x824329B4;
	sub_824368D8(ctx, base);
	// 824329B4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824329B8: 409A003C  bne cr6, 0x824329f4
	if !ctx.cr[6].eq {
	pc = 0x824329F4; continue 'dispatch;
	}
	// 824329BC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824329C0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824329C4: 409A0030  bne cr6, 0x824329f4
	if !ctx.cr[6].eq {
	pc = 0x824329F4; continue 'dispatch;
	}
	// 824329C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824329CC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824329D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824329D4: 48009DED  bl 0x8243c7c0
	ctx.lr = 0x824329D8;
	sub_8243C7C0(ctx, base);
	// 824329D8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824329DC: 409A0010  bne cr6, 0x824329ec
	if !ctx.cr[6].eq {
	pc = 0x824329EC; continue 'dispatch;
	}
	// 824329E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824329E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824329E8: 409A000C  bne cr6, 0x824329f4
	if !ctx.cr[6].eq {
	pc = 0x824329F4; continue 'dispatch;
	}
	// 824329EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824329F0: 480099A1  bl 0x8243c390
	ctx.lr = 0x824329F4;
	sub_8243C390(ctx, base);
	// 824329F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824329F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824329FC: 48006865  bl 0x82439260
	ctx.lr = 0x82432A00;
	sub_82439260(ctx, base);
	// 82432A00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432A04: 419A0034  beq cr6, 0x82432a38
	if ctx.cr[6].eq {
	pc = 0x82432A38; continue 'dispatch;
	}
	// 82432A08: 3860FECA  li r3, -0x136
	ctx.r[3].s64 = -310;
	// 82432A0C: 48003FBD  bl 0x824369c8
	ctx.lr = 0x82432A10;
	sub_824369C8(ctx, base);
	// 82432A10: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82432A14: 409A0010  bne cr6, 0x82432a24
	if !ctx.cr[6].eq {
	pc = 0x82432A24; continue 'dispatch;
	}
	// 82432A18: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82432A1C: 388BB5DC  addi r4, r11, -0x4a24
	ctx.r[4].s64 = ctx.r[11].s64 + -18980;
	// 82432A20: 4800000C  b 0x82432a2c
	pc = 0x82432A2C; continue 'dispatch;
	// 82432A24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82432A28: 388BB5E0  addi r4, r11, -0x4a20
	ctx.r[4].s64 = ctx.r[11].s64 + -18976;
	// 82432A2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432A30: 386B499C  addi r3, r11, 0x499c
	ctx.r[3].s64 = ctx.r[11].s64 + 18844;
	// 82432A34: 48004695  bl 0x824370c8
	ctx.lr = 0x82432A38;
	sub_824370C8(ctx, base);
	// 82432A38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82432A3C: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 82432A40: 48008CE1  bl 0x8243b720
	ctx.lr = 0x82432A44;
	sub_8243B720(ctx, base);
	// 82432A44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82432A48: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 82432A4C: 48008CD5  bl 0x8243b720
	ctx.lr = 0x82432A50;
	sub_8243B720(ctx, base);
	// 82432A50: 9BBF0082  stb r29, 0x82(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(130 as u32), ctx.r[29].u8 ) };
	// 82432A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432A58: 481026B4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82432A60 size=80
    let mut pc: u32 = 0x82432A60;
    'dispatch: loop {
        match pc {
            0x82432A60 => {
    //   block [0x82432A60..0x82432AB0)
	// 82432A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432A6C: 80A3045C  lwz r5, 0x45c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1116 as u32) ) } as u64;
	// 82432A70: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82432A74: 7CAB07B4  extsw r11, r5
	ctx.r[11].s64 = ctx.r[5].s32 as i64;
	// 82432A78: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82432A7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82432A80: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82432A84: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82432A88: C80B2F58  lfd f0, 0x2f58(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(12120 as u32) ) };
	// 82432A8C: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82432A90: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82432A94: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82432A98: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82432A9C: 4800057D  bl 0x82433018
	ctx.lr = 0x82432AA0;
	sub_82433018(ctx, base);
	// 82432AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432AB0 size=72
    let mut pc: u32 = 0x82432AB0;
    'dispatch: loop {
        match pc {
            0x82432AB0 => {
    //   block [0x82432AB0..0x82432AF8)
	// 82432AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432ABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432AC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432AC4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82432AC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432ACC: 419A0018  beq cr6, 0x82432ae4
	if ctx.cr[6].eq {
	pc = 0x82432AE4; continue 'dispatch;
	}
	// 82432AD0: 48008949  bl 0x8243b418
	ctx.lr = 0x82432AD4;
	sub_8243B418(ctx, base);
	// 82432AD4: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82432AD8: 409A000C  bne cr6, 0x82432ae4
	if !ctx.cr[6].eq {
	pc = 0x82432AE4; continue 'dispatch;
	}
	// 82432ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432AE0: 4BFFFC61  bl 0x82432740
	ctx.lr = 0x82432AE4;
	sub_82432740(ctx, base);
	// 82432AE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432AF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432AF8 size=164
    let mut pc: u32 = 0x82432AF8;
    'dispatch: loop {
        match pc {
            0x82432AF8 => {
    //   block [0x82432AF8..0x82432B9C)
	// 82432AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432B00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82432B04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432B08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432B0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82432B10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432B14: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82432B18: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82432B1C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82432B20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82432B24: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82432B28: 409AFFF4  bne cr6, 0x82432b1c
	if !ctx.cr[6].eq {
	pc = 0x82432B1C; continue 'dispatch;
	}
	// 82432B2C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82432B30: 815F043C  lwz r10, 0x43c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1084 as u32) ) } as u64;
	// 82432B34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82432B38: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82432B3C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82432B40: 40990024  ble cr6, 0x82432b64
	if !ctx.cr[6].gt {
	pc = 0x82432B64; continue 'dispatch;
	}
	// 82432B44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432B48: 386B49E8  addi r3, r11, 0x49e8
	ctx.r[3].s64 = ctx.r[11].s64 + 18920;
	// 82432B4C: 4800457D  bl 0x824370c8
	ctx.lr = 0x82432B50;
	sub_824370C8(ctx, base);
	// 82432B50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82432B54: 80BF043C  lwz r5, 0x43c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1084 as u32) ) } as u64;
	// 82432B58: 807F0438  lwz r3, 0x438(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82432B5C: 48100065  bl 0x82532bc0
	ctx.lr = 0x82432B60;
	sub_82532BC0(ctx, base);
	// 82432B60: 48000024  b 0x82432b84
	pc = 0x82432B84; continue 'dispatch;
	// 82432B64: 815F0438  lwz r10, 0x438(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82432B68: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82432B6C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82432B70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82432B74: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82432B78: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82432B7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82432B80: 409AFFEC  bne cr6, 0x82432b6c
	if !ctx.cr[6].eq {
	pc = 0x82432B6C; continue 'dispatch;
	}
	// 82432B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82432B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432BA0 size=140
    let mut pc: u32 = 0x82432BA0;
    'dispatch: loop {
        match pc {
            0x82432BA0 => {
    //   block [0x82432BA0..0x82432C2C)
	// 82432BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432BA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432BAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432BB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432BB4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82432BB8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82432BBC: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82432BC0: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82432BC4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82432BC8: 80BF0438  lwz r5, 0x438(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82432BCC: 4BFE52DD  bl 0x82417ea8
	ctx.lr = 0x82432BD0;
	sub_82417EA8(ctx, base);
	// 82432BD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432BD4: 409A0038  bne cr6, 0x82432c0c
	if !ctx.cr[6].eq {
	pc = 0x82432C0C; continue 'dispatch;
	}
	// 82432BD8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82432BDC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82432BE0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82432BE4: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82432BE8: 917F0444  stw r11, 0x444(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1092 as u32), ctx.r[11].u32 ) };
	// 82432BEC: 915F0448  stw r10, 0x448(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1096 as u32), ctx.r[10].u32 ) };
	// 82432BF0: 913F044C  stw r9, 0x44c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1100 as u32), ctx.r[9].u32 ) };
	// 82432BF4: 911F0440  stw r8, 0x440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1088 as u32), ctx.r[8].u32 ) };
	// 82432BF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432C04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432C08: 4E800020  blr
	return;
	// 82432C0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432C10: 386B4A04  addi r3, r11, 0x4a04
	ctx.r[3].s64 = ctx.r[11].s64 + 18948;
	// 82432C14: 480044B5  bl 0x824370c8
	ctx.lr = 0x82432C18;
	sub_824370C8(ctx, base);
	// 82432C18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432C24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432C30 size=132
    let mut pc: u32 = 0x82432C30;
    'dispatch: loop {
        match pc {
            0x82432C30 => {
    //   block [0x82432C30..0x82432CB4)
	// 82432C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432C38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432C3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432C40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432C44: 4800041D  bl 0x82433060
	ctx.lr = 0x82432C48;
	sub_82433060(ctx, base);
	// 82432C48: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82432C4C: 419A0024  beq cr6, 0x82432c70
	if ctx.cr[6].eq {
	pc = 0x82432C70; continue 'dispatch;
	}
	// 82432C50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432C54: 386B4A2C  addi r3, r11, 0x4a2c
	ctx.r[3].s64 = ctx.r[11].s64 + 18988;
	// 82432C58: 48004471  bl 0x824370c8
	ctx.lr = 0x82432C5C;
	sub_824370C8(ctx, base);
	// 82432C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432C68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432C6C: 4E800020  blr
	return;
	// 82432C70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82432C74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82432C78: 419A0028  beq cr6, 0x82432ca0
	if ctx.cr[6].eq {
	pc = 0x82432CA0; continue 'dispatch;
	}
	// 82432C7C: 817F05AC  lwz r11, 0x5ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82432C80: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82432C84: 419A001C  beq cr6, 0x82432ca0
	if ctx.cr[6].eq {
	pc = 0x82432CA0; continue 'dispatch;
	}
	// 82432C88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82432C8C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432C90: 917F05AC  stw r11, 0x5ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1452 as u32), ctx.r[11].u32 ) };
	// 82432C94: 480070C5  bl 0x82439d58
	ctx.lr = 0x82432C98;
	sub_82439D58(ctx, base);
	// 82432C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432C9C: 480092A5  bl 0x8243bf40
	ctx.lr = 0x82432CA0;
	sub_8243BF40(ctx, base);
	// 82432CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432CAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432CB8 size=92
    let mut pc: u32 = 0x82432CB8;
    'dispatch: loop {
        match pc {
            0x82432CB8 => {
    //   block [0x82432CB8..0x82432D14)
	// 82432CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432CC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432CC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432CCC: 480092B5  bl 0x8243bf80
	ctx.lr = 0x82432CD0;
	sub_8243BF80(ctx, base);
	// 82432CD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432CD4: 409A0018  bne cr6, 0x82432cec
	if !ctx.cr[6].eq {
	pc = 0x82432CEC; continue 'dispatch;
	}
	// 82432CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432CE8: 4E800020  blr
	return;
	// 82432CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432CF0: 48009171  bl 0x8243be60
	ctx.lr = 0x82432CF4;
	sub_8243BE60(ctx, base);
	// 82432CF4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82432CF8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82432CFC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82432D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432D0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432D18 size=300
    let mut pc: u32 = 0x82432D18;
    'dispatch: loop {
        match pc {
            0x82432D18 => {
    //   block [0x82432D18..0x82432E44)
	// 82432D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432D1C: 4810239D  bl 0x825350b8
	ctx.lr = 0x82432D20;
	sub_82535080(ctx, base);
	// 82432D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432D24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432D28: 48003A61  bl 0x82436788
	ctx.lr = 0x82432D2C;
	sub_82436788(ctx, base);
	// 82432D2C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82432D30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82432D34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D38: 939F05AC  stw r28, 0x5ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1452 as u32), ctx.r[28].u32 ) };
	// 82432D3C: 48000A0D  bl 0x82433748
	ctx.lr = 0x82432D40;
	sub_82433748(ctx, base);
	// 82432D40: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432D44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82432D48: 419A007C  beq cr6, 0x82432dc4
	if ctx.cr[6].eq {
	pc = 0x82432DC4; continue 'dispatch;
	}
	// 82432D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D50: 48002639  bl 0x82435388
	ctx.lr = 0x82432D54;
	sub_82435388(ctx, base);
	// 82432D54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432D58: 419A0018  beq cr6, 0x82432d70
	if ctx.cr[6].eq {
	pc = 0x82432D70; continue 'dispatch;
	}
	// 82432D5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432D60: 386B4A88  addi r3, r11, 0x4a88
	ctx.r[3].s64 = ctx.r[11].s64 + 19080;
	// 82432D64: 48004365  bl 0x824370c8
	ctx.lr = 0x82432D68;
	sub_824370C8(ctx, base);
	// 82432D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432D6C: 4810239C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82432D70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82432D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D78: 48008A49  bl 0x8243b7c0
	ctx.lr = 0x82432D7C;
	sub_8243B7C0(ctx, base);
	// 82432D7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82432D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D84: 48008A3D  bl 0x8243b7c0
	ctx.lr = 0x82432D88;
	sub_8243B7C0(ctx, base);
	// 82432D88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D8C: 4BFFF35D  bl 0x824320e8
	ctx.lr = 0x82432D90;
	sub_824320E8(ctx, base);
	// 82432D90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D94: 4BFFF2CD  bl 0x82432060
	ctx.lr = 0x82432D98;
	sub_82432060(ctx, base);
	// 82432D98: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432D9C: 419A0018  beq cr6, 0x82432db4
	if ctx.cr[6].eq {
	pc = 0x82432DB4; continue 'dispatch;
	}
	// 82432DA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432DA4: 386B4A5C  addi r3, r11, 0x4a5c
	ctx.r[3].s64 = ctx.r[11].s64 + 19036;
	// 82432DA8: 48004321  bl 0x824370c8
	ctx.lr = 0x82432DAC;
	sub_824370C8(ctx, base);
	// 82432DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432DB0: 48102358  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82432DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432DB8: 4BFFF311  bl 0x824320c8
	ctx.lr = 0x82432DBC;
	sub_824320C8(ctx, base);
	// 82432DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432DC0: 48001651  bl 0x82434410
	ctx.lr = 0x82432DC4;
	sub_82434410(ctx, base);
	// 82432DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432DC8: 4BFFF9B1  bl 0x82432778
	ctx.lr = 0x82432DCC;
	sub_82432778(ctx, base);
	// 82432DCC: 4BFFF8C5  bl 0x82432690
	ctx.lr = 0x82432DD0;
	sub_82432690(ctx, base);
	// 82432DD0: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 82432DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432DD8: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82432DDC: 4BFFFB65  bl 0x82432940
	ctx.lr = 0x82432DE0;
	sub_82432940(ctx, base);
	// 82432DE0: 3BDF0518  addi r30, r31, 0x518
	ctx.r[30].s64 = ctx.r[31].s64 + 1304;
	// 82432DE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82432DE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82432DEC: 48008935  bl 0x8243b720
	ctx.lr = 0x82432DF0;
	sub_8243B720(ctx, base);
	// 82432DF0: 3BBF053C  addi r29, r31, 0x53c
	ctx.r[29].s64 = ctx.r[31].s64 + 1340;
	// 82432DF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82432DF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82432DFC: 48008925  bl 0x8243b720
	ctx.lr = 0x82432E00;
	sub_8243B720(ctx, base);
	// 82432E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82432E04: 480087FD  bl 0x8243b600
	ctx.lr = 0x82432E08;
	sub_8243B600(ctx, base);
	// 82432E08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82432E0C: 480087F5  bl 0x8243b600
	ctx.lr = 0x82432E10;
	sub_8243B600(ctx, base);
	// 82432E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432E14: 809F007C  lwz r4, 0x7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82432E18: 480001A9  bl 0x82432fc0
	ctx.lr = 0x82432E1C;
	sub_82432FC0(ctx, base);
	// 82432E1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432E20: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82432E24: 480001AD  bl 0x82432fd0
	ctx.lr = 0x82432E28;
	sub_82432FD0(ctx, base);
	// 82432E28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82432E2C: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 82432E30: 9B9F0081  stb r28, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[28].u8 ) };
	// 82432E34: 939F056C  stw r28, 0x56c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1388 as u32), ctx.r[28].u32 ) };
	// 82432E38: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82432E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432E40: 481022C8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432E48 size=140
    let mut pc: u32 = 0x82432E48;
    'dispatch: loop {
        match pc {
            0x82432E48 => {
    //   block [0x82432E48..0x82432ED4)
	// 82432E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82432E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432E60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82432E64: 480001FD  bl 0x82433060
	ctx.lr = 0x82432E68;
	sub_82433060(ctx, base);
	// 82432E68: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82432E6C: 419A0014  beq cr6, 0x82432e80
	if ctx.cr[6].eq {
	pc = 0x82432E80; continue 'dispatch;
	}
	// 82432E70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432E74: 386B4AB0  addi r3, r11, 0x4ab0
	ctx.r[3].s64 = ctx.r[11].s64 + 19120;
	// 82432E78: 48004251  bl 0x824370c8
	ctx.lr = 0x82432E7C;
	sub_824370C8(ctx, base);
	// 82432E7C: 48000040  b 0x82432ebc
	pc = 0x82432EBC; continue 'dispatch;
	// 82432E80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82432E84: 48009135  bl 0x8243bfb8
	ctx.lr = 0x82432E88;
	sub_8243BFB8(ctx, base);
	// 82432E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432E8C: 4BFFF8FD  bl 0x82432788
	ctx.lr = 0x82432E90;
	sub_82432788(ctx, base);
	// 82432E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82432E94: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82432E98: 93DF0450  stw r30, 0x450(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1104 as u32), ctx.r[30].u32 ) };
	// 82432E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432EA0: 917F0468  stw r11, 0x468(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1128 as u32), ctx.r[11].u32 ) };
	// 82432EA4: 915F0464  stw r10, 0x464(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1124 as u32), ctx.r[10].u32 ) };
	// 82432EA8: 917F046C  stw r11, 0x46c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1132 as u32), ctx.r[11].u32 ) };
	// 82432EAC: 917F0470  stw r11, 0x470(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1136 as u32), ctx.r[11].u32 ) };
	// 82432EB0: 4BFFFE69  bl 0x82432d18
	ctx.lr = 0x82432EB4;
	sub_82432D18(ctx, base);
	// 82432EB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432EB8: 48009101  bl 0x8243bfb8
	ctx.lr = 0x82432EBC;
	sub_8243BFB8(ctx, base);
	// 82432EBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432EC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82432ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432ED8 size=56
    let mut pc: u32 = 0x82432ED8;
    'dispatch: loop {
        match pc {
            0x82432ED8 => {
    //   block [0x82432ED8..0x82432F10)
	// 82432ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432EE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432EE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432EE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432EEC: 480041B5  bl 0x824370a0
	ctx.lr = 0x82432EF0;
	sub_824370A0(ctx, base);
	// 82432EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432EF4: 4BFFF965  bl 0x82432858
	ctx.lr = 0x82432EF8;
	sub_82432858(ctx, base);
	// 82432EF8: 480041B9  bl 0x824370b0
	ctx.lr = 0x82432EFC;
	sub_824370B0(ctx, base);
	// 82432EFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432F08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432F10 size=72
    let mut pc: u32 = 0x82432F10;
    'dispatch: loop {
        match pc {
            0x82432F10 => {
    //   block [0x82432F10..0x82432F58)
	// 82432F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432F18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82432F1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432F24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432F28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82432F2C: 48004175  bl 0x824370a0
	ctx.lr = 0x82432F30;
	sub_824370A0(ctx, base);
	// 82432F30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82432F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432F38: 4BFFFF11  bl 0x82432e48
	ctx.lr = 0x82432F3C;
	sub_82432E48(ctx, base);
	// 82432F3C: 48004175  bl 0x824370b0
	ctx.lr = 0x82432F40;
	sub_824370B0(ctx, base);
	// 82432F40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432F4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82432F50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432F58 size=100
    let mut pc: u32 = 0x82432F58;
    'dispatch: loop {
        match pc {
            0x82432F58 => {
    //   block [0x82432F58..0x82432FBC)
	// 82432F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432F5C: 48102161  bl 0x825350bc
	ctx.lr = 0x82432F60;
	sub_82535080(ctx, base);
	// 82432F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432F68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82432F6C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82432F70: 480000F1  bl 0x82433060
	ctx.lr = 0x82432F74;
	sub_82433060(ctx, base);
	// 82432F74: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82432F78: 419A0018  beq cr6, 0x82432f90
	if ctx.cr[6].eq {
	pc = 0x82432F90; continue 'dispatch;
	}
	// 82432F7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432F80: 386B4ADC  addi r3, r11, 0x4adc
	ctx.r[3].s64 = ctx.r[11].s64 + 19164;
	// 82432F84: 48004145  bl 0x824370c8
	ctx.lr = 0x82432F88;
	sub_824370C8(ctx, base);
	// 82432F88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432F8C: 48102180  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82432F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432F94: 480003ED  bl 0x82433380
	ctx.lr = 0x82432F98;
	sub_82433380(ctx, base);
	// 82432F98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82432F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432FA0: 4BFFFF71  bl 0x82432f10
	ctx.lr = 0x82432FA4;
	sub_82432F10(ctx, base);
	// 82432FA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82432FA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82432FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432FB0: 4BFFFBF1  bl 0x82432ba0
	ctx.lr = 0x82432FB4;
	sub_82432BA0(ctx, base);
	// 82432FB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432FB8: 48102154  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FC0 size=16
    let mut pc: u32 = 0x82432FC0;
    'dispatch: loop {
        match pc {
            0x82432FC0 => {
    //   block [0x82432FC0..0x82432FD0)
	// 82432FC0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82432FC4: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432FC8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82432FCC: 4800992C  b 0x8243c8f8
	sub_8243C8F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FD0 size=16
    let mut pc: u32 = 0x82432FD0;
    'dispatch: loop {
        match pc {
            0x82432FD0 => {
    //   block [0x82432FD0..0x82432FE0)
	// 82432FD0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82432FD4: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432FD8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82432FDC: 4800991C  b 0x8243c8f8
	sub_8243C8F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FE0 size=16
    let mut pc: u32 = 0x82432FE0;
    'dispatch: loop {
        match pc {
            0x82432FE0 => {
    //   block [0x82432FE0..0x82432FF0)
	// 82432FE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432FE4: 419A000C  beq cr6, 0x82432ff0
	if ctx.cr[6].eq {
		sub_82432FF0(ctx, base);
		return;
	}
	// 82432FE8: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432FEC: 48000008  b 0x82432ff4
	sub_82432FF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FF0 size=8
    let mut pc: u32 = 0x82432FF0;
    'dispatch: loop {
        match pc {
            0x82432FF0 => {
    //   block [0x82432FF0..0x82432FF8)
	// 82432FF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432FF4: 48009904  b 0x8243c8f8
	sub_8243C8F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FF8 size=16
    let mut pc: u32 = 0x82432FF8;
    'dispatch: loop {
        match pc {
            0x82432FF8 => {
    //   block [0x82432FF8..0x82433008)
	// 82432FF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432FFC: 419A000C  beq cr6, 0x82433008
	if ctx.cr[6].eq {
		sub_82433008(ctx, base);
		return;
	}
	// 82433000: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433004: 48000008  b 0x8243300c
	sub_82433008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433008 size=8
    let mut pc: u32 = 0x82433008;
    'dispatch: loop {
        match pc {
            0x82433008 => {
    //   block [0x82433008..0x82433010)
	// 82433008: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243300C: 480097B4  b 0x8243c7c0
	sub_8243C7C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433010 size=8
    let mut pc: u32 = 0x82433010;
    'dispatch: loop {
        match pc {
            0x82433010 => {
    //   block [0x82433010..0x82433018)
	// 82433010: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82433014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433018 size=72
    let mut pc: u32 = 0x82433018;
    'dispatch: loop {
        match pc {
            0x82433018 => {
    //   block [0x82433018..0x82433060)
	// 82433018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82433024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243302C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82433034: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82433038: 48008321  bl 0x8243b358
	ctx.lr = 0x8243303C;
	sub_8243B358(ctx, base);
	// 8243303C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433044: 480006ED  bl 0x82433730
	ctx.lr = 0x82433048;
	sub_82433730(ctx, base);
	// 82433048: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243304C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433054: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243305C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433060 size=12
    let mut pc: u32 = 0x82433060;
    'dispatch: loop {
        match pc {
            0x82433060 => {
    //   block [0x82433060..0x8243306C)
	// 82433060: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433064: 409A0008  bne cr6, 0x8243306c
	if !ctx.cr[6].eq {
		sub_8243306C(ctx, base);
		return;
	}
	// 82433068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243306C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243306C size=8
    let mut pc: u32 = 0x8243306C;
    'dispatch: loop {
        match pc {
            0x8243306C => {
    //   block [0x8243306C..0x82433074)
	// 8243306C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82433070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433078 size=172
    let mut pc: u32 = 0x82433078;
    'dispatch: loop {
        match pc {
            0x82433078 => {
    //   block [0x82433078..0x82433124)
	// 82433078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243307C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433084: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82433088: 4BFFFFD9  bl 0x82433060
	ctx.lr = 0x8243308C;
	sub_82433060(ctx, base);
	// 8243308C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433090: 419A002C  beq cr6, 0x824330bc
	if ctx.cr[6].eq {
	pc = 0x824330BC; continue 'dispatch;
	}
	// 82433094: 3860FFF4  li r3, -0xc
	ctx.r[3].s64 = -12;
	// 82433098: 48003931  bl 0x824369c8
	ctx.lr = 0x8243309C;
	sub_824369C8(ctx, base);
	// 8243309C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824330A0: 386B4B08  addi r3, r11, 0x4b08
	ctx.r[3].s64 = ctx.r[11].s64 + 19208;
	// 824330A4: 48004025  bl 0x824370c8
	ctx.lr = 0x824330A8;
	sub_824370C8(ctx, base);
	// 824330A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824330AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824330B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824330B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824330B8: 4E800020  blr
	return;
	// 824330BC: 806A0004  lwz r3, 4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824330C0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 824330C4: 409A0050  bne cr6, 0x82433114
	if !ctx.cr[6].eq {
	pc = 0x82433114; continue 'dispatch;
	}
	// 824330C8: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 824330CC: 4800957D  bl 0x8243c648
	ctx.lr = 0x824330D0;
	sub_8243C648(ctx, base);
	// 824330D0: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 824330D4: 419A003C  beq cr6, 0x82433110
	if ctx.cr[6].eq {
	pc = 0x82433110; continue 'dispatch;
	}
	// 824330D8: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 824330DC: 419A0034  beq cr6, 0x82433110
	if ctx.cr[6].eq {
	pc = 0x82433110; continue 'dispatch;
	}
	// 824330E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824330E4: 40980018  bge cr6, 0x824330fc
	if !ctx.cr[6].lt {
	pc = 0x824330FC; continue 'dispatch;
	}
	// 824330E8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 824330EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824330F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824330F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824330F8: 4E800020  blr
	return;
	// 824330FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82433100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243310C: 4E800020  blr
	return;
	// 82433110: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82433114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243311C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433128 size=124
    let mut pc: u32 = 0x82433128;
    'dispatch: loop {
        match pc {
            0x82433128 => {
    //   block [0x82433128..0x824331A4)
	// 82433128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243312C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82433134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243313C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433140: 817F05A0  lwz r11, 0x5a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1440 as u32) ) } as u64;
	// 82433144: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433148: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243314C: 409A0040  bne cr6, 0x8243318c
	if !ctx.cr[6].eq {
	pc = 0x8243318C; continue 'dispatch;
	}
	// 82433150: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82433154: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82433158: 4800BDA9  bl 0x8243ef00
	ctx.lr = 0x8243315C;
	sub_8243EF00(ctx, base);
	// 8243315C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433160: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82433164: 419A0028  beq cr6, 0x8243318c
	if ctx.cr[6].eq {
	pc = 0x8243318C; continue 'dispatch;
	}
	// 82433168: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243316C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82433170: 4800C251  bl 0x8243f3c0
	ctx.lr = 0x82433174;
	sub_8243F3C0(ctx, base);
	// 82433174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82433178: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243317C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82433180: 4800C0C1  bl 0x8243f240
	ctx.lr = 0x82433184;
	sub_8243F240(ctx, base);
	// 82433184: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82433188: 917F05A0  stw r11, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[11].u32 ) };
	// 8243318C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433198: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243319C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824331A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824331A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824331A8 size=316
    let mut pc: u32 = 0x824331A8;
    'dispatch: loop {
        match pc {
            0x824331A8 => {
    //   block [0x824331A8..0x824332E4)
	// 824331A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824331AC: 48101F11  bl 0x825350bc
	ctx.lr = 0x824331B0;
	sub_82535080(ctx, base);
	// 824331B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824331B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824331B8: 4BFFFEA9  bl 0x82433060
	ctx.lr = 0x824331BC;
	sub_82433060(ctx, base);
	// 824331BC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824331C0: 419A0018  beq cr6, 0x824331d8
	if ctx.cr[6].eq {
	pc = 0x824331D8; continue 'dispatch;
	}
	// 824331C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824331C8: 386B4B50  addi r3, r11, 0x4b50
	ctx.r[3].s64 = ctx.r[11].s64 + 19280;
	// 824331CC: 48003EFD  bl 0x824370c8
	ctx.lr = 0x824331D0;
	sub_824370C8(ctx, base);
	// 824331D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824331D4: 48101F38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824331D8: 83BF0048  lwz r29, 0x48(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824331DC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824331E0: 409A0040  bne cr6, 0x82433220
	if !ctx.cr[6].eq {
	pc = 0x82433220; continue 'dispatch;
	}
	// 824331E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824331E8: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 824331EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824331F0: 48009709  bl 0x8243c8f8
	ctx.lr = 0x824331F4;
	sub_8243C8F8(ctx, base);
	// 824331F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824331F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824331FC: 917F059C  stw r11, 0x59c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1436 as u32), ctx.r[11].u32 ) };
	// 82433200: 917F05A0  stw r11, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[11].u32 ) };
	// 82433204: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82433208: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243320C: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433210: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433214: 4800C02D  bl 0x8243f240
	ctx.lr = 0x82433218;
	sub_8243F240(ctx, base);
	// 82433218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243321C: 48101EF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82433220: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82433224: 409A000C  bne cr6, 0x82433230
	if !ctx.cr[6].eq {
	pc = 0x82433230; continue 'dispatch;
	}
	// 82433228: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243322C: 4BFFFFBC  b 0x824331e8
	pc = 0x824331E8; continue 'dispatch;
	// 82433230: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82433234: 409A0028  bne cr6, 0x8243325c
	if !ctx.cr[6].eq {
	pc = 0x8243325C; continue 'dispatch;
	}
	// 82433238: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243323C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82433240: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82433244: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82433248: 409A000C  bne cr6, 0x82433254
	if !ctx.cr[6].eq {
	pc = 0x82433254; continue 'dispatch;
	}
	// 8243324C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82433250: 4BFFFFA0  b 0x824331f0
	pc = 0x824331F0; continue 'dispatch;
	// 82433254: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82433258: 4BFFFF98  b 0x824331f0
	pc = 0x824331F0; continue 'dispatch;
	// 8243325C: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82433260: 409A000C  bne cr6, 0x8243326c
	if !ctx.cr[6].eq {
	pc = 0x8243326C; continue 'dispatch;
	}
	// 82433264: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82433268: 4BFFFF80  b 0x824331e8
	pc = 0x824331E8; continue 'dispatch;
	// 8243326C: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 82433270: 409A000C  bne cr6, 0x8243327c
	if !ctx.cr[6].eq {
	pc = 0x8243327C; continue 'dispatch;
	}
	// 82433274: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82433278: 4BFFFF70  b 0x824331e8
	pc = 0x824331E8; continue 'dispatch;
	// 8243327C: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 82433280: 409A003C  bne cr6, 0x824332bc
	if !ctx.cr[6].eq {
	pc = 0x824332BC; continue 'dispatch;
	}
	// 82433284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82433288: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8243328C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82433290: 48009669  bl 0x8243c8f8
	ctx.lr = 0x82433294;
	sub_8243C8F8(ctx, base);
	// 82433294: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82433298: 817F05A8  lwz r11, 0x5a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1448 as u32) ) } as u64;
	// 8243329C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824332A0: 93DF059C  stw r30, 0x59c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1436 as u32), ctx.r[30].u32 ) };
	// 824332A4: 419A0028  beq cr6, 0x824332cc
	if ctx.cr[6].eq {
	pc = 0x824332CC; continue 'dispatch;
	}
	// 824332A8: 813F05A4  lwz r9, 0x5a4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1444 as u32) ) } as u64;
	// 824332AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824332B0: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 824332B4: 915F05A0  stw r10, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[10].u32 ) };
	// 824332B8: 4BFFFF4C  b 0x82433204
	pc = 0x82433204; continue 'dispatch;
	// 824332BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824332C0: 386B4B30  addi r3, r11, 0x4b30
	ctx.r[3].s64 = ctx.r[11].s64 + 19248;
	// 824332C4: 48003E05  bl 0x824370c8
	ctx.lr = 0x824332C8;
	sub_824370C8(ctx, base);
	// 824332C8: 4BFFFF2C  b 0x824331f4
	pc = 0x824331F4; continue 'dispatch;
	// 824332CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824332D0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824332D4: 38607530  li r3, 0x7530
	ctx.r[3].s64 = 30000;
	// 824332D8: 4800C0E9  bl 0x8243f3c0
	ctx.lr = 0x824332DC;
	sub_8243F3C0(ctx, base);
	// 824332DC: 93DF05A0  stw r30, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[30].u32 ) };
	// 824332E0: 4BFFFF28  b 0x82433208
	pc = 0x82433208; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824332E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824332E8 size=152
    let mut pc: u32 = 0x824332E8;
    'dispatch: loop {
        match pc {
            0x824332E8 => {
    //   block [0x824332E8..0x82433380)
	// 824332E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824332EC: 48101DCD  bl 0x825350b8
	ctx.lr = 0x824332F0;
	sub_82535080(ctx, base);
	// 824332F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824332F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824332F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824332FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82433300: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82433304: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82433308: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8243330C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82433310: 4BFFFD51  bl 0x82433060
	ctx.lr = 0x82433314;
	sub_82433060(ctx, base);
	// 82433314: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433318: 419A0018  beq cr6, 0x82433330
	if ctx.cr[6].eq {
	pc = 0x82433330; continue 'dispatch;
	}
	// 8243331C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433320: 386B4BA8  addi r3, r11, 0x4ba8
	ctx.r[3].s64 = ctx.r[11].s64 + 19368;
	// 82433324: 48003DA5  bl 0x824370c8
	ctx.lr = 0x82433328;
	sub_824370C8(ctx, base);
	// 82433328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243332C: 48101DDC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82433330: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433334: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433338: 419A0040  beq cr6, 0x82433378
	if ctx.cr[6].eq {
	pc = 0x82433378; continue 'dispatch;
	}
	// 8243333C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82433340: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82433344: 4800CAE5  bl 0x8243fe28
	ctx.lr = 0x82433348;
	sub_8243FE28(ctx, base);
	// 82433348: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243334C: 419A0018  beq cr6, 0x82433364
	if ctx.cr[6].eq {
	pc = 0x82433364; continue 'dispatch;
	}
	// 82433350: 3860FECB  li r3, -0x135
	ctx.r[3].s64 = -309;
	// 82433354: 48003675  bl 0x824369c8
	ctx.lr = 0x82433358;
	sub_824369C8(ctx, base);
	// 82433358: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243335C: 386B4B80  addi r3, r11, 0x4b80
	ctx.r[3].s64 = ctx.r[11].s64 + 19328;
	// 82433360: 48003D69  bl 0x824370c8
	ctx.lr = 0x82433364;
	sub_824370C8(ctx, base);
	// 82433364: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82433368: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243336C: 4098000C  bge cr6, 0x82433378
	if !ctx.cr[6].lt {
	pc = 0x82433378; continue 'dispatch;
	}
	// 82433370: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82433374: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82433378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243337C: 48101D8C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433380 size=80
    let mut pc: u32 = 0x82433380;
    'dispatch: loop {
        match pc {
            0x82433380 => {
    //   block [0x82433380..0x824333D0)
	// 82433380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243338C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82433390: 4BFFFCD1  bl 0x82433060
	ctx.lr = 0x82433394;
	sub_82433060(ctx, base);
	// 82433394: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433398: 419A0024  beq cr6, 0x824333bc
	if ctx.cr[6].eq {
	pc = 0x824333BC; continue 'dispatch;
	}
	// 8243339C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824333A0: 386B4BD4  addi r3, r11, 0x4bd4
	ctx.r[3].s64 = ctx.r[11].s64 + 19412;
	// 824333A4: 48003D25  bl 0x824370c8
	ctx.lr = 0x824333A8;
	sub_824370C8(ctx, base);
	// 824333A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824333AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824333B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824333B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824333B8: 4E800020  blr
	return;
	// 824333BC: 806A0454  lwz r3, 0x454(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1108 as u32) ) } as u64;
	// 824333C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824333C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824333C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824333CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824333D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824333D0 size=8
    let mut pc: u32 = 0x824333D0;
    'dispatch: loop {
        match pc {
            0x824333D0 => {
    //   block [0x824333D0..0x824333D8)
	// 824333D0: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 824333D4: 480095C4  b 0x8243c998
	sub_8243C998(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824333D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824333D8 size=88
    let mut pc: u32 = 0x824333D8;
    'dispatch: loop {
        match pc {
            0x824333D8 => {
    //   block [0x824333D8..0x82433430)
	// 824333D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824333DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824333E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824333E4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824333E8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824333EC: 4BFFFC75  bl 0x82433060
	ctx.lr = 0x824333F0;
	sub_82433060(ctx, base);
	// 824333F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824333F4: 419A0020  beq cr6, 0x82433414
	if ctx.cr[6].eq {
	pc = 0x82433414; continue 'dispatch;
	}
	// 824333F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824333FC: 386B4C04  addi r3, r11, 0x4c04
	ctx.r[3].s64 = ctx.r[11].s64 + 19460;
	// 82433400: 48003CC9  bl 0x824370c8
	ctx.lr = 0x82433404;
	sub_824370C8(ctx, base);
	// 82433404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243340C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433410: 4E800020  blr
	return;
	// 82433414: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 82433418: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243341C: 480094DD  bl 0x8243c8f8
	ctx.lr = 0x82433420;
	sub_8243C8F8(ctx, base);
	// 82433420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243342C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433430 size=80
    let mut pc: u32 = 0x82433430;
    'dispatch: loop {
        match pc {
            0x82433430 => {
    //   block [0x82433430..0x82433480)
	// 82433430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243343C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82433440: 4BFFFC21  bl 0x82433060
	ctx.lr = 0x82433444;
	sub_82433060(ctx, base);
	// 82433444: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433448: 419A0024  beq cr6, 0x8243346c
	if ctx.cr[6].eq {
	pc = 0x8243346C; continue 'dispatch;
	}
	// 8243344C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433450: 386B4C38  addi r3, r11, 0x4c38
	ctx.r[3].s64 = ctx.r[11].s64 + 19512;
	// 82433454: 48003C75  bl 0x824370c8
	ctx.lr = 0x82433458;
	sub_824370C8(ctx, base);
	// 82433458: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243345C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433468: 4E800020  blr
	return;
	// 8243346C: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243347C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433480 size=108
    let mut pc: u32 = 0x82433480;
    'dispatch: loop {
        match pc {
            0x82433480 => {
    //   block [0x82433480..0x824334EC)
	// 82433480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433488: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243348C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433494: 4BFFFBE5  bl 0x82433078
	ctx.lr = 0x82433498;
	sub_82433078(ctx, base);
	// 82433498: 817F0570  lwz r11, 0x570(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1392 as u32) ) } as u64;
	// 8243349C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824334A0: 409A0038  bne cr6, 0x824334d8
	if !ctx.cr[6].eq {
	pc = 0x824334D8; continue 'dispatch;
	}
	// 824334A4: 817F0574  lwz r11, 0x574(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1396 as u32) ) } as u64;
	// 824334A8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824334AC: 409A002C  bne cr6, 0x824334d8
	if !ctx.cr[6].eq {
	pc = 0x824334D8; continue 'dispatch;
	}
	// 824334B0: 817F0580  lwz r11, 0x580(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1408 as u32) ) } as u64;
	// 824334B4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824334B8: 419A000C  beq cr6, 0x824334c4
	if ctx.cr[6].eq {
	pc = 0x824334C4; continue 'dispatch;
	}
	// 824334BC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824334C0: 409A0018  bne cr6, 0x824334d8
	if !ctx.cr[6].eq {
	pc = 0x824334D8; continue 'dispatch;
	}
	// 824334C4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824334C8: 419A000C  beq cr6, 0x824334d4
	if ctx.cr[6].eq {
	pc = 0x824334D4; continue 'dispatch;
	}
	// 824334CC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 824334D0: 409A0008  bne cr6, 0x824334d8
	if !ctx.cr[6].eq {
	pc = 0x824334D8; continue 'dispatch;
	}
	// 824334D4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824334D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824334DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824334E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824334E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824334E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824334F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824334F0 size=164
    let mut pc: u32 = 0x824334F0;
    'dispatch: loop {
        match pc {
            0x824334F0 => {
    //   block [0x824334F0..0x82433594)
	// 824334F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824334F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824334F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824334FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433508: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243350C: 4BFFFB55  bl 0x82433060
	ctx.lr = 0x82433510;
	sub_82433060(ctx, base);
	// 82433510: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433514: 419A0014  beq cr6, 0x82433528
	if ctx.cr[6].eq {
	pc = 0x82433528; continue 'dispatch;
	}
	// 82433518: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243351C: 386B4CBC  addi r3, r11, 0x4cbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19644;
	// 82433520: 48003BA9  bl 0x824370c8
	ctx.lr = 0x82433524;
	sub_824370C8(ctx, base);
	// 82433524: 48000058  b 0x8243357c
	pc = 0x8243357C; continue 'dispatch;
	// 82433528: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243352C: 409A0014  bne cr6, 0x82433540
	if !ctx.cr[6].eq {
	pc = 0x82433540; continue 'dispatch;
	}
	// 82433530: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433534: 386B4C94  addi r3, r11, 0x4c94
	ctx.r[3].s64 = ctx.r[11].s64 + 19604;
	// 82433538: 48003B91  bl 0x824370c8
	ctx.lr = 0x8243353C;
	sub_824370C8(ctx, base);
	// 8243353C: 48000040  b 0x8243357c
	pc = 0x8243357C; continue 'dispatch;
	// 82433540: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433544: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433548: 4BFF4499  bl 0x824279e0
	ctx.lr = 0x8243354C;
	sub_824279E0(ctx, base);
	// 8243354C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82433550: 40980020  bge cr6, 0x82433570
	if !ctx.cr[6].lt {
	pc = 0x82433570; continue 'dispatch;
	}
	// 82433554: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82433558: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243355C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433560: 386B4C64  addi r3, r11, 0x4c64
	ctx.r[3].s64 = ctx.r[11].s64 + 19556;
	// 82433564: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82433568: 48003B61  bl 0x824370c8
	ctx.lr = 0x8243356C;
	sub_824370C8(ctx, base);
	// 8243356C: 48000010  b 0x8243357c
	pc = 0x8243357C; continue 'dispatch;
	// 82433570: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82433574: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82433578: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8243357C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243358C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433598 size=92
    let mut pc: u32 = 0x82433598;
    'dispatch: loop {
        match pc {
            0x82433598 => {
    //   block [0x82433598..0x824335F4)
	// 82433598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243359C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824335A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824335A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824335A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824335AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824335B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824335B4: 4BFFFAAD  bl 0x82433060
	ctx.lr = 0x824335B8;
	sub_82433060(ctx, base);
	// 824335B8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824335BC: 419A0014  beq cr6, 0x824335d0
	if ctx.cr[6].eq {
	pc = 0x824335D0; continue 'dispatch;
	}
	// 824335C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824335C4: 386B4CEC  addi r3, r11, 0x4cec
	ctx.r[3].s64 = ctx.r[11].s64 + 19692;
	// 824335C8: 48003B01  bl 0x824370c8
	ctx.lr = 0x824335CC;
	sub_824370C8(ctx, base);
	// 824335CC: 48000010  b 0x824335dc
	pc = 0x824335DC; continue 'dispatch;
	// 824335D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824335D4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824335D8: 4BFF43B1  bl 0x82427988
	ctx.lr = 0x824335DC;
	sub_82427988(ctx, base);
	// 824335DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824335E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824335E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824335E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824335EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824335F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824335F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824335F8 size=164
    let mut pc: u32 = 0x824335F8;
    'dispatch: loop {
        match pc {
            0x824335F8 => {
    //   block [0x824335F8..0x8243369C)
	// 824335F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824335FC: 48101ABD  bl 0x825350b8
	ctx.lr = 0x82433600;
	sub_82535080(ctx, base);
	// 82433600: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433608: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243360C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82433610: 4BFFFA51  bl 0x82433060
	ctx.lr = 0x82433614;
	sub_82433060(ctx, base);
	// 82433614: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433618: 419A0018  beq cr6, 0x82433630
	if ctx.cr[6].eq {
	pc = 0x82433630; continue 'dispatch;
	}
	// 8243361C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433620: 386B4D4C  addi r3, r11, 0x4d4c
	ctx.r[3].s64 = ctx.r[11].s64 + 19788;
	// 82433624: 48003AA5  bl 0x824370c8
	ctx.lr = 0x82433628;
	sub_824370C8(ctx, base);
	// 82433628: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243362C: 48101ADC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82433630: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82433634: 839F0054  lwz r28, 0x54(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433638: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8243363C: 80BF0438  lwz r5, 0x438(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82433640: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82433644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82433648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243364C: 4BFE485D  bl 0x82417ea8
	ctx.lr = 0x82433650;
	sub_82417EA8(ctx, base);
	// 82433650: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82433654: 409A002C  bne cr6, 0x82433680
	if !ctx.cr[6].eq {
	pc = 0x82433680; continue 'dispatch;
	}
	// 82433658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243365C: 4BFE411D  bl 0x82417778
	ctx.lr = 0x82433660;
	sub_82417778(ctx, base);
	// 82433660: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82433664: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82433668: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243366C: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433670: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82433674: 4BFF4065  bl 0x824276d8
	ctx.lr = 0x82433678;
	sub_824276D8(ctx, base);
	// 82433678: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243367C: 48101A8C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82433680: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433684: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82433688: 386B4D18  addi r3, r11, 0x4d18
	ctx.r[3].s64 = ctx.r[11].s64 + 19736;
	// 8243368C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433690: 48003A39  bl 0x824370c8
	ctx.lr = 0x82433694;
	sub_824370C8(ctx, base);
	// 82433694: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82433698: 48101A70  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824336A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824336A0 size=92
    let mut pc: u32 = 0x824336A0;
    'dispatch: loop {
        match pc {
            0x824336A0 => {
    //   block [0x824336A0..0x824336FC)
	// 824336A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824336A4: 48101A15  bl 0x825350b8
	ctx.lr = 0x824336A8;
	sub_82535080(ctx, base);
	// 824336A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824336AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824336B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824336B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824336B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824336BC: 4BFFF9A5  bl 0x82433060
	ctx.lr = 0x824336C0;
	sub_82433060(ctx, base);
	// 824336C0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824336C4: 419A0018  beq cr6, 0x824336dc
	if ctx.cr[6].eq {
	pc = 0x824336DC; continue 'dispatch;
	}
	// 824336C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824336CC: 386B4D78  addi r3, r11, 0x4d78
	ctx.r[3].s64 = ctx.r[11].s64 + 19832;
	// 824336D0: 480039F9  bl 0x824370c8
	ctx.lr = 0x824336D4;
	sub_824370C8(ctx, base);
	// 824336D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824336D8: 48101A30  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824336DC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 824336E0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824336E4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824336E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824336EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824336F0: 4BFF3FE9  bl 0x824276d8
	ctx.lr = 0x824336F4;
	sub_824276D8(ctx, base);
	// 824336F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824336F8: 48101A10  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433700 size=44
    let mut pc: u32 = 0x82433700;
    'dispatch: loop {
        match pc {
            0x82433700 => {
    //   block [0x82433700..0x8243372C)
	// 82433700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243370C: 4BFF414D  bl 0x82427858
	ctx.lr = 0x82433710;
	sub_82427858(ctx, base);
	// 82433710: 3963FFFD  addi r11, r3, -3
	ctx.r[11].s64 = ctx.r[3].s64 + -3;
	// 82433714: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82433718: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243371C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433730 size=12
    let mut pc: u32 = 0x82433730;
    'dispatch: loop {
        match pc {
            0x82433730 => {
    //   block [0x82433730..0x8243373C)
	// 82433730: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433734: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433738: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243373C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243373C size=8
    let mut pc: u32 = 0x8243373C;
    'dispatch: loop {
        match pc {
            0x8243373C => {
    //   block [0x8243373C..0x82433744)
	// 8243373C: 4BFF41CC  b 0x82427908
	sub_82427908(ctx, base);
	return;
	// 82433740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433748 size=12
    let mut pc: u32 = 0x82433748;
    'dispatch: loop {
        match pc {
            0x82433748 => {
    //   block [0x82433748..0x82433754)
	// 82433748: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243374C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433750: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433754 size=8
    let mut pc: u32 = 0x82433754;
    'dispatch: loop {
        match pc {
            0x82433754 => {
    //   block [0x82433754..0x8243375C)
	// 82433754: 4BFF4034  b 0x82427788
	sub_82427788(ctx, base);
	return;
	// 82433758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433760 size=184
    let mut pc: u32 = 0x82433760;
    'dispatch: loop {
        match pc {
            0x82433760 => {
    //   block [0x82433760..0x82433818)
	// 82433760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433768: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243376C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433778: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243377C: 4BFFF8E5  bl 0x82433060
	ctx.lr = 0x82433780;
	sub_82433060(ctx, base);
	// 82433780: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433784: 419A0014  beq cr6, 0x82433798
	if ctx.cr[6].eq {
	pc = 0x82433798; continue 'dispatch;
	}
	// 82433788: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243378C: 386B4DD8  addi r3, r11, 0x4dd8
	ctx.r[3].s64 = ctx.r[11].s64 + 19928;
	// 82433790: 48003939  bl 0x824370c8
	ctx.lr = 0x82433794;
	sub_824370C8(ctx, base);
	// 82433794: 4800006C  b 0x82433800
	pc = 0x82433800; continue 'dispatch;
	// 82433798: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8243379C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824337A0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824337A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824337A8: 409A0014  bne cr6, 0x824337bc
	if !ctx.cr[6].eq {
	pc = 0x824337BC; continue 'dispatch;
	}
	// 824337AC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824337B0: 409A000C  bne cr6, 0x824337bc
	if !ctx.cr[6].eq {
	pc = 0x824337BC; continue 'dispatch;
	}
	// 824337B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824337B8: 995F0081  stb r10, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[10].u8 ) };
	// 824337BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824337C0: 409A003C  bne cr6, 0x824337fc
	if !ctx.cr[6].eq {
	pc = 0x824337FC; continue 'dispatch;
	}
	// 824337C4: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 824337C8: 409A0034  bne cr6, 0x824337fc
	if !ctx.cr[6].eq {
	pc = 0x824337FC; continue 'dispatch;
	}
	// 824337CC: 48010A95  bl 0x82444260
	ctx.lr = 0x824337D0;
	sub_82444260(ctx, base);
	// 824337D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824337D4: 419A0010  beq cr6, 0x824337e4
	if ctx.cr[6].eq {
	pc = 0x824337E4; continue 'dispatch;
	}
	// 824337D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824337DC: 386B4DAC  addi r3, r11, 0x4dac
	ctx.r[3].s64 = ctx.r[11].s64 + 19884;
	// 824337E0: 480038E9  bl 0x824370c8
	ctx.lr = 0x824337E4;
	sub_824370C8(ctx, base);
	// 824337E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824337E8: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 824337EC: 48007F85  bl 0x8243b770
	ctx.lr = 0x824337F0;
	sub_8243B770(ctx, base);
	// 824337F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824337F4: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 824337F8: 48007F79  bl 0x8243b770
	ctx.lr = 0x824337FC;
	sub_8243B770(ctx, base);
	// 824337FC: 9BDF0080  stb r30, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82433800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243380C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433818 size=8
    let mut pc: u32 = 0x82433818;
    'dispatch: loop {
        match pc {
            0x82433818 => {
    //   block [0x82433818..0x82433820)
	// 82433818: 90830570  stw r4, 0x570(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1392 as u32), ctx.r[4].u32 ) };
	// 8243381C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82433820 size=164
    let mut pc: u32 = 0x82433820;
    'dispatch: loop {
        match pc {
            0x82433820 => {
    //   block [0x82433820..0x824338C4)
	// 82433820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243382C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433830: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82433834: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 82433838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243383C: 4BFFF7BD  bl 0x82432ff8
	ctx.lr = 0x82433840;
	sub_82432FF8(ctx, base);
	// 82433840: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82433844: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433848: 4800B6B9  bl 0x8243ef00
	ctx.lr = 0x8243384C;
	sub_8243EF00(ctx, base);
	// 8243384C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433850: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82433854: 41990030  bgt cr6, 0x82433884
	if ctx.cr[6].gt {
	pc = 0x82433884; continue 'dispatch;
	}
	// 82433858: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243385C: 386B4E04  addi r3, r11, 0x4e04
	ctx.r[3].s64 = ctx.r[11].s64 + 19972;
	// 82433860: 48003869  bl 0x824370c8
	ctx.lr = 0x82433864;
	sub_824370C8(ctx, base);
	// 82433864: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433868: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8243386C: 917F0594  stw r11, 0x594(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 82433870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243387C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433880: 4E800020  blr
	return;
	// 82433884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433888: 48001139  bl 0x824349c0
	ctx.lr = 0x8243388C;
	sub_824349C0(ctx, base);
	// 8243388C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433890: 1D6303E8  mulli r11, r3, 0x3e8
	ctx.r[11].s64 = ctx.r[3].s64 * 1000;
	// 82433894: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82433898: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243389C: 1D6B0BB8  mulli r11, r11, 0xbb8
	ctx.r[11].s64 = ctx.r[11].s64 * 3000;
	// 824338A0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 824338A4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824338A8: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824338AC: 907F0594  stw r3, 0x594(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1428 as u32), ctx.r[3].u32 ) };
	// 824338B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824338B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824338B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824338BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824338C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824338C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824338C8 size=168
    let mut pc: u32 = 0x824338C8;
    'dispatch: loop {
        match pc {
            0x824338C8 => {
    //   block [0x824338C8..0x82433970)
	// 824338C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824338CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824338D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824338D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824338D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824338DC: 4BFFF785  bl 0x82433060
	ctx.lr = 0x824338E0;
	sub_82433060(ctx, base);
	// 824338E0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824338E4: 419A0024  beq cr6, 0x82433908
	if ctx.cr[6].eq {
	pc = 0x82433908; continue 'dispatch;
	}
	// 824338E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824338EC: 386B4E30  addi r3, r11, 0x4e30
	ctx.r[3].s64 = ctx.r[11].s64 + 20016;
	// 824338F0: 480037D9  bl 0x824370c8
	ctx.lr = 0x824338F4;
	sub_824370C8(ctx, base);
	// 824338F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824338F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824338FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433904: 4E800020  blr
	return;
	// 82433908: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243390C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433910: 4BFFFE51  bl 0x82433760
	ctx.lr = 0x82433914;
	sub_82433760(ctx, base);
	// 82433914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433918: 809F0454  lwz r4, 0x454(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 8243391C: 4BFFF5F5  bl 0x82432f10
	ctx.lr = 0x82433920;
	sub_82432F10(ctx, base);
	// 82433920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433924: 4BFFF13D  bl 0x82432a60
	ctx.lr = 0x82433928;
	sub_82432A60(ctx, base);
	// 82433928: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243392C: 4BFF4205  bl 0x82427b30
	ctx.lr = 0x82433930;
	sub_82427B30(ctx, base);
	// 82433930: 807F0450  lwz r3, 0x450(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1104 as u32) ) } as u64;
	// 82433934: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433938: 419A0014  beq cr6, 0x8243394c
	if ctx.cr[6].eq {
	pc = 0x8243394C; continue 'dispatch;
	}
	// 8243393C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82433940: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82433944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82433948: 4E800421  bctrl
	ctx.lr = 0x8243394C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243394C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433950: 480016E1  bl 0x82435030
	ctx.lr = 0x82433954;
	sub_82435030(ctx, base);
	// 82433954: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82433958: 917F056C  stw r11, 0x56c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1388 as u32), ctx.r[11].u32 ) };
	// 8243395C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243396C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433970 size=184
    let mut pc: u32 = 0x82433970;
    'dispatch: loop {
        match pc {
            0x82433970 => {
    //   block [0x82433970..0x82433A28)
	// 82433970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243397C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433988: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243398C: 4BFFF6D5  bl 0x82433060
	ctx.lr = 0x82433990;
	sub_82433060(ctx, base);
	// 82433990: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433994: 419A0014  beq cr6, 0x824339a8
	if ctx.cr[6].eq {
	pc = 0x824339A8; continue 'dispatch;
	}
	// 82433998: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243399C: 386B4E8C  addi r3, r11, 0x4e8c
	ctx.r[3].s64 = ctx.r[11].s64 + 20108;
	// 824339A0: 48003729  bl 0x824370c8
	ctx.lr = 0x824339A4;
	sub_824370C8(ctx, base);
	// 824339A4: 4800006C  b 0x82433a10
	pc = 0x82433A10; continue 'dispatch;
	// 824339A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824339AC: 409A0014  bne cr6, 0x824339c0
	if !ctx.cr[6].eq {
	pc = 0x824339C0; continue 'dispatch;
	}
	// 824339B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824339B4: 386B4E60  addi r3, r11, 0x4e60
	ctx.r[3].s64 = ctx.r[11].s64 + 20064;
	// 824339B8: 48003711  bl 0x824370c8
	ctx.lr = 0x824339BC;
	sub_824370C8(ctx, base);
	// 824339BC: 48000054  b 0x82433a10
	pc = 0x82433A10; continue 'dispatch;
	// 824339C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824339C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824339C8: 4BFFF131  bl 0x82432af8
	ctx.lr = 0x824339CC;
	sub_82432AF8(ctx, base);
	// 824339CC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824339D0: 4BFF4071  bl 0x82427a40
	ctx.lr = 0x824339D4;
	sub_82427A40(ctx, base);
	// 824339D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824339D8: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 824339DC: 4BFFFB15  bl 0x824334f0
	ctx.lr = 0x824339E0;
	sub_824334F0(ctx, base);
	// 824339E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824339E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824339E8: 4BFFFBB1  bl 0x82433598
	ctx.lr = 0x824339EC;
	sub_82433598(ctx, base);
	// 824339EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824339F0: 4BFFFED9  bl 0x824338c8
	ctx.lr = 0x824339F4;
	sub_824338C8(ctx, base);
	// 824339F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824339F8: 815F0578  lwz r10, 0x578(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1400 as u32) ) } as u64;
	// 824339FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82433A00: 917F057C  stw r11, 0x57c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1404 as u32), ctx.r[11].u32 ) };
	// 82433A04: 917F0574  stw r11, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[11].u32 ) };
	// 82433A08: 409A0008  bne cr6, 0x82433a10
	if !ctx.cr[6].eq {
	pc = 0x82433A10; continue 'dispatch;
	}
	// 82433A0C: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	// 82433A10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433A1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433A28 size=96
    let mut pc: u32 = 0x82433A28;
    'dispatch: loop {
        match pc {
            0x82433A28 => {
    //   block [0x82433A28..0x82433A88)
	// 82433A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433A30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433A34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433A38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433A3C: 4BFFF625  bl 0x82433060
	ctx.lr = 0x82433A40;
	sub_82433060(ctx, base);
	// 82433A40: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433A44: 419A0024  beq cr6, 0x82433a68
	if ctx.cr[6].eq {
	pc = 0x82433A68; continue 'dispatch;
	}
	// 82433A48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433A4C: 386B4EBC  addi r3, r11, 0x4ebc
	ctx.r[3].s64 = ctx.r[11].s64 + 20156;
	// 82433A50: 48003679  bl 0x824370c8
	ctx.lr = 0x82433A54;
	sub_824370C8(ctx, base);
	// 82433A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433A60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433A64: 4E800020  blr
	return;
	// 82433A68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82433A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433A70: 4BFFFCF1  bl 0x82433760
	ctx.lr = 0x82433A74;
	sub_82433760(ctx, base);
	// 82433A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433A80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433A88 size=148
    let mut pc: u32 = 0x82433A88;
    'dispatch: loop {
        match pc {
            0x82433A88 => {
    //   block [0x82433A88..0x82433B1C)
	// 82433A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433A8C: 48101631  bl 0x825350bc
	ctx.lr = 0x82433A90;
	sub_82535080(ctx, base);
	// 82433A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433A98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82433A9C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82433AA0: 4BFFF5C1  bl 0x82433060
	ctx.lr = 0x82433AA4;
	sub_82433060(ctx, base);
	// 82433AA4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433AA8: 419A0018  beq cr6, 0x82433ac0
	if ctx.cr[6].eq {
	pc = 0x82433AC0; continue 'dispatch;
	}
	// 82433AAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433AB0: 386B4EF0  addi r3, r11, 0x4ef0
	ctx.r[3].s64 = ctx.r[11].s64 + 20208;
	// 82433AB4: 48003615  bl 0x824370c8
	ctx.lr = 0x82433AB8;
	sub_824370C8(ctx, base);
	// 82433AB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433ABC: 48101650  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82433AC0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433AC4: 4BFF3F7D  bl 0x82427a40
	ctx.lr = 0x82433AC8;
	sub_82427A40(ctx, base);
	// 82433AC8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82433ACC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433AD4: 4BFFFB25  bl 0x824335f8
	ctx.lr = 0x82433AD8;
	sub_824335F8(ctx, base);
	// 82433AD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82433ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433AE0: 4BFFFAB9  bl 0x82433598
	ctx.lr = 0x82433AE4;
	sub_82433598(ctx, base);
	// 82433AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433AE8: 4BFFFDE1  bl 0x824338c8
	ctx.lr = 0x82433AEC;
	sub_824338C8(ctx, base);
	// 82433AEC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82433AF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82433AF4: 813F0578  lwz r9, 0x578(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1400 as u32) ) } as u64;
	// 82433AF8: 93DF0584  stw r30, 0x584(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1412 as u32), ctx.r[30].u32 ) };
	// 82433AFC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82433B00: 93BF0588  stw r29, 0x588(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1416 as u32), ctx.r[29].u32 ) };
	// 82433B04: 915F057C  stw r10, 0x57c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1404 as u32), ctx.r[10].u32 ) };
	// 82433B08: 917F0574  stw r11, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[11].u32 ) };
	// 82433B0C: 409A0008  bne cr6, 0x82433b14
	if !ctx.cr[6].eq {
	pc = 0x82433B14; continue 'dispatch;
	}
	// 82433B10: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	// 82433B14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433B18: 481015F4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433B20 size=168
    let mut pc: u32 = 0x82433B20;
    'dispatch: loop {
        match pc {
            0x82433B20 => {
    //   block [0x82433B20..0x82433BC8)
	// 82433B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433B24: 48101595  bl 0x825350b8
	ctx.lr = 0x82433B28;
	sub_82535080(ctx, base);
	// 82433B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433B30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82433B34: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82433B38: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82433B3C: 4BFFF525  bl 0x82433060
	ctx.lr = 0x82433B40;
	sub_82433060(ctx, base);
	// 82433B40: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433B44: 419A0018  beq cr6, 0x82433b5c
	if ctx.cr[6].eq {
	pc = 0x82433B5C; continue 'dispatch;
	}
	// 82433B48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433B4C: 386B4F20  addi r3, r11, 0x4f20
	ctx.r[3].s64 = ctx.r[11].s64 + 20256;
	// 82433B50: 48003579  bl 0x824370c8
	ctx.lr = 0x82433B54;
	sub_824370C8(ctx, base);
	// 82433B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433B58: 481015B0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82433B5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433B64: 4BFFEF95  bl 0x82432af8
	ctx.lr = 0x82433B68;
	sub_82432AF8(ctx, base);
	// 82433B68: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433B6C: 4BFF3ED5  bl 0x82427a40
	ctx.lr = 0x82433B70;
	sub_82427A40(ctx, base);
	// 82433B70: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82433B74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82433B78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433B80: 4BFFFB21  bl 0x824336a0
	ctx.lr = 0x82433B84;
	sub_824336A0(ctx, base);
	// 82433B84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82433B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433B8C: 4BFFFA0D  bl 0x82433598
	ctx.lr = 0x82433B90;
	sub_82433598(ctx, base);
	// 82433B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433B94: 4BFFFD35  bl 0x824338c8
	ctx.lr = 0x82433B98;
	sub_824338C8(ctx, base);
	// 82433B98: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82433B9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82433BA0: 813F0578  lwz r9, 0x578(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1400 as u32) ) } as u64;
	// 82433BA4: 93BF058C  stw r29, 0x58c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1420 as u32), ctx.r[29].u32 ) };
	// 82433BA8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82433BAC: 939F0590  stw r28, 0x590(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1424 as u32), ctx.r[28].u32 ) };
	// 82433BB0: 915F057C  stw r10, 0x57c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1404 as u32), ctx.r[10].u32 ) };
	// 82433BB4: 917F0574  stw r11, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[11].u32 ) };
	// 82433BB8: 409A0008  bne cr6, 0x82433bc0
	if !ctx.cr[6].eq {
	pc = 0x82433BC0; continue 'dispatch;
	}
	// 82433BBC: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	// 82433BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433BC4: 48101544  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433BC8 size=164
    let mut pc: u32 = 0x82433BC8;
    'dispatch: loop {
        match pc {
            0x82433BC8 => {
    //   block [0x82433BC8..0x82433C6C)
	// 82433BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433BD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433BD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433BD8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82433BDC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82433BE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433BE4: 4BFFF705  bl 0x824332e8
	ctx.lr = 0x82433BE8;
	sub_824332E8(ctx, base);
	// 82433BE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82433BEC: 817F057C  lwz r11, 0x57c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1404 as u32) ) } as u64;
	// 82433BF0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82433BF4: 915F0578  stw r10, 0x578(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1400 as u32), ctx.r[10].u32 ) };
	// 82433BF8: 419A0040  beq cr6, 0x82433c38
	if ctx.cr[6].eq {
	pc = 0x82433C38; continue 'dispatch;
	}
	// 82433BFC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82433C00: 419A0024  beq cr6, 0x82433c24
	if ctx.cr[6].eq {
	pc = 0x82433C24; continue 'dispatch;
	}
	// 82433C04: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82433C08: 409A003C  bne cr6, 0x82433c44
	if !ctx.cr[6].eq {
	pc = 0x82433C44; continue 'dispatch;
	}
	// 82433C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433C10: 80DF0590  lwz r6, 0x590(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1424 as u32) ) } as u64;
	// 82433C14: 80BF058C  lwz r5, 0x58c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1420 as u32) ) } as u64;
	// 82433C18: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82433C1C: 4BFFFF05  bl 0x82433b20
	ctx.lr = 0x82433C20;
	sub_82433B20(ctx, base);
	// 82433C20: 48000024  b 0x82433c44
	pc = 0x82433C44; continue 'dispatch;
	// 82433C24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433C28: 80BF0588  lwz r5, 0x588(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82433C2C: 809F0584  lwz r4, 0x584(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1412 as u32) ) } as u64;
	// 82433C30: 4BFFFE59  bl 0x82433a88
	ctx.lr = 0x82433C34;
	sub_82433A88(ctx, base);
	// 82433C34: 48000010  b 0x82433c44
	pc = 0x82433C44; continue 'dispatch;
	// 82433C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433C3C: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82433C40: 4BFFFD31  bl 0x82433970
	ctx.lr = 0x82433C44;
	sub_82433970(ctx, base);
	// 82433C44: 817F0598  lwz r11, 0x598(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1432 as u32) ) } as u64;
	// 82433C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82433C4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82433C50: 915F0578  stw r10, 0x578(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1400 as u32), ctx.r[10].u32 ) };
	// 82433C54: 917F0598  stw r11, 0x598(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1432 as u32), ctx.r[11].u32 ) };
	// 82433C58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433C64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433C70 size=112
    let mut pc: u32 = 0x82433C70;
    'dispatch: loop {
        match pc {
            0x82433C70 => {
    //   block [0x82433C70..0x82433CE0)
	// 82433C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433C78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433C7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433C80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433C84: 4BFFF3DD  bl 0x82433060
	ctx.lr = 0x82433C88;
	sub_82433060(ctx, base);
	// 82433C88: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433C8C: 419A0024  beq cr6, 0x82433cb0
	if ctx.cr[6].eq {
	pc = 0x82433CB0; continue 'dispatch;
	}
	// 82433C90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433C94: 386B4F54  addi r3, r11, 0x4f54
	ctx.r[3].s64 = ctx.r[11].s64 + 20308;
	// 82433C98: 48003431  bl 0x824370c8
	ctx.lr = 0x82433C9C;
	sub_824370C8(ctx, base);
	// 82433C9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433CAC: 4E800020  blr
	return;
	// 82433CB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82433CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433CB8: 4BFFF8E1  bl 0x82433598
	ctx.lr = 0x82433CBC;
	sub_82433598(ctx, base);
	// 82433CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433CC0: 4BFFFD69  bl 0x82433a28
	ctx.lr = 0x82433CC4;
	sub_82433A28(ctx, base);
	// 82433CC4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82433CC8: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	// 82433CCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433CE0 size=128
    let mut pc: u32 = 0x82433CE0;
    'dispatch: loop {
        match pc {
            0x82433CE0 => {
    //   block [0x82433CE0..0x82433D60)
	// 82433CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433CEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433CF0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82433CF4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82433CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433CFC: 4BFFF5ED  bl 0x824332e8
	ctx.lr = 0x82433D00;
	sub_824332E8(ctx, base);
	// 82433D00: 3D401062  lis r10, 0x1062
	ctx.r[10].s64 = 274857984;
	// 82433D04: 817F0594  lwz r11, 0x594(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1428 as u32) ) } as u64;
	// 82433D08: 614A4DD3  ori r10, r10, 0x4dd3
	ctx.r[10].u64 = ctx.r[10].u64 | 19923;
	// 82433D0C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433D10: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433D14: 7D6B5096  mulhw r11, r11, r10
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 82433D18: 7D6B3670  srawi r11, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 82433D1C: 7D284BD6  divw r9, r8, r9
	ctx.r[9].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 82433D20: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82433D24: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82433D28: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82433D2C: 41990020  bgt cr6, 0x82433d4c
	if ctx.cr[6].gt {
	pc = 0x82433D4C; continue 'dispatch;
	}
	// 82433D30: 817F0580  lwz r11, 0x580(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82433D34: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82433D38: 409A0014  bne cr6, 0x82433d4c
	if !ctx.cr[6].eq {
	pc = 0x82433D4C; continue 'dispatch;
	}
	// 82433D3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433D40: 4BFFFF31  bl 0x82433c70
	ctx.lr = 0x82433D44;
	sub_82433C70(ctx, base);
	// 82433D44: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82433D48: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	// 82433D4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433D58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433D60 size=164
    let mut pc: u32 = 0x82433D60;
    'dispatch: loop {
        match pc {
            0x82433D60 => {
    //   block [0x82433D60..0x82433E04)
	// 82433D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433D70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433D74: 817F0570  lwz r11, 0x570(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1392 as u32) ) } as u64;
	// 82433D78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82433D7C: 419A0074  beq cr6, 0x82433df0
	if ctx.cr[6].eq {
	pc = 0x82433DF0; continue 'dispatch;
	}
	// 82433D80: 817F0574  lwz r11, 0x574(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1396 as u32) ) } as u64;
	// 82433D84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82433D88: 419A0068  beq cr6, 0x82433df0
	if ctx.cr[6].eq {
	pc = 0x82433DF0; continue 'dispatch;
	}
	// 82433D8C: 817F0580  lwz r11, 0x580(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82433D90: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82433D94: 409A0028  bne cr6, 0x82433dbc
	if !ctx.cr[6].eq {
	pc = 0x82433DBC; continue 'dispatch;
	}
	// 82433D98: 4BFFF6E9  bl 0x82433480
	ctx.lr = 0x82433D9C;
	sub_82433480(ctx, base);
	// 82433D9C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82433DA0: 409A001C  bne cr6, 0x82433dbc
	if !ctx.cr[6].eq {
	pc = 0x82433DBC; continue 'dispatch;
	}
	// 82433DA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433DA8: 4BFFFA79  bl 0x82433820
	ctx.lr = 0x82433DAC;
	sub_82433820(ctx, base);
	// 82433DAC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82433DB0: 41980040  blt cr6, 0x82433df0
	if ctx.cr[6].lt {
	pc = 0x82433DF0; continue 'dispatch;
	}
	// 82433DB4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82433DB8: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	// 82433DBC: 817F0580  lwz r11, 0x580(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82433DC0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82433DC4: 419A000C  beq cr6, 0x82433dd0
	if ctx.cr[6].eq {
	pc = 0x82433DD0; continue 'dispatch;
	}
	// 82433DC8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82433DCC: 409A0024  bne cr6, 0x82433df0
	if !ctx.cr[6].eq {
	pc = 0x82433DF0; continue 'dispatch;
	}
	// 82433DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433DD4: 4BFFF2A5  bl 0x82433078
	ctx.lr = 0x82433DD8;
	sub_82433078(ctx, base);
	// 82433DD8: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82433DDC: 409A000C  bne cr6, 0x82433de8
	if !ctx.cr[6].eq {
	pc = 0x82433DE8; continue 'dispatch;
	}
	// 82433DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433DE4: 4BFFFDE5  bl 0x82433bc8
	ctx.lr = 0x82433DE8;
	sub_82433BC8(ctx, base);
	// 82433DE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433DEC: 4BFFFEF5  bl 0x82433ce0
	ctx.lr = 0x82433DF0;
	sub_82433CE0(ctx, base);
	// 82433DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433DFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433E08 size=112
    let mut pc: u32 = 0x82433E08;
    'dispatch: loop {
        match pc {
            0x82433E08 => {
    //   block [0x82433E08..0x82433E78)
	// 82433E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433E10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82433E14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433E18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433E1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82433E20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82433E24: 4BFFE45D  bl 0x82432280
	ctx.lr = 0x82433E28;
	sub_82432280(ctx, base);
	// 82433E28: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433E2C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82433E30: 419A0008  beq cr6, 0x82433e38
	if ctx.cr[6].eq {
	pc = 0x82433E38; continue 'dispatch;
	}
	// 82433E34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82433E38: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82433E3C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82433E40: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82433E44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82433E48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82433E4C: 4BFFE7FD  bl 0x82432648
	ctx.lr = 0x82433E50;
	sub_82432648(ctx, base);
	// 82433E50: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433E54: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433E58: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82433E5C: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82433E60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433E6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433E70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433E78 size=20
    let mut pc: u32 = 0x82433E78;
    'dispatch: loop {
        match pc {
            0x82433E78 => {
    //   block [0x82433E78..0x82433E8C)
	// 82433E78: 8163056C  lwz r11, 0x56c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1388 as u32) ) } as u64;
	// 82433E7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82433E80: 409A000C  bne cr6, 0x82433e8c
	if !ctx.cr[6].eq {
		sub_82433E8C(ctx, base);
		return;
	}
	// 82433E84: 9083056C  stw r4, 0x56c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1388 as u32), ctx.r[4].u32 ) };
	// 82433E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433E8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433E8C size=8
    let mut pc: u32 = 0x82433E8C;
    'dispatch: loop {
        match pc {
            0x82433E8C => {
    //   block [0x82433E8C..0x82433E94)
	// 82433E8C: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82433E90: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433E94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433E94 size=12
    let mut pc: u32 = 0x82433E94;
    'dispatch: loop {
        match pc {
            0x82433E94 => {
    //   block [0x82433E94..0x82433EA0)
	// 82433E94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433E98: 386B4F80  addi r3, r11, 0x4f80
	ctx.r[3].s64 = ctx.r[11].s64 + 20352;
	// 82433E9C: 4800322C  b 0x824370c8
	sub_824370C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433EA0 size=80
    let mut pc: u32 = 0x82433EA0;
    'dispatch: loop {
        match pc {
            0x82433EA0 => {
    //   block [0x82433EA0..0x82433EF0)
	// 82433EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433EA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82433EAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433EB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82433EBC: 817F0424  lwz r11, 0x424(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1060 as u32) ) } as u64;
	// 82433EC0: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82433EC4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82433EC8: 40980008  bge cr6, 0x82433ed0
	if !ctx.cr[6].lt {
	pc = 0x82433ED0; continue 'dispatch;
	}
	// 82433ECC: 4BFFE73D  bl 0x82432608
	ctx.lr = 0x82433ED0;
	sub_82432608(ctx, base);
	// 82433ED0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82433ED4: 917F0424  stw r11, 0x424(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1060 as u32), ctx.r[11].u32 ) };
	// 82433ED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433EE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433EE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433EF0 size=120
    let mut pc: u32 = 0x82433EF0;
    'dispatch: loop {
        match pc {
            0x82433EF0 => {
    //   block [0x82433EF0..0x82433F68)
	// 82433EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433EF4: 481011C1  bl 0x825350b4
	ctx.lr = 0x82433EF8;
	sub_82535080(ctx, base);
	// 82433EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433EFC: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82433F00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433F04: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82433F08: 837F040C  lwz r27, 0x40c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1036 as u32) ) } as u64;
	// 82433F0C: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82433F10: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82433F14: 4800299D  bl 0x824368b0
	ctx.lr = 0x82433F18;
	sub_824368B0(ctx, base);
	// 82433F18: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433F1C: 419A0018  beq cr6, 0x82433f34
	if ctx.cr[6].eq {
	pc = 0x82433F34; continue 'dispatch;
	}
	// 82433F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82433F24: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82433F28: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82433F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433F30: 481011D4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82433F34: 397F03F4  addi r11, r31, 0x3f4
	ctx.r[11].s64 = ctx.r[31].s64 + 1012;
	// 82433F38: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82433F3C: 419AFFE4  beq cr6, 0x82433f20
	if ctx.cr[6].eq {
	pc = 0x82433F20; continue 'dispatch;
	}
	// 82433F40: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82433F44: 419A0014  beq cr6, 0x82433f58
	if ctx.cr[6].eq {
	pc = 0x82433F58; continue 'dispatch;
	}
	// 82433F48: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82433F4C: 4099000C  ble cr6, 0x82433f58
	if !ctx.cr[6].gt {
	pc = 0x82433F58; continue 'dispatch;
	}
	// 82433F50: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82433F54: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 82433F58: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 82433F5C: 939E0048  stw r28, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[28].u32 ) };
	// 82433F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433F64: 481011A0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433F68 size=8
    let mut pc: u32 = 0x82433F68;
    'dispatch: loop {
        match pc {
            0x82433F68 => {
    //   block [0x82433F68..0x82433F70)
	// 82433F68: 1CA503E8  mulli r5, r5, 0x3e8
	ctx.r[5].s64 = ctx.r[5].s64 * 1000;
	// 82433F6C: 4801060C  b 0x82444578
	sub_82444578(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433F70 size=24
    let mut pc: u32 = 0x82433F70;
    'dispatch: loop {
        match pc {
            0x82433F70 => {
    //   block [0x82433F70..0x82433F88)
	// 82433F70: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433F74: 419A001C  beq cr6, 0x82433f90
	if ctx.cr[6].eq {
		sub_82433F90(ctx, base);
		return;
	}
	// 82433F78: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82433F7C: 419A000C  beq cr6, 0x82433f88
	if ctx.cr[6].eq {
		sub_82433F88(ctx, base);
		return;
	}
	// 82433F80: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82433F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433F88 size=8
    let mut pc: u32 = 0x82433F88;
    'dispatch: loop {
        match pc {
            0x82433F88 => {
    //   block [0x82433F88..0x82433F90)
	// 82433F88: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82433F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433F90 size=8
    let mut pc: u32 = 0x82433F90;
    'dispatch: loop {
        match pc {
            0x82433F90 => {
    //   block [0x82433F90..0x82433F98)
	// 82433F90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82433F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433F98 size=156
    let mut pc: u32 = 0x82433F98;
    'dispatch: loop {
        match pc {
            0x82433F98 => {
    //   block [0x82433F98..0x82433FD8)
	// 82433F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433FA4: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82433FA8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82433FAC: 41990068  bgt cr6, 0x82434014
	if ctx.cr[6].gt {
	pc = 0x82434014; continue 'dispatch;
	}
	// 82433FB0: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82433FB4: 398C3FC8  addi r12, r12, 0x3fc8
	ctx.r[12].s64 = ctx.r[12].s64 + 16328;
	// 82433FB8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82433FBC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82433FC0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82433FC4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82434020; continue 'dispatch;
		},
		1 => {
	pc = 0x82433FD8; continue 'dispatch;
		},
		2 => {
	pc = 0x82433FEC; continue 'dispatch;
		},
		3 => {
	pc = 0x82434000; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82433FC8: 82434020  lwz r18, 0x4020(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16416 as u32) ) } as u64;
	// 82433FCC: 82433FD8  lwz r18, 0x3fd8(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16344 as u32) ) } as u64;
	// 82433FD0: 82433FEC  lwz r18, 0x3fec(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16364 as u32) ) } as u64;
	// 82433FD4: 82434000  lwz r18, 0x4000(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16384 as u32) ) } as u64;
            }
            0x82433FD8 => {
    //   block [0x82433FD8..0x82433FEC)
	// 82433FD8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82433FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433FE8: 4E800020  blr
	return;
            }
            0x82433FEC => {
    //   block [0x82433FEC..0x82434000)
	// 82433FEC: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82433FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433FFC: 4E800020  blr
	return;
            }
            0x82434000 => {
    //   block [0x82434000..0x82434020)
	// 82434000: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82434004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243400C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434010: 4E800020  blr
	return;
	// 82434014: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434018: 386B4FBC  addi r3, r11, 0x4fbc
	ctx.r[3].s64 = ctx.r[11].s64 + 20412;
	// 8243401C: 480030AD  bl 0x824370c8
	ctx.lr = 0x82434020;
	sub_824370C8(ctx, base);
            }
            0x82434020 => {
    //   block [0x82434020..0x82434034)
	// 82434020: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82434024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243402C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434038 size=132
    let mut pc: u32 = 0x82434038;
    'dispatch: loop {
        match pc {
            0x82434038 => {
    //   block [0x82434038..0x824340BC)
	// 82434038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243403C: 48101081  bl 0x825350bc
	ctx.lr = 0x82434040;
	sub_82535080(ctx, base);
	// 82434040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434044: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 82434048: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243404C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82434050: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82434054: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82434058: 40990028  ble cr6, 0x82434080
	if !ctx.cr[6].gt {
	pc = 0x82434080; continue 'dispatch;
	}
	// 8243405C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82434060: 40990018  ble cr6, 0x82434078
	if !ctx.cr[6].gt {
	pc = 0x82434078; continue 'dispatch;
	}
	// 82434064: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82434068: 409A0018  bne cr6, 0x82434080
	if !ctx.cr[6].eq {
	pc = 0x82434080; continue 'dispatch;
	}
	// 8243406C: 8964006C  lbz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 82434070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82434074: 409A0018  bne cr6, 0x8243408c
	if !ctx.cr[6].eq {
	pc = 0x8243408C; continue 'dispatch;
	}
	// 82434078: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8243407C: 48000010  b 0x8243408c
	pc = 0x8243408C; continue 'dispatch;
	// 82434080: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434084: 386B4FEC  addi r3, r11, 0x4fec
	ctx.r[3].s64 = ctx.r[11].s64 + 20460;
	// 82434088: 48003041  bl 0x824370c8
	ctx.lr = 0x8243408C;
	sub_824370C8(ctx, base);
	// 8243408C: 48002825  bl 0x824368b0
	ctx.lr = 0x82434090;
	sub_824368B0(ctx, base);
	// 82434090: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434094: 409A001C  bne cr6, 0x824340b0
	if !ctx.cr[6].eq {
	pc = 0x824340B0; continue 'dispatch;
	}
	// 82434098: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243409C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824340A0: 48004411  bl 0x824384b0
	ctx.lr = 0x824340A4;
	sub_824384B0(ctx, base);
	// 824340A4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824340A8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824340AC: 419A0008  beq cr6, 0x824340b4
	if ctx.cr[6].eq {
	pc = 0x824340B4; continue 'dispatch;
	}
	// 824340B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824340B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824340B8: 48101054  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824340C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824340C0 size=80
    let mut pc: u32 = 0x824340C0;
    'dispatch: loop {
        match pc {
            0x824340C0 => {
    //   block [0x824340C0..0x82434110)
	// 824340C0: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 824340C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824340C8: 91630098  stw r11, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 824340CC: 8164005C  lwz r11, 0x5c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 824340D0: 9163009C  stw r11, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 824340D4: 8964006C  lbz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 824340D8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824340DC: 916300A0  stw r11, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 824340E0: 8964006D  lbz r11, 0x6d(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(109 as u32) ) } as u64;
	// 824340E4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824340E8: 916300A4  stw r11, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 824340EC: 8964006E  lbz r11, 0x6e(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(110 as u32) ) } as u64;
	// 824340F0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824340F4: 916300A8  stw r11, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 824340F8: 8164003C  lwz r11, 0x3c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 824340FC: 916300AC  stw r11, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82434100: 81640040  lwz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 82434104: 914300B4  stw r10, 0xb4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82434108: 916300B0  stw r11, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8243410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434110 size=140
    let mut pc: u32 = 0x82434110;
    'dispatch: loop {
        match pc {
            0x82434110 => {
    //   block [0x82434110..0x8243419C)
	// 82434110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243411C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434124: 81440038  lwz r10, 0x38(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82434128: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8243412C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82434130: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82434134: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82434138: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243413C: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82434140: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82434144: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82434148: 419A0040  beq cr6, 0x82434188
	if ctx.cr[6].eq {
	pc = 0x82434188; continue 'dispatch;
	}
	// 8243414C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82434150: 40990038  ble cr6, 0x82434188
	if !ctx.cr[6].gt {
	pc = 0x82434188; continue 'dispatch;
	}
	// 82434154: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82434158: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243415C: 388AFFFC  addi r4, r10, -4
	ctx.r[4].s64 = ctx.r[10].s64 + -4;
	// 82434160: 38690004  addi r3, r9, 4
	ctx.r[3].s64 = ctx.r[9].s64 + 4;
	// 82434164: 4800445D  bl 0x824385c0
	ctx.lr = 0x82434168;
	sub_824385C0(ctx, base);
	// 82434168: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243416C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82434170: 419A0018  beq cr6, 0x82434188
	if ctx.cr[6].eq {
	pc = 0x82434188; continue 'dispatch;
	}
	// 82434174: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82434178: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243417C: 4099000C  ble cr6, 0x82434188
	if !ctx.cr[6].gt {
	pc = 0x82434188; continue 'dispatch;
	}
	// 82434180: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82434184: 915F009C  stw r10, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 82434188: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243418C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434194: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82434198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824341A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824341A0 size=64
    let mut pc: u32 = 0x824341A0;
    'dispatch: loop {
        match pc {
            0x824341A0 => {
    //   block [0x824341A0..0x824341E0)
	// 824341A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824341A4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824341A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824341AC: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824341B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824341B4: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824341B8: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 824341BC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824341C0: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824341C4: A163000C  lhz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824341C8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824341CC: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824341D0: A163000C  lhz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824341D4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824341D8: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824341DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824341E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824341E0 size=120
    let mut pc: u32 = 0x824341E0;
    'dispatch: loop {
        match pc {
            0x824341E0 => {
    //   block [0x824341E0..0x82434258)
	// 824341E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824341E4: 48100ED5  bl 0x825350b8
	ctx.lr = 0x824341E8;
	sub_82535080(ctx, base);
	// 824341E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824341EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824341F0: 4BFFEE71  bl 0x82433060
	ctx.lr = 0x824341F4;
	sub_82433060(ctx, base);
	// 824341F4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824341F8: 419A0018  beq cr6, 0x82434210
	if ctx.cr[6].eq {
	pc = 0x82434210; continue 'dispatch;
	}
	// 824341FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434200: 386B5020  addi r3, r11, 0x5020
	ctx.r[3].s64 = ctx.r[11].s64 + 20512;
	// 82434204: 48002EC5  bl 0x824370c8
	ctx.lr = 0x82434208;
	sub_824370C8(ctx, base);
	// 82434208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243420C: 48100EFC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82434210: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82434214: 83DF0088  lwz r30, 0x88(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82434218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243421C: 83BF008C  lwz r29, 0x8c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82434220: 839F0090  lwz r28, 0x90(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82434224: 4BFFFC55  bl 0x82433e78
	ctx.lr = 0x82434228;
	sub_82433E78(ctx, base);
	// 82434228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243422C: 4BFFF205  bl 0x82433430
	ctx.lr = 0x82434230;
	sub_82433430(ctx, base);
	// 82434230: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82434234: 4099001C  ble cr6, 0x82434250
	if !ctx.cr[6].gt {
	pc = 0x82434250; continue 'dispatch;
	}
	// 82434238: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243423C: 480064E5  bl 0x8243a720
	ctx.lr = 0x82434240;
	sub_8243A720(ctx, base);
	// 82434240: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82434244: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434248: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8243424C: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82434250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82434254: 48100EB4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434258 size=108
    let mut pc: u32 = 0x82434258;
    'dispatch: loop {
        match pc {
            0x82434258 => {
    //   block [0x82434258..0x824342C4)
	// 82434258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243425C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82434264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243426C: 4BFFEDF5  bl 0x82433060
	ctx.lr = 0x82434270;
	sub_82433060(ctx, base);
	// 82434270: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434274: 419A0028  beq cr6, 0x8243429c
	if ctx.cr[6].eq {
	pc = 0x8243429C; continue 'dispatch;
	}
	// 82434278: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243427C: 386B504C  addi r3, r11, 0x504c
	ctx.r[3].s64 = ctx.r[11].s64 + 20556;
	// 82434280: 48002E49  bl 0x824370c8
	ctx.lr = 0x82434284;
	sub_824370C8(ctx, base);
	// 82434284: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82434288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243428C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82434298: 4E800020  blr
	return;
	// 8243429C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824342A0: 4BFFF191  bl 0x82433430
	ctx.lr = 0x824342A4;
	sub_82433430(ctx, base);
	// 824342A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824342A8: 419AFFDC  beq cr6, 0x82434284
	if ctx.cr[6].eq {
	pc = 0x82434284; continue 'dispatch;
	}
	// 824342AC: 48010FFD  bl 0x824452a8
	ctx.lr = 0x824342B0;
	sub_824452A8(ctx, base);
	// 824342B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824342B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824342B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824342BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824342C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824342C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824342C8 size=52
    let mut pc: u32 = 0x824342C8;
    'dispatch: loop {
        match pc {
            0x824342C8 => {
    //   block [0x824342C8..0x824342FC)
	// 824342C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824342CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824342D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824342D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824342D8: 48011539  bl 0x82445810
	ctx.lr = 0x824342DC;
	sub_82445810(ctx, base);
	// 824342DC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824342E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824342E4: 409A0008  bne cr6, 0x824342ec
	if !ctx.cr[6].eq {
	pc = 0x824342EC; continue 'dispatch;
	}
	// 824342E8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824342EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824342F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824342F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824342F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434300 size=52
    let mut pc: u32 = 0x82434300;
    'dispatch: loop {
        match pc {
            0x82434300 => {
    //   block [0x82434300..0x82434334)
	// 82434300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243430C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434310: 48011529  bl 0x82445838
	ctx.lr = 0x82434314;
	sub_82445838(ctx, base);
	// 82434314: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434318: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243431C: 409A0008  bne cr6, 0x82434324
	if !ctx.cr[6].eq {
	pc = 0x82434324; continue 'dispatch;
	}
	// 82434320: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434324: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243432C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434338 size=72
    let mut pc: u32 = 0x82434338;
    'dispatch: loop {
        match pc {
            0x82434338 => {
    //   block [0x82434338..0x82434380)
	// 82434338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243433C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434344: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434348: 48011451  bl 0x82445798
	ctx.lr = 0x8243434C;
	sub_82445798(ctx, base);
	// 8243434C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434350: 419A0018  beq cr6, 0x82434368
	if ctx.cr[6].eq {
	pc = 0x82434368; continue 'dispatch;
	}
	// 82434354: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82434358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243435C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434364: 4E800020  blr
	return;
	// 82434368: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243436C: 55631838  slwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82434370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243437C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434380 size=140
    let mut pc: u32 = 0x82434380;
    'dispatch: loop {
        match pc {
            0x82434380 => {
    //   block [0x82434380..0x8243440C)
	// 82434380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243438C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82434390: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82434394: 48011725  bl 0x82445ab8
	ctx.lr = 0x82434398;
	sub_82445AB8(ctx, base);
	// 82434398: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243439C: 419A0018  beq cr6, 0x824343b4
	if ctx.cr[6].eq {
	pc = 0x824343B4; continue 'dispatch;
	}
	// 824343A0: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 824343A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824343A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824343AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824343B0: 4E800020  blr
	return;
	// 824343B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824343B8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824343BC: 419A003C  beq cr6, 0x824343f8
	if ctx.cr[6].eq {
	pc = 0x824343F8; continue 'dispatch;
	}
	// 824343C0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824343C4: 419A0020  beq cr6, 0x824343e4
	if ctx.cr[6].eq {
	pc = 0x824343E4; continue 'dispatch;
	}
	// 824343C8: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824343CC: 409AFFD4  bne cr6, 0x824343a0
	if !ctx.cr[6].eq {
	pc = 0x824343A0; continue 'dispatch;
	}
	// 824343D0: 38600061  li r3, 0x61
	ctx.r[3].s64 = 97;
	// 824343D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824343D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824343DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824343E0: 4E800020  blr
	return;
	// 824343E4: 38600051  li r3, 0x51
	ctx.r[3].s64 = 81;
	// 824343E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824343EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824343F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824343F4: 4E800020  blr
	return;
	// 824343F8: 38600021  li r3, 0x21
	ctx.r[3].s64 = 33;
	// 824343FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434410 size=112
    let mut pc: u32 = 0x82434410;
    'dispatch: loop {
        match pc {
            0x82434410 => {
    //   block [0x82434410..0x82434480)
	// 82434410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434414: 394300D4  addi r10, r3, 0xd4
	ctx.r[10].s64 = ctx.r[3].s64 + 212;
	// 82434418: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8243441C: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82434420: 916300C4  stw r11, 0xc4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82434424: 916300C8  stw r11, 0xc8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82434428: 916300CC  stw r11, 0xcc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 8243442C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82434430: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82434434: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434438: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8243443C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82434440: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82434444: 910A0014  stw r8, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82434448: 394A0024  addi r10, r10, 0x24
	ctx.r[10].s64 = ctx.r[10].s64 + 36;
	// 8243444C: 409AFFE0  bne cr6, 0x8243442c
	if !ctx.cr[6].eq {
	pc = 0x8243442C; continue 'dispatch;
	}
	// 82434450: 394301F8  addi r10, r3, 0x1f8
	ctx.r[10].s64 = ctx.r[3].s64 + 504;
	// 82434454: 916301F0  stw r11, 0x1f0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 82434458: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8243445C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82434460: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82434464: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434468: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243446C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82434470: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82434474: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82434478: 409AFFE4  bne cr6, 0x8243445c
	if !ctx.cr[6].eq {
	pc = 0x8243445C; continue 'dispatch;
	}
	// 8243447C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434480 size=108
    let mut pc: u32 = 0x82434480;
    'dispatch: loop {
        match pc {
            0x82434480 => {
    //   block [0x82434480..0x824344C4)
	// 82434480: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82434484: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82434488: 41990064  bgt cr6, 0x824344ec
	if ctx.cr[6].gt {
		sub_824344EC(ctx, base);
		return;
	}
	// 8243448C: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82434490: 398C44A4  addi r12, r12, 0x44a4
	ctx.r[12].s64 = ctx.r[12].s64 + 17572;
	// 82434494: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82434498: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8243449C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 824344A0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x824344C4; continue 'dispatch;
		},
		1 => {
	pc = 0x824344CC; continue 'dispatch;
		},
		2 => {
	pc = 0x824344CC; continue 'dispatch;
		},
		3 => {
	pc = 0x824344CC; continue 'dispatch;
		},
		4 => {
	pc = 0x824344CC; continue 'dispatch;
		},
		5 => {
	pc = 0x824344D4; continue 'dispatch;
		},
		6 => {
	pc = 0x824344DC; continue 'dispatch;
		},
		7 => {
	pc = 0x824344E4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 824344A4: 824344C4  lwz r18, 0x44c4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17604 as u32) ) } as u64;
	// 824344A8: 824344CC  lwz r18, 0x44cc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17612 as u32) ) } as u64;
	// 824344AC: 824344CC  lwz r18, 0x44cc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17612 as u32) ) } as u64;
	// 824344B0: 824344CC  lwz r18, 0x44cc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17612 as u32) ) } as u64;
	// 824344B4: 824344CC  lwz r18, 0x44cc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17612 as u32) ) } as u64;
	// 824344B8: 824344D4  lwz r18, 0x44d4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17620 as u32) ) } as u64;
	// 824344BC: 824344DC  lwz r18, 0x44dc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17628 as u32) ) } as u64;
	// 824344C0: 824344E4  lwz r18, 0x44e4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17636 as u32) ) } as u64;
            }
            0x824344C4 => {
    //   block [0x824344C4..0x824344CC)
	// 824344C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824344C8: 4E800020  blr
	return;
            }
            0x824344CC => {
    //   block [0x824344CC..0x824344D4)
	// 824344CC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824344D0: 4E800020  blr
	return;
            }
            0x824344D4 => {
    //   block [0x824344D4..0x824344DC)
	// 824344D4: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 824344D8: 4E800020  blr
	return;
            }
            0x824344DC => {
    //   block [0x824344DC..0x824344E4)
	// 824344DC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 824344E0: 4E800020  blr
	return;
            }
            0x824344E4 => {
    //   block [0x824344E4..0x824344EC)
	// 824344E4: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 824344E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824344EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824344EC size=8
    let mut pc: u32 = 0x824344EC;
    'dispatch: loop {
        match pc {
            0x824344EC => {
    //   block [0x824344EC..0x824344F4)
	// 824344EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824344F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824344F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824344F8 size=500
    let mut pc: u32 = 0x824344F8;
    'dispatch: loop {
        match pc {
            0x824344F8 => {
    //   block [0x824344F8..0x824346EC)
	// 824344F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824344FC: 48100BB9  bl 0x825350b4
	ctx.lr = 0x82434500;
	sub_82535080(ctx, base);
	// 82434500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434508: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8243450C: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82434510: 817F0518  lwz r11, 0x518(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1304 as u32) ) } as u64;
	// 82434514: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434518: 409A0050  bne cr6, 0x82434568
	if !ctx.cr[6].eq {
	pc = 0x82434568; continue 'dispatch;
	}
	// 8243451C: 817F0530  lwz r11, 0x530(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) } as u64;
	// 82434520: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434524: 409A0044  bne cr6, 0x82434568
	if !ctx.cr[6].eq {
	pc = 0x82434568; continue 'dispatch;
	}
	// 82434528: 83BF0534  lwz r29, 0x534(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1332 as u32) ) } as u64;
	// 8243452C: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434530: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82434534: 814B01F4  lwz r10, 0x1f4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 82434538: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8243453C: 409A002C  bne cr6, 0x82434568
	if !ctx.cr[6].eq {
	pc = 0x82434568; continue 'dispatch;
	}
	// 82434540: 816B01F8  lwz r11, 0x1f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(504 as u32) ) } as u64;
	// 82434544: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82434548: 419A000C  beq cr6, 0x82434554
	if ctx.cr[6].eq {
	pc = 0x82434554; continue 'dispatch;
	}
	// 8243454C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82434550: 409A0018  bne cr6, 0x82434568
	if !ctx.cr[6].eq {
	pc = 0x82434568; continue 'dispatch;
	}
	// 82434554: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82434558: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243455C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434560: 48006F91  bl 0x8243b4f0
	ctx.lr = 0x82434564;
	sub_8243B4F0(ctx, base);
	// 82434564: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82434568: 817F053C  lwz r11, 0x53c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1340 as u32) ) } as u64;
	// 8243456C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82434570: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82434574: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434578: 409A003C  bne cr6, 0x824345b4
	if !ctx.cr[6].eq {
	pc = 0x824345B4; continue 'dispatch;
	}
	// 8243457C: 817F0554  lwz r11, 0x554(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1364 as u32) ) } as u64;
	// 82434580: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434584: 409A0030  bne cr6, 0x824345b4
	if !ctx.cr[6].eq {
	pc = 0x824345B4; continue 'dispatch;
	}
	// 82434588: 83DF0558  lwz r30, 0x558(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1368 as u32) ) } as u64;
	// 8243458C: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434590: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82434594: 816B01F4  lwz r11, 0x1f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 82434598: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243459C: 409A0018  bne cr6, 0x824345b4
	if !ctx.cr[6].eq {
	pc = 0x824345B4; continue 'dispatch;
	}
	// 824345A0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824345A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824345A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824345AC: 48006F45  bl 0x8243b4f0
	ctx.lr = 0x824345B0;
	sub_8243B4F0(ctx, base);
	// 824345B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824345B4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824345B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 824345BC: 393F01F8  addi r9, r31, 0x1f8
	ctx.r[9].s64 = ctx.r[31].s64 + 504;
	// 824345C0: 8169FFFC  lwz r11, -4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824345C4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824345C8: 409A0028  bne cr6, 0x824345f0
	if !ctx.cr[6].eq {
	pc = 0x824345F0; continue 'dispatch;
	}
	// 824345CC: 396AFFFE  addi r11, r10, -2
	ctx.r[11].s64 = ctx.r[10].s64 + -2;
	// 824345D0: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824345D4: 419A001C  beq cr6, 0x824345f0
	if ctx.cr[6].eq {
	pc = 0x824345F0; continue 'dispatch;
	}
	// 824345D8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 824345DC: 419A0014  beq cr6, 0x824345f0
	if ctx.cr[6].eq {
	pc = 0x824345F0; continue 'dispatch;
	}
	// 824345E0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824345E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824345E8: 409A0008  bne cr6, 0x824345f0
	if !ctx.cr[6].eq {
	pc = 0x824345F0; continue 'dispatch;
	}
	// 824345EC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 824345F0: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 824345F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824345F8: 409A0028  bne cr6, 0x82434620
	if !ctx.cr[6].eq {
	pc = 0x82434620; continue 'dispatch;
	}
	// 824345FC: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82434600: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82434604: 419A001C  beq cr6, 0x82434620
	if ctx.cr[6].eq {
	pc = 0x82434620; continue 'dispatch;
	}
	// 82434608: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8243460C: 419A0014  beq cr6, 0x82434620
	if ctx.cr[6].eq {
	pc = 0x82434620; continue 'dispatch;
	}
	// 82434610: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82434614: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434618: 409A0008  bne cr6, 0x82434620
	if !ctx.cr[6].eq {
	pc = 0x82434620; continue 'dispatch;
	}
	// 8243461C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82434620: 8169001C  lwz r11, 0x1c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82434624: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434628: 409A0024  bne cr6, 0x8243464c
	if !ctx.cr[6].eq {
	pc = 0x8243464C; continue 'dispatch;
	}
	// 8243462C: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82434630: 419A001C  beq cr6, 0x8243464c
	if ctx.cr[6].eq {
	pc = 0x8243464C; continue 'dispatch;
	}
	// 82434634: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82434638: 419A0014  beq cr6, 0x8243464c
	if ctx.cr[6].eq {
	pc = 0x8243464C; continue 'dispatch;
	}
	// 8243463C: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82434640: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434644: 409A0008  bne cr6, 0x8243464c
	if !ctx.cr[6].eq {
	pc = 0x8243464C; continue 'dispatch;
	}
	// 82434648: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8243464C: 8169002C  lwz r11, 0x2c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(44 as u32) ) } as u64;
	// 82434650: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434654: 409A0028  bne cr6, 0x8243467c
	if !ctx.cr[6].eq {
	pc = 0x8243467C; continue 'dispatch;
	}
	// 82434658: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8243465C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82434660: 419A001C  beq cr6, 0x8243467c
	if ctx.cr[6].eq {
	pc = 0x8243467C; continue 'dispatch;
	}
	// 82434664: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82434668: 419A0014  beq cr6, 0x8243467c
	if ctx.cr[6].eq {
	pc = 0x8243467C; continue 'dispatch;
	}
	// 8243466C: 81690030  lwz r11, 0x30(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 82434670: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434674: 409A0008  bne cr6, 0x8243467c
	if !ctx.cr[6].eq {
	pc = 0x8243467C; continue 'dispatch;
	}
	// 82434678: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8243467C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82434680: 39290040  addi r9, r9, 0x40
	ctx.r[9].s64 = ctx.r[9].s64 + 64;
	// 82434684: 396AFFFE  addi r11, r10, -2
	ctx.r[11].s64 = ctx.r[10].s64 + -2;
	// 82434688: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8243468C: 4198FF34  blt cr6, 0x824345c0
	if ctx.cr[6].lt {
	pc = 0x824345C0; continue 'dispatch;
	}
	// 82434690: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 82434694: 419A000C  beq cr6, 0x824346a0
	if ctx.cr[6].eq {
	pc = 0x824346A0; continue 'dispatch;
	}
	// 82434698: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 8243469C: 409A0014  bne cr6, 0x824346b0
	if !ctx.cr[6].eq {
	pc = 0x824346B0; continue 'dispatch;
	}
	// 824346A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824346A4: 38800043  li r4, 0x43
	ctx.r[4].s64 = 67;
	// 824346A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824346AC: 4BFFE935  bl 0x82432fe0
	ctx.lr = 0x824346B0;
	sub_82432FE0(ctx, base);
	// 824346B0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824346B4: 409A0010  bne cr6, 0x824346c4
	if !ctx.cr[6].eq {
	pc = 0x824346C4; continue 'dispatch;
	}
	// 824346B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824346BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824346C0: 4BFFE901  bl 0x82432fc0
	ctx.lr = 0x824346C4;
	sub_82432FC0(ctx, base);
	// 824346C4: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 824346C8: 409A001C  bne cr6, 0x824346e4
	if !ctx.cr[6].eq {
	pc = 0x824346E4; continue 'dispatch;
	}
	// 824346CC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824346D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824346D4: 4BFFEAD5  bl 0x824331a8
	ctx.lr = 0x824346D8;
	sub_824331A8(ctx, base);
	// 824346D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824346DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824346E0: 4BFFE8E1  bl 0x82432fc0
	ctx.lr = 0x824346E4;
	sub_82432FC0(ctx, base);
	// 824346E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824346E8: 48100A1C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824346F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824346F0 size=120
    let mut pc: u32 = 0x824346F0;
    'dispatch: loop {
        match pc {
            0x824346F0 => {
    //   block [0x824346F0..0x82434768)
	// 824346F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824346F4: 481009C9  bl 0x825350bc
	ctx.lr = 0x824346F8;
	sub_82535080(ctx, base);
	// 824346F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824346FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82434700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434704: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82434708: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243470C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82434710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434714: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434718: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243471C: 48010EC5  bl 0x824455e0
	ctx.lr = 0x82434720;
	sub_824455E0(ctx, base);
	// 82434720: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434724: 409A003C  bne cr6, 0x82434760
	if !ctx.cr[6].eq {
	pc = 0x82434760; continue 'dispatch;
	}
	// 82434728: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243472C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434730: 409A0030  bne cr6, 0x82434760
	if !ctx.cr[6].eq {
	pc = 0x82434760; continue 'dispatch;
	}
	// 82434734: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82434738: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243473C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82434740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434744: 4801120D  bl 0x82445950
	ctx.lr = 0x82434748;
	sub_82445950(ctx, base);
	// 82434748: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243474C: 409A0014  bne cr6, 0x82434760
	if !ctx.cr[6].eq {
	pc = 0x82434760; continue 'dispatch;
	}
	// 82434750: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82434754: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82434758: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243475C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82434764: 481009A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434768 size=136
    let mut pc: u32 = 0x82434768;
    'dispatch: loop {
        match pc {
            0x82434768 => {
    //   block [0x82434768..0x824347F0)
	// 82434768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82434774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82434778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243477C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82434780: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82434784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434788: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243478C: 48010E55  bl 0x824455e0
	ctx.lr = 0x82434790;
	sub_824455E0(ctx, base);
	// 82434790: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434794: 409A0040  bne cr6, 0x824347d4
	if !ctx.cr[6].eq {
	pc = 0x824347D4; continue 'dispatch;
	}
	// 82434798: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243479C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824347A0: 409A0034  bne cr6, 0x824347d4
	if !ctx.cr[6].eq {
	pc = 0x824347D4; continue 'dispatch;
	}
	// 824347A4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 824347A8: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 824347AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824347B0: 480111F1  bl 0x824459a0
	ctx.lr = 0x824347B4;
	sub_824459A0(ctx, base);
	// 824347B4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824347B8: 409A001C  bne cr6, 0x824347d4
	if !ctx.cr[6].eq {
	pc = 0x824347D4; continue 'dispatch;
	}
	// 824347BC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824347C0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824347C4: 409A0008  bne cr6, 0x824347cc
	if !ctx.cr[6].eq {
	pc = 0x824347CC; continue 'dispatch;
	}
	// 824347C8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824347CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824347D0: 48000008  b 0x824347d8
	pc = 0x824347D8; continue 'dispatch;
	// 824347D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824347D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824347DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824347E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824347E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824347E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824347EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824347F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824347F0 size=52
    let mut pc: u32 = 0x824347F0;
    'dispatch: loop {
        match pc {
            0x824347F0 => {
    //   block [0x824347F0..0x82434824)
	// 824347F0: 816300C8  lwz r11, 0xc8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) } as u64;
	// 824347F4: 7D6A1E70  srawi r10, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 824347F8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 824347FC: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434800: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82434804: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434808: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243480C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434810: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82434814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82434818: 814B00D0  lwz r10, 0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8243481C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82434820: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434824(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434824 size=8
    let mut pc: u32 = 0x82434824;
    'dispatch: loop {
        match pc {
            0x82434824 => {
    //   block [0x82434824..0x8243482C)
	// 82434824: 806B00E4  lwz r3, 0xe4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 82434828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434830 size=336
    let mut pc: u32 = 0x82434830;
    'dispatch: loop {
        match pc {
            0x82434830 => {
    //   block [0x82434830..0x82434980)
	// 82434830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434834: 48100851  bl 0x82535084
	ctx.lr = 0x82434838;
	sub_82535080(ctx, base);
	// 82434838: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243483C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82434840: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82434844: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82434848: 835D0048  lwz r26, 0x48(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243484C: 831E0020  lwz r24, 0x20(r30)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82434850: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82434854: 4BFFF71D  bl 0x82433f70
	ctx.lr = 0x82434858;
	sub_82433F70(ctx, base);
	// 82434858: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8243485C: 82DE0000  lwz r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82434860: 82BE0004  lwz r21, 4(r30)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82434864: 829E0008  lwz r20, 8(r30)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82434868: 827E000C  lwz r19, 0xc(r30)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243486C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82434870: 4BFFF729  bl 0x82433f98
	ctx.lr = 0x82434874;
	sub_82433F98(ctx, base);
	// 82434874: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82434878: 837E0034  lwz r27, 0x34(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8243487C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82434880: 839E0018  lwz r28, 0x18(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82434884: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434888: 835E0030  lwz r26, 0x30(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8243488C: 823E002C  lwz r17, 0x2c(r30)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82434890: 821E0024  lwz r16, 0x24(r30)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82434894: 81FE0028  lwz r15, 0x28(r30)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82434898: 4800A669  bl 0x8243ef00
	ctx.lr = 0x8243489C;
	sub_8243EF00(ctx, base);
	// 8243489C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824348A0: 419A0010  beq cr6, 0x824348b0
	if ctx.cr[6].eq {
	pc = 0x824348B0; continue 'dispatch;
	}
	// 824348A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824348A8: 386B5080  addi r3, r11, 0x5080
	ctx.r[3].s64 = ctx.r[11].s64 + 20608;
	// 824348AC: 4800281D  bl 0x824370c8
	ctx.lr = 0x824348B0;
	sub_824370C8(ctx, base);
	// 824348B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824348B4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824348B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824348BC: 4BFFF6AD  bl 0x82433f68
	ctx.lr = 0x824348C0;
	sub_82433F68(ctx, base);
	// 824348C0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824348C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824348C8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824348CC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824348D0: 4BFFF699  bl 0x82433f68
	ctx.lr = 0x824348D4;
	sub_82433F68(ctx, base);
	// 824348D4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824348D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824348DC: 931F0000  stw r24, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 824348E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824348E4: 92FF0008  stw r23, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 824348E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824348EC: 92DF000C  stw r22, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[22].u32 ) };
	// 824348F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824348F4: 92BF0010  stw r21, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[21].u32 ) };
	// 824348F8: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824348FC: 929F0014  stw r20, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[20].u32 ) };
	// 82434900: 927F0018  stw r19, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[19].u32 ) };
	// 82434904: 925F001C  stw r18, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[18].u32 ) };
	// 82434908: 933F0024  stw r25, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[25].u32 ) };
	// 8243490C: 937F0028  stw r27, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[27].u32 ) };
	// 82434910: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82434914: 923F0030  stw r17, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[17].u32 ) };
	// 82434918: 935F0038  stw r26, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[26].u32 ) };
	// 8243491C: 921F003C  stw r16, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[16].u32 ) };
	// 82434920: 91FF0040  stw r15, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[15].u32 ) };
	// 82434924: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82434928: 4BFFF5C9  bl 0x82433ef0
	ctx.lr = 0x8243492C;
	sub_82433EF0(ctx, base);
	// 8243492C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82434930: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434934: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82434938: 4BFFF7D9  bl 0x82434110
	ctx.lr = 0x8243493C;
	sub_82434110(ctx, base);
	// 8243493C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434940: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82434944: 80DF009C  lwz r6, 0x9c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82434948: 80BF0098  lwz r5, 0x98(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8243494C: 4BFFF6ED  bl 0x82434038
	ctx.lr = 0x82434950;
	sub_82434038(ctx, base);
	// 82434950: 397F0060  addi r11, r31, 0x60
	ctx.r[11].s64 = ctx.r[31].s64 + 96;
	// 82434954: 393E0048  addi r9, r30, 0x48
	ctx.r[9].s64 = ctx.r[30].s64 + 72;
	// 82434958: 907F004C  stw r3, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 8243495C: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 82434960: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82434964: E9490000  ld r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82434968: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8243496C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82434970: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82434974: 4200FFF0  bdnz 0x82434964
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82434964; continue 'dispatch;
	}
	// 82434978: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8243497C: 48100758  b 0x825350d4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434980 size=60
    let mut pc: u32 = 0x82434980;
    'dispatch: loop {
        match pc {
            0x82434980 => {
    //   block [0x82434980..0x824349BC)
	// 82434980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434988: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243498C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434990: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82434994: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82434998: 4800FEC9  bl 0x82444860
	ctx.lr = 0x8243499C;
	sub_82444860(ctx, base);
	// 8243499C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824349A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824349A4: 4BFFF7FD  bl 0x824341a0
	ctx.lr = 0x824349A8;
	sub_824341A0(ctx, base);
	// 824349A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824349AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824349B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824349B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824349B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824349C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824349C0 size=4
    let mut pc: u32 = 0x824349C0;
    'dispatch: loop {
        match pc {
            0x824349C0 => {
    //   block [0x824349C0..0x824349C4)
	// 824349C0: 4BFFFE30  b 0x824347f0
	sub_824347F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824349C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824349C8 size=260
    let mut pc: u32 = 0x824349C8;
    'dispatch: loop {
        match pc {
            0x824349C8 => {
    //   block [0x824349C8..0x82434ACC)
	// 824349C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824349CC: 481006DD  bl 0x825350a8
	ctx.lr = 0x824349D0;
	sub_82535080(ctx, base);
	// 824349D0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824349D4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824349D8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824349DC: 817A01F0  lwz r11, 0x1f0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(496 as u32) ) } as u64;
	// 824349E0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824349E4: 419A00E0  beq cr6, 0x82434ac4
	if ctx.cr[6].eq {
	pc = 0x82434AC4; continue 'dispatch;
	}
	// 824349E8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824349EC: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 824349F0: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 824349F4: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
	// 824349F8: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824349FC: 393D00C0  addi r9, r29, 0xc0
	ctx.r[9].s64 = ctx.r[29].s64 + 192;
	// 82434A00: 7FEBD214  add r31, r11, r26
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82434A04: 55592036  slwi r25, r10, 4
	ctx.r[25].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82434A08: 553E063E  clrlwi r30, r9, 0x18
	ctx.r[30].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82434A0C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82434A10: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434A14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434A18: 939F01F4  stw r28, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[28].u32 ) };
	// 82434A1C: 939F01F8  stw r28, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[28].u32 ) };
	// 82434A20: 939F01FC  stw r28, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[28].u32 ) };
	// 82434A24: 7F99D12E  stwx r28, r25, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32), ctx.r[28].u32) };
	// 82434A28: 48010BB9  bl 0x824455e0
	ctx.lr = 0x82434A2C;
	sub_824455E0(ctx, base);
	// 82434A2C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434A30: 409A0078  bne cr6, 0x82434aa8
	if !ctx.cr[6].eq {
	pc = 0x82434AA8; continue 'dispatch;
	}
	// 82434A34: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434A38: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434A3C: 409A006C  bne cr6, 0x82434aa8
	if !ctx.cr[6].eq {
	pc = 0x82434AA8; continue 'dispatch;
	}
	// 82434A40: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82434A44: 931F01F4  stw r24, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[24].u32 ) };
	// 82434A48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434A4C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434A50: 48010E11  bl 0x82445860
	ctx.lr = 0x82434A54;
	sub_82445860(ctx, base);
	// 82434A54: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434A58: 409A0010  bne cr6, 0x82434a68
	if !ctx.cr[6].eq {
	pc = 0x82434A68; continue 'dispatch;
	}
	// 82434A5C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82434A60: 4BFFFA21  bl 0x82434480
	ctx.lr = 0x82434A64;
	sub_82434480(ctx, base);
	// 82434A64: 907F01F8  stw r3, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[3].u32 ) };
	// 82434A68: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82434A6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434A70: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434A74: 48010E3D  bl 0x824458b0
	ctx.lr = 0x82434A78;
	sub_824458B0(ctx, base);
	// 82434A78: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434A7C: 409A000C  bne cr6, 0x82434a88
	if !ctx.cr[6].eq {
	pc = 0x82434A88; continue 'dispatch;
	}
	// 82434A80: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82434A84: 917F01FC  stw r11, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u32 ) };
	// 82434A88: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82434A8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434A90: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434A94: 48010E45  bl 0x824458d8
	ctx.lr = 0x82434A98;
	sub_824458D8(ctx, base);
	// 82434A98: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434A9C: 409A000C  bne cr6, 0x82434aa8
	if !ctx.cr[6].eq {
	pc = 0x82434AA8; continue 'dispatch;
	}
	// 82434AA0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82434AA4: 7D79D12E  stwx r11, r25, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32), ctx.r[11].u32) };
	// 82434AA8: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82434AAC: 557D063E  clrlwi r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82434AB0: 2B1D0020  cmplwi cr6, r29, 0x20
	ctx.cr[6].compare_u32(ctx.r[29].u32, 32 as u32, &mut ctx.xer);
	// 82434AB4: 4198FF40  blt cr6, 0x824349f4
	if ctx.cr[6].lt {
	pc = 0x824349F4; continue 'dispatch;
	}
	// 82434AB8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82434ABC: 931A01F0  stw r24, 0x1f0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(496 as u32), ctx.r[24].u32 ) };
	// 82434AC0: 4BFFFA39  bl 0x824344f8
	ctx.lr = 0x82434AC4;
	sub_824344F8(ctx, base);
	// 82434AC4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82434AC8: 48100630  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434AD0 size=316
    let mut pc: u32 = 0x82434AD0;
    'dispatch: loop {
        match pc {
            0x82434AD0 => {
    //   block [0x82434AD0..0x82434C0C)
	// 82434AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434AD4: 481005E1  bl 0x825350b4
	ctx.lr = 0x82434AD8;
	sub_82535080(ctx, base);
	// 82434AD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434AE0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82434AE4: 4BFFE57D  bl 0x82433060
	ctx.lr = 0x82434AE8;
	sub_82433060(ctx, base);
	// 82434AE8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434AEC: 419A0030  beq cr6, 0x82434b1c
	if ctx.cr[6].eq {
	pc = 0x82434B1C; continue 'dispatch;
	}
	// 82434AF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434AF4: 386B50A0  addi r3, r11, 0x50a0
	ctx.r[3].s64 = ctx.r[11].s64 + 20640;
	// 82434AF8: 480025D1  bl 0x824370c8
	ctx.lr = 0x82434AFC;
	sub_824370C8(ctx, base);
	// 82434AFC: 38A000A0  li r5, 0xa0
	ctx.r[5].s64 = 160;
	// 82434B00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82434B04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434B08: 481006C9  bl 0x825351d0
	ctx.lr = 0x82434B0C;
	sub_825351D0(ctx, base);
	// 82434B0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434B10: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434B14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82434B18: 481005EC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82434B1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82434B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434B24: 4BFFF355  bl 0x82433e78
	ctx.lr = 0x82434B28;
	sub_82433E78(ctx, base);
	// 82434B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434B2C: 4BFFE905  bl 0x82433430
	ctx.lr = 0x82434B30;
	sub_82433430(ctx, base);
	// 82434B30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82434B34: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82434B38: 419AFFC4  beq cr6, 0x82434afc
	if ctx.cr[6].eq {
	pc = 0x82434AFC; continue 'dispatch;
	}
	// 82434B3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434B40: 48005B09  bl 0x8243a648
	ctx.lr = 0x82434B44;
	sub_8243A648(ctx, base);
	// 82434B44: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434B48: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82434B4C: 419AFFC0  beq cr6, 0x82434b0c
	if ctx.cr[6].eq {
	pc = 0x82434B0C; continue 'dispatch;
	}
	// 82434B50: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82434B54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82434B58: 409A0058  bne cr6, 0x82434bb0
	if !ctx.cr[6].eq {
	pc = 0x82434BB0; continue 'dispatch;
	}
	// 82434B5C: 839F0018  lwz r28, 0x18(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82434B60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82434B64: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82434B68: 40990048  ble cr6, 0x82434bb0
	if !ctx.cr[6].gt {
	pc = 0x82434BB0; continue 'dispatch;
	}
	// 82434B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434B70: 4BFFF6E9  bl 0x82434258
	ctx.lr = 0x82434B74;
	sub_82434258(ctx, base);
	// 82434B74: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434B78: 409A0034  bne cr6, 0x82434bac
	if !ctx.cr[6].eq {
	pc = 0x82434BAC; continue 'dispatch;
	}
	// 82434B7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82434B80: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434B84: 48005B9D  bl 0x8243a720
	ctx.lr = 0x82434B88;
	sub_8243A720(ctx, base);
	// 82434B88: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82434B8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434B90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434B94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82434B98: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82434B9C: 48005AAD  bl 0x8243a648
	ctx.lr = 0x82434BA0;
	sub_8243A648(ctx, base);
	// 82434BA0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82434BA4: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82434BA8: 4198FFC4  blt cr6, 0x82434b6c
	if ctx.cr[6].lt {
	pc = 0x82434B6C; continue 'dispatch;
	}
	// 82434BAC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434BB0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82434BB4: 419AFF58  beq cr6, 0x82434b0c
	if ctx.cr[6].eq {
	pc = 0x82434B0C; continue 'dispatch;
	}
	// 82434BB8: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82434BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434BC0: 909F0088  stw r4, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[4].u32 ) };
	// 82434BC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434BC8: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82434BCC: 4BFFF4F5  bl 0x824340c0
	ctx.lr = 0x82434BD0;
	sub_824340C0(ctx, base);
	// 82434BD0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82434BD4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434BD8: 4BFFFC59  bl 0x82434830
	ctx.lr = 0x82434BDC;
	sub_82434830(ctx, base);
	// 82434BDC: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82434BE0: 815B0030  lwz r10, 0x30(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 82434BE4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82434BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434BEC: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82434BF0: 915F00C8  stw r10, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 82434BF4: 4BFFF2AD  bl 0x82433ea0
	ctx.lr = 0x82434BF8;
	sub_82433EA0(ctx, base);
	// 82434BF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82434BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C00: 4BFFF209  bl 0x82433e08
	ctx.lr = 0x82434C04;
	sub_82433E08(ctx, base);
	// 82434C04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82434C08: 481004FC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82434C10 size=320
    let mut pc: u32 = 0x82434C10;
    'dispatch: loop {
        match pc {
            0x82434C10 => {
    //   block [0x82434C10..0x82434D50)
	// 82434C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434C14: 48100499  bl 0x825350ac
	ctx.lr = 0x82434C18;
	sub_82535080(ctx, base);
	// 82434C18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434C1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82434C20: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82434C24: 2F050800  cmpwi cr6, r5, 0x800
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2048, &mut ctx.xer);
	// 82434C28: 817E00C4  lwz r11, 0xc4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82434C2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434C30: 917E00C4  stw r11, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82434C34: 41980114  blt cr6, 0x82434d48
	if ctx.cr[6].lt {
	pc = 0x82434D48; continue 'dispatch;
	}
	// 82434C38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82434C3C: 419A010C  beq cr6, 0x82434d48
	if ctx.cr[6].eq {
	pc = 0x82434D48; continue 'dispatch;
	}
	// 82434C40: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82434C44: 48010745  bl 0x82445388
	ctx.lr = 0x82434C48;
	sub_82445388(ctx, base);
	// 82434C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434C4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82434C50: 419A00F8  beq cr6, 0x82434d48
	if ctx.cr[6].eq {
	pc = 0x82434D48; continue 'dispatch;
	}
	// 82434C54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434C58: 48010801  bl 0x82445458
	ctx.lr = 0x82434C5C;
	sub_82445458(ctx, base);
	// 82434C5C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434C60: 409A00E0  bne cr6, 0x82434d40
	if !ctx.cr[6].eq {
	pc = 0x82434D40; continue 'dispatch;
	}
	// 82434C64: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434C68: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434C6C: 409A00D4  bne cr6, 0x82434d40
	if !ctx.cr[6].eq {
	pc = 0x82434D40; continue 'dispatch;
	}
	// 82434C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C74: 4BFFFAF5  bl 0x82434768
	ctx.lr = 0x82434C78;
	sub_82434768(ctx, base);
	// 82434C78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82434C7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C80: 4BFFF649  bl 0x824342c8
	ctx.lr = 0x82434C84;
	sub_824342C8(ctx, base);
	// 82434C84: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82434C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C8C: 4BFFF6F5  bl 0x82434380
	ctx.lr = 0x82434C90;
	sub_82434380(ctx, base);
	// 82434C90: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82434C94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C98: 4BFFF6A1  bl 0x82434338
	ctx.lr = 0x82434C9C;
	sub_82434338(ctx, base);
	// 82434C9C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82434CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434CA4: 4BFFF65D  bl 0x82434300
	ctx.lr = 0x82434CA8;
	sub_82434300(ctx, base);
	// 82434CA8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82434CAC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82434CB0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82434CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434CB8: 4BFFFA39  bl 0x824346f0
	ctx.lr = 0x82434CBC;
	sub_824346F0(ctx, base);
	// 82434CBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82434CC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82434CC4: 4BFFFD05  bl 0x824349c8
	ctx.lr = 0x82434CC8;
	sub_824349C8(ctx, base);
	// 82434CC8: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82434CCC: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82434CD0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82434CD4: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82434CD8: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82434CDC: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 82434CE0: 80C10058  lwz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82434CE4: 394B0006  addi r10, r11, 6
	ctx.r[10].s64 = ctx.r[11].s64 + 6;
	// 82434CE8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82434CEC: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82434CF0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434CF4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82434CF8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82434CFC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434D00: 910B00D4  stw r8, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[8].u32 ) };
	// 82434D04: 7CEAF12E  stwx r7, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[7].u32) };
	// 82434D08: 93AB00E0  stw r29, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[29].u32 ) };
	// 82434D0C: 938B00E4  stw r28, 0xe4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[28].u32 ) };
	// 82434D10: 936B00E8  stw r27, 0xe8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), ctx.r[27].u32 ) };
	// 82434D14: 934B00EC  stw r26, 0xec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[26].u32 ) };
	// 82434D18: 932B00F0  stw r25, 0xf0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[25].u32 ) };
	// 82434D1C: 90CB00DC  stw r6, 0xdc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), ctx.r[6].u32 ) };
	// 82434D20: 90AB00D0  stw r5, 0xd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[5].u32 ) };
	// 82434D24: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82434D28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434D2C: 7D6A1E70  srawi r10, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82434D30: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82434D34: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434D38: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82434D3C: 917E00CC  stw r11, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82434D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434D44: 480106DD  bl 0x82445420
	ctx.lr = 0x82434D48;
	sub_82445420(ctx, base);
	// 82434D48: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82434D4C: 481003B0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434D50 size=20
    let mut pc: u32 = 0x82434D50;
    'dispatch: loop {
        match pc {
            0x82434D50 => {
    //   block [0x82434D50..0x82434D64)
	// 82434D50: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82434D54: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82434D58: 388B4C10  addi r4, r11, 0x4c10
	ctx.r[4].s64 = ctx.r[11].s64 + 19472;
	// 82434D5C: 80650048  lwz r3, 0x48(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(72 as u32) ) } as u64;
	// 82434D60: 480079B8  b 0x8243c718
	sub_8243C718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82434D68 size=280
    let mut pc: u32 = 0x82434D68;
    'dispatch: loop {
        match pc {
            0x82434D68 => {
    //   block [0x82434D68..0x82434E80)
	// 82434D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434D6C: 4810034D  bl 0x825350b8
	ctx.lr = 0x82434D70;
	sub_82535080(ctx, base);
	// 82434D70: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82434D74: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82434D78: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82434D7C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82434D80: 41990008  bgt cr6, 0x82434d88
	if ctx.cr[6].gt {
	pc = 0x82434D88; continue 'dispatch;
	}
	// 82434D84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82434D88: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 82434D8C: 419A00BC  beq cr6, 0x82434e48
	if ctx.cr[6].eq {
	pc = 0x82434E48; continue 'dispatch;
	}
	// 82434D90: 2F1F0006  cmpwi cr6, r31, 6
	ctx.cr[6].compare_i32(ctx.r[31].s32, 6, &mut ctx.xer);
	// 82434D94: 419A00B4  beq cr6, 0x82434e48
	if ctx.cr[6].eq {
	pc = 0x82434E48; continue 'dispatch;
	}
	// 82434D98: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 82434D9C: 419A0064  beq cr6, 0x82434e00
	if ctx.cr[6].eq {
	pc = 0x82434E00; continue 'dispatch;
	}
	// 82434DA0: 2F1F0007  cmpwi cr6, r31, 7
	ctx.cr[6].compare_i32(ctx.r[31].s32, 7, &mut ctx.xer);
	// 82434DA4: 419A005C  beq cr6, 0x82434e00
	if ctx.cr[6].eq {
	pc = 0x82434E00; continue 'dispatch;
	}
	// 82434DA8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82434DAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82434DB0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82434DB4: 3FA00000  lis r29, 0
	ctx.r[29].s64 = 0;
	// 82434DB8: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82434DBC: 3BE05DCC  li r31, 0x5dcc
	ctx.r[31].s64 = 24012;
	// 82434DC0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82434DC4: 3BC05F0C  li r30, 0x5f0c
	ctx.r[30].s64 = 24332;
	// 82434DC8: 557C5828  slwi r28, r11, 0xb
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82434DCC: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82434DD0: 7F8A0E70  srawi r10, r28, 1
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[28].s32 >> 1) as i64;
	// 82434DD4: 557C5828  slwi r28, r11, 0xb
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82434DD8: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82434DDC: 63BD81C0  ori r29, r29, 0x81c0
	ctx.r[29].u64 = ctx.r[29].u64 | 33216;
	// 82434DE0: 396B0800  addi r11, r11, 0x800
	ctx.r[11].s64 = ctx.r[11].s64 + 2048;
	// 82434DE4: 93840000  stw r28, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82434DE8: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82434DEC: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434DF0: 93E70000  stw r31, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82434DF4: 93C80000  stw r30, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82434DF8: 93A90000  stw r29, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82434DFC: 4810030C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82434E00: 7D631E70  srawi r3, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82434E04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434E08: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82434E0C: 7C635E70  srawi r3, r3, 0xb
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 11) as i64;
	// 82434E10: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82434E14: 547F5828  slwi r31, r3, 0xb
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(11);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82434E18: 7D4351D6  mullw r10, r3, r10
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82434E1C: 7FE30E70  srawi r3, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 82434E20: 555F5828  slwi r31, r10, 0xb
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82434E24: 7D430194  addze r10, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82434E28: 394A0800  addi r10, r10, 0x800
	ctx.r[10].s64 = ctx.r[10].s64 + 2048;
	// 82434E2C: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82434E30: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E34: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434E38: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E3C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E40: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E44: 481002C4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82434E48: 7D631E70  srawi r3, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82434E4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434E50: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82434E54: 7C635E70  srawi r3, r3, 0xb
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 11) as i64;
	// 82434E58: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82434E5C: 7D4351D6  mullw r10, r3, r10
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82434E60: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434E64: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434E68: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E6C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E70: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E74: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E78: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E7C: 4810028C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82434E80 size=60
    let mut pc: u32 = 0x82434E80;
    'dispatch: loop {
        match pc {
            0x82434E80 => {
    //   block [0x82434E80..0x82434EBC)
	// 82434E80: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82434E84: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82434E88: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82434E8C: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82434E90: 3929001F  addi r9, r9, 0x1f
	ctx.r[9].s64 = ctx.r[9].s64 + 31;
	// 82434E94: 7D292E70  srawi r9, r9, 5
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 5) as i64;
	// 82434E98: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82434E9C: 7D480E70  srawi r8, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 82434EA0: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82434EA4: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82434EA8: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82434EAC: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82434EB0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434EB4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434EC0 size=132
    let mut pc: u32 = 0x82434EC0;
    'dispatch: loop {
        match pc {
            0x82434EC0 => {
    //   block [0x82434EC0..0x82434EFC)
	// 82434EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434ECC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82434ED0: 41990054  bgt cr6, 0x82434f24
	if ctx.cr[6].gt {
	pc = 0x82434F24; continue 'dispatch;
	}
	// 82434ED4: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82434ED8: 398C4EEC  addi r12, r12, 0x4eec
	ctx.r[12].s64 = ctx.r[12].s64 + 20204;
	// 82434EDC: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82434EE0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82434EE4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82434EE8: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
	pc = 0x82434F30; continue 'dispatch;
		},
		1 => {
	pc = 0x82434EFC; continue 'dispatch;
		},
		2 => {
	pc = 0x82434F10; continue 'dispatch;
		},
		3 => {
	pc = 0x82434F30; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82434EEC: 82434F30  lwz r18, 0x4f30(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20272 as u32) ) } as u64;
	// 82434EF0: 82434EFC  lwz r18, 0x4efc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20220 as u32) ) } as u64;
	// 82434EF4: 82434F10  lwz r18, 0x4f10(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20240 as u32) ) } as u64;
	// 82434EF8: 82434F30  lwz r18, 0x4f30(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20272 as u32) ) } as u64;
            }
            0x82434EFC => {
    //   block [0x82434EFC..0x82434F10)
	// 82434EFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82434F00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434F0C: 4E800020  blr
	return;
            }
            0x82434F10 => {
    //   block [0x82434F10..0x82434F30)
	// 82434F10: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82434F14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434F20: 4E800020  blr
	return;
	// 82434F24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434F28: 386B5138  addi r3, r11, 0x5138
	ctx.r[3].s64 = ctx.r[11].s64 + 20792;
	// 82434F2C: 4800219D  bl 0x824370c8
	ctx.lr = 0x82434F30;
	sub_824370C8(ctx, base);
            }
            0x82434F30 => {
    //   block [0x82434F30..0x82434F44)
	// 82434F30: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82434F34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434F48 size=28
    let mut pc: u32 = 0x82434F48;
    'dispatch: loop {
        match pc {
            0x82434F48 => {
    //   block [0x82434F48..0x82434F64)
	// 82434F48: 39604000  li r11, 0x4000
	ctx.r[11].s64 = 16384;
	// 82434F4C: 39400700  li r10, 0x700
	ctx.r[10].s64 = 1792;
	// 82434F50: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434F54: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434F58: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82434F5C: 386B0700  addi r3, r11, 0x700
	ctx.r[3].s64 = ctx.r[11].s64 + 1792;
	// 82434F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434F68 size=8
    let mut pc: u32 = 0x82434F68;
    'dispatch: loop {
        match pc {
            0x82434F68 => {
    //   block [0x82434F68..0x82434F70)
	// 82434F68: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82434F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434F70 size=88
    let mut pc: u32 = 0x82434F70;
    'dispatch: loop {
        match pc {
            0x82434F70 => {
    //   block [0x82434F70..0x82434FC8)
	// 82434F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434F78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82434F7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82434F80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434F88: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82434F8C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82434F90: 4BFFCC41  bl 0x82431bd0
	ctx.lr = 0x82434F94;
	sub_82431BD0(ctx, base);
	// 82434F94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82434F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434F9C: 4BFFD015  bl 0x82431fb0
	ctx.lr = 0x82434FA0;
	sub_82431FB0(ctx, base);
	// 82434FA0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434FA4: 3C7E0002  addis r3, r30, 2
	ctx.r[3].s64 = ctx.r[30].s64 + 131072;
	// 82434FA8: 419A0008  beq cr6, 0x82434fb0
	if ctx.cr[6].eq {
	pc = 0x82434FB0; continue 'dispatch;
	}
	// 82434FAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82434FB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82434FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434FBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82434FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82434FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82434FC8 size=100
    let mut pc: u32 = 0x82434FC8;
    'dispatch: loop {
        match pc {
            0x82434FC8 => {
    //   block [0x82434FC8..0x8243502C)
	// 82434FC8: 3963000F  addi r11, r3, 0xf
	ctx.r[11].s64 = ctx.r[3].s64 + 15;
	// 82434FCC: 3944000F  addi r10, r4, 0xf
	ctx.r[10].s64 = ctx.r[4].s64 + 15;
	// 82434FD0: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82434FD4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82434FD8: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82434FDC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434FE0: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82434FE4: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82434FE8: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82434FEC: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82434FF0: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82434FF4: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 82434FF8: 7D293E70  srawi r9, r9, 7
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 7) as i64;
	// 82434FFC: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82435000: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 82435004: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435008: 7D673E70  srawi r7, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 8243500C: 7D6941D6  mullw r11, r9, r8
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82435010: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82435014: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82435018: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243501C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435020: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435024: 386B0080  addi r3, r11, 0x80
	ctx.r[3].s64 = ctx.r[11].s64 + 128;
	// 82435028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435030 size=224
    let mut pc: u32 = 0x82435030;
    'dispatch: loop {
        match pc {
            0x82435030 => {
    //   block [0x82435030..0x82435110)
	// 82435030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243503C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82435040: 814B0450  lwz r10, 0x450(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1104 as u32) ) } as u64;
	// 82435044: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82435048: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8243504C: 419A00B4  beq cr6, 0x82435100
	if ctx.cr[6].eq {
	pc = 0x82435100; continue 'dispatch;
	}
	// 82435050: 812B0474  lwz r9, 0x474(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1140 as u32) ) } as u64;
	// 82435054: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82435058: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8243505C: 409A002C  bne cr6, 0x82435088
	if !ctx.cr[6].eq {
	pc = 0x82435088; continue 'dispatch;
	}
	// 82435060: 812B0478  lwz r9, 0x478(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1144 as u32) ) } as u64;
	// 82435064: 816B047C  lwz r11, 0x47c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1148 as u32) ) } as u64;
	// 82435068: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8243506C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82435070: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82435074: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82435078: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8243507C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82435080: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82435084: 48000058  b 0x824350dc
	pc = 0x824350DC; continue 'dispatch;
	// 82435088: 812B0454  lwz r9, 0x454(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1108 as u32) ) } as u64;
	// 8243508C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82435090: 409A0024  bne cr6, 0x824350b4
	if !ctx.cr[6].eq {
	pc = 0x824350B4; continue 'dispatch;
	}
	// 82435094: 812B0458  lwz r9, 0x458(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1112 as u32) ) } as u64;
	// 82435098: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8243509C: 812B045C  lwz r9, 0x45c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1116 as u32) ) } as u64;
	// 824350A0: 816B0460  lwz r11, 0x460(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1120 as u32) ) } as u64;
	// 824350A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 824350A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824350AC: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 824350B0: 48000024  b 0x824350d4
	pc = 0x824350D4; continue 'dispatch;
	// 824350B4: 814B0468  lwz r10, 0x468(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1128 as u32) ) } as u64;
	// 824350B8: 812B0464  lwz r9, 0x464(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1124 as u32) ) } as u64;
	// 824350BC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 824350C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824350C4: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 824350C8: 814B046C  lwz r10, 0x46c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 824350CC: 816B0470  lwz r11, 0x470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1136 as u32) ) } as u64;
	// 824350D0: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 824350D4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824350D8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 824350DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824350E0: 48004F69  bl 0x8243a048
	ctx.lr = 0x824350E4;
	sub_8243A048(ctx, base);
	// 824350E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824350E8: 419A0018  beq cr6, 0x82435100
	if ctx.cr[6].eq {
	pc = 0x82435100; continue 'dispatch;
	}
	// 824350EC: 3860FEC8  li r3, -0x138
	ctx.r[3].s64 = -312;
	// 824350F0: 480018D9  bl 0x824369c8
	ctx.lr = 0x824350F4;
	sub_824369C8(ctx, base);
	// 824350F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824350F8: 386B5160  addi r3, r11, 0x5160
	ctx.r[3].s64 = ctx.r[11].s64 + 20832;
	// 824350FC: 48001FCD  bl 0x824370c8
	ctx.lr = 0x82435100;
	sub_824370C8(ctx, base);
	// 82435100: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82435104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435110 size=76
    let mut pc: u32 = 0x82435110;
    'dispatch: loop {
        match pc {
            0x82435110 => {
    //   block [0x82435110..0x8243514C)
	// 82435110: 3963FFFE  addi r11, r3, -2
	ctx.r[11].s64 = ctx.r[3].s64 + -2;
	// 82435114: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82435118: 4199003C  bgt cr6, 0x82435154
	if ctx.cr[6].gt {
	pc = 0x82435154; continue 'dispatch;
	}
	// 8243511C: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82435120: 398C5134  addi r12, r12, 0x5134
	ctx.r[12].s64 = ctx.r[12].s64 + 20788;
	// 82435124: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82435128: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8243512C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82435130: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8243514C; continue 'dispatch;
		},
		1 => {
	pc = 0x8243514C; continue 'dispatch;
		},
		2 => {
	pc = 0x82435154; continue 'dispatch;
		},
		3 => {
	pc = 0x82435154; continue 'dispatch;
		},
		4 => {
	pc = 0x8243514C; continue 'dispatch;
		},
		5 => {
	pc = 0x8243514C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82435134: 8243514C  lwz r18, 0x514c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20812 as u32) ) } as u64;
	// 82435138: 8243514C  lwz r18, 0x514c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20812 as u32) ) } as u64;
	// 8243513C: 82435154  lwz r18, 0x5154(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20820 as u32) ) } as u64;
	// 82435140: 82435154  lwz r18, 0x5154(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20820 as u32) ) } as u64;
	// 82435144: 8243514C  lwz r18, 0x514c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20812 as u32) ) } as u64;
	// 82435148: 8243514C  lwz r18, 0x514c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20812 as u32) ) } as u64;
            }
            0x8243514C => {
    //   block [0x8243514C..0x82435154)
	// 8243514C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435150: 4E800020  blr
	return;
            }
            0x82435154 => {
    //   block [0x82435154..0x8243515C)
	// 82435154: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82435158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435160 size=24
    let mut pc: u32 = 0x82435160;
    'dispatch: loop {
        match pc {
            0x82435160 => {
    //   block [0x82435160..0x82435178)
	// 82435160: 396303F4  addi r11, r3, 0x3f4
	ctx.r[11].s64 = ctx.r[3].s64 + 1012;
	// 82435164: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82435168: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8243516C: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82435170: 9163040C  stw r11, 0x40c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1036 as u32), ctx.r[11].u32 ) };
	// 82435174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435178 size=140
    let mut pc: u32 = 0x82435178;
    'dispatch: loop {
        match pc {
            0x82435178 => {
    //   block [0x82435178..0x82435204)
	// 82435178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243517C: 480FFF3D  bl 0x825350b8
	ctx.lr = 0x82435180;
	sub_82535080(ctx, base);
	// 82435180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435184: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82435188: 817D040C  lwz r11, 0x40c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1036 as u32) ) } as u64;
	// 8243518C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435190: 409A0018  bne cr6, 0x824351a8
	if !ctx.cr[6].eq {
	pc = 0x824351A8; continue 'dispatch;
	}
	// 82435194: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435198: 386B51BC  addi r3, r11, 0x51bc
	ctx.r[3].s64 = ctx.r[11].s64 + 20924;
	// 8243519C: 48001F2D  bl 0x824370c8
	ctx.lr = 0x824351A0;
	sub_824370C8(ctx, base);
	// 824351A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824351A4: 480FFF64  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824351A8: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 824351AC: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824351B0: 3BCA0003  addi r30, r10, 3
	ctx.r[30].s64 = ctx.r[10].s64 + 3;
	// 824351B4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824351B8: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824351BC: 7D7EF9D6  mullw r11, r30, r31
	ctx.r[11].s64 = (ctx.r[30].s32 as i64) * (ctx.r[31].s32 as i64);
	// 824351C0: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824351C4: 40980018  bge cr6, 0x824351dc
	if !ctx.cr[6].lt {
	pc = 0x824351DC; continue 'dispatch;
	}
	// 824351C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824351CC: 386B5180  addi r3, r11, 0x5180
	ctx.r[3].s64 = ctx.r[11].s64 + 20864;
	// 824351D0: 48001EF9  bl 0x824370c8
	ctx.lr = 0x824351D4;
	sub_824370C8(ctx, base);
	// 824351D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824351D8: 480FFF30  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824351DC: 480016D5  bl 0x824368b0
	ctx.lr = 0x824351E0;
	sub_824368B0(ctx, base);
	// 824351E0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824351E4: 409A0018  bne cr6, 0x824351fc
	if !ctx.cr[6].eq {
	pc = 0x824351FC; continue 'dispatch;
	}
	// 824351E8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824351EC: 807D0048  lwz r3, 0x48(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 824351F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824351F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824351F8: 4800FE89  bl 0x82445080
	ctx.lr = 0x824351FC;
	sub_82445080(ctx, base);
	// 824351FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435200: 480FFF08  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435208 size=60
    let mut pc: u32 = 0x82435208;
    'dispatch: loop {
        match pc {
            0x82435208 => {
    //   block [0x82435208..0x82435244)
	// 82435208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243520C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435214: 48005F9D  bl 0x8243b1b0
	ctx.lr = 0x82435218;
	sub_8243B1B0(ctx, base);
	// 82435218: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243521C: 419A0018  beq cr6, 0x82435234
	if ctx.cr[6].eq {
	pc = 0x82435234; continue 'dispatch;
	}
	// 82435220: 3860FECE  li r3, -0x132
	ctx.r[3].s64 = -306;
	// 82435224: 480017A5  bl 0x824369c8
	ctx.lr = 0x82435228;
	sub_824369C8(ctx, base);
	// 82435228: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243522C: 386B51F4  addi r3, r11, 0x51f4
	ctx.r[3].s64 = ctx.r[11].s64 + 20980;
	// 82435230: 48001E99  bl 0x824370c8
	ctx.lr = 0x82435234;
	sub_824370C8(ctx, base);
	// 82435234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243523C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82435248 size=316
    let mut pc: u32 = 0x82435248;
    'dispatch: loop {
        match pc {
            0x82435248 => {
    //   block [0x82435248..0x82435384)
	// 82435248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243524C: 480FFE71  bl 0x825350bc
	ctx.lr = 0x82435250;
	sub_82535080(ctx, base);
	// 82435250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435254: 83E30048  lwz r31, 0x48(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82435258: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243525C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435260: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82435264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435268: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8243526C: 4800768D  bl 0x8243c8f8
	ctx.lr = 0x82435270;
	sub_8243C8F8(ctx, base);
	// 82435270: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82435274: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82435278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243527C: 4800767D  bl 0x8243c8f8
	ctx.lr = 0x82435280;
	sub_8243C8F8(ctx, base);
	// 82435280: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435284: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82435288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243528C: 4800766D  bl 0x8243c8f8
	ctx.lr = 0x82435290;
	sub_8243C8F8(ctx, base);
	// 82435290: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82435294: 38800017  li r4, 0x17
	ctx.r[4].s64 = 23;
	// 82435298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243529C: 4800765D  bl 0x8243c8f8
	ctx.lr = 0x824352A0;
	sub_8243C8F8(ctx, base);
	// 824352A0: 7D7EE9D6  mullw r11, r30, r29
	ctx.r[11].s64 = (ctx.r[30].s32 as i64) * (ctx.r[29].s32 as i64);
	// 824352A4: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 824352A8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824352AC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824352B0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824352B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824352B8: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824352BC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824352C0: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824352C4: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824352C8: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 824352CC: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 824352D0: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 824352D4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824352D8: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 824352DC: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 824352E0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 824352E4: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824352E8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824352EC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824352F0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824352F4: 41990008  bgt cr6, 0x824352fc
	if ctx.cr[6].gt {
	pc = 0x824352FC; continue 'dispatch;
	}
	// 824352F8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824352FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82435300: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 82435304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435308: 480075F1  bl 0x8243c8f8
	ctx.lr = 0x8243530C;
	sub_8243C8F8(ctx, base);
	// 8243530C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82435310: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82435314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435318: 480075E1  bl 0x8243c8f8
	ctx.lr = 0x8243531C;
	sub_8243C8F8(ctx, base);
	// 8243531C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82435320: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 82435324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435328: 480075D1  bl 0x8243c8f8
	ctx.lr = 0x8243532C;
	sub_8243C8F8(ctx, base);
	// 8243532C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82435330: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82435334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435338: 480075C1  bl 0x8243c8f8
	ctx.lr = 0x8243533C;
	sub_8243C8F8(ctx, base);
	// 8243533C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435340: 38800033  li r4, 0x33
	ctx.r[4].s64 = 51;
	// 82435344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435348: 480075B1  bl 0x8243c8f8
	ctx.lr = 0x8243534C;
	sub_8243C8F8(ctx, base);
	// 8243534C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435350: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82435354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435358: 480075A1  bl 0x8243c8f8
	ctx.lr = 0x8243535C;
	sub_8243C8F8(ctx, base);
	// 8243535C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435360: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82435364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435368: 48007591  bl 0x8243c8f8
	ctx.lr = 0x8243536C;
	sub_8243C8F8(ctx, base);
	// 8243536C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435370: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82435374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435378: 4800AC61  bl 0x8243ffd8
	ctx.lr = 0x8243537C;
	sub_8243FFD8(ctx, base);
	// 8243537C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435380: 480FFD8C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435388 size=168
    let mut pc: u32 = 0x82435388;
    'dispatch: loop {
        match pc {
            0x82435388 => {
    //   block [0x82435388..0x82435430)
	// 82435388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82435394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243539C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824353A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824353A4: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824353A8: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 824353AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824353B0: 48005EA9  bl 0x8243b258
	ctx.lr = 0x824353B4;
	sub_8243B258(ctx, base);
	// 824353B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824353B8: 419A0018  beq cr6, 0x824353d0
	if ctx.cr[6].eq {
	pc = 0x824353D0; continue 'dispatch;
	}
	// 824353BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824353C0: 386B5248  addi r3, r11, 0x5248
	ctx.r[3].s64 = ctx.r[11].s64 + 21064;
	// 824353C4: 48001D05  bl 0x824370c8
	ctx.lr = 0x824353C8;
	sub_824370C8(ctx, base);
	// 824353C8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824353CC: 4800004C  b 0x82435418
	pc = 0x82435418; continue 'dispatch;
	// 824353D0: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 824353D4: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 824353D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824353DC: 388B69F8  addi r4, r11, 0x69f8
	ctx.r[4].s64 = ctx.r[11].s64 + 27128;
	// 824353E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824353E4: 480125AD  bl 0x82447990
	ctx.lr = 0x824353E8;
	sub_82447990(ctx, base);
	// 824353E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824353EC: 419A0020  beq cr6, 0x8243540c
	if ctx.cr[6].eq {
	pc = 0x8243540C; continue 'dispatch;
	}
	// 824353F0: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 824353F4: 480015D5  bl 0x824369c8
	ctx.lr = 0x824353F8;
	sub_824369C8(ctx, base);
	// 824353F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824353FC: 386B5214  addi r3, r11, 0x5214
	ctx.r[3].s64 = ctx.r[11].s64 + 21012;
	// 82435400: 48001CC9  bl 0x824370c8
	ctx.lr = 0x82435404;
	sub_824370C8(ctx, base);
	// 82435404: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82435408: 48000010  b 0x82435418
	pc = 0x82435418; continue 'dispatch;
	// 8243540C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435410: 4BFFFD69  bl 0x82435178
	ctx.lr = 0x82435414;
	sub_82435178(ctx, base);
	// 82435414: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243541C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82435428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243542C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435430 size=16
    let mut pc: u32 = 0x82435430;
    'dispatch: loop {
        match pc {
            0x82435430 => {
    //   block [0x82435430..0x82435440)
	// 82435430: 80A30460  lwz r5, 0x460(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1120 as u32) ) } as u64;
	// 82435434: 8083045C  lwz r4, 0x45c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1116 as u32) ) } as u64;
	// 82435438: 80630458  lwz r3, 0x458(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1112 as u32) ) } as u64;
	// 8243543C: 4BFEE00C  b 0x82423448
	sub_82423448(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435440 size=96
    let mut pc: u32 = 0x82435440;
    'dispatch: loop {
        match pc {
            0x82435440 => {
    //   block [0x82435440..0x824354A0)
	// 82435440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435448: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243544C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435450: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82435454: 48001335  bl 0x82436788
	ctx.lr = 0x82435458;
	sub_82436788(ctx, base);
	// 82435458: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243545C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82435460: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435464: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82435468: 409A0024  bne cr6, 0x8243548c
	if !ctx.cr[6].eq {
	pc = 0x8243548C; continue 'dispatch;
	}
	// 8243546C: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435470: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82435474: 409A0008  bne cr6, 0x8243547c
	if !ctx.cr[6].eq {
	pc = 0x8243547C; continue 'dispatch;
	}
	// 82435478: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8243547C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82435480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435484: 409A0008  bne cr6, 0x8243548c
	if !ctx.cr[6].eq {
	pc = 0x8243548C; continue 'dispatch;
	}
	// 82435488: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8243548C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243549C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824354A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824354A0 size=64
    let mut pc: u32 = 0x824354A0;
    'dispatch: loop {
        match pc {
            0x824354A0 => {
    //   block [0x824354A0..0x824354E0)
	// 824354A0: 81040018  lwz r8, 0x18(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 824354A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824354A8: 39630498  addi r11, r3, 0x498
	ctx.r[11].s64 = ctx.r[3].s64 + 1176;
	// 824354AC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824354B0: 91030480  stw r8, 0x480(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1152 as u32), ctx.r[8].u32 ) };
	// 824354B4: 8104001C  lwz r8, 0x1c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 824354B8: 91030484  stw r8, 0x484(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1156 as u32), ctx.r[8].u32 ) };
	// 824354BC: 81040018  lwz r8, 0x18(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 824354C0: 9123048C  stw r9, 0x48c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1164 as u32), ctx.r[9].u32 ) };
	// 824354C4: 91030488  stw r8, 0x488(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1160 as u32), ctx.r[8].u32 ) };
	// 824354C8: 91230494  stw r9, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[9].u32 ) };
	// 824354CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824354D0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824354D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824354D8: 4200FFF8  bdnz 0x824354d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824354D0; continue 'dispatch;
	}
	// 824354DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824354E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824354E0 size=108
    let mut pc: u32 = 0x824354E0;
    'dispatch: loop {
        match pc {
            0x824354E0 => {
    //   block [0x824354E0..0x8243554C)
	// 824354E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824354E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824354E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824354EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824354F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824354F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824354F8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824354FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82435500: 419A001C  beq cr6, 0x8243551c
	if ctx.cr[6].eq {
	pc = 0x8243551C; continue 'dispatch;
	}
	// 82435504: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82435508: 419A0014  beq cr6, 0x8243551c
	if ctx.cr[6].eq {
	pc = 0x8243551C; continue 'dispatch;
	}
	// 8243550C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435510: 386B52A4  addi r3, r11, 0x52a4
	ctx.r[3].s64 = ctx.r[11].s64 + 21156;
	// 82435514: 48001BB5  bl 0x824370c8
	ctx.lr = 0x82435518;
	sub_824370C8(ctx, base);
	// 82435518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243551C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82435520: 2F0B000E  cmpwi cr6, r11, 0xe
	ctx.cr[6].compare_i32(ctx.r[11].s32, 14, &mut ctx.xer);
	// 82435524: 40990014  ble cr6, 0x82435538
	if !ctx.cr[6].gt {
	pc = 0x82435538; continue 'dispatch;
	}
	// 82435528: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243552C: 386B5278  addi r3, r11, 0x5278
	ctx.r[3].s64 = ctx.r[11].s64 + 21112;
	// 82435530: 48001B99  bl 0x824370c8
	ctx.lr = 0x82435534;
	sub_824370C8(ctx, base);
	// 82435534: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243553C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435550 size=32
    let mut pc: u32 = 0x82435550;
    'dispatch: loop {
        match pc {
            0x82435550 => {
    //   block [0x82435550..0x82435570)
	// 82435550: 39630480  addi r11, r3, 0x480
	ctx.r[11].s64 = ctx.r[3].s64 + 1152;
	// 82435554: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435558: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243555C: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82435560: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82435564: 4099000C  ble cr6, 0x82435570
	if !ctx.cr[6].gt {
		sub_82435570(ctx, base);
		return;
	}
	// 82435568: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243556C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435570 size=20
    let mut pc: u32 = 0x82435570;
    'dispatch: loop {
        match pc {
            0x82435570 => {
    //   block [0x82435570..0x82435584)
	// 82435570: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435574: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82435578: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 8243557C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82435580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435588 size=68
    let mut pc: u32 = 0x82435588;
    'dispatch: loop {
        match pc {
            0x82435588 => {
    //   block [0x82435588..0x824355CC)
	// 82435588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243558C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243559C: 480011ED  bl 0x82436788
	ctx.lr = 0x824355A0;
	sub_82436788(ctx, base);
	// 824355A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824355A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824355A8: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 824355AC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824355B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824355B4: 4E800421  bctrl
	ctx.lr = 0x824355B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824355B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824355BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824355C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824355C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824355C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824355D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824355D0 size=68
    let mut pc: u32 = 0x824355D0;
    'dispatch: loop {
        match pc {
            0x824355D0 => {
    //   block [0x824355D0..0x82435614)
	// 824355D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824355D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824355D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824355DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824355E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824355E4: 480011A5  bl 0x82436788
	ctx.lr = 0x824355E8;
	sub_82436788(ctx, base);
	// 824355E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824355EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824355F0: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 824355F4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824355F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824355FC: 4E800421  bctrl
	ctx.lr = 0x82435600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82435600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243560C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435618 size=16
    let mut pc: u32 = 0x82435618;
    'dispatch: loop {
        match pc {
            0x82435618 => {
    //   block [0x82435618..0x82435628)
	// 82435618: 81630494  lwz r11, 0x494(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8243561C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82435620: 91630494  stw r11, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[11].u32 ) };
	// 82435624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435628 size=16
    let mut pc: u32 = 0x82435628;
    'dispatch: loop {
        match pc {
            0x82435628 => {
    //   block [0x82435628..0x82435638)
	// 82435628: 81630494  lwz r11, 0x494(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8243562C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82435630: 91630494  stw r11, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[11].u32 ) };
	// 82435634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435638 size=8
    let mut pc: u32 = 0x82435638;
    'dispatch: loop {
        match pc {
            0x82435638 => {
    //   block [0x82435638..0x82435640)
	// 82435638: 80630494  lwz r3, 0x494(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8243563C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435640 size=144
    let mut pc: u32 = 0x82435640;
    'dispatch: loop {
        match pc {
            0x82435640 => {
    //   block [0x82435640..0x824356D0)
	// 82435640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435644: 480FFA71  bl 0x825350b4
	ctx.lr = 0x82435648;
	sub_82535080(ctx, base);
	// 82435648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243564C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82435650: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82435654: 3BAB9B78  addi r29, r11, -0x6488
	ctx.r[29].s64 = ctx.r[11].s64 + -25736;
	// 82435658: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8243565C: 3BDDFFD8  addi r30, r29, -0x28
	ctx.r[30].s64 = ctx.r[29].s64 + -40;
	// 82435660: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82435664: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82435668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243566C: 4BFF77ED  bl 0x8242ce58
	ctx.lr = 0x82435670;
	sub_8242CE58(ctx, base);
	// 82435670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82435674: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82435678: 419A0028  beq cr6, 0x824356a0
	if ctx.cr[6].eq {
	pc = 0x824356A0; continue 'dispatch;
	}
	// 8243567C: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82435680: 397D0100  addi r11, r29, 0x100
	ctx.r[11].s64 = ctx.r[29].s64 + 256;
	// 82435684: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82435688: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8243568C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82435690: 4198FFD4  blt cr6, 0x82435664
	if ctx.cr[6].lt {
	pc = 0x82435664; continue 'dispatch;
	}
	// 82435694: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82435698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243569C: 480FFA68  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824356A0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824356A4: 40990020  ble cr6, 0x824356c4
	if !ctx.cr[6].gt {
	pc = 0x824356C4; continue 'dispatch;
	}
	// 824356A8: 3BFDFFD8  addi r31, r29, -0x28
	ctx.r[31].s64 = ctx.r[29].s64 + -40;
	// 824356AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824356B0: 4BFF7889  bl 0x8242cf38
	ctx.lr = 0x824356B4;
	sub_8242CF38(ctx, base);
	// 824356B4: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 824356B8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824356BC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824356C0: 409AFFEC  bne cr6, 0x824356ac
	if !ctx.cr[6].eq {
	pc = 0x824356AC; continue 'dispatch;
	}
	// 824356C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824356C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824356CC: 480FFA38  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824356D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824356D0 size=16
    let mut pc: u32 = 0x824356D0;
    'dispatch: loop {
        match pc {
            0x824356D0 => {
    //   block [0x824356D0..0x824356E0)
	// 824356D0: 816305B0  lwz r11, 0x5b0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1456 as u32) ) } as u64;
	// 824356D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824356D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824356DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824356E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824356E0 size=8
    let mut pc: u32 = 0x824356E0;
    'dispatch: loop {
        match pc {
            0x824356E0 => {
    //   block [0x824356E0..0x824356E8)
	// 824356E0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824356E4: 4BFF78E4  b 0x8242cfc8
	sub_8242CFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824356E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824356E8 size=4
    let mut pc: u32 = 0x824356E8;
    'dispatch: loop {
        match pc {
            0x824356E8 => {
    //   block [0x824356E8..0x824356EC)
	// 824356E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824356F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824356F0 size=16
    let mut pc: u32 = 0x824356F0;
    'dispatch: loop {
        match pc {
            0x824356F0 => {
    //   block [0x824356F0..0x82435700)
	// 824356F0: 816305B0  lwz r11, 0x5b0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1456 as u32) ) } as u64;
	// 824356F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824356F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824356FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435700 size=8
    let mut pc: u32 = 0x82435700;
    'dispatch: loop {
        match pc {
            0x82435700 => {
    //   block [0x82435700..0x82435708)
	// 82435700: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82435704: 4BFF795C  b 0x8242d060
	sub_8242D060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435708 size=4
    let mut pc: u32 = 0x82435708;
    'dispatch: loop {
        match pc {
            0x82435708 => {
    //   block [0x82435708..0x8243570C)
	// 82435708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435710 size=208
    let mut pc: u32 = 0x82435710;
    'dispatch: loop {
        match pc {
            0x82435710 => {
    //   block [0x82435710..0x824357E0)
	// 82435710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435714: 480FF9A1  bl 0x825350b4
	ctx.lr = 0x82435718;
	sub_82535080(ctx, base);
	// 82435718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243571C: 83E30028  lwz r31, 0x28(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435720: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82435724: 83A3002C  lwz r29, 0x2c(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82435728: 83C30030  lwz r30, 0x30(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8243572C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82435730: 83830008  lwz r28, 8(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435734: 8363000C  lwz r27, 0xc(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435738: 41990020  bgt cr6, 0x82435758
	if ctx.cr[6].gt {
	pc = 0x82435758; continue 'dispatch;
	}
	// 8243573C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82435740: 41990018  bgt cr6, 0x82435758
	if ctx.cr[6].gt {
	pc = 0x82435758; continue 'dispatch;
	}
	// 82435744: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82435748: 409A0010  bne cr6, 0x82435758
	if !ctx.cr[6].eq {
	pc = 0x82435758; continue 'dispatch;
	}
	// 8243574C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435754: 480FF9B0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82435758: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 8243575C: 4199001C  bgt cr6, 0x82435778
	if ctx.cr[6].gt {
	pc = 0x82435778; continue 'dispatch;
	}
	// 82435760: 2F1F0010  cmpwi cr6, r31, 0x10
	ctx.cr[6].compare_i32(ctx.r[31].s32, 16, &mut ctx.xer);
	// 82435764: 40990014  ble cr6, 0x82435778
	if !ctx.cr[6].gt {
	pc = 0x82435778; continue 'dispatch;
	}
	// 82435768: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243576C: 386B5310  addi r3, r11, 0x5310
	ctx.r[3].s64 = ctx.r[11].s64 + 21264;
	// 82435770: 48001959  bl 0x824370c8
	ctx.lr = 0x82435774;
	sub_824370C8(ctx, base);
	// 82435774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82435778: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8243577C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435780: 4BFFF849  bl 0x82434fc8
	ctx.lr = 0x82435784;
	sub_82434FC8(ctx, base);
	// 82435784: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82435788: 40980014  bge cr6, 0x8243579c
	if !ctx.cr[6].lt {
	pc = 0x8243579C; continue 'dispatch;
	}
	// 8243578C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435790: 386B52EC  addi r3, r11, 0x52ec
	ctx.r[3].s64 = ctx.r[11].s64 + 21228;
	// 82435794: 48001935  bl 0x824370c8
	ctx.lr = 0x82435798;
	sub_824370C8(ctx, base);
	// 82435798: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8243579C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824357A0: 40990034  ble cr6, 0x824357d4
	if !ctx.cr[6].gt {
	pc = 0x824357D4; continue 'dispatch;
	}
	// 824357A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824357A8: 3BAB52C8  addi r29, r11, 0x52c8
	ctx.r[29].s64 = ctx.r[11].s64 + 21192;
	// 824357AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824357B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824357B4: 409A0010  bne cr6, 0x824357c4
	if !ctx.cr[6].eq {
	pc = 0x824357C4; continue 'dispatch;
	}
	// 824357B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824357BC: 4800190D  bl 0x824370c8
	ctx.lr = 0x824357C0;
	sub_824370C8(ctx, base);
	// 824357C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824357C4: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 824357C8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824357CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824357D0: 409AFFDC  bne cr6, 0x824357ac
	if !ctx.cr[6].eq {
	pc = 0x824357AC; continue 'dispatch;
	}
	// 824357D4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 824357D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824357DC: 480FF928  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824357E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824357E0 size=168
    let mut pc: u32 = 0x824357E0;
    'dispatch: loop {
        match pc {
            0x824357E0 => {
    //   block [0x824357E0..0x82435888)
	// 824357E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824357E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824357E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824357EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824357F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824357F4: 817F0494  lwz r11, 0x494(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1172 as u32) ) } as u64;
	// 824357F8: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 824357FC: 41980028  blt cr6, 0x82435824
	if ctx.cr[6].lt {
	pc = 0x82435824; continue 'dispatch;
	}
	// 82435800: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435804: 386B5334  addi r3, r11, 0x5334
	ctx.r[3].s64 = ctx.r[11].s64 + 21300;
	// 82435808: 480018C1  bl 0x824370c8
	ctx.lr = 0x8243580C;
	sub_824370C8(ctx, base);
	// 8243580C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243581C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435820: 4E800020  blr
	return;
	// 82435824: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82435828: 4198FFE4  blt cr6, 0x8243580c
	if ctx.cr[6].lt {
	pc = 0x8243580C; continue 'dispatch;
	}
	// 8243582C: 817F0480  lwz r11, 0x480(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1152 as u32) ) } as u64;
	// 82435830: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435834: 419A0010  beq cr6, 0x82435844
	if ctx.cr[6].eq {
	pc = 0x82435844; continue 'dispatch;
	}
	// 82435838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243583C: 4BFFFD15  bl 0x82435550
	ctx.lr = 0x82435840;
	sub_82435550(ctx, base);
	// 82435840: 4800000C  b 0x8243584c
	pc = 0x8243584C; continue 'dispatch;
	// 82435844: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82435848: 4BFFFD41  bl 0x82435588
	ctx.lr = 0x8243584C;
	sub_82435588(ctx, base);
	// 8243584C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82435850: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82435854: 419A001C  beq cr6, 0x82435870
	if ctx.cr[6].eq {
	pc = 0x82435870; continue 'dispatch;
	}
	// 82435858: 817F0494  lwz r11, 0x494(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8243585C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435860: 396B0126  addi r11, r11, 0x126
	ctx.r[11].s64 = ctx.r[11].s64 + 294;
	// 82435864: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435868: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 8243586C: 4BFFFDAD  bl 0x82435618
	ctx.lr = 0x82435870;
	sub_82435618(ctx, base);
	// 82435870: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82435874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243587C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435888 size=68
    let mut pc: u32 = 0x82435888;
    'dispatch: loop {
        match pc {
            0x82435888 => {
    //   block [0x82435888..0x824358CC)
	// 82435888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243588C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243589C: 817F0480  lwz r11, 0x480(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1152 as u32) ) } as u64;
	// 824358A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824358A4: 409A000C  bne cr6, 0x824358b0
	if !ctx.cr[6].eq {
	pc = 0x824358B0; continue 'dispatch;
	}
	// 824358A8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824358AC: 4BFFFD25  bl 0x824355d0
	ctx.lr = 0x824358B0;
	sub_824355D0(ctx, base);
	// 824358B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824358B4: 4BFFFD75  bl 0x82435628
	ctx.lr = 0x824358B8;
	sub_82435628(ctx, base);
	// 824358B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824358BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824358C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824358C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824358C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824358D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824358D0 size=76
    let mut pc: u32 = 0x824358D0;
    'dispatch: loop {
        match pc {
            0x824358D0 => {
    //   block [0x824358D0..0x8243591C)
	// 824358D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824358D4: 480FF7E5  bl 0x825350b8
	ctx.lr = 0x824358D8;
	sub_82535080(ctx, base);
	// 824358D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824358DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824358E0: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 824358E4: 3BFE0514  addi r31, r30, 0x514
	ctx.r[31].s64 = ctx.r[30].s64 + 1300;
	// 824358E8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824358EC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824358F0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824358F4: 419A0010  beq cr6, 0x82435904
	if ctx.cr[6].eq {
	pc = 0x82435904; continue 'dispatch;
	}
	// 824358F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824358FC: 4BFFFF8D  bl 0x82435888
	ctx.lr = 0x82435900;
	sub_82435888(ctx, base);
	// 82435900: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82435904: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82435908: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 8243590C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82435910: 409AFFDC  bne cr6, 0x824358ec
	if !ctx.cr[6].eq {
	pc = 0x824358EC; continue 'dispatch;
	}
	// 82435914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435918: 480FF7F0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82435920 size=224
    let mut pc: u32 = 0x82435920;
    'dispatch: loop {
        match pc {
            0x82435920 => {
    //   block [0x82435920..0x82435A00)
	// 82435920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435924: 480FF795  bl 0x825350b8
	ctx.lr = 0x82435928;
	sub_82535080(ctx, base);
	// 82435928: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243592C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82435930: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82435934: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82435938: 4BFFFDD9  bl 0x82435710
	ctx.lr = 0x8243593C;
	sub_82435710(ctx, base);
	// 8243593C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82435940: 409A0018  bne cr6, 0x82435958
	if !ctx.cr[6].eq {
	pc = 0x82435958; continue 'dispatch;
	}
	// 82435944: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82435948: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243594C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82435950: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435954: 480FF7B4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82435958: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8243595C: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82435960: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82435964: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82435968: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243596C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435970: 4BFFF511  bl 0x82434e80
	ctx.lr = 0x82435974;
	sub_82434E80(ctx, base);
	// 82435974: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82435978: 4BFFF549  bl 0x82434ec0
	ctx.lr = 0x8243597C;
	sub_82434EC0(ctx, base);
	// 8243597C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82435980: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82435984: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82435988: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 8243598C: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82435990: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82435994: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82435998: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243599C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 824359A0: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 824359A4: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824359A8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824359AC: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 824359B0: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 824359B4: 7D293E70  srawi r9, r9, 7
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 7) as i64;
	// 824359B8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824359BC: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 824359C0: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 824359C4: 7D673E70  srawi r7, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 824359C8: 7D6941D6  mullw r11, r9, r8
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 824359CC: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824359D0: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 824359D4: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824359D8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824359DC: 556A402E  slwi r10, r11, 8
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824359E0: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824359E4: 394A0080  addi r10, r10, 0x80
	ctx.r[10].s64 = ctx.r[10].s64 + 128;
	// 824359E8: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 824359EC: 7D4AF1D6  mullw r10, r10, r30
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[30].s32 as i64);
	// 824359F0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824359F4: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824359F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824359FC: 480FF70C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435A00 size=180
    let mut pc: u32 = 0x82435A00;
    'dispatch: loop {
        match pc {
            0x82435A00 => {
    //   block [0x82435A00..0x82435AB4)
	// 82435A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435A08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435A0C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435A10: 3921006C  addi r9, r1, 0x6c
	ctx.r[9].s64 = ctx.r[1].s64 + 108;
	// 82435A14: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 82435A18: 38E10064  addi r7, r1, 0x64
	ctx.r[7].s64 = ctx.r[1].s64 + 100;
	// 82435A1C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82435A20: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82435A24: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82435A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82435A2C: 4BFFF33D  bl 0x82434d68
	ctx.lr = 0x82435A30;
	sub_82434D68(ctx, base);
	// 82435A30: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82435A34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82435A38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435A3C: 4BFFFEE5  bl 0x82435920
	ctx.lr = 0x82435A40;
	sub_82435920(ctx, base);
	// 82435A40: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82435A44: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 82435A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435A4C: 4BFFF4FD  bl 0x82434f48
	ctx.lr = 0x82435A50;
	sub_82434F48(ctx, base);
	// 82435A50: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82435A54: 4BFFF515  bl 0x82434f68
	ctx.lr = 0x82435A58;
	sub_82434F68(ctx, base);
	// 82435A58: 7D6A1A14  add r11, r10, r3
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82435A5C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82435A60: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A64: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82435A68: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A6C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82435A70: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A74: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82435A78: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A7C: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82435A80: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A84: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82435A88: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A8C: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82435A90: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A94: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82435A98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A9C: 386B08C0  addi r3, r11, 0x8c0
	ctx.r[3].s64 = ctx.r[11].s64 + 2240;
	// 82435AA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435AB8 size=100
    let mut pc: u32 = 0x82435AB8;
    'dispatch: loop {
        match pc {
            0x82435AB8 => {
    //   block [0x82435AB8..0x82435B1C)
	// 82435AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435AC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82435AC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435ACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82435AD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82435AD4: 409A0018  bne cr6, 0x82435aec
	if !ctx.cr[6].eq {
	pc = 0x82435AEC; continue 'dispatch;
	}
	// 82435AD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435ADC: 386B5358  addi r3, r11, 0x5358
	ctx.r[3].s64 = ctx.r[11].s64 + 21336;
	// 82435AE0: 480015E9  bl 0x824370c8
	ctx.lr = 0x82435AE4;
	sub_824370C8(ctx, base);
	// 82435AE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435AE8: 4800001C  b 0x82435b04
	pc = 0x82435B04; continue 'dispatch;
	// 82435AEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435AF0: 4BFFFF11  bl 0x82435a00
	ctx.lr = 0x82435AF4;
	sub_82435A00(ctx, base);
	// 82435AF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82435AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435AFC: 4BFFF475  bl 0x82434f70
	ctx.lr = 0x82435B00;
	sub_82434F70(ctx, base);
	// 82435B00: 7C63F214  add r3, r3, r30
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82435B04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82435B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435B10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82435B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82435B20 size=292
    let mut pc: u32 = 0x82435B20;
    'dispatch: loop {
        match pc {
            0x82435B20 => {
    //   block [0x82435B20..0x82435C44)
	// 82435B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435B24: 480FF595  bl 0x825350b8
	ctx.lr = 0x82435B28;
	sub_82535080(ctx, base);
	// 82435B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435B2C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435B30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82435B34: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435B38: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82435B3C: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82435B40: 81240028  lwz r9, 0x28(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435B44: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 82435B48: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82435B4C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82435B50: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82435B54: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82435B58: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435B5C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82435B60: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82435B64: 55472036  slwi r7, r10, 4
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82435B68: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435B6C: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82435B70: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 82435B74: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82435B78: 7D083E70  srawi r8, r8, 7
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 7) as i64;
	// 82435B7C: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435B80: 7CE70E70  srawi r7, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 82435B84: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 82435B88: 7D663E70  srawi r6, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 82435B8C: 7D6839D6  mullw r11, r8, r7
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82435B90: 7D060194  addze r8, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435B94: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82435B98: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82435B9C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435BA0: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435BA4: 3BCB0080  addi r30, r11, 0x80
	ctx.r[30].s64 = ctx.r[11].s64 + 128;
	// 82435BA8: 419A0048  beq cr6, 0x82435bf0
	if ctx.cr[6].eq {
	pc = 0x82435BF0; continue 'dispatch;
	}
	// 82435BAC: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 82435BB0: 4198002C  blt cr6, 0x82435bdc
	if ctx.cr[6].lt {
	pc = 0x82435BDC; continue 'dispatch;
	}
	// 82435BB4: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82435BB8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82435BBC: 41980020  blt cr6, 0x82435bdc
	if ctx.cr[6].lt {
	pc = 0x82435BDC; continue 'dispatch;
	}
	// 82435BC0: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82435BC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82435BC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82435BCC: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82435BD0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82435BD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82435BD8: 4800003C  b 0x82435c14
	pc = 0x82435C14; continue 'dispatch;
	// 82435BDC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82435BE0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82435BE4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82435BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435BEC: 480FF51C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82435BF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82435BF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435BF8: 4BFFFBE9  bl 0x824357e0
	ctx.lr = 0x82435BFC;
	sub_824357E0(ctx, base);
	// 82435BFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82435C00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82435C04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435C08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82435C0C: 4BFFFBD5  bl 0x824357e0
	ctx.lr = 0x82435C10;
	sub_824357E0(ctx, base);
	// 82435C10: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82435C14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82435C18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435C1C: 419A0010  beq cr6, 0x82435c2c
	if ctx.cr[6].eq {
	pc = 0x82435C2C; continue 'dispatch;
	}
	// 82435C20: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82435C24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435C28: 409A0010  bne cr6, 0x82435c38
	if !ctx.cr[6].eq {
	pc = 0x82435C38; continue 'dispatch;
	}
	// 82435C2C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82435C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435C34: 480FF4D4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82435C38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435C40: 480FF4C8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82435C48 size=328
    let mut pc: u32 = 0x82435C48;
    'dispatch: loop {
        match pc {
            0x82435C48 => {
    //   block [0x82435C48..0x82435D90)
	// 82435C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435C4C: 480FF461  bl 0x825350ac
	ctx.lr = 0x82435C50;
	sub_82535080(ctx, base);
	// 82435C50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435C54: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82435C58: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82435C5C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82435C60: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82435C64: 839F0010  lwz r28, 0x10(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82435C68: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435C6C: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435C70: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82435C74: 4BFFF24D  bl 0x82434ec0
	ctx.lr = 0x82435C78;
	sub_82434EC0(ctx, base);
	// 82435C78: 397E000F  addi r11, r30, 0xf
	ctx.r[11].s64 = ctx.r[30].s64 + 15;
	// 82435C7C: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435C80: 395D000F  addi r10, r29, 0xf
	ctx.r[10].s64 = ctx.r[29].s64 + 15;
	// 82435C84: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82435C88: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82435C8C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82435C90: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82435C94: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435C98: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82435C9C: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82435CA0: 55472036  slwi r7, r10, 4
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82435CA4: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435CA8: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82435CAC: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 82435CB0: 7D083E70  srawi r8, r8, 7
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 7) as i64;
	// 82435CB4: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435CB8: 7CE70E70  srawi r7, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 82435CBC: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 82435CC0: 7D663E70  srawi r6, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 82435CC4: 7D6839D6  mullw r11, r8, r7
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82435CC8: 7D060194  addze r8, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435CCC: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82435CD0: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82435CD4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435CD8: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435CDC: 3BCB0080  addi r30, r11, 0x80
	ctx.r[30].s64 = ctx.r[11].s64 + 128;
	// 82435CE0: 419A0070  beq cr6, 0x82435d50
	if ctx.cr[6].eq {
	pc = 0x82435D50; continue 'dispatch;
	}
	// 82435CE4: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82435CE8: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82435CEC: 41980058  blt cr6, 0x82435d44
	if ctx.cr[6].lt {
	pc = 0x82435D44; continue 'dispatch;
	}
	// 82435CF0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82435CF4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82435CF8: 4198004C  blt cr6, 0x82435d44
	if ctx.cr[6].lt {
	pc = 0x82435D44; continue 'dispatch;
	}
	// 82435CFC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82435D00: 40990084  ble cr6, 0x82435d84
	if !ctx.cr[6].gt {
	pc = 0x82435D84; continue 'dispatch;
	}
	// 82435D04: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82435D08: 213B0008  subfic r9, r27, 8
	ctx.xer.ca = ctx.r[27].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[27].s64;
	// 82435D0C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82435D10: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82435D14: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82435D18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435D1C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82435D20: 409A0008  bne cr6, 0x82435d28
	if !ctx.cr[6].eq {
	pc = 0x82435D28; continue 'dispatch;
	}
	// 82435D24: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82435D28: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82435D2C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82435D30: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82435D34: 409AFFD8  bne cr6, 0x82435d0c
	if !ctx.cr[6].eq {
	pc = 0x82435D0C; continue 'dispatch;
	}
	// 82435D38: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82435D3C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435D40: 480FF3BC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 82435D44: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82435D48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435D4C: 480FF3B0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 82435D50: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82435D54: 40990030  ble cr6, 0x82435d84
	if !ctx.cr[6].gt {
	pc = 0x82435D84; continue 'dispatch;
	}
	// 82435D58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82435D5C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82435D60: 4BFFFA81  bl 0x824357e0
	ctx.lr = 0x82435D64;
	sub_824357E0(ctx, base);
	// 82435D64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82435D68: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82435D6C: 409A0008  bne cr6, 0x82435d74
	if !ctx.cr[6].eq {
	pc = 0x82435D74; continue 'dispatch;
	}
	// 82435D70: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82435D74: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82435D78: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82435D7C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82435D80: 409AFFD8  bne cr6, 0x82435d58
	if !ctx.cr[6].eq {
	pc = 0x82435D58; continue 'dispatch;
	}
	// 82435D84: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82435D88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435D8C: 480FF370  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435D90 size=1008
    let mut pc: u32 = 0x82435D90;
    'dispatch: loop {
        match pc {
            0x82435D90 => {
    //   block [0x82435D90..0x82436180)
	// 82435D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435D94: 480FF2ED  bl 0x82535080
	ctx.lr = 0x82435D98;
	sub_82535080(ctx, base);
	// 82435D98: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435D9C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82435DA0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82435DA4: 3BEB9B48  addi r31, r11, -0x64b8
	ctx.r[31].s64 = ctx.r[11].s64 + -25784;
	// 82435DA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82435DAC: 393F0140  addi r9, r31, 0x140
	ctx.r[9].s64 = ctx.r[31].s64 + 320;
	// 82435DB0: 391F013C  addi r8, r31, 0x13c
	ctx.r[8].s64 = ctx.r[31].s64 + 316;
	// 82435DB4: 38FF0004  addi r7, r31, 4
	ctx.r[7].s64 = ctx.r[31].s64 + 4;
	// 82435DB8: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82435DBC: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 82435DC0: 829D0010  lwz r20, 0x10(r29)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82435DC4: 38BF0134  addi r5, r31, 0x134
	ctx.r[5].s64 = ctx.r[31].s64 + 308;
	// 82435DC8: 821D0008  lwz r16, 8(r29)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435DCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82435DD0: 81FD000C  lwz r15, 0xc(r29)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435DD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435DD8: 4BFFEF91  bl 0x82434d68
	ctx.lr = 0x82435DDC;
	sub_82434D68(ctx, base);
	// 82435DDC: 38BF002C  addi r5, r31, 0x2c
	ctx.r[5].s64 = ctx.r[31].s64 + 44;
	// 82435DE0: 389F0138  addi r4, r31, 0x138
	ctx.r[4].s64 = ctx.r[31].s64 + 312;
	// 82435DE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435DE8: 4BFFFB39  bl 0x82435920
	ctx.lr = 0x82435DEC;
	sub_82435920(ctx, base);
	// 82435DEC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82435DF0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82435DF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435DF8: 4BFFF151  bl 0x82434f48
	ctx.lr = 0x82435DFC;
	sub_82434F48(ctx, base);
	// 82435DFC: 4BFFF16D  bl 0x82434f68
	ctx.lr = 0x82435E00;
	sub_82434F68(ctx, base);
	// 82435E00: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82435E04: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435E08: 7C6E1B78  mr r14, r3
	ctx.r[14].u64 = ctx.r[3].u64;
	// 82435E0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E10: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435E14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82435E18: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82435E1C: 388B0080  addi r4, r11, 0x80
	ctx.r[4].s64 = ctx.r[11].s64 + 128;
	// 82435E20: 4BFFF9C1  bl 0x824357e0
	ctx.lr = 0x82435E24;
	sub_824357E0(ctx, base);
	// 82435E24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82435E28: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82435E2C: 388B0040  addi r4, r11, 0x40
	ctx.r[4].s64 = ctx.r[11].s64 + 64;
	// 82435E30: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E34: 4BFFF9AD  bl 0x824357e0
	ctx.lr = 0x82435E38;
	sub_824357E0(ctx, base);
	// 82435E38: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82435E3C: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82435E40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82435E44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E48: 4BFFFCD9  bl 0x82435b20
	ctx.lr = 0x82435E4C;
	sub_82435B20(ctx, base);
	// 82435E4C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82435E50: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82435E54: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82435E58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E5C: 4BFFFDED  bl 0x82435c48
	ctx.lr = 0x82435E60;
	sub_82435C48(ctx, base);
	// 82435E60: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82435E64: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82435E68: 4BFFF2A9  bl 0x82435110
	ctx.lr = 0x82435E6C;
	sub_82435110(ctx, base);
	// 82435E6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82435E70: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82435E74: 409A0028  bne cr6, 0x82435e9c
	if !ctx.cr[6].eq {
	pc = 0x82435E9C; continue 'dispatch;
	}
	// 82435E78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E7C: 809F013C  lwz r4, 0x13c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 82435E80: 4BFFF961  bl 0x824357e0
	ctx.lr = 0x82435E84;
	sub_824357E0(ctx, base);
	// 82435E84: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82435E88: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E8C: 809F0140  lwz r4, 0x140(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 82435E90: 4BFFF951  bl 0x824357e0
	ctx.lr = 0x82435E94;
	sub_824357E0(ctx, base);
	// 82435E94: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82435E98: 4800000C  b 0x82435ea4
	pc = 0x82435EA4; continue 'dispatch;
	// 82435E9C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82435EA0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82435EA4: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 82435EA8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435EAC: 4BFFF935  bl 0x824357e0
	ctx.lr = 0x82435EB0;
	sub_824357E0(ctx, base);
	// 82435EB0: 82210054  lwz r17, 0x54(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82435EB4: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82435EB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435EBC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82435EC0: 4BFFF921  bl 0x824357e0
	ctx.lr = 0x82435EC4;
	sub_824357E0(ctx, base);
	// 82435EC4: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82435EC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435ECC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82435ED0: 4BFFF911  bl 0x824357e0
	ctx.lr = 0x82435ED4;
	sub_824357E0(ctx, base);
	// 82435ED4: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82435ED8: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 82435EDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435EE0: 4BFFF901  bl 0x824357e0
	ctx.lr = 0x82435EE4;
	sub_824357E0(ctx, base);
	// 82435EE4: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82435EE8: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 82435EEC: 419A0274  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435EF0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82435EF4: 419A026C  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435EF8: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82435EFC: 409A0264  bne cr6, 0x82436160
	if !ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F00: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82435F04: 409A025C  bne cr6, 0x82436160
	if !ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F08: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 82435F0C: 419A0254  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F10: 82C10058  lwz r22, 0x58(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82435F14: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82435F18: 419A0248  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82435F20: 419A0240  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F24: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 82435F28: 419A0238  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F2C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82435F30: 409A0020  bne cr6, 0x82435f50
	if !ctx.cr[6].eq {
	pc = 0x82435F50; continue 'dispatch;
	}
	// 82435F34: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82435F38: 419A000C  beq cr6, 0x82435f44
	if ctx.cr[6].eq {
	pc = 0x82435F44; continue 'dispatch;
	}
	// 82435F3C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82435F40: 409A0010  bne cr6, 0x82435f50
	if !ctx.cr[6].eq {
	pc = 0x82435F50; continue 'dispatch;
	}
	// 82435F44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435F48: 386B53F8  addi r3, r11, 0x53f8
	ctx.r[3].s64 = ctx.r[11].s64 + 21496;
	// 82435F4C: 4800021C  b 0x82436168
	pc = 0x82436168; continue 'dispatch;
	// 82435F50: 397B003F  addi r11, r27, 0x3f
	ctx.r[11].s64 = ctx.r[27].s64 + 63;
	// 82435F54: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82435F58: 557B0032  rlwinm r27, r11, 0, 0, 0x19
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82435F5C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82435F60: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82435F64: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82435F68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435F6C: 937F0130  stw r27, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[27].u32 ) };
	// 82435F70: 4BFFEF11  bl 0x82434e80
	ctx.lr = 0x82435F74;
	sub_82434E80(ctx, base);
	// 82435F74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82435F78: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82435F7C: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82435F80: 3BCB4130  addi r30, r11, 0x4130
	ctx.r[30].s64 = ctx.r[11].s64 + 16688;
	// 82435F84: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82435F88: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82435F8C: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 82435F90: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82435F94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82435F98: 935E00B4  stw r26, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[26].u32 ) };
	// 82435F9C: 835E010C  lwz r26, 0x10c(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(268 as u32) ) } as u64;
	// 82435FA0: 931E00C4  stw r24, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[24].u32 ) };
	// 82435FA4: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82435FA8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82435FAC: 917E004C  stw r11, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82435FB0: 915E0050  stw r10, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82435FB4: 913E0054  stw r9, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82435FB8: 917E0058  stw r11, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82435FBC: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82435FC0: 915E005C  stw r10, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82435FC4: 929E0060  stw r20, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[20].u32 ) };
	// 82435FC8: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82435FCC: 419A0044  beq cr6, 0x82436010
	if ctx.cr[6].eq {
	pc = 0x82436010; continue 'dispatch;
	}
	// 82435FD0: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 82435FD4: 419A0014  beq cr6, 0x82435fe8
	if ctx.cr[6].eq {
	pc = 0x82435FE8; continue 'dispatch;
	}
	// 82435FD8: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 82435FDC: 409A0058  bne cr6, 0x82436034
	if !ctx.cr[6].eq {
	pc = 0x82436034; continue 'dispatch;
	}
	// 82435FE0: 389E00C8  addi r4, r30, 0xc8
	ctx.r[4].s64 = ctx.r[30].s64 + 200;
	// 82435FE4: 48000030  b 0x82436014
	pc = 0x82436014; continue 'dispatch;
	// 82435FE8: 389E0068  addi r4, r30, 0x68
	ctx.r[4].s64 = ctx.r[30].s64 + 104;
	// 82435FEC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82435FF0: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 82435FF4: 480FEB5D  bl 0x82534b50
	ctx.lr = 0x82435FF8;
	sub_82534B50(ctx, base);
	// 82435FF8: 39400800  li r10, 0x800
	ctx.r[10].s64 = 2048;
	// 82435FFC: 937C0458  stw r27, 0x458(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1112 as u32), ctx.r[27].u32 ) };
	// 82436000: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82436004: 396BF800  addi r11, r11, -0x800
	ctx.r[11].s64 = ctx.r[11].s64 + -2048;
	// 82436008: 915C0460  stw r10, 0x460(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1120 as u32), ctx.r[10].u32 ) };
	// 8243600C: 48000024  b 0x82436030
	pc = 0x82436030; continue 'dispatch;
	// 82436010: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82436014: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 82436018: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8243601C: 480FEB35  bl 0x82534b50
	ctx.lr = 0x82436020;
	sub_82534B50(ctx, base);
	// 82436020: 937C0458  stw r27, 0x458(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1112 as u32), ctx.r[27].u32 ) };
	// 82436024: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82436028: 935C0460  stw r26, 0x460(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1120 as u32), ctx.r[26].u32 ) };
	// 8243602C: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 82436030: 917C045C  stw r11, 0x45c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1116 as u32), ctx.r[11].u32 ) };
	// 82436034: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82436038: 93410098  stw r26, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[26].u32 ) };
	// 8243603C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82436040: 419A0018  beq cr6, 0x82436058
	if ctx.cr[6].eq {
	pc = 0x82436058; continue 'dispatch;
	}
	// 82436044: 7D4BD3D6  divw r10, r11, r26
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 82436048: 7D4AD1D6  mullw r10, r10, r26
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[26].s32 as i64);
	// 8243604C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82436050: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82436054: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82436058: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243605C: 4BFFEE65  bl 0x82434ec0
	ctx.lr = 0x82436060;
	sub_82434EC0(ctx, base);
	// 82436060: 815F0134  lwz r10, 0x134(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82436064: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82436068: 387E0044  addi r3, r30, 0x44
	ctx.r[3].s64 = ctx.r[30].s64 + 68;
	// 8243606C: 92610074  stw r19, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[19].u32 ) };
	// 82436070: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82436074: 9281009C  stw r20, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[20].u32 ) };
	// 82436078: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8243607C: 920100A0  stw r16, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[16].u32 ) };
	// 82436080: 91E100A4  stw r15, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[15].u32 ) };
	// 82436084: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82436088: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243608C: 924100AC  stw r18, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[18].u32 ) };
	// 82436090: 922100B0  stw r17, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[17].u32 ) };
	// 82436094: 916100A8  stw r11, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82436098: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8243609C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824360A0: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 824360A4: 4800E55D  bl 0x82444600
	ctx.lr = 0x824360A8;
	sub_82444600(ctx, base);
	// 824360A8: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 824360AC: 409A000C  bne cr6, 0x824360b8
	if !ctx.cr[6].eq {
	pc = 0x824360B8; continue 'dispatch;
	}
	// 824360B0: 387E00AC  addi r3, r30, 0xac
	ctx.r[3].s64 = ctx.r[30].s64 + 172;
	// 824360B4: 48010125  bl 0x824461d8
	ctx.lr = 0x824360B8;
	sub_824461D8(ctx, base);
	// 824360B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824360BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824360C0: 48004EE9  bl 0x8243afa8
	ctx.lr = 0x824360C4;
	sub_8243AFA8(ctx, base);
	// 824360C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824360C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824360CC: 409A0024  bne cr6, 0x824360f0
	if !ctx.cr[6].eq {
	pc = 0x824360F0; continue 'dispatch;
	}
	// 824360D0: 3860FECF  li r3, -0x131
	ctx.r[3].s64 = -305;
	// 824360D4: 480008F5  bl 0x824369c8
	ctx.lr = 0x824360D8;
	sub_824369C8(ctx, base);
	// 824360D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824360DC: 386B53CC  addi r3, r11, 0x53cc
	ctx.r[3].s64 = ctx.r[11].s64 + 21452;
	// 824360E0: 48000FE9  bl 0x824370c8
	ctx.lr = 0x824360E4;
	sub_824370C8(ctx, base);
	// 824360E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824360E8: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 824360EC: 480FEFE4  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 824360F0: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 824360F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824360F8: 388B69F8  addi r4, r11, 0x69f8
	ctx.r[4].s64 = ctx.r[11].s64 + 27128;
	// 824360FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436100: 48011891  bl 0x82447990
	ctx.lr = 0x82436104;
	sub_82447990(ctx, base);
	// 82436104: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436108: 419A0024  beq cr6, 0x8243612c
	if ctx.cr[6].eq {
	pc = 0x8243612C; continue 'dispatch;
	}
	// 8243610C: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 82436110: 480008B9  bl 0x824369c8
	ctx.lr = 0x82436114;
	sub_824369C8(ctx, base);
	// 82436114: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436118: 386B53A4  addi r3, r11, 0x53a4
	ctx.r[3].s64 = ctx.r[11].s64 + 21412;
	// 8243611C: 48000FAD  bl 0x824370c8
	ctx.lr = 0x82436120;
	sub_824370C8(ctx, base);
	// 82436120: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436124: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82436128: 480FEFA8  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 8243612C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82436130: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82436134: 38A00800  li r5, 0x800
	ctx.r[5].s64 = 2048;
	// 82436138: 91DC043C  stw r14, 0x43c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1084 as u32), ctx.r[14].u32 ) };
	// 8243613C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82436140: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82436144: 917C0438  stw r11, 0x438(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1080 as u32), ctx.r[11].u32 ) };
	// 82436148: 4BFFF019  bl 0x82435160
	ctx.lr = 0x8243614C;
	sub_82435160(ctx, base);
	// 8243614C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436150: 92FC056C  stw r23, 0x56c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1388 as u32), ctx.r[23].u32 ) };
	// 82436154: 92FC05AC  stw r23, 0x5ac(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1452 as u32), ctx.r[23].u32 ) };
	// 82436158: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 8243615C: 480FEF74  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 82436160: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436164: 386B5388  addi r3, r11, 0x5388
	ctx.r[3].s64 = ctx.r[11].s64 + 21384;
	// 82436168: 48000F61  bl 0x824370c8
	ctx.lr = 0x8243616C;
	sub_824370C8(ctx, base);
	// 8243616C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82436170: 4BFFF761  bl 0x824358d0
	ctx.lr = 0x82436174;
	sub_824358D0(ctx, base);
	// 82436174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436178: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 8243617C: 480FEF54  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436180 size=216
    let mut pc: u32 = 0x82436180;
    'dispatch: loop {
        match pc {
            0x82436180 => {
    //   block [0x82436180..0x82436258)
	// 82436180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436184: 480FEF39  bl 0x825350bc
	ctx.lr = 0x82436188;
	sub_82535080(ctx, base);
	// 82436188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243618C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436190: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82436194: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436198: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243619C: 4BFFBA35  bl 0x82431bd0
	ctx.lr = 0x824361A0;
	sub_82431BD0(ctx, base);
	// 824361A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824361A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824361A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824361AC: 4BFFF635  bl 0x824357e0
	ctx.lr = 0x824361B0;
	sub_824357E0(ctx, base);
	// 824361B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824361B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824361B8: 409A0024  bne cr6, 0x824361dc
	if !ctx.cr[6].eq {
	pc = 0x824361DC; continue 'dispatch;
	}
	// 824361BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824361C0: 386B543C  addi r3, r11, 0x543c
	ctx.r[3].s64 = ctx.r[11].s64 + 21564;
	// 824361C4: 48000F05  bl 0x824370c8
	ctx.lr = 0x824361C8;
	sub_824370C8(ctx, base);
	// 824361C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824361CC: 4BFFF705  bl 0x824358d0
	ctx.lr = 0x824361D0;
	sub_824358D0(ctx, base);
	// 824361D0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824361D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824361D8: 480FEF34  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824361DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824361E0: 917F00BC  stw r11, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 824361E4: 93BF00C0  stw r29, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 824361E8: 4BFFBDC9  bl 0x82431fb0
	ctx.lr = 0x824361EC;
	sub_82431FB0(ctx, base);
	// 824361EC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824361F0: 409A0050  bne cr6, 0x82436240
	if !ctx.cr[6].eq {
	pc = 0x82436240; continue 'dispatch;
	}
	// 824361F4: 3C800002  lis r4, 2
	ctx.r[4].s64 = 131072;
	// 824361F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824361FC: 4BFFF5E5  bl 0x824357e0
	ctx.lr = 0x82436200;
	sub_824357E0(ctx, base);
	// 82436200: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436204: 409A0024  bne cr6, 0x82436228
	if !ctx.cr[6].eq {
	pc = 0x82436228; continue 'dispatch;
	}
	// 82436208: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243620C: 386B5414  addi r3, r11, 0x5414
	ctx.r[3].s64 = ctx.r[11].s64 + 21524;
	// 82436210: 48000EB9  bl 0x824370c8
	ctx.lr = 0x82436214;
	sub_824370C8(ctx, base);
	// 82436214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436218: 4BFFF6B9  bl 0x824358d0
	ctx.lr = 0x8243621C;
	sub_824358D0(ctx, base);
	// 8243621C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82436220: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436224: 480FEEE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82436228: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8243622C: 907F0414  stw r3, 0x414(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1044 as u32), ctx.r[3].u32 ) };
	// 82436230: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436234: 917F0418  stw r11, 0x418(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1048 as u32), ctx.r[11].u32 ) };
	// 82436238: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243623C: 480FEED0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82436240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82436244: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436248: 917F0414  stw r11, 0x414(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1044 as u32), ctx.r[11].u32 ) };
	// 8243624C: 917F0418  stw r11, 0x418(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1048 as u32), ctx.r[11].u32 ) };
	// 82436250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436254: 480FEEB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436258 size=332
    let mut pc: u32 = 0x82436258;
    'dispatch: loop {
        match pc {
            0x82436258 => {
    //   block [0x82436258..0x824363A4)
	// 82436258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82436264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243626C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436270: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82436274: 419A0118  beq cr6, 0x8243638c
	if ctx.cr[6].eq {
	pc = 0x8243638C; continue 'dispatch;
	}
	// 82436278: 4BFFF459  bl 0x824356d0
	ctx.lr = 0x8243627C;
	sub_824356D0(ctx, base);
	// 8243627C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82436280: 48005D39  bl 0x8243bfb8
	ctx.lr = 0x82436284;
	sub_8243BFB8(ctx, base);
	// 82436284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436288: 4BFFC501  bl 0x82432788
	ctx.lr = 0x8243628C;
	sub_82432788(ctx, base);
	// 8243628C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82436290: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436294: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82436298: 48005D21  bl 0x8243bfb8
	ctx.lr = 0x8243629C;
	sub_8243BFB8(ctx, base);
	// 8243629C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824362A0: 4BFFBD99  bl 0x82432038
	ctx.lr = 0x824362A4;
	sub_82432038(ctx, base);
	// 824362A4: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 824362A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362AC: 419A0008  beq cr6, 0x824362b4
	if ctx.cr[6].eq {
	pc = 0x824362B4; continue 'dispatch;
	}
	// 824362B0: 4BFFB931  bl 0x82431be0
	ctx.lr = 0x824362B4;
	sub_82431BE0(ctx, base);
	// 824362B4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824362B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362BC: 419A0008  beq cr6, 0x824362c4
	if ctx.cr[6].eq {
	pc = 0x824362C4; continue 'dispatch;
	}
	// 824362C0: 4BFF1819  bl 0x82427ad8
	ctx.lr = 0x824362C4;
	sub_82427AD8(ctx, base);
	// 824362C4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824362C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362CC: 419A0008  beq cr6, 0x824362d4
	if ctx.cr[6].eq {
	pc = 0x824362D4; continue 'dispatch;
	}
	// 824362D0: 480050A9  bl 0x8243b378
	ctx.lr = 0x824362D4;
	sub_8243B378(ctx, base);
	// 824362D4: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 824362D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362DC: 419A0014  beq cr6, 0x824362f0
	if ctx.cr[6].eq {
	pc = 0x824362F0; continue 'dispatch;
	}
	// 824362E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824362E4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824362E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824362EC: 4E800421  bctrl
	ctx.lr = 0x824362F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824362F0: 807F0474  lwz r3, 0x474(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1140 as u32) ) } as u64;
	// 824362F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362F8: 419A0014  beq cr6, 0x8243630c
	if ctx.cr[6].eq {
	pc = 0x8243630C; continue 'dispatch;
	}
	// 824362FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82436300: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82436308: 4E800421  bctrl
	ctx.lr = 0x8243630C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243630C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82436310: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436314: 419A0008  beq cr6, 0x8243631c
	if ctx.cr[6].eq {
	pc = 0x8243631C; continue 'dispatch;
	}
	// 82436318: 4BFFEEF1  bl 0x82435208
	ctx.lr = 0x8243631C;
	sub_82435208(ctx, base);
	// 8243631C: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 82436320: 48005509  bl 0x8243b828
	ctx.lr = 0x82436324;
	sub_8243B828(ctx, base);
	// 82436324: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 82436328: 48005501  bl 0x8243b828
	ctx.lr = 0x8243632C;
	sub_8243B828(ctx, base);
	// 8243632C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436330: 4BFFF5A1  bl 0x824358d0
	ctx.lr = 0x82436334;
	sub_824358D0(ctx, base);
	// 82436334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436338: 4BFFF301  bl 0x82435638
	ctx.lr = 0x8243633C;
	sub_82435638(ctx, base);
	// 8243633C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436340: 419A0010  beq cr6, 0x82436350
	if ctx.cr[6].eq {
	pc = 0x82436350; continue 'dispatch;
	}
	// 82436344: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436348: 386B5460  addi r3, r11, 0x5460
	ctx.r[3].s64 = ctx.r[11].s64 + 21600;
	// 8243634C: 48000D7D  bl 0x824370c8
	ctx.lr = 0x82436350;
	sub_824370C8(ctx, base);
	// 82436350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436354: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82436358: 4BFFF399  bl 0x824356f0
	ctx.lr = 0x8243635C;
	sub_824356F0(ctx, base);
	// 8243635C: 807F05B0  lwz r3, 0x5b0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82436360: 4BFF6C69  bl 0x8242cfc8
	ctx.lr = 0x82436364;
	sub_8242CFC8(ctx, base);
	// 82436364: 83DF05B0  lwz r30, 0x5b0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82436368: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243636C: 41980020  blt cr6, 0x8243638c
	if ctx.cr[6].lt {
	pc = 0x8243638C; continue 'dispatch;
	}
	// 82436370: 38A005B4  li r5, 0x5b4
	ctx.r[5].s64 = 1460;
	// 82436374: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243637C: 480FEE55  bl 0x825351d0
	ctx.lr = 0x82436380;
	sub_825351D0(ctx, base);
	// 82436380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82436384: 93DF05B0  stw r30, 0x5b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1456 as u32), ctx.r[30].u32 ) };
	// 82436388: 4BFF6CD9  bl 0x8242d060
	ctx.lr = 0x8243638C;
	sub_8242D060(ctx, base);
	// 8243638C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243639C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824363A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824363A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824363A8 size=4
    let mut pc: u32 = 0x824363A8;
    'dispatch: loop {
        match pc {
            0x824363A8 => {
    //   block [0x824363A8..0x824363AC)
	// 824363A8: 4BFFFEB0  b 0x82436258
	sub_82436258(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824363B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824363B0 size=984
    let mut pc: u32 = 0x824363B0;
    'dispatch: loop {
        match pc {
            0x824363B0 => {
    //   block [0x824363B0..0x82436788)
	// 824363B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824363B4: 480FECF5  bl 0x825350a8
	ctx.lr = 0x824363B8;
	sub_82535080(ctx, base);
	// 824363B8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824363BC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824363C0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 824363C4: 409A001C  bne cr6, 0x824363e0
	if !ctx.cr[6].eq {
	pc = 0x824363E0; continue 'dispatch;
	}
	// 824363C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824363CC: 386B55B8  addi r3, r11, 0x55b8
	ctx.r[3].s64 = ctx.r[11].s64 + 21944;
	// 824363D0: 48000CF9  bl 0x824370c8
	ctx.lr = 0x824363D4;
	sub_824370C8(ctx, base);
	// 824363D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824363D8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824363DC: 480FED1C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 824363E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824363E4: 4BFFF0FD  bl 0x824354e0
	ctx.lr = 0x824363E8;
	sub_824354E0(ctx, base);
	// 824363E8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824363EC: 409A0390  bne cr6, 0x8243677c
	if !ctx.cr[6].eq {
	pc = 0x8243677C; continue 'dispatch;
	}
	// 824363F0: 48000399  bl 0x82436788
	ctx.lr = 0x824363F4;
	sub_82436788(ctx, base);
	// 824363F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824363F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824363FC: 397C006C  addi r11, r28, 0x6c
	ctx.r[11].s64 = ctx.r[28].s64 + 108;
	// 82436400: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82436404: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82436408: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8243640C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82436410: 409A0014  bne cr6, 0x82436424
	if !ctx.cr[6].eq {
	pc = 0x82436424; continue 'dispatch;
	}
	// 82436414: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82436418: 396B05B4  addi r11, r11, 0x5b4
	ctx.r[11].s64 = ctx.r[11].s64 + 1460;
	// 8243641C: 2F1D0008  cmpwi cr6, r29, 8
	ctx.cr[6].compare_i32(ctx.r[29].s32, 8, &mut ctx.xer);
	// 82436420: 4198FFE4  blt cr6, 0x82436404
	if ctx.cr[6].lt {
	pc = 0x82436404; continue 'dispatch;
	}
	// 82436424: 2F1D0008  cmpwi cr6, r29, 8
	ctx.cr[6].compare_i32(ctx.r[29].s32, 8, &mut ctx.xer);
	// 82436428: 409A0024  bne cr6, 0x8243644c
	if !ctx.cr[6].eq {
	pc = 0x8243644C; continue 'dispatch;
	}
	// 8243642C: 3860FFF5  li r3, -0xb
	ctx.r[3].s64 = -11;
	// 82436430: 48000599  bl 0x824369c8
	ctx.lr = 0x82436434;
	sub_824369C8(ctx, base);
	// 82436434: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436438: 386B5568  addi r3, r11, 0x5568
	ctx.r[3].s64 = ctx.r[11].s64 + 21864;
	// 8243643C: 48000C8D  bl 0x824370c8
	ctx.lr = 0x82436440;
	sub_824370C8(ctx, base);
	// 82436440: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436444: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436448: 480FECB0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 8243644C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82436450: 4BFFEFF1  bl 0x82435440
	ctx.lr = 0x82436454;
	sub_82435440(ctx, base);
	// 82436454: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82436458: 409A001C  bne cr6, 0x82436474
	if !ctx.cr[6].eq {
	pc = 0x82436474; continue 'dispatch;
	}
	// 8243645C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436460: 386B5528  addi r3, r11, 0x5528
	ctx.r[3].s64 = ctx.r[11].s64 + 21800;
	// 82436464: 48000C65  bl 0x824370c8
	ctx.lr = 0x82436468;
	sub_824370C8(ctx, base);
	// 82436468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243646C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436470: 480FEC88  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82436474: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82436478: 57B8103A  slwi r24, r29, 2
	ctx.r[24].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 8243647C: 3B2B9B50  addi r25, r11, -0x64b0
	ctx.r[25].s64 = ctx.r[11].s64 + -25776;
	// 82436480: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82436484: 4BFF6B45  bl 0x8242cfc8
	ctx.lr = 0x82436488;
	sub_8242CFC8(ctx, base);
	// 82436488: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243648C: 419802F0  blt cr6, 0x8243677c
	if ctx.cr[6].lt {
	pc = 0x8243677C; continue 'dispatch;
	}
	// 82436490: 38A005B4  li r5, 0x5b4
	ctx.r[5].s64 = 1460;
	// 82436494: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243649C: 480FED35  bl 0x825351d0
	ctx.lr = 0x824364A0;
	sub_825351D0(ctx, base);
	// 824364A0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824364A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824364A8: 4BFFEFF9  bl 0x824354a0
	ctx.lr = 0x824364AC;
	sub_824354A0(ctx, base);
	// 824364AC: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 824364B0: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 824364B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824364B8: 480FE699  bl 0x82534b50
	ctx.lr = 0x824364BC;
	sub_82534B50(ctx, base);
	// 824364BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824364C0: 4BFFF251  bl 0x82435710
	ctx.lr = 0x824364C4;
	sub_82435710(ctx, base);
	// 824364C4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824364C8: 409A0010  bne cr6, 0x824364d8
	if !ctx.cr[6].eq {
	pc = 0x824364D8; continue 'dispatch;
	}
	// 824364CC: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824364D0: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 824364D4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824364D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824364DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824364E0: 4BFFF8B1  bl 0x82435d90
	ctx.lr = 0x824364E4;
	sub_82435D90(ctx, base);
	// 824364E4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824364E8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824364EC: 935F0048  stw r26, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 824364F0: 409A0024  bne cr6, 0x82436514
	if !ctx.cr[6].eq {
	pc = 0x82436514; continue 'dispatch;
	}
	// 824364F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824364F8: 386B5504  addi r3, r11, 0x5504
	ctx.r[3].s64 = ctx.r[11].s64 + 21764;
	// 824364FC: 48000BCD  bl 0x824370c8
	ctx.lr = 0x82436500;
	sub_824370C8(ctx, base);
	// 82436500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436504: 4BFFFEA5  bl 0x824363a8
	ctx.lr = 0x82436508;
	sub_824363A8(ctx, base);
	// 82436508: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243650C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436510: 480FEBE8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82436514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436518: 4BFFEC61  bl 0x82435178
	ctx.lr = 0x8243651C;
	sub_82435178(ctx, base);
	// 8243651C: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82436520: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82436524: 38E1005C  addi r7, r1, 0x5c
	ctx.r[7].s64 = ctx.r[1].s64 + 92;
	// 82436528: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8243652C: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82436530: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82436534: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82436538: 4BFFE831  bl 0x82434d68
	ctx.lr = 0x8243653C;
	sub_82434D68(ctx, base);
	// 8243653C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82436540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436544: 80DC000C  lwz r6, 0xc(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436548: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243654C: 4BFFECFD  bl 0x82435248
	ctx.lr = 0x82436550;
	sub_82435248(ctx, base);
	// 82436550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436554: 4BFFEEDD  bl 0x82435430
	ctx.lr = 0x82436558;
	sub_82435430(ctx, base);
	// 82436558: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243655C: 907F0454  stw r3, 0x454(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1108 as u32), ctx.r[3].u32 ) };
	// 82436560: 409A0024  bne cr6, 0x82436584
	if !ctx.cr[6].eq {
	pc = 0x82436584; continue 'dispatch;
	}
	// 82436564: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436568: 386B54E0  addi r3, r11, 0x54e0
	ctx.r[3].s64 = ctx.r[11].s64 + 21728;
	// 8243656C: 48000B5D  bl 0x824370c8
	ctx.lr = 0x82436570;
	sub_824370C8(ctx, base);
	// 82436570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436574: 4BFFFE35  bl 0x824363a8
	ctx.lr = 0x82436578;
	sub_824363A8(ctx, base);
	// 82436578: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243657C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436580: 480FEB78  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82436584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436588: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243658C: 4BFEF0AD  bl 0x82425638
	ctx.lr = 0x82436590;
	sub_82425638(ctx, base);
	// 82436590: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436594: 907F0474  stw r3, 0x474(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1140 as u32), ctx.r[3].u32 ) };
	// 82436598: 409A0024  bne cr6, 0x824365bc
	if !ctx.cr[6].eq {
	pc = 0x824365BC; continue 'dispatch;
	}
	// 8243659C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824365A0: 386B54BC  addi r3, r11, 0x54bc
	ctx.r[3].s64 = ctx.r[11].s64 + 21692;
	// 824365A4: 48000B25  bl 0x824370c8
	ctx.lr = 0x824365A8;
	sub_824370C8(ctx, base);
	// 824365A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824365AC: 4BFFFDFD  bl 0x824363a8
	ctx.lr = 0x824365B0;
	sub_824363A8(ctx, base);
	// 824365B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824365B4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824365B8: 480FEB40  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 824365BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824365C0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 824365C4: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 824365C8: 3BBF0050  addi r29, r31, 0x50
	ctx.r[29].s64 = ctx.r[31].s64 + 80;
	// 824365CC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 824365D0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 824365D4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824365D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824365DC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824365E0: 939F0044  stw r28, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[28].u32 ) };
	// 824365E4: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 824365E8: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824365EC: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 824365F0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824365F4: 4800624D  bl 0x8243c840
	ctx.lr = 0x824365F8;
	sub_8243C840(ctx, base);
	// 824365F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824365FC: 419A0008  beq cr6, 0x82436604
	if ctx.cr[6].eq {
	pc = 0x82436604; continue 'dispatch;
	}
	// 82436600: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82436604: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436608: 939F0078  stw r28, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[28].u32 ) };
	// 8243660C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436610: 939F007C  stw r28, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[28].u32 ) };
	// 82436614: 9BDF0080  stb r30, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82436618: 9BDF0081  stb r30, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8243661C: 9BDF0082  stb r30, 0x82(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(130 as u32), ctx.r[30].u8 ) };
	// 82436620: 9BDF0083  stb r30, 0x83(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(131 as u32), ctx.r[30].u8 ) };
	// 82436624: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82436628: 48001769  bl 0x82437d90
	ctx.lr = 0x8243662C;
	sub_82437D90(ctx, base);
	// 8243662C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436634: 48001795  bl 0x82437dc8
	ctx.lr = 0x82436638;
	sub_82437DC8(ctx, base);
	// 82436638: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 8243663C: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82436640: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82436644: 48004D2D  bl 0x8243b370
	ctx.lr = 0x82436648;
	sub_8243B370(ctx, base);
	// 82436648: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243664C: 907F004C  stw r3, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 82436650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436654: 419A0124  beq cr6, 0x82436778
	if ctx.cr[6].eq {
	pc = 0x82436778; continue 'dispatch;
	}
	// 82436658: 4BFFC409  bl 0x82432a60
	ctx.lr = 0x8243665C;
	sub_82432A60(ctx, base);
	// 8243665C: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 82436660: 4BFF0FF9  bl 0x82427658
	ctx.lr = 0x82436664;
	sub_82427658(ctx, base);
	// 82436664: 809F004C  lwz r4, 0x4c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82436668: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8243666C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82436670: 4BFF1029  bl 0x82427698
	ctx.lr = 0x82436674;
	sub_82427698(ctx, base);
	// 82436674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436678: 4BFFFB09  bl 0x82436180
	ctx.lr = 0x8243667C;
	sub_82436180(ctx, base);
	// 8243667C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82436680: 419AFE80  beq cr6, 0x82436500
	if ctx.cr[6].eq {
	pc = 0x82436500; continue 'dispatch;
	}
	// 82436684: 80DB000C  lwz r6, 0xc(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436688: 80BB0008  lwz r5, 8(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243668C: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82436690: 807F00BC  lwz r3, 0xbc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82436694: 4BFFB545  bl 0x82431bd8
	ctx.lr = 0x82436698;
	sub_82431BD8(ctx, base);
	// 82436698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243669C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824366A0: 409A0024  bne cr6, 0x824366c4
	if !ctx.cr[6].eq {
	pc = 0x824366C4; continue 'dispatch;
	}
	// 824366A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824366A8: 386B54A0  addi r3, r11, 0x54a0
	ctx.r[3].s64 = ctx.r[11].s64 + 21664;
	// 824366AC: 48000A1D  bl 0x824370c8
	ctx.lr = 0x824366B0;
	sub_824370C8(ctx, base);
	// 824366B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824366B4: 4BFFFCF5  bl 0x824363a8
	ctx.lr = 0x824366B8;
	sub_824363A8(ctx, base);
	// 824366B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824366BC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824366C0: 480FEA38  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 824366C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824366C8: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824366CC: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 824366D0: 4BFFB831  bl 0x82431f00
	ctx.lr = 0x824366D4;
	sub_82431F00(ctx, base);
	// 824366D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824366D8: 4BFFB8F9  bl 0x82431fd0
	ctx.lr = 0x824366DC;
	sub_82431FD0(ctx, base);
	// 824366DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824366E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824366E4: 917F0410  stw r11, 0x410(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1040 as u32), ctx.r[11].u32 ) };
	// 824366E8: 4BFFB979  bl 0x82432060
	ctx.lr = 0x824366EC;
	sub_82432060(ctx, base);
	// 824366EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824366F0: 419A0024  beq cr6, 0x82436714
	if ctx.cr[6].eq {
	pc = 0x82436714; continue 'dispatch;
	}
	// 824366F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824366F8: 386B5478  addi r3, r11, 0x5478
	ctx.r[3].s64 = ctx.r[11].s64 + 21624;
	// 824366FC: 480009CD  bl 0x824370c8
	ctx.lr = 0x82436700;
	sub_824370C8(ctx, base);
	// 82436700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436704: 4BFFFCA5  bl 0x824363a8
	ctx.lr = 0x82436708;
	sub_824363A8(ctx, base);
	// 82436708: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243670C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436710: 480FE9E8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82436714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436718: 4BFFB9B1  bl 0x824320c8
	ctx.lr = 0x8243671C;
	sub_824320C8(ctx, base);
	// 8243671C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436720: 4BFFDCF1  bl 0x82434410
	ctx.lr = 0x82436724;
	sub_82434410(ctx, base);
	// 82436724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436728: 4BFFE629  bl 0x82434d50
	ctx.lr = 0x8243672C;
	sub_82434D50(ctx, base);
	// 8243672C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82436730: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82436734: 93DF0580  stw r30, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[30].u32 ) };
	// 82436738: 93DF0570  stw r30, 0x570(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1392 as u32), ctx.r[30].u32 ) };
	// 8243673C: 93DF0574  stw r30, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[30].u32 ) };
	// 82436740: 93DF0578  stw r30, 0x578(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1400 as u32), ctx.r[30].u32 ) };
	// 82436744: 93DF0598  stw r30, 0x598(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1432 as u32), ctx.r[30].u32 ) };
	// 82436748: 917F0594  stw r11, 0x594(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 8243674C: 93DF059C  stw r30, 0x59c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1436 as u32), ctx.r[30].u32 ) };
	// 82436750: 93DF05A0  stw r30, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[30].u32 ) };
	// 82436754: 93DF05A4  stw r30, 0x5a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1444 as u32), ctx.r[30].u32 ) };
	// 82436758: 93DF05A8  stw r30, 0x5a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1448 as u32), ctx.r[30].u32 ) };
	// 8243675C: 7D78C82E  lwzx r11, r24, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82436760: 917F05B0  stw r11, 0x5b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1456 as u32), ctx.r[11].u32 ) };
	// 82436764: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82436768: 4BFF68F9  bl 0x8242d060
	ctx.lr = 0x8243676C;
	sub_8242D060(ctx, base);
	// 8243676C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436774: 4098000C  bge cr6, 0x82436780
	if !ctx.cr[6].lt {
	pc = 0x82436780; continue 'dispatch;
	}
	// 82436778: 4BFFFC31  bl 0x824363a8
	ctx.lr = 0x8243677C;
	sub_824363A8(ctx, base);
	// 8243677C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436780: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436784: 480FE974  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82436788 size=12
    let mut pc: u32 = 0x82436788;
    'dispatch: loop {
        match pc {
            0x82436788 => {
    //   block [0x82436788..0x82436794)
	// 82436788: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243678C: 386B0EC0  addi r3, r11, 0xec0
	ctx.r[3].s64 = ctx.r[11].s64 + 3776;
	// 82436790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82436798 size=8
    let mut pc: u32 = 0x82436798;
    'dispatch: loop {
        match pc {
            0x82436798 => {
    //   block [0x82436798..0x824367A0)
	// 82436798: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8243679C: 4800092C  b 0x824370c8
	sub_824370C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824367A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824367A0 size=104
    let mut pc: u32 = 0x824367A0;
    'dispatch: loop {
        match pc {
            0x824367A0 => {
    //   block [0x824367A0..0x82436808)
	// 824367A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824367A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824367A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824367AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824367B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824367B4: 38CB56B0  addi r6, r11, 0x56b0
	ctx.r[6].s64 = ctx.r[11].s64 + 22192;
	// 824367B8: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824367BC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824367C0: 388BC2C0  addi r4, r11, -0x3d40
	ctx.r[4].s64 = ctx.r[11].s64 + -15680;
	// 824367C4: 48000825  bl 0x82436fe8
	ctx.lr = 0x824367C8;
	sub_82436FE8(ctx, base);
	// 824367C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824367CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824367D0: 38AB5698  addi r5, r11, 0x5698
	ctx.r[5].s64 = ctx.r[11].s64 + 22168;
	// 824367D4: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824367D8: 386BC598  addi r3, r11, -0x3a68
	ctx.r[3].s64 = ctx.r[11].s64 + -14952;
	// 824367DC: 4800088D  bl 0x82437068
	ctx.lr = 0x824367E0;
	sub_82437068(ctx, base);
	// 824367E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824367E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824367E8: 38AB5680  addi r5, r11, 0x5680
	ctx.r[5].s64 = ctx.r[11].s64 + 22144;
	// 824367EC: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824367F0: 386BC5F8  addi r3, r11, -0x3a08
	ctx.r[3].s64 = ctx.r[11].s64 + -14856;
	// 824367F4: 4800083D  bl 0x82437030
	ctx.lr = 0x824367F8;
	sub_82437030(ctx, base);
	// 824367F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824367FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82436808 size=168
    let mut pc: u32 = 0x82436808;
    'dispatch: loop {
        match pc {
            0x82436808 => {
    //   block [0x82436808..0x824368B0)
	// 82436808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243680C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82436814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243681C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82436820: 4BFFFF69  bl 0x82436788
	ctx.lr = 0x82436824;
	sub_82436788(ctx, base);
	// 82436824: 38A02E10  li r5, 0x2e10
	ctx.r[5].s64 = 11792;
	// 82436828: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243682C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436830: 480FE9A1  bl 0x825351d0
	ctx.lr = 0x82436834;
	sub_825351D0(ctx, base);
	// 82436834: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436838: 480055F1  bl 0x8243be28
	ctx.lr = 0x8243683C;
	sub_8243BE28(ctx, base);
	// 8243683C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82436840: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82436844: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82436848: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8243684C: 419A0028  beq cr6, 0x82436874
	if ctx.cr[6].eq {
	pc = 0x82436874; continue 'dispatch;
	}
	// 82436850: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82436854: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82436858: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243685C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82436860: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82436864: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82436868: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243686C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82436870: 4800001C  b 0x8243688c
	pc = 0x8243688C; continue 'dispatch;
	// 82436874: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82436878: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8243687C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82436880: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82436884: C0091FF4  lfs f0, 0x1ff4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8180 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82436888: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8243688C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82436890: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82436894: 917F2E0C  stw r11, 0x2e0c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11788 as u32), ctx.r[11].u32 ) };
	// 82436898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243689C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824368A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824368A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824368A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824368AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824368B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824368B0 size=36
    let mut pc: u32 = 0x824368B0;
    'dispatch: loop {
        match pc {
            0x824368B0 => {
    //   block [0x824368B0..0x824368D4)
	// 824368B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824368B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824368B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824368BC: 4BFFFECD  bl 0x82436788
	ctx.lr = 0x824368C0;
	sub_82436788(ctx, base);
	// 824368C0: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 824368C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824368C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824368CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824368D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824368D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824368D8 size=36
    let mut pc: u32 = 0x824368D8;
    'dispatch: loop {
        match pc {
            0x824368D8 => {
    //   block [0x824368D8..0x824368FC)
	// 824368D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824368DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824368E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824368E4: 4BFFFEA5  bl 0x82436788
	ctx.lr = 0x824368E8;
	sub_82436788(ctx, base);
	// 824368E8: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 824368EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824368F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824368F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824368F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436900 size=196
    let mut pc: u32 = 0x82436900;
    'dispatch: loop {
        match pc {
            0x82436900 => {
    //   block [0x82436900..0x82436980)
	// 82436900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243690C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436910: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436914: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82436918: 4BF8B361  bl 0x823c1c78
	ctx.lr = 0x8243691C;
	sub_823C1C78(ctx, base);
	// 8243691C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436920: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82436924: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82436928: 4BFE5B89  bl 0x8241c4b0
	ctx.lr = 0x8243692C;
	sub_8241C4B0(ctx, base);
	// 8243692C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82436930: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82436934: 419A0014  beq cr6, 0x82436948
	if ctx.cr[6].eq {
	pc = 0x82436948; continue 'dispatch;
	}
	// 82436938: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243693C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82436940: 419A0008  beq cr6, 0x82436948
	if ctx.cr[6].eq {
	pc = 0x82436948; continue 'dispatch;
	}
	// 82436944: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82436948: 2B1F0005  cmplwi cr6, r31, 5
	ctx.cr[6].compare_u32(ctx.r[31].u32, 5 as u32, &mut ctx.xer);
	// 8243694C: 4199005C  bgt cr6, 0x824369a8
	if ctx.cr[6].gt {
	pc = 0x824369A8; continue 'dispatch;
	}
	// 82436950: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82436954: 398C6968  addi r12, r12, 0x6968
	ctx.r[12].s64 = ctx.r[12].s64 + 26984;
	// 82436958: 57E0103A  slwi r0, r31, 2
	ctx.r[0].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8243695C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82436960: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82436964: 4E800420  bctr
	match ctx.r[31].u64 {
		0 => {
	pc = 0x824369A8; continue 'dispatch;
		},
		1 => {
	pc = 0x82436980; continue 'dispatch;
		},
		2 => {
	pc = 0x82436988; continue 'dispatch;
		},
		3 => {
	pc = 0x82436990; continue 'dispatch;
		},
		4 => {
	pc = 0x82436998; continue 'dispatch;
		},
		5 => {
	pc = 0x824369A0; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82436968: 824369A8  lwz r18, 0x69a8(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27048 as u32) ) } as u64;
	// 8243696C: 82436980  lwz r18, 0x6980(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27008 as u32) ) } as u64;
	// 82436970: 82436988  lwz r18, 0x6988(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27016 as u32) ) } as u64;
	// 82436974: 82436990  lwz r18, 0x6990(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27024 as u32) ) } as u64;
	// 82436978: 82436998  lwz r18, 0x6998(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27032 as u32) ) } as u64;
	// 8243697C: 824369A0  lwz r18, 0x69a0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27040 as u32) ) } as u64;
            }
            0x82436980 => {
    //   block [0x82436980..0x82436988)
	// 82436980: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82436984: 48000028  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x82436988 => {
    //   block [0x82436988..0x82436990)
	// 82436988: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8243698C: 48000020  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x82436990 => {
    //   block [0x82436990..0x82436998)
	// 82436990: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82436994: 48000018  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x82436998 => {
    //   block [0x82436998..0x824369A0)
	// 82436998: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8243699C: 48000010  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x824369A0 => {
    //   block [0x824369A0..0x824369A8)
	// 824369A0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 824369A4: 48000008  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x824369A8 => {
    //   block [0x824369A8..0x824369C4)
	// 824369A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824369AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824369B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824369B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824369B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824369BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824369C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824369C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824369C8 size=48
    let mut pc: u32 = 0x824369C8;
    'dispatch: loop {
        match pc {
            0x824369C8 => {
    //   block [0x824369C8..0x824369F8)
	// 824369C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824369CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824369D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824369D4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824369D8: 4BFFFDB1  bl 0x82436788
	ctx.lr = 0x824369DC;
	sub_82436788(ctx, base);
	// 824369DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824369E0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 824369E4: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 824369E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824369EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824369F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824369F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824369F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824369F8 size=488
    let mut pc: u32 = 0x824369F8;
    'dispatch: loop {
        match pc {
            0x824369F8 => {
    //   block [0x824369F8..0x82436BE0)
	// 824369F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824369FC: 480FE6C1  bl 0x825350bc
	ctx.lr = 0x82436A00;
	sub_82535080(ctx, base);
	// 82436A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436A04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82436A08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82436A0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82436A10: 419A0020  beq cr6, 0x82436a30
	if ctx.cr[6].eq {
	pc = 0x82436A30; continue 'dispatch;
	}
	// 82436A14: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82436A18: 4BFFCA19  bl 0x82433430
	ctx.lr = 0x82436A1C;
	sub_82433430(ctx, base);
	// 82436A1C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82436A20: 93CB0EA8  stw r30, 0xea8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3752 as u32), ctx.r[30].u32 ) };
	// 82436A24: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82436A28: 906B3CD0  stw r3, 0x3cd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(15568 as u32), ctx.r[3].u32 ) };
	// 82436A2C: 4800001C  b 0x82436a48
	pc = 0x82436A48; continue 'dispatch;
	// 82436A30: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82436A34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82436A38: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82436A3C: 916A0EA8  stw r11, 0xea8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(3752 as u32), ctx.r[11].u32 ) };
	// 82436A40: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82436A44: 916A3CD0  stw r11, 0x3cd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(15568 as u32), ctx.r[11].u32 ) };
	// 82436A48: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82436A4C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82436A50: 3BCB9C90  addi r30, r11, -0x6370
	ctx.r[30].s64 = ctx.r[11].s64 + -25456;
	// 82436A54: 419A0020  beq cr6, 0x82436a74
	if ctx.cr[6].eq {
	pc = 0x82436A74; continue 'dispatch;
	}
	// 82436A58: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82436A5C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82436A60: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 82436A64: 7FEAF12E  stwx r31, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u32) };
	// 82436A68: 4098000C  bge cr6, 0x82436a74
	if !ctx.cr[6].lt {
	pc = 0x82436A74; continue 'dispatch;
	}
	// 82436A6C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82436A70: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82436A74: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436A78: 61650F15  ori r5, r11, 0xf15
	ctx.r[5].u64 = ctx.r[11].u64 | 3861;
	// 82436A7C: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82436A80: 4199008C  bgt cr6, 0x82436b0c
	if ctx.cr[6].gt {
	pc = 0x82436B0C; continue 'dispatch;
	}
	// 82436A84: 419A007C  beq cr6, 0x82436b00
	if ctx.cr[6].eq {
	pc = 0x82436B00; continue 'dispatch;
	}
	// 82436A88: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436A8C: 61650C04  ori r5, r11, 0xc04
	ctx.r[5].u64 = ctx.r[11].u64 | 3076;
	// 82436A90: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82436A94: 41990050  bgt cr6, 0x82436ae4
	if ctx.cr[6].gt {
	pc = 0x82436AE4; continue 'dispatch;
	}
	// 82436A98: 419A0040  beq cr6, 0x82436ad8
	if ctx.cr[6].eq {
	pc = 0x82436AD8; continue 'dispatch;
	}
	// 82436A9C: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436AA0: 616B0408  ori r11, r11, 0x408
	ctx.r[11].u64 = ctx.r[11].u64 | 1032;
	// 82436AA4: 7D6BF851  subf. r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82436AA8: 418200A8  beq 0x82436b50
	if ctx.cr[0].eq {
	pc = 0x82436B50; continue 'dispatch;
	}
	// 82436AAC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82436AB0: 419A00A0  beq cr6, 0x82436b50
	if ctx.cr[6].eq {
	pc = 0x82436B50; continue 'dispatch;
	}
	// 82436AB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436AB8: 388B599C  addi r4, r11, 0x599c
	ctx.r[4].s64 = ctx.r[11].s64 + 22940;
	// 82436ABC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82436AC0: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 82436AC4: 480FC15D  bl 0x82532c20
	ctx.lr = 0x82436AC8;
	sub_82532C20(ctx, base);
	// 82436AC8: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 82436ACC: 480005FD  bl 0x824370c8
	ctx.lr = 0x82436AD0;
	sub_824370C8(ctx, base);
	// 82436AD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436AD4: 480FE638  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82436AD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436ADC: 388B5918  addi r4, r11, 0x5918
	ctx.r[4].s64 = ctx.r[11].s64 + 22808;
	// 82436AE0: 4BFFFFE0  b 0x82436ac0
	pc = 0x82436AC0; continue 'dispatch;
	// 82436AE4: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436AE8: 61650F04  ori r5, r11, 0xf04
	ctx.r[5].u64 = ctx.r[11].u64 | 3844;
	// 82436AEC: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82436AF0: 409AFFC4  bne cr6, 0x82436ab4
	if !ctx.cr[6].eq {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436AF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436AF8: 388B58B8  addi r4, r11, 0x58b8
	ctx.r[4].s64 = ctx.r[11].s64 + 22712;
	// 82436AFC: 4BFFFFC4  b 0x82436ac0
	pc = 0x82436AC0; continue 'dispatch;
	// 82436B00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436B04: 388B5848  addi r4, r11, 0x5848
	ctx.r[4].s64 = ctx.r[11].s64 + 22600;
	// 82436B08: 4BFFFFB8  b 0x82436ac0
	pc = 0x82436AC0; continue 'dispatch;
	// 82436B0C: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436B10: 61650F1F  ori r5, r11, 0xf1f
	ctx.r[5].u64 = ctx.r[11].u64 | 3871;
	// 82436B14: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82436B18: 419900AC  bgt cr6, 0x82436bc4
	if ctx.cr[6].gt {
	pc = 0x82436BC4; continue 'dispatch;
	}
	// 82436B1C: 419A009C  beq cr6, 0x82436bb8
	if ctx.cr[6].eq {
	pc = 0x82436BB8; continue 'dispatch;
	}
	// 82436B20: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436B24: 616B0F17  ori r11, r11, 0xf17
	ctx.r[11].u64 = ctx.r[11].u64 | 3863;
	// 82436B28: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82436B2C: 4198FF88  blt cr6, 0x82436ab4
	if ctx.cr[6].lt {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436B30: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436B34: 616B0F18  ori r11, r11, 0xf18
	ctx.r[11].u64 = ctx.r[11].u64 | 3864;
	// 82436B38: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82436B3C: 40990020  ble cr6, 0x82436b5c
	if !ctx.cr[6].gt {
	pc = 0x82436B5C; continue 'dispatch;
	}
	// 82436B40: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436B44: 616B0F1C  ori r11, r11, 0xf1c
	ctx.r[11].u64 = ctx.r[11].u64 | 3868;
	// 82436B48: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82436B4C: 409AFF68  bne cr6, 0x82436ab4
	if !ctx.cr[6].eq {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436B50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436B54: 388B57F0  addi r4, r11, 0x57f0
	ctx.r[4].s64 = ctx.r[11].s64 + 22512;
	// 82436B58: 4BFFFF64  b 0x82436abc
	pc = 0x82436ABC; continue 'dispatch;
	// 82436B5C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82436B60: 419A0028  beq cr6, 0x82436b88
	if ctx.cr[6].eq {
	pc = 0x82436B88; continue 'dispatch;
	}
	// 82436B64: 817D00D0  lwz r11, 0xd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 82436B68: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82436B6C: 409A001C  bne cr6, 0x82436b88
	if !ctx.cr[6].eq {
	pc = 0x82436B88; continue 'dispatch;
	}
	// 82436B70: 80DD00D8  lwz r6, 0xd8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82436B74: 80FD00DC  lwz r7, 0xdc(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(220 as u32) ) } as u64;
	// 82436B78: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82436B7C: 4099000C  ble cr6, 0x82436b88
	if !ctx.cr[6].gt {
	pc = 0x82436B88; continue 'dispatch;
	}
	// 82436B80: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82436B84: 41990010  bgt cr6, 0x82436b94
	if ctx.cr[6].gt {
	pc = 0x82436B94; continue 'dispatch;
	}
	// 82436B88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436B8C: 388B5790  addi r4, r11, 0x5790
	ctx.r[4].s64 = ctx.r[11].s64 + 22416;
	// 82436B90: 4BFFFF2C  b 0x82436abc
	pc = 0x82436ABC; continue 'dispatch;
	// 82436B94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436B98: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 82436B9C: 388B5728  addi r4, r11, 0x5728
	ctx.r[4].s64 = ctx.r[11].s64 + 22312;
	// 82436BA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82436BA4: 480FC07D  bl 0x82532c20
	ctx.lr = 0x82436BA8;
	sub_82532C20(ctx, base);
	// 82436BA8: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 82436BAC: 4800051D  bl 0x824370c8
	ctx.lr = 0x82436BB0;
	sub_824370C8(ctx, base);
	// 82436BB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436BB4: 480FE558  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82436BB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436BBC: 388B56E0  addi r4, r11, 0x56e0
	ctx.r[4].s64 = ctx.r[11].s64 + 22240;
	// 82436BC0: 4BFFFF00  b 0x82436ac0
	pc = 0x82436AC0; continue 'dispatch;
	// 82436BC4: 2F1FFFFD  cmpwi cr6, r31, -3
	ctx.cr[6].compare_i32(ctx.r[31].s32, -3, &mut ctx.xer);
	// 82436BC8: 4198FEEC  blt cr6, 0x82436ab4
	if ctx.cr[6].lt {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436BCC: 2F1FFFFE  cmpwi cr6, r31, -2
	ctx.cr[6].compare_i32(ctx.r[31].s32, -2, &mut ctx.xer);
	// 82436BD0: 4199FEE4  bgt cr6, 0x82436ab4
	if ctx.cr[6].gt {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436BD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436BD8: 388B56C8  addi r4, r11, 0x56c8
	ctx.r[4].s64 = ctx.r[11].s64 + 22216;
	// 82436BDC: 4BFFFEE0  b 0x82436abc
	pc = 0x82436ABC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436BE0 size=68
    let mut pc: u32 = 0x82436BE0;
    'dispatch: loop {
        match pc {
            0x82436BE0 => {
    //   block [0x82436BE0..0x82436C24)
	// 82436BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436BEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436BF0: 419A001C  beq cr6, 0x82436c0c
	if ctx.cr[6].eq {
	pc = 0x82436C0C; continue 'dispatch;
	}
	// 82436BF4: 4BFFC83D  bl 0x82433430
	ctx.lr = 0x82436BF8;
	sub_82433430(ctx, base);
	// 82436BF8: 48009379  bl 0x8243ff70
	ctx.lr = 0x82436BFC;
	sub_8243FF70(ctx, base);
	// 82436BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436C08: 4E800020  blr
	return;
	// 82436C0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436C10: 48009361  bl 0x8243ff70
	ctx.lr = 0x82436C14;
	sub_8243FF70(ctx, base);
	// 82436C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


