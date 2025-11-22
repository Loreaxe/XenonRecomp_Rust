pub fn sub_82200EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82200EB0 size=16
    let mut pc: u32 = 0x82200EB0;
    'dispatch: loop {
        match pc {
            0x82200EB0 => {
    //   block [0x82200EB0..0x82200EC0)
	// 82200EB0: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82200EB4: 396B3F00  addi r11, r11, 0x3f00
	ctx.r[11].s64 = ctx.r[11].s64 + 16128;
	// 82200EB8: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82200EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82200EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82200EC0 size=8
    let mut pc: u32 = 0x82200EC0;
    'dispatch: loop {
        match pc {
            0x82200EC0 => {
    //   block [0x82200EC0..0x82200EC8)
	// 82200EC0: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82200EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82200EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82200EC8 size=188
    let mut pc: u32 = 0x82200EC8;
    'dispatch: loop {
        match pc {
            0x82200EC8 => {
    //   block [0x82200EC8..0x82200F84)
	// 82200EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82200ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82200ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82200ED4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82200ED8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82200EDC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82200EE0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82200EE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82200EE8: 4E800421  bctrl
	ctx.lr = 0x82200EEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82200EEC: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 82200EF0: 5468103A  slwi r8, r3, 2
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82200EF4: 89410054  lbz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82200EF8: 396B5510  addi r11, r11, 0x5510
	ctx.r[11].s64 = ctx.r[11].s64 + 21776;
	// 82200EFC: 89210055  lbz r9, 0x55(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82200F00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82200F04: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 82200F08: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82200F0C: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82200F10: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82200F14: 41980054  blt cr6, 0x82200f68
	if ctx.cr[6].lt {
	pc = 0x82200F68; continue 'dispatch;
	}
	// 82200F18: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82200F1C: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82200F20: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82200F24: 40980044  bge cr6, 0x82200f68
	if !ctx.cr[6].lt {
	pc = 0x82200F68; continue 'dispatch;
	}
	// 82200F28: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82200F2C: 4198003C  blt cr6, 0x82200f68
	if ctx.cr[6].lt {
	pc = 0x82200F68; continue 'dispatch;
	}
	// 82200F30: A10B0002  lhz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82200F34: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82200F38: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82200F3C: 4098002C  bge cr6, 0x82200f68
	if !ctx.cr[6].lt {
	pc = 0x82200F68; continue 'dispatch;
	}
	// 82200F40: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82200F44: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82200F48: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82200F4C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82200F50: 7D6A5A2E  lhzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82200F54: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 82200F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82200F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82200F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82200F64: 4E800020  blr
	return;
	// 82200F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82200F6C: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82200F70: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 82200F74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82200F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82200F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82200F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82200F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82200F88 size=108
    let mut pc: u32 = 0x82200F88;
    'dispatch: loop {
        match pc {
            0x82200F88 => {
    //   block [0x82200F88..0x82200FF4)
	// 82200F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82200F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82200F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82200F94: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82200F98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82200F9C: 419A000C  beq cr6, 0x82200fa8
	if ctx.cr[6].eq {
	pc = 0x82200FA8; continue 'dispatch;
	}
	// 82200FA0: 4BFF34C9  bl 0x821f4468
	ctx.lr = 0x82200FA4;
	sub_821F4468(ctx, base);
	// 82200FA4: 4800000C  b 0x82200fb0
	pc = 0x82200FB0; continue 'dispatch;
	// 82200FA8: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82200FAC: 386BBABC  addi r3, r11, -0x4544
	ctx.r[3].s64 = ctx.r[11].s64 + -17732;
	// 82200FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82200FB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82200FB8: 41990018  bgt cr6, 0x82200fd0
	if ctx.cr[6].gt {
	pc = 0x82200FD0; continue 'dispatch;
	}
	// 82200FBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82200FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82200FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82200FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82200FCC: 4E800020  blr
	return;
	// 82200FD0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82200FD4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82200FD8: 7D4B5051  subf. r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82200FDC: 4182FFE0  beq 0x82200fbc
	if ctx.cr[0].eq {
	pc = 0x82200FBC; continue 'dispatch;
	}
	// 82200FE0: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82200FE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82200FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82200FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82200FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82200FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82200FF8 size=232
    let mut pc: u32 = 0x82200FF8;
    'dispatch: loop {
        match pc {
            0x82200FF8 => {
    //   block [0x82200FF8..0x822010E0)
	// 82200FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82200FFC: 483340B5  bl 0x825350b0
	ctx.lr = 0x82201000;
	sub_82535080(ctx, base);
	// 82201000: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201004: 3FE08311  lis r31, -0x7cef
	ctx.r[31].s64 = -2096037888;
	// 82201008: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220100C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82201010: 388B39F8  addi r4, r11, 0x39f8
	ctx.r[4].s64 = ctx.r[11].s64 + 14840;
	// 82201014: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82201018: 817F3AF0  lwz r11, 0x3af0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(15088 as u32) ) } as u64;
	// 8220101C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82201020: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82201024: 5565053E  clrlwi r5, r11, 0x14
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82201028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8220102C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82201030: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82201034: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82201038: 48331BE9  bl 0x82532c20
	ctx.lr = 0x8220103C;
	sub_82532C20(ctx, base);
	// 8220103C: 817F3AF0  lwz r11, 0x3af0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(15088 as u32) ) } as u64;
	// 82201040: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82201044: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82201048: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8220104C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82201050: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82201054: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 82201058: 917F3AF0  stw r11, 0x3af0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(15088 as u32), ctx.r[11].u32 ) };
	// 8220105C: 419A0020  beq cr6, 0x8220107c
	if ctx.cr[6].eq {
	pc = 0x8220107C; continue 'dispatch;
	}
	// 82201060: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82201064: 4816942D  bl 0x8236a490
	ctx.lr = 0x82201068;
	sub_8236A490(ctx, base);
	// 82201068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220106C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82201070: 419A002C  beq cr6, 0x8220109c
	if ctx.cr[6].eq {
	pc = 0x8220109C; continue 'dispatch;
	}
	// 82201074: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82201078: 48000028  b 0x822010a0
	pc = 0x822010A0; continue 'dispatch;
	// 8220107C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82201080: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82201084: 4816940D  bl 0x8236a490
	ctx.lr = 0x82201088;
	sub_8236A490(ctx, base);
	// 82201088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220108C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82201090: 419A000C  beq cr6, 0x8220109c
	if ctx.cr[6].eq {
	pc = 0x8220109C; continue 'dispatch;
	}
	// 82201094: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82201098: 48000008  b 0x822010a0
	pc = 0x822010A0; continue 'dispatch;
	// 8220109C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822010A0: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 822010A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822010A8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822010AC: 419A0028  beq cr6, 0x822010d4
	if ctx.cr[6].eq {
	pc = 0x822010D4; continue 'dispatch;
	}
	// 822010B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822010B4: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 822010B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822010BC: 48000025  bl 0x822010e0
	ctx.lr = 0x822010C0;
	sub_822010E0(ctx, base);
	// 822010C0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 822010C4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822010C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822010CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822010D0: 480DA129  bl 0x822db1f8
	ctx.lr = 0x822010D4;
	sub_822DB1F8(ctx, base);
	// 822010D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822010D8: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 822010DC: 48334024  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822010E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822010E0 size=136
    let mut pc: u32 = 0x822010E0;
    'dispatch: loop {
        match pc {
            0x822010E0 => {
    //   block [0x822010E0..0x82201168)
	// 822010E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822010E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822010E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822010EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822010F0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822010F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822010F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822010FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201100: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201104: 4E800421  bctrl
	ctx.lr = 0x82201108;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201108: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220110C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82201110: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82201114: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201118: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8220111C: 419A0024  beq cr6, 0x82201140
	if ctx.cr[6].eq {
	pc = 0x82201140; continue 'dispatch;
	}
	// 82201120: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82201124: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82201128: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8220112C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82201130: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82201134: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82201138: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8220113C: 4082FFE8  bne 0x82201124
	if !ctx.cr[0].eq {
	pc = 0x82201124; continue 'dispatch;
	}
	// 82201140: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82201144: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82201148: 419A0008  beq cr6, 0x82201150
	if ctx.cr[6].eq {
	pc = 0x82201150; continue 'dispatch;
	}
	// 8220114C: 4BF1BF0D  bl 0x8211d058
	ctx.lr = 0x82201150;
	sub_8211D058(ctx, base);
	// 82201150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82201158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8220115C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82201160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82201164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201168 size=16
    let mut pc: u32 = 0x82201168;
    'dispatch: loop {
        match pc {
            0x82201168 => {
    //   block [0x82201168..0x82201178)
	// 82201168: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8220116C: 396B3F00  addi r11, r11, 0x3f00
	ctx.r[11].s64 = ctx.r[11].s64 + 16128;
	// 82201170: 386B0050  addi r3, r11, 0x50
	ctx.r[3].s64 = ctx.r[11].s64 + 80;
	// 82201174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201178 size=8
    let mut pc: u32 = 0x82201178;
    'dispatch: loop {
        match pc {
            0x82201178 => {
    //   block [0x82201178..0x82201180)
	// 82201178: 3860000B  li r3, 0xb
	ctx.r[3].s64 = 11;
	// 8220117C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82201180 size=264
    let mut pc: u32 = 0x82201180;
    'dispatch: loop {
        match pc {
            0x82201180 => {
    //   block [0x82201180..0x82201288)
	// 82201180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82201184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82201188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8220118C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201190: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82201194: 556A04E6  rlwinm r10, r11, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82201198: 557F053E  clrlwi r31, r11, 0x14
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 8220119C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822011A0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822011A4: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 822011A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822011AC: 419A0060  beq cr6, 0x8220120c
	if ctx.cr[6].eq {
	pc = 0x8220120C; continue 'dispatch;
	}
	// 822011B0: 4E800421  bctrl
	ctx.lr = 0x822011B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822011B4: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 822011B8: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822011BC: 396B5510  addi r11, r11, 0x5510
	ctx.r[11].s64 = ctx.r[11].s64 + 21776;
	// 822011C0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822011C4: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 822011C8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822011CC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822011D0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 822011D4: 41980094  blt cr6, 0x82201268
	if ctx.cr[6].lt {
	pc = 0x82201268; continue 'dispatch;
	}
	// 822011D8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822011DC: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 822011E0: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822011E4: 40980084  bge cr6, 0x82201268
	if !ctx.cr[6].lt {
	pc = 0x82201268; continue 'dispatch;
	}
	// 822011E8: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 822011EC: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 822011F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822011F4: 40990074  ble cr6, 0x82201268
	if !ctx.cr[6].gt {
	pc = 0x82201268; continue 'dispatch;
	}
	// 822011F8: 7D4AF9D6  mullw r10, r10, r31
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[31].s32 as i64);
	// 822011FC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82201200: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82201204: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82201208: 48000064  b 0x8220126c
	pc = 0x8220126C; continue 'dispatch;
	// 8220120C: 4E800421  bctrl
	ctx.lr = 0x82201210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201210: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 82201214: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82201218: 396B5510  addi r11, r11, 0x5510
	ctx.r[11].s64 = ctx.r[11].s64 + 21776;
	// 8220121C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82201220: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 82201224: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82201228: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220122C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82201230: 41980038  blt cr6, 0x82201268
	if ctx.cr[6].lt {
	pc = 0x82201268; continue 'dispatch;
	}
	// 82201234: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201238: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 8220123C: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82201240: 40980028  bge cr6, 0x82201268
	if !ctx.cr[6].lt {
	pc = 0x82201268; continue 'dispatch;
	}
	// 82201244: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82201248: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 8220124C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82201250: 40990018  ble cr6, 0x82201268
	if !ctx.cr[6].gt {
	pc = 0x82201268; continue 'dispatch;
	}
	// 82201254: 7D4AF9D6  mullw r10, r10, r31
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82201258: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8220125C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82201260: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82201264: 48000008  b 0x8220126c
	pc = 0x8220126C; continue 'dispatch;
	// 82201268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8220126C: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201270: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 82201274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82201278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8220127C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82201280: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82201284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82201288 size=344
    let mut pc: u32 = 0x82201288;
    'dispatch: loop {
        match pc {
            0x82201288 => {
    //   block [0x82201288..0x822013E0)
	// 82201288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220128C: 48333E21  bl 0x825350ac
	ctx.lr = 0x82201290;
	sub_82535080(ctx, base);
	// 82201290: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201294: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82201298: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8220129C: 38EB3A04  addi r7, r11, 0x3a04
	ctx.r[7].s64 = ctx.r[11].s64 + 14852;
	// 822012A0: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 822012A4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 822012A8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 822012AC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 822012B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822012B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822012B8: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 822012BC: FB9B0000  std r28, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[28].u64 ) };
	// 822012C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822012C4: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 822012C8: 481691C9  bl 0x8236a490
	ctx.lr = 0x822012CC;
	sub_8236A490(ctx, base);
	// 822012CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822012D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822012D4: 419A000C  beq cr6, 0x822012e0
	if ctx.cr[6].eq {
	pc = 0x822012E0; continue 'dispatch;
	}
	// 822012D8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 822012DC: 48000008  b 0x822012e4
	pc = 0x822012E4; continue 'dispatch;
	// 822012E0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 822012E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822012E8: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822012EC: 93FB0004  stw r31, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 822012F0: 419A00E4  beq cr6, 0x822013d4
	if ctx.cr[6].eq {
	pc = 0x822013D4; continue 'dispatch;
	}
	// 822012F4: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 822012F8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822012FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82201300: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201304: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201308: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8220130C: 4E800421  bctrl
	ctx.lr = 0x82201310;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201310: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82201314: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201318: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8220131C: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82201320: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82201324: 419A0024  beq cr6, 0x82201348
	if ctx.cr[6].eq {
	pc = 0x82201348; continue 'dispatch;
	}
	// 82201328: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8220132C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82201330: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82201334: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82201338: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8220133C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82201340: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82201344: 4082FFE8  bne 0x8220132c
	if !ctx.cr[0].eq {
	pc = 0x8220132C; continue 'dispatch;
	}
	// 82201348: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8220134C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82201350: 419A0008  beq cr6, 0x82201358
	if ctx.cr[6].eq {
	pc = 0x82201358; continue 'dispatch;
	}
	// 82201354: 4BF1BD05  bl 0x8211d058
	ctx.lr = 0x82201358;
	sub_8211D058(ctx, base);
	// 82201358: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8220135C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82201360: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82201364: 419A0024  beq cr6, 0x82201388
	if ctx.cr[6].eq {
	pc = 0x82201388; continue 'dispatch;
	}
	// 82201368: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8220136C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82201370: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82201374: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82201378: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8220137C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82201380: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82201384: 4082FFE8  bne 0x8220136c
	if !ctx.cr[0].eq {
	pc = 0x8220136C; continue 'dispatch;
	}
	// 82201388: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8220138C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201390: 480D7261  bl 0x822d85f0
	ctx.lr = 0x82201394;
	sub_822D85F0(ctx, base);
	// 82201394: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82201398: 397F0094  addi r11, r31, 0x94
	ctx.r[11].s64 = ctx.r[31].s64 + 148;
	// 8220139C: 394A3A0C  addi r10, r10, 0x3a0c
	ctx.r[10].s64 = ctx.r[10].s64 + 14860;
	// 822013A0: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 822013A4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 822013A8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822013AC: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822013B0: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 822013B4: 935F0090  stw r26, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[26].u32 ) };
	// 822013B8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822013BC: 4BF22ADD  bl 0x82123e98
	ctx.lr = 0x822013C0;
	sub_82123E98(ctx, base);
	// 822013C0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822013C4: 933F009C  stw r25, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[25].u32 ) };
	// 822013C8: 419A000C  beq cr6, 0x822013d4
	if ctx.cr[6].eq {
	pc = 0x822013D4; continue 'dispatch;
	}
	// 822013CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822013D0: 4BF1BC89  bl 0x8211d058
	ctx.lr = 0x822013D4;
	sub_8211D058(ctx, base);
	// 822013D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822013D8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822013DC: 48333D20  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822013E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822013E0 size=124
    let mut pc: u32 = 0x822013E0;
    'dispatch: loop {
        match pc {
            0x822013E0 => {
    //   block [0x822013E0..0x8220145C)
	// 822013E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822013E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822013E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822013EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822013F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822013F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822013F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822013FC: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82201400: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82201404: 419A0008  beq cr6, 0x8220140c
	if ctx.cr[6].eq {
	pc = 0x8220140C; continue 'dispatch;
	}
	// 82201408: 4BF1BC51  bl 0x8211d058
	ctx.lr = 0x8220140C;
	sub_8211D058(ctx, base);
	// 8220140C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201410: 480D72D9  bl 0x822d86e8
	ctx.lr = 0x82201414;
	sub_822D86E8(ctx, base);
	// 82201414: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82201418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8220141C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201420: 419A0024  beq cr6, 0x82201444
	if ctx.cr[6].eq {
	pc = 0x82201444; continue 'dispatch;
	}
	// 82201424: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82201428: 816BFAC0  lwz r11, -0x540(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8220142C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82201430: 409A0014  bne cr6, 0x82201444
	if !ctx.cr[6].eq {
	pc = 0x82201444; continue 'dispatch;
	}
	// 82201434: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82201438: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8220143C: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82201440: 995F0025  stb r10, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[10].u8 ) };
	// 82201444: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82201448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8220144C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82201450: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82201454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82201458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82201460 size=576
    let mut pc: u32 = 0x82201460;
    'dispatch: loop {
        match pc {
            0x82201460 => {
    //   block [0x82201460..0x822016A0)
	// 82201460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82201464: 48333C51  bl 0x825350b4
	ctx.lr = 0x82201468;
	sub_82535080(ctx, base);
	// 82201468: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220146C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82201470: 3D608286  lis r11, -0x7d7a
	ctx.r[11].s64 = -2105147392;
	// 82201474: 396B76FC  addi r11, r11, 0x76fc
	ctx.r[11].s64 = ctx.r[11].s64 + 30460;
	// 82201478: 815D0090  lwz r10, 0x90(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 8220147C: 807D0094  lwz r3, 0x94(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 82201480: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82201484: 7F8A582E  lwzx r28, r10, r11
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82201488: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8220148C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82201490: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82201494: 3B6BBABC  addi r27, r11, -0x4544
	ctx.r[27].s64 = ctx.r[11].s64 + -17732;
	// 82201498: 419A000C  beq cr6, 0x822014a4
	if ctx.cr[6].eq {
	pc = 0x822014A4; continue 'dispatch;
	}
	// 8220149C: 4BFF2FCD  bl 0x821f4468
	ctx.lr = 0x822014A0;
	sub_821F4468(ctx, base);
	// 822014A0: 48000008  b 0x822014a8
	pc = 0x822014A8; continue 'dispatch;
	// 822014A4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822014A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822014AC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822014B0: 4199000C  bgt cr6, 0x822014bc
	if ctx.cr[6].gt {
	pc = 0x822014BC; continue 'dispatch;
	}
	// 822014B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822014B8: 48000020  b 0x822014d8
	pc = 0x822014D8; continue 'dispatch;
	// 822014BC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822014C0: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822014C4: 7D4B5051  subf. r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822014C8: 4082000C  bne 0x822014d4
	if !ctx.cr[0].eq {
	pc = 0x822014D4; continue 'dispatch;
	}
	// 822014CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822014D0: 48000008  b 0x822014d8
	pc = 0x822014D8; continue 'dispatch;
	// 822014D4: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 822014D8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 822014DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822014E0: 48332BB1  bl 0x82534090
	ctx.lr = 0x822014E4;
	sub_82534090(ctx, base);
	// 822014E4: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 822014E8: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 822014EC: 409A00F0  bne cr6, 0x822015dc
	if !ctx.cr[6].eq {
	pc = 0x822015DC; continue 'dispatch;
	}
	// 822014F0: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 822014F4: 614ABEC8  ori r10, r10, 0xbec8
	ctx.r[10].u64 = ctx.r[10].u64 | 48840;
	// 822014F8: 7D7C532E  sthx r11, r28, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u16) };
	// 822014FC: 807D0094  lwz r3, 0x94(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 82201500: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82201504: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82201508: 419A000C  beq cr6, 0x82201514
	if ctx.cr[6].eq {
	pc = 0x82201514; continue 'dispatch;
	}
	// 8220150C: 4BFF2F5D  bl 0x821f4468
	ctx.lr = 0x82201510;
	sub_821F4468(ctx, base);
	// 82201510: 48000008  b 0x82201518
	pc = 0x82201518; continue 'dispatch;
	// 82201514: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82201518: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220151C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82201520: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82201524: 40990018  ble cr6, 0x8220153c
	if !ctx.cr[6].gt {
	pc = 0x8220153C; continue 'dispatch;
	}
	// 82201528: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220152C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82201530: 7D4B5051  subf. r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82201534: 41820008  beq 0x8220153c
	if ctx.cr[0].eq {
	pc = 0x8220153C; continue 'dispatch;
	}
	// 82201538: 7C8B1A14  add r4, r11, r3
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8220153C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82201540: 4815CEA1  bl 0x8235e3e0
	ctx.lr = 0x82201544;
	sub_8235E3E0(ctx, base);
	// 82201544: 83FD0094  lwz r31, 0x94(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 82201548: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8220154C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82201550: 419A0010  beq cr6, 0x82201560
	if ctx.cr[6].eq {
	pc = 0x82201560; continue 'dispatch;
	}
	// 82201554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201558: 4BFF2F11  bl 0x821f4468
	ctx.lr = 0x8220155C;
	sub_821F4468(ctx, base);
	// 8220155C: 48000008  b 0x82201564
	pc = 0x82201564; continue 'dispatch;
	// 82201560: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82201564: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201568: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8220156C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82201570: 40990018  ble cr6, 0x82201588
	if !ctx.cr[6].gt {
	pc = 0x82201588; continue 'dispatch;
	}
	// 82201574: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82201578: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220157C: 7D4B5051  subf. r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82201580: 41820008  beq 0x82201588
	if ctx.cr[0].eq {
	pc = 0x82201588; continue 'dispatch;
	}
	// 82201584: 7FCB1A14  add r30, r11, r3
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82201588: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8220158C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82201590: 419A0010  beq cr6, 0x822015a0
	if ctx.cr[6].eq {
	pc = 0x822015A0; continue 'dispatch;
	}
	// 82201594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201598: 4BFF2ED1  bl 0x821f4468
	ctx.lr = 0x8220159C;
	sub_821F4468(ctx, base);
	// 8220159C: 48000008  b 0x822015a4
	pc = 0x822015A4; continue 'dispatch;
	// 822015A0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822015A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822015A8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 822015AC: 4199000C  bgt cr6, 0x822015b8
	if ctx.cr[6].gt {
	pc = 0x822015B8; continue 'dispatch;
	}
	// 822015B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822015B4: 48000010  b 0x822015c4
	pc = 0x822015C4; continue 'dispatch;
	// 822015B8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 822015BC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822015C0: 7CAA5850  subf r5, r10, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 822015C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822015C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822015CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822015D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822015D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822015D8: 4BFB0451  bl 0x821b1a28
	ctx.lr = 0x822015DC;
	sub_821B1A28(ctx, base);
	// 822015DC: 807D0094  lwz r3, 0x94(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 822015E0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 822015E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822015E8: 419A000C  beq cr6, 0x822015f4
	if ctx.cr[6].eq {
	pc = 0x822015F4; continue 'dispatch;
	}
	// 822015EC: 4BFF2E7D  bl 0x821f4468
	ctx.lr = 0x822015F0;
	sub_821F4468(ctx, base);
	// 822015F0: 48000008  b 0x822015f8
	pc = 0x822015F8; continue 'dispatch;
	// 822015F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822015F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822015FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82201600: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82201604: 40990018  ble cr6, 0x8220161c
	if !ctx.cr[6].gt {
	pc = 0x8220161C; continue 'dispatch;
	}
	// 82201608: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8220160C: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82201610: 7D4B5051  subf. r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82201614: 41820008  beq 0x8220161c
	if ctx.cr[0].eq {
	pc = 0x8220161C; continue 'dispatch;
	}
	// 82201618: 7C8B1A14  add r4, r11, r3
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8220161C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82201620: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82201624: 4815E8FD  bl 0x8235ff20
	ctx.lr = 0x82201628;
	sub_8235FF20(ctx, base);
	// 82201628: 83FD009C  lwz r31, 0x9c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(156 as u32) ) } as u64;
	// 8220162C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82201630: 419A0064  beq cr6, 0x82201694
	if ctx.cr[6].eq {
	pc = 0x82201694; continue 'dispatch;
	}
	// 82201634: 807D0094  lwz r3, 0x94(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 82201638: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8220163C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82201640: 419A000C  beq cr6, 0x8220164c
	if ctx.cr[6].eq {
	pc = 0x8220164C; continue 'dispatch;
	}
	// 82201644: 4BFF2E25  bl 0x821f4468
	ctx.lr = 0x82201648;
	sub_821F4468(ctx, base);
	// 82201648: 48000008  b 0x82201650
	pc = 0x82201650; continue 'dispatch;
	// 8220164C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82201650: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82201654: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82201658: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8220165C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82201660: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82201664: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82201668: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8220166C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82201670: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82201674: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82201678: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8220167C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82201680: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82201684: 480DE055  bl 0x822df6d8
	ctx.lr = 0x82201688;
	sub_822DF6D8(ctx, base);
	// 82201688: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220168C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82201690: 419A0008  beq cr6, 0x82201698
	if ctx.cr[6].eq {
	pc = 0x82201698; continue 'dispatch;
	}
	// 82201694: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82201698: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8220169C: 48333A68  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822016A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822016A0 size=16
    let mut pc: u32 = 0x822016A0;
    'dispatch: loop {
        match pc {
            0x822016A0 => {
    //   block [0x822016A0..0x822016B0)
	// 822016A0: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 822016A4: 396B3F00  addi r11, r11, 0x3f00
	ctx.r[11].s64 = ctx.r[11].s64 + 16128;
	// 822016A8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 822016AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822016B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822016B0 size=8
    let mut pc: u32 = 0x822016B0;
    'dispatch: loop {
        match pc {
            0x822016B0 => {
    //   block [0x822016B0..0x822016B8)
	// 822016B0: 386000EC  li r3, 0xec
	ctx.r[3].s64 = 236;
	// 822016B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822016B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822016B8 size=128
    let mut pc: u32 = 0x822016B8;
    'dispatch: loop {
        match pc {
            0x822016B8 => {
    //   block [0x822016B8..0x82201738)
	// 822016B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822016BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822016C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822016C4: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822016C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822016CC: 409A000C  bne cr6, 0x822016d8
	if !ctx.cr[6].eq {
	pc = 0x822016D8; continue 'dispatch;
	}
	// 822016D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822016D4: 4800002C  b 0x82201700
	pc = 0x82201700; continue 'dispatch;
	// 822016D8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822016DC: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 822016E0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822016E4: 419A000C  beq cr6, 0x822016f0
	if ctx.cr[6].eq {
	pc = 0x822016F0; continue 'dispatch;
	}
	// 822016E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822016EC: 48000014  b 0x82201700
	pc = 0x82201700; continue 'dispatch;
	// 822016F0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822016F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822016F8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822016FC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82201700: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82201704: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82201708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8220170C: 419A0008  beq cr6, 0x82201714
	if ctx.cr[6].eq {
	pc = 0x82201714; continue 'dispatch;
	}
	// 82201710: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201714: 48110595  bl 0x82311ca8
	ctx.lr = 0x82201718;
	sub_82311CA8(ctx, base);
	// 82201718: 48110909  bl 0x82312020
	ctx.lr = 0x8220171C;
	sub_82312020(ctx, base);
	// 8220171C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82201720: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82201724: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82201728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220172C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82201730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82201734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82201738 size=44
    let mut pc: u32 = 0x82201738;
    'dispatch: loop {
        match pc {
            0x82201738 => {
    //   block [0x82201738..0x82201764)
	// 82201738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220173C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82201740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201744: 481108DD  bl 0x82312020
	ctx.lr = 0x82201748;
	sub_82312020(ctx, base);
	// 82201748: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8220174C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82201750: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82201754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82201758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8220175C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82201760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201768 size=16
    let mut pc: u32 = 0x82201768;
    'dispatch: loop {
        match pc {
            0x82201768 => {
    //   block [0x82201768..0x82201778)
	// 82201768: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8220176C: 396B3F00  addi r11, r11, 0x3f00
	ctx.r[11].s64 = ctx.r[11].s64 + 16128;
	// 82201770: 386B0058  addi r3, r11, 0x58
	ctx.r[3].s64 = ctx.r[11].s64 + 88;
	// 82201774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201778 size=8
    let mut pc: u32 = 0x82201778;
    'dispatch: loop {
        match pc {
            0x82201778 => {
    //   block [0x82201778..0x82201780)
	// 82201778: 3860000D  li r3, 0xd
	ctx.r[3].s64 = 13;
	// 8220177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201780 size=8
    let mut pc: u32 = 0x82201780;
    'dispatch: loop {
        match pc {
            0x82201780 => {
    //   block [0x82201780..0x82201788)
	// 82201780: 386000F1  li r3, 0xf1
	ctx.r[3].s64 = 241;
	// 82201784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201788 size=8
    let mut pc: u32 = 0x82201788;
    'dispatch: loop {
        match pc {
            0x82201788 => {
    //   block [0x82201788..0x82201790)
	// 82201788: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8220178C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82201790 size=412
    let mut pc: u32 = 0x82201790;
    'dispatch: loop {
        match pc {
            0x82201790 => {
    //   block [0x82201790..0x8220192C)
	// 82201790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82201794: 48333929  bl 0x825350bc
	ctx.lr = 0x82201798;
	sub_82535080(ctx, base);
	// 82201798: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220179C: 3D208288  lis r9, -0x7d78
	ctx.r[9].s64 = -2105016320;
	// 822017A0: 908100BC  stw r4, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[4].u32 ) };
	// 822017A4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 822017A8: 906100B4  stw r3, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[3].u32 ) };
	// 822017AC: 39298E40  addi r9, r9, -0x71c0
	ctx.r[9].s64 = ctx.r[9].s64 + -29120;
	// 822017B0: 90A100C4  stw r5, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[5].u32 ) };
	// 822017B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822017B8: 90C100CC  stw r6, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[6].u32 ) };
	// 822017BC: 39630090  addi r11, r3, 0x90
	ctx.r[11].s64 = ctx.r[3].s64 + 144;
	// 822017C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 822017C4: C00ABA38  lfs f0, -0x45c8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822017C8: 394300E0  addi r10, r3, 0xe0
	ctx.r[10].s64 = ctx.r[3].s64 + 224;
	// 822017CC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 822017D0: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 822017D4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 822017D8: 39293A48  addi r9, r9, 0x3a48
	ctx.r[9].s64 = ctx.r[9].s64 + 14920;
	// 822017DC: FBEB0020  std r31, 0x20(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u64 ) };
	// 822017E0: FBEB0028  std r31, 0x28(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[31].u64 ) };
	// 822017E4: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 822017E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822017EC: FBEB0030  std r31, 0x30(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[31].u64 ) };
	// 822017F0: FBEB0038  std r31, 0x38(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[31].u64 ) };
	// 822017F4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 822017F8: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 822017FC: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 82201800: 39293A7C  addi r9, r9, 0x3a7c
	ctx.r[9].s64 = ctx.r[9].s64 + 14972;
	// 82201804: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82201808: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8220180C: F94B0010  std r10, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82201810: F94B0018  std r10, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 82201814: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82201818: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 8220181C: B3EB0044  sth r31, 0x44(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[31].u16 ) };
	// 82201820: B3EB0046  sth r31, 0x46(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(70 as u32), ctx.r[31].u16 ) };
	// 82201824: F9410068  std r10, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u64 ) };
	// 82201828: F9410070  std r10, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u64 ) };
	// 8220182C: 83C100B4  lwz r30, 0xb4(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82201830: 812100C4  lwz r9, 0xc4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82201834: 387E0130  addi r3, r30, 0x130
	ctx.r[3].s64 = ctx.r[30].s64 + 304;
	// 82201838: A15E0014  lhz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220183C: D01E008C  stfs f0, 0x8c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82201840: 93BE0084  stw r29, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 82201844: 913E0080  stw r9, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 82201848: 510A1EDE  rlwimi r10, r8, 3, 0x1b, 0xf
	ctx.r[10].u64 = (((ctx.r[8].u32).rotate_left(3) as u64) & 0xFFFFFFFFFFFF001F) | (ctx.r[10].u64 & 0x000000000000FFE0);
	// 8220184C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82201850: 9BBE001D  stb r29, 0x1d(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(29 as u32), ctx.r[29].u8 ) };
	// 82201854: 9BBE001E  stb r29, 0x1e(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(30 as u32), ctx.r[29].u8 ) };
	// 82201858: B15E0014  sth r10, 0x14(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 8220185C: 913E00D0  stw r9, 0xd0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(208 as u32), ctx.r[9].u32 ) };
	// 82201860: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82201864: 93EB004C  stw r31, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[31].u32 ) };
	// 82201868: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8220186C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82201870: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82201874: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82201878: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8220187C: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82201880: B3EB0008  sth r31, 8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u16 ) };
	// 82201884: B3EB000A  sth r31, 0xa(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[31].u16 ) };
	// 82201888: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8220188C: B3EB0010  sth r31, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u16 ) };
	// 82201890: B3EB0012  sth r31, 0x12(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[31].u16 ) };
	// 82201894: 93EB0014  stw r31, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82201898: B3EB0018  sth r31, 0x18(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u16 ) };
	// 8220189C: B3EB001A  sth r31, 0x1a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(26 as u32), ctx.r[31].u16 ) };
	// 822018A0: 93EB001C  stw r31, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 822018A4: B3EB0020  sth r31, 0x20(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u16 ) };
	// 822018A8: B3EB0022  sth r31, 0x22(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(34 as u32), ctx.r[31].u16 ) };
	// 822018AC: 93EB0024  stw r31, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 822018B0: B3EB0028  sth r31, 0x28(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[31].u16 ) };
	// 822018B4: B3EB002A  sth r31, 0x2a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(42 as u32), ctx.r[31].u16 ) };
	// 822018B8: 48009DE1  bl 0x8220b698
	ctx.lr = 0x822018BC;
	sub_8220B698(ctx, base);
	// 822018BC: 812100BC  lwz r9, 0xbc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 822018C0: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 822018C4: 3D408287  lis r10, -0x7d79
	ctx.r[10].s64 = -2105081856;
	// 822018C8: 93BE0110  stw r29, 0x110(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(272 as u32), ctx.r[29].u32 ) };
	// 822018CC: 396B8E38  addi r11, r11, -0x71c8
	ctx.r[11].s64 = ctx.r[11].s64 + -29128;
	// 822018D0: 394A7790  addi r10, r10, 0x7790
	ctx.r[10].s64 = ctx.r[10].s64 + 30608;
	// 822018D4: 913E010C  stw r9, 0x10c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(268 as u32), ctx.r[9].u32 ) };
	// 822018D8: 812100CC  lwz r9, 0xcc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 822018DC: 913E0088  stw r9, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[9].u32 ) };
	// 822018E0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 822018E4: 917E00E4  stw r11, 0xe4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 822018E8: B3BE00E8  sth r29, 0xe8(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(232 as u32), ctx.r[29].u16 ) };
	// 822018EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822018F0: B3FE00EA  sth r31, 0xea(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(234 as u32), ctx.r[31].u16 ) };
	// 822018F4: 917E00EC  stw r11, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 822018F8: B3FE00F2  sth r31, 0xf2(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(242 as u32), ctx.r[31].u16 ) };
	// 822018FC: B13E00F0  sth r9, 0xf0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(240 as u32), ctx.r[9].u16 ) };
	// 82201900: 915E00F4  stw r10, 0xf4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 82201904: B3BE00F8  sth r29, 0xf8(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(248 as u32), ctx.r[29].u16 ) };
	// 82201908: B3FE00FA  sth r31, 0xfa(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(250 as u32), ctx.r[31].u16 ) };
	// 8220190C: 93FE00FC  stw r31, 0xfc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(252 as u32), ctx.r[31].u32 ) };
	// 82201910: B3BE0100  sth r29, 0x100(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(256 as u32), ctx.r[29].u16 ) };
	// 82201914: B3FE0102  sth r31, 0x102(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(258 as u32), ctx.r[31].u16 ) };
	// 82201918: 93FE0104  stw r31, 0x104(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8220191C: B13E0108  sth r9, 0x108(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(264 as u32), ctx.r[9].u16 ) };
	// 82201920: B3FE010A  sth r31, 0x10a(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(266 as u32), ctx.r[31].u16 ) };
	// 82201924: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82201928: 483337E4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201930 size=20
    let mut pc: u32 = 0x82201930;
    'dispatch: loop {
        match pc {
            0x82201930 => {
    //   block [0x82201930..0x82201944)
	// 82201930: 8063010C  lwz r3, 0x10c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) } as u64;
	// 82201934: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201938: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220193C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201940: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82201948 size=316
    let mut pc: u32 = 0x82201948;
    'dispatch: loop {
        match pc {
            0x82201948 => {
    //   block [0x82201948..0x82201A84)
	// 82201948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220194C: 48333769  bl 0x825350b4
	ctx.lr = 0x82201950;
	sub_82535080(ctx, base);
	// 82201950: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201954: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82201958: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220195C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82201960: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82201964: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201968: 837F0088  lwz r27, 0x88(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8220196C: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 82201970: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82201974: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82201978: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 8220197C: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 82201980: 99410053  stb r10, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[10].u8 ) };
	// 82201984: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82201988: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 8220198C: 3940000D  li r10, 0xd
	ctx.r[10].s64 = 13;
	// 82201990: 99410055  stb r10, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[10].u8 ) };
	// 82201994: 3940000E  li r10, 0xe
	ctx.r[10].s64 = 14;
	// 82201998: 99410056  stb r10, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u8 ) };
	// 8220199C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 822019A0: 99410057  stb r10, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[10].u8 ) };
	// 822019A4: 39400011  li r10, 0x11
	ctx.r[10].s64 = 17;
	// 822019A8: 99410058  stb r10, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u8 ) };
	// 822019AC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822019B0: 99410059  stb r10, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[10].u8 ) };
	// 822019B4: 39400015  li r10, 0x15
	ctx.r[10].s64 = 21;
	// 822019B8: 9941005A  stb r10, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[10].u8 ) };
	// 822019BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822019C0: 4E800421  bctrl
	ctx.lr = 0x822019C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822019C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822019C8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822019CC: 419A00B0  beq cr6, 0x82201a7c
	if ctx.cr[6].eq {
	pc = 0x82201A7C; continue 'dispatch;
	}
	// 822019D0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 822019D4: 815F0110  lwz r10, 0x110(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 822019D8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822019DC: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 822019E0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 822019E4: 555CDFFE  rlwinm r28, r10, 0x1b, 0x1f, 0x1f
	ctx.r[28].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 822019E8: C00B203C  lfs f0, 0x203c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8252 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822019EC: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 822019F0: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 822019F4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822019F8: C01E00F0  lfs f0, 0xf0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822019FC: 3BABB620  addi r29, r11, -0x49e0
	ctx.r[29].s64 = ctx.r[11].s64 + -18912;
	// 82201A00: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82201A04: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82201A08: C01E00F4  lfs f0, 0xf4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(244 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82201A0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82201A10: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82201A14: C01E00F8  lfs f0, 0xf8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(248 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82201A18: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82201A1C: 48182A1D  bl 0x82384438
	ctx.lr = 0x82201A20;
	sub_82384438(ctx, base);
	// 82201A20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82201A24: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82201A28: C00BBFFC  lfs f0, -0x4004(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82201A2C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82201A30: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82201A34: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82201A38: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82201A3C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82201A40: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82201A44: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82201A48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82201A4C: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 82201A50: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82201A54: C00B0030  lfs f0, 0x30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82201A58: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82201A5C: C00B0034  lfs f0, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82201A60: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82201A64: C00B0038  lfs f0, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82201A68: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82201A6C: 481829CD  bl 0x82384438
	ctx.lr = 0x82201A70;
	sub_82384438(ctx, base);
	// 82201A70: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82201A74: 2B1F000B  cmplwi cr6, r31, 0xb
	ctx.cr[6].compare_u32(ctx.r[31].u32, 11 as u32, &mut ctx.xer);
	// 82201A78: 4198FFB8  blt cr6, 0x82201a30
	if ctx.cr[6].lt {
	pc = 0x82201A30; continue 'dispatch;
	}
	// 82201A7C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82201A80: 48333684  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201A88 size=4
    let mut pc: u32 = 0x82201A88;
    'dispatch: loop {
        match pc {
            0x82201A88 => {
    //   block [0x82201A88..0x82201A8C)
	// 82201A88: 480026C0  b 0x82204148
	sub_82204148(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82201A90 size=568
    let mut pc: u32 = 0x82201A90;
    'dispatch: loop {
        match pc {
            0x82201A90 => {
    //   block [0x82201A90..0x82201CC8)
	// 82201A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82201A94: 48333621  bl 0x825350b4
	ctx.lr = 0x82201A98;
	sub_82535080(ctx, base);
	// 82201A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201A9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82201AA0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82201AA4: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82201AA8: 817F0110  lwz r11, 0x110(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 82201AAC: B3BF00FA  sth r29, 0xfa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(250 as u32), ctx.r[29].u16 ) };
	// 82201AB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201AB4: 409A0010  bne cr6, 0x82201ac4
	if !ctx.cr[6].eq {
	pc = 0x82201AC4; continue 'dispatch;
	}
	// 82201AB8: 93BF00F4  stw r29, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[29].u32 ) };
	// 82201ABC: B3BF00F8  sth r29, 0xf8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[29].u16 ) };
	// 82201AC0: 4800005C  b 0x82201b1c
	pc = 0x82201B1C; continue 'dispatch;
	// 82201AC4: 3D608287  lis r11, -0x7d79
	ctx.r[11].s64 = -2105081856;
	// 82201AC8: B39F00F8  sth r28, 0xf8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[28].u16 ) };
	// 82201ACC: 396B7790  addi r11, r11, 0x7790
	ctx.r[11].s64 = ctx.r[11].s64 + 30608;
	// 82201AD0: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 82201AD4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82201AD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201ADC: 419A003C  beq cr6, 0x82201b18
	if ctx.cr[6].eq {
	pc = 0x82201B18; continue 'dispatch;
	}
	// 82201AE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201AE8: 83DF0088  lwz r30, 0x88(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82201AEC: 837F0118  lwz r27, 0x118(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 82201AF0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82201AF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201AF8: 4E800421  bctrl
	ctx.lr = 0x82201AFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201AFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82201B00: 387F0130  addi r3, r31, 0x130
	ctx.r[3].s64 = ctx.r[31].s64 + 304;
	// 82201B04: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82201B08: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82201B0C: 48009D1D  bl 0x8220b828
	ctx.lr = 0x82201B10;
	sub_8220B828(ctx, base);
	// 82201B10: 907F0114  stw r3, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[3].u32 ) };
	// 82201B14: 48000008  b 0x82201b1c
	pc = 0x82201B1C; continue 'dispatch;
	// 82201B18: 93BF0114  stw r29, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[29].u32 ) };
	// 82201B1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201B20: 48002751  bl 0x82204270
	ctx.lr = 0x82201B24;
	sub_82204270(ctx, base);
	// 82201B24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82201B28: 409A0024  bne cr6, 0x82201b4c
	if !ctx.cr[6].eq {
	pc = 0x82201B4C; continue 'dispatch;
	}
	// 82201B2C: 817F011C  lwz r11, 0x11c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 82201B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201B34: 419A0018  beq cr6, 0x82201b4c
	if ctx.cr[6].eq {
	pc = 0x82201B4C; continue 'dispatch;
	}
	// 82201B38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201B40: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82201B44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201B48: 4E800421  bctrl
	ctx.lr = 0x82201B4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201B4C: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82201B50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82201B54: 419A0024  beq cr6, 0x82201b78
	if ctx.cr[6].eq {
	pc = 0x82201B78; continue 'dispatch;
	}
	// 82201B58: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82201B5C: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82201B60: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82201B64: 409A0018  bne cr6, 0x82201b7c
	if !ctx.cr[6].eq {
	pc = 0x82201B7C; continue 'dispatch;
	}
	// 82201B68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82201B6C: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82201B70: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82201B74: 4198FFE8  blt cr6, 0x82201b5c
	if ctx.cr[6].lt {
	pc = 0x82201B5C; continue 'dispatch;
	}
	// 82201B78: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82201B7C: 3D40830F  lis r10, -0x7cf1
	ctx.r[10].s64 = -2096168960;
	// 82201B80: 1D6B03C0  mulli r11, r11, 0x3c0
	ctx.r[11].s64 = ctx.r[11].s64 * 960;
	// 82201B84: 392A4A00  addi r9, r10, 0x4a00
	ctx.r[9].s64 = ctx.r[10].s64 + 18944;
	// 82201B88: 394900B0  addi r10, r9, 0xb0
	ctx.r[10].s64 = ctx.r[9].s64 + 176;
	// 82201B8C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82201B90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201B94: 419A0008  beq cr6, 0x82201b9c
	if ctx.cr[6].eq {
	pc = 0x82201B9C; continue 'dispatch;
	}
	// 82201B98: 9B8B0010  stb r28, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[28].u8 ) };
	// 82201B9C: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82201BA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201BA4: 419A0014  beq cr6, 0x82201bb8
	if ctx.cr[6].eq {
	pc = 0x82201BB8; continue 'dispatch;
	}
	// 82201BA8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82201BAC: 216B001F  subfic r11, r11, 0x1f
	ctx.xer.ca = ctx.r[11].u32 <= 31 as u32;
	ctx.r[11].s64 = (31 as i64) - ctx.r[11].s64;
	// 82201BB0: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82201BB4: 41980008  blt cr6, 0x82201bbc
	if ctx.cr[6].lt {
	pc = 0x82201BBC; continue 'dispatch;
	}
	// 82201BB8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82201BBC: 1D6B03C0  mulli r11, r11, 0x3c0
	ctx.r[11].s64 = ctx.r[11].s64 * 960;
	// 82201BC0: 394900B0  addi r10, r9, 0xb0
	ctx.r[10].s64 = ctx.r[9].s64 + 176;
	// 82201BC4: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82201BC8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82201BCC: 419A00F4  beq cr6, 0x82201cc0
	if ctx.cr[6].eq {
	pc = 0x82201CC0; continue 'dispatch;
	}
	// 82201BD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201BD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201BD8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82201BDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201BE0: 4E800421  bctrl
	ctx.lr = 0x82201BE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201BE4: C04303B8  lfs f2, 0x3b8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(952 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82201BE8: C02303B0  lfs f1, 0x3b0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(944 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82201BEC: 4BFB5C75  bl 0x821b7860
	ctx.lr = 0x82201BF0;
	sub_821B7860(ctx, base);
	// 82201BF0: 817F0120  lwz r11, 0x120(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 82201BF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201BF8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82201BFC: 394BD6D0  addi r10, r11, -0x2930
	ctx.r[10].s64 = ctx.r[11].s64 + -10544;
	// 82201C00: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82201C04: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82201C08: 409A0014  bne cr6, 0x82201c1c
	if !ctx.cr[6].eq {
	pc = 0x82201C1C; continue 'dispatch;
	}
	// 82201C0C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82201C10: 40980020  bge cr6, 0x82201c30
	if !ctx.cr[6].lt {
	pc = 0x82201C30; continue 'dispatch;
	}
	// 82201C14: 939F0120  stw r28, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[28].u32 ) };
	// 82201C18: 48000014  b 0x82201c2c
	pc = 0x82201C2C; continue 'dispatch;
	// 82201C1C: C1AAFBBC  lfs f13, -0x444(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-1092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82201C20: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 82201C24: 4099000C  ble cr6, 0x82201c30
	if !ctx.cr[6].gt {
	pc = 0x82201C30; continue 'dispatch;
	}
	// 82201C28: 93BF0120  stw r29, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[29].u32 ) };
	// 82201C2C: D01F0124  stfs f0, 0x124(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 82201C30: 817F0120  lwz r11, 0x120(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 82201C34: C19F0124  lfs f12, 0x124(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82201C38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201C3C: 419A0018  beq cr6, 0x82201c54
	if ctx.cr[6].eq {
	pc = 0x82201C54; continue 'dispatch;
	}
	// 82201C40: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82201C44: C1AAFB48  lfs f13, -0x4b8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-1208 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82201C48: EC0C037C  fnmsubs f0, f12, f13, f0
	ctx.f[0].f64 = -(((ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82201C4C: C16B2BCC  lfs f11, 0x2bcc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82201C50: 48000014  b 0x82201c64
	pc = 0x82201C64; continue 'dispatch;
	// 82201C54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82201C58: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82201C5C: EC0C037A  fmadds f0, f12, f13, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82201C60: C16B22BC  lfs f11, 0x22bc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8892 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82201C64: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82201C68: 397E0360  addi r11, r30, 0x360
	ctx.r[11].s64 = ctx.r[30].s64 + 864;
	// 82201C6C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82201C70: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82201C74: 40980008  bge cr6, 0x82201c7c
	if !ctx.cr[6].lt {
	pc = 0x82201C7C; continue 'dispatch;
	}
	// 82201C78: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82201C7C: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82201C80: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82201C84: 40990008  ble cr6, 0x82201c8c
	if !ctx.cr[6].gt {
	pc = 0x82201C8C; continue 'dispatch;
	}
	// 82201C88: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82201C8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82201C90: C1BF0124  lfs f13, 0x124(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82201C94: C18B1FF8  lfs f12, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82201C98: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82201C9C: 40990024  ble cr6, 0x82201cc0
	if !ctx.cr[6].gt {
	pc = 0x82201CC0; continue 'dispatch;
	}
	// 82201CA0: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 82201CA4: C15F008C  lfs f10, 0x8c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82201CA8: 396B0C40  addi r11, r11, 0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + 3136;
	// 82201CAC: C00B06A4  lfs f0, 0x6a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1700 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82201CB0: EC0A0032  fmuls f0, f10, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82201CB4: EC006AFC  fnmsubs f0, f0, f11, f13
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82201CB8: FC00602E  fsel f0, f0, f0, f12
	ctx.f[0].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[12].f64 };
	// 82201CBC: D01F0124  stfs f0, 0x124(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 82201CC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82201CC4: 48333440  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82201CC8 size=396
    let mut pc: u32 = 0x82201CC8;
    'dispatch: loop {
        match pc {
            0x82201CC8 => {
    //   block [0x82201CC8..0x82201E54)
	// 82201CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82201CCC: 483333DD  bl 0x825350a8
	ctx.lr = 0x82201CD0;
	sub_82535080(ctx, base);
	// 82201CD0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201CD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82201CD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82201CDC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82201CE0: 817E0114  lwz r11, 0x114(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(276 as u32) ) } as u64;
	// 82201CE4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201CE8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82201CEC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82201CF0: 697A0001  xori r26, r11, 1
	ctx.r[26].u64 = ctx.r[11].u64 ^ 1;
	// 82201CF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201CFC: 419A0014  beq cr6, 0x82201d10
	if ctx.cr[6].eq {
	pc = 0x82201D10; continue 'dispatch;
	}
	// 82201D00: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82201D04: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82201D08: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82201D0C: 419A0008  beq cr6, 0x82201d14
	if ctx.cr[6].eq {
	pc = 0x82201D14; continue 'dispatch;
	}
	// 82201D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82201D14: A16B001E  lhz r11, 0x1e(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(30 as u32) ) } as u64;
	// 82201D18: 2B0B3D3D  cmplwi cr6, r11, 0x3d3d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15677 as u32, &mut ctx.xer);
	// 82201D1C: 409A0014  bne cr6, 0x82201d30
	if !ctx.cr[6].eq {
	pc = 0x82201D30; continue 'dispatch;
	}
	// 82201D20: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82201D24: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82201D28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82201D2C: 41980008  blt cr6, 0x82201d34
	if ctx.cr[6].lt {
	pc = 0x82201D34; continue 'dispatch;
	}
	// 82201D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82201D34: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 82201D38: 3BA000FF  li r29, 0xff
	ctx.r[29].s64 = 255;
	// 82201D3C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82201D40: 419A0060  beq cr6, 0x82201da0
	if ctx.cr[6].eq {
	pc = 0x82201DA0; continue 'dispatch;
	}
	// 82201D44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201D48: 419A0050  beq cr6, 0x82201d98
	if ctx.cr[6].eq {
	pc = 0x82201D98; continue 'dispatch;
	}
	// 82201D4C: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 82201D50: A0BF0006  lhz r5, 6(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82201D54: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82201D58: A09F0004  lhz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82201D5C: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82201D60: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82201D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82201D68: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82201D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82201D70: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82201D74: 48169E1D  bl 0x8236bb90
	ctx.lr = 0x82201D78;
	sub_8236BB90(ctx, base);
	// 82201D78: 817E013C  lwz r11, 0x13c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(316 as u32) ) } as u64;
	// 82201D7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82201D80: 7D495836  sld r9, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = (ctx.r[10].u64) << ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 82201D84: 3D60829E  lis r11, -0x7d62
	ctx.r[11].s64 = -2103574528;
	// 82201D88: E94B6410  ld r10, 0x6410(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(25616 as u32) ) };
	// 82201D8C: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82201D90: F94B6410  std r10, 0x6410(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(25616 as u32), ctx.r[10].u64 ) };
	// 82201D94: 48000020  b 0x82201db4
	pc = 0x82201DB4; continue 'dispatch;
	// 82201D98: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82201D9C: 48000018  b 0x82201db4
	pc = 0x82201DB4; continue 'dispatch;
	// 82201DA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82201DA4: 419A0010  beq cr6, 0x82201db4
	if ctx.cr[6].eq {
	pc = 0x82201DB4; continue 'dispatch;
	}
	// 82201DA8: 3D40829E  lis r10, -0x7d62
	ctx.r[10].s64 = -2103574528;
	// 82201DAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82201DB0: F96A6410  std r11, 0x6410(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(25616 as u32), ctx.r[11].u64 ) };
	// 82201DB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201DB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82201DBC: 839E0088  lwz r28, 0x88(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82201DC0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82201DC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201DC8: 4E800421  bctrl
	ctx.lr = 0x82201DCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201DCC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201DD0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82201DD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82201DD8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82201DDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201DE0: 4E800421  bctrl
	ctx.lr = 0x82201DE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201DE4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82201DE8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82201DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201DF0: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82201DF4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82201DF8: 48007F21  bl 0x82209d18
	ctx.lr = 0x82201DFC;
	sub_82209D18(ctx, base);
	// 82201DFC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82201E00: 419A004C  beq cr6, 0x82201e4c
	if ctx.cr[6].eq {
	pc = 0x82201E4C; continue 'dispatch;
	}
	// 82201E04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82201E08: A0BF0006  lhz r5, 6(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82201E0C: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82201E10: A09F0004  lhz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82201E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82201E18: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82201E20: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82201E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82201E28: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82201E2C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82201E30: 48169D61  bl 0x8236bb90
	ctx.lr = 0x82201E34;
	sub_8236BB90(ctx, base);
	// 82201E34: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82201E38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82201E3C: A0BF0006  lhz r5, 6(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82201E40: A09F0004  lhz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82201E44: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201E48: 48169F21  bl 0x8236bd68
	ctx.lr = 0x82201E4C;
	sub_8236BD68(ctx, base);
	// 82201E4C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82201E50: 483332A8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82201E58 size=8
    let mut pc: u32 = 0x82201E58;
    'dispatch: loop {
        match pc {
            0x82201E58 => {
    //   block [0x82201E58..0x82201E60)
	// 82201E58: 3863002C  addi r3, r3, 0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + 44;
	// 82201E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82201E60 size=104
    let mut pc: u32 = 0x82201E60;
    'dispatch: loop {
        match pc {
            0x82201E60 => {
    //   block [0x82201E60..0x82201EC8)
	// 82201E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82201E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82201E68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82201E6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82201E70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201E74: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201E78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82201E7C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82201E80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201E84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201E88: 4E800421  bctrl
	ctx.lr = 0x82201E8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201E8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201E90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82201E94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201E98: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201E9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201EA0: 4E800421  bctrl
	ctx.lr = 0x82201EA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201EA4: 7D63F050  subf r11, r3, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[3].s64;
	// 82201EA8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82201EAC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82201EB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82201EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82201EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82201EBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82201EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82201EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82201EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82201EC8 size=1020
    let mut pc: u32 = 0x82201EC8;
    'dispatch: loop {
        match pc {
            0x82201EC8 => {
    //   block [0x82201EC8..0x822022C4)
	// 82201EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82201ECC: 483331D5  bl 0x825350a0
	ctx.lr = 0x82201ED0;
	sub_82535080(ctx, base);
	// 82201ED0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82201ED4: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82201ED8: F88100C8  std r4, 0xc8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[4].u64 ) };
	// 82201EDC: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 82201EE0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82201EE4: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 82201EE8: 392100C8  addi r9, r1, 0xc8
	ctx.r[9].s64 = ctx.r[1].s64 + 200;
	// 82201EEC: 3AEB86B8  addi r23, r11, -0x7948
	ctx.r[23].s64 = ctx.r[11].s64 + -31048;
	// 82201EF0: E9690000  ld r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82201EF4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82201EF8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82201EFC: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 82201F00: 409803B8  bge cr6, 0x822022b8
	if !ctx.cr[6].lt {
	pc = 0x822022B8; continue 'dispatch;
	}
	// 82201F04: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82201F08: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82201F0C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82201F10: 7D4BB8AE  lbzx r10, r11, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82201F14: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82201F18: 409A03A0  bne cr6, 0x822022b8
	if !ctx.cr[6].eq {
	pc = 0x822022B8; continue 'dispatch;
	}
	// 82201F1C: 39570008  addi r10, r23, 8
	ctx.r[10].s64 = ctx.r[23].s64 + 8;
	// 82201F20: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82201F24: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82201F28: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82201F2C: 409A038C  bne cr6, 0x822022b8
	if !ctx.cr[6].eq {
	pc = 0x822022B8; continue 'dispatch;
	}
	// 82201F30: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82201F34: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82201F38: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 82201F3C: 4198FFB4  blt cr6, 0x82201ef0
	if ctx.cr[6].lt {
	pc = 0x82201EF0; continue 'dispatch;
	}
	// 82201F40: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201F44: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82201F48: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82201F4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82201F50: 4E800421  bctrl
	ctx.lr = 0x82201F54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82201F54: 7EDDB378  mr r29, r22
	ctx.r[29].u64 = ctx.r[22].u64;
	// 82201F58: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 82201F5C: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 82201F60: 3B8100C8  addi r28, r1, 0xc8
	ctx.r[28].s64 = ctx.r[1].s64 + 200;
	// 82201F64: EBFC0000  ld r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 82201F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82201F6C: 4817927D  bl 0x8237b1e8
	ctx.lr = 0x82201F70;
	sub_8237B1E8(ctx, base);
	// 82201F70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82201F74: 419A0344  beq cr6, 0x822022b8
	if ctx.cr[6].eq {
	pc = 0x822022B8; continue 'dispatch;
	}
	// 82201F78: FBE10050  std r31, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u64 ) };
	// 82201F7C: 7D63EA14  add r11, r3, r29
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82201F80: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82201F84: 557D0036  rlwinm r29, r11, 0, 0, 0x1b
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82201F88: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82201F8C: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 82201F90: 40980034  bge cr6, 0x82201fc4
	if !ctx.cr[6].lt {
	pc = 0x82201FC4; continue 'dispatch;
	}
	// 82201F94: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82201F98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82201F9C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82201FA0: 7D4BBA14  add r10, r11, r23
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82201FA4: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82201FA8: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82201FAC: 409A0018  bne cr6, 0x82201fc4
	if !ctx.cr[6].eq {
	pc = 0x82201FC4; continue 'dispatch;
	}
	// 82201FB0: 39370008  addi r9, r23, 8
	ctx.r[9].s64 = ctx.r[23].s64 + 8;
	// 82201FB4: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82201FB8: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82201FBC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82201FC0: 419A000C  beq cr6, 0x82201fcc
	if ctx.cr[6].eq {
	pc = 0x82201FCC; continue 'dispatch;
	}
	// 82201FC4: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82201FC8: 48000008  b 0x82201fd0
	pc = 0x82201FD0; continue 'dispatch;
	// 82201FCC: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82201FD0: A16B000A  lhz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82201FD4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82201FD8: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 82201FDC: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82201FE0: 2B1B0001  cmplwi cr6, r27, 1
	ctx.cr[6].compare_u32(ctx.r[27].u32, 1 as u32, &mut ctx.xer);
	// 82201FE4: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82201FE8: 4198FF7C  blt cr6, 0x82201f64
	if ctx.cr[6].lt {
	pc = 0x82201F64; continue 'dispatch;
	}
	// 82201FEC: 3D40829E  lis r10, -0x7d62
	ctx.r[10].s64 = -2103574528;
	// 82201FF0: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82201FF4: 3BEA63D0  addi r31, r10, 0x63d0
	ctx.r[31].s64 = ctx.r[10].s64 + 25552;
	// 82201FF8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82201FFC: 7F7E5850  subf r27, r30, r11
	ctx.r[27].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82202000: 615D9F70  ori r29, r10, 0x9f70
	ctx.r[29].u64 = ctx.r[10].u64 | 40816;
	// 82202004: 394B000F  addi r10, r11, 0xf
	ctx.r[10].s64 = ctx.r[11].s64 + 15;
	// 82202008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8220200C: 554B0036  rlwinm r11, r10, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82202010: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82202014: 4850B249  bl 0x8270d25c
	ctx.lr = 0x82202018;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82202018: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8220201C: 3B380004  addi r25, r24, 4
	ctx.r[25].s64 = ctx.r[24].s64 + 4;
	// 82202020: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82202024: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82202028: 806BB9F8  lwz r3, -0x4608(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17928 as u32) ) } as u64;
	// 8220202C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82202030: 38CB3AC4  addi r6, r11, 0x3ac4
	ctx.r[6].s64 = ctx.r[11].s64 + 15044;
	// 82202034: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202038: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8220203C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82202040: 4E800421  bctrl
	ctx.lr = 0x82202044;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82202044: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82202048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8220204C: 4850B221  bl 0x8270d26c
	ctx.lr = 0x82202050;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82202050: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82202054: 419A0264  beq cr6, 0x822022b8
	if ctx.cr[6].eq {
	pc = 0x822022B8; continue 'dispatch;
	}
	// 82202058: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220205C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82202060: 409A000C  bne cr6, 0x8220206c
	if !ctx.cr[6].eq {
	pc = 0x8220206C; continue 'dispatch;
	}
	// 82202064: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82202068: 4800002C  b 0x82202094
	pc = 0x82202094; continue 'dispatch;
	// 8220206C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82202070: 81390008  lwz r9, 8(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82202074: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82202078: 419A000C  beq cr6, 0x82202084
	if ctx.cr[6].eq {
	pc = 0x82202084; continue 'dispatch;
	}
	// 8220207C: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82202080: 48000014  b 0x82202094
	pc = 0x82202094; continue 'dispatch;
	// 82202084: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202088: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8220208C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82202090: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82202094: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82202098: 7EDAB378  mr r26, r22
	ctx.r[26].u64 = ctx.r[22].u64;
	// 8220209C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822020A0: 419A0008  beq cr6, 0x822020a8
	if ctx.cr[6].eq {
	pc = 0x822020A8; continue 'dispatch;
	}
	// 822020A4: 834A0000  lwz r26, 0(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822020A8: 7D7AE214  add r11, r26, r28
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[28].u64;
	// 822020AC: 7D1D5850  subf r8, r29, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 822020B0: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 822020B4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 822020B8: 61649F60  ori r4, r11, 0x9f60
	ctx.r[4].u64 = ctx.r[11].u64 | 40800;
	// 822020BC: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 822020C0: 61659F62  ori r5, r11, 0x9f62
	ctx.r[5].u64 = ctx.r[11].u64 | 40802;
	// 822020C4: 419A0010  beq cr6, 0x822020d4
	if ctx.cr[6].eq {
	pc = 0x822020D4; continue 'dispatch;
	}
	// 822020C8: 7EC8232E  sthx r22, r8, r4
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32), ctx.r[22].u16) };
	// 822020CC: 7EC82B2E  sthx r22, r8, r5
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32), ctx.r[22].u16) };
	// 822020D0: 48000008  b 0x822020d8
	pc = 0x822020D8; continue 'dispatch;
	// 822020D4: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 822020D8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 822020DC: 91180114  stw r8, 0x114(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(276 as u32), ctx.r[8].u32 ) };
	// 822020E0: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 822020E4: 38CBA210  addi r6, r11, -0x5df0
	ctx.r[6].s64 = ctx.r[11].s64 + -24048;
	// 822020E8: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822020EC: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 822020F0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 822020F4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822020F8: 7D4B3214  add r10, r11, r6
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 822020FC: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82202100: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82202104: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82202108: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8220210C: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82202110: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82202114: F8EA0000  std r7, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 82202118: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8220211C: 4200FFF0  bdnz 0x8220210c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8220210C; continue 'dispatch;
	}
	// 82202120: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82202124: 5569043E  clrlwi r9, r11, 0x10
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82202128: 2B09005F  cmplwi cr6, r9, 0x5f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 95 as u32, &mut ctx.xer);
	// 8220212C: 4198FFBC  blt cr6, 0x822020e8
	if ctx.cr[6].lt {
	pc = 0x822020E8; continue 'dispatch;
	}
	// 82202130: 3960005F  li r11, 0x5f
	ctx.r[11].s64 = 95;
	// 82202134: 7FFADA14  add r31, r26, r27
	ctx.r[31].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 82202138: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 8220213C: 3B8100C8  addi r28, r1, 0xc8
	ctx.r[28].s64 = ctx.r[1].s64 + 200;
	// 82202140: 3BB80094  addi r29, r24, 0x94
	ctx.r[29].s64 = ctx.r[24].s64 + 148;
	// 82202144: 7D68232E  sthx r11, r8, r4
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32), ctx.r[11].u16) };
	// 82202148: 7D682B2E  sthx r11, r8, r5
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32), ctx.r[11].u16) };
	// 8220214C: E97C0000  ld r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 82202150: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82202154: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82202158: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8220215C: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 82202160: 40980034  bge cr6, 0x82202194
	if !ctx.cr[6].lt {
	pc = 0x82202194; continue 'dispatch;
	}
	// 82202164: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82202168: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8220216C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82202170: 7D4BBA14  add r10, r11, r23
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82202174: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202178: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 8220217C: 409A0018  bne cr6, 0x82202194
	if !ctx.cr[6].eq {
	pc = 0x82202194; continue 'dispatch;
	}
	// 82202180: 39370008  addi r9, r23, 8
	ctx.r[9].s64 = ctx.r[23].s64 + 8;
	// 82202184: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82202188: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8220218C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82202190: 419A000C  beq cr6, 0x8220219c
	if ctx.cr[6].eq {
	pc = 0x8220219C; continue 'dispatch;
	}
	// 82202194: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82202198: 48000008  b 0x822021a0
	pc = 0x822021A0; continue 'dispatch;
	// 8220219C: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 822021A0: 894B0007  lbz r10, 7(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 822021A4: A3CB000A  lhz r30, 0xa(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 822021A8: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 822021AC: 40980014  bge cr6, 0x822021c0
	if !ctx.cr[6].lt {
	pc = 0x822021C0; continue 'dispatch;
	}
	// 822021B0: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822021B4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 822021B8: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822021BC: 48000028  b 0x822021e4
	pc = 0x822021E4; continue 'dispatch;
	// 822021C0: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 822021C4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 822021C8: 4098000C  bge cr6, 0x822021d4
	if !ctx.cr[6].lt {
	pc = 0x822021D4; continue 'dispatch;
	}
	// 822021CC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 822021D0: 48000014  b 0x822021e4
	pc = 0x822021E4; continue 'dispatch;
	// 822021D4: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822021D8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 822021DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822021E0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822021E4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 822021E8: 80780114  lwz r3, 0x114(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(276 as u32) ) } as u64;
	// 822021EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822021F0: 48185B49  bl 0x82387d38
	ctx.lr = 0x822021F4;
	sub_82387D38(ctx, base);
	// 822021F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822021F8: 419A00A4  beq cr6, 0x8220229c
	if ctx.cr[6].eq {
	pc = 0x8220229C; continue 'dispatch;
	}
	// 822021FC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82202200: 57CB0BFC  rlwinm r11, r30, 1, 0xf, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x7FFFFFFFu64;
	// 82202204: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82202208: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8220220C: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 82202210: 2B1B0001  cmplwi cr6, r27, 1
	ctx.cr[6].compare_u32(ctx.r[27].u32, 1 as u32, &mut ctx.xer);
	// 82202214: 4198FF38  blt cr6, 0x8220214c
	if ctx.cr[6].lt {
	pc = 0x8220214C; continue 'dispatch;
	}
	// 82202218: 3B980014  addi r28, r24, 0x14
	ctx.r[28].s64 = ctx.r[24].s64 + 20;
	// 8220221C: 7EDFB378  mr r31, r22
	ctx.r[31].u64 = ctx.r[22].u64;
	// 82202220: 3BA100C8  addi r29, r1, 0xc8
	ctx.r[29].s64 = ctx.r[1].s64 + 200;
	// 82202224: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82202228: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8220222C: E89D0000  ld r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 82202230: 935E0000  stw r26, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82202234: 48179065  bl 0x8237b298
	ctx.lr = 0x82202238;
	sub_8237B298(ctx, base);
	// 82202238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220223C: 419A0034  beq cr6, 0x82202270
	if ctx.cr[6].eq {
	pc = 0x82202270; continue 'dispatch;
	}
	// 82202240: 3963000F  addi r11, r3, 0xf
	ctx.r[11].s64 = ctx.r[3].s64 + 15;
	// 82202244: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82202248: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8220224C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82202250: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82202254: 7F4BD214  add r26, r11, r26
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82202258: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 8220225C: 4198FFCC  blt cr6, 0x82202228
	if ctx.cr[6].lt {
	pc = 0x82202228; continue 'dispatch;
	}
	// 82202260: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82202264: 98780010  stb r3, 0x10(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(16 as u32), ctx.r[3].u8 ) };
	// 82202268: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8220226C: 48332E84  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 82202270: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82202274: 419A0028  beq cr6, 0x8220229c
	if ctx.cr[6].eq {
	pc = 0x8220229C; continue 'dispatch;
	}
	// 82202278: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8220227C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202280: 48178E69  bl 0x8237b0e8
	ctx.lr = 0x82202284;
	sub_8237B0E8(ctx, base);
	// 82202284: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82202288: 92DE0080  stw r22, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[22].u32 ) };
	// 8220228C: 92DE0000  stw r22, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 82202290: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82202294: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82202298: 409AFFE4  bne cr6, 0x8220227c
	if !ctx.cr[6].eq {
	pc = 0x8220227C; continue 'dispatch;
	}
	// 8220229C: 92D80114  stw r22, 0x114(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(276 as u32), ctx.r[22].u32 ) };
	// 822022A0: 92D90000  stw r22, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 822022A4: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 822022A8: 92D90004  stw r22, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[22].u32 ) };
	// 822022AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822022B0: 419A0008  beq cr6, 0x822022b8
	if ctx.cr[6].eq {
	pc = 0x822022B8; continue 'dispatch;
	}
	// 822022B4: 4BF1ADA5  bl 0x8211d058
	ctx.lr = 0x822022B8;
	sub_8211D058(ctx, base);
	// 822022B8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 822022BC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822022C0: 48332E30  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822022C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822022C8 size=288
    let mut pc: u32 = 0x822022C8;
    'dispatch: loop {
        match pc {
            0x822022C8 => {
    //   block [0x822022C8..0x822023E8)
	// 822022C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822022CC: 48332DF1  bl 0x825350bc
	ctx.lr = 0x822022D0;
	sub_82535080(ctx, base);
	// 822022D0: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822022D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822022D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822022DC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822022E0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822022E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822022E8: 4E800421  bctrl
	ctx.lr = 0x822022EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822022EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822022F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822022F4: 4BFFFB6D  bl 0x82201e60
	ctx.lr = 0x822022F8;
	sub_82201E60(ctx, base);
	// 822022F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822022FC: 409A000C  bne cr6, 0x82202308
	if !ctx.cr[6].eq {
	pc = 0x82202308; continue 'dispatch;
	}
	// 82202300: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82202304: 48332E08  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82202308: 88BE0010  lbz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220230C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82202310: 4099009C  ble cr6, 0x822023ac
	if !ctx.cr[6].gt {
	pc = 0x822023AC; continue 'dispatch;
	}
	// 82202314: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 82202318: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8220231C: 390B86B8  addi r8, r11, -0x7948
	ctx.r[8].s64 = ctx.r[11].s64 + -31048;
	// 82202320: 3D604754  lis r11, 0x4754
	ctx.r[11].s64 = 1196687360;
	// 82202324: 38DE0014  addi r6, r30, 0x14
	ctx.r[6].s64 = ctx.r[30].s64 + 20;
	// 82202328: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 8220232C: 61634D57  ori r3, r11, 0x4d57
	ctx.r[3].u64 = ctx.r[11].u64 | 19799;
	// 82202330: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202334: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82202338: 419A0058  beq cr6, 0x82202390
	if ctx.cr[6].eq {
	pc = 0x82202390; continue 'dispatch;
	}
	// 8220233C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202340: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82202344: 409A004C  bne cr6, 0x82202390
	if !ctx.cr[6].eq {
	pc = 0x82202390; continue 'dispatch;
	}
	// 82202348: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8220234C: 2B0901FF  cmplwi cr6, r9, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 511 as u32, &mut ctx.xer);
	// 82202350: 40980040  bge cr6, 0x82202390
	if !ctx.cr[6].lt {
	pc = 0x82202390; continue 'dispatch;
	}
	// 82202354: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82202358: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8220235C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82202360: 7FEB40AE  lbzx r31, r11, r8
	ctx.r[31].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82202364: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 82202368: 409A0028  bne cr6, 0x82202390
	if !ctx.cr[6].eq {
	pc = 0x82202390; continue 'dispatch;
	}
	// 8220236C: 3BE80008  addi r31, r8, 8
	ctx.r[31].s64 = ctx.r[8].s64 + 8;
	// 82202370: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82202374: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82202378: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8220237C: 409A0014  bne cr6, 0x82202390
	if !ctx.cr[6].eq {
	pc = 0x82202390; continue 'dispatch;
	}
	// 82202380: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82202384: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82202388: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8220238C: 48000008  b 0x82202394
	pc = 0x82202394; continue 'dispatch;
	// 82202390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82202394: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82202398: F9640000  std r11, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8220239C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 822023A0: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 822023A4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 822023A8: 409AFF88  bne cr6, 0x82202330
	if !ctx.cr[6].eq {
	pc = 0x82202330; continue 'dispatch;
	}
	// 822023AC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822023B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822023B4: 48000035  bl 0x822023e8
	ctx.lr = 0x822023B8;
	sub_822023E8(ctx, base);
	// 822023B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822023BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822023C0: 419A001C  beq cr6, 0x822023dc
	if ctx.cr[6].eq {
	pc = 0x822023DC; continue 'dispatch;
	}
	// 822023C4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822023C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822023CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822023D0: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 822023D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822023D8: 4E800421  bctrl
	ctx.lr = 0x822023DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822023DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822023E0: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 822023E4: 48332D28  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822023E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822023E8 size=1052
    let mut pc: u32 = 0x822023E8;
    'dispatch: loop {
        match pc {
            0x822023E8 => {
    //   block [0x822023E8..0x82202804)
	// 822023E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822023EC: 48332CAD  bl 0x82535098
	ctx.lr = 0x822023F0;
	sub_82535080(ctx, base);
	// 822023F0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822023F4: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 822023F8: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 822023FC: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 82202400: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82202404: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82202408: 7E89A378  mr r9, r20
	ctx.r[9].u64 = ctx.r[20].u64;
	// 8220240C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82202410: 3ACB86B8  addi r22, r11, -0x7948
	ctx.r[22].s64 = ctx.r[11].s64 + -31048;
	// 82202414: 419A0058  beq cr6, 0x8220246c
	if ctx.cr[6].eq {
	pc = 0x8220246C; continue 'dispatch;
	}
	// 82202418: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8220241C: E9680000  ld r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82202420: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82202424: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82202428: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 8220242C: 409803CC  bge cr6, 0x822027f8
	if !ctx.cr[6].lt {
	pc = 0x822027F8; continue 'dispatch;
	}
	// 82202430: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82202434: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82202438: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8220243C: 7D4BB0AE  lbzx r10, r11, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82202440: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82202444: 409A03B4  bne cr6, 0x822027f8
	if !ctx.cr[6].eq {
	pc = 0x822027F8; continue 'dispatch;
	}
	// 82202448: 39560008  addi r10, r22, 8
	ctx.r[10].s64 = ctx.r[22].s64 + 8;
	// 8220244C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82202450: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82202454: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82202458: 409A03A0  bne cr6, 0x822027f8
	if !ctx.cr[6].eq {
	pc = 0x822027F8; continue 'dispatch;
	}
	// 8220245C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82202460: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82202464: 7F09B840  cmplw cr6, r9, r23
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82202468: 4198FFB4  blt cr6, 0x8220241c
	if ctx.cr[6].lt {
	pc = 0x8220241C; continue 'dispatch;
	}
	// 8220246C: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202470: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82202474: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8220247C: 4E800421  bctrl
	ctx.lr = 0x82202480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82202480: 7E9BA378  mr r27, r20
	ctx.r[27].u64 = ctx.r[20].u64;
	// 82202484: 7E9DA378  mr r29, r20
	ctx.r[29].u64 = ctx.r[20].u64;
	// 82202488: 7E9CA378  mr r28, r20
	ctx.r[28].u64 = ctx.r[20].u64;
	// 8220248C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82202490: 419A0090  beq cr6, 0x82202520
	if ctx.cr[6].eq {
	pc = 0x82202520; continue 'dispatch;
	}
	// 82202494: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82202498: E87F0000  ld r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8220249C: 48178D4D  bl 0x8237b1e8
	ctx.lr = 0x822024A0;
	sub_8237B1E8(ctx, base);
	// 822024A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822024A4: 419A0354  beq cr6, 0x822027f8
	if ctx.cr[6].eq {
	pc = 0x822027F8; continue 'dispatch;
	}
	// 822024A8: E97F0000  ld r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 822024AC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 822024B0: 7D63DA14  add r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[27].u64;
	// 822024B4: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 822024B8: 557B0036  rlwinm r27, r11, 0, 0, 0x1b
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 822024BC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822024C0: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 822024C4: 40980034  bge cr6, 0x822024f8
	if !ctx.cr[6].lt {
	pc = 0x822024F8; continue 'dispatch;
	}
	// 822024C8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822024CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822024D0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822024D4: 7D4BB214  add r10, r11, r22
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 822024D8: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822024DC: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 822024E0: 409A0018  bne cr6, 0x822024f8
	if !ctx.cr[6].eq {
	pc = 0x822024F8; continue 'dispatch;
	}
	// 822024E4: 39360008  addi r9, r22, 8
	ctx.r[9].s64 = ctx.r[22].s64 + 8;
	// 822024E8: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822024EC: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822024F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822024F4: 419A000C  beq cr6, 0x82202500
	if ctx.cr[6].eq {
	pc = 0x82202500; continue 'dispatch;
	}
	// 822024F8: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 822024FC: 48000008  b 0x82202504
	pc = 0x82202504; continue 'dispatch;
	// 82202500: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82202504: A16B000A  lhz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82202508: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8220250C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82202510: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82202514: 7F1CB840  cmplw cr6, r28, r23
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82202518: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8220251C: 4198FF7C  blt cr6, 0x82202498
	if ctx.cr[6].lt {
	pc = 0x82202498; continue 'dispatch;
	}
	// 82202520: 3D40829E  lis r10, -0x7d62
	ctx.r[10].s64 = -2103574528;
	// 82202524: 7D7DDA14  add r11, r29, r27
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[27].u64;
	// 82202528: 3BEA63D0  addi r31, r10, 0x63d0
	ctx.r[31].s64 = ctx.r[10].s64 + 25552;
	// 8220252C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82202530: 7F5D5850  subf r26, r29, r11
	ctx.r[26].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82202534: 615C9F70  ori r28, r10, 0x9f70
	ctx.r[28].u64 = ctx.r[10].u64 | 40816;
	// 82202538: 394B000F  addi r10, r11, 0xf
	ctx.r[10].s64 = ctx.r[11].s64 + 15;
	// 8220253C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82202540: 554B0036  rlwinm r11, r10, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82202544: 7F6BE214  add r27, r11, r28
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82202548: 4850AD15  bl 0x8270d25c
	ctx.lr = 0x8220254C;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8220254C: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82202550: 3B150004  addi r24, r21, 4
	ctx.r[24].s64 = ctx.r[21].s64 + 4;
	// 82202554: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82202558: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8220255C: 806BB9F8  lwz r3, -0x4608(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17928 as u32) ) } as u64;
	// 82202560: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82202564: 38CB3AC4  addi r6, r11, 0x3ac4
	ctx.r[6].s64 = ctx.r[11].s64 + 15044;
	// 82202568: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220256C: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82202570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82202574: 4E800421  bctrl
	ctx.lr = 0x82202578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82202578: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8220257C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82202580: 4850ACED  bl 0x8270d26c
	ctx.lr = 0x82202584;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82202584: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82202588: 419A0270  beq cr6, 0x822027f8
	if ctx.cr[6].eq {
	pc = 0x822027F8; continue 'dispatch;
	}
	// 8220258C: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202590: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82202594: 409A000C  bne cr6, 0x822025a0
	if !ctx.cr[6].eq {
	pc = 0x822025A0; continue 'dispatch;
	}
	// 82202598: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 8220259C: 4800002C  b 0x822025c8
	pc = 0x822025C8; continue 'dispatch;
	// 822025A0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822025A4: 81380008  lwz r9, 8(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 822025A8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822025AC: 419A000C  beq cr6, 0x822025b8
	if ctx.cr[6].eq {
	pc = 0x822025B8; continue 'dispatch;
	}
	// 822025B0: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 822025B4: 48000014  b 0x822025c8
	pc = 0x822025C8; continue 'dispatch;
	// 822025B8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822025BC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822025C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822025C4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 822025C8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 822025CC: 7E99A378  mr r25, r20
	ctx.r[25].u64 = ctx.r[20].u64;
	// 822025D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822025D4: 419A0008  beq cr6, 0x822025dc
	if ctx.cr[6].eq {
	pc = 0x822025DC; continue 'dispatch;
	}
	// 822025D8: 832A0000  lwz r25, 0(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822025DC: 7D79DA14  add r11, r25, r27
	ctx.r[11].u64 = ctx.r[25].u64 + ctx.r[27].u64;
	// 822025E0: 7CFC5850  subf r7, r28, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 822025E4: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 822025E8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 822025EC: 61649F60  ori r4, r11, 0x9f60
	ctx.r[4].u64 = ctx.r[11].u64 | 40800;
	// 822025F0: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 822025F4: 61659F62  ori r5, r11, 0x9f62
	ctx.r[5].u64 = ctx.r[11].u64 | 40802;
	// 822025F8: 419A0010  beq cr6, 0x82202608
	if ctx.cr[6].eq {
	pc = 0x82202608; continue 'dispatch;
	}
	// 822025FC: 7E87232E  sthx r20, r7, r4
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32), ctx.r[20].u16) };
	// 82202600: 7E872B2E  sthx r20, r7, r5
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[5].u32), ctx.r[20].u16) };
	// 82202604: 48000008  b 0x8220260c
	pc = 0x8220260C; continue 'dispatch;
	// 82202608: 7E87A378  mr r7, r20
	ctx.r[7].u64 = ctx.r[20].u64;
	// 8220260C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82202610: 90F50114  stw r7, 0x114(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(276 as u32), ctx.r[7].u32 ) };
	// 82202614: 7E89A378  mr r9, r20
	ctx.r[9].u64 = ctx.r[20].u64;
	// 82202618: 38CBA210  addi r6, r11, -0x5df0
	ctx.r[6].s64 = ctx.r[11].s64 + -24048;
	// 8220261C: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82202620: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82202624: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82202628: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8220262C: 7D4B3214  add r10, r11, r6
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82202630: 7C6B3A14  add r3, r11, r7
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82202634: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82202638: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8220263C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82202640: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82202644: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82202648: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 8220264C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82202650: 4200FFF0  bdnz 0x82202640
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82202640; continue 'dispatch;
	}
	// 82202654: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82202658: 5569043E  clrlwi r9, r11, 0x10
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8220265C: 2B09005F  cmplwi cr6, r9, 0x5f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 95 as u32, &mut ctx.xer);
	// 82202660: 4198FFBC  blt cr6, 0x8220261c
	if ctx.cr[6].lt {
	pc = 0x8220261C; continue 'dispatch;
	}
	// 82202664: 3960005F  li r11, 0x5f
	ctx.r[11].s64 = 95;
	// 82202668: 7FB9D214  add r29, r25, r26
	ctx.r[29].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 8220266C: 7E9CA378  mr r28, r20
	ctx.r[28].u64 = ctx.r[20].u64;
	// 82202670: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82202674: 7D67232E  sthx r11, r7, r4
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32), ctx.r[11].u16) };
	// 82202678: 7D672B2E  sthx r11, r7, r5
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[5].u32), ctx.r[11].u16) };
	// 8220267C: 419A00D8  beq cr6, 0x82202754
	if ctx.cr[6].eq {
	pc = 0x82202754; continue 'dispatch;
	}
	// 82202680: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 82202684: 3B750094  addi r27, r21, 0x94
	ctx.r[27].s64 = ctx.r[21].s64 + 148;
	// 82202688: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8220268C: E97A0000  ld r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) };
	// 82202690: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82202694: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82202698: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 8220269C: 40980034  bge cr6, 0x822026d0
	if !ctx.cr[6].lt {
	pc = 0x822026D0; continue 'dispatch;
	}
	// 822026A0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822026A4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822026A8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822026AC: 7D4BB214  add r10, r11, r22
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 822026B0: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822026B4: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 822026B8: 409A0018  bne cr6, 0x822026d0
	if !ctx.cr[6].eq {
	pc = 0x822026D0; continue 'dispatch;
	}
	// 822026BC: 39360008  addi r9, r22, 8
	ctx.r[9].s64 = ctx.r[22].s64 + 8;
	// 822026C0: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822026C4: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822026C8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822026CC: 419A000C  beq cr6, 0x822026d8
	if ctx.cr[6].eq {
	pc = 0x822026D8; continue 'dispatch;
	}
	// 822026D0: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 822026D4: 48000008  b 0x822026dc
	pc = 0x822026DC; continue 'dispatch;
	// 822026D8: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 822026DC: 894B0007  lbz r10, 7(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 822026E0: A3EB000A  lhz r31, 0xa(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 822026E4: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 822026E8: 40980014  bge cr6, 0x822026fc
	if !ctx.cr[6].lt {
	pc = 0x822026FC; continue 'dispatch;
	}
	// 822026EC: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822026F0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 822026F4: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822026F8: 48000028  b 0x82202720
	pc = 0x82202720; continue 'dispatch;
	// 822026FC: 894B0009  lbz r10, 9(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 82202700: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82202704: 4098000C  bge cr6, 0x82202710
	if !ctx.cr[6].lt {
	pc = 0x82202710; continue 'dispatch;
	}
	// 82202708: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8220270C: 48000014  b 0x82202720
	pc = 0x82202720; continue 'dispatch;
	// 82202710: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82202714: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82202718: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220271C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202720: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82202724: 80750114  lwz r3, 0x114(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(276 as u32) ) } as u64;
	// 82202728: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8220272C: 4818560D  bl 0x82387d38
	ctx.lr = 0x82202730;
	sub_82387D38(ctx, base);
	// 82202730: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82202734: 419A00A8  beq cr6, 0x822027dc
	if ctx.cr[6].eq {
	pc = 0x822027DC; continue 'dispatch;
	}
	// 82202738: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8220273C: 57EB0BFC  rlwinm r11, r31, 1, 0xf, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x7FFFFFFFu64;
	// 82202740: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82202744: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82202748: 3B5A0008  addi r26, r26, 8
	ctx.r[26].s64 = ctx.r[26].s64 + 8;
	// 8220274C: 7F1CB840  cmplw cr6, r28, r23
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82202750: 4198FF38  blt cr6, 0x82202688
	if ctx.cr[6].lt {
	pc = 0x82202688; continue 'dispatch;
	}
	// 82202754: 7E9DA378  mr r29, r20
	ctx.r[29].u64 = ctx.r[20].u64;
	// 82202758: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 8220275C: 419A0044  beq cr6, 0x822027a0
	if ctx.cr[6].eq {
	pc = 0x822027A0; continue 'dispatch;
	}
	// 82202760: 3B950014  addi r28, r21, 0x14
	ctx.r[28].s64 = ctx.r[21].s64 + 20;
	// 82202764: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82202768: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8220276C: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82202770: E89E0000  ld r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82202774: 48178B25  bl 0x8237b298
	ctx.lr = 0x82202778;
	sub_8237B298(ctx, base);
	// 82202778: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220277C: 419A0034  beq cr6, 0x822027b0
	if ctx.cr[6].eq {
	pc = 0x822027B0; continue 'dispatch;
	}
	// 82202780: 3963000F  addi r11, r3, 0xf
	ctx.r[11].s64 = ctx.r[3].s64 + 15;
	// 82202784: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82202788: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8220278C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82202790: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82202794: 7F2BCA14  add r25, r11, r25
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82202798: 7F1DB840  cmplw cr6, r29, r23
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[23].u32, &mut ctx.xer);
	// 8220279C: 4198FFCC  blt cr6, 0x82202768
	if ctx.cr[6].lt {
	pc = 0x82202768; continue 'dispatch;
	}
	// 822027A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822027A4: 9AF50010  stb r23, 0x10(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(16 as u32), ctx.r[23].u8 ) };
	// 822027A8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 822027AC: 4833293C  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
	// 822027B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822027B4: 419A0028  beq cr6, 0x822027dc
	if ctx.cr[6].eq {
	pc = 0x822027DC; continue 'dispatch;
	}
	// 822027B8: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 822027BC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822027C0: 48178929  bl 0x8237b0e8
	ctx.lr = 0x822027C4;
	sub_8237B0E8(ctx, base);
	// 822027C4: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 822027C8: 929F0080  stw r20, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[20].u32 ) };
	// 822027CC: 929F0000  stw r20, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 822027D0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 822027D4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822027D8: 409AFFE4  bne cr6, 0x822027bc
	if !ctx.cr[6].eq {
	pc = 0x822027BC; continue 'dispatch;
	}
	// 822027DC: 92950114  stw r20, 0x114(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(276 as u32), ctx.r[20].u32 ) };
	// 822027E0: 92980000  stw r20, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 822027E4: 80780004  lwz r3, 4(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 822027E8: 92980004  stw r20, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[20].u32 ) };
	// 822027EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822027F0: 419A0008  beq cr6, 0x822027f8
	if ctx.cr[6].eq {
	pc = 0x822027F8; continue 'dispatch;
	}
	// 822027F4: 4BF1A865  bl 0x8211d058
	ctx.lr = 0x822027F8;
	sub_8211D058(ctx, base);
	// 822027F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822027FC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82202800: 483328E8  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202808 size=120
    let mut pc: u32 = 0x82202808;
    'dispatch: loop {
        match pc {
            0x82202808 => {
    //   block [0x82202808..0x82202880)
	// 82202808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220280C: 483328AD  bl 0x825350b8
	ctx.lr = 0x82202810;
	sub_82535080(ctx, base);
	// 82202810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202814: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82202818: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8220281C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82202820: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202828: 419A002C  beq cr6, 0x82202854
	if ctx.cr[6].eq {
	pc = 0x82202854; continue 'dispatch;
	}
	// 8220282C: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 82202830: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202834: 481788B5  bl 0x8237b0e8
	ctx.lr = 0x82202838;
	sub_8237B0E8(ctx, base);
	// 82202838: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 8220283C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82202840: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82202844: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82202848: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220284C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82202850: 4198FFE0  blt cr6, 0x82202830
	if ctx.cr[6].lt {
	pc = 0x82202830; continue 'dispatch;
	}
	// 82202854: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82202858: 93DD0114  stw r30, 0x114(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(276 as u32), ctx.r[30].u32 ) };
	// 8220285C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82202860: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82202864: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82202868: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220286C: 419A0008  beq cr6, 0x82202874
	if ctx.cr[6].eq {
	pc = 0x82202874; continue 'dispatch;
	}
	// 82202870: 4BF1A7E9  bl 0x8211d058
	ctx.lr = 0x82202874;
	sub_8211D058(ctx, base);
	// 82202874: 9BDD0010  stb r30, 0x10(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82202878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8220287C: 4833288C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202880 size=148
    let mut pc: u32 = 0x82202880;
    'dispatch: loop {
        match pc {
            0x82202880 => {
    //   block [0x82202880..0x82202914)
	// 82202880: 88A30010  lbz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202884: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82202888: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8220288C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82202890: 419A007C  beq cr6, 0x8220290c
	if ctx.cr[6].eq {
	pc = 0x8220290C; continue 'dispatch;
	}
	// 82202894: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 82202898: 38E30014  addi r7, r3, 0x14
	ctx.r[7].s64 = ctx.r[3].s64 + 20;
	// 8220289C: 38CB86B8  addi r6, r11, -0x7948
	ctx.r[6].s64 = ctx.r[11].s64 + -31048;
	// 822028A0: 3D604754  lis r11, 0x4754
	ctx.r[11].s64 = 1196687360;
	// 822028A4: 61644D57  ori r4, r11, 0x4d57
	ctx.r[4].u64 = ctx.r[11].u64 | 19799;
	// 822028A8: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 822028AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822028B0: 419A0064  beq cr6, 0x82202914
	if ctx.cr[6].eq {
		sub_82202914(ctx, base);
		return;
	}
	// 822028B4: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 822028B8: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 822028BC: 409A0058  bne cr6, 0x82202914
	if !ctx.cr[6].eq {
		sub_82202914(ctx, base);
		return;
	}
	// 822028C0: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 822028C4: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 822028C8: 4098004C  bge cr6, 0x82202914
	if !ctx.cr[6].lt {
		sub_82202914(ctx, base);
		return;
	}
	// 822028CC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822028D0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822028D4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822028D8: 7D4B30AE  lbzx r10, r11, r6
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 822028DC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 822028E0: 409A0034  bne cr6, 0x82202914
	if !ctx.cr[6].eq {
		sub_82202914(ctx, base);
		return;
	}
	// 822028E4: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 822028E8: 39260008  addi r9, r6, 8
	ctx.r[9].s64 = ctx.r[6].s64 + 8;
	// 822028EC: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822028F0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822028F4: 409A0020  bne cr6, 0x82202914
	if !ctx.cr[6].eq {
		sub_82202914(ctx, base);
		return;
	}
	// 822028F8: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 822028FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82202900: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82202904: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82202908: 4198FFA0  blt cr6, 0x822028a8
	if ctx.cr[6].lt {
	pc = 0x822028A8; continue 'dispatch;
	}
	// 8220290C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82202910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202914 size=8
    let mut pc: u32 = 0x82202914;
    'dispatch: loop {
        match pc {
            0x82202914 => {
    //   block [0x82202914..0x8220291C)
	// 82202914: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82202918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202920 size=100
    let mut pc: u32 = 0x82202920;
    'dispatch: loop {
        match pc {
            0x82202920 => {
    //   block [0x82202920..0x82202984)
	// 82202920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82202924: 48332799  bl 0x825350bc
	ctx.lr = 0x82202928;
	sub_82535080(ctx, base);
	// 82202928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220292C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82202930: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82202934: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82202938: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220293C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202940: 419A003C  beq cr6, 0x8220297c
	if ctx.cr[6].eq {
	pc = 0x8220297C; continue 'dispatch;
	}
	// 82202944: 3BDD0014  addi r30, r29, 0x14
	ctx.r[30].s64 = ctx.r[29].s64 + 20;
	// 82202948: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220294C: 48178B1D  bl 0x8237b468
	ctx.lr = 0x82202950;
	sub_8237B468(ctx, base);
	// 82202950: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82202954: 419A0024  beq cr6, 0x82202978
	if ctx.cr[6].eq {
	pc = 0x82202978; continue 'dispatch;
	}
	// 82202958: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220295C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82202960: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82202964: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82202968: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8220296C: 4198FFDC  blt cr6, 0x82202948
	if ctx.cr[6].lt {
	pc = 0x82202948; continue 'dispatch;
	}
	// 82202970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82202974: 48332798  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82202978: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8220297C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82202980: 4833278C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202988 size=268
    let mut pc: u32 = 0x82202988;
    'dispatch: loop {
        match pc {
            0x82202988 => {
    //   block [0x82202988..0x82202A94)
	// 82202988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220298C: 48332721  bl 0x825350ac
	ctx.lr = 0x82202990;
	sub_82535080(ctx, base);
	// 82202990: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202994: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82202998: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8220299C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 822029A0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 822029A4: 897A0010  lbz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 822029A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822029AC: 419A00E0  beq cr6, 0x82202a8c
	if ctx.cr[6].eq {
	pc = 0x82202A8C; continue 'dispatch;
	}
	// 822029B0: 3D604754  lis r11, 0x4754
	ctx.r[11].s64 = 1196687360;
	// 822029B4: 3BDA0014  addi r30, r26, 0x14
	ctx.r[30].s64 = ctx.r[26].s64 + 20;
	// 822029B8: 61794D57  ori r25, r11, 0x4d57
	ctx.r[25].u64 = ctx.r[11].u64 | 19799;
	// 822029BC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 822029C0: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 822029C4: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 822029C8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822029CC: 80FE0080  lwz r7, 0x80(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 822029D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822029D4: 419A009C  beq cr6, 0x82202a70
	if ctx.cr[6].eq {
	pc = 0x82202A70; continue 'dispatch;
	}
	// 822029D8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 822029DC: 419A0094  beq cr6, 0x82202a70
	if ctx.cr[6].eq {
	pc = 0x82202A70; continue 'dispatch;
	}
	// 822029E0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822029E4: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 822029E8: 409A0088  bne cr6, 0x82202a70
	if !ctx.cr[6].eq {
	pc = 0x82202A70; continue 'dispatch;
	}
	// 822029EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822029F0: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822029F4: 810A0024  lwz r8, 0x24(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 822029F8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822029FC: 40980034  bge cr6, 0x82202a30
	if !ctx.cr[6].lt {
	pc = 0x82202A30; continue 'dispatch;
	}
	// 82202A00: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82202A04: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82202A08: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82202A0C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82202A10: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82202A14: 9BEB0000  stb r31, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 82202A18: 98AB0001  stb r5, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[5].u8 ) };
	// 82202A1C: 9BAB0002  stb r29, 2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82202A20: 98AB0003  stb r5, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 82202A24: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82202A28: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82202A2C: 90AB000C  stw r5, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82202A30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202A34: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82202A38: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82202A3C: 40980034  bge cr6, 0x82202a70
	if !ctx.cr[6].lt {
	pc = 0x82202A70; continue 'dispatch;
	}
	// 82202A40: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82202A44: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82202A48: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82202A4C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82202A50: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82202A54: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82202A58: 98AB0001  stb r5, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[5].u8 ) };
	// 82202A5C: 98AB0002  stb r5, 2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[5].u8 ) };
	// 82202A60: 98AB0003  stb r5, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 82202A64: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82202A68: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82202A6C: 90AB000C  stw r5, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82202A70: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202A74: 481794DD  bl 0x8237bf50
	ctx.lr = 0x82202A78;
	sub_8237BF50(ctx, base);
	// 82202A78: 897A0010  lbz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202A7C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82202A80: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82202A84: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82202A88: 4198FF40  blt cr6, 0x822029c8
	if ctx.cr[6].lt {
	pc = 0x822029C8; continue 'dispatch;
	}
	// 82202A8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82202A90: 4833266C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202A98 size=308
    let mut pc: u32 = 0x82202A98;
    'dispatch: loop {
        match pc {
            0x82202A98 => {
    //   block [0x82202A98..0x82202BCC)
	// 82202A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82202A9C: 4833260D  bl 0x825350a8
	ctx.lr = 0x82202AA0;
	sub_82535080(ctx, base);
	// 82202AA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202AA4: 54CB3032  slwi r11, r6, 6
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82202AA8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82202AAC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82202AB0: 7FAB2A14  add r29, r11, r5
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82202AB4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82202AB8: 419A0054  beq cr6, 0x82202b0c
	if ctx.cr[6].eq {
	pc = 0x82202B0C; continue 'dispatch;
	}
	// 82202ABC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82202AC0: 419A004C  beq cr6, 0x82202b0c
	if ctx.cr[6].eq {
	pc = 0x82202B0C; continue 'dispatch;
	}
	// 82202AC4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202AC8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82202ACC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82202AD0: 4098003C  bge cr6, 0x82202b0c
	if !ctx.cr[6].lt {
	pc = 0x82202B0C; continue 'dispatch;
	}
	// 82202AD4: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82202AD8: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82202ADC: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82202AE0: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82202AE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82202AE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82202AEC: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82202AF0: 9B8B0001  stb r28, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[28].u8 ) };
	// 82202AF4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82202AF8: 992B0002  stb r9, 2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u8 ) };
	// 82202AFC: 9B8B0003  stb r28, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[28].u8 ) };
	// 82202B00: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82202B04: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82202B08: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82202B0C: 89790010  lbz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202B10: 7F9AE378  mr r26, r28
	ctx.r[26].u64 = ctx.r[28].u64;
	// 82202B14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202B18: 419A00AC  beq cr6, 0x82202bc4
	if ctx.cr[6].eq {
	pc = 0x82202BC4; continue 'dispatch;
	}
	// 82202B1C: 3D604754  lis r11, 0x4754
	ctx.r[11].s64 = 1196687360;
	// 82202B20: 3B790014  addi r27, r25, 0x14
	ctx.r[27].s64 = ctx.r[25].s64 + 20;
	// 82202B24: 61784D57  ori r24, r11, 0x4d57
	ctx.r[24].u64 = ctx.r[11].u64 | 19799;
	// 82202B28: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202B2C: 80FB0080  lwz r7, 0x80(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(128 as u32) ) } as u64;
	// 82202B30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82202B34: 419A007C  beq cr6, 0x82202bb0
	if ctx.cr[6].eq {
	pc = 0x82202BB0; continue 'dispatch;
	}
	// 82202B38: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82202B3C: 419A0074  beq cr6, 0x82202bb0
	if ctx.cr[6].eq {
	pc = 0x82202BB0; continue 'dispatch;
	}
	// 82202B40: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82202B44: 419A006C  beq cr6, 0x82202bb0
	if ctx.cr[6].eq {
	pc = 0x82202BB0; continue 'dispatch;
	}
	// 82202B48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202B4C: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82202B50: 409A0060  bne cr6, 0x82202bb0
	if !ctx.cr[6].eq {
	pc = 0x82202BB0; continue 'dispatch;
	}
	// 82202B54: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82202B58: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82202B5C: A14B000A  lhz r10, 0xa(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82202B60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82202B64: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82202B68: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82202B6C: 83CB000C  lwz r30, 0xc(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82202B70: 419A0040  beq cr6, 0x82202bb0
	if ctx.cr[6].eq {
	pc = 0x82202BB0; continue 'dispatch;
	}
	// 82202B74: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82202B78: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202B7C: 7CA8F214  add r5, r8, r30
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 82202B80: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82202B84: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 82202B88: 7C685214  add r3, r8, r10
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82202B8C: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82202B90: 481651A1  bl 0x82367d30
	ctx.lr = 0x82202B94;
	sub_82367D30(ctx, base);
	// 82202B94: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82202B98: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82202B9C: 38E70002  addi r7, r7, 2
	ctx.r[7].s64 = ctx.r[7].s64 + 2;
	// 82202BA0: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82202BA4: A16B000A  lhz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82202BA8: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82202BAC: 4198FFCC  blt cr6, 0x82202b78
	if ctx.cr[6].lt {
	pc = 0x82202B78; continue 'dispatch;
	}
	// 82202BB0: 89790010  lbz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202BB4: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82202BB8: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82202BBC: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82202BC0: 4198FF68  blt cr6, 0x82202b28
	if ctx.cr[6].lt {
	pc = 0x82202B28; continue 'dispatch;
	}
	// 82202BC4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82202BC8: 48332530  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202BD0 size=28
    let mut pc: u32 = 0x82202BD0;
    'dispatch: loop {
        match pc {
            0x82202BD0 => {
    //   block [0x82202BD0..0x82202BEC)
	// 82202BD0: 81630114  lwz r11, 0x114(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 82202BD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202BD8: 419A0014  beq cr6, 0x82202bec
	if ctx.cr[6].eq {
		sub_82202BEC(ctx, base);
		return;
	}
	// 82202BDC: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82202BE0: 614A9F62  ori r10, r10, 0x9f62
	ctx.r[10].u64 = ctx.r[10].u64 | 40802;
	// 82202BE4: 7C6B522E  lhzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82202BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202BEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202BEC size=8
    let mut pc: u32 = 0x82202BEC;
    'dispatch: loop {
        match pc {
            0x82202BEC => {
    //   block [0x82202BEC..0x82202BF4)
	// 82202BEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82202BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202BF8 size=92
    let mut pc: u32 = 0x82202BF8;
    'dispatch: loop {
        match pc {
            0x82202BF8 => {
    //   block [0x82202BF8..0x82202C54)
	// 82202BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82202BFC: 483324BD  bl 0x825350b8
	ctx.lr = 0x82202C00;
	sub_82535080(ctx, base);
	// 82202C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202C04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82202C08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82202C0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82202C10: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202C14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202C18: 419A0030  beq cr6, 0x82202c48
	if ctx.cr[6].eq {
	pc = 0x82202C48; continue 'dispatch;
	}
	// 82202C1C: 3BDD0014  addi r30, r29, 0x14
	ctx.r[30].s64 = ctx.r[29].s64 + 20;
	// 82202C20: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202C24: 481798ED  bl 0x8237c510
	ctx.lr = 0x82202C28;
	sub_8237C510(ctx, base);
	// 82202C28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82202C2C: 419A0008  beq cr6, 0x82202c34
	if ctx.cr[6].eq {
	pc = 0x82202C34; continue 'dispatch;
	}
	// 82202C30: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82202C34: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202C38: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82202C3C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82202C40: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82202C44: 4198FFDC  blt cr6, 0x82202c20
	if ctx.cr[6].lt {
	pc = 0x82202C20; continue 'dispatch;
	}
	// 82202C48: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82202C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82202C50: 483324B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202C58 size=24
    let mut pc: u32 = 0x82202C58;
    'dispatch: loop {
        match pc {
            0x82202C58 => {
    //   block [0x82202C58..0x82202C70)
	// 82202C58: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82202C5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82202C60: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82202C64: 89680010  lbz r11, 0x10(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202C68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202C6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82202C70 size=44
    let mut pc: u32 = 0x82202C70;
    'dispatch: loop {
        match pc {
            0x82202C70 => {
    //   block [0x82202C70..0x82202C9C)
	// 82202C70: 3D604754  lis r11, 0x4754
	ctx.r[11].s64 = 1196687360;
	// 82202C74: 39480014  addi r10, r8, 0x14
	ctx.r[10].s64 = ctx.r[8].s64 + 20;
	// 82202C78: 61674D57  ori r7, r11, 0x4d57
	ctx.r[7].u64 = ctx.r[11].u64 | 19799;
	// 82202C7C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202C80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202C84: 419A0018  beq cr6, 0x82202c9c
	if ctx.cr[6].eq {
		sub_82202C9C(ctx, base);
		return;
	}
	// 82202C88: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202C8C: 7F063840  cmplw cr6, r6, r7
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82202C90: 409A000C  bne cr6, 0x82202c9c
	if !ctx.cr[6].eq {
		sub_82202C9C(ctx, base);
		return;
	}
	// 82202C94: D02B0018  stfs f1, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82202C98: 48000008  b 0x82202ca0
	sub_82202C9C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202C9C size=28
    let mut pc: u32 = 0x82202C9C;
    'dispatch: loop {
        match pc {
            0x82202C9C => {
    //   block [0x82202C9C..0x82202CB8)
	// 82202C9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82202CA0: 89680010  lbz r11, 0x10(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202CA4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82202CA8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82202CAC: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82202CB0: 4198FFCC  blt cr6, 0x82202c7c
	if ctx.cr[6].lt {
		sub_82202C70(ctx, base);
		return;
	}
	// 82202CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202CB8 size=24
    let mut pc: u32 = 0x82202CB8;
    'dispatch: loop {
        match pc {
            0x82202CB8 => {
    //   block [0x82202CB8..0x82202CD0)
	// 82202CB8: 89430010  lbz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202CBC: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82202CC0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82202CC4: 4198000C  blt cr6, 0x82202cd0
	if ctx.cr[6].lt {
		sub_82202CD0(ctx, base);
		return;
	}
	// 82202CC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82202CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202CD0 size=20
    let mut pc: u32 = 0x82202CD0;
    'dispatch: loop {
        match pc {
            0x82202CD0 => {
    //   block [0x82202CD0..0x82202CE4)
	// 82202CD0: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 82202CD4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82202CD8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82202CDC: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82202CE0: 48178830  b 0x8237b510
	sub_8237B510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202CE8 size=24
    let mut pc: u32 = 0x82202CE8;
    'dispatch: loop {
        match pc {
            0x82202CE8 => {
    //   block [0x82202CE8..0x82202D00)
	// 82202CE8: 89430010  lbz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202CEC: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82202CF0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82202CF4: 4198000C  blt cr6, 0x82202d00
	if ctx.cr[6].lt {
		sub_82202D00(ctx, base);
		return;
	}
	// 82202CF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82202CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82202D00 size=20
    let mut pc: u32 = 0x82202D00;
    'dispatch: loop {
        match pc {
            0x82202D00 => {
    //   block [0x82202D00..0x82202D14)
	// 82202D00: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 82202D04: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82202D08: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82202D0C: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82202D10: 481788B8  b 0x8237b5c8
	sub_8237B5C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202D18 size=108
    let mut pc: u32 = 0x82202D18;
    'dispatch: loop {
        match pc {
            0x82202D18 => {
    //   block [0x82202D18..0x82202D84)
	// 82202D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82202D1C: 4833239D  bl 0x825350b8
	ctx.lr = 0x82202D20;
	sub_82535080(ctx, base);
	// 82202D20: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82202D24: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202D28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82202D2C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82202D30: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82202D34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82202D38: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202D3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202D40: 419A0034  beq cr6, 0x82202d74
	if ctx.cr[6].eq {
	pc = 0x82202D74; continue 'dispatch;
	}
	// 82202D44: 3BDD0014  addi r30, r29, 0x14
	ctx.r[30].s64 = ctx.r[29].s64 + 20;
	// 82202D48: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202D4C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82202D50: 48178961  bl 0x8237b6b0
	ctx.lr = 0x82202D54;
	sub_8237B6B0(ctx, base);
	// 82202D54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82202D58: 409A0008  bne cr6, 0x82202d60
	if !ctx.cr[6].eq {
	pc = 0x82202D60; continue 'dispatch;
	}
	// 82202D5C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82202D60: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202D64: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82202D68: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82202D6C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82202D70: 4198FFD8  blt cr6, 0x82202d48
	if ctx.cr[6].lt {
	pc = 0x82202D48; continue 'dispatch;
	}
	// 82202D74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82202D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82202D7C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82202D80: 48332388  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82202D88 size=244
    let mut pc: u32 = 0x82202D88;
    'dispatch: loop {
        match pc {
            0x82202D88 => {
    //   block [0x82202D88..0x82202E7C)
	// 82202D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82202D8C: 48332325  bl 0x825350b0
	ctx.lr = 0x82202D90;
	sub_82535080(ctx, base);
	// 82202D90: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82202D94: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202D98: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82202D9C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82202DA0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82202DA4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82202DA8: 897C0010  lbz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202DAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202DB0: 419A0078  beq cr6, 0x82202e28
	if ctx.cr[6].eq {
	pc = 0x82202E28; continue 'dispatch;
	}
	// 82202DB4: 3D604754  lis r11, 0x4754
	ctx.r[11].s64 = 1196687360;
	// 82202DB8: 3BDC0014  addi r30, r28, 0x14
	ctx.r[30].s64 = ctx.r[28].s64 + 20;
	// 82202DBC: 617B4D57  ori r27, r11, 0x4d57
	ctx.r[27].u64 = ctx.r[11].u64 | 19799;
	// 82202DC0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202DC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202DC8: 419A0048  beq cr6, 0x82202e10
	if ctx.cr[6].eq {
	pc = 0x82202E10; continue 'dispatch;
	}
	// 82202DCC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202DD0: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82202DD4: 409A003C  bne cr6, 0x82202e10
	if !ctx.cr[6].eq {
	pc = 0x82202E10; continue 'dispatch;
	}
	// 82202DD8: 83EB0028  lwz r31, 0x28(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82202DDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82202DE0: 419A0030  beq cr6, 0x82202e10
	if ctx.cr[6].eq {
	pc = 0x82202E10; continue 'dispatch;
	}
	// 82202DE4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202DE8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82202DEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202DF0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82202DF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82202DF8: 4E800421  bctrl
	ctx.lr = 0x82202DFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82202DFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82202E00: 409A0038  bne cr6, 0x82202e38
	if !ctx.cr[6].eq {
	pc = 0x82202E38; continue 'dispatch;
	}
	// 82202E04: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82202E08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82202E0C: 409AFFD8  bne cr6, 0x82202de4
	if !ctx.cr[6].eq {
	pc = 0x82202DE4; continue 'dispatch;
	}
	// 82202E10: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82202E14: 897C0010  lbz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202E18: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82202E1C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82202E20: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82202E24: 4198FF9C  blt cr6, 0x82202dc0
	if ctx.cr[6].lt {
	pc = 0x82202DC0; continue 'dispatch;
	}
	// 82202E28: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82202E2C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82202E30: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82202E34: 483322CC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82202E38: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202E3C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82202E40: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82202E44: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202E48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202E4C: 419AFFC8  beq cr6, 0x82202e14
	if ctx.cr[6].eq {
	pc = 0x82202E14; continue 'dispatch;
	}
	// 82202E50: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202E54: 55080020  rlwinm r8, r8, 0, 0, 0x10
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82202E58: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82202E5C: 419A0008  beq cr6, 0x82202e64
	if ctx.cr[6].eq {
	pc = 0x82202E64; continue 'dispatch;
	}
	// 82202E60: D3E90000  stfs f31, 0(r9)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82202E64: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82202E68: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82202E6C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82202E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202E74: 409AFFDC  bne cr6, 0x82202e50
	if !ctx.cr[6].eq {
	pc = 0x82202E50; continue 'dispatch;
	}
	// 82202E78: 4BFFFF9C  b 0x82202e14
	pc = 0x82202E14; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202E80 size=88
    let mut pc: u32 = 0x82202E80;
    'dispatch: loop {
        match pc {
            0x82202E80 => {
    //   block [0x82202E80..0x82202ED8)
	// 82202E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82202E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82202E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82202E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82202E90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82202E98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82202E9C: 4BFFEFC5  bl 0x82201e60
	ctx.lr = 0x82202EA0;
	sub_82201E60(ctx, base);
	// 82202EA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82202EA4: 419A001C  beq cr6, 0x82202ec0
	if ctx.cr[6].eq {
	pc = 0x82202EC0; continue 'dispatch;
	}
	// 82202EA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202EAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82202EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82202EB4: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82202EB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82202EBC: 4E800421  bctrl
	ctx.lr = 0x82202EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82202EC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82202EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82202EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82202ECC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82202ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82202ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202ED8 size=124
    let mut pc: u32 = 0x82202ED8;
    'dispatch: loop {
        match pc {
            0x82202ED8 => {
    //   block [0x82202ED8..0x82202F54)
	// 82202ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82202EDC: 483321D9  bl 0x825350b4
	ctx.lr = 0x82202EE0;
	sub_82535080(ctx, base);
	// 82202EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202EE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82202EE8: 89440010  lbz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202EEC: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202EF0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82202EF4: 419A0010  beq cr6, 0x82202f04
	if ctx.cr[6].eq {
	pc = 0x82202F04; continue 'dispatch;
	}
	// 82202EF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82202EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82202F00: 48332204  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82202F04: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82202F08: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82202F0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82202F10: 419A0038  beq cr6, 0x82202f48
	if ctx.cr[6].eq {
	pc = 0x82202F48; continue 'dispatch;
	}
	// 82202F14: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 82202F18: 7F9D2050  subf r28, r29, r4
	ctx.r[28].s64 = ctx.r[4].s64 - ctx.r[29].s64;
	// 82202F1C: 7C9CF82E  lwzx r4, r28, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82202F20: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202F24: 48178845  bl 0x8237b768
	ctx.lr = 0x82202F28;
	sub_8237B768(ctx, base);
	// 82202F28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82202F2C: 409A0008  bne cr6, 0x82202f34
	if !ctx.cr[6].eq {
	pc = 0x82202F34; continue 'dispatch;
	}
	// 82202F30: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82202F34: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202F38: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82202F3C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82202F40: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82202F44: 4198FFD8  blt cr6, 0x82202f1c
	if ctx.cr[6].lt {
	pc = 0x82202F1C; continue 'dispatch;
	}
	// 82202F48: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82202F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82202F50: 483321B4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82202F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82202F58 size=588
    let mut pc: u32 = 0x82202F58;
    'dispatch: loop {
        match pc {
            0x82202F58 => {
    //   block [0x82202F58..0x822031A4)
	// 82202F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82202F5C: 48332141  bl 0x8253509c
	ctx.lr = 0x82202F60;
	sub_82535080(ctx, base);
	// 82202F60: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82202F64: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82202F68: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82202F6C: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82202F70: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82202F74: 88B70010  lbz r5, 0x10(r23)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 82202F78: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82202F7C: 419A0028  beq cr6, 0x82202fa4
	if ctx.cr[6].eq {
	pc = 0x82202FA4; continue 'dispatch;
	}
	// 82202F80: 38D70014  addi r6, r23, 0x14
	ctx.r[6].s64 = ctx.r[23].s64 + 20;
	// 82202F84: 80660000  lwz r3, 0(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82202F88: 48178961  bl 0x8237b8e8
	ctx.lr = 0x82202F8C;
	sub_8237B8E8(ctx, base);
	// 82202F8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82202F90: 409A0020  bne cr6, 0x82202fb0
	if !ctx.cr[6].eq {
	pc = 0x82202FB0; continue 'dispatch;
	}
	// 82202F94: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82202F98: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82202F9C: 7F192840  cmplw cr6, r25, r5
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82202FA0: 4198FFE4  blt cr6, 0x82202f84
	if ctx.cr[6].lt {
	pc = 0x82202F84; continue 'dispatch;
	}
	// 82202FA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82202FA8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82202FAC: 48332140  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
	// 82202FB0: 89430007  lbz r10, 7(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(7 as u32) ) } as u64;
	// 82202FB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82202FB8: 2B0A000F  cmplwi cr6, r10, 0xf
	ctx.cr[6].compare_u32(ctx.r[10].u32, 15 as u32, &mut ctx.xer);
	// 82202FBC: 41980008  blt cr6, 0x82202fc4
	if ctx.cr[6].lt {
	pc = 0x82202FC4; continue 'dispatch;
	}
	// 82202FC0: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82202FC4: 557E043E  clrlwi r30, r11, 0x10
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82202FC8: 2B0A000F  cmplwi cr6, r10, 0xf
	ctx.cr[6].compare_u32(ctx.r[10].u32, 15 as u32, &mut ctx.xer);
	// 82202FCC: 4098000C  bge cr6, 0x82202fd8
	if !ctx.cr[6].lt {
	pc = 0x82202FD8; continue 'dispatch;
	}
	// 82202FD0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82202FD4: 48000010  b 0x82202fe4
	pc = 0x82202FE4; continue 'dispatch;
	// 82202FD8: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82202FDC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82202FE0: 83AB0020  lwz r29, 0x20(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82202FE4: 2B0A000F  cmplwi cr6, r10, 0xf
	ctx.cr[6].compare_u32(ctx.r[10].u32, 15 as u32, &mut ctx.xer);
	// 82202FE8: 4098000C  bge cr6, 0x82202ff4
	if !ctx.cr[6].lt {
	pc = 0x82202FF4; continue 'dispatch;
	}
	// 82202FEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82202FF0: 48000034  b 0x82203024
	pc = 0x82203024; continue 'dispatch;
	// 82202FF4: A143000E  lhz r10, 0xe(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 82202FF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82202FFC: 409A000C  bne cr6, 0x82203008
	if !ctx.cr[6].eq {
	pc = 0x82203008; continue 'dispatch;
	}
	// 82203000: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82203004: 48000020  b 0x82203024
	pc = 0x82203024; continue 'dispatch;
	// 82203008: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8220300C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82203010: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82203014: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82203018: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8220301C: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82203020: 557F0036  rlwinm r31, r11, 0, 0, 0x1b
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82203024: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 82203028: 419A0128  beq cr6, 0x82203150
	if ctx.cr[6].eq {
	pc = 0x82203150; continue 'dispatch;
	}
	// 8220302C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82203030: 419A0168  beq cr6, 0x82203198
	if ctx.cr[6].eq {
	pc = 0x82203198; continue 'dispatch;
	}
	// 82203034: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82203038: 7FD8F378  mr r24, r30
	ctx.r[24].u64 = ctx.r[30].u64;
	// 8220303C: 3BCBB620  addi r30, r11, -0x49e0
	ctx.r[30].s64 = ctx.r[11].s64 + -18912;
	// 82203040: 3B400030  li r26, 0x30
	ctx.r[26].s64 = 48;
	// 82203044: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82203048: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 8220304C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203050: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203054: 409A00BC  bne cr6, 0x82203110
	if !ctx.cr[6].eq {
	pc = 0x82203110; continue 'dispatch;
	}
	// 82203058: 39790025  addi r11, r25, 0x25
	ctx.r[11].s64 = ctx.r[25].s64 + 37;
	// 8220305C: A15D0002  lhz r10, 2(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822031A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822031A8 size=232
    let mut pc: u32 = 0x822031A8;
    'dispatch: loop {
        match pc {
            0x822031A8 => {
    //   block [0x822031A8..0x82203290)
	// 822031A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822031AC: 48331F09  bl 0x825350b4
	ctx.lr = 0x822031B0;
	sub_82535080(ctx, base);
	// 822031B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822031B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822031B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822031BC: 897C0010  lbz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 822031C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822031C4: 419A00B4  beq cr6, 0x82203278
	if ctx.cr[6].eq {
	pc = 0x82203278; continue 'dispatch;
	}
	// 822031C8: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 822031CC: 3BDC0014  addi r30, r28, 0x14
	ctx.r[30].s64 = ctx.r[28].s64 + 20;
	// 822031D0: 3BEB86B8  addi r31, r11, -0x7948
	ctx.r[31].s64 = ctx.r[11].s64 + -31048;
	// 822031D4: 3D604754  lis r11, 0x4754
	ctx.r[11].s64 = 1196687360;
	// 822031D8: 617B4D57  ori r27, r11, 0x4d57
	ctx.r[27].u64 = ctx.r[11].u64 | 19799;
	// 822031DC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822031E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822031E4: 419A0058  beq cr6, 0x8220323c
	if ctx.cr[6].eq {
	pc = 0x8220323C; continue 'dispatch;
	}
	// 822031E8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822031EC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 822031F0: 409A004C  bne cr6, 0x8220323c
	if !ctx.cr[6].eq {
	pc = 0x8220323C; continue 'dispatch;
	}
	// 822031F4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822031F8: 2B0901FF  cmplwi cr6, r9, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 511 as u32, &mut ctx.xer);
	// 822031FC: 40980040  bge cr6, 0x8220323c
	if !ctx.cr[6].lt {
	pc = 0x8220323C; continue 'dispatch;
	}
	// 82203200: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82203204: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82203208: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8220320C: 7D0BF8AE  lbzx r8, r11, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82203210: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 82203214: 409A0028  bne cr6, 0x8220323c
	if !ctx.cr[6].eq {
	pc = 0x8220323C; continue 'dispatch;
	}
	// 82203218: 391F0008  addi r8, r31, 8
	ctx.r[8].s64 = ctx.r[31].s64 + 8;
	// 8220321C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82203220: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82203224: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203228: 409A0014  bne cr6, 0x8220323c
	if !ctx.cr[6].eq {
	pc = 0x8220323C; continue 'dispatch;
	}
	// 8220322C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82203230: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82203234: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82203238: 48000008  b 0x82203240
	pc = 0x82203240; continue 'dispatch;
	// 8220323C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82203240: 3CE00410  lis r7, 0x410
	ctx.r[7].s64 = 68157440;
	// 82203244: 3CC00410  lis r6, 0x410
	ctx.r[6].s64 = 68157440;
	// 82203248: 60E70011  ori r7, r7, 0x11
	ctx.r[7].u64 = ctx.r[7].u64 | 17;
	// 8220324C: 60C60012  ori r6, r6, 0x12
	ctx.r[6].u64 = ctx.r[6].u64 | 18;
	// 82203250: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82203254: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82203258: 48178941  bl 0x8237bb98
	ctx.lr = 0x8220325C;
	sub_8237BB98(ctx, base);
	// 8220325C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82203260: 419A0024  beq cr6, 0x82203284
	if ctx.cr[6].eq {
	pc = 0x82203284; continue 'dispatch;
	}
	// 82203264: 897C0010  lbz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82203268: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8220326C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82203270: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82203274: 4198FF68  blt cr6, 0x822031dc
	if ctx.cr[6].lt {
	pc = 0x822031DC; continue 'dispatch;
	}
	// 82203278: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8220327C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82203280: 48331E84  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82203284: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82203288: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8220328C: 48331E78  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82203290 size=104
    let mut pc: u32 = 0x82203290;
    'dispatch: loop {
        match pc {
            0x82203290 => {
    //   block [0x82203290..0x822032F8)
	// 82203290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82203294: 48331E25  bl 0x825350b8
	ctx.lr = 0x82203298;
	sub_82535080(ctx, base);
	// 82203298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220329C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822032A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822032A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822032A8: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 822032AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822032B0: 419A0030  beq cr6, 0x822032e0
	if ctx.cr[6].eq {
	pc = 0x822032E0; continue 'dispatch;
	}
	// 822032B4: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 822032B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822032BC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822032C0: 48000DE1  bl 0x822040a0
	ctx.lr = 0x822032C4;
	sub_822040A0(ctx, base);
	// 822032C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822032C8: 419A0024  beq cr6, 0x822032ec
	if ctx.cr[6].eq {
	pc = 0x822032EC; continue 'dispatch;
	}
	// 822032CC: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 822032D0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 822032D4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 822032D8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822032DC: 4198FFDC  blt cr6, 0x822032b8
	if ctx.cr[6].lt {
	pc = 0x822032B8; continue 'dispatch;
	}
	// 822032E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822032E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822032E8: 48331E20  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 822032EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822032F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822032F4: 48331E14  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822032F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822032F8 size=612
    let mut pc: u32 = 0x822032F8;
    'dispatch: loop {
        match pc {
            0x822032F8 => {
    //   block [0x822032F8..0x8220355C)
	// 822032F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822032FC: 48331D95  bl 0x82535090
	ctx.lr = 0x82203300;
	sub_82535080(ctx, base);
	// 82203300: 81230114  lwz r9, 0x114(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 82203304: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82203308: 419A0210  beq cr6, 0x82203518
	if ctx.cr[6].eq {
	pc = 0x82203518; continue 'dispatch;
	}
	// 8220330C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82203310: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82203314: 61689F62  ori r8, r11, 0x9f62
	ctx.r[8].u64 = ctx.r[11].u64 | 40802;
	// 82203318: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8220331C: 552A003E  slwi r10, r9, 0
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82203320: 61679F60  ori r7, r11, 0x9f60
	ctx.r[7].u64 = ctx.r[11].u64 | 40800;
	// 82203324: 3961FF74  addi r11, r1, -0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + -140;
	// 82203328: 9361FF70  stw r27, -0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.r[27].u32 ) };
	// 8220332C: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 82203330: 7F2A422E  lhzx r25, r10, r8
	ctx.r[25].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82203334: 7FA93A2E  lhzx r29, r9, r7
	ctx.r[29].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82203338: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8220333C: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82203340: 7F1DC840  cmplw cr6, r29, r25
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82203344: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82203348: 409801D0  bge cr6, 0x82203518
	if !ctx.cr[6].lt {
	pc = 0x82203518; continue 'dispatch;
	}
	// 8220334C: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82203350: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82203354: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82203358: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8220335C: 3D608286  lis r11, -0x7d7a
	ctx.r[11].s64 = -2105147392;
	// 82203360: 3B8BF630  addi r28, r11, -0x9d0
	ctx.r[28].s64 = ctx.r[11].s64 + -2512;
	// 82203364: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82203368: 386B3B0C  addi r3, r11, 0x3b0c
	ctx.r[3].s64 = ctx.r[11].s64 + 15116;
	// 8220336C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82203370: 3BEB3AD4  addi r31, r11, 0x3ad4
	ctx.r[31].s64 = ctx.r[11].s64 + 15060;
	// 82203374: 8967004B  lbz r11, 0x4b(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(75 as u32) ) } as u64;
	// 82203378: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 8220337C: 409A018C  bne cr6, 0x82203508
	if !ctx.cr[6].eq {
	pc = 0x82203508; continue 'dispatch;
	}
	// 82203380: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82203384: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82203388: 3961FF70  addi r11, r1, -0x90
	ctx.r[11].s64 = ctx.r[1].s64 + -144;
	// 8220338C: 7D64582E  lwzx r11, r4, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82203390: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203394: 409A0038  bne cr6, 0x822033cc
	if !ctx.cr[6].eq {
	pc = 0x822033CC; continue 'dispatch;
	}
	// 82203398: 8147002C  lwz r10, 0x2c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) } as u64;
	// 8220339C: 7D64E02E  lwzx r11, r4, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 822033A0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822033A4: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822033A8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822033AC: 7D084850  subf r8, r8, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 822033B0: 419A0014  beq cr6, 0x822033c4
	if ctx.cr[6].eq {
	pc = 0x822033C4; continue 'dispatch;
	}
	// 822033B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822033B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822033BC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 822033C0: 419AFFE0  beq cr6, 0x822033a0
	if ctx.cr[6].eq {
	pc = 0x822033A0; continue 'dispatch;
	}
	// 822033C4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 822033C8: 419A0018  beq cr6, 0x822033e0
	if ctx.cr[6].eq {
	pc = 0x822033E0; continue 'dispatch;
	}
	// 822033CC: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 822033D0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 822033D4: 2B040010  cmplwi cr6, r4, 0x10
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16 as u32, &mut ctx.xer);
	// 822033D8: 4198FFB0  blt cr6, 0x82203388
	if ctx.cr[6].lt {
	pc = 0x82203388; continue 'dispatch;
	}
	// 822033DC: 4800012C  b 0x82203508
	pc = 0x82203508; continue 'dispatch;
	// 822033E0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 822033E4: 39430008  addi r10, r3, 8
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	// 822033E8: 1D6B001C  mulli r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 * 28;
	// 822033EC: 7DABFC2E  lfsx f13, r11, r31
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822033F0: 7C0B1C2E  lfsx f0, r11, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822033F4: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 822033F8: 393F0008  addi r9, r31, 8
	ctx.r[9].s64 = ctx.r[31].s64 + 8;
	// 822033FC: 3903000C  addi r8, r3, 0xc
	ctx.r[8].s64 = ctx.r[3].s64 + 12;
	// 82203400: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82203404: 3B030010  addi r24, r3, 0x10
	ctx.r[24].s64 = ctx.r[3].s64 + 16;
	// 82203408: 3AFF0010  addi r23, r31, 0x10
	ctx.r[23].s64 = ctx.r[31].s64 + 16;
	// 8220340C: 3AC30014  addi r22, r3, 0x14
	ctx.r[22].s64 = ctx.r[3].s64 + 20;
	// 82203410: 3ABF0014  addi r21, r31, 0x14
	ctx.r[21].s64 = ctx.r[31].s64 + 20;
	// 82203414: 3A830018  addi r20, r3, 0x18
	ctx.r[20].s64 = ctx.r[3].s64 + 24;
	// 82203418: 3A7F0018  addi r19, r31, 0x18
	ctx.r[19].s64 = ctx.r[31].s64 + 24;
	// 8220341C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82203420: D801FF60  stfd f0, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[0].u64 ) };
	// 82203424: A241FF66  lhz r18, -0x9a(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-154 as u32) ) } as u64;
	// 82203428: B2470038  sth r18, 0x38(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(56 as u32), ctx.r[18].u16 ) };
	// 8220342C: 7C0B542E  lfsx f0, r11, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82203430: 7DAB4C2E  lfsx f13, r11, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82203434: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82203438: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8220343C: D801FF60  stfd f0, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[0].u64 ) };
	// 82203440: A141FF66  lhz r10, -0x9a(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-154 as u32) ) } as u64;
	// 82203444: B147003A  sth r10, 0x3a(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(58 as u32), ctx.r[10].u16 ) };
	// 82203448: 7DAB242E  lfsx f13, r11, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8220344C: 7C0B442E  lfsx f0, r11, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82203450: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82203454: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82203458: D801FF60  stfd f0, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[0].u64 ) };
	// 8220345C: 8941FF67  lbz r10, -0x99(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-153 as u32) ) } as u64;
	// 82203460: 9947003C  stb r10, 0x3c(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(60 as u32), ctx.r[10].u8 ) };
	// 82203464: 7DABBC2E  lfsx f13, r11, r23
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82203468: 7C0BC42E  lfsx f0, r11, r24
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8220346C: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82203470: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82203474: D801FF60  stfd f0, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[0].u64 ) };
	// 82203478: 8941FF67  lbz r10, -0x99(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-153 as u32) ) } as u64;
	// 8220347C: 9947003D  stb r10, 0x3d(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(61 as u32), ctx.r[10].u8 ) };
	// 82203480: 7DABAC2E  lfsx f13, r11, r21
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[21].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82203484: 7C0BB42E  lfsx f0, r11, r22
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[22].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82203488: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 8220348C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82203490: D801FF60  stfd f0, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[0].u64 ) };
	// 82203494: 8941FF67  lbz r10, -0x99(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-153 as u32) ) } as u64;
	// 82203498: 9947003E  stb r10, 0x3e(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(62 as u32), ctx.r[10].u8 ) };
	// 8220349C: 7C0BA42E  lfsx f0, r11, r20
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[20].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822034A0: 7DAB9C2E  lfsx f13, r11, r19
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[19].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822034A4: 89470044  lbz r10, 0x44(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(68 as u32) ) } as u64;
	// 822034A8: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 822034AC: 554A077A  rlwinm r10, r10, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 822034B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822034B4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 822034B8: D801FF60  stfd f0, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[0].u64 ) };
	// 822034BC: 8941FF67  lbz r10, -0x99(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-153 as u32) ) } as u64;
	// 822034C0: 9947003F  stb r10, 0x3f(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(63 as u32), ctx.r[10].u8 ) };
	// 822034C4: 419A002C  beq cr6, 0x822034f0
	if ctx.cr[6].eq {
	pc = 0x822034F0; continue 'dispatch;
	}
	// 822034C8: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 822034CC: 81470030  lwz r10, 0x30(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) } as u64;
	// 822034D0: 391F0004  addi r8, r31, 4
	ctx.r[8].s64 = ctx.r[31].s64 + 4;
	// 822034D4: 7C0B4C2E  lfsx f0, r11, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822034D8: 7DAB442E  lfsx f13, r11, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822034DC: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 822034E0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 822034E4: D801FF60  stfd f0, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[0].u64 ) };
	// 822034E8: A161FF66  lhz r11, -0x9a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-154 as u32) ) } as u64;
	// 822034EC: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 822034F0: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822034F4: 3941FF70  addi r10, r1, -0x90
	ctx.r[10].s64 = ctx.r[1].s64 + -144;
	// 822034F8: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 822034FC: 2B1A0004  cmplwi cr6, r26, 4
	ctx.cr[6].compare_u32(ctx.r[26].u32, 4 as u32, &mut ctx.xer);
	// 82203500: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82203504: 4098001C  bge cr6, 0x82203520
	if !ctx.cr[6].lt {
	pc = 0x82203520; continue 'dispatch;
	}
	// 82203508: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8220350C: 38E70050  addi r7, r7, 0x50
	ctx.r[7].s64 = ctx.r[7].s64 + 80;
	// 82203510: 7F1DC840  cmplw cr6, r29, r25
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82203514: 4198FE60  blt cr6, 0x82203374
	if ctx.cr[6].lt {
	pc = 0x82203374; continue 'dispatch;
	}
	// 82203518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8220351C: 48331BC4  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
	// 82203520: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82203524: 419A0030  beq cr6, 0x82203554
	if ctx.cr[6].eq {
	pc = 0x82203554; continue 'dispatch;
	}
	// 82203528: 2B060004  cmplwi cr6, r6, 4
	ctx.cr[6].compare_u32(ctx.r[6].u32, 4 as u32, &mut ctx.xer);
	// 8220352C: 41980028  blt cr6, 0x82203554
	if ctx.cr[6].lt {
	pc = 0x82203554; continue 'dispatch;
	}
	// 82203530: 3961FF70  addi r11, r1, -0x90
	ctx.r[11].s64 = ctx.r[1].s64 + -144;
	// 82203534: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203538: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8220353C: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82203540: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82203544: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82203548: 91250004  stw r9, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8220354C: 91050008  stw r8, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82203550: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82203554: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82203558: 48331B88  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203560 size=372
    let mut pc: u32 = 0x82203560;
    'dispatch: loop {
        match pc {
            0x82203560 => {
    //   block [0x82203560..0x822036D4)
	// 82203560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82203564: 48331B51  bl 0x825350b4
	ctx.lr = 0x82203568;
	sub_82535080(ctx, base);
	// 82203568: 81230114  lwz r9, 0x114(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 8220356C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82203570: 419A0120  beq cr6, 0x82203690
	if ctx.cr[6].eq {
	pc = 0x82203690; continue 'dispatch;
	}
	// 82203574: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82203578: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8220357C: 61689F62  ori r8, r11, 0x9f62
	ctx.r[8].u64 = ctx.r[11].u64 | 40802;
	// 82203580: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82203584: 552A003E  slwi r10, r9, 0
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82203588: 61679F60  ori r7, r11, 0x9f60
	ctx.r[7].u64 = ctx.r[11].u64 | 40800;
	// 8220358C: 3961FFC4  addi r11, r1, -0x3c
	ctx.r[11].s64 = ctx.r[1].s64 + -60;
	// 82203590: 9061FFC0  stw r3, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[3].u32 ) };
	// 82203594: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82203598: 7F6A422E  lhzx r27, r10, r8
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8220359C: 7FE93A2E  lhzx r31, r9, r7
	ctx.r[31].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 822035A0: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 822035A4: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 822035A8: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 822035AC: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 822035B0: 409800E0  bge cr6, 0x82203690
	if !ctx.cr[6].lt {
	pc = 0x82203690; continue 'dispatch;
	}
	// 822035B4: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822035B8: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 822035BC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822035C0: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822035C4: 3D608286  lis r11, -0x7d7a
	ctx.r[11].s64 = -2105147392;
	// 822035C8: 3BABF630  addi r29, r11, -0x9d0
	ctx.r[29].s64 = ctx.r[11].s64 + -2512;
	// 822035CC: 8966004B  lbz r11, 0x4b(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(75 as u32) ) } as u64;
	// 822035D0: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 822035D4: 409A00AC  bne cr6, 0x82203680
	if !ctx.cr[6].eq {
	pc = 0x82203680; continue 'dispatch;
	}
	// 822035D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822035DC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 822035E0: 3961FFC0  addi r11, r1, -0x40
	ctx.r[11].s64 = ctx.r[1].s64 + -64;
	// 822035E4: 7D67582E  lwzx r11, r7, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822035E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822035EC: 409A0038  bne cr6, 0x82203624
	if !ctx.cr[6].eq {
	pc = 0x82203624; continue 'dispatch;
	}
	// 822035F0: 8146002C  lwz r10, 0x2c(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(44 as u32) ) } as u64;
	// 822035F4: 7D67E82E  lwzx r11, r7, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 822035F8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822035FC: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203600: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82203604: 7D084850  subf r8, r8, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82203608: 419A0014  beq cr6, 0x8220361c
	if ctx.cr[6].eq {
	pc = 0x8220361C; continue 'dispatch;
	}
	// 8220360C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82203610: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82203614: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82203618: 419AFFE0  beq cr6, 0x822035f8
	if ctx.cr[6].eq {
	pc = 0x822035F8; continue 'dispatch;
	}
	// 8220361C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82203620: 419A0018  beq cr6, 0x82203638
	if ctx.cr[6].eq {
	pc = 0x82203638; continue 'dispatch;
	}
	// 82203624: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82203628: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8220362C: 2B070010  cmplwi cr6, r7, 0x10
	ctx.cr[6].compare_u32(ctx.r[7].u32, 16 as u32, &mut ctx.xer);
	// 82203630: 4198FFB0  blt cr6, 0x822035e0
	if ctx.cr[6].lt {
	pc = 0x822035E0; continue 'dispatch;
	}
	// 82203634: 4800004C  b 0x82203680
	pc = 0x82203680; continue 'dispatch;
	// 82203638: 89660044  lbz r11, 0x44(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(68 as u32) ) } as u64;
	// 8220363C: B066003A  sth r3, 0x3a(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(58 as u32), ctx.r[3].u16 ) };
	// 82203640: 556B077A  rlwinm r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82203644: 9866003D  stb r3, 0x3d(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(61 as u32), ctx.r[3].u8 ) };
	// 82203648: 9866003C  stb r3, 0x3c(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(60 as u32), ctx.r[3].u8 ) };
	// 8220364C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203650: 9866003F  stb r3, 0x3f(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(63 as u32), ctx.r[3].u8 ) };
	// 82203654: 9866003E  stb r3, 0x3e(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(62 as u32), ctx.r[3].u8 ) };
	// 82203658: 419A0010  beq cr6, 0x82203668
	if ctx.cr[6].eq {
	pc = 0x82203668; continue 'dispatch;
	}
	// 8220365C: 81660030  lwz r11, 0x30(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(48 as u32) ) } as u64;
	// 82203660: B06B0002  sth r3, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[3].u16 ) };
	// 82203664: B06B0004  sth r3, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u16 ) };
	// 82203668: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8220366C: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 82203670: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82203674: 2B1C0004  cmplwi cr6, r28, 4
	ctx.cr[6].compare_u32(ctx.r[28].u32, 4 as u32, &mut ctx.xer);
	// 82203678: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 8220367C: 4098001C  bge cr6, 0x82203698
	if !ctx.cr[6].lt {
	pc = 0x82203698; continue 'dispatch;
	}
	// 82203680: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82203684: 38C60050  addi r6, r6, 0x50
	ctx.r[6].s64 = ctx.r[6].s64 + 80;
	// 82203688: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8220368C: 4198FF40  blt cr6, 0x822035cc
	if ctx.cr[6].lt {
	pc = 0x822035CC; continue 'dispatch;
	}
	// 82203690: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82203694: 48331A70  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82203698: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8220369C: 419A0030  beq cr6, 0x822036cc
	if ctx.cr[6].eq {
	pc = 0x822036CC; continue 'dispatch;
	}
	// 822036A0: 2B050004  cmplwi cr6, r5, 4
	ctx.cr[6].compare_u32(ctx.r[5].u32, 4 as u32, &mut ctx.xer);
	// 822036A4: 41980028  blt cr6, 0x822036cc
	if ctx.cr[6].lt {
	pc = 0x822036CC; continue 'dispatch;
	}
	// 822036A8: 3961FFC0  addi r11, r1, -0x40
	ctx.r[11].s64 = ctx.r[1].s64 + -64;
	// 822036AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822036B0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822036B4: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822036B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822036BC: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822036C0: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822036C4: 91040008  stw r8, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 822036C8: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 822036CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822036D0: 48331A34  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822036D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822036D8 size=732
    let mut pc: u32 = 0x822036D8;
    'dispatch: loop {
        match pc {
            0x822036D8 => {
    //   block [0x822036D8..0x822039B4)
	// 822036D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822036DC: 483319AD  bl 0x82535088
	ctx.lr = 0x822036E0;
	sub_82535080(ctx, base);
	// 822036E0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822036E4: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 822036E8: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 822036EC: 2B190002  cmplwi cr6, r25, 2
	ctx.cr[6].compare_u32(ctx.r[25].u32, 2 as u32, &mut ctx.xer);
	// 822036F0: 409802B8  bge cr6, 0x822039a8
	if !ctx.cr[6].lt {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 822036F4: 2B050064  cmplwi cr6, r5, 0x64
	ctx.cr[6].compare_u32(ctx.r[5].u32, 100 as u32, &mut ctx.xer);
	// 822036F8: 40990008  ble cr6, 0x82203700
	if !ctx.cr[6].gt {
	pc = 0x82203700; continue 'dispatch;
	}
	// 822036FC: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 82203700: 89720010  lbz r11, 0x10(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[18].u32.wrapping_add(16 as u32) ) } as u64;
	// 82203704: 39400019  li r10, 0x19
	ctx.r[10].s64 = 25;
	// 82203708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8220370C: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82203710: 7F455396  divwu r26, r5, r10
	ctx.r[26].u32 = ctx.r[5].u32 / ctx.r[10].u32;
	// 82203714: 7D795A14  add r11, r25, r11
	ctx.r[11].u64 = ctx.r[25].u64 + ctx.r[11].u64;
	// 82203718: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8220371C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82203720: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82203724: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82203728: 3D0B0607  addis r8, r11, 0x607
	ctx.r[8].s64 = ctx.r[11].s64 + 101122048;
	// 8220372C: 39088113  addi r8, r8, -0x7eed
	ctx.r[8].s64 = ctx.r[8].s64 + -32493;
	// 82203730: 419A0278  beq cr6, 0x822039a8
	if ctx.cr[6].eq {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 82203734: 3D404754  lis r10, 0x4754
	ctx.r[10].s64 = 1196687360;
	// 82203738: 81720014  lwz r11, 0x14(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220373C: 3B120014  addi r24, r18, 0x14
	ctx.r[24].s64 = ctx.r[18].s64 + 20;
	// 82203740: 61504D57  ori r16, r10, 0x4d57
	ctx.r[16].u64 = ctx.r[10].u64 | 19799;
	// 82203744: 3D408310  lis r10, -0x7cf0
	ctx.r[10].s64 = -2096103424;
	// 82203748: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 8220374C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203750: 3AAA86B8  addi r21, r10, -0x7948
	ctx.r[21].s64 = ctx.r[10].s64 + -31048;
	// 82203754: 419A0154  beq cr6, 0x822038a8
	if ctx.cr[6].eq {
	pc = 0x822038A8; continue 'dispatch;
	}
	// 82203758: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220375C: 7F0A8040  cmplw cr6, r10, r16
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[16].u32, &mut ctx.xer);
	// 82203760: 409A0148  bne cr6, 0x822038a8
	if !ctx.cr[6].eq {
	pc = 0x822038A8; continue 'dispatch;
	}
	// 82203764: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82203768: 2B0901FF  cmplwi cr6, r9, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 511 as u32, &mut ctx.xer);
	// 8220376C: 4098023C  bge cr6, 0x822039a8
	if !ctx.cr[6].lt {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 82203770: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82203774: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82203778: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8220377C: 7CEAA8AE  lbzx r7, r10, r21
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82203780: 2B070001  cmplwi cr6, r7, 1
	ctx.cr[6].compare_u32(ctx.r[7].u32, 1 as u32, &mut ctx.xer);
	// 82203784: 409A0224  bne cr6, 0x822039a8
	if !ctx.cr[6].eq {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 82203788: 38F50008  addi r7, r21, 8
	ctx.r[7].s64 = ctx.r[21].s64 + 8;
	// 8220378C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82203790: 7D4A382E  lwzx r10, r10, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82203794: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82203798: 409A0210  bne cr6, 0x822039a8
	if !ctx.cr[6].eq {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 8220379C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 822037A0: 2B0B013D  cmplwi cr6, r11, 0x13d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 317 as u32, &mut ctx.xer);
	// 822037A4: 40980204  bge cr6, 0x822039a8
	if !ctx.cr[6].lt {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 822037A8: 396BFFC3  addi r11, r11, -0x3d
	ctx.r[11].s64 = ctx.r[11].s64 + -61;
	// 822037AC: 3BC0000C  li r30, 0xc
	ctx.r[30].s64 = 12;
	// 822037B0: 556BD97E  srwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822037B4: 396B0104  addi r11, r11, 0x104
	ctx.r[11].s64 = ctx.r[11].s64 + 260;
	// 822037B8: 557DA814  slwi r29, r11, 0x15
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(21);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 822037BC: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 822037C0: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 822037C4: 7F7D4050  subf r27, r29, r8
	ctx.r[27].s64 = ctx.r[8].s64 - ctx.r[29].s64;
	// 822037C8: 3B8BF080  addi r28, r11, -0xf80
	ctx.r[28].s64 = ctx.r[11].s64 + -3968;
	// 822037CC: 7CBBFA14  add r5, r27, r31
	ctx.r[5].u64 = ctx.r[27].u64 + ctx.r[31].u64;
	// 822037D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822037D4: 387C1600  addi r3, r28, 0x1600
	ctx.r[3].s64 = ctx.r[28].s64 + 5632;
	// 822037D8: 481A54C1  bl 0x823a8c98
	ctx.lr = 0x822037DC;
	sub_823A8C98(ctx, base);
	// 822037DC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 822037E0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 822037E4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822037E8: 409AFFE4  bne cr6, 0x822037cc
	if !ctx.cr[6].eq {
	pc = 0x822037CC; continue 'dispatch;
	}
	// 822037EC: 3D790135  addis r11, r25, 0x135
	ctx.r[11].s64 = ctx.r[25].s64 + 20250624;
	// 822037F0: 3C9D0010  addis r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 1048576;
	// 822037F4: 396B804F  addi r11, r11, -0x7fb1
	ctx.r[11].s64 = ctx.r[11].s64 + -32689;
	// 822037F8: 387C1600  addi r3, r28, 0x1600
	ctx.r[3].s64 = ctx.r[28].s64 + 5632;
	// 822037FC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82203800: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82203804: 7CABD214  add r5, r11, r26
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82203808: 481A5491  bl 0x823a8c98
	ctx.lr = 0x8220380C;
	sub_823A8C98(ctx, base);
	// 8220380C: 89720010  lbz r11, 0x10(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[18].u32.wrapping_add(16 as u32) ) } as u64;
	// 82203810: 7E338B78  mr r19, r17
	ctx.r[19].u64 = ctx.r[17].u64;
	// 82203814: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203818: 419A0184  beq cr6, 0x8220399c
	if ctx.cr[6].eq {
	pc = 0x8220399C; continue 'dispatch;
	}
	// 8220381C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82203820: 575C2036  slwi r28, r26, 4
	ctx.r[28].u32 = ctx.r[26].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82203824: 3B4BD060  addi r26, r11, -0x2fa0
	ctx.r[26].s64 = ctx.r[11].s64 + -12192;
	// 82203828: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8220382C: 3ADA000C  addi r22, r26, 0xc
	ctx.r[22].s64 = ctx.r[26].s64 + 12;
	// 82203830: 3B6B9580  addi r27, r11, -0x6a80
	ctx.r[27].s64 = ctx.r[11].s64 + -27264;
	// 82203834: 3D608287  lis r11, -0x7d79
	ctx.r[11].s64 = -2105081856;
	// 82203838: 3AFA0008  addi r23, r26, 8
	ctx.r[23].s64 = ctx.r[26].s64 + 8;
	// 8220383C: 3B3A0004  addi r25, r26, 4
	ctx.r[25].s64 = ctx.r[26].s64 + 4;
	// 82203840: 3A8BBAE0  addi r20, r11, -0x4520
	ctx.r[20].s64 = ctx.r[11].s64 + -17696;
	// 82203844: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203848: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8220384C: 419A0064  beq cr6, 0x822038b0
	if ctx.cr[6].eq {
	pc = 0x822038B0; continue 'dispatch;
	}
	// 82203850: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203854: 7F0B8040  cmplw cr6, r11, r16
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[16].u32, &mut ctx.xer);
	// 82203858: 409A0058  bne cr6, 0x822038b0
	if !ctx.cr[6].eq {
	pc = 0x822038B0; continue 'dispatch;
	}
	// 8220385C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82203860: 2B0901FF  cmplwi cr6, r9, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 511 as u32, &mut ctx.xer);
	// 82203864: 4098004C  bge cr6, 0x822038b0
	if !ctx.cr[6].lt {
	pc = 0x822038B0; continue 'dispatch;
	}
	// 82203868: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8220386C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82203870: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82203874: 7D0BA8AE  lbzx r8, r11, r21
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82203878: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 8220387C: 409A0034  bne cr6, 0x822038b0
	if !ctx.cr[6].eq {
	pc = 0x822038B0; continue 'dispatch;
	}
	// 82203880: 39150008  addi r8, r21, 8
	ctx.r[8].s64 = ctx.r[21].s64 + 8;
	// 82203884: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82203888: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8220388C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203890: 409A0020  bne cr6, 0x822038b0
	if !ctx.cr[6].eq {
	pc = 0x822038B0; continue 'dispatch;
	}
	// 82203894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82203898: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8220389C: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 822038A0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 822038A4: 48000010  b 0x822038b4
	pc = 0x822038B4; continue 'dispatch;
	// 822038A8: 7E2B8B78  mr r11, r17
	ctx.r[11].u64 = ctx.r[17].u64;
	// 822038AC: 4BFFFEFC  b 0x822037a8
	pc = 0x822037A8; continue 'dispatch;
	// 822038B0: FA210050  std r17, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[17].u64 ) };
	// 822038B4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822038B8: 7C1CB42E  lfsx f0, r28, r22
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[22].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822038BC: 7DBCBC2E  lfsx f13, r28, r23
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[23].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822038C0: 7D9CCC2E  lfsx f12, r28, r25
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[25].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822038C4: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 822038C8: 7D7CD42E  lfsx f11, r28, r26
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 822038CC: 409800DC  bge cr6, 0x822039a8
	if !ctx.cr[6].lt {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 822038D0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822038D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822038D8: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822038DC: 7D6AAA14  add r11, r10, r21
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 822038E0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822038E4: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 822038E8: 409A00C0  bne cr6, 0x822039a8
	if !ctx.cr[6].eq {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 822038EC: 39350008  addi r9, r21, 8
	ctx.r[9].s64 = ctx.r[21].s64 + 8;
	// 822038F0: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822038F4: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822038F8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822038FC: 409A00AC  bne cr6, 0x822039a8
	if !ctx.cr[6].eq {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 82203900: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82203904: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82203908: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8220390C: 419A009C  beq cr6, 0x822039a8
	if ctx.cr[6].eq {
	pc = 0x822039A8; continue 'dispatch;
	}
	// 82203910: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82203914: D1610064  stfs f11, 0x64(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82203918: D1810068  stfs f12, 0x68(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8220391C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82203920: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82203924: 92810060  stw r20, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[20].u32 ) };
	// 82203928: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8220392C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82203930: 40980058  bge cr6, 0x82203988
	if !ctx.cr[6].lt {
	pc = 0x82203988; continue 'dispatch;
	}
	// 82203934: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82203938: 387B0080  addi r3, r27, 0x80
	ctx.r[3].s64 = ctx.r[27].s64 + 128;
	// 8220393C: 4819E205  bl 0x823a1b40
	ctx.lr = 0x82203940;
	sub_823A1B40(ctx, base);
	// 82203940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82203944: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82203948: 419A0034  beq cr6, 0x8220397c
	if ctx.cr[6].eq {
	pc = 0x8220397C; continue 'dispatch;
	}
	// 8220394C: 387B0380  addi r3, r27, 0x380
	ctx.r[3].s64 = ctx.r[27].s64 + 896;
	// 82203950: 4850990D  bl 0x8270d25c
	ctx.lr = 0x82203954;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82203954: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203958: 3D60821C  lis r11, -0x7de4
	ctx.r[11].s64 = -2112094208;
	// 8220395C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82203960: 388BC5E0  addi r4, r11, -0x3a20
	ctx.r[4].s64 = ctx.r[11].s64 + -14880;
	// 82203964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82203968: 816A0050  lwz r11, 0x50(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 8220396C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82203970: 4E800421  bctrl
	ctx.lr = 0x82203974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82203974: 387B0380  addi r3, r27, 0x380
	ctx.r[3].s64 = ctx.r[27].s64 + 896;
	// 82203978: 485098F5  bl 0x8270d26c
	ctx.lr = 0x8220397C;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8220397C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82203980: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82203984: 4198FFB0  blt cr6, 0x82203934
	if ctx.cr[6].lt {
	pc = 0x82203934; continue 'dispatch;
	}
	// 82203988: 89720010  lbz r11, 0x10(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[18].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220398C: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 82203990: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 82203994: 7F135840  cmplw cr6, r19, r11
	ctx.cr[6].compare_u32(ctx.r[19].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82203998: 4198FEAC  blt cr6, 0x82203844
	if ctx.cr[6].lt {
	pc = 0x82203844; continue 'dispatch;
	}
	// 8220399C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822039A0: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 822039A4: 48331734  b 0x825350d8
	sub_825350D0(ctx, base);
	return;
	// 822039A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822039AC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 822039B0: 48331728  b 0x825350d8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822039B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822039B8 size=216
    let mut pc: u32 = 0x822039B8;
    'dispatch: loop {
        match pc {
            0x822039B8 => {
    //   block [0x822039B8..0x82203A90)
	// 822039B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822039BC: 483316F5  bl 0x825350b0
	ctx.lr = 0x822039C0;
	sub_82535080(ctx, base);
	// 822039C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822039C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822039C8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 822039CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822039D0: 897C0010  lbz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 822039D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822039D8: 419A00A0  beq cr6, 0x82203a78
	if ctx.cr[6].eq {
	pc = 0x82203A78; continue 'dispatch;
	}
	// 822039DC: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 822039E0: 3BDC0014  addi r30, r28, 0x14
	ctx.r[30].s64 = ctx.r[28].s64 + 20;
	// 822039E4: 3BEB86B8  addi r31, r11, -0x7948
	ctx.r[31].s64 = ctx.r[11].s64 + -31048;
	// 822039E8: 3D604754  lis r11, 0x4754
	ctx.r[11].s64 = 1196687360;
	// 822039EC: 617B4D57  ori r27, r11, 0x4d57
	ctx.r[27].u64 = ctx.r[11].u64 | 19799;
	// 822039F0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822039F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822039F8: 419A0058  beq cr6, 0x82203a50
	if ctx.cr[6].eq {
	pc = 0x82203A50; continue 'dispatch;
	}
	// 822039FC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203A00: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82203A04: 409A004C  bne cr6, 0x82203a50
	if !ctx.cr[6].eq {
	pc = 0x82203A50; continue 'dispatch;
	}
	// 82203A08: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82203A0C: 2B0901FF  cmplwi cr6, r9, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 511 as u32, &mut ctx.xer);
	// 82203A10: 40980040  bge cr6, 0x82203a50
	if !ctx.cr[6].lt {
	pc = 0x82203A50; continue 'dispatch;
	}
	// 82203A14: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82203A18: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82203A1C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82203A20: 7D0BF8AE  lbzx r8, r11, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82203A24: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 82203A28: 409A0028  bne cr6, 0x82203a50
	if !ctx.cr[6].eq {
	pc = 0x82203A50; continue 'dispatch;
	}
	// 82203A2C: 391F0008  addi r8, r31, 8
	ctx.r[8].s64 = ctx.r[31].s64 + 8;
	// 82203A30: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82203A34: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82203A38: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203A3C: 409A0014  bne cr6, 0x82203a50
	if !ctx.cr[6].eq {
	pc = 0x82203A50; continue 'dispatch;
	}
	// 82203A40: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82203A44: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82203A48: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82203A4C: 48000008  b 0x82203a54
	pc = 0x82203A54; continue 'dispatch;
	// 82203A50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82203A54: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82203A58: 4BFB8DF9  bl 0x821bc850
	ctx.lr = 0x82203A5C;
	sub_821BC850(ctx, base);
	// 82203A5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82203A60: 419A0024  beq cr6, 0x82203a84
	if ctx.cr[6].eq {
	pc = 0x82203A84; continue 'dispatch;
	}
	// 82203A64: 897C0010  lbz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82203A68: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82203A6C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82203A70: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82203A74: 4198FF7C  blt cr6, 0x822039f0
	if ctx.cr[6].lt {
	pc = 0x822039F0; continue 'dispatch;
	}
	// 82203A78: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82203A7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82203A80: 48331680  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82203A84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82203A88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82203A8C: 48331674  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203A90 size=8
    let mut pc: u32 = 0x82203A90;
    'dispatch: loop {
        match pc {
            0x82203A90 => {
    //   block [0x82203A90..0x82203A98)
	// 82203A90: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82203A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203A98 size=8
    let mut pc: u32 = 0x82203A98;
    'dispatch: loop {
        match pc {
            0x82203A98 => {
    //   block [0x82203A98..0x82203AA0)
	// 82203A98: 806300F8  lwz r3, 0xf8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(248 as u32) ) } as u64;
	// 82203A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203AA0 size=8
    let mut pc: u32 = 0x82203AA0;
    'dispatch: loop {
        match pc {
            0x82203AA0 => {
    //   block [0x82203AA0..0x82203AA8)
	// 82203AA0: 806300FC  lwz r3, 0xfc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(252 as u32) ) } as u64;
	// 82203AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203AA8 size=24
    let mut pc: u32 = 0x82203AA8;
    'dispatch: loop {
        match pc {
            0x82203AA8 => {
    //   block [0x82203AA8..0x82203AC0)
	// 82203AA8: 816300F4  lwz r11, 0xf4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(244 as u32) ) } as u64;
	// 82203AAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203AB0: 409A0010  bne cr6, 0x82203ac0
	if !ctx.cr[6].eq {
		sub_82203AC0(ctx, base);
		return;
	}
	// 82203AB4: B16300F0  sth r11, 0xf0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[11].u16 ) };
	// 82203AB8: B16300F2  sth r11, 0xf2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(242 as u32), ctx.r[11].u16 ) };
	// 82203ABC: 4800001C  b 0x82203ad8
	sub_82203AC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203AC0 size=32
    let mut pc: u32 = 0x82203AC0;
    'dispatch: loop {
        match pc {
            0x82203AC0 => {
    //   block [0x82203AC0..0x82203AE0)
	// 82203AC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82203AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82203AC8: 3D608287  lis r11, -0x7d79
	ctx.r[11].s64 = -2105081856;
	// 82203ACC: 396B7790  addi r11, r11, 0x7790
	ctx.r[11].s64 = ctx.r[11].s64 + 30608;
	// 82203AD0: B14300F0  sth r10, 0xf0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[10].u16 ) };
	// 82203AD4: B12300F2  sth r9, 0xf2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(242 as u32), ctx.r[9].u16 ) };
	// 82203AD8: 916300EC  stw r11, 0xec(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 82203ADC: 48000794  b 0x82204270
	sub_82204270(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82203AE0 size=128
    let mut pc: u32 = 0x82203AE0;
    'dispatch: loop {
        match pc {
            0x82203AE0 => {
    //   block [0x82203AE0..0x82203B60)
	// 82203AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82203AE4: 483315CD  bl 0x825350b0
	ctx.lr = 0x82203AE8;
	sub_82535080(ctx, base);
	// 82203AE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82203AEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82203AF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82203AF4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82203AF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203AFC: 839F0088  lwz r28, 0x88(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82203B00: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82203B04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82203B08: 4E800421  bctrl
	ctx.lr = 0x82203B0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82203B0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203B10: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82203B14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82203B18: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82203B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82203B20: 4E800421  bctrl
	ctx.lr = 0x82203B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82203B24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203B28: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82203B2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82203B30: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82203B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82203B38: 4E800421  bctrl
	ctx.lr = 0x82203B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82203B3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82203B40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82203B44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82203B48: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82203B4C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82203B50: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82203B54: 480063B5  bl 0x82209f08
	ctx.lr = 0x82203B58;
	sub_82209F08(ctx, base);
	// 82203B58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82203B5C: 483315A4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203B60 size=8
    let mut pc: u32 = 0x82203B60;
    'dispatch: loop {
        match pc {
            0x82203B60 => {
    //   block [0x82203B60..0x82203B68)
	// 82203B60: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82203B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203B68 size=12
    let mut pc: u32 = 0x82203B68;
    'dispatch: loop {
        match pc {
            0x82203B68 => {
    //   block [0x82203B68..0x82203B74)
	// 82203B68: 816300FC  lwz r11, 0xfc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(252 as u32) ) } as u64;
	// 82203B6C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82203B78 size=324
    let mut pc: u32 = 0x82203B78;
    'dispatch: loop {
        match pc {
            0x82203B78 => {
    //   block [0x82203B78..0x82203CBC)
	// 82203B78: 39630090  addi r11, r3, 0x90
	ctx.r[11].s64 = ctx.r[3].s64 + 144;
	// 82203B7C: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82203B80: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82203B84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82203B88: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82203B8C: 90C1002C  stw r6, 0x2c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82203B90: 9161FFE0  stw r11, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[11].u32 ) };
	// 82203B94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82203B98: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82203B9C: 396300E0  addi r11, r3, 0xe0
	ctx.r[11].s64 = ctx.r[3].s64 + 224;
	// 82203BA0: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82203BA4: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82203BA8: 396B8E40  addi r11, r11, -0x71c0
	ctx.r[11].s64 = ctx.r[11].s64 + -29120;
	// 82203BAC: 9161FFE4  stw r11, -0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), ctx.r[11].u32 ) };
	// 82203BB0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82203BB4: 396B3B88  addi r11, r11, 0x3b88
	ctx.r[11].s64 = ctx.r[11].s64 + 15240;
	// 82203BB8: 9161FFE8  stw r11, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[11].u32 ) };
	// 82203BBC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82203BC0: 396B3BBC  addi r11, r11, 0x3bbc
	ctx.r[11].s64 = ctx.r[11].s64 + 15292;
	// 82203BC4: 9161FFEC  stw r11, -0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), ctx.r[11].u32 ) };
	// 82203BC8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82203BCC: 396B8E38  addi r11, r11, -0x71c8
	ctx.r[11].s64 = ctx.r[11].s64 + -29128;
	// 82203BD0: 9161FFF4  stw r11, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 82203BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82203BD8: 8141FFE0  lwz r10, -0x20(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 82203BDC: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82203BE0: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82203BE4: F96A0010  std r11, 0x10(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82203BE8: F96A0018  std r11, 0x18(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82203BEC: F96A0020  std r11, 0x20(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82203BF0: F96A0028  std r11, 0x28(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 82203BF4: F96A0030  std r11, 0x30(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(48 as u32), ctx.r[11].u64 ) };
	// 82203BF8: F96A0038  std r11, 0x38(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u64 ) };
	// 82203BFC: 916A0040  stw r11, 0x40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82203C00: B16A0044  sth r11, 0x44(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(68 as u32), ctx.r[11].u16 ) };
	// 82203C04: B16A0046  sth r11, 0x46(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(70 as u32), ctx.r[11].u16 ) };
	// 82203C08: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82203C0C: 80C1FFE4  lwz r6, -0x1c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82203C10: A0E30014  lhz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82203C14: D003008C  stfs f0, 0x8c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82203C18: 91030084  stw r8, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[8].u32 ) };
	// 82203C1C: 9903001D  stb r8, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[8].u8 ) };
	// 82203C20: 9903001E  stb r8, 0x1e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[8].u8 ) };
	// 82203C24: 90C300D0  stw r6, 0xd0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(208 as u32), ctx.r[6].u32 ) };
	// 82203C28: 81210024  lwz r9, 0x24(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 82203C2C: 91230080  stw r9, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 82203C30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82203C34: 51271EDE  rlwimi r7, r9, 3, 0x1b, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(3) as u64) & 0xFFFFFFFFFFFF001F) | (ctx.r[7].u64 & 0x000000000000FFE0);
	// 82203C38: B0E30014  sth r7, 0x14(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u16 ) };
	// 82203C3C: 916A0048  stw r11, 0x48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82203C40: 916A004C  stw r11, 0x4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82203C44: 8141FFE8  lwz r10, -0x18(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82203C48: 80E1FFEC  lwz r7, -0x14(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82203C4C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82203C50: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82203C54: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82203C58: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82203C5C: B16A0008  sth r11, 8(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 82203C60: B16A000A  sth r11, 0xa(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82203C64: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82203C68: B16A0010  sth r11, 0x10(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 82203C6C: B16A0012  sth r11, 0x12(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 82203C70: 916A0014  stw r11, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82203C74: B16A0018  sth r11, 0x18(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 82203C78: B16A001A  sth r11, 0x1a(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(26 as u32), ctx.r[11].u16 ) };
	// 82203C7C: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82203C80: 914300FC  stw r10, 0xfc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(252 as u32), ctx.r[10].u32 ) };
	// 82203C84: 8141002C  lwz r10, 0x2c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(44 as u32) ) } as u64;
	// 82203C88: 91430088  stw r10, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82203C8C: 8141FFF4  lwz r10, -0xc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82203C90: B12300E8  sth r9, 0xe8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), ctx.r[9].u16 ) };
	// 82203C94: B16300EA  sth r11, 0xea(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(234 as u32), ctx.r[11].u16 ) };
	// 82203C98: 914300E4  stw r10, 0xe4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 82203C9C: 914300EC  stw r10, 0xec(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 82203CA0: B12300F0  sth r9, 0xf0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[9].u16 ) };
	// 82203CA4: B16300F2  sth r11, 0xf2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(242 as u32), ctx.r[11].u16 ) };
	// 82203CA8: 916300F4  stw r11, 0xf4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 82203CAC: B12300F8  sth r9, 0xf8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(248 as u32), ctx.r[9].u16 ) };
	// 82203CB0: B16300FA  sth r11, 0xfa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(250 as u32), ctx.r[11].u16 ) };
	// 82203CB4: 91030100  stw r8, 0x100(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[8].u32 ) };
	// 82203CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203CC0 size=40
    let mut pc: u32 = 0x82203CC0;
    'dispatch: loop {
        match pc {
            0x82203CC0 => {
    //   block [0x82203CC0..0x82203CE8)
	// 82203CC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82203CC4: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 82203CC8: 814B0100  lwz r10, 0x100(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82203CCC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82203CD0: 409A0018  bne cr6, 0x82203ce8
	if !ctx.cr[6].eq {
		sub_82203CE8(ctx, base);
		return;
	}
	// 82203CD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82203CD8: B14B00F0  sth r10, 0xf0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[10].u16 ) };
	// 82203CDC: B14B00F2  sth r10, 0xf2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(242 as u32), ctx.r[10].u16 ) };
	// 82203CE0: 914B00EC  stw r10, 0xec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 82203CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203CE8 size=32
    let mut pc: u32 = 0x82203CE8;
    'dispatch: loop {
        match pc {
            0x82203CE8 => {
    //   block [0x82203CE8..0x82203D08)
	// 82203CE8: 3D408287  lis r10, -0x7d79
	ctx.r[10].s64 = -2105081856;
	// 82203CEC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82203CF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82203CF4: 394A7790  addi r10, r10, 0x7790
	ctx.r[10].s64 = ctx.r[10].s64 + 30608;
	// 82203CF8: B12B00F0  sth r9, 0xf0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[9].u16 ) };
	// 82203CFC: B10B00F2  sth r8, 0xf2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(242 as u32), ctx.r[8].u16 ) };
	// 82203D00: 914B00EC  stw r10, 0xec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 82203D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82203D08 size=100
    let mut pc: u32 = 0x82203D08;
    'dispatch: loop {
        match pc {
            0x82203D08 => {
    //   block [0x82203D08..0x82203D6C)
	// 82203D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82203D0C: 483313A9  bl 0x825350b4
	ctx.lr = 0x82203D10;
	sub_82535080(ctx, base);
	// 82203D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82203D14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82203D18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82203D1C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82203D20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203D24: 839F0088  lwz r28, 0x88(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82203D28: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82203D2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82203D30: 4E800421  bctrl
	ctx.lr = 0x82203D34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82203D34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203D38: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82203D3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82203D40: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82203D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82203D48: 4E800421  bctrl
	ctx.lr = 0x82203D4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82203D4C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82203D50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82203D54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82203D58: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82203D5C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82203D60: 48005FB9  bl 0x82209d18
	ctx.lr = 0x82203D64;
	sub_82209D18(ctx, base);
	// 82203D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82203D68: 4833139C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203D70 size=12
    let mut pc: u32 = 0x82203D70;
    'dispatch: loop {
        match pc {
            0x82203D70 => {
    //   block [0x82203D70..0x82203D7C)
	// 82203D70: 81630100  lwz r11, 0x100(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82203D74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203D78: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203D7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203D7C size=8
    let mut pc: u32 = 0x82203D7C;
    'dispatch: loop {
        match pc {
            0x82203D7C => {
    //   block [0x82203D7C..0x82203D84)
	// 82203D7C: 48000684  b 0x82204400
	sub_82204400(ctx, base);
	return;
	// 82203D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82203D88 size=252
    let mut pc: u32 = 0x82203D88;
    'dispatch: loop {
        match pc {
            0x82203D88 => {
    //   block [0x82203D88..0x82203E84)
	// 82203D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82203D8C: 48331329  bl 0x825350b4
	ctx.lr = 0x82203D90;
	sub_82535080(ctx, base);
	// 82203D90: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82203D94: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82203D98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82203D9C: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 82203DA0: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
	// 82203DA4: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82203DA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82203DAC: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82203DB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82203DB4: A09F0004  lhz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82203DB8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82203DBC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82203DC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82203DC4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203DC8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82203DCC: A0BF0006  lhz r5, 6(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82203DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82203DD4: 48167DBD  bl 0x8236bb90
	ctx.lr = 0x82203DD8;
	sub_8236BB90(ctx, base);
	// 82203DD8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203DDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82203DE0: 839E0088  lwz r28, 0x88(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82203DE4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82203DE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82203DEC: 4E800421  bctrl
	ctx.lr = 0x82203DF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82203DF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203DF4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82203DF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82203DFC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82203E00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82203E04: 4E800421  bctrl
	ctx.lr = 0x82203E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82203E08: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82203E0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82203E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82203E14: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82203E18: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82203E1C: 48005EFD  bl 0x82209d18
	ctx.lr = 0x82203E20;
	sub_82209D18(ctx, base);
	// 82203E20: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82203E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82203E28: A0BF0006  lhz r5, 6(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82203E2C: A09F0004  lhz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82203E30: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203E34: 48167F35  bl 0x8236bd68
	ctx.lr = 0x82203E38;
	sub_8236BD68(ctx, base);
	// 82203E38: 3D4082B5  lis r10, -0x7d4b
	ctx.r[10].s64 = -2102067200;
	// 82203E3C: 3FA082C0  lis r29, -0x7d40
	ctx.r[29].s64 = -2101346304;
	// 82203E40: 3BCA0C40  addi r30, r10, 0xc40
	ctx.r[30].s64 = ctx.r[10].s64 + 3136;
	// 82203E44: 817DB9FC  lwz r11, -0x4604(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-17924 as u32) ) } as u64;
	// 82203E48: 815E0688  lwz r10, 0x688(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1672 as u32) ) } as u64;
	// 82203E4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203E50: 4098002C  bge cr6, 0x82203e7c
	if !ctx.cr[6].lt {
	pc = 0x82203E7C; continue 'dispatch;
	}
	// 82203E54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82203E58: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203E5C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82203E60: 3CC0C003  lis r6, -0x3ffd
	ctx.r[6].s64 = -1073545216;
	// 82203E64: 38A000FE  li r5, 0xfe
	ctx.r[5].s64 = 254;
	// 82203E68: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 82203E6C: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82203E70: 481680F1  bl 0x8236bf60
	ctx.lr = 0x82203E74;
	sub_8236BF60(ctx, base);
	// 82203E74: 817E0688  lwz r11, 0x688(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1672 as u32) ) } as u64;
	// 82203E78: 917DB9FC  stw r11, -0x4604(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-17924 as u32), ctx.r[11].u32 ) };
	// 82203E7C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82203E80: 48331284  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203E88 size=8
    let mut pc: u32 = 0x82203E88;
    'dispatch: loop {
        match pc {
            0x82203E88 => {
    //   block [0x82203E88..0x82203E90)
	// 82203E88: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82203E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82203E90 size=8
    let mut pc: u32 = 0x82203E90;
    'dispatch: loop {
        match pc {
            0x82203E90 => {
    //   block [0x82203E90..0x82203E98)
	// 82203E90: 3863001C  addi r3, r3, 0x1c
	ctx.r[3].s64 = ctx.r[3].s64 + 28;
	// 82203E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82203E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82203E98 size=516
    let mut pc: u32 = 0x82203E98;
    'dispatch: loop {
        match pc {
            0x82203E98 => {
    //   block [0x82203E98..0x8220409C)
	// 82203E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82203E9C: 48331221  bl 0x825350bc
	ctx.lr = 0x82203EA0;
	sub_82535080(ctx, base);
	// 82203EA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82203EA4: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82203EA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82203EAC: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82203EB0: 7FBEF82E  lwzx r29, r30, r31
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82203EB4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82203EB8: 419A0124  beq cr6, 0x82203fdc
	if ctx.cr[6].eq {
	pc = 0x82203FDC; continue 'dispatch;
	}
	// 82203EBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82203EC0: 48184339  bl 0x823881f8
	ctx.lr = 0x82203EC4;
	sub_823881F8(ctx, base);
	// 82203EC4: A17D0008  lhz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82203EC8: 556B077A  rlwinm r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82203ECC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203ED0: 419A0014  beq cr6, 0x82203ee4
	if ctx.cr[6].eq {
	pc = 0x82203EE4; continue 'dispatch;
	}
	// 82203ED4: A17D000C  lhz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82203ED8: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82203EDC: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 82203EE0: 419A002C  beq cr6, 0x82203f0c
	if ctx.cr[6].eq {
	pc = 0x82203F0C; continue 'dispatch;
	}
	// 82203EE4: 897F000E  lbz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 82203EE8: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82203EEC: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82203EF0: 419A0060  beq cr6, 0x82203f50
	if ctx.cr[6].eq {
	pc = 0x82203F50; continue 'dispatch;
	}
	// 82203EF4: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82203EF8: 419A0058  beq cr6, 0x82203f50
	if ctx.cr[6].eq {
	pc = 0x82203F50; continue 'dispatch;
	}
	// 82203EFC: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82203F00: 419A0050  beq cr6, 0x82203f50
	if ctx.cr[6].eq {
	pc = 0x82203F50; continue 'dispatch;
	}
	// 82203F04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82203F08: 409A0034  bne cr6, 0x82203f3c
	if !ctx.cr[6].eq {
	pc = 0x82203F3C; continue 'dispatch;
	}
	// 82203F0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82203F10: 48184351  bl 0x82388260
	ctx.lr = 0x82203F14;
	sub_82388260(ctx, base);
	// 82203F14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82203F18: 7D7EF92E  stwx r11, r30, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82203F1C: A17F0022  lhz r11, 0x22(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(34 as u32) ) } as u64;
	// 82203F20: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82203F24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82203F28: 419A00B4  beq cr6, 0x82203fdc
	if ctx.cr[6].eq {
	pc = 0x82203FDC; continue 'dispatch;
	}
	// 82203F2C: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82203F30: B17F0022  sth r11, 0x22(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(34 as u32), ctx.r[11].u16 ) };
	// 82203F34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82203F38: 483311D4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82203F3C: 3D600820  lis r11, 0x820
	ctx.r[11].s64 = 136314880;
	// 82203F40: 616B003F  ori r11, r11, 0x3f
	ctx.r[11].u64 = ctx.r[11].u64 | 63;
	// 82203F44: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82203F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82203F4C: 483311C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82203F50: 3D200820  lis r9, 0x820
	ctx.r[9].s64 = 136314880;
	// 82203F54: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82203F58: 3D400840  lis r10, 0x840
	ctx.r[10].s64 = 138412032;
	// 82203F5C: 6128003F  ori r8, r9, 0x3f
	ctx.r[8].u64 = ctx.r[9].u64 | 63;
	// 82203F60: 614A000E  ori r10, r10, 0xe
	ctx.r[10].u64 = ctx.r[10].u64 | 14;
	// 82203F64: 3D200820  lis r9, 0x820
	ctx.r[9].s64 = 136314880;
	// 82203F68: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203F6C: 61290040  ori r9, r9, 0x40
	ctx.r[9].u64 = ctx.r[9].u64 | 64;
	// 82203F70: 419900AC  bgt cr6, 0x8220401c
	if ctx.cr[6].gt {
	pc = 0x8220401C; continue 'dispatch;
	}
	// 82203F74: 419A00A0  beq cr6, 0x82204014
	if ctx.cr[6].eq {
	pc = 0x82204014; continue 'dispatch;
	}
	// 82203F78: 3D400830  lis r10, 0x830
	ctx.r[10].s64 = 137363456;
	// 82203F7C: 614A0014  ori r10, r10, 0x14
	ctx.r[10].u64 = ctx.r[10].u64 | 20;
	// 82203F80: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203F84: 41990060  bgt cr6, 0x82203fe4
	if ctx.cr[6].gt {
	pc = 0x82203FE4; continue 'dispatch;
	}
	// 82203F88: 3D400830  lis r10, 0x830
	ctx.r[10].s64 = 137363456;
	// 82203F8C: 614A000F  ori r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u64 | 15;
	// 82203F90: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203F94: 40980080  bge cr6, 0x82204014
	if !ctx.cr[6].lt {
	pc = 0x82204014; continue 'dispatch;
	}
	// 82203F98: 3D400820  lis r10, 0x820
	ctx.r[10].s64 = 136314880;
	// 82203F9C: 614A0037  ori r10, r10, 0x37
	ctx.r[10].u64 = ctx.r[10].u64 | 55;
	// 82203FA0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203FA4: 4198001C  blt cr6, 0x82203fc0
	if ctx.cr[6].lt {
	pc = 0x82203FC0; continue 'dispatch;
	}
	// 82203FA8: 3D400820  lis r10, 0x820
	ctx.r[10].s64 = 136314880;
	// 82203FAC: 614A0038  ori r10, r10, 0x38
	ctx.r[10].u64 = ctx.r[10].u64 | 56;
	// 82203FB0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203FB4: 409900E0  ble cr6, 0x82204094
	if !ctx.cr[6].gt {
	pc = 0x82204094; continue 'dispatch;
	}
	// 82203FB8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82203FBC: 419A0058  beq cr6, 0x82204014
	if ctx.cr[6].eq {
	pc = 0x82204014; continue 'dispatch;
	}
	// 82203FC0: 3D600820  lis r11, 0x820
	ctx.r[11].s64 = 136314880;
	// 82203FC4: 616B003E  ori r11, r11, 0x3e
	ctx.r[11].u64 = ctx.r[11].u64 | 62;
	// 82203FC8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82203FCC: A17D000A  lhz r11, 0xa(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(10 as u32) ) } as u64;
	// 82203FD0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82203FD4: 40980008  bge cr6, 0x82203fdc
	if !ctx.cr[6].lt {
	pc = 0x82203FDC; continue 'dispatch;
	}
	// 82203FD8: 911D0000  stw r8, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82203FDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82203FE0: 4833112C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82203FE4: 3D400840  lis r10, 0x840
	ctx.r[10].s64 = 138412032;
	// 82203FE8: 614A0008  ori r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 8;
	// 82203FEC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82203FF0: 419A0024  beq cr6, 0x82204014
	if ctx.cr[6].eq {
	pc = 0x82204014; continue 'dispatch;
	}
	// 82203FF4: 3D400840  lis r10, 0x840
	ctx.r[10].s64 = 138412032;
	// 82203FF8: 614A000A  ori r10, r10, 0xa
	ctx.r[10].u64 = ctx.r[10].u64 | 10;
	// 82203FFC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82204000: 4099FFC0  ble cr6, 0x82203fc0
	if !ctx.cr[6].gt {
	pc = 0x82203FC0; continue 'dispatch;
	}
	// 82204004: 3D400840  lis r10, 0x840
	ctx.r[10].s64 = 138412032;
	// 82204008: 614A000C  ori r10, r10, 0xc
	ctx.r[10].u64 = ctx.r[10].u64 | 12;
	// 8220400C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82204010: 4199FFB0  bgt cr6, 0x82203fc0
	if ctx.cr[6].gt {
	pc = 0x82203FC0; continue 'dispatch;
	}
	// 82204014: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82204018: 4BFFFFB4  b 0x82203fcc
	pc = 0x82203FCC; continue 'dispatch;
	// 8220401C: 3D400850  lis r10, 0x850
	ctx.r[10].s64 = 139460608;
	// 82204020: 614A001B  ori r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u64 | 27;
	// 82204024: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82204028: 4199003C  bgt cr6, 0x82204064
	if ctx.cr[6].gt {
	pc = 0x82204064; continue 'dispatch;
	}
	// 8220402C: 419A0068  beq cr6, 0x82204094
	if ctx.cr[6].eq {
	pc = 0x82204094; continue 'dispatch;
	}
	// 82204030: 3D400840  lis r10, 0x840
	ctx.r[10].s64 = 138412032;
	// 82204034: 614A0021  ori r10, r10, 0x21
	ctx.r[10].u64 = ctx.r[10].u64 | 33;
	// 82204038: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8220403C: 4198FF84  blt cr6, 0x82203fc0
	if ctx.cr[6].lt {
	pc = 0x82203FC0; continue 'dispatch;
	}
	// 82204040: 3D400840  lis r10, 0x840
	ctx.r[10].s64 = 138412032;
	// 82204044: 614A0023  ori r10, r10, 0x23
	ctx.r[10].u64 = ctx.r[10].u64 | 35;
	// 82204048: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8220404C: 4099FFC8  ble cr6, 0x82204014
	if !ctx.cr[6].gt {
	pc = 0x82204014; continue 'dispatch;
	}
	// 82204050: 3D400850  lis r10, 0x850
	ctx.r[10].s64 = 139460608;
	// 82204054: 614A000E  ori r10, r10, 0xe
	ctx.r[10].u64 = ctx.r[10].u64 | 14;
	// 82204058: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8220405C: 419A0038  beq cr6, 0x82204094
	if ctx.cr[6].eq {
	pc = 0x82204094; continue 'dispatch;
	}
	// 82204060: 4BFFFF60  b 0x82203fc0
	pc = 0x82203FC0; continue 'dispatch;
	// 82204064: 3D400850  lis r10, 0x850
	ctx.r[10].s64 = 139460608;
	// 82204068: 614A001D  ori r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u64 | 29;
	// 8220406C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82204070: 4198FF50  blt cr6, 0x82203fc0
	if ctx.cr[6].lt {
	pc = 0x82203FC0; continue 'dispatch;
	}
	// 82204074: 3D400850  lis r10, 0x850
	ctx.r[10].s64 = 139460608;
	// 82204078: 614A001E  ori r10, r10, 0x1e
	ctx.r[10].u64 = ctx.r[10].u64 | 30;
	// 8220407C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82204080: 40990014  ble cr6, 0x82204094
	if !ctx.cr[6].gt {
	pc = 0x82204094; continue 'dispatch;
	}
	// 82204084: 3D400850  lis r10, 0x850
	ctx.r[10].s64 = 139460608;
	// 82204088: 614A0021  ori r10, r10, 0x21
	ctx.r[10].u64 = ctx.r[10].u64 | 33;
	// 8220408C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82204090: 409AFF30  bne cr6, 0x82203fc0
	if !ctx.cr[6].eq {
	pc = 0x82203FC0; continue 'dispatch;
	}
	// 82204094: 911D0000  stw r8, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82204098: 4BFFFF34  b 0x82203fcc
	pc = 0x82203FCC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822040A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822040A0 size=164
    let mut pc: u32 = 0x822040A0;
    'dispatch: loop {
        match pc {
            0x822040A0 => {
    //   block [0x822040A0..0x82204144)
	// 822040A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822040A4: 48331019  bl 0x825350bc
	ctx.lr = 0x822040A8;
	sub_82535080(ctx, base);
	// 822040A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822040AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822040B0: 2B1D0004  cmplwi cr6, r29, 4
	ctx.cr[6].compare_u32(ctx.r[29].u32, 4 as u32, &mut ctx.xer);
	// 822040B4: 41980010  blt cr6, 0x822040c4
	if ctx.cr[6].lt {
	pc = 0x822040C4; continue 'dispatch;
	}
	// 822040B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822040BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822040C0: 4833104C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 822040C4: 481778A5  bl 0x8237b968
	ctx.lr = 0x822040C8;
	sub_8237B968(ctx, base);
	// 822040C8: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 822040CC: 2B230000  cmpldi cr6, r3, 0
	ctx.cr[6].compare_u64(ctx.r[3].u64, 0, &mut ctx.xer);
	// 822040D0: 419AFFE8  beq cr6, 0x822040b8
	if ctx.cr[6].eq {
	pc = 0x822040B8; continue 'dispatch;
	}
	// 822040D4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822040D8: 2B0B01FF  cmplwi cr6, r11, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 511 as u32, &mut ctx.xer);
	// 822040DC: 4198000C  blt cr6, 0x822040e8
	if ctx.cr[6].lt {
	pc = 0x822040E8; continue 'dispatch;
	}
	// 822040E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822040E4: 4800001C  b 0x82204100
	pc = 0x82204100; continue 'dispatch;
	// 822040E8: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822040EC: 3D408310  lis r10, -0x7cf0
	ctx.r[10].s64 = -2096103424;
	// 822040F0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 822040F4: 394A86B8  addi r10, r10, -0x7948
	ctx.r[10].s64 = ctx.r[10].s64 + -31048;
	// 822040F8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822040FC: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82204100: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82204104: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82204108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8220410C: 4099002C  ble cr6, 0x82204138
	if !ctx.cr[6].gt {
	pc = 0x82204138; continue 'dispatch;
	}
	// 82204110: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82204114: 3D408220  lis r10, -0x7de0
	ctx.r[10].s64 = -2111832064;
	// 82204118: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8220411C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82204120: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82204124: 481981C5  bl 0x8239c2e8
	ctx.lr = 0x82204128;
	sub_8239C2E8(ctx, base);
	// 82204128: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220412C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82204130: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82204134: 4198FFDC  blt cr6, 0x82204110
	if ctx.cr[6].lt {
	pc = 0x82204110; continue 'dispatch;
	}
	// 82204138: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8220413C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82204140: 48330FCC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82204148 size=296
    let mut pc: u32 = 0x82204148;
    'dispatch: loop {
        match pc {
            0x82204148 => {
    //   block [0x82204148..0x82204270)
	// 82204148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220414C: 48330F6D  bl 0x825350b8
	ctx.lr = 0x82204150;
	sub_82535080(ctx, base);
	// 82204150: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82204154: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204158: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220415C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82204160: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204164: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82204168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8220416C: 4E800421  bctrl
	ctx.lr = 0x82204170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204170: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204174: 419A00F0  beq cr6, 0x82204264
	if ctx.cr[6].eq {
	pc = 0x82204264; continue 'dispatch;
	}
	// 82204178: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8220417C: 816BBFA0  lwz r11, -0x4060(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16480 as u32) ) } as u64;
	// 82204180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204184: 419A00E0  beq cr6, 0x82204264
	if ctx.cr[6].eq {
	pc = 0x82204264; continue 'dispatch;
	}
	// 82204188: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8220418C: C1BF008C  lfs f13, 0x8c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82204190: 3D4082B6  lis r10, -0x7d4a
	ctx.r[10].s64 = -2102001664;
	// 82204194: 396B3100  addi r11, r11, 0x3100
	ctx.r[11].s64 = ctx.r[11].s64 + 12544;
	// 82204198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8220419C: C00B005C  lfs f0, 0x5c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822041A0: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 822041A4: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 822041A8: 3BCBB5B4  addi r30, r11, -0x4a4c
	ctx.r[30].s64 = ctx.r[11].s64 + -19020;
	// 822041AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822041B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822041B4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822041B8: 916AB65C  stw r11, -0x49a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18852 as u32), ctx.r[11].u32 ) };
	// 822041BC: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 822041C0: 396BBFF0  addi r11, r11, -0x4010
	ctx.r[11].s64 = ctx.r[11].s64 + -16400;
	// 822041C4: 81490020  lwz r10, 0x20(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 822041C8: 3D200003  lis r9, 3
	ctx.r[9].s64 = 196608;
	// 822041CC: 612985D4  ori r9, r9, 0x85d4
	ctx.r[9].u64 = ctx.r[9].u64 | 34260;
	// 822041D0: 7C0B4C2E  lfsx f0, r11, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822041D4: EFED0032  fmuls f31, f13, f0
	ctx.f[31].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 822041D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 822041DC: 4E800421  bctrl
	ctx.lr = 0x822041E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822041E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822041E4: 419A0054  beq cr6, 0x82204238
	if ctx.cr[6].eq {
	pc = 0x82204238; continue 'dispatch;
	}
	// 822041E8: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 822041EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822041F0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822041F4: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822041F8: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 822041FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204200: 4E800421  bctrl
	ctx.lr = 0x82204204;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204204: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204208: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8220420C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82204210: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82204214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204218: 4E800421  bctrl
	ctx.lr = 0x8220421C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8220421C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82204220: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82204224: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82204228: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8220422C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82204230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204234: 4E800421  bctrl
	ctx.lr = 0x82204238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204238: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8220423C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82204240: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204244: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82204248: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8220424C: 4E800421  bctrl
	ctx.lr = 0x82204250;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82204254: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82204258: 48184C89  bl 0x82388ee0
	ctx.lr = 0x8220425C;
	sub_82388EE0(ctx, base);
	// 8220425C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82204260: 48184DB9  bl 0x82389018
	ctx.lr = 0x82204264;
	sub_82389018(ctx, base);
	// 82204264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82204268: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8220426C: 48330E9C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204270 size=272
    let mut pc: u32 = 0x82204270;
    'dispatch: loop {
        match pc {
            0x82204270 => {
    //   block [0x82204270..0x82204380)
	// 82204270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204274: 48330E49  bl 0x825350bc
	ctx.lr = 0x82204278;
	sub_82535080(ctx, base);
	// 82204278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220427C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82204280: 807E0080  lwz r3, 0x80(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82204284: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204288: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220428C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204290: 4E800421  bctrl
	ctx.lr = 0x82204294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204294: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204298: 409A000C  bne cr6, 0x822042a4
	if !ctx.cr[6].eq {
	pc = 0x822042A4; continue 'dispatch;
	}
	// 8220429C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822042A0: 48330E6C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 822042A4: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 822042A8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822042AC: 419A0024  beq cr6, 0x822042d0
	if ctx.cr[6].eq {
	pc = 0x822042D0; continue 'dispatch;
	}
	// 822042B0: A17E0014  lhz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 822042B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822042B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822042BC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 822042C0: 514B5D2A  rlwimi r11, r10, 0xb, 0x14, 0x15
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(11) as u64) & 0x0000000000000C00) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF3FF);
	// 822042C4: B17E0014  sth r11, 0x14(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u16 ) };
	// 822042C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822042CC: 48330E40  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 822042D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822042D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822042D8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 822042DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822042E0: 4E800421  bctrl
	ctx.lr = 0x822042E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822042E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822042E8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822042EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822042F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822042F4: 4E800421  bctrl
	ctx.lr = 0x822042F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822042F8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822042FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82204300: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82204304: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82204308: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8220430C: 4E800421  bctrl
	ctx.lr = 0x82204310;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204310: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82204314: 4098004C  bge cr6, 0x82204360
	if !ctx.cr[6].lt {
	pc = 0x82204360; continue 'dispatch;
	}
	// 82204318: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220431C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82204320: 419A0020  beq cr6, 0x82204340
	if ctx.cr[6].eq {
	pc = 0x82204340; continue 'dispatch;
	}
	// 82204324: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204328: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8220432C: 80BE0080  lwz r5, 0x80(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82204330: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82204334: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82204338: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8220433C: 4E800421  bctrl
	ctx.lr = 0x82204340;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204340: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204344: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82204348: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8220434C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82204350: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204354: 4E800421  bctrl
	ctx.lr = 0x82204358;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204358: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8220435C: 4198FFBC  blt cr6, 0x82204318
	if ctx.cr[6].lt {
	pc = 0x82204318; continue 'dispatch;
	}
	// 82204360: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82204368: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220436C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204370: 4E800421  bctrl
	ctx.lr = 0x82204374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204374: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82204378: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8220437C: 48330D90  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204380 size=124
    let mut pc: u32 = 0x82204380;
    'dispatch: loop {
        match pc {
            0x82204380 => {
    //   block [0x82204380..0x822043FC)
	// 82204380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204388: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8220438C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204390: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82204394: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82204398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8220439C: 419A0048  beq cr6, 0x822043e4
	if ctx.cr[6].eq {
	pc = 0x822043E4; continue 'dispatch;
	}
	// 822043A0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 822043A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822043A8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822043AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822043B0: 4E800421  bctrl
	ctx.lr = 0x822043B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822043B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822043B8: 419A002C  beq cr6, 0x822043e4
	if ctx.cr[6].eq {
	pc = 0x822043E4; continue 'dispatch;
	}
	// 822043BC: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 822043C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822043C4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822043C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822043CC: 4E800421  bctrl
	ctx.lr = 0x822043D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822043D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822043D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822043D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822043DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822043E0: 4E800020  blr
	return;
	// 822043E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822043E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822043EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822043F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822043F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822043F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204400 size=116
    let mut pc: u32 = 0x82204400;
    'dispatch: loop {
        match pc {
            0x82204400 => {
    //   block [0x82204400..0x82204474)
	// 82204400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204404: 48330CB5  bl 0x825350b8
	ctx.lr = 0x82204408;
	sub_82535080(ctx, base);
	// 82204408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220440C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82204410: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82204414: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204418: 419A0054  beq cr6, 0x8220446c
	if ctx.cr[6].eq {
	pc = 0x8220446C; continue 'dispatch;
	}
	// 8220441C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82204420: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204424: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204428: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8220442C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204430: 4E800421  bctrl
	ctx.lr = 0x82204434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204434: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204438: 547D3032  slwi r29, r3, 6
	ctx.r[29].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8220443C: 839F0088  lwz r28, 0x88(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82204440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82204444: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82204448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8220444C: 4E800421  bctrl
	ctx.lr = 0x82204450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204450: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82204454: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82204458: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8220445C: 815E0058  lwz r10, 0x58(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82204460: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82204464: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82204468: 4E800421  bctrl
	ctx.lr = 0x8220446C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8220446C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82204470: 48330C98  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204478 size=4
    let mut pc: u32 = 0x82204478;
    'dispatch: loop {
        match pc {
            0x82204478 => {
    //   block [0x82204478..0x8220447C)
	// 82204478: 4BFFFDF8  b 0x82204270
	sub_82204270(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204480 size=4
    let mut pc: u32 = 0x82204480;
    'dispatch: loop {
        match pc {
            0x82204480 => {
    //   block [0x82204480..0x82204484)
	// 82204480: 4BFFFF00  b 0x82204380
	sub_82204380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204488 size=72
    let mut pc: u32 = 0x82204488;
    'dispatch: loop {
        match pc {
            0x82204488 => {
    //   block [0x82204488..0x822044D0)
	// 82204488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82204494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204498: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220449C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822044A0: 396B3BF8  addi r11, r11, 0x3bf8
	ctx.r[11].s64 = ctx.r[11].s64 + 15352;
	// 822044A4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 822044A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822044AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822044B0: 419A000C  beq cr6, 0x822044bc
	if ctx.cr[6].eq {
	pc = 0x822044BC; continue 'dispatch;
	}
	// 822044B4: 4832E705  bl 0x82532bb8
	ctx.lr = 0x822044B8;
	sub_82532BB8(ctx, base);
	// 822044B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822044BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822044C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822044C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822044C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822044CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822044D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822044D0 size=112
    let mut pc: u32 = 0x822044D0;
    'dispatch: loop {
        match pc {
            0x822044D0 => {
    //   block [0x822044D0..0x82204540)
	// 822044D0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822044D4: 396B3C24  addi r11, r11, 0x3c24
	ctx.r[11].s64 = ctx.r[11].s64 + 15396;
	// 822044D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822044DC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 822044E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822044E4: 419A004C  beq cr6, 0x82204530
	if ctx.cr[6].eq {
	pc = 0x82204530; continue 'dispatch;
	}
	// 822044E8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822044EC: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 822044F0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822044F4: 409A003C  bne cr6, 0x82204530
	if !ctx.cr[6].eq {
	pc = 0x82204530; continue 'dispatch;
	}
	// 822044F8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 822044FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204500: 419A0030  beq cr6, 0x82204530
	if ctx.cr[6].eq {
	pc = 0x82204530; continue 'dispatch;
	}
	// 82204504: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82204508: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220450C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82204510: 409A0020  bne cr6, 0x82204530
	if !ctx.cr[6].eq {
	pc = 0x82204530; continue 'dispatch;
	}
	// 82204514: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82204518: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220451C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82204520: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82204524: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82204528: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 8220452C: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82204530: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82204534: 396B3BF8  addi r11, r11, 0x3bf8
	ctx.r[11].s64 = ctx.r[11].s64 + 15352;
	// 82204538: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8220453C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204540 size=8
    let mut pc: u32 = 0x82204540;
    'dispatch: loop {
        match pc {
            0x82204540 => {
    //   block [0x82204540..0x82204548)
	// 82204540: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82204544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204548 size=20
    let mut pc: u32 = 0x82204548;
    'dispatch: loop {
        match pc {
            0x82204548 => {
    //   block [0x82204548..0x8220455C)
	// 82204548: 548B07FE  clrlwi r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8220454C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82204550: 419A000C  beq cr6, 0x8220455c
	if ctx.cr[6].eq {
		sub_8220455C(ctx, base);
		return;
	}
	// 82204554: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82204558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220455C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220455C size=16
    let mut pc: u32 = 0x8220455C;
    'dispatch: loop {
        match pc {
            0x8220455C => {
    //   block [0x8220455C..0x8220456C)
	// 8220455C: 548B077A  rlwinm r11, r4, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82204560: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82204564: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82204568: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220456C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220456C size=8
    let mut pc: u32 = 0x8220456C;
    'dispatch: loop {
        match pc {
            0x8220456C => {
    //   block [0x8220456C..0x82204574)
	// 8220456C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82204570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204578 size=8
    let mut pc: u32 = 0x82204578;
    'dispatch: loop {
        match pc {
            0x82204578 => {
    //   block [0x82204578..0x82204580)
	// 82204578: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8220457C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204580 size=16
    let mut pc: u32 = 0x82204580;
    'dispatch: loop {
        match pc {
            0x82204580 => {
    //   block [0x82204580..0x82204590)
	// 82204580: 548B07FE  clrlwi r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82204584: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 82204588: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8220458C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204590 size=8
    let mut pc: u32 = 0x82204590;
    'dispatch: loop {
        match pc {
            0x82204590 => {
    //   block [0x82204590..0x82204598)
	// 82204590: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82204594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204598 size=20
    let mut pc: u32 = 0x82204598;
    'dispatch: loop {
        match pc {
            0x82204598 => {
    //   block [0x82204598..0x822045AC)
	// 82204598: 708B0005  andi. r11, r4, 5
	ctx.r[11].u64 = ctx.r[4].u64 & 5;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8220459C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822045A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822045A4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 822045A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822045B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822045B0 size=12
    let mut pc: u32 = 0x822045B0;
    'dispatch: loop {
        match pc {
            0x822045B0 => {
    //   block [0x822045B0..0x822045BC)
	// 822045B0: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 822045B4: 3863002C  addi r3, r3, 0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + 44;
	// 822045B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822045BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822045BC size=8
    let mut pc: u32 = 0x822045BC;
    'dispatch: loop {
        match pc {
            0x822045BC => {
    //   block [0x822045BC..0x822045C4)
	// 822045BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822045C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822045C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822045C8 size=16
    let mut pc: u32 = 0x822045C8;
    'dispatch: loop {
        match pc {
            0x822045C8 => {
    //   block [0x822045C8..0x822045D8)
	// 822045C8: 548B07FE  clrlwi r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 822045CC: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 822045D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822045D4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822045D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822045D8 size=8
    let mut pc: u32 = 0x822045D8;
    'dispatch: loop {
        match pc {
            0x822045D8 => {
    //   block [0x822045D8..0x822045E0)
	// 822045D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822045DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822045E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822045E0 size=12
    let mut pc: u32 = 0x822045E0;
    'dispatch: loop {
        match pc {
            0x822045E0 => {
    //   block [0x822045E0..0x822045EC)
	// 822045E0: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 822045E4: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 822045E8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822045EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822045EC size=8
    let mut pc: u32 = 0x822045EC;
    'dispatch: loop {
        match pc {
            0x822045EC => {
    //   block [0x822045EC..0x822045F4)
	// 822045EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822045F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822045F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822045F8 size=16
    let mut pc: u32 = 0x822045F8;
    'dispatch: loop {
        match pc {
            0x822045F8 => {
    //   block [0x822045F8..0x82204608)
	// 822045F8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 822045FC: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82204600: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82204604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204608 size=16
    let mut pc: u32 = 0x82204608;
    'dispatch: loop {
        match pc {
            0x82204608 => {
    //   block [0x82204608..0x82204618)
	// 82204608: 548B07FE  clrlwi r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8220460C: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82204610: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82204614: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204618 size=8
    let mut pc: u32 = 0x82204618;
    'dispatch: loop {
        match pc {
            0x82204618 => {
    //   block [0x82204618..0x82204620)
	// 82204618: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8220461C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204620 size=48
    let mut pc: u32 = 0x82204620;
    'dispatch: loop {
        match pc {
            0x82204620 => {
    //   block [0x82204620..0x82204650)
	// 82204620: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82204624: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82204628: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220462C: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 82204630: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82204634: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82204638: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8220463C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82204640: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82204644: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82204648: 7D4B412E  stwx r10, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 8220464C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204650 size=8
    let mut pc: u32 = 0x82204650;
    'dispatch: loop {
        match pc {
            0x82204650 => {
    //   block [0x82204650..0x82204658)
	// 82204650: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82204654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204658 size=20
    let mut pc: u32 = 0x82204658;
    'dispatch: loop {
        match pc {
            0x82204658 => {
    //   block [0x82204658..0x8220466C)
	// 82204658: 548B07FE  clrlwi r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8220465C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82204660: 419A000C  beq cr6, 0x8220466c
	if ctx.cr[6].eq {
		sub_8220466C(ctx, base);
		return;
	}
	// 82204664: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82204668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220466C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220466C size=16
    let mut pc: u32 = 0x8220466C;
    'dispatch: loop {
        match pc {
            0x8220466C => {
    //   block [0x8220466C..0x8220467C)
	// 8220466C: 548B077A  rlwinm r11, r4, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82204670: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82204674: 419A0008  beq cr6, 0x8220467c
	if ctx.cr[6].eq {
		sub_8220467C(ctx, base);
		return;
	}
	// 82204678: 48184EA0  b 0x82389518
	sub_82389518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220467C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220467C size=8
    let mut pc: u32 = 0x8220467C;
    'dispatch: loop {
        match pc {
            0x8220467C => {
    //   block [0x8220467C..0x82204684)
	// 8220467C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82204680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82204688 size=64
    let mut pc: u32 = 0x82204688;
    'dispatch: loop {
        match pc {
            0x82204688 => {
    //   block [0x82204688..0x822046C8)
	// 82204688: 3D208286  lis r9, -0x7d7a
	ctx.r[9].s64 = -2105147392;
	// 8220468C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82204690: 3900002A  li r8, 0x2a
	ctx.r[8].s64 = 42;
	// 82204694: 3960002A  li r11, 0x2a
	ctx.r[11].s64 = 42;
	// 82204698: 5467043E  clrlwi r7, r3, 0x10
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8220469C: 38C9F390  addi r6, r9, -0xc70
	ctx.r[6].s64 = ctx.r[9].s64 + -3184;
	// 822046A0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 822046A4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 822046A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822046AC: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822046B0: 7D29322E  lhzx r9, r9, r6
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 822046B4: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 822046B8: 419A0028  beq cr6, 0x822046e0
	if ctx.cr[6].eq {
		sub_822046E0(ctx, base);
		return;
	}
	// 822046BC: 4099000C  ble cr6, 0x822046c8
	if !ctx.cr[6].gt {
		sub_822046C8(ctx, base);
		return;
	}
	// 822046C0: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 822046C4: 48000008  b 0x822046cc
	sub_822046C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822046C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822046C8 size=24
    let mut pc: u32 = 0x822046C8;
    'dispatch: loop {
        match pc {
            0x822046C8 => {
    //   block [0x822046C8..0x822046E0)
	// 822046C8: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 822046CC: 7D6A4050  subf r11, r10, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 822046D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822046D4: 4098FFCC  bge cr6, 0x822046a0
	if !ctx.cr[6].lt {
		sub_82204688(ctx, base);
		return;
	}
	// 822046D8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 822046DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822046E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822046E0 size=8
    let mut pc: u32 = 0x822046E0;
    'dispatch: loop {
        match pc {
            0x822046E0 => {
    //   block [0x822046E0..0x822046E8)
	// 822046E0: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 822046E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822046E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822046E8 size=408
    let mut pc: u32 = 0x822046E8;
    'dispatch: loop {
        match pc {
            0x822046E8 => {
    //   block [0x822046E8..0x82204880)
	// 822046E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822046EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822046F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822046F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822046F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822046FC: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82204700: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82204704: 396BBA00  addi r11, r11, -0x4600
	ctx.r[11].s64 = ctx.r[11].s64 + -17920;
	// 82204708: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8220470C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82204710: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82204714: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82204718: 419A0028  beq cr6, 0x82204740
	if ctx.cr[6].eq {
	pc = 0x82204740; continue 'dispatch;
	}
	// 8220471C: A1050000  lhz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204720: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204724: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204728: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8220472C: 419A0050  beq cr6, 0x8220477c
	if ctx.cr[6].eq {
	pc = 0x8220477C; continue 'dispatch;
	}
	// 82204730: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82204734: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82204738: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8220473C: 4198FFE8  blt cr6, 0x82204724
	if ctx.cr[6].lt {
	pc = 0x82204724; continue 'dispatch;
	}
	// 82204740: A0650000  lhz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204744: 4BFFFF45  bl 0x82204688
	ctx.lr = 0x82204748;
	sub_82204688(ctx, base);
	// 82204748: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8220474C: 2B0A002A  cmplwi cr6, r10, 0x2a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 42 as u32, &mut ctx.xer);
	// 82204750: 40980114  bge cr6, 0x82204864
	if !ctx.cr[6].lt {
	pc = 0x82204864; continue 'dispatch;
	}
	// 82204754: 3D608286  lis r11, -0x7d7a
	ctx.r[11].s64 = -2105147392;
	// 82204758: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8220475C: 396BF390  addi r11, r11, -0xc70
	ctx.r[11].s64 = ctx.r[11].s64 + -3184;
	// 82204760: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82204764: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82204768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8220476C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82204770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204774: 4E800421  bctrl
	ctx.lr = 0x82204778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82204778: 480000F0  b 0x82204868
	pc = 0x82204868; continue 'dispatch;
	// 8220477C: A1650008  lhz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82204780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204784: 419A00B4  beq cr6, 0x82204838
	if ctx.cr[6].eq {
	pc = 0x82204838; continue 'dispatch;
	}
	// 82204788: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8220478C: 419A00D8  beq cr6, 0x82204864
	if ctx.cr[6].eq {
	pc = 0x82204864; continue 'dispatch;
	}
	// 82204790: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82204794: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82204798: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8220479C: 396B4350  addi r11, r11, 0x4350
	ctx.r[11].s64 = ctx.r[11].s64 + 17232;
	// 822047A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822047A4: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822047A8: 395F0027  addi r10, r31, 0x27
	ctx.r[10].s64 = ctx.r[31].s64 + 39;
	// 822047AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822047B0: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 822047B4: A165000A  lhz r11, 0xa(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(10 as u32) ) } as u64;
	// 822047B8: 554A0036  rlwinm r10, r10, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 822047BC: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 822047C0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822047C4: A1650008  lhz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 822047C8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822047CC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822047D0: A1650008  lhz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 822047D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822047D8: 419A0058  beq cr6, 0x82204830
	if ctx.cr[6].eq {
	pc = 0x82204830; continue 'dispatch;
	}
	// 822047DC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 822047E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822047E4: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 822047E8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 822047EC: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 822047F0: 3D403F80  lis r10, 0x3f80
	ctx.r[10].s64 = 1065353216;
	// 822047F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822047F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 822047FC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82204800: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204880 size=244
    let mut pc: u32 = 0x82204880;
    'dispatch: loop {
        match pc {
            0x82204880 => {
    //   block [0x82204880..0x82204974)
	// 82204880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220488C: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82204890: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82204894: 396BBA00  addi r11, r11, -0x4600
	ctx.r[11].s64 = ctx.r[11].s64 + -17920;
	// 82204898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8220489C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822048A0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822048A4: 419A0028  beq cr6, 0x822048cc
	if ctx.cr[6].eq {
	pc = 0x822048CC; continue 'dispatch;
	}
	// 822048A8: A1050000  lhz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 822048AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822048B0: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822048B4: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822048B8: 419A0064  beq cr6, 0x8220491c
	if ctx.cr[6].eq {
	pc = 0x8220491C; continue 'dispatch;
	}
	// 822048BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822048C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822048C4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822048C8: 4198FFE8  blt cr6, 0x822048b0
	if ctx.cr[6].lt {
	pc = 0x822048B0; continue 'dispatch;
	}
	// 822048CC: A0650000  lhz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 822048D0: 4BFFFDB9  bl 0x82204688
	ctx.lr = 0x822048D4;
	sub_82204688(ctx, base);
	// 822048D4: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 822048D8: 2B0B002A  cmplwi cr6, r11, 0x2a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 42 as u32, &mut ctx.xer);
	// 822048DC: 40980084  bge cr6, 0x82204960
	if !ctx.cr[6].lt {
	pc = 0x82204960; continue 'dispatch;
	}
	// 822048E0: 3D408286  lis r10, -0x7d7a
	ctx.r[10].s64 = -2105147392;
	// 822048E4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822048E8: 394AF390  addi r10, r10, -0xc70
	ctx.r[10].s64 = ctx.r[10].s64 + -3184;
	// 822048EC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 822048F0: 392A000C  addi r9, r10, 0xc
	ctx.r[9].s64 = ctx.r[10].s64 + 12;
	// 822048F4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 822048F8: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822048FC: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82204900: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82204904: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82204908: 4E800421  bctrl
	ctx.lr = 0x8220490C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8220490C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82204910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82204914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82204918: 4E800020  blr
	return;
	// 8220491C: A1650008  lhz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82204920: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204924: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82204928: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8220492C: 419A0020  beq cr6, 0x8220494c
	if ctx.cr[6].eq {
	pc = 0x8220494C; continue 'dispatch;
	}
	// 82204930: A1650008  lhz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82204934: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 82204938: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8220493C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82204940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82204944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82204948: 4E800020  blr
	return;
	// 8220494C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82204950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82204954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82204958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220495C: 4E800020  blr
	return;
	// 82204960: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82204964: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82204968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8220496C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82204970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204978 size=12
    let mut pc: u32 = 0x82204978;
    'dispatch: loop {
        match pc {
            0x82204978 => {
    //   block [0x82204978..0x82204984)
	// 82204978: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220497C: A06B0000  lhz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82204980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82204988 size=208
    let mut pc: u32 = 0x82204988;
    'dispatch: loop {
        match pc {
            0x82204988 => {
    //   block [0x82204988..0x82204A58)
	// 82204988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82204994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220499C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822049A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822049A4: 396B3C50  addi r11, r11, 0x3c50
	ctx.r[11].s64 = ctx.r[11].s64 + 15440;
	// 822049A8: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822049AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822049B0: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822049B4: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822049B8: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822049BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822049C0: 48184AE1  bl 0x823894a0
	ctx.lr = 0x822049C4;
	sub_823894A0(ctx, base);
	// 822049C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822049C8: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 822049CC: 409A0010  bne cr6, 0x822049dc
	if !ctx.cr[6].eq {
	pc = 0x822049DC; continue 'dispatch;
	}
	// 822049D0: 3D608284  lis r11, -0x7d7c
	ctx.r[11].s64 = -2105278464;
	// 822049D4: 816BC8BC  lwz r11, -0x3744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14148 as u32) ) } as u64;
	// 822049D8: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 822049DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 822049E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822049E4: 48184ABD  bl 0x823894a0
	ctx.lr = 0x822049E8;
	sub_823894A0(ctx, base);
	// 822049E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822049EC: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 822049F0: 409A000C  bne cr6, 0x822049fc
	if !ctx.cr[6].eq {
	pc = 0x822049FC; continue 'dispatch;
	}
	// 822049F4: 48184B25  bl 0x82389518
	ctx.lr = 0x822049F8;
	sub_82389518(ctx, base);
	// 822049F8: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 822049FC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82204A00: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82204A04: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82204A08: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82204A0C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82204A10: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82204A14: C00ABA38  lfs f0, -0x45c8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82204A18: 3D403F80  lis r10, 0x3f80
	ctx.r[10].s64 = 1065353216;
	// 82204A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204A58 size=144
    let mut pc: u32 = 0x82204A58;
    'dispatch: loop {
        match pc {
            0x82204A58 => {
    //   block [0x82204A58..0x82204AE8)
	// 82204A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204A60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82204A64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204A68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82204A6C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82204A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82204A74: 396B3C7C  addi r11, r11, 0x3c7c
	ctx.r[11].s64 = ctx.r[11].s64 + 15484;
	// 82204A78: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82204A7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82204A80: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82204A84: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82204A88: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82204A8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82204A90: 48184A11  bl 0x823894a0
	ctx.lr = 0x82204A94;
	sub_823894A0(ctx, base);
	// 82204A94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204A98: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82204A9C: 409A0010  bne cr6, 0x82204aac
	if !ctx.cr[6].eq {
	pc = 0x82204AAC; continue 'dispatch;
	}
	// 82204AA0: 3D608284  lis r11, -0x7d7c
	ctx.r[11].s64 = -2105278464;
	// 82204AA4: 816BC8BC  lwz r11, -0x3744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14148 as u32) ) } as u64;
	// 82204AA8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82204AAC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82204AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82204AB4: 481849ED  bl 0x823894a0
	ctx.lr = 0x82204AB8;
	sub_823894A0(ctx, base);
	// 82204AB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204ABC: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82204AC0: 409A0010  bne cr6, 0x82204ad0
	if !ctx.cr[6].eq {
	pc = 0x82204AD0; continue 'dispatch;
	}
	// 82204AC4: 48184A55  bl 0x82389518
	ctx.lr = 0x82204AC8;
	sub_82389518(ctx, base);
	// 82204AC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82204ACC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82204AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82204AD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82204AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82204ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82204AE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82204AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82204AE8 size=184
    let mut pc: u32 = 0x82204AE8;
    'dispatch: loop {
        match pc {
            0x82204AE8 => {
    //   block [0x82204AE8..0x82204BA0)
	// 82204AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204AF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82204AF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204AF8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82204AFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82204B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82204B04: 396B3CA8  addi r11, r11, 0x3ca8
	ctx.r[11].s64 = ctx.r[11].s64 + 15528;
	// 82204B08: 393F003F  addi r9, r31, 0x3f
	ctx.r[9].s64 = ctx.r[31].s64 + 63;
	// 82204B0C: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82204B10: 55290036  rlwinm r9, r9, 0, 0, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82204B14: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82204B18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82204B1C: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82204B20: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82204B24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82204B28: A165000A  lhz r11, 0xa(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(10 as u32) ) } as u64;
	// 82204B2C: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82204B30: B17F0018  sth r11, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 82204B34: A1650008  lhz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82204B38: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82204B3C: B17F001A  sth r11, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[11].u16 ) };
	// 82204B40: 48184961  bl 0x823894a0
	ctx.lr = 0x82204B44;
	sub_823894A0(ctx, base);
	// 82204B44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204B48: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82204B4C: 409A0010  bne cr6, 0x82204b5c
	if !ctx.cr[6].eq {
	pc = 0x82204B5C; continue 'dispatch;
	}
	// 82204B50: 3D608284  lis r11, -0x7d7c
	ctx.r[11].s64 = -2105278464;
	// 82204B54: 816BC8BC  lwz r11, -0x3744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14148 as u32) ) } as u64;
	// 82204B58: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82204B5C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82204B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82204B64: 4818493D  bl 0x823894a0
	ctx.lr = 0x82204B68;
	sub_823894A0(ctx, base);
	// 82204B68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204B6C: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82204B70: 409A000C  bne cr6, 0x82204b7c
	if !ctx.cr[6].eq {
	pc = 0x82204B7C; continue 'dispatch;
	}
	// 82204B74: 481849A5  bl 0x82389518
	ctx.lr = 0x82204B78;
	sub_82389518(ctx, base);
	// 82204B78: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82204B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82204B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82204B84: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82204B88: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82204B8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82204B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82204B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82204B98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82204B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204BA0 size=16
    let mut pc: u32 = 0x82204BA0;
    'dispatch: loop {
        match pc {
            0x82204BA0 => {
    //   block [0x82204BA0..0x82204BB0)
	// 82204BA0: A163001A  lhz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204BA4: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 82204BA8: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82204BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82204BB0 size=48
    let mut pc: u32 = 0x82204BB0;
    'dispatch: loop {
        match pc {
            0x82204BB0 => {
    //   block [0x82204BB0..0x82204BE0)
	// 82204BB0: A1430018  lhz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82204BB4: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82204BB8: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82204BBC: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 82204BC0: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82204BC4: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82204BC8: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82204BCC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82204BD0: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82204BD4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82204BD8: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82204BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204BE0 size=16
    let mut pc: u32 = 0x82204BE0;
    'dispatch: loop {
        match pc {
            0x82204BE0 => {
    //   block [0x82204BE0..0x82204BF0)
	// 82204BE0: A163001A  lhz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204BE4: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 82204BE8: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82204BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204BF0 size=100
    let mut pc: u32 = 0x82204BF0;
    'dispatch: loop {
        match pc {
            0x82204BF0 => {
    //   block [0x82204BF0..0x82204C54)
	// 82204BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204BFC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82204C00: A167001A  lhz r11, 0x1a(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204C04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204C08: 419A003C  beq cr6, 0x82204c44
	if ctx.cr[6].eq {
	pc = 0x82204C44; continue 'dispatch;
	}
	// 82204C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82204C10: 80C70024  lwz r6, 0x24(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(36 as u32) ) } as u64;
	// 82204C14: 550B3032  slwi r11, r8, 6
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82204C18: 8127001C  lwz r9, 0x1c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 82204C1C: 81470020  lwz r10, 0x20(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) } as u64;
	// 82204C20: 7CA65A14  add r5, r6, r11
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82204C24: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82204C28: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82204C2C: 48163105  bl 0x82367d30
	ctx.lr = 0x82204C30;
	sub_82367D30(ctx, base);
	// 82204C30: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 82204C34: A147001A  lhz r10, 0x1a(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204C38: 5568043E  clrlwi r8, r11, 0x10
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82204C3C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82204C40: 4198FFD0  blt cr6, 0x82204c10
	if ctx.cr[6].lt {
	pc = 0x82204C10; continue 'dispatch;
	}
	// 82204C44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82204C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82204C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82204C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82204C58 size=68
    let mut pc: u32 = 0x82204C58;
    'dispatch: loop {
        match pc {
            0x82204C58 => {
    //   block [0x82204C58..0x82204C9C)
	// 82204C58: A1430018  lhz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82204C5C: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82204C60: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82204C64: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 82204C68: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82204C6C: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82204C70: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82204C74: A1630018  lhz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82204C78: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82204C7C: 556B103E  rotlwi r11, r11, 2
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82204C80: 7D4B412E  stwx r10, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 82204C84: C0030028  lfs f0, 0x28(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82204C88: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82204C8C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82204C90: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82204C94: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82204C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204CA0 size=328
    let mut pc: u32 = 0x82204CA0;
    'dispatch: loop {
        match pc {
            0x82204CA0 => {
    //   block [0x82204CA0..0x82204DE8)
	// 82204CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204CA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82204CAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82204CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204CB4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82204CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82204CBC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82204CC0: 396B3D00  addi r11, r11, 0x3d00
	ctx.r[11].s64 = ctx.r[11].s64 + 15616;
	// 82204CC4: 395F004F  addi r10, r31, 0x4f
	ctx.r[10].s64 = ctx.r[31].s64 + 79;
	// 82204CC8: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82204CCC: 554A0036  rlwinm r10, r10, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82204CD0: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82204CD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82204CD8: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82204CDC: FBDF0010  std r30, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u64 ) };
	// 82204CE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82204CE4: A165000A  lhz r11, 0xa(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(10 as u32) ) } as u64;
	// 82204CE8: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82204CEC: B17F0018  sth r11, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 82204CF0: A1650008  lhz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82204CF4: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82204CF8: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82204CFC: B17F001A  sth r11, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[11].u16 ) };
	// 82204D00: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82204D04: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 82204D08: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82204D0C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82204D10: A15F001A  lhz r10, 0x1a(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204D14: 554A303E  rotlwi r10, r10, 6
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(6)) as u64;
	// 82204D18: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82204D1C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82204D20: 48184781  bl 0x823894a0
	ctx.lr = 0x82204D24;
	sub_823894A0(ctx, base);
	// 82204D24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204D28: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82204D2C: 409A0010  bne cr6, 0x82204d3c
	if !ctx.cr[6].eq {
	pc = 0x82204D3C; continue 'dispatch;
	}
	// 82204D30: 3D608284  lis r11, -0x7d7c
	ctx.r[11].s64 = -2105278464;
	// 82204D34: 816BC8BC  lwz r11, -0x3744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14148 as u32) ) } as u64;
	// 82204D38: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82204D3C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82204D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82204D44: 4818475D  bl 0x823894a0
	ctx.lr = 0x82204D48;
	sub_823894A0(ctx, base);
	// 82204D48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204D4C: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82204D50: 409A006C  bne cr6, 0x82204dbc
	if !ctx.cr[6].eq {
	pc = 0x82204DBC; continue 'dispatch;
	}
	// 82204D54: 3D208311  lis r9, -0x7cef
	ctx.r[9].s64 = -2096037888;
	// 82204D58: 81493C44  lwz r10, 0x3c44(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(15428 as u32) ) } as u64;
	// 82204D5C: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82204D60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204D64: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 82204D68: 396B3B70  addi r11, r11, 0x3b70
	ctx.r[11].s64 = ctx.r[11].s64 + 15216;
	// 82204D6C: 409A004C  bne cr6, 0x82204db8
	if !ctx.cr[6].eq {
	pc = 0x82204DB8; continue 'dispatch;
	}
	// 82204D70: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 82204D74: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82204D78: 91493C44  stw r10, 0x3c44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(15428 as u32), ctx.r[10].u32 ) };
	// 82204D7C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204DE8 size=24
    let mut pc: u32 = 0x82204DE8;
    'dispatch: loop {
        match pc {
            0x82204DE8 => {
    //   block [0x82204DE8..0x82204E00)
	// 82204DE8: A163001A  lhz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204DEC: 556A083E  rotlwi r10, r11, 1
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82204DF0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82204DF4: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82204DF8: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82204DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82204E00 size=60
    let mut pc: u32 = 0x82204E00;
    'dispatch: loop {
        match pc {
            0x82204E00 => {
    //   block [0x82204E00..0x82204E3C)
	// 82204E00: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82204E04: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82204E08: A1230018  lhz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82204E0C: 394A0007  addi r10, r10, 7
	ctx.r[10].s64 = ctx.r[10].s64 + 7;
	// 82204E10: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 82204E14: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82204E18: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82204E1C: 7D4A182E  lwzx r10, r10, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82204E20: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82204E24: C003002C  lfs f0, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82204E28: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82204E2C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82204E30: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82204E34: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82204E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204E40 size=356
    let mut pc: u32 = 0x82204E40;
    'dispatch: loop {
        match pc {
            0x82204E40 => {
    //   block [0x82204E40..0x82204FA4)
	// 82204E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82204E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82204E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204E54: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82204E58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82204E5C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82204E60: 396B3D2C  addi r11, r11, 0x3d2c
	ctx.r[11].s64 = ctx.r[11].s64 + 15660;
	// 82204E64: 395F0057  addi r10, r31, 0x57
	ctx.r[10].s64 = ctx.r[31].s64 + 87;
	// 82204E68: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82204E6C: 554A0036  rlwinm r10, r10, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82204E70: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82204E74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82204E78: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82204E7C: FBDF0010  std r30, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u64 ) };
	// 82204E80: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82204E84: A165000A  lhz r11, 0xa(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(10 as u32) ) } as u64;
	// 82204E88: 556B083E  rotlwi r11, r11, 1
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82204E8C: B17F0018  sth r11, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 82204E90: A1650008  lhz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82204E94: B17F001A  sth r11, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[11].u16 ) };
	// 82204E98: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82204E9C: 81250064  lwz r9, 0x64(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(100 as u32) ) } as u64;
	// 82204EA0: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82204EA4: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 82204EA8: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82204EAC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82204EB0: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82204EB4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82204EB8: A15F001A  lhz r10, 0x1a(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204EBC: 554A303E  rotlwi r10, r10, 6
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(6)) as u64;
	// 82204EC0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82204EC4: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82204EC8: A17F001A  lhz r11, 0x1a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204ECC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82204ED0: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 82204ED4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82204ED8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82204EDC: 481845C5  bl 0x823894a0
	ctx.lr = 0x82204EE0;
	sub_823894A0(ctx, base);
	// 82204EE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204EE4: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82204EE8: 409A0010  bne cr6, 0x82204ef8
	if !ctx.cr[6].eq {
	pc = 0x82204EF8; continue 'dispatch;
	}
	// 82204EEC: 3D608284  lis r11, -0x7d7c
	ctx.r[11].s64 = -2105278464;
	// 82204EF0: 816BC8BC  lwz r11, -0x3744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14148 as u32) ) } as u64;
	// 82204EF4: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82204EF8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82204EFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82204F00: 481845A1  bl 0x823894a0
	ctx.lr = 0x82204F04;
	sub_823894A0(ctx, base);
	// 82204F04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82204F08: 907F0040  stw r3, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[3].u32 ) };
	// 82204F0C: 409A006C  bne cr6, 0x82204f78
	if !ctx.cr[6].eq {
	pc = 0x82204F78; continue 'dispatch;
	}
	// 82204F10: 3D208311  lis r9, -0x7cef
	ctx.r[9].s64 = -2096037888;
	// 82204F14: 81493C44  lwz r10, 0x3c44(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(15428 as u32) ) } as u64;
	// 82204F18: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82204F1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204F20: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 82204F24: 396B3B70  addi r11, r11, 0x3b70
	ctx.r[11].s64 = ctx.r[11].s64 + 15216;
	// 82204F28: 409A004C  bne cr6, 0x82204f74
	if !ctx.cr[6].eq {
	pc = 0x82204F74; continue 'dispatch;
	}
	// 82204F2C: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 82204F30: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82204F34: 91493C44  stw r10, 0x3c44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(15428 as u32), ctx.r[10].u32 ) };
	// 82204F38: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82204FA8 size=16
    let mut pc: u32 = 0x82204FA8;
    'dispatch: loop {
        match pc {
            0x82204FA8 => {
    //   block [0x82204FA8..0x82204FB8)
	// 82204FA8: A163001A  lhz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204FAC: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 82204FB0: 386B0050  addi r3, r11, 0x50
	ctx.r[3].s64 = ctx.r[11].s64 + 80;
	// 82204FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82204FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82204FB8 size=112
    let mut pc: u32 = 0x82204FB8;
    'dispatch: loop {
        match pc {
            0x82204FB8 => {
    //   block [0x82204FB8..0x82205028)
	// 82204FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82204FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82204FC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82204FC4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82204FC8: A168001A  lhz r11, 0x1a(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(26 as u32) ) } as u64;
	// 82204FCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82204FD0: 419A0048  beq cr6, 0x82205018
	if ctx.cr[6].eq {
	pc = 0x82205018; continue 'dispatch;
	}
	// 82204FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82204FD8: 8148002C  lwz r10, 0x2c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(44 as u32) ) } as u64;
	// 82204FDC: 54EB3032  slwi r11, r7, 6
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82204FE0: 80C80030  lwz r6, 0x30(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(48 as u32) ) } as u64;
	// 82204FE4: 392A0007  addi r9, r10, 7
	ctx.r[9].s64 = ctx.r[10].s64 + 7;
	// 82204FE8: 81480028  lwz r10, 0x28(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(40 as u32) ) } as u64;
	// 82204FEC: 7CA65A14  add r5, r6, r11
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82204FF0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82204FF4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82204FF8: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82204FFC: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82205000: 48162D31  bl 0x82367d30
	ctx.lr = 0x82205004;
	sub_82367D30(ctx, base);
	// 82205004: 39670001  addi r11, r7, 1
	ctx.r[11].s64 = ctx.r[7].s64 + 1;
	// 82205008: A148001A  lhz r10, 0x1a(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(26 as u32) ) } as u64;
	// 8220500C: 5567043E  clrlwi r7, r11, 0x10
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82205010: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82205014: 4198FFC4  blt cr6, 0x82204fd8
	if ctx.cr[6].lt {
	pc = 0x82204FD8; continue 'dispatch;
	}
	// 82205018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220501C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82205028 size=80
    let mut pc: u32 = 0x82205028;
    'dispatch: loop {
        match pc {
            0x82205028 => {
    //   block [0x82205028..0x82205078)
	// 82205028: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8220502C: 3D6082B6  lis r11, -0x7d4a
	ctx.r[11].s64 = -2102001664;
	// 82205030: A1230018  lhz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82205034: 394A0007  addi r10, r10, 7
	ctx.r[10].s64 = ctx.r[10].s64 + 7;
	// 82205038: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 8220503C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82205040: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82205044: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82205048: 7D4A182E  lwzx r10, r10, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8220504C: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82205050: A1630018  lhz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82205054: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82205058: 556B103E  rotlwi r11, r11, 2
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 8220505C: 7D4B412E  stwx r10, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 82205060: C0030034  lfs f0, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82205064: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82205068: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8220506C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82205070: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82205074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205078 size=44
    let mut pc: u32 = 0x82205078;
    'dispatch: loop {
        match pc {
            0x82205078 => {
    //   block [0x82205078..0x822050A4)
	// 82205078: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220507C: 419A0028  beq cr6, 0x822050a4
	if ctx.cr[6].eq {
		sub_822050A4(ctx, base);
		return;
	}
	// 82205080: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82205084: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8220508C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205090: 396BDA38  addi r11, r11, -0x25c8
	ctx.r[11].s64 = ctx.r[11].s64 + -9672;
	// 82205094: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205098: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8220509C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822050A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822050A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822050A4 size=8
    let mut pc: u32 = 0x822050A4;
    'dispatch: loop {
        match pc {
            0x822050A4 => {
    //   block [0x822050A4..0x822050AC)
	// 822050A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822050A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822050B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822050B0 size=8
    let mut pc: u32 = 0x822050B0;
    'dispatch: loop {
        match pc {
            0x822050B0 => {
    //   block [0x822050B0..0x822050B8)
	// 822050B0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 822050B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822050B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822050B8 size=44
    let mut pc: u32 = 0x822050B8;
    'dispatch: loop {
        match pc {
            0x822050B8 => {
    //   block [0x822050B8..0x822050E4)
	// 822050B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822050BC: 419A0028  beq cr6, 0x822050e4
	if ctx.cr[6].eq {
		sub_822050E4(ctx, base);
		return;
	}
	// 822050C0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 822050C4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822050C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822050CC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822050D0: 396BDA64  addi r11, r11, -0x259c
	ctx.r[11].s64 = ctx.r[11].s64 + -9628;
	// 822050D4: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822050D8: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822050DC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822050E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822050E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822050E4 size=8
    let mut pc: u32 = 0x822050E4;
    'dispatch: loop {
        match pc {
            0x822050E4 => {
    //   block [0x822050E4..0x822050EC)
	// 822050E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822050E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822050F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822050F0 size=8
    let mut pc: u32 = 0x822050F0;
    'dispatch: loop {
        match pc {
            0x822050F0 => {
    //   block [0x822050F0..0x822050F8)
	// 822050F0: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 822050F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822050F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822050F8 size=44
    let mut pc: u32 = 0x822050F8;
    'dispatch: loop {
        match pc {
            0x822050F8 => {
    //   block [0x822050F8..0x82205124)
	// 822050F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822050FC: 419A0028  beq cr6, 0x82205124
	if ctx.cr[6].eq {
		sub_82205124(ctx, base);
		return;
	}
	// 82205100: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205104: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8220510C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205110: 396B3DA4  addi r11, r11, 0x3da4
	ctx.r[11].s64 = ctx.r[11].s64 + 15780;
	// 82205114: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205118: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8220511C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205124 size=8
    let mut pc: u32 = 0x82205124;
    'dispatch: loop {
        match pc {
            0x82205124 => {
    //   block [0x82205124..0x8220512C)
	// 82205124: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205130 size=44
    let mut pc: u32 = 0x82205130;
    'dispatch: loop {
        match pc {
            0x82205130 => {
    //   block [0x82205130..0x8220515C)
	// 82205130: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205134: 419A0028  beq cr6, 0x8220515c
	if ctx.cr[6].eq {
		sub_8220515C(ctx, base);
		return;
	}
	// 82205138: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220513C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82205144: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205148: 396B3DD0  addi r11, r11, 0x3dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 15824;
	// 8220514C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205150: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82205154: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220515C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220515C size=8
    let mut pc: u32 = 0x8220515C;
    'dispatch: loop {
        match pc {
            0x8220515C => {
    //   block [0x8220515C..0x82205164)
	// 8220515C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205168 size=44
    let mut pc: u32 = 0x82205168;
    'dispatch: loop {
        match pc {
            0x82205168 => {
    //   block [0x82205168..0x82205194)
	// 82205168: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220516C: 419A0028  beq cr6, 0x82205194
	if ctx.cr[6].eq {
		sub_82205194(ctx, base);
		return;
	}
	// 82205170: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205174: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8220517C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205180: 396B3DFC  addi r11, r11, 0x3dfc
	ctx.r[11].s64 = ctx.r[11].s64 + 15868;
	// 82205184: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205188: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8220518C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205194 size=8
    let mut pc: u32 = 0x82205194;
    'dispatch: loop {
        match pc {
            0x82205194 => {
    //   block [0x82205194..0x8220519C)
	// 82205194: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822051A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822051A0 size=44
    let mut pc: u32 = 0x822051A0;
    'dispatch: loop {
        match pc {
            0x822051A0 => {
    //   block [0x822051A0..0x822051CC)
	// 822051A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822051A4: 419A0028  beq cr6, 0x822051cc
	if ctx.cr[6].eq {
		sub_822051CC(ctx, base);
		return;
	}
	// 822051A8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822051AC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822051B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822051B4: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822051B8: 396B3E28  addi r11, r11, 0x3e28
	ctx.r[11].s64 = ctx.r[11].s64 + 15912;
	// 822051BC: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822051C0: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822051C4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822051C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822051CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822051CC size=8
    let mut pc: u32 = 0x822051CC;
    'dispatch: loop {
        match pc {
            0x822051CC => {
    //   block [0x822051CC..0x822051D4)
	// 822051CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822051D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822051D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822051D8 size=44
    let mut pc: u32 = 0x822051D8;
    'dispatch: loop {
        match pc {
            0x822051D8 => {
    //   block [0x822051D8..0x82205204)
	// 822051D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822051DC: 419A0028  beq cr6, 0x82205204
	if ctx.cr[6].eq {
		sub_82205204(ctx, base);
		return;
	}
	// 822051E0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822051E4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822051E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822051EC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822051F0: 396B3E54  addi r11, r11, 0x3e54
	ctx.r[11].s64 = ctx.r[11].s64 + 15956;
	// 822051F4: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822051F8: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822051FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205204(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205204 size=8
    let mut pc: u32 = 0x82205204;
    'dispatch: loop {
        match pc {
            0x82205204 => {
    //   block [0x82205204..0x8220520C)
	// 82205204: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205210 size=44
    let mut pc: u32 = 0x82205210;
    'dispatch: loop {
        match pc {
            0x82205210 => {
    //   block [0x82205210..0x8220523C)
	// 82205210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205214: 419A0028  beq cr6, 0x8220523c
	if ctx.cr[6].eq {
		sub_8220523C(ctx, base);
		return;
	}
	// 82205218: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220521C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82205224: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205228: 396B3E80  addi r11, r11, 0x3e80
	ctx.r[11].s64 = ctx.r[11].s64 + 16000;
	// 8220522C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205230: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82205234: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220523C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220523C size=8
    let mut pc: u32 = 0x8220523C;
    'dispatch: loop {
        match pc {
            0x8220523C => {
    //   block [0x8220523C..0x82205244)
	// 8220523C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205248 size=92
    let mut pc: u32 = 0x82205248;
    'dispatch: loop {
        match pc {
            0x82205248 => {
    //   block [0x82205248..0x822052A4)
	// 82205248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220524C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220525C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205260: 419A002C  beq cr6, 0x8220528c
	if ctx.cr[6].eq {
	pc = 0x8220528C; continue 'dispatch;
	}
	// 82205264: 4BFFF885  bl 0x82204ae8
	ctx.lr = 0x82205268;
	sub_82204AE8(ctx, base);
	// 82205268: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220526C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205270: 396B3EAC  addi r11, r11, 0x3eac
	ctx.r[11].s64 = ctx.r[11].s64 + 16044;
	// 82205274: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220527C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205288: 4E800020  blr
	return;
	// 8220528C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220529C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822052A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822052A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822052A8 size=16
    let mut pc: u32 = 0x822052A8;
    'dispatch: loop {
        match pc {
            0x822052A8 => {
    //   block [0x822052A8..0x822052B8)
	// 822052A8: A1630008  lhz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822052AC: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 822052B0: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 822052B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822052B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822052B8 size=92
    let mut pc: u32 = 0x822052B8;
    'dispatch: loop {
        match pc {
            0x822052B8 => {
    //   block [0x822052B8..0x82205314)
	// 822052B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822052BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822052C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822052C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822052C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822052CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822052D0: 419A002C  beq cr6, 0x822052fc
	if ctx.cr[6].eq {
	pc = 0x822052FC; continue 'dispatch;
	}
	// 822052D4: 4BFFF815  bl 0x82204ae8
	ctx.lr = 0x822052D8;
	sub_82204AE8(ctx, base);
	// 822052D8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822052DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822052E0: 396B3ED8  addi r11, r11, 0x3ed8
	ctx.r[11].s64 = ctx.r[11].s64 + 16088;
	// 822052E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822052E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822052EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822052F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822052F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822052F8: 4E800020  blr
	return;
	// 822052FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220530C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205318 size=92
    let mut pc: u32 = 0x82205318;
    'dispatch: loop {
        match pc {
            0x82205318 => {
    //   block [0x82205318..0x82205374)
	// 82205318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220531C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205320: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220532C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205330: 419A002C  beq cr6, 0x8220535c
	if ctx.cr[6].eq {
	pc = 0x8220535C; continue 'dispatch;
	}
	// 82205334: 4BFFF7B5  bl 0x82204ae8
	ctx.lr = 0x82205338;
	sub_82204AE8(ctx, base);
	// 82205338: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220533C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205340: 396B3F04  addi r11, r11, 0x3f04
	ctx.r[11].s64 = ctx.r[11].s64 + 16132;
	// 82205344: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220534C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205358: 4E800020  blr
	return;
	// 8220535C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220536C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205378 size=92
    let mut pc: u32 = 0x82205378;
    'dispatch: loop {
        match pc {
            0x82205378 => {
    //   block [0x82205378..0x822053D4)
	// 82205378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220537C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220538C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205390: 419A002C  beq cr6, 0x822053bc
	if ctx.cr[6].eq {
	pc = 0x822053BC; continue 'dispatch;
	}
	// 82205394: 4BFFF755  bl 0x82204ae8
	ctx.lr = 0x82205398;
	sub_82204AE8(ctx, base);
	// 82205398: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220539C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822053A0: 396B3F30  addi r11, r11, 0x3f30
	ctx.r[11].s64 = ctx.r[11].s64 + 16176;
	// 822053A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822053A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822053AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822053B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822053B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822053B8: 4E800020  blr
	return;
	// 822053BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822053C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822053C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822053C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822053CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822053D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822053D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822053D8 size=44
    let mut pc: u32 = 0x822053D8;
    'dispatch: loop {
        match pc {
            0x822053D8 => {
    //   block [0x822053D8..0x82205404)
	// 822053D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822053DC: 419A0028  beq cr6, 0x82205404
	if ctx.cr[6].eq {
		sub_82205404(ctx, base);
		return;
	}
	// 822053E0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822053E4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822053E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822053EC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822053F0: 396B3F5C  addi r11, r11, 0x3f5c
	ctx.r[11].s64 = ctx.r[11].s64 + 16220;
	// 822053F4: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822053F8: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822053FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205404(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205404 size=8
    let mut pc: u32 = 0x82205404;
    'dispatch: loop {
        match pc {
            0x82205404 => {
    //   block [0x82205404..0x8220540C)
	// 82205404: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205410 size=44
    let mut pc: u32 = 0x82205410;
    'dispatch: loop {
        match pc {
            0x82205410 => {
    //   block [0x82205410..0x8220543C)
	// 82205410: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205414: 419A0028  beq cr6, 0x8220543c
	if ctx.cr[6].eq {
		sub_8220543C(ctx, base);
		return;
	}
	// 82205418: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220541C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82205424: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205428: 396B3F88  addi r11, r11, 0x3f88
	ctx.r[11].s64 = ctx.r[11].s64 + 16264;
	// 8220542C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205430: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82205434: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220543C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220543C size=8
    let mut pc: u32 = 0x8220543C;
    'dispatch: loop {
        match pc {
            0x8220543C => {
    //   block [0x8220543C..0x82205444)
	// 8220543C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205448 size=92
    let mut pc: u32 = 0x82205448;
    'dispatch: loop {
        match pc {
            0x82205448 => {
    //   block [0x82205448..0x822054A4)
	// 82205448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220544C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220545C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205460: 419A002C  beq cr6, 0x8220548c
	if ctx.cr[6].eq {
	pc = 0x8220548C; continue 'dispatch;
	}
	// 82205464: 4BFFF5F5  bl 0x82204a58
	ctx.lr = 0x82205468;
	sub_82204A58(ctx, base);
	// 82205468: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220546C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205470: 396B3FB4  addi r11, r11, 0x3fb4
	ctx.r[11].s64 = ctx.r[11].s64 + 16308;
	// 82205474: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220547C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205488: 4E800020  blr
	return;
	// 8220548C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220549C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822054A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822054A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822054A8 size=44
    let mut pc: u32 = 0x822054A8;
    'dispatch: loop {
        match pc {
            0x822054A8 => {
    //   block [0x822054A8..0x822054D4)
	// 822054A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822054AC: 419A0028  beq cr6, 0x822054d4
	if ctx.cr[6].eq {
		sub_822054D4(ctx, base);
		return;
	}
	// 822054B0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822054B4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822054B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822054BC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822054C0: 396B3FE0  addi r11, r11, 0x3fe0
	ctx.r[11].s64 = ctx.r[11].s64 + 16352;
	// 822054C4: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822054C8: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822054CC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822054D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822054D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822054D4 size=8
    let mut pc: u32 = 0x822054D4;
    'dispatch: loop {
        match pc {
            0x822054D4 => {
    //   block [0x822054D4..0x822054DC)
	// 822054D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822054D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822054E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822054E0 size=92
    let mut pc: u32 = 0x822054E0;
    'dispatch: loop {
        match pc {
            0x822054E0 => {
    //   block [0x822054E0..0x8220553C)
	// 822054E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822054E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822054E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822054EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822054F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822054F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822054F8: 419A002C  beq cr6, 0x82205524
	if ctx.cr[6].eq {
	pc = 0x82205524; continue 'dispatch;
	}
	// 822054FC: 4BFFF5ED  bl 0x82204ae8
	ctx.lr = 0x82205500;
	sub_82204AE8(ctx, base);
	// 82205500: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205508: 396B400C  addi r11, r11, 0x400c
	ctx.r[11].s64 = ctx.r[11].s64 + 16396;
	// 8220550C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220551C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205520: 4E800020  blr
	return;
	// 82205524: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220552C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205534: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205540 size=92
    let mut pc: u32 = 0x82205540;
    'dispatch: loop {
        match pc {
            0x82205540 => {
    //   block [0x82205540..0x8220559C)
	// 82205540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8220554C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205554: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205558: 419A002C  beq cr6, 0x82205584
	if ctx.cr[6].eq {
	pc = 0x82205584; continue 'dispatch;
	}
	// 8220555C: 4BFFF58D  bl 0x82204ae8
	ctx.lr = 0x82205560;
	sub_82204AE8(ctx, base);
	// 82205560: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205568: 396B4038  addi r11, r11, 0x4038
	ctx.r[11].s64 = ctx.r[11].s64 + 16440;
	// 8220556C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220557C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205580: 4E800020  blr
	return;
	// 82205584: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220558C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822055A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822055A0 size=44
    let mut pc: u32 = 0x822055A0;
    'dispatch: loop {
        match pc {
            0x822055A0 => {
    //   block [0x822055A0..0x822055CC)
	// 822055A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822055A4: 419A0028  beq cr6, 0x822055cc
	if ctx.cr[6].eq {
		sub_822055CC(ctx, base);
		return;
	}
	// 822055A8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822055AC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822055B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822055B4: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822055B8: 396B4064  addi r11, r11, 0x4064
	ctx.r[11].s64 = ctx.r[11].s64 + 16484;
	// 822055BC: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822055C0: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822055C4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822055C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822055CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822055CC size=8
    let mut pc: u32 = 0x822055CC;
    'dispatch: loop {
        match pc {
            0x822055CC => {
    //   block [0x822055CC..0x822055D4)
	// 822055CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822055D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822055D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822055D8 size=92
    let mut pc: u32 = 0x822055D8;
    'dispatch: loop {
        match pc {
            0x822055D8 => {
    //   block [0x822055D8..0x82205634)
	// 822055D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822055DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822055E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822055E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822055E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822055EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822055F0: 419A002C  beq cr6, 0x8220561c
	if ctx.cr[6].eq {
	pc = 0x8220561C; continue 'dispatch;
	}
	// 822055F4: 4BFFF84D  bl 0x82204e40
	ctx.lr = 0x822055F8;
	sub_82204E40(ctx, base);
	// 822055F8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822055FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205600: 396B4090  addi r11, r11, 0x4090
	ctx.r[11].s64 = ctx.r[11].s64 + 16528;
	// 82205604: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220560C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205618: 4E800020  blr
	return;
	// 8220561C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220562C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205638 size=16
    let mut pc: u32 = 0x82205638;
    'dispatch: loop {
        match pc {
            0x82205638 => {
    //   block [0x82205638..0x82205648)
	// 82205638: A1630008  lhz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220563C: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 82205640: 386B0050  addi r3, r11, 0x50
	ctx.r[3].s64 = ctx.r[11].s64 + 80;
	// 82205644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205648 size=92
    let mut pc: u32 = 0x82205648;
    'dispatch: loop {
        match pc {
            0x82205648 => {
    //   block [0x82205648..0x822056A4)
	// 82205648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220564C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205658: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220565C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205660: 419A002C  beq cr6, 0x8220568c
	if ctx.cr[6].eq {
	pc = 0x8220568C; continue 'dispatch;
	}
	// 82205664: 4BFFF7DD  bl 0x82204e40
	ctx.lr = 0x82205668;
	sub_82204E40(ctx, base);
	// 82205668: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220566C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205670: 396B40BC  addi r11, r11, 0x40bc
	ctx.r[11].s64 = ctx.r[11].s64 + 16572;
	// 82205674: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220567C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205688: 4E800020  blr
	return;
	// 8220568C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220569C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822056A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822056A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822056A8 size=44
    let mut pc: u32 = 0x822056A8;
    'dispatch: loop {
        match pc {
            0x822056A8 => {
    //   block [0x822056A8..0x822056D4)
	// 822056A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822056AC: 419A0028  beq cr6, 0x822056d4
	if ctx.cr[6].eq {
		sub_822056D4(ctx, base);
		return;
	}
	// 822056B0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822056B4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822056B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822056BC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822056C0: 396B40E8  addi r11, r11, 0x40e8
	ctx.r[11].s64 = ctx.r[11].s64 + 16616;
	// 822056C4: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822056C8: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822056CC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822056D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822056D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822056D4 size=8
    let mut pc: u32 = 0x822056D4;
    'dispatch: loop {
        match pc {
            0x822056D4 => {
    //   block [0x822056D4..0x822056DC)
	// 822056D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822056D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822056E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822056E0 size=44
    let mut pc: u32 = 0x822056E0;
    'dispatch: loop {
        match pc {
            0x822056E0 => {
    //   block [0x822056E0..0x8220570C)
	// 822056E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822056E4: 419A0028  beq cr6, 0x8220570c
	if ctx.cr[6].eq {
		sub_8220570C(ctx, base);
		return;
	}
	// 822056E8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822056EC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822056F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822056F4: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822056F8: 396B4114  addi r11, r11, 0x4114
	ctx.r[11].s64 = ctx.r[11].s64 + 16660;
	// 822056FC: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205700: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82205704: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220570C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220570C size=8
    let mut pc: u32 = 0x8220570C;
    'dispatch: loop {
        match pc {
            0x8220570C => {
    //   block [0x8220570C..0x82205714)
	// 8220570C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205718 size=12
    let mut pc: u32 = 0x82205718;
    'dispatch: loop {
        match pc {
            0x82205718 => {
    //   block [0x82205718..0x82205724)
	// 82205718: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220571C: 419A0008  beq cr6, 0x82205724
	if ctx.cr[6].eq {
		sub_82205724(ctx, base);
		return;
	}
	// 82205720: 480015F0  b 0x82206d10
	sub_82206D10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205724 size=8
    let mut pc: u32 = 0x82205724;
    'dispatch: loop {
        match pc {
            0x82205724 => {
    //   block [0x82205724..0x8220572C)
	// 82205724: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205730 size=16
    let mut pc: u32 = 0x82205730;
    'dispatch: loop {
        match pc {
            0x82205730 => {
    //   block [0x82205730..0x82205740)
	// 82205730: A1630008  lhz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82205734: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 82205738: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 8220573C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205740 size=44
    let mut pc: u32 = 0x82205740;
    'dispatch: loop {
        match pc {
            0x82205740 => {
    //   block [0x82205740..0x8220576C)
	// 82205740: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205744: 419A0028  beq cr6, 0x8220576c
	if ctx.cr[6].eq {
		sub_8220576C(ctx, base);
		return;
	}
	// 82205748: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220574C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82205754: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205758: 396B416C  addi r11, r11, 0x416c
	ctx.r[11].s64 = ctx.r[11].s64 + 16748;
	// 8220575C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205760: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82205764: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220576C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220576C size=8
    let mut pc: u32 = 0x8220576C;
    'dispatch: loop {
        match pc {
            0x8220576C => {
    //   block [0x8220576C..0x82205774)
	// 8220576C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205778 size=44
    let mut pc: u32 = 0x82205778;
    'dispatch: loop {
        match pc {
            0x82205778 => {
    //   block [0x82205778..0x822057A4)
	// 82205778: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220577C: 419A0028  beq cr6, 0x822057a4
	if ctx.cr[6].eq {
		sub_822057A4(ctx, base);
		return;
	}
	// 82205780: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205784: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8220578C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205790: 396B4198  addi r11, r11, 0x4198
	ctx.r[11].s64 = ctx.r[11].s64 + 16792;
	// 82205794: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205798: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8220579C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822057A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822057A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822057A4 size=8
    let mut pc: u32 = 0x822057A4;
    'dispatch: loop {
        match pc {
            0x822057A4 => {
    //   block [0x822057A4..0x822057AC)
	// 822057A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822057A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822057B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822057B0 size=92
    let mut pc: u32 = 0x822057B0;
    'dispatch: loop {
        match pc {
            0x822057B0 => {
    //   block [0x822057B0..0x8220580C)
	// 822057B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822057B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822057B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822057BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822057C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822057C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822057C8: 419A002C  beq cr6, 0x822057f4
	if ctx.cr[6].eq {
	pc = 0x822057F4; continue 'dispatch;
	}
	// 822057CC: 4BFFF675  bl 0x82204e40
	ctx.lr = 0x822057D0;
	sub_82204E40(ctx, base);
	// 822057D0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822057D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822057D8: 396B41C4  addi r11, r11, 0x41c4
	ctx.r[11].s64 = ctx.r[11].s64 + 16836;
	// 822057DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822057E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822057E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822057E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822057EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822057F0: 4E800020  blr
	return;
	// 822057F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822057F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822057FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205810 size=44
    let mut pc: u32 = 0x82205810;
    'dispatch: loop {
        match pc {
            0x82205810 => {
    //   block [0x82205810..0x8220583C)
	// 82205810: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205814: 419A0028  beq cr6, 0x8220583c
	if ctx.cr[6].eq {
		sub_8220583C(ctx, base);
		return;
	}
	// 82205818: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220581C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82205824: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205828: 396B41F0  addi r11, r11, 0x41f0
	ctx.r[11].s64 = ctx.r[11].s64 + 16880;
	// 8220582C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205830: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82205834: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220583C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220583C size=8
    let mut pc: u32 = 0x8220583C;
    'dispatch: loop {
        match pc {
            0x8220583C => {
    //   block [0x8220583C..0x82205844)
	// 8220583C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205848 size=92
    let mut pc: u32 = 0x82205848;
    'dispatch: loop {
        match pc {
            0x82205848 => {
    //   block [0x82205848..0x822058A4)
	// 82205848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220584C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220585C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205860: 419A002C  beq cr6, 0x8220588c
	if ctx.cr[6].eq {
	pc = 0x8220588C; continue 'dispatch;
	}
	// 82205864: 4BFFF125  bl 0x82204988
	ctx.lr = 0x82205868;
	sub_82204988(ctx, base);
	// 82205868: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220586C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205870: 396B421C  addi r11, r11, 0x421c
	ctx.r[11].s64 = ctx.r[11].s64 + 16924;
	// 82205874: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220587C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205888: 4E800020  blr
	return;
	// 8220588C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220589C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822058A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822058A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822058A8 size=44
    let mut pc: u32 = 0x822058A8;
    'dispatch: loop {
        match pc {
            0x822058A8 => {
    //   block [0x822058A8..0x822058D4)
	// 822058A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822058AC: 419A0028  beq cr6, 0x822058d4
	if ctx.cr[6].eq {
		sub_822058D4(ctx, base);
		return;
	}
	// 822058B0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822058B4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822058B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822058BC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 822058C0: 396B4248  addi r11, r11, 0x4248
	ctx.r[11].s64 = ctx.r[11].s64 + 16968;
	// 822058C4: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 822058C8: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 822058CC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822058D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822058D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822058D4 size=8
    let mut pc: u32 = 0x822058D4;
    'dispatch: loop {
        match pc {
            0x822058D4 => {
    //   block [0x822058D4..0x822058DC)
	// 822058D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822058D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822058E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822058E0 size=12
    let mut pc: u32 = 0x822058E0;
    'dispatch: loop {
        match pc {
            0x822058E0 => {
    //   block [0x822058E0..0x822058EC)
	// 822058E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822058E4: 419A0008  beq cr6, 0x822058ec
	if ctx.cr[6].eq {
		sub_822058EC(ctx, base);
		return;
	}
	// 822058E8: 4812A118  b 0x8232fa00
	sub_8232FA00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822058EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822058EC size=8
    let mut pc: u32 = 0x822058EC;
    'dispatch: loop {
        match pc {
            0x822058EC => {
    //   block [0x822058EC..0x822058F4)
	// 822058EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822058F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822058F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822058F8 size=92
    let mut pc: u32 = 0x822058F8;
    'dispatch: loop {
        match pc {
            0x822058F8 => {
    //   block [0x822058F8..0x82205954)
	// 822058F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822058FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220590C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205910: 419A002C  beq cr6, 0x8220593c
	if ctx.cr[6].eq {
	pc = 0x8220593C; continue 'dispatch;
	}
	// 82205914: 4BFFF38D  bl 0x82204ca0
	ctx.lr = 0x82205918;
	sub_82204CA0(ctx, base);
	// 82205918: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220591C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205920: 396B4274  addi r11, r11, 0x4274
	ctx.r[11].s64 = ctx.r[11].s64 + 17012;
	// 82205924: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220592C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205938: 4E800020  blr
	return;
	// 8220593C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220594C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205958 size=24
    let mut pc: u32 = 0x82205958;
    'dispatch: loop {
        match pc {
            0x82205958 => {
    //   block [0x82205958..0x82205970)
	// 82205958: A1630008  lhz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220595C: 556A083E  rotlwi r10, r11, 1
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82205960: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82205964: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82205968: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 8220596C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205970 size=92
    let mut pc: u32 = 0x82205970;
    'dispatch: loop {
        match pc {
            0x82205970 => {
    //   block [0x82205970..0x822059CC)
	// 82205970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205978: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8220597C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205980: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205984: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205988: 419A002C  beq cr6, 0x822059b4
	if ctx.cr[6].eq {
	pc = 0x822059B4; continue 'dispatch;
	}
	// 8220598C: 4BFFF4B5  bl 0x82204e40
	ctx.lr = 0x82205990;
	sub_82204E40(ctx, base);
	// 82205990: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205998: 396B42A0  addi r11, r11, 0x42a0
	ctx.r[11].s64 = ctx.r[11].s64 + 17056;
	// 8220599C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822059A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822059A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822059A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822059AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822059B0: 4E800020  blr
	return;
	// 822059B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822059B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822059BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822059C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822059C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822059C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822059D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822059D0 size=92
    let mut pc: u32 = 0x822059D0;
    'dispatch: loop {
        match pc {
            0x822059D0 => {
    //   block [0x822059D0..0x82205A2C)
	// 822059D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822059D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822059D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822059DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822059E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822059E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822059E8: 419A002C  beq cr6, 0x82205a14
	if ctx.cr[6].eq {
	pc = 0x82205A14; continue 'dispatch;
	}
	// 822059EC: 4BFFF06D  bl 0x82204a58
	ctx.lr = 0x822059F0;
	sub_82204A58(ctx, base);
	// 822059F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822059F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822059F8: 396B42CC  addi r11, r11, 0x42cc
	ctx.r[11].s64 = ctx.r[11].s64 + 17100;
	// 822059FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205A0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205A10: 4E800020  blr
	return;
	// 82205A14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205A24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205A30 size=92
    let mut pc: u32 = 0x82205A30;
    'dispatch: loop {
        match pc {
            0x82205A30 => {
    //   block [0x82205A30..0x82205A8C)
	// 82205A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205A38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205A3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205A40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205A44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82205A48: 419A002C  beq cr6, 0x82205a74
	if ctx.cr[6].eq {
	pc = 0x82205A74; continue 'dispatch;
	}
	// 82205A4C: 4BFFEF3D  bl 0x82204988
	ctx.lr = 0x82205A50;
	sub_82204988(ctx, base);
	// 82205A50: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205A54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82205A58: 396B42F8  addi r11, r11, 0x42f8
	ctx.r[11].s64 = ctx.r[11].s64 + 17144;
	// 82205A5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205A70: 4E800020  blr
	return;
	// 82205A74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205A90 size=44
    let mut pc: u32 = 0x82205A90;
    'dispatch: loop {
        match pc {
            0x82205A90 => {
    //   block [0x82205A90..0x82205ABC)
	// 82205A90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205A94: 419A0028  beq cr6, 0x82205abc
	if ctx.cr[6].eq {
		sub_82205ABC(ctx, base);
		return;
	}
	// 82205A98: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205A9C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82205AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82205AA4: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82205AA8: 396B4324  addi r11, r11, 0x4324
	ctx.r[11].s64 = ctx.r[11].s64 + 17188;
	// 82205AAC: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82205AB0: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82205AB4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205ABC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82205ABC size=8
    let mut pc: u32 = 0x82205ABC;
    'dispatch: loop {
        match pc {
            0x82205ABC => {
    //   block [0x82205ABC..0x82205AC4)
	// 82205ABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82205AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82205AC8 size=356
    let mut pc: u32 = 0x82205AC8;
    'dispatch: loop {
        match pc {
            0x82205AC8 => {
    //   block [0x82205AC8..0x82205C2C)
	// 82205AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82205AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205AE0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82205AE4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205AE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205AEC: 419A0048  beq cr6, 0x82205b34
	if ctx.cr[6].eq {
	pc = 0x82205B34; continue 'dispatch;
	}
	// 82205AF0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205AF4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205AF8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82205AFC: 409A0038  bne cr6, 0x82205b34
	if !ctx.cr[6].eq {
	pc = 0x82205B34; continue 'dispatch;
	}
	// 82205B00: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205B04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205B08: 419A002C  beq cr6, 0x82205b34
	if ctx.cr[6].eq {
	pc = 0x82205B34; continue 'dispatch;
	}
	// 82205B0C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205B10: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205B14: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82205B18: 409A001C  bne cr6, 0x82205b34
	if !ctx.cr[6].eq {
	pc = 0x82205B34; continue 'dispatch;
	}
	// 82205B1C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82205B20: A12B0014  lhz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205B24: 9BCB0025  stb r30, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[30].u8 ) };
	// 82205B28: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82205B2C: 612A8000  ori r10, r9, 0x8000
	ctx.r[10].u64 = ctx.r[9].u64 | 32768;
	// 82205B30: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82205B34: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205B38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82205B3C: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82205B40: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82205B44: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82205B48: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 82205B4C: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82205B50: 48164941  bl 0x8236a490
	ctx.lr = 0x82205B54;
	sub_8236A490(ctx, base);
	// 82205B54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205B58: 419A000C  beq cr6, 0x82205b64
	if ctx.cr[6].eq {
	pc = 0x82205B64; continue 'dispatch;
	}
	// 82205B5C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205B60: 48000008  b 0x82205b68
	pc = 0x82205B68; continue 'dispatch;
	// 82205B64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82205B68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205B6C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82205B70: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82205B74: 419A00A0  beq cr6, 0x82205c14
	if ctx.cr[6].eq {
	pc = 0x82205C14; continue 'dispatch;
	}
	// 82205B78: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82205B7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82205B80: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82205B84: 3929D70C  addi r9, r9, -0x28f4
	ctx.r[9].s64 = ctx.r[9].s64 + -10484;
	// 82205B88: 9BC3001C  stb r30, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 82205B8C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82205B90: 812B0058  lwz r9, 0x58(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82205B94: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82205B98: 91430088  stw r10, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82205B9C: 91230080  stw r9, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 82205BA0: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82205BA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205BA8: 419A0040  beq cr6, 0x82205be8
	if ctx.cr[6].eq {
	pc = 0x82205BE8; continue 'dispatch;
	}
	// 82205BAC: 39230090  addi r9, r3, 0x90
	ctx.r[9].s64 = ctx.r[3].s64 + 144;
	// 82205BB0: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82205BB4: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82205BB8: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82205BBC: 7DA04FAE  stfiwx f13, 0, r9
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82205BC0: C16B0004  lfs f11, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82205BC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82205BC8: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82205BCC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82205BD0: C18B22C8  lfs f12, 0x22c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8904 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82205BD4: C1AABA38  lfs f13, -0x45c8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82205BD8: ED8B0332  fmuls f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 82205BDC: D1A300AC  stfs f13, 0xac(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82205BE0: D18300A4  stfs f12, 0xa4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82205BE4: 48000028  b 0x82205c0c
	pc = 0x82205C0C; continue 'dispatch;
	// 82205BE8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82205BEC: 93C30090  stw r30, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82205BF0: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82205BF4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82205BF8: C1AB2198  lfs f13, 0x2198(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8600 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82205BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82205C00: D1A300A4  stfs f13, 0xa4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82205C04: C18BBA38  lfs f12, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82205C08: D18300AC  stfs f12, 0xac(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82205C0C: D00300A8  stfs f0, 0xa8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82205C10: D00300A0  stfs f0, 0xa0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82205C14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82205C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205C20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82205C24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205C30 size=220
    let mut pc: u32 = 0x82205C30;
    'dispatch: loop {
        match pc {
            0x82205C30 => {
    //   block [0x82205C30..0x82205D0C)
	// 82205C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205C38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205C3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205C40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205C44: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205C48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205C4C: 419A004C  beq cr6, 0x82205c98
	if ctx.cr[6].eq {
	pc = 0x82205C98; continue 'dispatch;
	}
	// 82205C50: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205C54: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205C58: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82205C5C: 409A003C  bne cr6, 0x82205c98
	if !ctx.cr[6].eq {
	pc = 0x82205C98; continue 'dispatch;
	}
	// 82205C60: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205C64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205C68: 419A0030  beq cr6, 0x82205c98
	if ctx.cr[6].eq {
	pc = 0x82205C98; continue 'dispatch;
	}
	// 82205C6C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205C70: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205C74: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82205C78: 409A0020  bne cr6, 0x82205c98
	if !ctx.cr[6].eq {
	pc = 0x82205C98; continue 'dispatch;
	}
	// 82205C7C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82205C80: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205C84: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82205C88: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82205C8C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82205C90: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82205C94: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82205C98: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82205CA0: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82205CA4: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82205CA8: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82205CAC: 38600088  li r3, 0x88
	ctx.r[3].s64 = 136;
	// 82205CB0: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82205CB4: 481647DD  bl 0x8236a490
	ctx.lr = 0x82205CB8;
	sub_8236A490(ctx, base);
	// 82205CB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205CBC: 419A000C  beq cr6, 0x82205cc8
	if ctx.cr[6].eq {
	pc = 0x82205CC8; continue 'dispatch;
	}
	// 82205CC0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205CC4: 48000008  b 0x82205ccc
	pc = 0x82205CCC; continue 'dispatch;
	// 82205CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82205CCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205CD0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82205CD4: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82205CD8: 419A0020  beq cr6, 0x82205cf8
	if ctx.cr[6].eq {
	pc = 0x82205CF8; continue 'dispatch;
	}
	// 82205CDC: 3880004E  li r4, 0x4e
	ctx.r[4].s64 = 78;
	// 82205CE0: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82205CE4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82205CE8: 480DFF69  bl 0x822e5c50
	ctx.lr = 0x82205CEC;
	sub_822E5C50(ctx, base);
	// 82205CEC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205CF0: 396B3D58  addi r11, r11, 0x3d58
	ctx.r[11].s64 = ctx.r[11].s64 + 15704;
	// 82205CF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205D04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205D10 size=272
    let mut pc: u32 = 0x82205D10;
    'dispatch: loop {
        match pc {
            0x82205D10 => {
    //   block [0x82205D10..0x82205E20)
	// 82205D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205D18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82205D1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205D20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205D24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205D28: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82205D2C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205D30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205D34: 419A0048  beq cr6, 0x82205d7c
	if ctx.cr[6].eq {
	pc = 0x82205D7C; continue 'dispatch;
	}
	// 82205D38: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205D3C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205D40: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82205D44: 409A0038  bne cr6, 0x82205d7c
	if !ctx.cr[6].eq {
	pc = 0x82205D7C; continue 'dispatch;
	}
	// 82205D48: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205D4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205D50: 419A002C  beq cr6, 0x82205d7c
	if ctx.cr[6].eq {
	pc = 0x82205D7C; continue 'dispatch;
	}
	// 82205D54: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205D58: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205D5C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82205D60: 409A001C  bne cr6, 0x82205d7c
	if !ctx.cr[6].eq {
	pc = 0x82205D7C; continue 'dispatch;
	}
	// 82205D64: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82205D68: A12B0014  lhz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205D6C: 9BCB0025  stb r30, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[30].u8 ) };
	// 82205D70: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82205D74: 612A8000  ori r10, r9, 0x8000
	ctx.r[10].u64 = ctx.r[9].u64 | 32768;
	// 82205D78: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82205D7C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205D80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82205D84: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82205D88: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82205D8C: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82205D90: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 82205D94: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82205D98: 481646F9  bl 0x8236a490
	ctx.lr = 0x82205D9C;
	sub_8236A490(ctx, base);
	// 82205D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205DA0: 419A000C  beq cr6, 0x82205dac
	if ctx.cr[6].eq {
	pc = 0x82205DAC; continue 'dispatch;
	}
	// 82205DA4: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205DA8: 48000008  b 0x82205db0
	pc = 0x82205DB0; continue 'dispatch;
	// 82205DAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82205DB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205DB4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82205DB8: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82205DBC: 419A004C  beq cr6, 0x82205e08
	if ctx.cr[6].eq {
	pc = 0x82205E08; continue 'dispatch;
	}
	// 82205DC0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82205DC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82205DC8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82205DCC: 3929D778  addi r9, r9, -0x2888
	ctx.r[9].s64 = ctx.r[9].s64 + -10376;
	// 82205DD0: 9BC3001C  stb r30, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 82205DD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82205DD8: 892B0006  lbz r9, 6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82205DDC: 9123008C  stw r9, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[9].u32 ) };
	// 82205DE0: 812B0058  lwz r9, 0x58(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82205DE4: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82205DE8: 91230080  stw r9, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 82205DEC: 812B0058  lwz r9, 0x58(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82205DF0: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82205DF4: 91230084  stw r9, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82205DF8: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82205DFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82205E00: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82205E04: 91430090  stw r10, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 82205E08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82205E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205E14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82205E18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205E20 size=220
    let mut pc: u32 = 0x82205E20;
    'dispatch: loop {
        match pc {
            0x82205E20 => {
    //   block [0x82205E20..0x82205EFC)
	// 82205E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205E28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205E2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205E30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205E34: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205E38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205E3C: 419A004C  beq cr6, 0x82205e88
	if ctx.cr[6].eq {
	pc = 0x82205E88; continue 'dispatch;
	}
	// 82205E40: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205E44: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205E48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82205E4C: 409A003C  bne cr6, 0x82205e88
	if !ctx.cr[6].eq {
	pc = 0x82205E88; continue 'dispatch;
	}
	// 82205E50: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205E54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205E58: 419A0030  beq cr6, 0x82205e88
	if ctx.cr[6].eq {
	pc = 0x82205E88; continue 'dispatch;
	}
	// 82205E5C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205E60: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205E64: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82205E68: 409A0020  bne cr6, 0x82205e88
	if !ctx.cr[6].eq {
	pc = 0x82205E88; continue 'dispatch;
	}
	// 82205E6C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82205E70: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205E74: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82205E78: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82205E7C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82205E80: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82205E84: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82205E88: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82205E90: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82205E94: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82205E98: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82205E9C: 38600088  li r3, 0x88
	ctx.r[3].s64 = 136;
	// 82205EA0: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82205EA4: 481645ED  bl 0x8236a490
	ctx.lr = 0x82205EA8;
	sub_8236A490(ctx, base);
	// 82205EA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205EAC: 419A000C  beq cr6, 0x82205eb8
	if ctx.cr[6].eq {
	pc = 0x82205EB8; continue 'dispatch;
	}
	// 82205EB0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205EB4: 48000008  b 0x82205ebc
	pc = 0x82205EBC; continue 'dispatch;
	// 82205EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82205EBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205EC0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82205EC4: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82205EC8: 419A0020  beq cr6, 0x82205ee8
	if ctx.cr[6].eq {
	pc = 0x82205EE8; continue 'dispatch;
	}
	// 82205ECC: 3880004B  li r4, 0x4b
	ctx.r[4].s64 = 75;
	// 82205ED0: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82205ED4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82205ED8: 480DFD79  bl 0x822e5c50
	ctx.lr = 0x82205EDC;
	sub_822E5C50(ctx, base);
	// 82205EDC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205EE0: 396B3D58  addi r11, r11, 0x3d58
	ctx.r[11].s64 = ctx.r[11].s64 + 15704;
	// 82205EE4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205EF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205F00 size=220
    let mut pc: u32 = 0x82205F00;
    'dispatch: loop {
        match pc {
            0x82205F00 => {
    //   block [0x82205F00..0x82205FDC)
	// 82205F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205F08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205F0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205F14: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205F18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205F1C: 419A004C  beq cr6, 0x82205f68
	if ctx.cr[6].eq {
	pc = 0x82205F68; continue 'dispatch;
	}
	// 82205F20: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205F24: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205F28: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82205F2C: 409A003C  bne cr6, 0x82205f68
	if !ctx.cr[6].eq {
	pc = 0x82205F68; continue 'dispatch;
	}
	// 82205F30: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205F34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205F38: 419A0030  beq cr6, 0x82205f68
	if ctx.cr[6].eq {
	pc = 0x82205F68; continue 'dispatch;
	}
	// 82205F3C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205F40: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82205F44: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82205F48: 409A0020  bne cr6, 0x82205f68
	if !ctx.cr[6].eq {
	pc = 0x82205F68; continue 'dispatch;
	}
	// 82205F4C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82205F50: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205F54: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82205F58: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82205F5C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82205F60: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82205F64: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82205F68: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82205F70: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82205F74: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82205F78: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82205F7C: 38600088  li r3, 0x88
	ctx.r[3].s64 = 136;
	// 82205F80: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82205F84: 4816450D  bl 0x8236a490
	ctx.lr = 0x82205F88;
	sub_8236A490(ctx, base);
	// 82205F88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205F8C: 419A000C  beq cr6, 0x82205f98
	if ctx.cr[6].eq {
	pc = 0x82205F98; continue 'dispatch;
	}
	// 82205F90: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82205F94: 48000008  b 0x82205f9c
	pc = 0x82205F9C; continue 'dispatch;
	// 82205F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82205F9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82205FA0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82205FA4: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82205FA8: 419A0020  beq cr6, 0x82205fc8
	if ctx.cr[6].eq {
	pc = 0x82205FC8; continue 'dispatch;
	}
	// 82205FAC: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 82205FB0: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82205FB4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82205FB8: 480DFC99  bl 0x822e5c50
	ctx.lr = 0x82205FBC;
	sub_822E5C50(ctx, base);
	// 82205FBC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82205FC0: 396B3D58  addi r11, r11, 0x3d58
	ctx.r[11].s64 = ctx.r[11].s64 + 15704;
	// 82205FC4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82205FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82205FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82205FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82205FD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82205FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82205FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82205FE0 size=220
    let mut pc: u32 = 0x82205FE0;
    'dispatch: loop {
        match pc {
            0x82205FE0 => {
    //   block [0x82205FE0..0x822060BC)
	// 82205FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82205FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82205FE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82205FEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82205FF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82205FF4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82205FF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82205FFC: 419A004C  beq cr6, 0x82206048
	if ctx.cr[6].eq {
	pc = 0x82206048; continue 'dispatch;
	}
	// 82206000: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206004: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206008: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8220600C: 409A003C  bne cr6, 0x82206048
	if !ctx.cr[6].eq {
	pc = 0x82206048; continue 'dispatch;
	}
	// 82206010: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206014: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206018: 419A0030  beq cr6, 0x82206048
	if ctx.cr[6].eq {
	pc = 0x82206048; continue 'dispatch;
	}
	// 8220601C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206020: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206024: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206028: 409A0020  bne cr6, 0x82206048
	if !ctx.cr[6].eq {
	pc = 0x82206048; continue 'dispatch;
	}
	// 8220602C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206030: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206034: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206038: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 8220603C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206040: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206044: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206048: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220604C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206050: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206054: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206058: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 8220605C: 38600088  li r3, 0x88
	ctx.r[3].s64 = 136;
	// 82206060: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206064: 4816442D  bl 0x8236a490
	ctx.lr = 0x82206068;
	sub_8236A490(ctx, base);
	// 82206068: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220606C: 419A000C  beq cr6, 0x82206078
	if ctx.cr[6].eq {
	pc = 0x82206078; continue 'dispatch;
	}
	// 82206070: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206074: 48000008  b 0x8220607c
	pc = 0x8220607C; continue 'dispatch;
	// 82206078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8220607C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206080: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206084: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206088: 419A0020  beq cr6, 0x822060a8
	if ctx.cr[6].eq {
	pc = 0x822060A8; continue 'dispatch;
	}
	// 8220608C: 3880004D  li r4, 0x4d
	ctx.r[4].s64 = 77;
	// 82206090: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206094: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206098: 480DFBB9  bl 0x822e5c50
	ctx.lr = 0x8220609C;
	sub_822E5C50(ctx, base);
	// 8220609C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822060A0: 396B3D58  addi r11, r11, 0x3d58
	ctx.r[11].s64 = ctx.r[11].s64 + 15704;
	// 822060A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822060A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822060AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822060B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822060B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822060B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822060C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822060C0 size=244
    let mut pc: u32 = 0x822060C0;
    'dispatch: loop {
        match pc {
            0x822060C0 => {
    //   block [0x822060C0..0x822061B4)
	// 822060C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822060C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822060C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822060CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822060D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822060D4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822060D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822060DC: 419A004C  beq cr6, 0x82206128
	if ctx.cr[6].eq {
	pc = 0x82206128; continue 'dispatch;
	}
	// 822060E0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822060E4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822060E8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822060EC: 409A003C  bne cr6, 0x82206128
	if !ctx.cr[6].eq {
	pc = 0x82206128; continue 'dispatch;
	}
	// 822060F0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822060F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822060F8: 419A0030  beq cr6, 0x82206128
	if ctx.cr[6].eq {
	pc = 0x82206128; continue 'dispatch;
	}
	// 822060FC: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206100: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206104: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206108: 409A0020  bne cr6, 0x82206128
	if !ctx.cr[6].eq {
	pc = 0x82206128; continue 'dispatch;
	}
	// 8220610C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206110: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206114: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206118: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 8220611C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206120: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206124: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206128: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220612C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206130: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206134: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206138: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 8220613C: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82206140: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206144: 4816434D  bl 0x8236a490
	ctx.lr = 0x82206148;
	sub_8236A490(ctx, base);
	// 82206148: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220614C: 419A000C  beq cr6, 0x82206158
	if ctx.cr[6].eq {
	pc = 0x82206158; continue 'dispatch;
	}
	// 82206150: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206154: 48000008  b 0x8220615c
	pc = 0x8220615C; continue 'dispatch;
	// 82206158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8220615C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206160: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206164: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206168: 419A0038  beq cr6, 0x822061a0
	if ctx.cr[6].eq {
	pc = 0x822061A0; continue 'dispatch;
	}
	// 8220616C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82206170: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82206174: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82206178: 38DF0020  addi r6, r31, 0x20
	ctx.r[6].s64 = ctx.r[31].s64 + 32;
	// 8220617C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206180: 3929C6C8  addi r9, r9, -0x3938
	ctx.r[9].s64 = ctx.r[9].s64 + -14648;
	// 82206184: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206188: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8220618C: 90C3008C  stw r6, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82206190: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82206194: 91430088  stw r10, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82206198: 91030084  stw r8, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[8].u32 ) };
	// 8220619C: 90E30080  stw r7, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[7].u32 ) };
	// 822061A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822061A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822061A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822061AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822061B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822061B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822061B8 size=20
    let mut pc: u32 = 0x822061B8;
    'dispatch: loop {
        match pc {
            0x822061B8 => {
    //   block [0x822061B8..0x822061CC)
	// 822061B8: 80A3001C  lwz r5, 0x1c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 822061BC: 38C30020  addi r6, r3, 0x20
	ctx.r[6].s64 = ctx.r[3].s64 + 32;
	// 822061C0: 8083000C  lwz r4, 0xc(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822061C4: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822061C8: 480EB218  b 0x822f13e0
	sub_822F13E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822061D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822061D0 size=236
    let mut pc: u32 = 0x822061D0;
    'dispatch: loop {
        match pc {
            0x822061D0 => {
    //   block [0x822061D0..0x822062BC)
	// 822061D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822061D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822061D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822061DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822061E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822061E4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822061E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822061EC: 419A004C  beq cr6, 0x82206238
	if ctx.cr[6].eq {
	pc = 0x82206238; continue 'dispatch;
	}
	// 822061F0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822061F4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822061F8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822061FC: 409A003C  bne cr6, 0x82206238
	if !ctx.cr[6].eq {
	pc = 0x82206238; continue 'dispatch;
	}
	// 82206200: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206204: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206208: 419A0030  beq cr6, 0x82206238
	if ctx.cr[6].eq {
	pc = 0x82206238; continue 'dispatch;
	}
	// 8220620C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206210: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206214: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206218: 409A0020  bne cr6, 0x82206238
	if !ctx.cr[6].eq {
	pc = 0x82206238; continue 'dispatch;
	}
	// 8220621C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206220: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206224: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206228: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 8220622C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206230: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206234: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206238: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220623C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206240: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206244: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206248: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 8220624C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82206250: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206254: 4816423D  bl 0x8236a490
	ctx.lr = 0x82206258;
	sub_8236A490(ctx, base);
	// 82206258: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220625C: 419A000C  beq cr6, 0x82206268
	if ctx.cr[6].eq {
	pc = 0x82206268; continue 'dispatch;
	}
	// 82206260: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206264: 48000008  b 0x8220626c
	pc = 0x8220626C; continue 'dispatch;
	// 82206268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8220626C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206270: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206274: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206278: 419A0030  beq cr6, 0x822062a8
	if ctx.cr[6].eq {
	pc = 0x822062A8; continue 'dispatch;
	}
	// 8220627C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82206280: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82206284: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206288: 38FF0020  addi r7, r31, 0x20
	ctx.r[7].s64 = ctx.r[31].s64 + 32;
	// 8220628C: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206290: 3929C73C  addi r9, r9, -0x38c4
	ctx.r[9].s64 = ctx.r[9].s64 + -14532;
	// 82206294: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82206298: 90E3008C  stw r7, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[7].u32 ) };
	// 8220629C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822062A0: 91430084  stw r10, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 822062A4: 91030080  stw r8, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 822062A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822062AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822062B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822062B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822062B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822062C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822062C0 size=20
    let mut pc: u32 = 0x822062C0;
    'dispatch: loop {
        match pc {
            0x822062C0 => {
    //   block [0x822062C0..0x822062D4)
	// 822062C0: 80A3001C  lwz r5, 0x1c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 822062C4: 38C30020  addi r6, r3, 0x20
	ctx.r[6].s64 = ctx.r[3].s64 + 32;
	// 822062C8: 8083000C  lwz r4, 0xc(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822062CC: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822062D0: 480EF778  b 0x822f5a48
	sub_822F5A48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822062D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822062D8 size=220
    let mut pc: u32 = 0x822062D8;
    'dispatch: loop {
        match pc {
            0x822062D8 => {
    //   block [0x822062D8..0x822063B4)
	// 822062D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822062DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822062E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822062E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822062E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822062EC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822062F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822062F4: 419A004C  beq cr6, 0x82206340
	if ctx.cr[6].eq {
	pc = 0x82206340; continue 'dispatch;
	}
	// 822062F8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822062FC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206300: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82206304: 409A003C  bne cr6, 0x82206340
	if !ctx.cr[6].eq {
	pc = 0x82206340; continue 'dispatch;
	}
	// 82206308: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220630C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206310: 419A0030  beq cr6, 0x82206340
	if ctx.cr[6].eq {
	pc = 0x82206340; continue 'dispatch;
	}
	// 82206314: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206318: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220631C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206320: 409A0020  bne cr6, 0x82206340
	if !ctx.cr[6].eq {
	pc = 0x82206340; continue 'dispatch;
	}
	// 82206324: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206328: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220632C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206330: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82206334: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206338: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 8220633C: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206340: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206348: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 8220634C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206350: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82206354: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 82206358: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8220635C: 48164135  bl 0x8236a490
	ctx.lr = 0x82206360;
	sub_8236A490(ctx, base);
	// 82206360: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206364: 419A000C  beq cr6, 0x82206370
	if ctx.cr[6].eq {
	pc = 0x82206370; continue 'dispatch;
	}
	// 82206368: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220636C: 48000008  b 0x82206374
	pc = 0x82206374; continue 'dispatch;
	// 82206370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82206374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206378: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8220637C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206380: 419A0020  beq cr6, 0x822063a0
	if ctx.cr[6].eq {
	pc = 0x822063A0; continue 'dispatch;
	}
	// 82206384: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 82206388: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8220638C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206390: 39290B18  addi r9, r9, 0xb18
	ctx.r[9].s64 = ctx.r[9].s64 + 2840;
	// 82206394: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82206398: 91430080  stw r10, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8220639C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822063A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822063A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822063A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822063AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822063B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822063B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822063B8 size=232
    let mut pc: u32 = 0x822063B8;
    'dispatch: loop {
        match pc {
            0x822063B8 => {
    //   block [0x822063B8..0x822064A0)
	// 822063B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822063BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822063C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822063C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822063C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822063CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822063D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822063D4: 419A004C  beq cr6, 0x82206420
	if ctx.cr[6].eq {
	pc = 0x82206420; continue 'dispatch;
	}
	// 822063D8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822063DC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822063E0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822063E4: 409A003C  bne cr6, 0x82206420
	if !ctx.cr[6].eq {
	pc = 0x82206420; continue 'dispatch;
	}
	// 822063E8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822063EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822063F0: 419A0030  beq cr6, 0x82206420
	if ctx.cr[6].eq {
	pc = 0x82206420; continue 'dispatch;
	}
	// 822063F4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822063F8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822063FC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206400: 409A0020  bne cr6, 0x82206420
	if !ctx.cr[6].eq {
	pc = 0x82206420; continue 'dispatch;
	}
	// 82206404: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206408: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220640C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206410: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82206414: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206418: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 8220641C: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206420: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206428: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 8220642C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206430: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82206434: 38600088  li r3, 0x88
	ctx.r[3].s64 = 136;
	// 82206438: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8220643C: 48164055  bl 0x8236a490
	ctx.lr = 0x82206440;
	sub_8236A490(ctx, base);
	// 82206440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206444: 419A000C  beq cr6, 0x82206450
	if ctx.cr[6].eq {
	pc = 0x82206450; continue 'dispatch;
	}
	// 82206448: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220644C: 48000008  b 0x82206454
	pc = 0x82206454; continue 'dispatch;
	// 82206450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82206454: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206458: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8220645C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206460: 419A002C  beq cr6, 0x8220648c
	if ctx.cr[6].eq {
	pc = 0x8220648C; continue 'dispatch;
	}
	// 82206464: A1030014  lhz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206468: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8220646C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206470: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206474: 3929B910  addi r9, r9, -0x46f0
	ctx.r[9].s64 = ctx.r[9].s64 + -18160;
	// 82206478: 61088000  ori r8, r8, 0x8000
	ctx.r[8].u64 = ctx.r[8].u64 | 32768;
	// 8220647C: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82206480: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82206484: B1030014  sth r8, 0x14(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u16 ) };
	// 82206488: 91430080  stw r10, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8220648C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82206490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8220649C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822064A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822064A0 size=252
    let mut pc: u32 = 0x822064A0;
    'dispatch: loop {
        match pc {
            0x822064A0 => {
    //   block [0x822064A0..0x8220659C)
	// 822064A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822064A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822064A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822064AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822064B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822064B4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822064B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822064BC: 419A004C  beq cr6, 0x82206508
	if ctx.cr[6].eq {
	pc = 0x82206508; continue 'dispatch;
	}
	// 822064C0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822064C4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822064C8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822064CC: 409A003C  bne cr6, 0x82206508
	if !ctx.cr[6].eq {
	pc = 0x82206508; continue 'dispatch;
	}
	// 822064D0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822064D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822064D8: 419A0030  beq cr6, 0x82206508
	if ctx.cr[6].eq {
	pc = 0x82206508; continue 'dispatch;
	}
	// 822064DC: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822064E0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822064E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822064E8: 409A0020  bne cr6, 0x82206508
	if !ctx.cr[6].eq {
	pc = 0x82206508; continue 'dispatch;
	}
	// 822064EC: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 822064F0: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822064F4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822064F8: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 822064FC: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206500: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206504: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206508: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220650C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206510: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206514: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206518: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 8220651C: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 82206520: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206524: 48163F6D  bl 0x8236a490
	ctx.lr = 0x82206528;
	sub_8236A490(ctx, base);
	// 82206528: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8220652C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206530: 419A000C  beq cr6, 0x8220653c
	if ctx.cr[6].eq {
	pc = 0x8220653C; continue 'dispatch;
	}
	// 82206534: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206538: 48000008  b 0x82206540
	pc = 0x82206540; continue 'dispatch;
	// 8220653C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82206540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206544: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206548: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8220654C: 419A003C  beq cr6, 0x82206588
	if ctx.cr[6].eq {
	pc = 0x82206588; continue 'dispatch;
	}
	// 82206550: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82206554: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82206558: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 8220655C: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82206560: 3908C8C4  addi r8, r8, -0x373c
	ctx.r[8].s64 = ctx.r[8].s64 + -14140;
	// 82206564: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206568: 38E70D0C  addi r7, r7, 0xd0c
	ctx.r[7].s64 = ctx.r[7].s64 + 3340;
	// 8220656C: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82206570: F8C30090  std r6, 0x90(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[6].u64 ) };
	// 82206574: F8C300C0  std r6, 0xc0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), ctx.r[6].u64 ) };
	// 82206578: 90E30080  stw r7, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[7].u32 ) };
	// 8220657C: 9163013C  stw r11, 0x13c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 82206580: 91430138  stw r10, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[10].u32 ) };
	// 82206584: 91230134  stw r9, 0x134(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(308 as u32), ctx.r[9].u32 ) };
	// 82206588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220658C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82206598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822065A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822065A0 size=212
    let mut pc: u32 = 0x822065A0;
    'dispatch: loop {
        match pc {
            0x822065A0 => {
    //   block [0x822065A0..0x82206674)
	// 822065A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822065A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822065A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822065AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822065B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822065B4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822065B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822065BC: 419A004C  beq cr6, 0x82206608
	if ctx.cr[6].eq {
	pc = 0x82206608; continue 'dispatch;
	}
	// 822065C0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822065C4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822065C8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822065CC: 409A003C  bne cr6, 0x82206608
	if !ctx.cr[6].eq {
	pc = 0x82206608; continue 'dispatch;
	}
	// 822065D0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822065D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822065D8: 419A0030  beq cr6, 0x82206608
	if ctx.cr[6].eq {
	pc = 0x82206608; continue 'dispatch;
	}
	// 822065DC: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822065E0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822065E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822065E8: 409A0020  bne cr6, 0x82206608
	if !ctx.cr[6].eq {
	pc = 0x82206608; continue 'dispatch;
	}
	// 822065EC: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 822065F0: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822065F4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822065F8: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 822065FC: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206600: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206604: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206608: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220660C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206610: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206614: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206618: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 8220661C: 38600088  li r3, 0x88
	ctx.r[3].s64 = 136;
	// 82206620: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206624: 48163E6D  bl 0x8236a490
	ctx.lr = 0x82206628;
	sub_8236A490(ctx, base);
	// 82206628: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220662C: 419A000C  beq cr6, 0x82206638
	if ctx.cr[6].eq {
	pc = 0x82206638; continue 'dispatch;
	}
	// 82206630: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206634: 48000008  b 0x8220663c
	pc = 0x8220663C; continue 'dispatch;
	// 82206638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8220663C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206640: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206644: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206648: 419A0018  beq cr6, 0x82206660
	if ctx.cr[6].eq {
	pc = 0x82206660; continue 'dispatch;
	}
	// 8220664C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82206650: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206654: 394ADA08  addi r10, r10, -0x25f8
	ctx.r[10].s64 = ctx.r[10].s64 + -9720;
	// 82206658: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8220665C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82206660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82206664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220666C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82206670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82206678 size=212
    let mut pc: u32 = 0x82206678;
    'dispatch: loop {
        match pc {
            0x82206678 => {
    //   block [0x82206678..0x8220674C)
	// 82206678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220667C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82206680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82206684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220668C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206694: 419A004C  beq cr6, 0x822066e0
	if ctx.cr[6].eq {
	pc = 0x822066E0; continue 'dispatch;
	}
	// 82206698: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220669C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822066A0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822066A4: 409A003C  bne cr6, 0x822066e0
	if !ctx.cr[6].eq {
	pc = 0x822066E0; continue 'dispatch;
	}
	// 822066A8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822066AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822066B0: 419A0030  beq cr6, 0x822066e0
	if ctx.cr[6].eq {
	pc = 0x822066E0; continue 'dispatch;
	}
	// 822066B4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822066B8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822066BC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822066C0: 409A0020  bne cr6, 0x822066e0
	if !ctx.cr[6].eq {
	pc = 0x822066E0; continue 'dispatch;
	}
	// 822066C4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 822066C8: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822066CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822066D0: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 822066D4: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 822066D8: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 822066DC: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 822066E0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822066E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822066E8: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 822066EC: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 822066F0: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 822066F4: 38600610  li r3, 0x610
	ctx.r[3].s64 = 1552;
	// 822066F8: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 822066FC: 48163D95  bl 0x8236a490
	ctx.lr = 0x82206700;
	sub_8236A490(ctx, base);
	// 82206700: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206704: 419A000C  beq cr6, 0x82206710
	if ctx.cr[6].eq {
	pc = 0x82206710; continue 'dispatch;
	}
	// 82206708: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220670C: 48000008  b 0x82206714
	pc = 0x82206714; continue 'dispatch;
	// 82206710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82206714: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206718: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8220671C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206720: 419A0018  beq cr6, 0x82206738
	if ctx.cr[6].eq {
	pc = 0x82206738; continue 'dispatch;
	}
	// 82206724: 38FF0020  addi r7, r31, 0x20
	ctx.r[7].s64 = ctx.r[31].s64 + 32;
	// 82206728: 80DF001C  lwz r6, 0x1c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8220672C: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206730: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206734: 481242ED  bl 0x8232aa20
	ctx.lr = 0x82206738;
	sub_8232AA20(ctx, base);
	// 82206738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220673C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206744: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82206748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82206750 size=240
    let mut pc: u32 = 0x82206750;
    'dispatch: loop {
        match pc {
            0x82206750 => {
    //   block [0x82206750..0x82206840)
	// 82206750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82206758: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8220675C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206760: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206764: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206768: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8220676C: 419A004C  beq cr6, 0x822067b8
	if ctx.cr[6].eq {
	pc = 0x822067B8; continue 'dispatch;
	}
	// 82206770: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206774: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206778: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8220677C: 409A003C  bne cr6, 0x822067b8
	if !ctx.cr[6].eq {
	pc = 0x822067B8; continue 'dispatch;
	}
	// 82206780: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206788: 419A0030  beq cr6, 0x822067b8
	if ctx.cr[6].eq {
	pc = 0x822067B8; continue 'dispatch;
	}
	// 8220678C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206790: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206794: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206798: 409A0020  bne cr6, 0x822067b8
	if !ctx.cr[6].eq {
	pc = 0x822067B8; continue 'dispatch;
	}
	// 8220679C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 822067A0: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822067A4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822067A8: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 822067AC: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 822067B0: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 822067B4: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 822067B8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822067BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822067C0: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 822067C4: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 822067C8: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 822067CC: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 822067D0: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 822067D4: 48163CBD  bl 0x8236a490
	ctx.lr = 0x822067D8;
	sub_8236A490(ctx, base);
	// 822067D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822067DC: 419A000C  beq cr6, 0x822067e8
	if ctx.cr[6].eq {
	pc = 0x822067E8; continue 'dispatch;
	}
	// 822067E0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 822067E4: 48000008  b 0x822067ec
	pc = 0x822067EC; continue 'dispatch;
	// 822067E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822067EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822067F0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822067F4: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 822067F8: 419A0034  beq cr6, 0x8220682c
	if ctx.cr[6].eq {
	pc = 0x8220682C; continue 'dispatch;
	}
	// 822067FC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82206800: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206804: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82206808: 3929D9BC  addi r9, r9, -0x2644
	ctx.r[9].s64 = ctx.r[9].s64 + -9796;
	// 8220680C: 390A0040  addi r8, r10, 0x40
	ctx.r[8].s64 = ctx.r[10].s64 + 64;
	// 82206810: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82206814: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206818: 91230080  stw r9, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 8220681C: A16B000A  lhz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82206820: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82206824: 91430088  stw r10, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82206828: 9103008C  stw r8, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[8].u32 ) };
	// 8220682C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82206830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8220683C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82206840 size=204
    let mut pc: u32 = 0x82206840;
    'dispatch: loop {
        match pc {
            0x82206840 => {
    //   block [0x82206840..0x8220690C)
	// 82206840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82206848: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8220684C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206850: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206854: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206858: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8220685C: 419A004C  beq cr6, 0x822068a8
	if ctx.cr[6].eq {
	pc = 0x822068A8; continue 'dispatch;
	}
	// 82206860: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206864: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206868: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8220686C: 409A003C  bne cr6, 0x822068a8
	if !ctx.cr[6].eq {
	pc = 0x822068A8; continue 'dispatch;
	}
	// 82206870: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206874: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206878: 419A0030  beq cr6, 0x822068a8
	if ctx.cr[6].eq {
	pc = 0x822068A8; continue 'dispatch;
	}
	// 8220687C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206880: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206884: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206888: 409A0020  bne cr6, 0x822068a8
	if !ctx.cr[6].eq {
	pc = 0x822068A8; continue 'dispatch;
	}
	// 8220688C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206890: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206894: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206898: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 8220689C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 822068A0: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 822068A4: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 822068A8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822068AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822068B0: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 822068B4: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 822068B8: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 822068BC: 386016C0  li r3, 0x16c0
	ctx.r[3].s64 = 5824;
	// 822068C0: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 822068C4: 48163BCD  bl 0x8236a490
	ctx.lr = 0x822068C8;
	sub_8236A490(ctx, base);
	// 822068C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822068CC: 419A000C  beq cr6, 0x822068d8
	if ctx.cr[6].eq {
	pc = 0x822068D8; continue 'dispatch;
	}
	// 822068D0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 822068D4: 48000008  b 0x822068dc
	pc = 0x822068DC; continue 'dispatch;
	// 822068D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822068DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822068E0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822068E4: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 822068E8: 419A0010  beq cr6, 0x822068f8
	if ctx.cr[6].eq {
	pc = 0x822068F8; continue 'dispatch;
	}
	// 822068EC: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822068F0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822068F4: 48120F15  bl 0x82327808
	ctx.lr = 0x822068F8;
	sub_82327808(ctx, base);
	// 822068F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822068FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82206908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82206910 size=216
    let mut pc: u32 = 0x82206910;
    'dispatch: loop {
        match pc {
            0x82206910 => {
    //   block [0x82206910..0x822069E8)
	// 82206910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82206918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8220691C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206924: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8220692C: 419A004C  beq cr6, 0x82206978
	if ctx.cr[6].eq {
	pc = 0x82206978; continue 'dispatch;
	}
	// 82206930: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206934: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206938: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8220693C: 409A003C  bne cr6, 0x82206978
	if !ctx.cr[6].eq {
	pc = 0x82206978; continue 'dispatch;
	}
	// 82206940: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206948: 419A0030  beq cr6, 0x82206978
	if ctx.cr[6].eq {
	pc = 0x82206978; continue 'dispatch;
	}
	// 8220694C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206950: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206954: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206958: 409A0020  bne cr6, 0x82206978
	if !ctx.cr[6].eq {
	pc = 0x82206978; continue 'dispatch;
	}
	// 8220695C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206960: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206964: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206968: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 8220696C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206970: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206974: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206978: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8220697C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206980: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206984: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206988: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 8220698C: 38600580  li r3, 0x580
	ctx.r[3].s64 = 1408;
	// 82206990: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206994: 48163AFD  bl 0x8236a490
	ctx.lr = 0x82206998;
	sub_8236A490(ctx, base);
	// 82206998: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220699C: 419A000C  beq cr6, 0x822069a8
	if ctx.cr[6].eq {
	pc = 0x822069A8; continue 'dispatch;
	}
	// 822069A0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 822069A4: 48000008  b 0x822069ac
	pc = 0x822069AC; continue 'dispatch;
	// 822069A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822069AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822069B0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822069B4: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 822069B8: 419A001C  beq cr6, 0x822069d4
	if ctx.cr[6].eq {
	pc = 0x822069D4; continue 'dispatch;
	}
	// 822069BC: 391F0034  addi r8, r31, 0x34
	ctx.r[8].s64 = ctx.r[31].s64 + 52;
	// 822069C0: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822069C4: 38FF002C  addi r7, r31, 0x2c
	ctx.r[7].s64 = ctx.r[31].s64 + 44;
	// 822069C8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822069CC: 38DF001C  addi r6, r31, 0x1c
	ctx.r[6].s64 = ctx.r[31].s64 + 28;
	// 822069D0: 48122349  bl 0x82328d18
	ctx.lr = 0x822069D4;
	sub_82328D18(ctx, base);
	// 822069D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822069D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822069DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822069E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822069E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822069E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822069E8 size=216
    let mut pc: u32 = 0x822069E8;
    'dispatch: loop {
        match pc {
            0x822069E8 => {
    //   block [0x822069E8..0x82206AC0)
	// 822069E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822069EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822069F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822069F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822069F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822069FC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206A00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206A04: 419A004C  beq cr6, 0x82206a50
	if ctx.cr[6].eq {
	pc = 0x82206A50; continue 'dispatch;
	}
	// 82206A08: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206A0C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206A10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82206A14: 409A003C  bne cr6, 0x82206a50
	if !ctx.cr[6].eq {
	pc = 0x82206A50; continue 'dispatch;
	}
	// 82206A18: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206A1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206A20: 419A0030  beq cr6, 0x82206a50
	if ctx.cr[6].eq {
	pc = 0x82206A50; continue 'dispatch;
	}
	// 82206A24: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206A28: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206A2C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206A30: 409A0020  bne cr6, 0x82206a50
	if !ctx.cr[6].eq {
	pc = 0x82206A50; continue 'dispatch;
	}
	// 82206A34: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206A38: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206A3C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206A40: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82206A44: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206A48: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206A4C: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206A50: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206A58: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206A5C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206A60: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82206A64: 38600480  li r3, 0x480
	ctx.r[3].s64 = 1152;
	// 82206A68: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206A6C: 48163A25  bl 0x8236a490
	ctx.lr = 0x82206A70;
	sub_8236A490(ctx, base);
	// 82206A70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206A74: 419A000C  beq cr6, 0x82206a80
	if ctx.cr[6].eq {
	pc = 0x82206A80; continue 'dispatch;
	}
	// 82206A78: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206A7C: 48000008  b 0x82206a84
	pc = 0x82206A84; continue 'dispatch;
	// 82206A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82206A84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206A88: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206A8C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206A90: 419A001C  beq cr6, 0x82206aac
	if ctx.cr[6].eq {
	pc = 0x82206AAC; continue 'dispatch;
	}
	// 82206A94: 391F0034  addi r8, r31, 0x34
	ctx.r[8].s64 = ctx.r[31].s64 + 52;
	// 82206A98: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206A9C: 38FF002C  addi r7, r31, 0x2c
	ctx.r[7].s64 = ctx.r[31].s64 + 44;
	// 82206AA0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206AA4: 38DF001C  addi r6, r31, 0x1c
	ctx.r[6].s64 = ctx.r[31].s64 + 28;
	// 82206AA8: 4811E119  bl 0x82324bc0
	ctx.lr = 0x82206AAC;
	sub_82324BC0(ctx, base);
	// 82206AAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82206AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82206ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82206AC0 size=252
    let mut pc: u32 = 0x82206AC0;
    'dispatch: loop {
        match pc {
            0x82206AC0 => {
    //   block [0x82206AC0..0x82206BBC)
	// 82206AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82206AC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82206ACC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206AD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206AD4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206AD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206ADC: 419A004C  beq cr6, 0x82206b28
	if ctx.cr[6].eq {
	pc = 0x82206B28; continue 'dispatch;
	}
	// 82206AE0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206AE4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206AE8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82206AEC: 409A003C  bne cr6, 0x82206b28
	if !ctx.cr[6].eq {
	pc = 0x82206B28; continue 'dispatch;
	}
	// 82206AF0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206AF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206AF8: 419A0030  beq cr6, 0x82206b28
	if ctx.cr[6].eq {
	pc = 0x82206B28; continue 'dispatch;
	}
	// 82206AFC: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206B00: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206B04: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206B08: 409A0020  bne cr6, 0x82206b28
	if !ctx.cr[6].eq {
	pc = 0x82206B28; continue 'dispatch;
	}
	// 82206B0C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206B10: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206B14: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206B18: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82206B1C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206B20: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206B24: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206B28: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206B30: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206B34: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206B38: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82206B3C: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 82206B40: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206B44: 4816394D  bl 0x8236a490
	ctx.lr = 0x82206B48;
	sub_8236A490(ctx, base);
	// 82206B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82206B4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206B50: 419A000C  beq cr6, 0x82206b5c
	if ctx.cr[6].eq {
	pc = 0x82206B5C; continue 'dispatch;
	}
	// 82206B54: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206B58: 48000008  b 0x82206b60
	pc = 0x82206B60; continue 'dispatch;
	// 82206B5C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82206B60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206B64: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82206B68: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206B6C: 419A003C  beq cr6, 0x82206ba8
	if ctx.cr[6].eq {
	pc = 0x82206BA8; continue 'dispatch;
	}
	// 82206B70: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82206B74: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206B78: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206B7C: 3929D880  addi r9, r9, -0x2780
	ctx.r[9].s64 = ctx.r[9].s64 + -10112;
	// 82206B80: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82206B84: F9630080  std r11, 0x80(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u64 ) };
	// 82206B88: F9630088  std r11, 0x88(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u64 ) };
	// 82206B8C: F9630090  std r11, 0x90(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u64 ) };
	// 82206B90: F9630098  std r11, 0x98(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u64 ) };
	// 82206B94: F96300A0  std r11, 0xa0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u64 ) };
	// 82206B98: 816A0058  lwz r11, 0x58(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 82206B9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82206BA0: 910300AC  stw r8, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[8].u32 ) };
	// 82206BA4: 916300A8  stw r11, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82206BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82206BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206BB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82206BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82206BC0 size=332
    let mut pc: u32 = 0x82206BC0;
    'dispatch: loop {
        match pc {
            0x82206BC0 => {
    //   block [0x82206BC0..0x82206D0C)
	// 82206BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206BC4: 4832E4F5  bl 0x825350b8
	ctx.lr = 0x82206BC8;
	sub_82535080(ctx, base);
	// 82206BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206BCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206BD0: 3B800005  li r28, 5
	ctx.r[28].s64 = 5;
	// 82206BD4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82206BD8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206BDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206BE0: 419A0044  beq cr6, 0x82206c24
	if ctx.cr[6].eq {
	pc = 0x82206C24; continue 'dispatch;
	}
	// 82206BE4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206BE8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206BEC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82206BF0: 409A0034  bne cr6, 0x82206c24
	if !ctx.cr[6].eq {
	pc = 0x82206C24; continue 'dispatch;
	}
	// 82206BF4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206BF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206BFC: 419A0028  beq cr6, 0x82206c24
	if ctx.cr[6].eq {
	pc = 0x82206C24; continue 'dispatch;
	}
	// 82206C00: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206C04: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206C08: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206C0C: 409A0018  bne cr6, 0x82206c24
	if !ctx.cr[6].eq {
	pc = 0x82206C24; continue 'dispatch;
	}
	// 82206C10: A14B0014  lhz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206C14: 9B8B0019  stb r28, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[28].u8 ) };
	// 82206C18: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 82206C1C: 9BAB0025  stb r29, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[29].u8 ) };
	// 82206C20: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206C24: 3FC0830F  lis r30, -0x7cf1
	ctx.r[30].s64 = -2096168960;
	// 82206C28: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206C30: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206C34: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82206C38: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82206C3C: 809EFAC0  lwz r4, -0x540(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206C40: 48163851  bl 0x8236a490
	ctx.lr = 0x82206C44;
	sub_8236A490(ctx, base);
	// 82206C44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206C48: 419A000C  beq cr6, 0x82206c54
	if ctx.cr[6].eq {
	pc = 0x82206C54; continue 'dispatch;
	}
	// 82206C4C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206C50: 48000008  b 0x82206c58
	pc = 0x82206C58; continue 'dispatch;
	// 82206C54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82206C58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206C5C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206C60: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206C64: 419A00A0  beq cr6, 0x82206d04
	if ctx.cr[6].eq {
	pc = 0x82206D04; continue 'dispatch;
	}
	// 82206C68: 3D4082C0  lis r10, -0x7d40
	ctx.r[10].s64 = -2101346304;
	// 82206C6C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206C70: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82206C74: 392BD8B8  addi r9, r11, -0x2748
	ctx.r[9].s64 = ctx.r[11].s64 + -10056;
	// 82206C78: 816ABC14  lwz r11, -0x43ec(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17388 as u32) ) } as u64;
	// 82206C7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206C80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206C84: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82206C88: 419A0018  beq cr6, 0x82206ca0
	if ctx.cr[6].eq {
	pc = 0x82206CA0; continue 'dispatch;
	}
	// 82206C8C: 813EFAC0  lwz r9, -0x540(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206C90: 7F091840  cmplw cr6, r9, r3
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82206C94: 409A000C  bne cr6, 0x82206ca0
	if !ctx.cr[6].eq {
	pc = 0x82206CA0; continue 'dispatch;
	}
	// 82206C98: 9B830019  stb r28, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[28].u8 ) };
	// 82206C9C: 9BA30025  stb r29, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[29].u8 ) };
	// 82206CA0: 812B0058  lwz r9, 0x58(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82206CA4: 906ABC14  stw r3, -0x43ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17388 as u32), ctx.r[3].u32 ) };
	// 82206CA8: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82206CAC: 91030084  stw r8, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[8].u32 ) };
	// 82206CB0: 91430080  stw r10, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82206CB4: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82206CB8: C1AA0000  lfs f13, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82206CBC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82206CC0: C00A2054  lfs f0, 0x2054(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82206CC4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82206CC8: D0030088  stfs f0, 0x88(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82206CCC: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82206CD0: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82206CD4: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82206CD8: C00A2198  lfs f0, 0x2198(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8600 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82206CDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82206CE0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82206CE4: D003008C  stfs f0, 0x8c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82206CE8: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82206CEC: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82206CF0: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82206CF4: D0030090  stfs f0, 0x90(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82206CF8: 396BBC18  addi r11, r11, -0x43e8
	ctx.r[11].s64 = ctx.r[11].s64 + -17384;
	// 82206CFC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82206D00: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82206D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82206D08: 4832E400  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82206D10 size=220
    let mut pc: u32 = 0x82206D10;
    'dispatch: loop {
        match pc {
            0x82206D10 => {
    //   block [0x82206D10..0x82206DEC)
	// 82206D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82206D18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82206D1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206D20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206D24: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206D28: 393F0047  addi r9, r31, 0x47
	ctx.r[9].s64 = ctx.r[31].s64 + 71;
	// 82206D2C: 394B3CD4  addi r10, r11, 0x3cd4
	ctx.r[10].s64 = ctx.r[11].s64 + 15572;
	// 82206D30: 552B0036  rlwinm r11, r9, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82206D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82206D38: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82206D3C: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82206D40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82206D44: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82206D48: F93F0010  std r9, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u64 ) };
	// 82206D4C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82206D50: A145000A  lhz r10, 0xa(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(10 as u32) ) } as u64;
	// 82206D54: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82206D58: B15F0018  sth r10, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 82206D5C: A1450008  lhz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206D60: B15F001A  sth r10, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[10].u16 ) };
	// 82206D64: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82206D68: 81250064  lwz r9, 0x64(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(100 as u32) ) } as u64;
	// 82206D6C: 554A303E  rotlwi r10, r10, 6
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(6)) as u64;
	// 82206D70: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82206D74: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82206D78: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 82206D7C: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82206D80: 48182721  bl 0x823894a0
	ctx.lr = 0x82206D84;
	sub_823894A0(ctx, base);
	// 82206D84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206D88: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82206D8C: 409A0010  bne cr6, 0x82206d9c
	if !ctx.cr[6].eq {
	pc = 0x82206D9C; continue 'dispatch;
	}
	// 82206D90: 3D608284  lis r11, -0x7d7c
	ctx.r[11].s64 = -2105278464;
	// 82206D94: 816BC8BC  lwz r11, -0x3744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14148 as u32) ) } as u64;
	// 82206D98: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82206D9C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82206DA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82206DA4: 481826FD  bl 0x823894a0
	ctx.lr = 0x82206DA8;
	sub_823894A0(ctx, base);
	// 82206DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206DAC: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82206DB0: 409A000C  bne cr6, 0x82206dbc
	if !ctx.cr[6].eq {
	pc = 0x82206DBC; continue 'dispatch;
	}
	// 82206DB4: 48182765  bl 0x82389518
	ctx.lr = 0x82206DB8;
	sub_82389518(ctx, base);
	// 82206DB8: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82206DBC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206DC0: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82206DC4: 396B4140  addi r11, r11, 0x4140
	ctx.r[11].s64 = ctx.r[11].s64 + 16704;
	// 82206DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82206DCC: C00ABA38  lfs f0, -0x45c8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82206DD0: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82206DD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82206DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82206DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206DE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82206DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82206DF0 size=284
    let mut pc: u32 = 0x82206DF0;
    'dispatch: loop {
        match pc {
            0x82206DF0 => {
    //   block [0x82206DF0..0x82206F0C)
	// 82206DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206DF4: 4832E2C5  bl 0x825350b8
	ctx.lr = 0x82206DF8;
	sub_82535080(ctx, base);
	// 82206DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206DFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82206E00: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82206E04: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206E08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206E0C: 419A0048  beq cr6, 0x82206e54
	if ctx.cr[6].eq {
	pc = 0x82206E54; continue 'dispatch;
	}
	// 82206E10: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206E14: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206E18: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82206E1C: 409A0038  bne cr6, 0x82206e54
	if !ctx.cr[6].eq {
	pc = 0x82206E54; continue 'dispatch;
	}
	// 82206E20: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206E24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206E28: 419A002C  beq cr6, 0x82206e54
	if ctx.cr[6].eq {
	pc = 0x82206E54; continue 'dispatch;
	}
	// 82206E2C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206E30: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206E34: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206E38: 409A001C  bne cr6, 0x82206e54
	if !ctx.cr[6].eq {
	pc = 0x82206E54; continue 'dispatch;
	}
	// 82206E3C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206E40: A12B0014  lhz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206E44: 9B8B0025  stb r28, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[28].u8 ) };
	// 82206E48: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82206E4C: 612A8000  ori r10, r9, 0x8000
	ctx.r[10].u64 = ctx.r[9].u64 | 32768;
	// 82206E50: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206E54: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206E58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206E5C: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206E60: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206E64: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82206E68: 38600510  li r3, 0x510
	ctx.r[3].s64 = 1296;
	// 82206E6C: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206E70: 48163621  bl 0x8236a490
	ctx.lr = 0x82206E74;
	sub_8236A490(ctx, base);
	// 82206E74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206E78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82206E7C: 419A000C  beq cr6, 0x82206e88
	if ctx.cr[6].eq {
	pc = 0x82206E88; continue 'dispatch;
	}
	// 82206E80: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206E84: 48000008  b 0x82206e8c
	pc = 0x82206E8C; continue 'dispatch;
	// 82206E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82206E8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82206E90: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206E94: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82206E98: 419A006C  beq cr6, 0x82206f04
	if ctx.cr[6].eq {
	pc = 0x82206F04; continue 'dispatch;
	}
	// 82206E9C: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206EA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82206EA8: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82206EAC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82206EB0: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 82206EB4: 4812767D  bl 0x8232e530
	ctx.lr = 0x82206EB8;
	sub_8232E530(ctx, base);
	// 82206EB8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82206EBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82206EC0: 9B9F001C  stb r28, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u8 ) };
	// 82206EC4: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82206EC8: 93BF0504  stw r29, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[29].u32 ) };
	// 82206ECC: 396BD818  addi r11, r11, -0x27e8
	ctx.r[11].s64 = ctx.r[11].s64 + -10216;
	// 82206ED0: 93DF03CC  stw r30, 0x3cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(972 as u32), ctx.r[30].u32 ) };
	// 82206ED4: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82206ED8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82206EDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82206EE0: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82206EE4: 913F0394  stw r9, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[9].u32 ) };
	// 82206EE8: C1AA203C  lfs f13, 0x203c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8252 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82206EEC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82206EF0: D1BF0398  stfs f13, 0x398(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(920 as u32), tmp.u32 ) };
	// 82206EF4: C18A2690  lfs f12, 0x2690(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9872 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82206EF8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82206EFC: D19F039C  stfs f12, 0x39c(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(924 as u32), tmp.u32 ) };
	// 82206F00: 915F0390  stw r10, 0x390(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(912 as u32), ctx.r[10].u32 ) };
	// 82206F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82206F08: 4832E200  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82206F10 size=204
    let mut pc: u32 = 0x82206F10;
    'dispatch: loop {
        match pc {
            0x82206F10 => {
    //   block [0x82206F10..0x82206FDC)
	// 82206F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82206F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82206F1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206F24: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206F28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206F2C: 419A004C  beq cr6, 0x82206f78
	if ctx.cr[6].eq {
	pc = 0x82206F78; continue 'dispatch;
	}
	// 82206F30: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206F34: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206F38: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82206F3C: 409A003C  bne cr6, 0x82206f78
	if !ctx.cr[6].eq {
	pc = 0x82206F78; continue 'dispatch;
	}
	// 82206F40: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206F44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82206F48: 419A0030  beq cr6, 0x82206f78
	if ctx.cr[6].eq {
	pc = 0x82206F78; continue 'dispatch;
	}
	// 82206F4C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206F50: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82206F54: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82206F58: 409A0020  bne cr6, 0x82206f78
	if !ctx.cr[6].eq {
	pc = 0x82206F78; continue 'dispatch;
	}
	// 82206F5C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82206F60: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82206F64: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82206F68: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82206F6C: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82206F70: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82206F74: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82206F78: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82206F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82206F80: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82206F84: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82206F88: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82206F8C: 38600230  li r3, 0x230
	ctx.r[3].s64 = 560;
	// 82206F90: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82206F94: 481634FD  bl 0x8236a490
	ctx.lr = 0x82206F98;
	sub_8236A490(ctx, base);
	// 82206F98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206F9C: 419A000C  beq cr6, 0x82206fa8
	if ctx.cr[6].eq {
	pc = 0x82206FA8; continue 'dispatch;
	}
	// 82206FA0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82206FA4: 48000008  b 0x82206fac
	pc = 0x82206FAC; continue 'dispatch;
	// 82206FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82206FAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82206FB0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82206FB4: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82206FB8: 419A0010  beq cr6, 0x82206fc8
	if ctx.cr[6].eq {
	pc = 0x82206FC8; continue 'dispatch;
	}
	// 82206FBC: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82206FC0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82206FC4: 4812C7B5  bl 0x82333778
	ctx.lr = 0x82206FC8;
	sub_82333778(ctx, base);
	// 82206FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82206FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82206FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82206FD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82206FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82206FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82206FE0 size=304
    let mut pc: u32 = 0x82206FE0;
    'dispatch: loop {
        match pc {
            0x82206FE0 => {
    //   block [0x82206FE0..0x82207110)
	// 82206FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82206FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82206FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82206FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82206FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82206FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82206FF8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82206FFC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207000: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207004: 419A0048  beq cr6, 0x8220704c
	if ctx.cr[6].eq {
	pc = 0x8220704C; continue 'dispatch;
	}
	// 82207008: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220700C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207010: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82207014: 409A0038  bne cr6, 0x8220704c
	if !ctx.cr[6].eq {
	pc = 0x8220704C; continue 'dispatch;
	}
	// 82207018: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220701C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207020: 419A002C  beq cr6, 0x8220704c
	if ctx.cr[6].eq {
	pc = 0x8220704C; continue 'dispatch;
	}
	// 82207024: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207028: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220702C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207030: 409A001C  bne cr6, 0x8220704c
	if !ctx.cr[6].eq {
	pc = 0x8220704C; continue 'dispatch;
	}
	// 82207034: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82207038: A12B0014  lhz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220703C: 9BCB0025  stb r30, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[30].u8 ) };
	// 82207040: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82207044: 612A8000  ori r10, r9, 0x8000
	ctx.r[10].u64 = ctx.r[9].u64 | 32768;
	// 82207048: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 8220704C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207050: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82207054: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82207058: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8220705C: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82207060: 38600180  li r3, 0x180
	ctx.r[3].s64 = 384;
	// 82207064: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82207068: 48163429  bl 0x8236a490
	ctx.lr = 0x8220706C;
	sub_8236A490(ctx, base);
	// 8220706C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82207070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207074: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82207078: 419A000C  beq cr6, 0x82207084
	if ctx.cr[6].eq {
	pc = 0x82207084; continue 'dispatch;
	}
	// 8220707C: 81470030  lwz r10, 0x30(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207080: 48000008  b 0x82207088
	pc = 0x82207088; continue 'dispatch;
	// 82207084: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82207088: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8220708C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82207090: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82207094: 419A0064  beq cr6, 0x822070f8
	if ctx.cr[6].eq {
	pc = 0x822070F8; continue 'dispatch;
	}
	// 82207098: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8220709C: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822070A0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 822070A4: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822070A8: 394ADCA0  addi r10, r10, -0x2360
	ctx.r[10].s64 = ctx.r[10].s64 + -9056;
	// 822070AC: 38670168  addi r3, r7, 0x168
	ctx.r[3].s64 = ctx.r[7].s64 + 360;
	// 822070B0: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822070B4: 91670080  stw r11, 0x80(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 822070B8: 91670084  stw r11, 0x84(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 822070BC: 912700AC  stw r9, 0xac(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(172 as u32), ctx.r[9].u32 ) };
	// 822070C0: 916700C0  stw r11, 0xc0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 822070C4: B16700C4  sth r11, 0xc4(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(196 as u32), ctx.r[11].u16 ) };
	// 822070C8: B16700C6  sth r11, 0xc6(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(198 as u32), ctx.r[11].u16 ) };
	// 822070CC: 4812E485  bl 0x82335550
	ctx.lr = 0x822070D0;
	sub_82335550(ctx, base);
	// 822070D0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 822070D4: 396BDCC0  addi r11, r11, -0x2340
	ctx.r[11].s64 = ctx.r[11].s64 + -9024;
	// 822070D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822070DC: 90C70160  stw r6, 0x160(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(352 as u32), ctx.r[6].u32 ) };
	// 822070E0: 81660058  lwz r11, 0x58(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) } as u64;
	// 822070E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822070E8: 93C70170  stw r30, 0x170(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(368 as u32), ctx.r[30].u32 ) };
	// 822070EC: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 822070F0: 90E7016C  stw r7, 0x16c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(364 as u32), ctx.r[7].u32 ) };
	// 822070F4: 91670164  stw r11, 0x164(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(356 as u32), ctx.r[11].u32 ) };
	// 822070F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822070FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82207100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82207104: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82207108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8220710C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82207110 size=344
    let mut pc: u32 = 0x82207110;
    'dispatch: loop {
        match pc {
            0x82207110 => {
    //   block [0x82207110..0x82207268)
	// 82207110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82207114: 4832DFA5  bl 0x825350b8
	ctx.lr = 0x82207118;
	sub_82535080(ctx, base);
	// 82207118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220711C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82207120: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207128: 419A004C  beq cr6, 0x82207174
	if ctx.cr[6].eq {
	pc = 0x82207174; continue 'dispatch;
	}
	// 8220712C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207130: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207134: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82207138: 409A003C  bne cr6, 0x82207174
	if !ctx.cr[6].eq {
	pc = 0x82207174; continue 'dispatch;
	}
	// 8220713C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207144: 419A0030  beq cr6, 0x82207174
	if ctx.cr[6].eq {
	pc = 0x82207174; continue 'dispatch;
	}
	// 82207148: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220714C: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207150: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207154: 409A0020  bne cr6, 0x82207174
	if !ctx.cr[6].eq {
	pc = 0x82207174; continue 'dispatch;
	}
	// 82207158: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8220715C: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207160: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82207164: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82207168: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 8220716C: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82207170: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82207174: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207178: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8220717C: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82207180: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82207184: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82207188: 38600400  li r3, 0x400
	ctx.r[3].s64 = 1024;
	// 8220718C: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82207190: 48163301  bl 0x8236a490
	ctx.lr = 0x82207194;
	sub_8236A490(ctx, base);
	// 82207194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82207198: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8220719C: 419A000C  beq cr6, 0x822071a8
	if ctx.cr[6].eq {
	pc = 0x822071A8; continue 'dispatch;
	}
	// 822071A0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 822071A4: 48000008  b 0x822071ac
	pc = 0x822071AC; continue 'dispatch;
	// 822071A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822071AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822071B0: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822071B4: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 822071B8: 419A00A8  beq cr6, 0x82207260
	if ctx.cr[6].eq {
	pc = 0x82207260; continue 'dispatch;
	}
	// 822071BC: 3B9E001C  addi r28, r30, 0x1c
	ctx.r[28].s64 = ctx.r[30].s64 + 28;
	// 822071C0: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822071C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822071C8: 80DE0040  lwz r6, 0x40(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 822071CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822071D0: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822071D4: 4812735D  bl 0x8232e530
	ctx.lr = 0x822071D8;
	sub_8232E530(ctx, base);
	// 822071D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 822071DC: 387F0390  addi r3, r31, 0x390
	ctx.r[3].s64 = ctx.r[31].s64 + 912;
	// 822071E0: 396BDD38  addi r11, r11, -0x22c8
	ctx.r[11].s64 = ctx.r[11].s64 + -8904;
	// 822071E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822071E8: 4812E369  bl 0x82335550
	ctx.lr = 0x822071EC;
	sub_82335550(ctx, base);
	// 822071EC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 822071F0: 397E002C  addi r11, r30, 0x2c
	ctx.r[11].s64 = ctx.r[30].s64 + 44;
	// 822071F4: 394ADD58  addi r10, r10, -0x22a8
	ctx.r[10].s64 = ctx.r[10].s64 + -8872;
	// 822071F8: 393E0034  addi r9, r30, 0x34
	ctx.r[9].s64 = ctx.r[30].s64 + 52;
	// 822071FC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82207200: 917F03EC  stw r11, 0x3ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1004 as u32), ctx.r[11].u32 ) };
	// 82207204: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207208: 93BF03E0  stw r29, 0x3e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(992 as u32), ctx.r[29].u32 ) };
	// 8220720C: 913F03E4  stw r9, 0x3e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(996 as u32), ctx.r[9].u32 ) };
	// 82207210: 939F03E8  stw r28, 0x3e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1000 as u32), ctx.r[28].u32 ) };
	// 82207214: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82207218: 93FF0394  stw r31, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[31].u32 ) };
	// 8220721C: 897D0004  lbz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207220: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82207224: 409A0010  bne cr6, 0x82207234
	if !ctx.cr[6].eq {
	pc = 0x82207234; continue 'dispatch;
	}
	// 82207228: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 8220722C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207230: 917F0398  stw r11, 0x398(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(920 as u32), ctx.r[11].u32 ) };
	// 82207234: 815F0398  lwz r10, 0x398(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 82207238: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8220723C: 396BBCD0  addi r11, r11, -0x4330
	ctx.r[11].s64 = ctx.r[11].s64 + -17200;
	// 82207240: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82207244: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82207248: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8220724C: 409A0010  bne cr6, 0x8220725c
	if !ctx.cr[6].eq {
	pc = 0x8220725C; continue 'dispatch;
	}
	// 82207250: 7FEA592E  stwx r31, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[31].u32) };
	// 82207254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82207258: 4832DEB0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8220725C: 913F03F4  stw r9, 0x3f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1012 as u32), ctx.r[9].u32 ) };
	// 82207260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82207264: 4832DEA4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82207268 size=272
    let mut pc: u32 = 0x82207268;
    'dispatch: loop {
        match pc {
            0x82207268 => {
    //   block [0x82207268..0x82207378)
	// 82207268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220726C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82207270: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82207274: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82207278: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220727C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207284: 419A004C  beq cr6, 0x822072d0
	if ctx.cr[6].eq {
	pc = 0x822072D0; continue 'dispatch;
	}
	// 82207288: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220728C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207290: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82207294: 409A003C  bne cr6, 0x822072d0
	if !ctx.cr[6].eq {
	pc = 0x822072D0; continue 'dispatch;
	}
	// 82207298: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220729C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822072A0: 419A0030  beq cr6, 0x822072d0
	if ctx.cr[6].eq {
	pc = 0x822072D0; continue 'dispatch;
	}
	// 822072A4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822072A8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822072AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822072B0: 409A0020  bne cr6, 0x822072d0
	if !ctx.cr[6].eq {
	pc = 0x822072D0; continue 'dispatch;
	}
	// 822072B4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 822072B8: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822072BC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822072C0: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 822072C4: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 822072C8: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 822072CC: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 822072D0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822072D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822072D8: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 822072DC: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 822072E0: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 822072E4: 38600760  li r3, 0x760
	ctx.r[3].s64 = 1888;
	// 822072E8: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 822072EC: 481631A5  bl 0x8236a490
	ctx.lr = 0x822072F0;
	sub_8236A490(ctx, base);
	// 822072F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822072F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822072F8: 419A000C  beq cr6, 0x82207304
	if ctx.cr[6].eq {
	pc = 0x82207304; continue 'dispatch;
	}
	// 822072FC: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207300: 48000008  b 0x82207308
	pc = 0x82207308; continue 'dispatch;
	// 82207304: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82207308: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8220730C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82207310: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82207314: 419A0050  beq cr6, 0x82207364
	if ctx.cr[6].eq {
	pc = 0x82207364; continue 'dispatch;
	}
	// 82207318: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8220731C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207320: 390A3D84  addi r8, r10, 0x3d84
	ctx.r[8].s64 = ctx.r[10].s64 + 15748;
	// 82207324: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82207328: 394A3D78  addi r10, r10, 0x3d78
	ctx.r[10].s64 = ctx.r[10].s64 + 15736;
	// 8220732C: 91230120  stw r9, 0x120(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), ctx.r[9].u32 ) };
	// 82207330: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82207334: F9630140  std r11, 0x140(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(320 as u32), ctx.r[11].u64 ) };
	// 82207338: F9630170  std r11, 0x170(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(368 as u32), ctx.r[11].u64 ) };
	// 8220733C: 91430130  stw r10, 0x130(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), ctx.r[10].u32 ) };
	// 82207340: F96302B0  std r11, 0x2b0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(688 as u32), ctx.r[11].u64 ) };
	// 82207344: F96302E0  std r11, 0x2e0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(736 as u32), ctx.r[11].u64 ) };
	// 82207348: 914302A0  stw r10, 0x2a0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(672 as u32), ctx.r[10].u32 ) };
	// 8220734C: F9630420  std r11, 0x420(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(1056 as u32), ctx.r[11].u64 ) };
	// 82207350: F9630450  std r11, 0x450(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(1104 as u32), ctx.r[11].u64 ) };
	// 82207354: 91430410  stw r10, 0x410(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1040 as u32), ctx.r[10].u32 ) };
	// 82207358: F9630590  std r11, 0x590(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(1424 as u32), ctx.r[11].u64 ) };
	// 8220735C: F96305C0  std r11, 0x5c0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(1472 as u32), ctx.r[11].u64 ) };
	// 82207360: 91430580  stw r10, 0x580(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1408 as u32), ctx.r[10].u32 ) };
	// 82207364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82207368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8220736C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82207370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82207374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82207378 size=248
    let mut pc: u32 = 0x82207378;
    'dispatch: loop {
        match pc {
            0x82207378 => {
    //   block [0x82207378..0x82207470)
	// 82207378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8220737C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82207380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82207384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82207388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8220738C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207390: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207394: 419A004C  beq cr6, 0x822073e0
	if ctx.cr[6].eq {
	pc = 0x822073E0; continue 'dispatch;
	}
	// 82207398: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220739C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822073A0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822073A4: 409A003C  bne cr6, 0x822073e0
	if !ctx.cr[6].eq {
	pc = 0x822073E0; continue 'dispatch;
	}
	// 822073A8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822073AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822073B0: 419A0030  beq cr6, 0x822073e0
	if ctx.cr[6].eq {
	pc = 0x822073E0; continue 'dispatch;
	}
	// 822073B4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822073B8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822073BC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822073C0: 409A0020  bne cr6, 0x822073e0
	if !ctx.cr[6].eq {
	pc = 0x822073E0; continue 'dispatch;
	}
	// 822073C4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 822073C8: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822073CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822073D0: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 822073D4: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 822073D8: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 822073DC: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 822073E0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822073E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822073E8: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 822073EC: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 822073F0: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 822073F4: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 822073F8: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 822073FC: 48163095  bl 0x8236a490
	ctx.lr = 0x82207400;
	sub_8236A490(ctx, base);
	// 82207400: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207404: 419A000C  beq cr6, 0x82207410
	if ctx.cr[6].eq {
	pc = 0x82207410; continue 'dispatch;
	}
	// 82207408: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220740C: 48000008  b 0x82207414
	pc = 0x82207414; continue 'dispatch;
	// 82207410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207414: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207418: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8220741C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82207420: 419A003C  beq cr6, 0x8220745c
	if ctx.cr[6].eq {
	pc = 0x8220745C; continue 'dispatch;
	}
	// 82207424: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82207428: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220742C: 397F0060  addi r11, r31, 0x60
	ctx.r[11].s64 = ctx.r[31].s64 + 96;
	// 82207430: 3D0082C0  lis r8, -0x7d40
	ctx.r[8].s64 = -2101346304;
	// 82207434: 3908BC50  addi r8, r8, -0x43b0
	ctx.r[8].s64 = ctx.r[8].s64 + -17328;
	// 82207438: C0091FF8  lfs f0, 0x1ff8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8220743C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82207440: 3929DCEC  addi r9, r9, -0x2314
	ctx.r[9].s64 = ctx.r[9].s64 + -8980;
	// 82207444: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82207448: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8220744C: 814A0050  lwz r10, 0x50(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 82207450: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207454: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82207458: 7D6A412E  stwx r11, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u32) };
	// 8220745C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82207460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82207464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82207468: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8220746C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82207470 size=376
    let mut pc: u32 = 0x82207470;
    'dispatch: loop {
        match pc {
            0x82207470 => {
    //   block [0x82207470..0x822075E8)
	// 82207470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82207474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82207478: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8220747C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82207480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82207484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82207488: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8220748C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207494: 419A0048  beq cr6, 0x822074dc
	if ctx.cr[6].eq {
	pc = 0x822074DC; continue 'dispatch;
	}
	// 82207498: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220749C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822074A0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822074A4: 409A0038  bne cr6, 0x822074dc
	if !ctx.cr[6].eq {
	pc = 0x822074DC; continue 'dispatch;
	}
	// 822074A8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822074AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822074B0: 419A002C  beq cr6, 0x822074dc
	if ctx.cr[6].eq {
	pc = 0x822074DC; continue 'dispatch;
	}
	// 822074B4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822074B8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822074BC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822074C0: 409A001C  bne cr6, 0x822074dc
	if !ctx.cr[6].eq {
	pc = 0x822074DC; continue 'dispatch;
	}
	// 822074C4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 822074C8: A12B0014  lhz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822074CC: 9BCB0025  stb r30, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[30].u8 ) };
	// 822074D0: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 822074D4: 612A8000  ori r10, r9, 0x8000
	ctx.r[10].u64 = ctx.r[9].u64 | 32768;
	// 822074D8: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 822074DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822074E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822074E4: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 822074E8: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 822074EC: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 822074F0: 38600110  li r3, 0x110
	ctx.r[3].s64 = 272;
	// 822074F4: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 822074F8: 48162F99  bl 0x8236a490
	ctx.lr = 0x822074FC;
	sub_8236A490(ctx, base);
	// 822074FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207500: 419A000C  beq cr6, 0x8220750c
	if ctx.cr[6].eq {
	pc = 0x8220750C; continue 'dispatch;
	}
	// 82207504: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207508: 48000008  b 0x82207510
	pc = 0x82207510; continue 'dispatch;
	// 8220750C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207510: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207514: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82207518: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8220751C: 419A00B4  beq cr6, 0x822075d0
	if ctx.cr[6].eq {
	pc = 0x822075D0; continue 'dispatch;
	}
	// 82207520: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82207524: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207528: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8220752C: 394ADE0C  addi r10, r10, -0x21f4
	ctx.r[10].s64 = ctx.r[10].s64 + -8692;
	// 82207530: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82207534: 894B0007  lbz r10, 7(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 82207538: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8220753C: 419A0054  beq cr6, 0x82207590
	if ctx.cr[6].eq {
	pc = 0x82207590; continue 'dispatch;
	}
	// 82207540: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82207544: 394300C0  addi r10, r3, 0xc0
	ctx.r[10].s64 = ctx.r[3].s64 + 192;
	// 82207548: 2B070010  cmplwi cr6, r7, 0x10
	ctx.cr[6].compare_u32(ctx.r[7].u32, 16 as u32, &mut ctx.xer);
	// 8220754C: 4098002C  bge cr6, 0x82207578
	if !ctx.cr[6].lt {
	pc = 0x82207578; continue 'dispatch;
	}
	// 82207550: 810B005C  lwz r8, 0x5c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82207554: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82207558: 910AFFC0  stw r8, -0x40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-64 as u32), ctx.r[8].u32 ) };
	// 8220755C: 810B005C  lwz r8, 0x5c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82207560: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82207564: 39080020  addi r8, r8, 0x20
	ctx.r[8].s64 = ctx.r[8].s64 + 32;
	// 82207568: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8220756C: 81030100  lwz r8, 0x100(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82207570: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82207574: 91030100  stw r8, 0x100(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[8].u32 ) };
	// 82207578: 890B0007  lbz r8, 7(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 8220757C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82207580: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82207584: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 82207588: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8220758C: 4198FFBC  blt cr6, 0x82207548
	if ctx.cr[6].lt {
	pc = 0x82207548; continue 'dispatch;
	}
	// 82207590: 894B0005  lbz r10, 5(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82207594: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82207598: 409A001C  bne cr6, 0x822075b4
	if !ctx.cr[6].eq {
	pc = 0x822075B4; continue 'dispatch;
	}
	// 8220759C: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 822075A0: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822075A4: D0030108  stfs f0, 0x108(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 822075A8: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 822075AC: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822075B0: D003010C  stfs f0, 0x10c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 822075B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 822075B8: 9BC3001C  stb r30, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 822075BC: C00B223C  lfs f0, 0x223c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8764 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822075C0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 822075C4: D0030108  stfs f0, 0x108(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 822075C8: C1ABD218  lfs f13, -0x2de8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11752 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822075CC: D1A3010C  stfs f13, 0x10c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 822075D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822075D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822075D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822075DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822075E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822075E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822075E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822075E8 size=220
    let mut pc: u32 = 0x822075E8;
    'dispatch: loop {
        match pc {
            0x822075E8 => {
    //   block [0x822075E8..0x822076C4)
	// 822075E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822075EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822075F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822075F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822075F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822075FC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207600: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207604: 419A004C  beq cr6, 0x82207650
	if ctx.cr[6].eq {
	pc = 0x82207650; continue 'dispatch;
	}
	// 82207608: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220760C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207610: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82207614: 409A003C  bne cr6, 0x82207650
	if !ctx.cr[6].eq {
	pc = 0x82207650; continue 'dispatch;
	}
	// 82207618: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220761C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207620: 419A0030  beq cr6, 0x82207650
	if ctx.cr[6].eq {
	pc = 0x82207650; continue 'dispatch;
	}
	// 82207624: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207628: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220762C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207630: 409A0020  bne cr6, 0x82207650
	if !ctx.cr[6].eq {
	pc = 0x82207650; continue 'dispatch;
	}
	// 82207634: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82207638: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220763C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82207640: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82207644: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82207648: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 8220764C: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82207650: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82207658: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 8220765C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82207660: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82207664: 386003C0  li r3, 0x3c0
	ctx.r[3].s64 = 960;
	// 82207668: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8220766C: 48162E25  bl 0x8236a490
	ctx.lr = 0x82207670;
	sub_8236A490(ctx, base);
	// 82207670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207674: 419A000C  beq cr6, 0x82207680
	if ctx.cr[6].eq {
	pc = 0x82207680; continue 'dispatch;
	}
	// 82207678: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220767C: 48000008  b 0x82207684
	pc = 0x82207684; continue 'dispatch;
	// 82207680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207684: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207688: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8220768C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82207690: 419A0020  beq cr6, 0x822076b0
	if ctx.cr[6].eq {
	pc = 0x822076B0; continue 'dispatch;
	}
	// 82207694: 391F002C  addi r8, r31, 0x2c
	ctx.r[8].s64 = ctx.r[31].s64 + 44;
	// 82207698: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8220769C: 38FF0028  addi r7, r31, 0x28
	ctx.r[7].s64 = ctx.r[31].s64 + 40;
	// 822076A0: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822076A4: 38DF001C  addi r6, r31, 0x1c
	ctx.r[6].s64 = ctx.r[31].s64 + 28;
	// 822076A8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822076AC: 4812E32D  bl 0x823359d8
	ctx.lr = 0x822076B0;
	sub_823359D8(ctx, base);
	// 822076B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822076B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822076B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822076BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822076C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822076C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822076C8 size=220
    let mut pc: u32 = 0x822076C8;
    'dispatch: loop {
        match pc {
            0x822076C8 => {
    //   block [0x822076C8..0x822077A4)
	// 822076C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822076CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822076D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822076D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822076D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822076DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822076E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822076E4: 419A004C  beq cr6, 0x82207730
	if ctx.cr[6].eq {
	pc = 0x82207730; continue 'dispatch;
	}
	// 822076E8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822076EC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822076F0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822076F4: 409A003C  bne cr6, 0x82207730
	if !ctx.cr[6].eq {
	pc = 0x82207730; continue 'dispatch;
	}
	// 822076F8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822076FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207700: 419A0030  beq cr6, 0x82207730
	if ctx.cr[6].eq {
	pc = 0x82207730; continue 'dispatch;
	}
	// 82207704: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207708: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220770C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207710: 409A0020  bne cr6, 0x82207730
	if !ctx.cr[6].eq {
	pc = 0x82207730; continue 'dispatch;
	}
	// 82207714: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82207718: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220771C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82207720: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82207724: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82207728: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 8220772C: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82207730: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82207738: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 8220773C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82207740: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82207744: 386003C0  li r3, 0x3c0
	ctx.r[3].s64 = 960;
	// 82207748: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8220774C: 48162D45  bl 0x8236a490
	ctx.lr = 0x82207750;
	sub_8236A490(ctx, base);
	// 82207750: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207754: 419A000C  beq cr6, 0x82207760
	if ctx.cr[6].eq {
	pc = 0x82207760; continue 'dispatch;
	}
	// 82207758: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220775C: 48000008  b 0x82207764
	pc = 0x82207764; continue 'dispatch;
	// 82207760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207764: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207768: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8220776C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82207770: 419A0020  beq cr6, 0x82207790
	if ctx.cr[6].eq {
	pc = 0x82207790; continue 'dispatch;
	}
	// 82207774: 391F0034  addi r8, r31, 0x34
	ctx.r[8].s64 = ctx.r[31].s64 + 52;
	// 82207778: 813F0040  lwz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8220777C: 38FF002C  addi r7, r31, 0x2c
	ctx.r[7].s64 = ctx.r[31].s64 + 44;
	// 82207780: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82207784: 38DF001C  addi r6, r31, 0x1c
	ctx.r[6].s64 = ctx.r[31].s64 + 28;
	// 82207788: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220778C: 4812E24D  bl 0x823359d8
	ctx.lr = 0x82207790;
	sub_823359D8(ctx, base);
	// 82207790: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82207794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82207798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8220779C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822077A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822077A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822077A8 size=276
    let mut pc: u32 = 0x822077A8;
    'dispatch: loop {
        match pc {
            0x822077A8 => {
    //   block [0x822077A8..0x822078BC)
	// 822077A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822077AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822077B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822077B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822077B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822077BC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822077C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822077C4: 419A004C  beq cr6, 0x82207810
	if ctx.cr[6].eq {
	pc = 0x82207810; continue 'dispatch;
	}
	// 822077C8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822077CC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822077D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822077D4: 409A003C  bne cr6, 0x82207810
	if !ctx.cr[6].eq {
	pc = 0x82207810; continue 'dispatch;
	}
	// 822077D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822077DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822077E0: 419A0030  beq cr6, 0x82207810
	if ctx.cr[6].eq {
	pc = 0x82207810; continue 'dispatch;
	}
	// 822077E4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822077E8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822077EC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822077F0: 409A0020  bne cr6, 0x82207810
	if !ctx.cr[6].eq {
	pc = 0x82207810; continue 'dispatch;
	}
	// 822077F4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 822077F8: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822077FC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82207800: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82207804: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82207808: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 8220780C: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82207810: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82207818: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 8220781C: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82207820: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82207824: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 82207828: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8220782C: 48162C65  bl 0x8236a490
	ctx.lr = 0x82207830;
	sub_8236A490(ctx, base);
	// 82207830: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207834: 419A000C  beq cr6, 0x82207840
	if ctx.cr[6].eq {
	pc = 0x82207840; continue 'dispatch;
	}
	// 82207838: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220783C: 48000008  b 0x82207844
	pc = 0x82207844; continue 'dispatch;
	// 82207840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207844: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207848: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8220784C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82207850: 419A0058  beq cr6, 0x822078a8
	if ctx.cr[6].eq {
	pc = 0x822078A8; continue 'dispatch;
	}
	// 82207854: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82207858: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220785C: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82207860: 3929DE4C  addi r9, r9, -0x21b4
	ctx.r[9].s64 = ctx.r[9].s64 + -8628;
	// 82207864: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82207868: 91430088  stw r10, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 8220786C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82207870: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207874: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82207878: 409A0010  bne cr6, 0x82207888
	if !ctx.cr[6].eq {
	pc = 0x82207888; continue 'dispatch;
	}
	// 8220787C: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82207880: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207884: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82207888: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8220788C: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82207890: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82207894: D00300A0  stfs f0, 0xa0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82207898: D00300AC  stfs f0, 0xac(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8220789C: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822078A0: D1A300A4  stfs f13, 0xa4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 822078A4: D1A300A8  stfs f13, 0xa8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 822078A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822078AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822078B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822078B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822078B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822078C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822078C0 size=412
    let mut pc: u32 = 0x822078C0;
    'dispatch: loop {
        match pc {
            0x822078C0 => {
    //   block [0x822078C0..0x82207A5C)
	// 822078C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822078C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822078C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822078CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822078D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822078D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822078D8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 822078DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822078E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822078E4: 419A0048  beq cr6, 0x8220792c
	if ctx.cr[6].eq {
	pc = 0x8220792C; continue 'dispatch;
	}
	// 822078E8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822078EC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822078F0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822078F4: 409A0038  bne cr6, 0x8220792c
	if !ctx.cr[6].eq {
	pc = 0x8220792C; continue 'dispatch;
	}
	// 822078F8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822078FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207900: 419A002C  beq cr6, 0x8220792c
	if ctx.cr[6].eq {
	pc = 0x8220792C; continue 'dispatch;
	}
	// 82207904: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207908: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8220790C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207910: 409A001C  bne cr6, 0x8220792c
	if !ctx.cr[6].eq {
	pc = 0x8220792C; continue 'dispatch;
	}
	// 82207914: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82207918: A12B0014  lhz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8220791C: 9BCB0025  stb r30, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[30].u8 ) };
	// 82207920: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82207924: 612A8000  ori r10, r9, 0x8000
	ctx.r[10].u64 = ctx.r[9].u64 | 32768;
	// 82207928: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 8220792C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207930: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82207934: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82207938: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8220793C: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82207940: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 82207944: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82207948: 48162B49  bl 0x8236a490
	ctx.lr = 0x8220794C;
	sub_8236A490(ctx, base);
	// 8220794C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82207950: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207954: 419A000C  beq cr6, 0x82207960
	if ctx.cr[6].eq {
	pc = 0x82207960; continue 'dispatch;
	}
	// 82207958: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8220795C: 48000008  b 0x82207964
	pc = 0x82207964; continue 'dispatch;
	// 82207960: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82207964: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207968: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8220796C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82207970: 419A00D4  beq cr6, 0x82207a44
	if ctx.cr[6].eq {
	pc = 0x82207A44; continue 'dispatch;
	}
	// 82207974: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207978: 811F0068  lwz r8, 0x68(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8220797C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207980: 396BF2DC  addi r11, r11, -0xd24
	ctx.r[11].s64 = ctx.r[11].s64 + -3364;
	// 82207984: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82207988: F8E30130  std r7, 0x130(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), ctx.r[7].u64 ) };
	// 8220798C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207990: 9BC3001C  stb r30, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 82207994: A14B0014  lhz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207998: 554A06FE  clrlwi r10, r10, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8220799C: 2B0A0018  cmplwi cr6, r10, 0x18
	ctx.cr[6].compare_u32(ctx.r[10].u32, 24 as u32, &mut ctx.xer);
	// 822079A0: 409A0010  bne cr6, 0x822079b0
	if !ctx.cr[6].eq {
	pc = 0x822079B0; continue 'dispatch;
	}
	// 822079A4: 396B0090  addi r11, r11, 0x90
	ctx.r[11].s64 = ctx.r[11].s64 + 144;
	// 822079A8: 91630138  stw r11, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[11].u32 ) };
	// 822079AC: 48000008  b 0x822079b4
	pc = 0x822079B4; continue 'dispatch;
	// 822079B0: 90E30138  stw r7, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[7].u32 ) };
	// 822079B4: 910300E0  stw r8, 0xe0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(224 as u32), ctx.r[8].u32 ) };
	// 822079B8: 39690010  addi r11, r9, 0x10
	ctx.r[11].s64 = ctx.r[9].s64 + 16;
	// 822079BC: 394300A0  addi r10, r3, 0xa0
	ctx.r[10].s64 = ctx.r[3].s64 + 160;
	// 822079C0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 822079C4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 822079C8: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 822079CC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 822079D0: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 822079D4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 822079D8: 4200FFF0  bdnz 0x822079c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x822079C8; continue 'dispatch;
	}
	// 822079DC: 89690004  lbz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 822079E0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822079E4: 41980010  blt cr6, 0x822079f4
	if ctx.cr[6].lt {
	pc = 0x822079F4; continue 'dispatch;
	}
	// 822079E8: 81690050  lwz r11, 0x50(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(80 as u32) ) } as u64;
	// 822079EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822079F0: 48000008  b 0x822079f8
	pc = 0x822079F8; continue 'dispatch;
	// 822079F4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 822079F8: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 822079FC: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82207A00: 89690004  lbz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207A04: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82207A08: 41980014  blt cr6, 0x82207a1c
	if ctx.cr[6].lt {
	pc = 0x82207A1C; continue 'dispatch;
	}
	// 82207A0C: 81690050  lwz r11, 0x50(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(80 as u32) ) } as u64;
	// 82207A10: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207A14: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82207A18: 48000008  b 0x82207a20
	pc = 0x82207A20; continue 'dispatch;
	// 82207A1C: 90E30080  stw r7, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[7].u32 ) };
	// 82207A20: 89690004  lbz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207A24: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82207A28: 41980014  blt cr6, 0x82207a3c
	if ctx.cr[6].lt {
	pc = 0x82207A3C; continue 'dispatch;
	}
	// 82207A2C: 81690050  lwz r11, 0x50(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(80 as u32) ) } as u64;
	// 82207A30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207A34: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82207A38: 48000008  b 0x82207a40
	pc = 0x82207A40; continue 'dispatch;
	// 82207A3C: 90E3008C  stw r7, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[7].u32 ) };
	// 82207A40: 93C30090  stw r30, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82207A44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82207A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82207A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82207A50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82207A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82207A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82207A60 size=276
    let mut pc: u32 = 0x82207A60;
    'dispatch: loop {
        match pc {
            0x82207A60 => {
    //   block [0x82207A60..0x82207B74)
	// 82207A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82207A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82207A68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82207A6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82207A70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82207A74: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207A78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207A7C: 419A004C  beq cr6, 0x82207ac8
	if ctx.cr[6].eq {
	pc = 0x82207AC8; continue 'dispatch;
	}
	// 82207A80: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207A84: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207A88: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82207A8C: 409A003C  bne cr6, 0x82207ac8
	if !ctx.cr[6].eq {
	pc = 0x82207AC8; continue 'dispatch;
	}
	// 82207A90: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207A94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207A98: 419A0030  beq cr6, 0x82207ac8
	if ctx.cr[6].eq {
	pc = 0x82207AC8; continue 'dispatch;
	}
	// 82207A9C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207AA0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207AA4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207AA8: 409A0020  bne cr6, 0x82207ac8
	if !ctx.cr[6].eq {
	pc = 0x82207AC8; continue 'dispatch;
	}
	// 82207AAC: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82207AB0: A10B0014  lhz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207AB4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82207AB8: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82207ABC: 610A8000  ori r10, r8, 0x8000
	ctx.r[10].u64 = ctx.r[8].u64 | 32768;
	// 82207AC0: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82207AC4: B14B0014  sth r10, 0x14(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82207AC8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82207AD0: 38EB1160  addi r7, r11, 0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + 4448;
	// 82207AD4: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 82207AD8: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 82207ADC: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82207AE0: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 82207AE4: 481629AD  bl 0x8236a490
	ctx.lr = 0x82207AE8;
	sub_8236A490(ctx, base);
	// 82207AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207AEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207AF0: 419A000C  beq cr6, 0x82207afc
	if ctx.cr[6].eq {
	pc = 0x82207AFC; continue 'dispatch;
	}
	// 82207AF4: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82207AF8: 48000008  b 0x82207b00
	pc = 0x82207B00; continue 'dispatch;
	// 82207AFC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82207B00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207B04: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82207B08: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82207B0C: 419A0054  beq cr6, 0x82207b60
	if ctx.cr[6].eq {
	pc = 0x82207B60; continue 'dispatch;
	}
	// 82207B10: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82207B14: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82207B18: 394AD6E0  addi r10, r10, -0x2920
	ctx.r[10].s64 = ctx.r[10].s64 + -10528;
	// 82207B1C: C009D6C8  lfs f0, -0x2938(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-10552 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82207B20: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82207B24: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82207B28: F9630090  std r11, 0x90(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u64 ) };
	// 82207B2C: C1A91FF8  lfs f13, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82207B30: F9630098  std r11, 0x98(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u64 ) };
	// 82207B34: F96300A0  std r11, 0xa0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u64 ) };
	// 82207B38: F96300A8  std r11, 0xa8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u64 ) };
	// 82207B3C: F96300B0  std r11, 0xb0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[11].u64 ) };
	// 82207B40: F96300B8  std r11, 0xb8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), ctx.r[11].u64 ) };
	// 82207B44: F96300C0  std r11, 0xc0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), ctx.r[11].u64 ) };
	// 82207B48: F96300C8  std r11, 0xc8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[11].u64 ) };
	// 82207B4C: F96300D0  std r11, 0xd0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(208 as u32), ctx.r[11].u64 ) };
	// 82207B50: F96300D8  std r11, 0xd8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(216 as u32), ctx.r[11].u64 ) };
	// 82207B54: D0030080  stfs f0, 0x80(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82207B58: D1A30084  stfs f13, 0x84(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82207B5C: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82207B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82207B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82207B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82207B6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82207B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82207B78 size=80
    let mut pc: u32 = 0x82207B78;
    'dispatch: loop {
        match pc {
            0x82207B78 => {
    //   block [0x82207B78..0x82207BC8)
	// 82207B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82207B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82207B80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82207B84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82207B88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82207B8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82207B90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82207B94: 4BFFC93D  bl 0x822044d0
	ctx.lr = 0x82207B98;
	sub_822044D0(ctx, base);
	// 82207B98: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82207B9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82207BA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207BA4: 419A000C  beq cr6, 0x82207bb0
	if ctx.cr[6].eq {
	pc = 0x82207BB0; continue 'dispatch;
	}
	// 82207BA8: 4832B011  bl 0x82532bb8
	ctx.lr = 0x82207BAC;
	sub_82532BB8(ctx, base);
	// 82207BAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82207BB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82207BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82207BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82207BBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82207BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82207BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82207BC8 size=304
    let mut pc: u32 = 0x82207BC8;
    'dispatch: loop {
        match pc {
            0x82207BC8 => {
    //   block [0x82207BC8..0x82207CF8)
	// 82207BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82207BCC: 4832D4E1  bl 0x825350ac
	ctx.lr = 0x82207BD0;
	sub_82535080(ctx, base);
	// 82207BD0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82207BD4: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82207BD8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82207BDC: FB2100C8  std r25, 0xc8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[25].u64 ) };
	// 82207BE0: 836100CC  lwz r27, 0xcc(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 82207BE4: 2B1B01FF  cmplwi cr6, r27, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[27].u32, 511 as u32, &mut ctx.xer);
	// 82207BE8: 409800E8  bge cr6, 0x82207cd0
	if !ctx.cr[6].lt {
	pc = 0x82207CD0; continue 'dispatch;
	}
	// 82207BEC: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 82207BF0: 3BCB86B8  addi r30, r11, -0x7948
	ctx.r[30].s64 = ctx.r[11].s64 + -31048;
	// 82207BF4: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82207BF8: 7D7B5A14  add r11, r27, r11
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82207BFC: 557F1838  slwi r31, r11, 3
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82207C00: 7D7FF0AE  lbzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82207C04: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82207C08: 409A00C8  bne cr6, 0x82207cd0
	if !ctx.cr[6].eq {
	pc = 0x82207CD0; continue 'dispatch;
	}
	// 82207C0C: 3BBE0008  addi r29, r30, 8
	ctx.r[29].s64 = ctx.r[30].s64 + 8;
	// 82207C10: 834100C8  lwz r26, 0xc8(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(200 as u32) ) } as u64;
	// 82207C14: 7D7FE82E  lwzx r11, r31, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82207C18: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82207C1C: 409A00B4  bne cr6, 0x82207cd0
	if !ctx.cr[6].eq {
	pc = 0x82207CD0; continue 'dispatch;
	}
	// 82207C20: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207C24: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207C28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82207C2C: 4E800421  bctrl
	ctx.lr = 0x82207C30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82207C30: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82207C34: 481735B5  bl 0x8237b1e8
	ctx.lr = 0x82207C38;
	sub_8237B1E8(ctx, base);
	// 82207C38: 7D7FF0AE  lbzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82207C3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82207C40: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82207C44: 409A0014  bne cr6, 0x82207c58
	if !ctx.cr[6].eq {
	pc = 0x82207C58; continue 'dispatch;
	}
	// 82207C48: 7D7FE82E  lwzx r11, r31, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82207C4C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82207C50: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82207C54: 419A0008  beq cr6, 0x82207c5c
	if ctx.cr[6].eq {
	pc = 0x82207C5C; continue 'dispatch;
	}
	// 82207C58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82207C5C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82207C60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82207C64: 388B43A8  addi r4, r11, 0x43a8
	ctx.r[4].s64 = ctx.r[11].s64 + 17320;
	// 82207C68: 4832AFB9  bl 0x82532c20
	ctx.lr = 0x82207C6C;
	sub_82532C20(ctx, base);
	// 82207C6C: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 82207C70: 3BFC0004  addi r31, r28, 4
	ctx.r[31].s64 = ctx.r[28].s64 + 4;
	// 82207C74: 386BDBE0  addi r3, r11, -0x2420
	ctx.r[3].s64 = ctx.r[11].s64 + -9248;
	// 82207C78: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82207C7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82207C80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82207C84: 4BFED665  bl 0x821f52e8
	ctx.lr = 0x82207C88;
	sub_821F52E8(ctx, base);
	// 82207C88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207C8C: 419A0044  beq cr6, 0x82207cd0
	if ctx.cr[6].eq {
	pc = 0x82207CD0; continue 'dispatch;
	}
	// 82207C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82207C94: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82207C98: 48025751  bl 0x8222d3e8
	ctx.lr = 0x82207C9C;
	sub_8222D3E8(ctx, base);
	// 82207C9C: 481735FD  bl 0x8237b298
	ctx.lr = 0x82207CA0;
	sub_8237B298(ctx, base);
	// 82207CA0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82207CA4: 419A0038  beq cr6, 0x82207cdc
	if ctx.cr[6].eq {
	pc = 0x82207CDC; continue 'dispatch;
	}
	// 82207CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82207CAC: 4802573D  bl 0x8222d3e8
	ctx.lr = 0x82207CB0;
	sub_8222D3E8(ctx, base);
	// 82207CB0: 48173439  bl 0x8237b0e8
	ctx.lr = 0x82207CB4;
	sub_8237B0E8(ctx, base);
	// 82207CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207CB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82207CBC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207CC0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82207CC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207CC8: 419A0008  beq cr6, 0x82207cd0
	if ctx.cr[6].eq {
	pc = 0x82207CD0; continue 'dispatch;
	}
	// 82207CCC: 4BF1538D  bl 0x8211d058
	ctx.lr = 0x82207CD0;
	sub_8211D058(ctx, base);
	// 82207CD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82207CD4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82207CD8: 4832D424  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 82207CDC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207CE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82207CE4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82207CE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82207CEC: 4E800421  bctrl
	ctx.lr = 0x82207CF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82207CF0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82207CF4: 4832D408  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82207CF8 size=176
    let mut pc: u32 = 0x82207CF8;
    'dispatch: loop {
        match pc {
            0x82207CF8 => {
    //   block [0x82207CF8..0x82207DA8)
	// 82207CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82207CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82207D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82207D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82207D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82207D0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82207D10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82207D14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207D18: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82207D1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82207D20: 4E800421  bctrl
	ctx.lr = 0x82207D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82207D24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207D28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207D2C: 409A000C  bne cr6, 0x82207d38
	if !ctx.cr[6].eq {
	pc = 0x82207D38; continue 'dispatch;
	}
	// 82207D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82207D34: 4800002C  b 0x82207d60
	pc = 0x82207D60; continue 'dispatch;
	// 82207D38: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207D3C: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82207D40: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207D44: 419A000C  beq cr6, 0x82207d50
	if ctx.cr[6].eq {
	pc = 0x82207D50; continue 'dispatch;
	}
	// 82207D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82207D4C: 48000014  b 0x82207d60
	pc = 0x82207D60; continue 'dispatch;
	// 82207D50: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207D54: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82207D58: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82207D5C: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82207D60: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82207D64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82207D68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82207D6C: 419A0008  beq cr6, 0x82207d74
	if ctx.cr[6].eq {
	pc = 0x82207D74; continue 'dispatch;
	}
	// 82207D70: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207D74: 48173BF5  bl 0x8237b968
	ctx.lr = 0x82207D78;
	sub_8237B968(ctx, base);
	// 82207D78: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207D7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82207D80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82207D84: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82207D88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82207D8C: 4E800421  bctrl
	ctx.lr = 0x82207D90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82207D90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82207D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82207D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82207D9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82207DA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82207DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82207DA8 size=160
    let mut pc: u32 = 0x82207DA8;
    'dispatch: loop {
        match pc {
            0x82207DA8 => {
    //   block [0x82207DA8..0x82207E48)
	// 82207DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82207DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82207DB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82207DB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82207DB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82207DBC: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82207DC0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82207DC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207DC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207DCC: 409A000C  bne cr6, 0x82207dd8
	if !ctx.cr[6].eq {
	pc = 0x82207DD8; continue 'dispatch;
	}
	// 82207DD0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82207DD4: 4800002C  b 0x82207e00
	pc = 0x82207E00; continue 'dispatch;
	// 82207DD8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207DDC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207DE0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207DE4: 419A000C  beq cr6, 0x82207df0
	if ctx.cr[6].eq {
	pc = 0x82207DF0; continue 'dispatch;
	}
	// 82207DE8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82207DEC: 48000014  b 0x82207e00
	pc = 0x82207E00; continue 'dispatch;
	// 82207DF0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207DF4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82207DF8: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82207DFC: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82207E00: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82207E04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82207E08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82207E0C: 419A0008  beq cr6, 0x82207e14
	if ctx.cr[6].eq {
	pc = 0x82207E14; continue 'dispatch;
	}
	// 82207E10: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207E14: 481732D5  bl 0x8237b0e8
	ctx.lr = 0x82207E18;
	sub_8237B0E8(ctx, base);
	// 82207E18: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82207E1C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207E20: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82207E24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207E28: 419A0008  beq cr6, 0x82207e30
	if ctx.cr[6].eq {
	pc = 0x82207E30; continue 'dispatch;
	}
	// 82207E2C: 4BF1522D  bl 0x8211d058
	ctx.lr = 0x82207E30;
	sub_8211D058(ctx, base);
	// 82207E30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82207E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82207E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82207E3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82207E40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82207E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207E48 size=20
    let mut pc: u32 = 0x82207E48;
    'dispatch: loop {
        match pc {
            0x82207E48 => {
    //   block [0x82207E48..0x82207E5C)
	// 82207E48: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207E4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82207E50: 409A000C  bne cr6, 0x82207e5c
	if !ctx.cr[6].eq {
		sub_82207E5C(ctx, base);
		return;
	}
	// 82207E54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207E58: 4800002C  b 0x82207e84
	sub_82207E74(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207E5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207E5C size=24
    let mut pc: u32 = 0x82207E5C;
    'dispatch: loop {
        match pc {
            0x82207E5C => {
    //   block [0x82207E5C..0x82207E74)
	// 82207E5C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207E60: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82207E64: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207E68: 419A000C  beq cr6, 0x82207e74
	if ctx.cr[6].eq {
		sub_82207E74(ctx, base);
		return;
	}
	// 82207E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207E70: 48000014  b 0x82207e84
	sub_82207E74(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207E74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207E74 size=40
    let mut pc: u32 = 0x82207E74;
    'dispatch: loop {
        match pc {
            0x82207E74 => {
    //   block [0x82207E74..0x82207E9C)
	// 82207E74: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207E78: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82207E7C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82207E80: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82207E84: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82207E88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82207E8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207E90: 419A0008  beq cr6, 0x82207e98
	if ctx.cr[6].eq {
	pc = 0x82207E98; continue 'dispatch;
	}
	// 82207E94: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207E98: 481735D0  b 0x8237b468
	sub_8237B468(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207EA0 size=28
    let mut pc: u32 = 0x82207EA0;
    'dispatch: loop {
        match pc {
            0x82207EA0 => {
    //   block [0x82207EA0..0x82207EBC)
	// 82207EA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82207EA4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82207EA8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207EAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82207EB0: 409A000C  bne cr6, 0x82207ebc
	if !ctx.cr[6].eq {
		sub_82207EBC(ctx, base);
		return;
	}
	// 82207EB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207EB8: 4800002C  b 0x82207ee4
	sub_82207ED4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207EBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207EBC size=24
    let mut pc: u32 = 0x82207EBC;
    'dispatch: loop {
        match pc {
            0x82207EBC => {
    //   block [0x82207EBC..0x82207ED4)
	// 82207EBC: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207EC0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82207EC4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82207EC8: 419A000C  beq cr6, 0x82207ed4
	if ctx.cr[6].eq {
		sub_82207ED4(ctx, base);
		return;
	}
	// 82207ECC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207ED0: 48000014  b 0x82207ee4
	sub_82207ED4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207ED4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207ED4 size=40
    let mut pc: u32 = 0x82207ED4;
    'dispatch: loop {
        match pc {
            0x82207ED4 => {
    //   block [0x82207ED4..0x82207EFC)
	// 82207ED4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207ED8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82207EDC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82207EE0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82207EE4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82207EE8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82207EEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207EF0: 419A0008  beq cr6, 0x82207ef8
	if ctx.cr[6].eq {
	pc = 0x82207EF8; continue 'dispatch;
	}
	// 82207EF4: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207EF8: 48174058  b 0x8237bf50
	sub_8237BF50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207F00 size=28
    let mut pc: u32 = 0x82207F00;
    'dispatch: loop {
        match pc {
            0x82207F00 => {
    //   block [0x82207F00..0x82207F1C)
	// 82207F00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82207F04: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82207F08: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207F0C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82207F10: 409A000C  bne cr6, 0x82207f1c
	if !ctx.cr[6].eq {
		sub_82207F1C(ctx, base);
		return;
	}
	// 82207F14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207F18: 4800002C  b 0x82207f44
	sub_82207F34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207F1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207F1C size=24
    let mut pc: u32 = 0x82207F1C;
    'dispatch: loop {
        match pc {
            0x82207F1C => {
    //   block [0x82207F1C..0x82207F34)
	// 82207F1C: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207F20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82207F24: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82207F28: 419A000C  beq cr6, 0x82207f34
	if ctx.cr[6].eq {
		sub_82207F34(ctx, base);
		return;
	}
	// 82207F2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82207F30: 48000014  b 0x82207f44
	sub_82207F34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207F34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207F34 size=48
    let mut pc: u32 = 0x82207F34;
    'dispatch: loop {
        match pc {
            0x82207F34 => {
    //   block [0x82207F34..0x82207F64)
	// 82207F34: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207F38: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82207F3C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82207F40: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82207F44: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82207F48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82207F4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207F50: 419A0008  beq cr6, 0x82207f58
	if ctx.cr[6].eq {
	pc = 0x82207F58; continue 'dispatch;
	}
	// 82207F54: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207F58: 54CB3032  slwi r11, r6, 6
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82207F5C: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82207F60: 48173EC0  b 0x8237be20
	sub_8237BE20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82207F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82207F68 size=280
    let mut pc: u32 = 0x82207F68;
    'dispatch: loop {
        match pc {
            0x82207F68 => {
    //   block [0x82207F68..0x82208080)
	// 82207F68: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82207F6C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82207F74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82207F78: 409A000C  bne cr6, 0x82207f84
	if !ctx.cr[6].eq {
	pc = 0x82207F84; continue 'dispatch;
	}
	// 82207F7C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82207F80: 4800002C  b 0x82207fac
	pc = 0x82207FAC; continue 'dispatch;
	// 82207F84: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82207F88: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82207F8C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82207F90: 419A000C  beq cr6, 0x82207f9c
	if ctx.cr[6].eq {
	pc = 0x82207F9C; continue 'dispatch;
	}
	// 82207F94: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82207F98: 48000014  b 0x82207fac
	pc = 0x82207FAC; continue 'dispatch;
	// 82207F9C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207FA0: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82207FA4: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82207FA8: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 82207FAC: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82207FB0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82207FB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82207FB8: 419A0008  beq cr6, 0x82207fc0
	if ctx.cr[6].eq {
	pc = 0x82207FC0; continue 'dispatch;
	}
	// 82207FBC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207FC0: 54CB3032  slwi r11, r6, 6
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82207FC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82207FC8: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82207FCC: 419A00AC  beq cr6, 0x82208078
	if ctx.cr[6].eq {
	pc = 0x82208078; continue 'dispatch;
	}
	// 82207FD0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82207FD4: 419A00A4  beq cr6, 0x82208078
	if ctx.cr[6].eq {
	pc = 0x82208078; continue 'dispatch;
	}
	// 82207FD8: 3D204754  lis r9, 0x4754
	ctx.r[9].s64 = 1196687360;
	// 82207FDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207FE0: 61294D57  ori r9, r9, 0x4d57
	ctx.r[9].u64 = ctx.r[9].u64 | 19799;
	// 82207FE4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82207FE8: 409A0090  bne cr6, 0x82208078
	if !ctx.cr[6].eq {
	pc = 0x82208078; continue 'dispatch;
	}
	// 82207FEC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82207FF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82207FF4: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82207FF8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82207FFC: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82208000: 40980034  bge cr6, 0x82208034
	if !ctx.cr[6].lt {
	pc = 0x82208034; continue 'dispatch;
	}
	// 82208004: 80E40008  lwz r7, 8(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82208008: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8220800C: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82208010: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82208014: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82208018: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8220801C: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82208020: 98CB0002  stb r6, 2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 82208024: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82208028: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8220802C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82208030: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82208034: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82208038: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8220803C: 80A30024  lwz r5, 0x24(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82208040: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82208044: 40980034  bge cr6, 0x82208078
	if !ctx.cr[6].lt {
	pc = 0x82208078; continue 'dispatch;
	}
	// 82208048: 80E40008  lwz r7, 8(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8220804C: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82208050: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82208054: 7D683A14  add r11, r8, r7
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82208058: 90640000  stw r3, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8220805C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82208060: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82208064: 98CB0002  stb r6, 2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 82208068: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 8220806C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82208070: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82208074: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82208078: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8220807C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82208080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82208080 size=116
    let mut pc: u32 = 0x82208080;
    'dispatch: loop {
        match pc {
            0x82208080 => {
    //   block [0x82208080..0x822080F4)
	// 82208080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82208084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82208088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8220808C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82208090: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82208094: 409A000C  bne cr6, 0x822080a0
	if !ctx.cr[6].eq {
	pc = 0x822080A0; continue 'dispatch;
	}
	// 82208098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8220809C: 4800002C  b 0x822080c8
	pc = 0x822080C8; continue 'dispatch;
	// 822080A0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822080A4: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822080A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822080AC: 419A000C  beq cr6, 0x822080b8
	if ctx.cr[6].eq {
	pc = 0x822080B8; continue 'dispatch;
	}
	// 822080B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822080B4: 48000014  b 0x822080c8
	pc = 0x822080C8; continue 'dispatch;
	// 822080B8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822080BC: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 822080C0: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 822080C4: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 822080C8: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 822080CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822080D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822080D4: 419A0008  beq cr6, 0x822080dc
	if ctx.cr[6].eq {
	pc = 0x822080DC; continue 'dispatch;
	}
	// 822080D8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822080DC: 4817388D  bl 0x8237b968
	ctx.lr = 0x822080E0;
	sub_8237B968(ctx, base);
	// 822080E0: 48173F29  bl 0x8237c008
	ctx.lr = 0x822080E4;
	sub_8237C008(ctx, base);
	// 822080E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822080E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822080EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822080F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822080F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822080F8 size=132
    let mut pc: u32 = 0x822080F8;
    'dispatch: loop {
        match pc {
            0x822080F8 => {
    //   block [0x822080F8..0x8220817C)
	// 822080F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822080FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82208100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82208104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82208108: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8220810C: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82208110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82208114: 409A000C  bne cr6, 0x82208120
	if !ctx.cr[6].eq {
	pc = 0x82208120; continue 'dispatch;
	}
	// 82208118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8220811C: 4800002C  b 0x82208148
	pc = 0x82208148; continue 'dispatch;
	// 82208120: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82208124: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82208128: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8220812C: 419A000C  beq cr6, 0x82208138
	if ctx.cr[6].eq {
	pc = 0x82208138; continue 'dispatch;
	}
	// 82208130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82208134: 48000014  b 0x82208148
	pc = 0x82208148; continue 'dispatch;
	// 82208138: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220813C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82208140: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82208144: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82208148: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8220814C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82208150: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82208154: 419A0008  beq cr6, 0x8220815c
	if ctx.cr[6].eq {
	pc = 0x8220815C; continue 'dispatch;
	}
	// 82208158: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8220815C: 4817380D  bl 0x8237b968
	ctx.lr = 0x82208160;
	sub_8237B968(ctx, base);
	// 82208160: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82208164: 48173FFD  bl 0x8237c160
	ctx.lr = 0x82208168;
	sub_8237C160(ctx, base);
	// 82208168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8220816C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82208170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82208174: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82208178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82208180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82208180 size=20
    let mut pc: u32 = 0x82208180;
    'dispatch: loop {
        match pc {
            0x82208180 => {
    //   block [0x82208180..0x82208194)
	// 82208180: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82208184: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82208188: 409A000C  bne cr6, 0x82208194
	if !ctx.cr[6].eq {
		sub_82208194(ctx, base);
		return;
	}
	// 8220818C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82208190: 4800002C  b 0x822081bc
	sub_822081AC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82208194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82208194 size=24
    let mut pc: u32 = 0x82208194;
    'dispatch: loop {
        match pc {
            0x82208194 => {
    //   block [0x82208194..0x822081AC)
	// 82208194: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82208198: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8220819C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822081A0: 419A000C  beq cr6, 0x822081ac
	if ctx.cr[6].eq {
		sub_822081AC(ctx, base);
		return;
	}
	// 822081A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822081A8: 48000014  b 0x822081bc
	sub_822081AC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822081AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822081AC size=40
    let mut pc: u32 = 0x822081AC;
    'dispatch: loop {
        match pc {
            0x822081AC => {
    //   block [0x822081AC..0x822081D4)
	// 822081AC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822081B0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822081B4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822081B8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 822081BC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 822081C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822081C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822081C8: 419A0008  beq cr6, 0x822081d0
	if ctx.cr[6].eq {
	pc = 0x822081D0; continue 'dispatch;
	}
	// 822081CC: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822081D0: 48174110  b 0x8237c2e0
	sub_8237C2E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822081D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822081D8 size=20
    let mut pc: u32 = 0x822081D8;
    'dispatch: loop {
        match pc {
            0x822081D8 => {
    //   block [0x822081D8..0x822081EC)
	// 822081D8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822081DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822081E0: 409A000C  bne cr6, 0x822081ec
	if !ctx.cr[6].eq {
		sub_822081EC(ctx, base);
		return;
	}
	// 822081E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822081E8: 4800002C  b 0x82208214
	sub_82208204(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822081EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822081EC size=24
    let mut pc: u32 = 0x822081EC;
    'dispatch: loop {
        match pc {
            0x822081EC => {
    //   block [0x822081EC..0x82208204)
	// 822081EC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822081F0: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822081F4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822081F8: 419A000C  beq cr6, 0x82208204
	if ctx.cr[6].eq {
		sub_82208204(ctx, base);
		return;
	}
	// 822081FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82208200: 48000014  b 0x82208214
	sub_82208204(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82208204(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82208204 size=40
    let mut pc: u32 = 0x82208204;
    'dispatch: loop {
        match pc {
            0x82208204 => {
    //   block [0x82208204..0x8220822C)
	// 82208204: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82208208: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8220820C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82208210: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82208214: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82208218: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8220821C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82208220: 419A0008  beq cr6, 0x82208228
	if ctx.cr[6].eq {
	pc = 0x82208228; continue 'dispatch;
	}
	// 82208224: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82208228: 481741C8  b 0x8237c3f0
	sub_8237C3F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82208230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82208230 size=20
    let mut pc: u32 = 0x82208230;
    'dispatch: loop {
        match pc {
            0x82208230 => {
    //   block [0x82208230..0x82208244)
	// 82208230: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82208234: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82208238: 409A000C  bne cr6, 0x82208244
	if !ctx.cr[6].eq {
		sub_82208244(ctx, base);
		return;
	}
	// 8220823C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82208240: 4800002C  b 0x8220826c
	sub_8220825C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82208244(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82208244 size=24
    let mut pc: u32 = 0x82208244;
    'dispatch: loop {
        match pc {
            0x82208244 => {
    //   block [0x82208244..0x8220825C)
	// 82208244: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82208248: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8220824C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82208250: 419A000C  beq cr6, 0x8220825c
	if ctx.cr[6].eq {
		sub_8220825C(ctx, base);
		return;
	}
	// 82208254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82208258: 48000014  b 0x8220826c
	sub_8220825C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8220825C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8220825C size=40
    let mut pc: u32 = 0x8220825C;
    'dispatch: loop {
        match pc {
            0x8220825C => {
    //   block [0x8220825C..0x82208284)
	// 8220825C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82208260: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82208264: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82208268: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8220826C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82208270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82208274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82208278: 419A0008  beq cr6, 0x82208280
	if ctx.cr[6].eq {
	pc = 0x82208280; continue 'dispatch;
	}
	// 8220827C: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82208280: 48174290  b 0x8237c510
	sub_8237C510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


