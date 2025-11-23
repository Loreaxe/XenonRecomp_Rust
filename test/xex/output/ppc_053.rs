pub fn sub_8248EA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248EA40 size=300
    let mut pc: u32 = 0x8248EA40;
    'dispatch: loop {
        match pc {
            0x8248EA40 => {
    //   block [0x8248EA40..0x8248EA94)
	// 8248EA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248EA44: 480A6679  bl 0x825350bc
	ctx.lr = 0x8248EA48;
	sub_82535080(ctx, base);
	// 8248EA48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248EA4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248EA50: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8248EA54: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8248EA58: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248EA5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EA60: 409A0074  bne cr6, 0x8248ead4
	if !ctx.cr[6].eq {
	pc = 0x8248EAD4; continue 'dispatch;
	}
	// 8248EA64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EA68: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248EA6C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248EA70: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8248EA74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EA78: 419A001C  beq cr6, 0x8248ea94
	if ctx.cr[6].eq {
	pc = 0x8248EA94; continue 'dispatch;
	}
	// 8248EA7C: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8248EA80: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8248EA84: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8248EA88: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EA8C: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8248EA90: 48000010  b 0x8248eaa0
	pc = 0x8248EAA0; continue 'dispatch;
            }
            0x8248EA94 => {
    //   block [0x8248EA94..0x8248EAA0)
	// 8248EA94: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8248EA98: 4BFD53A9  bl 0x82463e40
	ctx.lr = 0x8248EA9C;
	sub_82463E40(ctx, base);
	// 8248EA9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x8248EAA0; continue 'dispatch;
            }
            0x8248EAA0 => {
    //   block [0x8248EAA0..0x8248EACC)
	// 8248EAA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EAA4: 419A0028  beq cr6, 0x8248eacc
	if ctx.cr[6].eq {
	pc = 0x8248EACC; continue 'dispatch;
	}
	// 8248EAA8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8248EAAC: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8248EAB0: B3EB0004  sth r31, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u16 ) };
	// 8248EAB4: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 8248EAB8: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8248EABC: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8248EAC0: B3EB000C  sth r31, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u16 ) };
	// 8248EAC4: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 8248EAC8: 48000008  b 0x8248ead0
	pc = 0x8248EAD0; continue 'dispatch;
            }
            0x8248EACC => {
    //   block [0x8248EACC..0x8248EAD0)
	// 8248EACC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8248EAD0; continue 'dispatch;
            }
            0x8248EAD0 => {
    //   block [0x8248EAD0..0x8248EAD4)
	// 8248EAD0: 917E00CC  stw r11, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	pc = 0x8248EAD4; continue 'dispatch;
            }
            0x8248EAD4 => {
    //   block [0x8248EAD4..0x8248EAF4)
	// 8248EAD4: 815E00CC  lwz r10, 0xcc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248EAD8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8248EADC: 3BEA0008  addi r31, r10, 8
	ctx.r[31].s64 = ctx.r[10].s64 + 8;
	// 8248EAE0: A13F0004  lhz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248EAE4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8248EAE8: 4099002C  ble cr6, 0x8248eb14
	if !ctx.cr[6].gt {
	pc = 0x8248EB14; continue 'dispatch;
	}
	// 8248EAEC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EAF0: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	pc = 0x8248EAF4; continue 'dispatch;
            }
            0x8248EAF4 => {
    //   block [0x8248EAF4..0x8248EB14)
	// 8248EAF4: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EAF8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8248EAFC: 419A0058  beq cr6, 0x8248eb54
	if ctx.cr[6].eq {
	pc = 0x8248EB54; continue 'dispatch;
	}
	// 8248EB00: A0FF0004  lhz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248EB04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248EB08: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248EB0C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8248EB10: 4198FFE4  blt cr6, 0x8248eaf4
	if ctx.cr[6].lt {
	pc = 0x8248EAF4; continue 'dispatch;
	}
	pc = 0x8248EB14; continue 'dispatch;
            }
            0x8248EB14 => {
    //   block [0x8248EB14..0x8248EB30)
	// 8248EB14: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248EB18: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 8248EB1C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248EB20: 409A0010  bne cr6, 0x8248eb30
	if !ctx.cr[6].eq {
	pc = 0x8248EB30; continue 'dispatch;
	}
	// 8248EB24: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8248EB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248EB2C: 4811A2A5  bl 0x825a8dd0
	ctx.lr = 0x8248EB30;
	sub_825A8DD0(ctx, base);
	pc = 0x8248EB30; continue 'dispatch;
            }
            0x8248EB30 => {
    //   block [0x8248EB30..0x8248EB54)
	// 8248EB30: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248EB34: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EB38: 556B103E  rotlwi r11, r11, 2
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 8248EB3C: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 8248EB40: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248EB44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248EB48: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8248EB4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248EB50: 480A65BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8248EB54 => {
    //   block [0x8248EB54..0x8248EB6C)
	// 8248EB54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248EB58: 4198FFBC  blt cr6, 0x8248eb14
	if ctx.cr[6].lt {
	pc = 0x8248EB14; continue 'dispatch;
	}
	// 8248EB5C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8248EB60: 7FAB412E  stwx r29, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[29].u32) };
	// 8248EB64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248EB68: 480A65A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248EB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248EB70 size=224
    let mut pc: u32 = 0x8248EB70;
    'dispatch: loop {
        match pc {
            0x8248EB70 => {
    //   block [0x8248EB70..0x8248EBC4)
	// 8248EB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248EB74: 480A6549  bl 0x825350bc
	ctx.lr = 0x8248EB78;
	sub_82535080(ctx, base);
	// 8248EB78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248EB7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248EB80: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8248EB84: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8248EB88: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248EB8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EB90: 409A0074  bne cr6, 0x8248ec04
	if !ctx.cr[6].eq {
	pc = 0x8248EC04; continue 'dispatch;
	}
	// 8248EB94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EB98: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248EB9C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248EBA0: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8248EBA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EBA8: 419A001C  beq cr6, 0x8248ebc4
	if ctx.cr[6].eq {
	pc = 0x8248EBC4; continue 'dispatch;
	}
	// 8248EBAC: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8248EBB0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8248EBB4: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8248EBB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EBBC: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8248EBC0: 48000010  b 0x8248ebd0
	pc = 0x8248EBD0; continue 'dispatch;
            }
            0x8248EBC4 => {
    //   block [0x8248EBC4..0x8248EBD0)
	// 8248EBC4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8248EBC8: 4BFD5279  bl 0x82463e40
	ctx.lr = 0x8248EBCC;
	sub_82463E40(ctx, base);
	// 8248EBCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x8248EBD0; continue 'dispatch;
            }
            0x8248EBD0 => {
    //   block [0x8248EBD0..0x8248EBFC)
	// 8248EBD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EBD4: 419A0028  beq cr6, 0x8248ebfc
	if ctx.cr[6].eq {
	pc = 0x8248EBFC; continue 'dispatch;
	}
	// 8248EBD8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8248EBDC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8248EBE0: B3CB0004  sth r30, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 8248EBE4: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 8248EBE8: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8248EBEC: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8248EBF0: B3CB000C  sth r30, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u16 ) };
	// 8248EBF4: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 8248EBF8: 48000008  b 0x8248ec00
	pc = 0x8248EC00; continue 'dispatch;
            }
            0x8248EBFC => {
    //   block [0x8248EBFC..0x8248EC00)
	// 8248EBFC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8248EC00; continue 'dispatch;
            }
            0x8248EC00 => {
    //   block [0x8248EC00..0x8248EC04)
	// 8248EC00: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	pc = 0x8248EC04; continue 'dispatch;
            }
            0x8248EC04 => {
    //   block [0x8248EC04..0x8248EC1C)
	// 8248EC04: 811F00CC  lwz r8, 0xcc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248EC08: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8248EC0C: A128000C  lhz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248EC10: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8248EC14: 40990024  ble cr6, 0x8248ec38
	if !ctx.cr[6].gt {
	pc = 0x8248EC38; continue 'dispatch;
	}
	// 8248EC18: 81480008  lwz r10, 8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x8248EC1C; continue 'dispatch;
            }
            0x8248EC1C => {
    //   block [0x8248EC1C..0x8248EC38)
	// 8248EC1C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EC20: 7F07E840  cmplw cr6, r7, r29
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8248EC24: 419A0018  beq cr6, 0x8248ec3c
	if ctx.cr[6].eq {
	pc = 0x8248EC3C; continue 'dispatch;
	}
	// 8248EC28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248EC2C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248EC30: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8248EC34: 4198FFE8  blt cr6, 0x8248ec1c
	if ctx.cr[6].lt {
	pc = 0x8248EC1C; continue 'dispatch;
	}
	pc = 0x8248EC38; continue 'dispatch;
            }
            0x8248EC38 => {
    //   block [0x8248EC38..0x8248EC3C)
	// 8248EC38: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x8248EC3C; continue 'dispatch;
            }
            0x8248EC3C => {
    //   block [0x8248EC3C..0x8248EC50)
	// 8248EC3C: 81480008  lwz r10, 8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248EC40: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8248EC44: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 8248EC48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248EC4C: 480A64C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248EC50 size=296
    let mut pc: u32 = 0x8248EC50;
    'dispatch: loop {
        match pc {
            0x8248EC50 => {
    //   block [0x8248EC50..0x8248ECA4)
	// 8248EC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248EC54: 480A6469  bl 0x825350bc
	ctx.lr = 0x8248EC58;
	sub_82535080(ctx, base);
	// 8248EC58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248EC5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248EC60: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8248EC64: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8248EC68: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248EC6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EC70: 409A0074  bne cr6, 0x8248ece4
	if !ctx.cr[6].eq {
	pc = 0x8248ECE4; continue 'dispatch;
	}
	// 8248EC74: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EC78: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248EC7C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248EC80: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8248EC84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EC88: 419A001C  beq cr6, 0x8248eca4
	if ctx.cr[6].eq {
	pc = 0x8248ECA4; continue 'dispatch;
	}
	// 8248EC8C: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8248EC90: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8248EC94: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8248EC98: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EC9C: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8248ECA0: 48000010  b 0x8248ecb0
	pc = 0x8248ECB0; continue 'dispatch;
            }
            0x8248ECA4 => {
    //   block [0x8248ECA4..0x8248ECB0)
	// 8248ECA4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8248ECA8: 4BFD5199  bl 0x82463e40
	ctx.lr = 0x8248ECAC;
	sub_82463E40(ctx, base);
	// 8248ECAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x8248ECB0; continue 'dispatch;
            }
            0x8248ECB0 => {
    //   block [0x8248ECB0..0x8248ECDC)
	// 8248ECB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248ECB4: 419A0028  beq cr6, 0x8248ecdc
	if ctx.cr[6].eq {
	pc = 0x8248ECDC; continue 'dispatch;
	}
	// 8248ECB8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8248ECBC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8248ECC0: B3CB0004  sth r30, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 8248ECC4: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 8248ECC8: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8248ECCC: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8248ECD0: B3CB000C  sth r30, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u16 ) };
	// 8248ECD4: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 8248ECD8: 48000008  b 0x8248ece0
	pc = 0x8248ECE0; continue 'dispatch;
            }
            0x8248ECDC => {
    //   block [0x8248ECDC..0x8248ECE0)
	// 8248ECDC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8248ECE0; continue 'dispatch;
            }
            0x8248ECE0 => {
    //   block [0x8248ECE0..0x8248ECE4)
	// 8248ECE0: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	pc = 0x8248ECE4; continue 'dispatch;
            }
            0x8248ECE4 => {
    //   block [0x8248ECE4..0x8248ED00)
	// 8248ECE4: 83FF00CC  lwz r31, 0xcc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248ECE8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8248ECEC: A11F0004  lhz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ECF0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8248ECF4: 4099002C  ble cr6, 0x8248ed20
	if !ctx.cr[6].gt {
	pc = 0x8248ED20; continue 'dispatch;
	}
	// 8248ECF8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248ECFC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	pc = 0x8248ED00; continue 'dispatch;
            }
            0x8248ED00 => {
    //   block [0x8248ED00..0x8248ED20)
	// 8248ED00: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248ED04: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8248ED08: 419A0058  beq cr6, 0x8248ed60
	if ctx.cr[6].eq {
	pc = 0x8248ED60; continue 'dispatch;
	}
	// 8248ED0C: A0FF0004  lhz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ED10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248ED14: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248ED18: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8248ED1C: 4198FFE4  blt cr6, 0x8248ed00
	if ctx.cr[6].lt {
	pc = 0x8248ED00; continue 'dispatch;
	}
	pc = 0x8248ED20; continue 'dispatch;
            }
            0x8248ED20 => {
    //   block [0x8248ED20..0x8248ED3C)
	// 8248ED20: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248ED24: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 8248ED28: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248ED2C: 409A0010  bne cr6, 0x8248ed3c
	if !ctx.cr[6].eq {
	pc = 0x8248ED3C; continue 'dispatch;
	}
	// 8248ED30: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8248ED34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248ED38: 4811A099  bl 0x825a8dd0
	ctx.lr = 0x8248ED3C;
	sub_825A8DD0(ctx, base);
	pc = 0x8248ED3C; continue 'dispatch;
            }
            0x8248ED3C => {
    //   block [0x8248ED3C..0x8248ED60)
	// 8248ED3C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ED40: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248ED44: 556B103E  rotlwi r11, r11, 2
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 8248ED48: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 8248ED4C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ED50: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248ED54: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8248ED58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248ED5C: 480A63B0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8248ED60 => {
    //   block [0x8248ED60..0x8248ED78)
	// 8248ED60: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248ED64: 4198FFBC  blt cr6, 0x8248ed20
	if ctx.cr[6].lt {
	pc = 0x8248ED20; continue 'dispatch;
	}
	// 8248ED68: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8248ED6C: 7FAB492E  stwx r29, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u32) };
	// 8248ED70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248ED74: 480A6398  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248ED78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248ED78 size=1112
    let mut pc: u32 = 0x8248ED78;
    'dispatch: loop {
        match pc {
            0x8248ED78 => {
    //   block [0x8248ED78..0x8248EDCC)
	// 8248ED78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248ED7C: 480A6335  bl 0x825350b0
	ctx.lr = 0x8248ED80;
	sub_82535080(ctx, base);
	// 8248ED80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248ED84: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8248ED88: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8248ED8C: 3BFC005C  addi r31, r28, 0x5c
	ctx.r[31].s64 = ctx.r[28].s64 + 92;
	// 8248ED90: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8248ED94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ED98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248ED9C: 409A0044  bne cr6, 0x8248ede0
	if !ctx.cr[6].eq {
	pc = 0x8248EDE0; continue 'dispatch;
	}
	// 8248EDA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248EDA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248EDA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248EDAC: 409A0020  bne cr6, 0x8248edcc
	if !ctx.cr[6].eq {
	pc = 0x8248EDCC; continue 'dispatch;
	}
	// 8248EDB0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EDB4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248EDB8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248EDBC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EDC0: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8248EDC4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248EDC8: 4BFD52F1  bl 0x824640b8
	ctx.lr = 0x8248EDCC;
	sub_824640B8(ctx, base);
	pc = 0x8248EDCC; continue 'dispatch;
            }
            0x8248EDCC => {
    //   block [0x8248EDCC..0x8248EDE0)
	// 8248EDCC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248EDD0: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8248EDD4: 534BF880  rlwimi r11, r26, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[26].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 8248EDD8: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8248EDDC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x8248EDE0; continue 'dispatch;
            }
            0x8248EDE0 => {
    //   block [0x8248EDE0..0x8248EE28)
	// 8248EDE0: A17C009C  lhz r11, 0x9c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(156 as u32) ) } as u64;
	// 8248EDE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248EDE8: 409A0058  bne cr6, 0x8248ee40
	if !ctx.cr[6].eq {
	pc = 0x8248EE40; continue 'dispatch;
	}
	// 8248EDEC: 3BFC0098  addi r31, r28, 0x98
	ctx.r[31].s64 = ctx.r[28].s64 + 152;
	// 8248EDF0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248EDF4: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248EDF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248EDFC: 409A002C  bne cr6, 0x8248ee28
	if !ctx.cr[6].eq {
	pc = 0x8248EE28; continue 'dispatch;
	}
	// 8248EE00: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EE04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248EE08: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 8248EE0C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EE10: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248EE14: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248EE18: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8248EE1C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248EE20: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8248EE24: 4BFD5295  bl 0x824640b8
	ctx.lr = 0x8248EE28;
	sub_824640B8(ctx, base);
	pc = 0x8248EE28; continue 'dispatch;
            }
            0x8248EE28 => {
    //   block [0x8248EE28..0x8248EE40)
	// 8248EE28: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248EE2C: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8248EE30: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248EE34: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 8248EE38: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 8248EE3C: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x8248EE40; continue 'dispatch;
            }
            0x8248EE40 => {
    //   block [0x8248EE40..0x8248EE7C)
	// 8248EE40: 817C00A4  lwz r11, 0xa4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(164 as u32) ) } as u64;
	// 8248EE44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248EE48: 409A0048  bne cr6, 0x8248ee90
	if !ctx.cr[6].eq {
	pc = 0x8248EE90; continue 'dispatch;
	}
	// 8248EE4C: 3BFC00A0  addi r31, r28, 0xa0
	ctx.r[31].s64 = ctx.r[28].s64 + 160;
	// 8248EE50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248EE54: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248EE58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248EE5C: 409A0020  bne cr6, 0x8248ee7c
	if !ctx.cr[6].eq {
	pc = 0x8248EE7C; continue 'dispatch;
	}
	// 8248EE60: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EE64: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248EE68: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248EE6C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EE70: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8248EE74: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248EE78: 4BFD5241  bl 0x824640b8
	ctx.lr = 0x8248EE7C;
	sub_824640B8(ctx, base);
	pc = 0x8248EE7C; continue 'dispatch;
            }
            0x8248EE7C => {
    //   block [0x8248EE7C..0x8248EE90)
	// 8248EE7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248EE80: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8248EE84: 534BF880  rlwimi r11, r26, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[26].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 8248EE88: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8248EE8C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x8248EE90; continue 'dispatch;
            }
            0x8248EE90 => {
    //   block [0x8248EE90..0x8248EECC)
	// 8248EE90: 817C00B0  lwz r11, 0xb0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(176 as u32) ) } as u64;
	// 8248EE94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248EE98: 409A0048  bne cr6, 0x8248eee0
	if !ctx.cr[6].eq {
	pc = 0x8248EEE0; continue 'dispatch;
	}
	// 8248EE9C: 3BFC00AC  addi r31, r28, 0xac
	ctx.r[31].s64 = ctx.r[28].s64 + 172;
	// 8248EEA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248EEA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248EEA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248EEAC: 409A0020  bne cr6, 0x8248eecc
	if !ctx.cr[6].eq {
	pc = 0x8248EECC; continue 'dispatch;
	}
	// 8248EEB0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EEB4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248EEB8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248EEBC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EEC0: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248EEC4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248EEC8: 4BFD51F1  bl 0x824640b8
	ctx.lr = 0x8248EECC;
	sub_824640B8(ctx, base);
	pc = 0x8248EECC; continue 'dispatch;
            }
            0x8248EECC => {
    //   block [0x8248EECC..0x8248EEE0)
	// 8248EECC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248EED0: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8248EED4: 534BF880  rlwimi r11, r26, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[26].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 8248EED8: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8248EEDC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x8248EEE0; continue 'dispatch;
            }
            0x8248EEE0 => {
    //   block [0x8248EEE0..0x8248EEF8)
	// 8248EEE0: A15C01F4  lhz r10, 0x1f4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(500 as u32) ) } as u64;
	// 8248EEE4: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8248EEE8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8248EEEC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248EEF0: 419A0030  beq cr6, 0x8248ef20
	if ctx.cr[6].eq {
	pc = 0x8248EF20; continue 'dispatch;
	}
	// 8248EEF4: 815C01F0  lwz r10, 0x1f0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(496 as u32) ) } as u64;
	pc = 0x8248EEF8; continue 'dispatch;
            }
            0x8248EEF8 => {
    //   block [0x8248EEF8..0x8248EF1C)
	// 8248EEF8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EEFC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8248EF00: 409A001C  bne cr6, 0x8248ef1c
	if !ctx.cr[6].eq {
	pc = 0x8248EF1C; continue 'dispatch;
	}
	// 8248EF04: A11C01F4  lhz r8, 0x1f4(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(500 as u32) ) } as u64;
	// 8248EF08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248EF0C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248EF10: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8248EF14: 4198FFE4  blt cr6, 0x8248eef8
	if ctx.cr[6].lt {
	pc = 0x8248EEF8; continue 'dispatch;
	}
	// 8248EF18: 48000008  b 0x8248ef20
	pc = 0x8248EF20; continue 'dispatch;
            }
            0x8248EF1C => {
    //   block [0x8248EF1C..0x8248EF20)
	// 8248EF1C: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	pc = 0x8248EF20; continue 'dispatch;
            }
            0x8248EF20 => {
    //   block [0x8248EF20..0x8248EF5C)
	// 8248EF20: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8248EF24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248EF28: 409A004C  bne cr6, 0x8248ef74
	if !ctx.cr[6].eq {
	pc = 0x8248EF74; continue 'dispatch;
	}
	// 8248EF2C: 3BFC01F0  addi r31, r28, 0x1f0
	ctx.r[31].s64 = ctx.r[28].s64 + 496;
	// 8248EF30: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248EF34: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248EF38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248EF3C: 409A0020  bne cr6, 0x8248ef5c
	if !ctx.cr[6].eq {
	pc = 0x8248EF5C; continue 'dispatch;
	}
	// 8248EF40: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EF44: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248EF48: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248EF4C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EF50: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248EF54: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248EF58: 4BFD5161  bl 0x824640b8
	ctx.lr = 0x8248EF5C;
	sub_824640B8(ctx, base);
	pc = 0x8248EF5C; continue 'dispatch;
            }
            0x8248EF5C => {
    //   block [0x8248EF5C..0x8248EF74)
	// 8248EF5C: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248EF60: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8248EF64: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248EF68: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 8248EF6C: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 8248EF70: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x8248EF74; continue 'dispatch;
            }
            0x8248EF74 => {
    //   block [0x8248EF74..0x8248EF98)
	// 8248EF74: 83FC00CC  lwz r31, 0xcc(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248EF78: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8248EF7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248EF80: 419A0090  beq cr6, 0x8248f010
	if ctx.cr[6].eq {
	pc = 0x8248F010; continue 'dispatch;
	}
	// 8248EF84: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248EF88: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8248EF8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248EF90: 419A0030  beq cr6, 0x8248efc0
	if ctx.cr[6].eq {
	pc = 0x8248EFC0; continue 'dispatch;
	}
	// 8248EF94: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x8248EF98; continue 'dispatch;
            }
            0x8248EF98 => {
    //   block [0x8248EF98..0x8248EFBC)
	// 8248EF98: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EF9C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248EFA0: 409A001C  bne cr6, 0x8248efbc
	if !ctx.cr[6].eq {
	pc = 0x8248EFBC; continue 'dispatch;
	}
	// 8248EFA4: A13F0004  lhz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248EFA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248EFAC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248EFB0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8248EFB4: 4198FFE4  blt cr6, 0x8248ef98
	if ctx.cr[6].lt {
	pc = 0x8248EF98; continue 'dispatch;
	}
	// 8248EFB8: 48000008  b 0x8248efc0
	pc = 0x8248EFC0; continue 'dispatch;
            }
            0x8248EFBC => {
    //   block [0x8248EFBC..0x8248EFC0)
	// 8248EFBC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	pc = 0x8248EFC0; continue 'dispatch;
            }
            0x8248EFC0 => {
    //   block [0x8248EFC0..0x8248EFF8)
	// 8248EFC0: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 8248EFC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248EFC8: 409A0048  bne cr6, 0x8248f010
	if !ctx.cr[6].eq {
	pc = 0x8248F010; continue 'dispatch;
	}
	// 8248EFCC: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248EFD0: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248EFD4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248EFD8: 409A0020  bne cr6, 0x8248eff8
	if !ctx.cr[6].eq {
	pc = 0x8248EFF8; continue 'dispatch;
	}
	// 8248EFDC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EFE0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248EFE4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248EFE8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248EFEC: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248EFF0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248EFF4: 4BFD50C5  bl 0x824640b8
	ctx.lr = 0x8248EFF8;
	sub_824640B8(ctx, base);
	pc = 0x8248EFF8; continue 'dispatch;
            }
            0x8248EFF8 => {
    //   block [0x8248EFF8..0x8248F010)
	// 8248EFF8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248EFFC: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8248F000: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F004: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 8248F008: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 8248F00C: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x8248F010; continue 'dispatch;
            }
            0x8248F010 => {
    //   block [0x8248F010..0x8248F038)
	// 8248F010: 817C00CC  lwz r11, 0xcc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248F014: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8248F018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248F01C: 419A0094  beq cr6, 0x8248f0b0
	if ctx.cr[6].eq {
	pc = 0x8248F0B0; continue 'dispatch;
	}
	// 8248F020: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 8248F024: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8248F028: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248F02C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F030: 419A0030  beq cr6, 0x8248f060
	if ctx.cr[6].eq {
	pc = 0x8248F060; continue 'dispatch;
	}
	// 8248F034: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x8248F038; continue 'dispatch;
            }
            0x8248F038 => {
    //   block [0x8248F038..0x8248F05C)
	// 8248F038: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F03C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248F040: 409A001C  bne cr6, 0x8248f05c
	if !ctx.cr[6].eq {
	pc = 0x8248F05C; continue 'dispatch;
	}
	// 8248F044: A13F0004  lhz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248F048: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248F04C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248F050: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8248F054: 4198FFE4  blt cr6, 0x8248f038
	if ctx.cr[6].lt {
	pc = 0x8248F038; continue 'dispatch;
	}
	// 8248F058: 48000008  b 0x8248f060
	pc = 0x8248F060; continue 'dispatch;
            }
            0x8248F05C => {
    //   block [0x8248F05C..0x8248F060)
	// 8248F05C: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	pc = 0x8248F060; continue 'dispatch;
            }
            0x8248F060 => {
    //   block [0x8248F060..0x8248F098)
	// 8248F060: 7FCB0774  extsb r11, r30
	ctx.r[11].s64 = ctx.r[30].s8 as i64;
	// 8248F064: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248F068: 409A0048  bne cr6, 0x8248f0b0
	if !ctx.cr[6].eq {
	pc = 0x8248F0B0; continue 'dispatch;
	}
	// 8248F06C: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248F070: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F074: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F078: 409A0020  bne cr6, 0x8248f098
	if !ctx.cr[6].eq {
	pc = 0x8248F098; continue 'dispatch;
	}
	// 8248F07C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F080: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F084: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F088: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F08C: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F090: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F094: 4BFD5025  bl 0x824640b8
	ctx.lr = 0x8248F098;
	sub_824640B8(ctx, base);
	pc = 0x8248F098; continue 'dispatch;
            }
            0x8248F098 => {
    //   block [0x8248F098..0x8248F0B0)
	// 8248F098: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248F09C: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8248F0A0: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F0A4: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 8248F0A8: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 8248F0AC: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x8248F0B0; continue 'dispatch;
            }
            0x8248F0B0 => {
    //   block [0x8248F0B0..0x8248F100)
	// 8248F0B0: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 8248F0B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248F0B8: 409A00BC  bne cr6, 0x8248f174
	if !ctx.cr[6].eq {
	pc = 0x8248F174; continue 'dispatch;
	}
	// 8248F0BC: 7FCB0774  extsb r11, r30
	ctx.r[11].s64 = ctx.r[30].s8 as i64;
	// 8248F0C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248F0C4: 409A00B0  bne cr6, 0x8248f174
	if !ctx.cr[6].eq {
	pc = 0x8248F174; continue 'dispatch;
	}
	// 8248F0C8: 83FC00CC  lwz r31, 0xcc(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248F0CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248F0D0: 419A00A0  beq cr6, 0x8248f170
	if ctx.cr[6].eq {
	pc = 0x8248F170; continue 'dispatch;
	}
	// 8248F0D4: A17F000E  lhz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 8248F0D8: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F0DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F0E0: 409A0020  bne cr6, 0x8248f100
	if !ctx.cr[6].eq {
	pc = 0x8248F100; continue 'dispatch;
	}
	// 8248F0E4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F0E8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F0EC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F0F0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248F0F4: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F0F8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F0FC: 4BFD4FBD  bl 0x824640b8
	ctx.lr = 0x8248F100;
	sub_824640B8(ctx, base);
	pc = 0x8248F100; continue 'dispatch;
            }
            0x8248F100 => {
    //   block [0x8248F100..0x8248F12C)
	// 8248F100: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248F104: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F108: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F10C: 409A0020  bne cr6, 0x8248f12c
	if !ctx.cr[6].eq {
	pc = 0x8248F12C; continue 'dispatch;
	}
	// 8248F110: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F114: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F118: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F11C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F120: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F124: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F128: 4BFD4F91  bl 0x824640b8
	ctx.lr = 0x8248F12C;
	sub_824640B8(ctx, base);
	pc = 0x8248F12C; continue 'dispatch;
            }
            0x8248F12C => {
    //   block [0x8248F12C..0x8248F158)
	// 8248F12C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F130: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248F134: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248F138: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8248F13C: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8248F140: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8248F144: 41980014  blt cr6, 0x8248f158
	if ctx.cr[6].lt {
	pc = 0x8248F158; continue 'dispatch;
	}
	// 8248F148: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8248F14C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8248F150: 4BFD4DC9  bl 0x82463f18
	ctx.lr = 0x8248F154;
	sub_82463F18(ctx, base);
	// 8248F154: 4800001C  b 0x8248f170
	pc = 0x8248F170; continue 'dispatch;
            }
            0x8248F158 => {
    //   block [0x8248F158..0x8248F170)
	// 8248F158: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8248F15C: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8248F160: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8248F164: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8248F168: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248F16C: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	pc = 0x8248F170; continue 'dispatch;
            }
            0x8248F170 => {
    //   block [0x8248F170..0x8248F174)
	// 8248F170: 937C00CC  stw r27, 0xcc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(204 as u32), ctx.r[27].u32 ) };
	pc = 0x8248F174; continue 'dispatch;
            }
            0x8248F174 => {
    //   block [0x8248F174..0x8248F1B0)
	// 8248F174: A17C01FC  lhz r11, 0x1fc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(508 as u32) ) } as u64;
	// 8248F178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248F17C: 409A004C  bne cr6, 0x8248f1c8
	if !ctx.cr[6].eq {
	pc = 0x8248F1C8; continue 'dispatch;
	}
	// 8248F180: 3BFC01F8  addi r31, r28, 0x1f8
	ctx.r[31].s64 = ctx.r[28].s64 + 504;
	// 8248F184: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248F188: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F18C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F190: 409A0020  bne cr6, 0x8248f1b0
	if !ctx.cr[6].eq {
	pc = 0x8248F1B0; continue 'dispatch;
	}
	// 8248F194: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F198: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F19C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F1A0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F1A4: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F1A8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F1AC: 4BFD4F0D  bl 0x824640b8
	ctx.lr = 0x8248F1B0;
	sub_824640B8(ctx, base);
	pc = 0x8248F1B0; continue 'dispatch;
            }
            0x8248F1B0 => {
    //   block [0x8248F1B0..0x8248F1C8)
	// 8248F1B0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248F1B4: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8248F1B8: 556B0462  rlwinm r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F1BC: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 8248F1C0: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 8248F1C4: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x8248F1C8; continue 'dispatch;
            }
            0x8248F1C8 => {
    //   block [0x8248F1C8..0x8248F1D0)
	// 8248F1C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8248F1CC: 480A5F34  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F1D0 size=324
    let mut pc: u32 = 0x8248F1D0;
    'dispatch: loop {
        match pc {
            0x8248F1D0 => {
    //   block [0x8248F1D0..0x8248F208)
	// 8248F1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F1D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F1DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F1E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248F1E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F1E8: 396BCD7C  addi r11, r11, -0x3284
	ctx.r[11].s64 = ctx.r[11].s64 + -12932;
	// 8248F1EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248F1F0: 48013339  bl 0x824a2528
	ctx.lr = 0x8248F1F4;
	sub_824A2528(ctx, base);
	// 8248F1F4: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8248F1F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248F1FC: 419A000C  beq cr6, 0x8248f208
	if ctx.cr[6].eq {
	pc = 0x8248F208; continue 'dispatch;
	}
	// 8248F200: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248F204: 480003A5  bl 0x8248f5a8
	ctx.lr = 0x8248F208;
	sub_8248F5A8(ctx, base);
	pc = 0x8248F208; continue 'dispatch;
            }
            0x8248F208 => {
    //   block [0x8248F208..0x8248F234)
	// 8248F208: A17F01FE  lhz r11, 0x1fe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(510 as u32) ) } as u64;
	// 8248F20C: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F210: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F214: 409A0020  bne cr6, 0x8248f234
	if !ctx.cr[6].eq {
	pc = 0x8248F234; continue 'dispatch;
	}
	// 8248F218: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F21C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F220: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F224: 809F01F8  lwz r4, 0x1f8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 8248F228: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F22C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F230: 4BFD4E89  bl 0x824640b8
	ctx.lr = 0x8248F234;
	sub_824640B8(ctx, base);
	pc = 0x8248F234; continue 'dispatch;
            }
            0x8248F234 => {
    //   block [0x8248F234..0x8248F260)
	// 8248F234: A17F01F6  lhz r11, 0x1f6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(502 as u32) ) } as u64;
	// 8248F238: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F23C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F240: 409A0020  bne cr6, 0x8248f260
	if !ctx.cr[6].eq {
	pc = 0x8248F260; continue 'dispatch;
	}
	// 8248F244: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F248: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F24C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F250: 809F01F0  lwz r4, 0x1f0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(496 as u32) ) } as u64;
	// 8248F254: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F258: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F25C: 4BFD4E5D  bl 0x824640b8
	ctx.lr = 0x8248F260;
	sub_824640B8(ctx, base);
	pc = 0x8248F260; continue 'dispatch;
            }
            0x8248F260 => {
    //   block [0x8248F260..0x8248F294)
	// 8248F260: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8248F264: 4800CF1D  bl 0x8249c180
	ctx.lr = 0x8248F268;
	sub_8249C180(ctx, base);
	// 8248F268: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8248F26C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F270: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248F274: 409A0020  bne cr6, 0x8248f294
	if !ctx.cr[6].eq {
	pc = 0x8248F294; continue 'dispatch;
	}
	// 8248F278: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F27C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F280: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F284: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8248F288: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F28C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F290: 4BFD4E29  bl 0x824640b8
	ctx.lr = 0x8248F294;
	sub_824640B8(ctx, base);
	pc = 0x8248F294; continue 'dispatch;
            }
            0x8248F294 => {
    //   block [0x8248F294..0x8248F2C0)
	// 8248F294: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 8248F298: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F29C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248F2A0: 409A0020  bne cr6, 0x8248f2c0
	if !ctx.cr[6].eq {
	pc = 0x8248F2C0; continue 'dispatch;
	}
	// 8248F2A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F2A8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F2AC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F2B0: 809F00A0  lwz r4, 0xa0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8248F2B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8248F2B8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F2BC: 4BFD4DFD  bl 0x824640b8
	ctx.lr = 0x8248F2C0;
	sub_824640B8(ctx, base);
	pc = 0x8248F2C0; continue 'dispatch;
            }
            0x8248F2C0 => {
    //   block [0x8248F2C0..0x8248F2F8)
	// 8248F2C0: A17F009E  lhz r11, 0x9e(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(158 as u32) ) } as u64;
	// 8248F2C4: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F2C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F2CC: 409A002C  bne cr6, 0x8248f2f8
	if !ctx.cr[6].eq {
	pc = 0x8248F2F8; continue 'dispatch;
	}
	// 8248F2D0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F2D4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F2D8: 556B04BE  clrlwi r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 8248F2DC: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8248F2E0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F2E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F2E8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8248F2EC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248F2F0: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8248F2F4: 4BFD4DC5  bl 0x824640b8
	ctx.lr = 0x8248F2F8;
	sub_824640B8(ctx, base);
	pc = 0x8248F2F8; continue 'dispatch;
            }
            0x8248F2F8 => {
    //   block [0x8248F2F8..0x8248F314)
	// 8248F2F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248F2FC: 48000165  bl 0x8248f460
	ctx.lr = 0x8248F300;
	sub_8248F460(ctx, base);
	// 8248F300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248F304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248F308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248F30C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248F310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F318 size=96
    let mut pc: u32 = 0x8248F318;
    'dispatch: loop {
        match pc {
            0x8248F318 => {
    //   block [0x8248F318..0x8248F360)
	// 8248F318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F320: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F328: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248F32C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F330: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8248F334: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8248F338: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F33C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248F340: 419A0020  beq cr6, 0x8248f360
	if ctx.cr[6].eq {
	pc = 0x8248F360; continue 'dispatch;
	}
	// 8248F344: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F348: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248F34C: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 8248F350: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248F354: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248F358: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248F35C: 4BFD4D5D  bl 0x824640b8
	ctx.lr = 0x8248F360;
	sub_824640B8(ctx, base);
	pc = 0x8248F360; continue 'dispatch;
            }
            0x8248F360 => {
    //   block [0x8248F360..0x8248F378)
	// 8248F360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248F364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248F368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248F36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248F370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248F374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F378 size=100
    let mut pc: u32 = 0x8248F378;
    'dispatch: loop {
        match pc {
            0x8248F378 => {
    //   block [0x8248F378..0x8248F3C0)
	// 8248F378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248F384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F390: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248F394: 4800CDED  bl 0x8249c180
	ctx.lr = 0x8248F398;
	sub_8249C180(ctx, base);
	// 8248F398: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8248F39C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248F3A0: 419A0020  beq cr6, 0x8248f3c0
	if ctx.cr[6].eq {
	pc = 0x8248F3C0; continue 'dispatch;
	}
	// 8248F3A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F3A8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248F3AC: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 8248F3B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248F3B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248F3B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248F3BC: 4BFD4CFD  bl 0x824640b8
	ctx.lr = 0x8248F3C0;
	sub_824640B8(ctx, base);
	pc = 0x8248F3C0; continue 'dispatch;
            }
            0x8248F3C0 => {
    //   block [0x8248F3C0..0x8248F3DC)
	// 8248F3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248F3C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248F3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248F3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248F3D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248F3D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248F3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F3E0 size=128
    let mut pc: u32 = 0x8248F3E0;
    'dispatch: loop {
        match pc {
            0x8248F3E0 => {
    //   block [0x8248F3E0..0x8248F420)
	// 8248F3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F3E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F3EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F3F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F3F4: A17F000E  lhz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 8248F3F8: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F3FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F400: 409A0020  bne cr6, 0x8248f420
	if !ctx.cr[6].eq {
	pc = 0x8248F420; continue 'dispatch;
	}
	// 8248F404: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F408: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F40C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F410: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248F414: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F418: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F41C: 4BFD4C9D  bl 0x824640b8
	ctx.lr = 0x8248F420;
	sub_824640B8(ctx, base);
	pc = 0x8248F420; continue 'dispatch;
            }
            0x8248F420 => {
    //   block [0x8248F420..0x8248F44C)
	// 8248F420: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248F424: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F428: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F42C: 409A0020  bne cr6, 0x8248f44c
	if !ctx.cr[6].eq {
	pc = 0x8248F44C; continue 'dispatch;
	}
	// 8248F430: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F434: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F438: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F43C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F440: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8248F444: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F448: 4BFD4C71  bl 0x824640b8
	ctx.lr = 0x8248F44C;
	sub_824640B8(ctx, base);
	pc = 0x8248F44C; continue 'dispatch;
            }
            0x8248F44C => {
    //   block [0x8248F44C..0x8248F460)
	// 8248F44C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248F450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248F454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248F458: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248F45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F460 size=220
    let mut pc: u32 = 0x8248F460;
    'dispatch: loop {
        match pc {
            0x8248F460 => {
    //   block [0x8248F460..0x8248F4C4)
	// 8248F460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F46C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F470: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F474: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248F478: 396BCD64  addi r11, r11, -0x329c
	ctx.r[11].s64 = ctx.r[11].s64 + -12956;
	// 8248F47C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248F480: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8248F484: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248F488: 419A003C  beq cr6, 0x8248f4c4
	if ctx.cr[6].eq {
	pc = 0x8248F4C4; continue 'dispatch;
	}
	// 8248F48C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248F490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248F494: 419A0030  beq cr6, 0x8248f4c4
	if ctx.cr[6].eq {
	pc = 0x8248F4C4; continue 'dispatch;
	}
	// 8248F498: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8248F49C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8248F4A0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8248F4A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248F4A8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8248F4AC: 409A0018  bne cr6, 0x8248f4c4
	if !ctx.cr[6].eq {
	pc = 0x8248F4C4; continue 'dispatch;
	}
	// 8248F4B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F4B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248F4B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F4BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248F4C0: 4E800421  bctrl
	ctx.lr = 0x8248F4C4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8248F4C4 => {
    //   block [0x8248F4C4..0x8248F4F0)
	// 8248F4C4: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8248F4C8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F4CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248F4D0: 409A0020  bne cr6, 0x8248f4f0
	if !ctx.cr[6].eq {
	pc = 0x8248F4F0; continue 'dispatch;
	}
	// 8248F4D4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F4D8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F4DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F4E0: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8248F4E4: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8248F4E8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F4EC: 4BFD4BCD  bl 0x824640b8
	ctx.lr = 0x8248F4F0;
	sub_824640B8(ctx, base);
	pc = 0x8248F4F0; continue 'dispatch;
            }
            0x8248F4F0 => {
    //   block [0x8248F4F0..0x8248F51C)
	// 8248F4F0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8248F4F4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8248F4F8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248F4FC: 409A0020  bne cr6, 0x8248f51c
	if !ctx.cr[6].eq {
	pc = 0x8248F51C; continue 'dispatch;
	}
	// 8248F500: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F504: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8248F508: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8248F50C: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8248F510: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8248F514: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8248F518: 4BFD4BA1  bl 0x824640b8
	ctx.lr = 0x8248F51C;
	sub_824640B8(ctx, base);
	pc = 0x8248F51C; continue 'dispatch;
            }
            0x8248F51C => {
    //   block [0x8248F51C..0x8248F53C)
	// 8248F51C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248F520: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8248F524: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248F528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248F52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248F530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248F534: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248F538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F540 size=100
    let mut pc: u32 = 0x8248F540;
    'dispatch: loop {
        match pc {
            0x8248F540 => {
    //   block [0x8248F540..0x8248F588)
	// 8248F540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248F54C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F558: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248F55C: 4BFFFF05  bl 0x8248f460
	ctx.lr = 0x8248F560;
	sub_8248F460(ctx, base);
	// 8248F560: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8248F564: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248F568: 419A0020  beq cr6, 0x8248f588
	if ctx.cr[6].eq {
	pc = 0x8248F588; continue 'dispatch;
	}
	// 8248F56C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F570: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248F574: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8248F578: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248F57C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248F580: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248F584: 4BFD4B35  bl 0x824640b8
	ctx.lr = 0x8248F588;
	sub_824640B8(ctx, base);
	pc = 0x8248F588; continue 'dispatch;
            }
            0x8248F588 => {
    //   block [0x8248F588..0x8248F5A4)
	// 8248F588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248F58C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248F590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248F594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248F598: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248F59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248F5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F5A8 size=152
    let mut pc: u32 = 0x8248F5A8;
    'dispatch: loop {
        match pc {
            0x8248F5A8 => {
    //   block [0x8248F5A8..0x8248F60C)
	// 8248F5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F5B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248F5B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F5B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F5BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F5C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248F5C4: 4BFFFE1D  bl 0x8248f3e0
	ctx.lr = 0x8248F5C8;
	sub_8248F3E0(ctx, base);
	// 8248F5C8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8248F5CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248F5D0: 419A0054  beq cr6, 0x8248f624
	if ctx.cr[6].eq {
	pc = 0x8248F624; continue 'dispatch;
	}
	// 8248F5D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248F5D8: 419A004C  beq cr6, 0x8248f624
	if ctx.cr[6].eq {
	pc = 0x8248F624; continue 'dispatch;
	}
	// 8248F5DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F5E0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248F5E4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248F5E8: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8248F5EC: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8248F5F0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8248F5F4: 41980018  blt cr6, 0x8248f60c
	if ctx.cr[6].lt {
	pc = 0x8248F60C; continue 'dispatch;
	}
	// 8248F5F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8248F5FC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8248F600: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8248F604: 4BFD4915  bl 0x82463f18
	ctx.lr = 0x8248F608;
	sub_82463F18(ctx, base);
	// 8248F608: 4800001C  b 0x8248f624
	pc = 0x8248F624; continue 'dispatch;
            }
            0x8248F60C => {
    //   block [0x8248F60C..0x8248F624)
	// 8248F60C: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8248F610: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8248F614: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8248F618: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8248F61C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248F620: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	pc = 0x8248F624; continue 'dispatch;
            }
            0x8248F624 => {
    //   block [0x8248F624..0x8248F640)
	// 8248F624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248F628: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248F62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248F630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248F634: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248F638: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F640 size=100
    let mut pc: u32 = 0x8248F640;
    'dispatch: loop {
        match pc {
            0x8248F640 => {
    //   block [0x8248F640..0x8248F688)
	// 8248F640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F648: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248F64C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F658: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248F65C: 4BFFFB75  bl 0x8248f1d0
	ctx.lr = 0x8248F660;
	sub_8248F1D0(ctx, base);
	// 8248F660: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8248F664: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248F668: 419A0020  beq cr6, 0x8248f688
	if ctx.cr[6].eq {
	pc = 0x8248F688; continue 'dispatch;
	}
	// 8248F66C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F670: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248F674: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 8248F678: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248F67C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248F680: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248F684: 4BFD4A35  bl 0x824640b8
	ctx.lr = 0x8248F688;
	sub_824640B8(ctx, base);
	pc = 0x8248F688; continue 'dispatch;
            }
            0x8248F688 => {
    //   block [0x8248F688..0x8248F6A4)
	// 8248F688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248F68C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248F690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248F694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248F698: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248F69C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248F6A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F6A8 size=4
    let mut pc: u32 = 0x8248F6A8;
    'dispatch: loop {
        match pc {
            0x8248F6A8 => {
    //   block [0x8248F6A8..0x8248F6AC)
	// 8248F6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F6B0 size=20
    let mut pc: u32 = 0x8248F6B0;
    'dispatch: loop {
        match pc {
            0x8248F6B0 => {
    //   block [0x8248F6B0..0x8248F6C4)
	// 8248F6B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F6B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8248F6B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F6BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248F6C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F6C8 size=4
    let mut pc: u32 = 0x8248F6C8;
    'dispatch: loop {
        match pc {
            0x8248F6C8 => {
    //   block [0x8248F6C8..0x8248F6CC)
	// 8248F6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F6D0 size=32
    let mut pc: u32 = 0x8248F6D0;
    'dispatch: loop {
        match pc {
            0x8248F6D0 => {
    //   block [0x8248F6D0..0x8248F6F0)
	// 8248F6D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248F6D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8248F6D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248F6DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248F6E0: 396BCDE0  addi r11, r11, -0x3220
	ctx.r[11].s64 = ctx.r[11].s64 + -12832;
	// 8248F6E4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8248F6E8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248F6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F6F0 size=12
    let mut pc: u32 = 0x8248F6F0;
    'dispatch: loop {
        match pc {
            0x8248F6F0 => {
    //   block [0x8248F6F0..0x8248F6FC)
	// 8248F6F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248F6F4: 386BCDE0  addi r3, r11, -0x3220
	ctx.r[3].s64 = ctx.r[11].s64 + -12832;
	// 8248F6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F700 size=4
    let mut pc: u32 = 0x8248F700;
    'dispatch: loop {
        match pc {
            0x8248F700 => {
    //   block [0x8248F700..0x8248F704)
	// 8248F700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F708 size=4
    let mut pc: u32 = 0x8248F708;
    'dispatch: loop {
        match pc {
            0x8248F708 => {
    //   block [0x8248F708..0x8248F70C)
	// 8248F708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F710 size=4
    let mut pc: u32 = 0x8248F710;
    'dispatch: loop {
        match pc {
            0x8248F710 => {
    //   block [0x8248F710..0x8248F714)
	// 8248F710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F718 size=16
    let mut pc: u32 = 0x8248F718;
    'dispatch: loop {
        match pc {
            0x8248F718 => {
    //   block [0x8248F718..0x8248F728)
	// 8248F718: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248F71C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248F720: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8248F724: 48013934  b 0x824a3058
	sub_824A3058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F728 size=4
    let mut pc: u32 = 0x8248F728;
    'dispatch: loop {
        match pc {
            0x8248F728 => {
    //   block [0x8248F728..0x8248F72C)
	// 8248F728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F730 size=4
    let mut pc: u32 = 0x8248F730;
    'dispatch: loop {
        match pc {
            0x8248F730 => {
    //   block [0x8248F730..0x8248F734)
	// 8248F730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F738 size=4
    let mut pc: u32 = 0x8248F738;
    'dispatch: loop {
        match pc {
            0x8248F738 => {
    //   block [0x8248F738..0x8248F73C)
	// 8248F738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F740 size=4
    let mut pc: u32 = 0x8248F740;
    'dispatch: loop {
        match pc {
            0x8248F740 => {
    //   block [0x8248F740..0x8248F744)
	// 8248F740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F748 size=4
    let mut pc: u32 = 0x8248F748;
    'dispatch: loop {
        match pc {
            0x8248F748 => {
    //   block [0x8248F748..0x8248F74C)
	// 8248F748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F750 size=4
    let mut pc: u32 = 0x8248F750;
    'dispatch: loop {
        match pc {
            0x8248F750 => {
    //   block [0x8248F750..0x8248F754)
	// 8248F750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F758 size=4
    let mut pc: u32 = 0x8248F758;
    'dispatch: loop {
        match pc {
            0x8248F758 => {
    //   block [0x8248F758..0x8248F75C)
	// 8248F758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F760 size=4
    let mut pc: u32 = 0x8248F760;
    'dispatch: loop {
        match pc {
            0x8248F760 => {
    //   block [0x8248F760..0x8248F764)
	// 8248F760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F768 size=4
    let mut pc: u32 = 0x8248F768;
    'dispatch: loop {
        match pc {
            0x8248F768 => {
    //   block [0x8248F768..0x8248F76C)
	// 8248F768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F770 size=4
    let mut pc: u32 = 0x8248F770;
    'dispatch: loop {
        match pc {
            0x8248F770 => {
    //   block [0x8248F770..0x8248F774)
	// 8248F770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F778 size=4
    let mut pc: u32 = 0x8248F778;
    'dispatch: loop {
        match pc {
            0x8248F778 => {
    //   block [0x8248F778..0x8248F77C)
	// 8248F778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F780 size=4
    let mut pc: u32 = 0x8248F780;
    'dispatch: loop {
        match pc {
            0x8248F780 => {
    //   block [0x8248F780..0x8248F784)
	// 8248F780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F788 size=4
    let mut pc: u32 = 0x8248F788;
    'dispatch: loop {
        match pc {
            0x8248F788 => {
    //   block [0x8248F788..0x8248F78C)
	// 8248F788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F790 size=4
    let mut pc: u32 = 0x8248F790;
    'dispatch: loop {
        match pc {
            0x8248F790 => {
    //   block [0x8248F790..0x8248F794)
	// 8248F790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F798 size=4
    let mut pc: u32 = 0x8248F798;
    'dispatch: loop {
        match pc {
            0x8248F798 => {
    //   block [0x8248F798..0x8248F79C)
	// 8248F798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7A0 size=4
    let mut pc: u32 = 0x8248F7A0;
    'dispatch: loop {
        match pc {
            0x8248F7A0 => {
    //   block [0x8248F7A0..0x8248F7A4)
	// 8248F7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7A8 size=4
    let mut pc: u32 = 0x8248F7A8;
    'dispatch: loop {
        match pc {
            0x8248F7A8 => {
    //   block [0x8248F7A8..0x8248F7AC)
	// 8248F7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7B0 size=4
    let mut pc: u32 = 0x8248F7B0;
    'dispatch: loop {
        match pc {
            0x8248F7B0 => {
    //   block [0x8248F7B0..0x8248F7B4)
	// 8248F7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7B8 size=4
    let mut pc: u32 = 0x8248F7B8;
    'dispatch: loop {
        match pc {
            0x8248F7B8 => {
    //   block [0x8248F7B8..0x8248F7BC)
	// 8248F7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7C0 size=4
    let mut pc: u32 = 0x8248F7C0;
    'dispatch: loop {
        match pc {
            0x8248F7C0 => {
    //   block [0x8248F7C0..0x8248F7C4)
	// 8248F7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7C8 size=4
    let mut pc: u32 = 0x8248F7C8;
    'dispatch: loop {
        match pc {
            0x8248F7C8 => {
    //   block [0x8248F7C8..0x8248F7CC)
	// 8248F7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7D0 size=4
    let mut pc: u32 = 0x8248F7D0;
    'dispatch: loop {
        match pc {
            0x8248F7D0 => {
    //   block [0x8248F7D0..0x8248F7D4)
	// 8248F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7D8 size=4
    let mut pc: u32 = 0x8248F7D8;
    'dispatch: loop {
        match pc {
            0x8248F7D8 => {
    //   block [0x8248F7D8..0x8248F7DC)
	// 8248F7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7E0 size=4
    let mut pc: u32 = 0x8248F7E0;
    'dispatch: loop {
        match pc {
            0x8248F7E0 => {
    //   block [0x8248F7E0..0x8248F7E4)
	// 8248F7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7E8 size=4
    let mut pc: u32 = 0x8248F7E8;
    'dispatch: loop {
        match pc {
            0x8248F7E8 => {
    //   block [0x8248F7E8..0x8248F7EC)
	// 8248F7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7F0 size=4
    let mut pc: u32 = 0x8248F7F0;
    'dispatch: loop {
        match pc {
            0x8248F7F0 => {
    //   block [0x8248F7F0..0x8248F7F4)
	// 8248F7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F7F8 size=4
    let mut pc: u32 = 0x8248F7F8;
    'dispatch: loop {
        match pc {
            0x8248F7F8 => {
    //   block [0x8248F7F8..0x8248F7FC)
	// 8248F7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F800 size=4
    let mut pc: u32 = 0x8248F800;
    'dispatch: loop {
        match pc {
            0x8248F800 => {
    //   block [0x8248F800..0x8248F804)
	// 8248F800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F808 size=4
    let mut pc: u32 = 0x8248F808;
    'dispatch: loop {
        match pc {
            0x8248F808 => {
    //   block [0x8248F808..0x8248F80C)
	// 8248F808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F810 size=16
    let mut pc: u32 = 0x8248F810;
    'dispatch: loop {
        match pc {
            0x8248F810 => {
    //   block [0x8248F810..0x8248F820)
	// 8248F810: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248F814: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8248F818: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248F81C: 4801369C  b 0x824a2eb8
	sub_824A2EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F820 size=4
    let mut pc: u32 = 0x8248F820;
    'dispatch: loop {
        match pc {
            0x8248F820 => {
    //   block [0x8248F820..0x8248F824)
	// 8248F820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F828 size=16
    let mut pc: u32 = 0x8248F828;
    'dispatch: loop {
        match pc {
            0x8248F828 => {
    //   block [0x8248F828..0x8248F838)
	// 8248F828: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248F82C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8248F830: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248F834: 48013684  b 0x824a2eb8
	sub_824A2EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F838 size=4
    let mut pc: u32 = 0x8248F838;
    'dispatch: loop {
        match pc {
            0x8248F838 => {
    //   block [0x8248F838..0x8248F83C)
	// 8248F838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F840 size=4
    let mut pc: u32 = 0x8248F840;
    'dispatch: loop {
        match pc {
            0x8248F840 => {
    //   block [0x8248F840..0x8248F844)
	// 8248F840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F848 size=4
    let mut pc: u32 = 0x8248F848;
    'dispatch: loop {
        match pc {
            0x8248F848 => {
    //   block [0x8248F848..0x8248F84C)
	// 8248F848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F850 size=4
    let mut pc: u32 = 0x8248F850;
    'dispatch: loop {
        match pc {
            0x8248F850 => {
    //   block [0x8248F850..0x8248F854)
	// 8248F850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F858 size=4
    let mut pc: u32 = 0x8248F858;
    'dispatch: loop {
        match pc {
            0x8248F858 => {
    //   block [0x8248F858..0x8248F85C)
	// 8248F858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F860 size=4
    let mut pc: u32 = 0x8248F860;
    'dispatch: loop {
        match pc {
            0x8248F860 => {
    //   block [0x8248F860..0x8248F864)
	// 8248F860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F868 size=4
    let mut pc: u32 = 0x8248F868;
    'dispatch: loop {
        match pc {
            0x8248F868 => {
    //   block [0x8248F868..0x8248F86C)
	// 8248F868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F870 size=4
    let mut pc: u32 = 0x8248F870;
    'dispatch: loop {
        match pc {
            0x8248F870 => {
    //   block [0x8248F870..0x8248F874)
	// 8248F870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F878 size=4
    let mut pc: u32 = 0x8248F878;
    'dispatch: loop {
        match pc {
            0x8248F878 => {
    //   block [0x8248F878..0x8248F87C)
	// 8248F878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F880 size=4
    let mut pc: u32 = 0x8248F880;
    'dispatch: loop {
        match pc {
            0x8248F880 => {
    //   block [0x8248F880..0x8248F884)
	// 8248F880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F888 size=4
    let mut pc: u32 = 0x8248F888;
    'dispatch: loop {
        match pc {
            0x8248F888 => {
    //   block [0x8248F888..0x8248F88C)
	// 8248F888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F890 size=4
    let mut pc: u32 = 0x8248F890;
    'dispatch: loop {
        match pc {
            0x8248F890 => {
    //   block [0x8248F890..0x8248F894)
	// 8248F890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F898 size=4
    let mut pc: u32 = 0x8248F898;
    'dispatch: loop {
        match pc {
            0x8248F898 => {
    //   block [0x8248F898..0x8248F89C)
	// 8248F898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8A0 size=4
    let mut pc: u32 = 0x8248F8A0;
    'dispatch: loop {
        match pc {
            0x8248F8A0 => {
    //   block [0x8248F8A0..0x8248F8A4)
	// 8248F8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8A8 size=4
    let mut pc: u32 = 0x8248F8A8;
    'dispatch: loop {
        match pc {
            0x8248F8A8 => {
    //   block [0x8248F8A8..0x8248F8AC)
	// 8248F8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8B0 size=4
    let mut pc: u32 = 0x8248F8B0;
    'dispatch: loop {
        match pc {
            0x8248F8B0 => {
    //   block [0x8248F8B0..0x8248F8B4)
	// 8248F8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8B8 size=4
    let mut pc: u32 = 0x8248F8B8;
    'dispatch: loop {
        match pc {
            0x8248F8B8 => {
    //   block [0x8248F8B8..0x8248F8BC)
	// 8248F8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8C0 size=4
    let mut pc: u32 = 0x8248F8C0;
    'dispatch: loop {
        match pc {
            0x8248F8C0 => {
    //   block [0x8248F8C0..0x8248F8C4)
	// 8248F8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8C8 size=4
    let mut pc: u32 = 0x8248F8C8;
    'dispatch: loop {
        match pc {
            0x8248F8C8 => {
    //   block [0x8248F8C8..0x8248F8CC)
	// 8248F8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8D0 size=4
    let mut pc: u32 = 0x8248F8D0;
    'dispatch: loop {
        match pc {
            0x8248F8D0 => {
    //   block [0x8248F8D0..0x8248F8D4)
	// 8248F8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8D8 size=4
    let mut pc: u32 = 0x8248F8D8;
    'dispatch: loop {
        match pc {
            0x8248F8D8 => {
    //   block [0x8248F8D8..0x8248F8DC)
	// 8248F8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8E0 size=4
    let mut pc: u32 = 0x8248F8E0;
    'dispatch: loop {
        match pc {
            0x8248F8E0 => {
    //   block [0x8248F8E0..0x8248F8E4)
	// 8248F8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8E8 size=4
    let mut pc: u32 = 0x8248F8E8;
    'dispatch: loop {
        match pc {
            0x8248F8E8 => {
    //   block [0x8248F8E8..0x8248F8EC)
	// 8248F8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8F0 size=4
    let mut pc: u32 = 0x8248F8F0;
    'dispatch: loop {
        match pc {
            0x8248F8F0 => {
    //   block [0x8248F8F0..0x8248F8F4)
	// 8248F8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F8F8 size=4
    let mut pc: u32 = 0x8248F8F8;
    'dispatch: loop {
        match pc {
            0x8248F8F8 => {
    //   block [0x8248F8F8..0x8248F8FC)
	// 8248F8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F900 size=4
    let mut pc: u32 = 0x8248F900;
    'dispatch: loop {
        match pc {
            0x8248F900 => {
    //   block [0x8248F900..0x8248F904)
	// 8248F900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F908 size=4
    let mut pc: u32 = 0x8248F908;
    'dispatch: loop {
        match pc {
            0x8248F908 => {
    //   block [0x8248F908..0x8248F90C)
	// 8248F908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F910 size=4
    let mut pc: u32 = 0x8248F910;
    'dispatch: loop {
        match pc {
            0x8248F910 => {
    //   block [0x8248F910..0x8248F914)
	// 8248F910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F918 size=20
    let mut pc: u32 = 0x8248F918;
    'dispatch: loop {
        match pc {
            0x8248F918 => {
    //   block [0x8248F918..0x8248F92C)
	// 8248F918: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F91C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8248F920: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248F928: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F930 size=4
    let mut pc: u32 = 0x8248F930;
    'dispatch: loop {
        match pc {
            0x8248F930 => {
    //   block [0x8248F930..0x8248F934)
	// 8248F930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F938 size=32
    let mut pc: u32 = 0x8248F938;
    'dispatch: loop {
        match pc {
            0x8248F938 => {
    //   block [0x8248F938..0x8248F958)
	// 8248F938: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248F93C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8248F940: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248F944: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248F948: 396BD854  addi r11, r11, -0x27ac
	ctx.r[11].s64 = ctx.r[11].s64 + -10156;
	// 8248F94C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8248F950: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248F954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F958 size=12
    let mut pc: u32 = 0x8248F958;
    'dispatch: loop {
        match pc {
            0x8248F958 => {
    //   block [0x8248F958..0x8248F964)
	// 8248F958: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248F95C: 386BD854  addi r3, r11, -0x27ac
	ctx.r[3].s64 = ctx.r[11].s64 + -10156;
	// 8248F960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F968 size=20
    let mut pc: u32 = 0x8248F968;
    'dispatch: loop {
        match pc {
            0x8248F968 => {
    //   block [0x8248F968..0x8248F97C)
	// 8248F968: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F96C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8248F970: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F974: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248F978: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F980 size=32
    let mut pc: u32 = 0x8248F980;
    'dispatch: loop {
        match pc {
            0x8248F980 => {
    //   block [0x8248F980..0x8248F9A0)
	// 8248F980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248F984: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8248F988: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248F98C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248F990: 396BCA84  addi r11, r11, -0x357c
	ctx.r[11].s64 = ctx.r[11].s64 + -13692;
	// 8248F994: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8248F998: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248F99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248F9A0 size=12
    let mut pc: u32 = 0x8248F9A0;
    'dispatch: loop {
        match pc {
            0x8248F9A0 => {
    //   block [0x8248F9A0..0x8248F9AC)
	// 8248F9A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248F9A4: 386BCA84  addi r3, r11, -0x357c
	ctx.r[3].s64 = ctx.r[11].s64 + -13692;
	// 8248F9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248F9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248F9B0 size=96
    let mut pc: u32 = 0x8248F9B0;
    'dispatch: loop {
        match pc {
            0x8248F9B0 => {
    //   block [0x8248F9B0..0x8248F9F8)
	// 8248F9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248F9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248F9B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248F9BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248F9C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248F9C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248F9C8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8248F9CC: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8248F9D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248F9D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248F9D8: 419A0020  beq cr6, 0x8248f9f8
	if ctx.cr[6].eq {
	pc = 0x8248F9F8; continue 'dispatch;
	}
	// 8248F9DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248F9E0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8248F9E4: 38C00029  li r6, 0x29
	ctx.r[6].s64 = 41;
	// 8248F9E8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248F9EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248F9F0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248F9F4: 4BFD46C5  bl 0x824640b8
	ctx.lr = 0x8248F9F8;
	sub_824640B8(ctx, base);
	pc = 0x8248F9F8; continue 'dispatch;
            }
            0x8248F9F8 => {
    //   block [0x8248F9F8..0x8248FA10)
	// 8248F9F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248F9FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248FA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248FA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248FA08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248FA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FA10 size=32
    let mut pc: u32 = 0x8248FA10;
    'dispatch: loop {
        match pc {
            0x8248FA10 => {
    //   block [0x8248FA10..0x8248FA30)
	// 8248FA10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248FA14: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8248FA18: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248FA1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248FA20: 396BD8A0  addi r11, r11, -0x2760
	ctx.r[11].s64 = ctx.r[11].s64 + -10080;
	// 8248FA24: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8248FA28: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FA30 size=20
    let mut pc: u32 = 0x8248FA30;
    'dispatch: loop {
        match pc {
            0x8248FA30 => {
    //   block [0x8248FA30..0x8248FA44)
	// 8248FA30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248FA34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8248FA38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248FA3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248FA40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FA48 size=12
    let mut pc: u32 = 0x8248FA48;
    'dispatch: loop {
        match pc {
            0x8248FA48 => {
    //   block [0x8248FA48..0x8248FA54)
	// 8248FA48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248FA4C: 386BD8A0  addi r3, r11, -0x2760
	ctx.r[3].s64 = ctx.r[11].s64 + -10080;
	// 8248FA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248FA58 size=80
    let mut pc: u32 = 0x8248FA58;
    'dispatch: loop {
        match pc {
            0x8248FA58 => {
    //   block [0x8248FA58..0x8248FAA8)
	// 8248FA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248FA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248FA60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248FA64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248FA68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248FA6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248FA70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248FA74: 4BFFE78D  bl 0x8248e200
	ctx.lr = 0x8248FA78;
	sub_8248E200(ctx, base);
	// 8248FA78: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8248FA7C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8248FA80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248FA84: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8248FA88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248FA8C: 4E800421  bctrl
	ctx.lr = 0x8248FA90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248FA90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248FA94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248FA98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248FA9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248FAA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248FAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FAA8 size=116
    let mut pc: u32 = 0x8248FAA8;
    'dispatch: loop {
        match pc {
            0x8248FAA8 => {
    //   block [0x8248FAA8..0x8248FADC)
	// 8248FAA8: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8248FAAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248FAB0: 419A006C  beq cr6, 0x8248fb1c
	if ctx.cr[6].eq {
		sub_8248FB1C(ctx, base);
		return;
	}
	// 8248FAB4: 54AB073E  clrlwi r11, r5, 0x1c
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 8248FAB8: 550907BE  clrlwi r9, r8, 0x1e
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 8248FABC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8248FAC0: 99630009  stb r11, 9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 8248FAC4: 554B07BE  clrlwi r11, r10, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 8248FAC8: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248FACC: 40980010  bge cr6, 0x8248fadc
	if !ctx.cr[6].lt {
	pc = 0x8248FADC; continue 'dispatch;
	}
	// 8248FAD0: 7CCB30F8  nor r11, r6, r6
	ctx.r[11].u64 = !(ctx.r[6].u64 | ctx.r[6].u64);
	// 8248FAD4: 55697022  slwi r9, r11, 0xe
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(14);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8248FAD8: 48000008  b 0x8248fae0
	pc = 0x8248FAE0; continue 'dispatch;
            }
            0x8248FADC => {
    //   block [0x8248FADC..0x8248FAE0)
	// 8248FADC: 54C97022  slwi r9, r6, 0xe
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(14);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	pc = 0x8248FAE0; continue 'dispatch;
            }
            0x8248FAE0 => {
    //   block [0x8248FAE0..0x8248FAF4)
	// 8248FAE0: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8248FAE4: 40980010  bge cr6, 0x8248faf4
	if !ctx.cr[6].lt {
	pc = 0x8248FAF4; continue 'dispatch;
	}
	// 8248FAE8: 7CEB38F8  nor r11, r7, r7
	ctx.r[11].u64 = !(ctx.r[7].u64 | ctx.r[7].u64);
	// 8248FAEC: 556B7022  slwi r11, r11, 0xe
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8248FAF0: 48000008  b 0x8248faf8
	pc = 0x8248FAF8; continue 'dispatch;
            }
            0x8248FAF4 => {
    //   block [0x8248FAF4..0x8248FAF8)
	// 8248FAF4: 54EB7022  slwi r11, r7, 0xe
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x8248FAF8; continue 'dispatch;
            }
            0x8248FAF8 => {
    //   block [0x8248FAF8..0x8248FB1C)
	// 8248FAF8: A143000A  lhz r10, 0xa(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 8248FAFC: A103000C  lhz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248FB00: 554A04BE  clrlwi r10, r10, 0x12
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 8248FB04: 550804BE  clrlwi r8, r8, 0x12
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00003FFFu64;
	// 8248FB08: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8248FB0C: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 8248FB10: B143000A  sth r10, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 8248FB14: B163000C  sth r11, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 8248FB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FB1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FB1C size=24
    let mut pc: u32 = 0x8248FB1C;
    'dispatch: loop {
        match pc {
            0x8248FB1C => {
    //   block [0x8248FB1C..0x8248FB34)
	// 8248FB1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248FB20: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 8248FB24: B163000A  sth r11, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 8248FB28: 99430009  stb r10, 9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(9 as u32), ctx.r[10].u8 ) };
	// 8248FB2C: B163000C  sth r11, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 8248FB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8248FB38 size=64
    let mut pc: u32 = 0x8248FB38;
    'dispatch: loop {
        match pc {
            0x8248FB38 => {
    //   block [0x8248FB38..0x8248FB78)
	// 8248FB38: C1A50000  lfs f13, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8248FB3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8248FB40: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248FB44: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8248FB48: C1850008  lfs f12, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8248FB4C: FC0B682E  fsel f0, f11, f0, f13
	ctx.f[0].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 8248FB50: EDA06028  fsubs f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8248FB54: FC0D032E  fsel f0, f13, f12, f0
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 8248FB58: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8248FB5C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8248FB60: 40980018  bge cr6, 0x8248fb78
	if !ctx.cr[6].lt {
		sub_8248FB78(ctx, base);
		return;
	}
	// 8248FB64: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8248FB68: C1ABD6C8  lfs f13, -0x2938(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10552 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8248FB6C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8248FB70: D0030048  stfs f0, 0x48(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 8248FB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8248FB78 size=16
    let mut pc: u32 = 0x8248FB78;
    'dispatch: loop {
        match pc {
            0x8248FB78 => {
    //   block [0x8248FB78..0x8248FB88)
	// 8248FB78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248FB7C: C00B8E30  lfs f0, -0x71d0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248FB80: D0030048  stfs f0, 0x48(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 8248FB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FB88 size=16
    let mut pc: u32 = 0x8248FB88;
    'dispatch: loop {
        match pc {
            0x8248FB88 => {
    //   block [0x8248FB88..0x8248FB98)
	// 8248FB88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248FB8C: 396BD8C4  addi r11, r11, -0x273c
	ctx.r[11].s64 = ctx.r[11].s64 + -10044;
	// 8248FB90: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248FB94: 4BFFF63C  b 0x8248f1d0
	sub_8248F1D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FB98 size=8
    let mut pc: u32 = 0x8248FB98;
    'dispatch: loop {
        match pc {
            0x8248FB98 => {
    //   block [0x8248FB98..0x8248FBA0)
	// 8248FB98: 386300E0  addi r3, r3, 0xe0
	ctx.r[3].s64 = ctx.r[3].s64 + 224;
	// 8248FB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FBA0 size=60
    let mut pc: u32 = 0x8248FBA0;
    'dispatch: loop {
        match pc {
            0x8248FBA0 => {
    //   block [0x8248FBA0..0x8248FBDC)
	// 8248FBA0: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8248FBA4: 419A004C  beq cr6, 0x8248fbf0
	if ctx.cr[6].eq {
		sub_8248FBF0(ctx, base);
		return;
	}
	// 8248FBA8: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8248FBAC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 8248FBB0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248FBB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248FBB8: 80A300C0  lwz r5, 0xc0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 8248FBBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248FBC0: 419A001C  beq cr6, 0x8248fbdc
	if ctx.cr[6].eq {
		sub_8248FBDC(ctx, base);
		return;
	}
	// 8248FBC4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8248FBC8: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 8248FBCC: 890B02C7  lbz r8, 0x2c7(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(711 as u32) ) } as u64;
	// 8248FBD0: 88EB02C6  lbz r7, 0x2c6(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(710 as u32) ) } as u64;
	// 8248FBD4: 88CB02C5  lbz r6, 0x2c5(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(709 as u32) ) } as u64;
	// 8248FBD8: 4BFFFED0  b 0x8248faa8
	sub_8248FAA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FBDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FBDC size=20
    let mut pc: u32 = 0x8248FBDC;
    'dispatch: loop {
        match pc {
            0x8248FBDC => {
    //   block [0x8248FBDC..0x8248FBF0)
	// 8248FBDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8248FBE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8248FBE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248FBE8: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 8248FBEC: 4BFFFEBC  b 0x8248faa8
	sub_8248FAA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FBF0 size=28
    let mut pc: u32 = 0x8248FBF0;
    'dispatch: loop {
        match pc {
            0x8248FBF0 => {
    //   block [0x8248FBF0..0x8248FC0C)
	// 8248FBF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8248FBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8248FBF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248FBFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8248FC00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8248FC04: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 8248FC08: 4BFFFEA0  b 0x8248faa8
	sub_8248FAA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FC0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FC0C size=4
    let mut pc: u32 = 0x8248FC0C;
    'dispatch: loop {
        match pc {
            0x8248FC0C => {
    //   block [0x8248FC0C..0x8248FC10)
	// 8248FC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FC10 size=28
    let mut pc: u32 = 0x8248FC10;
    'dispatch: loop {
        match pc {
            0x8248FC10 => {
    //   block [0x8248FC10..0x8248FC2C)
	// 8248FC10: 896300D9  lbz r11, 0xd9(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(217 as u32) ) } as u64;
	// 8248FC14: 396BFF01  addi r11, r11, -0xff
	ctx.r[11].s64 = ctx.r[11].s64 + -255;
	// 8248FC18: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8248FC1C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8248FC20: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8248FC24: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8248FC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FC30 size=24
    let mut pc: u32 = 0x8248FC30;
    'dispatch: loop {
        match pc {
            0x8248FC30 => {
    //   block [0x8248FC30..0x8248FC48)
	// 8248FC30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8248FC34: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 8248FC38: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8248FC3C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248FC40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248FC44: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248FC70 size=24
    let mut pc: u32 = 0x8248FC70;
    'dispatch: loop {
        match pc {
            0x8248FC70 => {
    //   block [0x8248FC70..0x8248FC88)
	// 8248FC70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8248FC74: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 8248FC78: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8248FC7C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8248FC80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248FC84: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8248FCA0 size=264
    let mut pc: u32 = 0x8248FCA0;
    'dispatch: loop {
        match pc {
            0x8248FCA0 => {
    //   block [0x8248FCA0..0x8248FDA8)
	// 8248FCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248FCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248FCA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248FCAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248FCB0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248FCB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248FCB8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8248FCBC: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8248FCC0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8248FCC4: 388B2690  addi r4, r11, 0x2690
	ctx.r[4].s64 = ctx.r[11].s64 + 9872;
	// 8248FCC8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8248FCCC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248FCD0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8248FCD4: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8248FCD8: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8248FCDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248FCE0: 4E800421  bctrl
	ctx.lr = 0x8248FCE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248FCE4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8248FCE8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8248FCEC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8248FCF0: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 8248FCF4: 39000160  li r8, 0x160
	ctx.r[8].s64 = 352;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248FDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8248FDA8 size=620
    let mut pc: u32 = 0x8248FDA8;
    'dispatch: loop {
        match pc {
            0x8248FDA8 => {
    //   block [0x8248FDA8..0x8248FE18)
	// 8248FDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248FDAC: 480A530D  bl 0x825350b8
	ctx.lr = 0x8248FDB0;
	sub_82535080(ctx, base);
	// 8248FDB0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8248FDB4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248FDB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248FDBC: D04100C4  stfs f2, 0xc4(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8248FDC0: D06100CC  stfs f3, 0xcc(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 8248FDC4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8248FDC8: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 8248FDCC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8248FDD0: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 8248FDD4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 8248FDD8: 419901AC  bgt cr6, 0x8248ff84
	if ctx.cr[6].gt {
	pc = 0x8248FF84; continue 'dispatch;
	}
	// 8248FDDC: 3D808249  lis r12, -0x7db7
	ctx.r[12].s64 = -2109145088;
	// 8248FDE0: 398CFDF4  addi r12, r12, -0x20c
	ctx.r[12].s64 = ctx.r[12].s64 + -524;
	// 8248FDE4: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8248FDE8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8248FDEC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8248FDF0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8248FEC0; continue 'dispatch;
		},
		1 => {
	pc = 0x8248FE18; continue 'dispatch;
		},
		2 => {
	pc = 0x8248FE78; continue 'dispatch;
		},
		3 => {
	pc = 0x8248FE48; continue 'dispatch;
		},
		4 => {
	pc = 0x8248FEA8; continue 'dispatch;
		},
		5 => {
	pc = 0x8248FF2C; continue 'dispatch;
		},
		6 => {
	pc = 0x8248FF84; continue 'dispatch;
		},
		7 => {
	pc = 0x8248FE60; continue 'dispatch;
		},
		8 => {
	pc = 0x8248FF54; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8248FDF4: 8248FEC0  lwz r18, -0x140(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-320 as u32) ) } as u64;
	// 8248FDF8: 8248FE18  lwz r18, -0x1e8(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-488 as u32) ) } as u64;
	// 8248FDFC: 8248FE78  lwz r18, -0x188(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-392 as u32) ) } as u64;
	// 8248FE00: 8248FE48  lwz r18, -0x1b8(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-440 as u32) ) } as u64;
	// 8248FE04: 8248FEA8  lwz r18, -0x158(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-344 as u32) ) } as u64;
	// 8248FE08: 8248FF2C  lwz r18, -0xd4(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-212 as u32) ) } as u64;
	// 8248FE0C: 8248FF84  lwz r18, -0x7c(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-124 as u32) ) } as u64;
	// 8248FE10: 8248FE60  lwz r18, -0x1a0(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-416 as u32) ) } as u64;
	// 8248FE14: 8248FF54  lwz r18, -0xac(r8)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-172 as u32) ) } as u64;
            }
            0x8248FE18 => {
    //   block [0x8248FE18..0x8248FE48)
	// 8248FE18: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FE1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248FE20: 419A017C  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248FE28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248FE2C: 4800B79D  bl 0x8249b5c8
	ctx.lr = 0x8248FE30;
	sub_8249B5C8(ctx, base);
	// 8248FE30: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248FE34: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8248FE38: 396BCA1C  addi r11, r11, -0x35e4
	ctx.r[11].s64 = ctx.r[11].s64 + -13796;
	// 8248FE3C: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 8248FE40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248FE44: 4800015C  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FE48 => {
    //   block [0x8248FE48..0x8248FE60)
	// 8248FE48: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FE4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248FE50: 419A014C  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FE54: 4800D4F5  bl 0x8249d348
	ctx.lr = 0x8248FE58;
	sub_8249D348(ctx, base);
	// 8248FE58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248FE5C: 48000144  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FE60 => {
    //   block [0x8248FE60..0x8248FE78)
	// 8248FE60: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FE64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248FE68: 419A0134  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FE6C: 48014045  bl 0x824a3eb0
	ctx.lr = 0x8248FE70;
	sub_824A3EB0(ctx, base);
	// 8248FE70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248FE74: 4800012C  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FE78 => {
    //   block [0x8248FE78..0x8248FEA8)
	// 8248FE78: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FE7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248FE80: 419A011C  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248FE88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248FE8C: 4800B73D  bl 0x8249b5c8
	ctx.lr = 0x8248FE90;
	sub_8249B5C8(ctx, base);
	// 8248FE90: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248FE94: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8248FE98: 396BCA84  addi r11, r11, -0x357c
	ctx.r[11].s64 = ctx.r[11].s64 + -13692;
	// 8248FE9C: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 8248FEA0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248FEA4: 480000FC  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FEA8 => {
    //   block [0x8248FEA8..0x8248FEC0)
	// 8248FEA8: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FEAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248FEB0: 419A00EC  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FEB4: 48013FB5  bl 0x824a3e68
	ctx.lr = 0x8248FEB8;
	sub_824A3E68(ctx, base);
	// 8248FEB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248FEBC: 480000E4  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FEC0 => {
    //   block [0x8248FEC0..0x8248FF2C)
	// 8248FEC0: C19E0028  lfs f12, 0x28(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8248FEC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8248FEC8: C1BE0014  lfs f13, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8248FECC: ED6D6028  fsubs f11, f13, f12
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 8248FED0: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248FED4: FD4B636E  fsel f10, f11, f13, f12
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 8248FED8: FDAB6B2E  fsel f13, f11, f12, f13
	ctx.f[13].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[13].f64 };
	// 8248FEDC: ED805028  fsubs f12, f0, f10
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 8248FEE0: ED606828  fsubs f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8248FEE4: FD8C502E  fsel f12, f12, f0, f10
	ctx.f[12].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[10].f64 };
	// 8248FEE8: FDAB036E  fsel f13, f11, f13, f0
	ctx.f[13].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 8248FEEC: C00B207C  lfs f0, 0x207c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8316 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248FEF0: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8248FEF4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8248FEF8: 4099FF50  ble cr6, 0x8248fe48
	if !ctx.cr[6].gt {
	pc = 0x8248FE48; continue 'dispatch;
	}
	// 8248FEFC: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FF00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248FF04: 419A0098  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FF08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248FF0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248FF10: 4800B6B9  bl 0x8249b5c8
	ctx.lr = 0x8248FF14;
	sub_8249B5C8(ctx, base);
	// 8248FF14: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248FF18: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8248FF1C: 396BCA1C  addi r11, r11, -0x35e4
	ctx.r[11].s64 = ctx.r[11].s64 + -13796;
	// 8248FF20: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 8248FF24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248FF28: 48000078  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FF2C => {
    //   block [0x8248FF2C..0x8248FF54)
	// 8248FF2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8248FF30: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FF34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248FF38: C00B24B0  lfs f0, 0x24b0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9392 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248FF3C: D00100C4  stfs f0, 0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8248FF40: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 8248FF44: 419A0058  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FF48: 4800C1D1  bl 0x8249c118
	ctx.lr = 0x8248FF4C;
	sub_8249C118(ctx, base);
	// 8248FF4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248FF50: 48000050  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FF54 => {
    //   block [0x8248FF54..0x8248FF84)
	// 8248FF54: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FF58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248FF5C: 419A0040  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FF60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248FF64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248FF68: 4800B661  bl 0x8249b5c8
	ctx.lr = 0x8248FF6C;
	sub_8249B5C8(ctx, base);
	// 8248FF6C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8248FF70: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 8248FF74: 396BCC9C  addi r11, r11, -0x3364
	ctx.r[11].s64 = ctx.r[11].s64 + -13156;
	// 8248FF78: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 8248FF7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248FF80: 48000020  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FF84 => {
    //   block [0x8248FF84..0x8248FF9C)
	// 8248FF84: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248FF88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248FF8C: 419A0010  beq cr6, 0x8248ff9c
	if ctx.cr[6].eq {
	pc = 0x8248FF9C; continue 'dispatch;
	}
	// 8248FF90: 4800D881  bl 0x8249d810
	ctx.lr = 0x8248FF94;
	sub_8249D810(ctx, base);
	// 8248FF94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248FF98: 48000008  b 0x8248ffa0
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FF9C => {
    //   block [0x8248FF9C..0x8248FFA0)
	// 8248FF9C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x8248FFA0; continue 'dispatch;
            }
            0x8248FFA0 => {
    //   block [0x8248FFA0..0x8248FFF0)
	// 8248FFA0: 2F1D0006  cmpwi cr6, r29, 6
	ctx.cr[6].compare_i32(ctx.r[29].s32, 6, &mut ctx.xer);
	// 8248FFA4: 419A004C  beq cr6, 0x8248fff0
	if ctx.cr[6].eq {
	pc = 0x8248FFF0; continue 'dispatch;
	}
	// 8248FFA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248FFAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248FFB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248FFB4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8248FFB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248FFBC: 4E800421  bctrl
	ctx.lr = 0x8248FFC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248FFC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248FFC4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8248FFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248FFCC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8248FFD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248FFD4: 4E800421  bctrl
	ctx.lr = 0x8248FFD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248FFD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248FFDC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8248FFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248FFE4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248FFE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248FFEC: 4E800421  bctrl
	ctx.lr = 0x8248FFF0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8248FFF0 => {
    //   block [0x8248FFF0..0x82490014)
	// 8248FFF0: 388100C4  addi r4, r1, 0xc4
	ctx.r[4].s64 = ctx.r[1].s64 + 196;
	// 8248FFF4: 387F00BC  addi r3, r31, 0xbc
	ctx.r[3].s64 = ctx.r[31].s64 + 188;
	// 8248FFF8: 4BFF38B9  bl 0x824838b0
	ctx.lr = 0x8248FFFC;
	sub_824838B0(ctx, base);
	// 8248FFFC: 388100CC  addi r4, r1, 0xcc
	ctx.r[4].s64 = ctx.r[1].s64 + 204;
	// 82490000: 387F00BD  addi r3, r31, 0xbd
	ctx.r[3].s64 = ctx.r[31].s64 + 189;
	// 82490004: 4BFF38AD  bl 0x824838b0
	ctx.lr = 0x82490008;
	sub_824838B0(ctx, base);
	// 82490008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8249000C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82490010: 480A50F8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82490018 size=684
    let mut pc: u32 = 0x82490018;
    'dispatch: loop {
        match pc {
            0x82490018 => {
    //   block [0x82490018..0x82490098)
	// 82490018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8249001C: 480A509D  bl 0x825350b8
	ctx.lr = 0x82490020;
	sub_82535080(ctx, base);
	// 82490020: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82490024: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490028: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8249002C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490030: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82490034: 4BFFE73D  bl 0x8248e770
	ctx.lr = 0x82490038;
	sub_8248E770(ctx, base);
	// 82490038: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8249003C: 3BBF00D0  addi r29, r31, 0xd0
	ctx.r[29].s64 = ctx.r[31].s64 + 208;
	// 82490040: 396BD8C4  addi r11, r11, -0x273c
	ctx.r[11].s64 = ctx.r[11].s64 + -10044;
	// 82490044: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82490048: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8249004C: 997F0080  stb r11, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82490050: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82490054: B17F0096  sth r11, 0x96(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(150 as u32), ctx.r[11].u16 ) };
	// 82490058: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249005C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82490060: 897E00B0  lbz r11, 0xb0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82490064: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82490068: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8249006C: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 82490070: C3EB1FF8  lfs f31, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82490074: 409A006C  bne cr6, 0x824900e0
	if !ctx.cr[6].eq {
	pc = 0x824900E0; continue 'dispatch;
	}
	// 82490078: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8249007C: 419A001C  beq cr6, 0x82490098
	if ctx.cr[6].eq {
	pc = 0x82490098; continue 'dispatch;
	}
	// 82490080: 38BE0020  addi r5, r30, 0x20
	ctx.r[5].s64 = ctx.r[30].s64 + 32;
	// 82490084: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82490088: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8249008C: 4800D785  bl 0x8249d810
	ctx.lr = 0x82490090;
	sub_8249D810(ctx, base);
	// 82490090: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82490094: 48000008  b 0x8249009c
	pc = 0x8249009C; continue 'dispatch;
            }
            0x82490098 => {
    //   block [0x82490098..0x8249009C)
	// 82490098: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	pc = 0x8249009C; continue 'dispatch;
            }
            0x8249009C => {
    //   block [0x8249009C..0x824900E0)
	// 8249009C: 389E00A4  addi r4, r30, 0xa4
	ctx.r[4].s64 = ctx.r[30].s64 + 164;
	// 824900A0: 387C00BC  addi r3, r28, 0xbc
	ctx.r[3].s64 = ctx.r[28].s64 + 188;
	// 824900A4: 4BFF380D  bl 0x824838b0
	ctx.lr = 0x824900A8;
	sub_824838B0(ctx, base);
	// 824900A8: 389E00A8  addi r4, r30, 0xa8
	ctx.r[4].s64 = ctx.r[30].s64 + 168;
	// 824900AC: 387C00BD  addi r3, r28, 0xbd
	ctx.r[3].s64 = ctx.r[28].s64 + 189;
	// 824900B0: 4BFF3801  bl 0x824838b0
	ctx.lr = 0x824900B4;
	sub_824838B0(ctx, base);
	// 824900B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824900B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824900BC: 4800B475  bl 0x8249b530
	ctx.lr = 0x824900C0;
	sub_8249B530(ctx, base);
	// 824900C0: 397F00E0  addi r11, r31, 0xe0
	ctx.r[11].s64 = ctx.r[31].s64 + 224;
	// 824900C4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824900C8: C01E00AC  lfs f0, 0xac(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824900CC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 824900D0: 41990090  bgt cr6, 0x82490160
	if ctx.cr[6].gt {
	pc = 0x82490160; continue 'dispatch;
	}
	// 824900D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824900D8: C00B8CB4  lfs f0, -0x734c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824900DC: 48000084  b 0x82490160
	pc = 0x82490160; continue 'dispatch;
            }
            0x824900E0 => {
    //   block [0x824900E0..0x82490160)
	// 824900E0: 391E0080  addi r8, r30, 0x80
	ctx.r[8].s64 = ctx.r[30].s64 + 128;
	// 824900E4: C07E00A8  lfs f3, 0xa8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 824900E8: 38FE0050  addi r7, r30, 0x50
	ctx.r[7].s64 = ctx.r[30].s64 + 80;
	// 824900EC: C05E00A4  lfs f2, 0xa4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(164 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 824900F0: 38BE0020  addi r5, r30, 0x20
	ctx.r[5].s64 = ctx.r[30].s64 + 32;
	// 824900F4: C03E0090  lfs f1, 0x90(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824900F8: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 824900FC: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82490100: 4BFFFCA9  bl 0x8248fda8
	ctx.lr = 0x82490104;
	sub_8248FDA8(ctx, base);
	// 82490104: 897E00B2  lbz r11, 0xb2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(178 as u32) ) } as u64;
	// 82490108: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8249010C: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82490110: 4800B421  bl 0x8249b530
	ctx.lr = 0x82490114;
	sub_8249B530(ctx, base);
	// 82490114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490118: 4BFFE0E9  bl 0x8248e200
	ctx.lr = 0x8249011C;
	sub_8248E200(ctx, base);
	// 8249011C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82490120: 389E0030  addi r4, r30, 0x30
	ctx.r[4].s64 = ctx.r[30].s64 + 48;
	// 82490124: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82490128: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8249012C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490130: 4E800421  bctrl
	ctx.lr = 0x82490134;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490138: 4BFFE0C9  bl 0x8248e200
	ctx.lr = 0x8249013C;
	sub_8248E200(ctx, base);
	// 8249013C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82490140: 389E0040  addi r4, r30, 0x40
	ctx.r[4].s64 = ctx.r[30].s64 + 64;
	// 82490144: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82490148: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8249014C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490150: 4E800421  bctrl
	ctx.lr = 0x82490154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490154: 397F00E0  addi r11, r31, 0xe0
	ctx.r[11].s64 = ctx.r[31].s64 + 224;
	// 82490158: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8249015C: C01E00AC  lfs f0, 0xac(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            0x82490160 => {
    //   block [0x82490160..0x824901A4)
	// 82490160: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82490164: 897E00B1  lbz r11, 0xb1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(177 as u32) ) } as u64;
	// 82490168: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8249016C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82490170: 419A0034  beq cr6, 0x824901a4
	if ctx.cr[6].eq {
	pc = 0x824901A4; continue 'dispatch;
	}
	// 82490174: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82490178: 409A0048  bne cr6, 0x824901c0
	if !ctx.cr[6].eq {
	pc = 0x824901C0; continue 'dispatch;
	}
	// 8249017C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82490180: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82490184: 80BF00C0  lwz r5, 0xc0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82490188: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8249018C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82490190: 419A0020  beq cr6, 0x824901b0
	if ctx.cr[6].eq {
	pc = 0x824901B0; continue 'dispatch;
	}
	// 82490194: 890B02C7  lbz r8, 0x2c7(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(711 as u32) ) } as u64;
	// 82490198: 88EB02C6  lbz r7, 0x2c6(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(710 as u32) ) } as u64;
	// 8249019C: 88CB02C5  lbz r6, 0x2c5(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(709 as u32) ) } as u64;
	// 824901A0: 4800001C  b 0x824901bc
	pc = 0x824901BC; continue 'dispatch;
            }
            0x824901A4 => {
    //   block [0x824901A4..0x824901B0)
	// 824901A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824901A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824901AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	pc = 0x824901B0; continue 'dispatch;
            }
            0x824901B0 => {
    //   block [0x824901B0..0x824901BC)
	// 824901B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824901B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824901B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	pc = 0x824901BC; continue 'dispatch;
            }
            0x824901BC => {
    //   block [0x824901BC..0x824901C0)
	// 824901BC: 4BFFF8ED  bl 0x8248faa8
	ctx.lr = 0x824901C0;
	sub_8248FAA8(ctx, base);
	pc = 0x824901C0; continue 'dispatch;
            }
            0x824901C0 => {
    //   block [0x824901C0..0x82490230)
	// 824901C0: C01E0094  lfs f0, 0x94(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824901C4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824901C8: D01F0184  stfs f0, 0x184(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 824901CC: C01E0098  lfs f0, 0x98(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824901D0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824901D4: D01F0188  stfs f0, 0x188(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), tmp.u32 ) };
	// 824901D8: 419A0064  beq cr6, 0x8249023c
	if ctx.cr[6].eq {
	pc = 0x8249023C; continue 'dispatch;
	}
	// 824901DC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824901E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824901E4: 4BFFFABD  bl 0x8248fca0
	ctx.lr = 0x824901E8;
	sub_8248FCA0(ctx, base);
	// 824901E8: C01F0058  lfs f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824901EC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 824901F0: 4199004C  bgt cr6, 0x8249023c
	if ctx.cr[6].gt {
	pc = 0x8249023C; continue 'dispatch;
	}
	// 824901F4: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824901F8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824901FC: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490200: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82490204: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82490208: C1A10068  lfs f13, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8249020C: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82490210: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82490214: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82490218: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8249021C: 40980014  bge cr6, 0x82490230
	if !ctx.cr[6].lt {
	pc = 0x82490230; continue 'dispatch;
	}
	// 82490220: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82490224: C1ABD6C8  lfs f13, -0x2938(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10552 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82490228: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8249022C: 4800000C  b 0x82490238
	pc = 0x82490238; continue 'dispatch;
            }
            0x82490230 => {
    //   block [0x82490230..0x82490238)
	// 82490230: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82490234: C00B8E30  lfs f0, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x82490238; continue 'dispatch;
            }
            0x82490238 => {
    //   block [0x82490238..0x8249023C)
	// 82490238: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	pc = 0x8249023C; continue 'dispatch;
            }
            0x8249023C => {
    //   block [0x8249023C..0x82490250)
	// 8249023C: 897E00B3  lbz r11, 0xb3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(179 as u32) ) } as u64;
	// 82490240: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82490244: 419A000C  beq cr6, 0x82490250
	if ctx.cr[6].eq {
	pc = 0x82490250; continue 'dispatch;
	}
	// 82490248: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8249024C: 4800002C  b 0x82490278
	pc = 0x82490278; continue 'dispatch;
            }
            0x82490250 => {
    //   block [0x82490250..0x82490264)
	// 82490250: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82490254: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82490258: 409A000C  bne cr6, 0x82490264
	if !ctx.cr[6].eq {
	pc = 0x82490264; continue 'dispatch;
	}
	// 8249025C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82490260: 48000018  b 0x82490278
	pc = 0x82490278; continue 'dispatch;
            }
            0x82490264 => {
    //   block [0x82490264..0x82490278)
	// 82490264: 897E00B0  lbz r11, 0xb0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82490268: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 8249026C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82490270: 419A0008  beq cr6, 0x82490278
	if ctx.cr[6].eq {
	pc = 0x82490278; continue 'dispatch;
	}
	// 82490274: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	pc = 0x82490278; continue 'dispatch;
            }
            0x82490278 => {
    //   block [0x82490278..0x824902A4)
	// 82490278: B17F002A  sth r11, 0x2a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(42 as u32), ctx.r[11].u16 ) };
	// 8249027C: 897E00B5  lbz r11, 0xb5(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(181 as u32) ) } as u64;
	// 82490280: 997F00BD  stb r11, 0xbd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(189 as u32), ctx.r[11].u8 ) };
	// 82490284: 897E00B4  lbz r11, 0xb4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 82490288: 997F00BC  stb r11, 0xbc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[11].u8 ) };
	// 8249028C: 897E00B6  lbz r11, 0xb6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(182 as u32) ) } as u64;
	// 82490290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82490294: 419A0010  beq cr6, 0x824902a4
	if ctx.cr[6].eq {
	pc = 0x824902A4; continue 'dispatch;
	}
	// 82490298: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 8249029C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 824902A0: 997F0021  stb r11, 0x21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	pc = 0x824902A4; continue 'dispatch;
            }
            0x824902A4 => {
    //   block [0x824902A4..0x824902C4)
	// 824902A4: C01E009C  lfs f0, 0x9c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824902A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824902AC: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 824902B0: C01E00A0  lfs f0, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824902B4: D01F0088  stfs f0, 0x88(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 824902B8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824902BC: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824902C0: 480A4E48  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824902C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824902C8 size=120
    let mut pc: u32 = 0x824902C8;
    'dispatch: loop {
        match pc {
            0x824902C8 => {
    //   block [0x824902C8..0x82490324)
	// 824902C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824902CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824902D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824902D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824902D8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 824902DC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824902E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824902E4: 419A0040  beq cr6, 0x82490324
	if ctx.cr[6].eq {
	pc = 0x82490324; continue 'dispatch;
	}
	// 824902E8: 81230084  lwz r9, 0x84(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824902EC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824902F0: 419A0034  beq cr6, 0x82490324
	if ctx.cr[6].eq {
	pc = 0x82490324; continue 'dispatch;
	}
	// 824902F4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 824902F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824902FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82490300: 99410058  stb r10, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u8 ) };
	// 82490304: 98A10059  stb r5, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[5].u8 ) };
	// 82490308: 98C1005A  stb r6, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[6].u8 ) };
	// 8249030C: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82490310: 48002EC9  bl 0x824931d8
	ctx.lr = 0x82490314;
	sub_824931D8(ctx, base);
	// 82490314: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82490318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8249031C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490320: 4E800020  blr
	return;
            }
            0x82490324 => {
    //   block [0x82490324..0x82490340)
	// 82490324: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82490328: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8249032C: 48011715  bl 0x824a1a40
	ctx.lr = 0x82490330;
	sub_824A1A40(ctx, base);
	// 82490330: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82490334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82490338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8249033C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82490340 size=496
    let mut pc: u32 = 0x82490340;
    'dispatch: loop {
        match pc {
            0x82490340 => {
    //   block [0x82490340..0x82490390)
	// 82490340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490344: 480A4D79  bl 0x825350bc
	ctx.lr = 0x82490348;
	sub_82535080(ctx, base);
	// 82490348: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8249034C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490350: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82490354: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82490358: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8249035C: 419A004C  beq cr6, 0x824903a8
	if ctx.cr[6].eq {
	pc = 0x824903A8; continue 'dispatch;
	}
	// 82490360: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82490364: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490368: 419A0028  beq cr6, 0x82490390
	if ctx.cr[6].eq {
	pc = 0x82490390; continue 'dispatch;
	}
	// 8249036C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82490370: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82490374: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82490378: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8249037C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82490380: 48002E59  bl 0x824931d8
	ctx.lr = 0x82490384;
	sub_824931D8(ctx, base);
	// 82490384: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82490388: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8249038C: 480A4D80  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82490390 => {
    //   block [0x82490390..0x824903A8)
	// 82490390: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82490394: 419A0014  beq cr6, 0x824903a8
	if ctx.cr[6].eq {
	pc = 0x824903A8; continue 'dispatch;
	}
	// 82490398: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8249039C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824903A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824903A4: 4BFFDC3D  bl 0x8248dfe0
	ctx.lr = 0x824903A8;
	sub_8248DFE0(ctx, base);
	pc = 0x824903A8; continue 'dispatch;
            }
            0x824903A8 => {
    //   block [0x824903A8..0x824903CC)
	// 824903A8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824903AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824903B0: 419A001C  beq cr6, 0x824903cc
	if ctx.cr[6].eq {
	pc = 0x824903CC; continue 'dispatch;
	}
	// 824903B4: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 824903B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824903BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824903C0: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 824903C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824903C8: 480101F9  bl 0x824a05c0
	ctx.lr = 0x824903CC;
	sub_824A05C0(ctx, base);
	pc = 0x824903CC; continue 'dispatch;
            }
            0x824903CC => {
    //   block [0x824903CC..0x824903EC)
	// 824903CC: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824903D0: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 824903D4: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824903D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824903DC: 419A0010  beq cr6, 0x824903ec
	if ctx.cr[6].eq {
	pc = 0x824903EC; continue 'dispatch;
	}
	// 824903E0: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 824903E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824903E8: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x824903EC; continue 'dispatch;
            }
            0x824903EC => {
    //   block [0x824903EC..0x82490430)
	// 824903EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824903F0: 419A0040  beq cr6, 0x82490430
	if ctx.cr[6].eq {
	pc = 0x82490430; continue 'dispatch;
	}
	// 824903F4: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824903F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824903FC: 419A0034  beq cr6, 0x82490430
	if ctx.cr[6].eq {
	pc = 0x82490430; continue 'dispatch;
	}
	// 82490400: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82490404: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82490408: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8249040C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490410: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82490414: 409A001C  bne cr6, 0x82490430
	if !ctx.cr[6].eq {
	pc = 0x82490430; continue 'dispatch;
	}
	// 82490418: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249041C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82490420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82490424: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8249042C: 4E800421  bctrl
	ctx.lr = 0x82490430;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82490430 => {
    //   block [0x82490430..0x82490468)
	// 82490430: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82490434: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82490438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8249043C: 4BFFF865  bl 0x8248fca0
	ctx.lr = 0x82490440;
	sub_8248FCA0(ctx, base);
	// 82490440: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82490444: 419A0024  beq cr6, 0x82490468
	if ctx.cr[6].eq {
	pc = 0x82490468; continue 'dispatch;
	}
	// 82490448: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8249044C: C1BF0058  lfs f13, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82490450: C00B8CB4  lfs f0, -0x734c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490454: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82490458: 419A0010  beq cr6, 0x82490468
	if ctx.cr[6].eq {
	pc = 0x82490468; continue 'dispatch;
	}
	// 8249045C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82490460: C00B2074  lfs f0, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490464: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	pc = 0x82490468; continue 'dispatch;
            }
            0x82490468 => {
    //   block [0x82490468..0x824904B8)
	// 82490468: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8249046C: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82490470: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490474: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82490478: 4199004C  bgt cr6, 0x824904c4
	if ctx.cr[6].gt {
	pc = 0x824904C4; continue 'dispatch;
	}
	// 8249047C: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82490480: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82490484: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490488: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8249048C: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82490490: C1A10068  lfs f13, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82490494: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82490498: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 8249049C: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824904A0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824904A4: 40980014  bge cr6, 0x824904b8
	if !ctx.cr[6].lt {
	pc = 0x824904B8; continue 'dispatch;
	}
	// 824904A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824904AC: C1ABD6C8  lfs f13, -0x2938(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10552 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824904B0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824904B4: 4800000C  b 0x824904c0
	pc = 0x824904C0; continue 'dispatch;
            }
            0x824904B8 => {
    //   block [0x824904B8..0x824904C0)
	// 824904B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824904BC: C00B8E30  lfs f0, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x824904C0; continue 'dispatch;
            }
            0x824904C0 => {
    //   block [0x824904C0..0x824904C4)
	// 824904C0: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	pc = 0x824904C4; continue 'dispatch;
            }
            0x824904C4 => {
    //   block [0x824904C4..0x824904D8)
	// 824904C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824904C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824904CC: 419A000C  beq cr6, 0x824904d8
	if ctx.cr[6].eq {
	pc = 0x824904D8; continue 'dispatch;
	}
	// 824904D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824904D4: 48013EA5  bl 0x824a4378
	ctx.lr = 0x824904D8;
	sub_824A4378(ctx, base);
	pc = 0x824904D8; continue 'dispatch;
            }
            0x824904D8 => {
    //   block [0x824904D8..0x82490524)
	// 824904D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824904DC: 48011F6D  bl 0x824a2448
	ctx.lr = 0x824904E0;
	sub_824A2448(ctx, base);
	// 824904E0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824904E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824904E8: 419A003C  beq cr6, 0x82490524
	if ctx.cr[6].eq {
	pc = 0x82490524; continue 'dispatch;
	}
	// 824904EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824904F0: 4800FE21  bl 0x824a0310
	ctx.lr = 0x824904F4;
	sub_824A0310(ctx, base);
	// 824904F4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824904F8: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824904FC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490500: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82490504: 40820020  bne 0x82490524
	if !ctx.cr[0].eq {
	pc = 0x82490524; continue 'dispatch;
	}
	// 82490508: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 8249050C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82490510: 409A0014  bne cr6, 0x82490524
	if !ctx.cr[6].eq {
	pc = 0x82490524; continue 'dispatch;
	}
	// 82490514: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82490518: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8249051C: 419A0008  beq cr6, 0x82490524
	if ctx.cr[6].eq {
	pc = 0x82490524; continue 'dispatch;
	}
	// 82490520: 48002CA1  bl 0x824931c0
	ctx.lr = 0x82490524;
	sub_824931C0(ctx, base);
	pc = 0x82490524; continue 'dispatch;
            }
            0x82490524 => {
    //   block [0x82490524..0x82490530)
	// 82490524: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82490528: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8249052C: 480A4BE0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490530 size=76
    let mut pc: u32 = 0x82490530;
    'dispatch: loop {
        match pc {
            0x82490530 => {
    //   block [0x82490530..0x8249057C)
	// 82490530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8249053C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490544: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82490548: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8249054C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82490550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490554: 4E800421  bctrl
	ctx.lr = 0x82490558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490558: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8249055C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490560: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82490564: 4BFFF73D  bl 0x8248fca0
	ctx.lr = 0x82490568;
	sub_8248FCA0(ctx, base);
	// 82490568: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8249056C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82490570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82490580 size=28
    let mut pc: u32 = 0x82490580;
    'dispatch: loop {
        match pc {
            0x82490580 => {
    //   block [0x82490580..0x8249059C)
	// 82490580: 896300D8  lbz r11, 0xd8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 82490584: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82490588: 419A0014  beq cr6, 0x8249059c
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8249059C);
		return;
	}
	// 8249058C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82490590: 419A000C  beq cr6, 0x8249059c
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8249059C);
		return;
	}
	// 82490594: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82490598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824905D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824905D0 size=304
    let mut pc: u32 = 0x824905D0;
    'dispatch: loop {
        match pc {
            0x824905D0 => {
    //   block [0x824905D0..0x8249061C)
	// 824905D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824905D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824905D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824905DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824905E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824905E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824905E8: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824905EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824905F0: 419A00F8  beq cr6, 0x824906e8
	if ctx.cr[6].eq {
	pc = 0x824906E8; continue 'dispatch;
	}
	// 824905F4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824905F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824905FC: 419A0020  beq cr6, 0x8249061c
	if ctx.cr[6].eq {
	pc = 0x8249061C; continue 'dispatch;
	}
	// 82490600: 39600015  li r11, 0x15
	ctx.r[11].s64 = 21;
	// 82490604: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82490608: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8249060C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490610: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82490614: 48002BC5  bl 0x824931d8
	ctx.lr = 0x82490618;
	sub_824931D8(ctx, base);
	// 82490618: 480000D0  b 0x824906e8
	pc = 0x824906E8; continue 'dispatch;
            }
            0x8249061C => {
    //   block [0x8249061C..0x82490674)
	// 8249061C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82490620: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82490624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82490628: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8249062C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82490630: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82490634: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82490638: 915F0084  stw r10, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 8249063C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82490640: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490644: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82490648: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249064C: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82490650: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490654: 4E800421  bctrl
	ctx.lr = 0x82490658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490658: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8249065C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490660: 419A0014  beq cr6, 0x82490674
	if ctx.cr[6].eq {
	pc = 0x82490674; continue 'dispatch;
	}
	// 82490664: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82490668: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8249066C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82490670: 48016861  bl 0x824a6ed0
	ctx.lr = 0x82490674;
	sub_824A6ED0(ctx, base);
            }
            0x82490674 => {
    //   block [0x82490674..0x824906AC)
	// 82490674: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82490678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8249067C: 4BFFD925  bl 0x8248dfa0
	ctx.lr = 0x82490680;
	sub_8248DFA0(ctx, base);
	// 82490680: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490684: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82490688: 409A0030  bne cr6, 0x824906b8
	if !ctx.cr[6].eq {
	pc = 0x824906B8; continue 'dispatch;
	}
	// 8249068C: 897F00C6  lbz r11, 0xc6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(198 as u32) ) } as u64;
	// 82490690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82490694: 419A0018  beq cr6, 0x824906ac
	if ctx.cr[6].eq {
	pc = 0x824906AC; continue 'dispatch;
	}
	// 82490698: 897E00D8  lbz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 8249069C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 824906A0: 419A000C  beq cr6, 0x824906ac
	if ctx.cr[6].eq {
	pc = 0x824906AC; continue 'dispatch;
	}
	// 824906A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824906A8: 4BFFDB59  bl 0x8248e200
	ctx.lr = 0x824906AC;
	sub_8248E200(ctx, base);
	pc = 0x824906AC; continue 'dispatch;
            }
            0x824906AC => {
    //   block [0x824906AC..0x824906B8)
	// 824906AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824906B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824906B4: 48015035  bl 0x824a56e8
	ctx.lr = 0x824906B8;
	sub_824A56E8(ctx, base);
	pc = 0x824906B8; continue 'dispatch;
            }
            0x824906B8 => {
    //   block [0x824906B8..0x824906E8)
	// 824906B8: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824906BC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824906C0: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824906C4: 40820024  bne 0x824906e8
	if !ctx.cr[0].eq {
	pc = 0x824906E8; continue 'dispatch;
	}
	// 824906C8: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 824906CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824906D0: 409A0018  bne cr6, 0x824906e8
	if !ctx.cr[6].eq {
	pc = 0x824906E8; continue 'dispatch;
	}
	// 824906D4: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824906D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824906DC: 419A000C  beq cr6, 0x824906e8
	if ctx.cr[6].eq {
	pc = 0x824906E8; continue 'dispatch;
	}
	// 824906E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824906E4: 48002ADD  bl 0x824931c0
	ctx.lr = 0x824906E8;
	sub_824931C0(ctx, base);
	pc = 0x824906E8; continue 'dispatch;
            }
            0x824906E8 => {
    //   block [0x824906E8..0x82490700)
	// 824906E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824906EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824906F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824906F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824906F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824906FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490700 size=68
    let mut pc: u32 = 0x82490700;
    'dispatch: loop {
        match pc {
            0x82490700 => {
    //   block [0x82490700..0x82490744)
	// 82490700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8249070C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490714: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82490718: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8249071C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82490720: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490724: 4E800421  bctrl
	ctx.lr = 0x82490728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8249072C: 4BFFFEA5  bl 0x824905d0
	ctx.lr = 0x82490730;
	sub_824905D0(ctx, base);
	// 82490730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82490734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82490738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8249073C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490748 size=68
    let mut pc: u32 = 0x82490748;
    'dispatch: loop {
        match pc {
            0x82490748 => {
    //   block [0x82490748..0x8249078C)
	// 82490748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8249074C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82490754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8249075C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82490760: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82490764: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82490768: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8249076C: 4E800421  bctrl
	ctx.lr = 0x82490770;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490774: 4BFFFE5D  bl 0x824905d0
	ctx.lr = 0x82490778;
	sub_824905D0(ctx, base);
	// 82490778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8249077C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82490780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490784: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490790 size=68
    let mut pc: u32 = 0x82490790;
    'dispatch: loop {
        match pc {
            0x82490790 => {
    //   block [0x82490790..0x824907D4)
	// 82490790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490798: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8249079C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824907A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824907A4: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 824907A8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 824907AC: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824907B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824907B4: 4E800421  bctrl
	ctx.lr = 0x824907B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824907B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824907BC: 4BFFFE15  bl 0x824905d0
	ctx.lr = 0x824907C0;
	sub_824905D0(ctx, base);
	// 824907C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824907C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824907C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824907CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824907D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824907D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824907D8 size=68
    let mut pc: u32 = 0x824907D8;
    'dispatch: loop {
        match pc {
            0x824907D8 => {
    //   block [0x824907D8..0x8249081C)
	// 824907D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824907DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824907E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824907E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824907E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824907EC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 824907F0: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 824907F4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 824907F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824907FC: 4E800421  bctrl
	ctx.lr = 0x82490800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490804: 4BFFFDCD  bl 0x824905d0
	ctx.lr = 0x82490808;
	sub_824905D0(ctx, base);
	// 82490808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8249080C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82490810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490820 size=176
    let mut pc: u32 = 0x82490820;
    'dispatch: loop {
        match pc {
            0x82490820 => {
    //   block [0x82490820..0x824908A4)
	// 82490820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490824: 480A4899  bl 0x825350bc
	ctx.lr = 0x82490828;
	sub_82535080(ctx, base);
	// 82490828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8249082C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490830: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82490834: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82490838: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8249083C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490840: 419A0064  beq cr6, 0x824908a4
	if ctx.cr[6].eq {
	pc = 0x824908A4; continue 'dispatch;
	}
	// 82490844: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82490848: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8249084C: 419A0058  beq cr6, 0x824908a4
	if ctx.cr[6].eq {
	pc = 0x824908A4; continue 'dispatch;
	}
	// 82490850: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490854: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82490858: 39200017  li r9, 0x17
	ctx.r[9].s64 = 23;
	// 8249085C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82490860: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82490864: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82490868: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8249086C: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82490870: 4BFD37C9  bl 0x82464038
	ctx.lr = 0x82490874;
	sub_82464038(ctx, base);
	pc = 0x824908A4; continue 'dispatch;
            }
            0x824908A4 => {
    //   block [0x824908A4..0x824908D0)
	// 824908A4: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 824908A8: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 824908AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824908B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824908B4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824908B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824908BC: 4E800421  bctrl
	ctx.lr = 0x824908C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824908C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824908C4: 4BFFFD0D  bl 0x824905d0
	ctx.lr = 0x824908C8;
	sub_824905D0(ctx, base);
	// 824908C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824908CC: 480A4840  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824908D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824908D0 size=156
    let mut pc: u32 = 0x824908D0;
    'dispatch: loop {
        match pc {
            0x824908D0 => {
    //   block [0x824908D0..0x82490934)
	// 824908D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824908D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824908D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824908DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824908E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824908E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824908E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824908EC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824908F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824908F4: 419A0040  beq cr6, 0x82490934
	if ctx.cr[6].eq {
	pc = 0x82490934; continue 'dispatch;
	}
	// 824908F8: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824908FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490900: 419A0034  beq cr6, 0x82490934
	if ctx.cr[6].eq {
	pc = 0x82490934; continue 'dispatch;
	}
	// 82490904: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82490908: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8249090C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82490910: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82490914: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490918: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8249091C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82490920: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490924: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82490928: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8249092C: 480028AD  bl 0x824931d8
	ctx.lr = 0x82490930;
	sub_824931D8(ctx, base);
	// 82490930: 48000024  b 0x82490954
	pc = 0x82490954; continue 'dispatch;
            }
            0x82490934 => {
    //   block [0x82490934..0x82490954)
	// 82490934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490938: 4BFFD8C9  bl 0x8248e200
	ctx.lr = 0x8249093C;
	sub_8248E200(ctx, base);
	// 8249093C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82490940: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82490944: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82490948: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8249094C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490950: 4E800421  bctrl
	ctx.lr = 0x82490954;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82490954 => {
    //   block [0x82490954..0x8249096C)
	// 82490954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82490958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8249095C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490960: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82490964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82490970 size=156
    let mut pc: u32 = 0x82490970;
    'dispatch: loop {
        match pc {
            0x82490970 => {
    //   block [0x82490970..0x824909D4)
	// 82490970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8249097C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82490980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490988: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8249098C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82490990: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82490994: 419A0040  beq cr6, 0x824909d4
	if ctx.cr[6].eq {
	pc = 0x824909D4; continue 'dispatch;
	}
	// 82490998: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 8249099C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824909A0: 419A0034  beq cr6, 0x824909d4
	if ctx.cr[6].eq {
	pc = 0x824909D4; continue 'dispatch;
	}
	// 824909A4: 39600019  li r11, 0x19
	ctx.r[11].s64 = 25;
	// 824909A8: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824909AC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824909B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824909B4: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824909B8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 824909BC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824909C0: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824909C4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 824909C8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 824909CC: 4800280D  bl 0x824931d8
	ctx.lr = 0x824909D0;
	sub_824931D8(ctx, base);
	// 824909D0: 48000024  b 0x824909f4
	pc = 0x824909F4; continue 'dispatch;
            }
            0x824909D4 => {
    //   block [0x824909D4..0x824909F4)
	// 824909D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824909D8: 4BFFD829  bl 0x8248e200
	ctx.lr = 0x824909DC;
	sub_8248E200(ctx, base);
	// 824909DC: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 824909E0: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 824909E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824909E8: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 824909EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824909F0: 4E800421  bctrl
	ctx.lr = 0x824909F4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824909F4 => {
    //   block [0x824909F4..0x82490A0C)
	// 824909F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824909F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824909FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490A00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82490A04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82490A10 size=156
    let mut pc: u32 = 0x82490A10;
    'dispatch: loop {
        match pc {
            0x82490A10 => {
    //   block [0x82490A10..0x82490A74)
	// 82490A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490A18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82490A1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82490A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490A24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490A28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82490A2C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82490A30: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82490A34: 419A0040  beq cr6, 0x82490a74
	if ctx.cr[6].eq {
	pc = 0x82490A74; continue 'dispatch;
	}
	// 82490A38: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82490A3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490A40: 419A0034  beq cr6, 0x82490a74
	if ctx.cr[6].eq {
	pc = 0x82490A74; continue 'dispatch;
	}
	// 82490A44: 3960001A  li r11, 0x1a
	ctx.r[11].s64 = 26;
	// 82490A48: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490A4C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82490A50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82490A54: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490A58: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82490A5C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82490A60: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490A64: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82490A68: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82490A6C: 4800276D  bl 0x824931d8
	ctx.lr = 0x82490A70;
	sub_824931D8(ctx, base);
	// 82490A70: 48000024  b 0x82490a94
	pc = 0x82490A94; continue 'dispatch;
            }
            0x82490A74 => {
    //   block [0x82490A74..0x82490A94)
	// 82490A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490A78: 4BFFD789  bl 0x8248e200
	ctx.lr = 0x82490A7C;
	sub_8248E200(ctx, base);
	// 82490A7C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82490A80: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82490A84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82490A88: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82490A8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490A90: 4E800421  bctrl
	ctx.lr = 0x82490A94;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82490A94 => {
    //   block [0x82490A94..0x82490AAC)
	// 82490A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82490A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82490A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490AA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82490AA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490AB0 size=176
    let mut pc: u32 = 0x82490AB0;
    'dispatch: loop {
        match pc {
            0x82490AB0 => {
    //   block [0x82490AB0..0x82490B34)
	// 82490AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490AB4: 480A4609  bl 0x825350bc
	ctx.lr = 0x82490AB8;
	sub_82535080(ctx, base);
	// 82490AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490AC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82490AC4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82490AC8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82490ACC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490AD0: 419A0064  beq cr6, 0x82490b34
	if ctx.cr[6].eq {
	pc = 0x82490B34; continue 'dispatch;
	}
	// 82490AD4: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82490AD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490ADC: 419A0058  beq cr6, 0x82490b34
	if ctx.cr[6].eq {
	pc = 0x82490B34; continue 'dispatch;
	}
	// 82490AE0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490AE4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82490AE8: 3920001B  li r9, 0x1b
	ctx.r[9].s64 = 27;
	// 82490AEC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82490AF0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82490AF4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82490AF8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82490AFC: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82490B00: 4BFD3539  bl 0x82464038
	ctx.lr = 0x82490B04;
	sub_82464038(ctx, base);
	pc = 0x82490B34; continue 'dispatch;
            }
            0x82490B34 => {
    //   block [0x82490B34..0x82490B60)
	// 82490B34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490B38: 4BFFD6C9  bl 0x8248e200
	ctx.lr = 0x82490B3C;
	sub_8248E200(ctx, base);
	// 82490B3C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82490B40: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82490B44: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82490B48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82490B4C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82490B50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490B54: 4E800421  bctrl
	ctx.lr = 0x82490B58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82490B5C: 480A45B0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82490B60 size=156
    let mut pc: u32 = 0x82490B60;
    'dispatch: loop {
        match pc {
            0x82490B60 => {
    //   block [0x82490B60..0x82490BC4)
	// 82490B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490B68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82490B6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82490B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490B74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490B78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82490B7C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82490B80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82490B84: 419A0040  beq cr6, 0x82490bc4
	if ctx.cr[6].eq {
	pc = 0x82490BC4; continue 'dispatch;
	}
	// 82490B88: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82490B8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490B90: 419A0034  beq cr6, 0x82490bc4
	if ctx.cr[6].eq {
	pc = 0x82490BC4; continue 'dispatch;
	}
	// 82490B94: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82490B98: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490B9C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82490BA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82490BA4: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490BA8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82490BAC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82490BB0: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490BB4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82490BB8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82490BBC: 4800261D  bl 0x824931d8
	ctx.lr = 0x82490BC0;
	sub_824931D8(ctx, base);
	// 82490BC0: 48000024  b 0x82490be4
	pc = 0x82490BE4; continue 'dispatch;
            }
            0x82490BC4 => {
    //   block [0x82490BC4..0x82490BE4)
	// 82490BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490BC8: 4BFFD639  bl 0x8248e200
	ctx.lr = 0x82490BCC;
	sub_8248E200(ctx, base);
	// 82490BCC: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82490BD0: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82490BD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82490BD8: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82490BDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490BE0: 4E800421  bctrl
	ctx.lr = 0x82490BE4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82490BE4 => {
    //   block [0x82490BE4..0x82490BFC)
	// 82490BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82490BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82490BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490BF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82490BF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490C00 size=200
    let mut pc: u32 = 0x82490C00;
    'dispatch: loop {
        match pc {
            0x82490C00 => {
    //   block [0x82490C00..0x82490C38)
	// 82490C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490C04: 480A44B9  bl 0x825350bc
	ctx.lr = 0x82490C08;
	sub_82535080(ctx, base);
	// 82490C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490C0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82490C10: 83FD0010  lwz r31, 0x10(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82490C14: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82490C18: 419A00A8  beq cr6, 0x82490cc0
	if ctx.cr[6].eq {
	pc = 0x82490CC0; continue 'dispatch;
	}
	// 82490C1C: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82490C20: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82490C24: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82490C28: 419A0010  beq cr6, 0x82490c38
	if ctx.cr[6].eq {
	pc = 0x82490C38; continue 'dispatch;
	}
	// 82490C2C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82490C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82490C34: 409A0008  bne cr6, 0x82490c3c
	if !ctx.cr[6].eq {
	pc = 0x82490C3C; continue 'dispatch;
	}
	pc = 0x82490C38; continue 'dispatch;
            }
            0x82490C38 => {
    //   block [0x82490C38..0x82490C3C)
	// 82490C38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82490C3C; continue 'dispatch;
            }
            0x82490C3C => {
    //   block [0x82490C3C..0x82490C58)
	// 82490C3C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82490C40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82490C44: 409A0038  bne cr6, 0x82490c7c
	if !ctx.cr[6].eq {
	pc = 0x82490C7C; continue 'dispatch;
	}
	// 82490C48: 2F1E000B  cmpwi cr6, r30, 0xb
	ctx.cr[6].compare_i32(ctx.r[30].s32, 11, &mut ctx.xer);
	// 82490C4C: 419A000C  beq cr6, 0x82490c58
	if ctx.cr[6].eq {
	pc = 0x82490C58; continue 'dispatch;
	}
	// 82490C50: 2F1E0009  cmpwi cr6, r30, 9
	ctx.cr[6].compare_i32(ctx.r[30].s32, 9, &mut ctx.xer);
	// 82490C54: 409A0028  bne cr6, 0x82490c7c
	if !ctx.cr[6].eq {
	pc = 0x82490C7C; continue 'dispatch;
	}
	pc = 0x82490C58; continue 'dispatch;
            }
            0x82490C58 => {
    //   block [0x82490C58..0x82490C7C)
	// 82490C58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490C60: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82490C64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490C68: 4E800421  bctrl
	ctx.lr = 0x82490C6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490C6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490C70: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82490C74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490C78: 4E800421  bctrl
	ctx.lr = 0x82490C7C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82490C7C => {
    //   block [0x82490C7C..0x82490CA4)
	// 82490C7C: 2F1E0008  cmpwi cr6, r30, 8
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8, &mut ctx.xer);
	// 82490C80: 409A0024  bne cr6, 0x82490ca4
	if !ctx.cr[6].eq {
	pc = 0x82490CA4; continue 'dispatch;
	}
	// 82490C84: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82490C88: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82490C8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490C90: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82490C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490C98: 4E800421  bctrl
	ctx.lr = 0x82490C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490C9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82490CA0: 480A446C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82490CA4 => {
    //   block [0x82490CA4..0x82490CC0)
	// 82490CA4: 2F1E000A  cmpwi cr6, r30, 0xa
	ctx.cr[6].compare_i32(ctx.r[30].s32, 10, &mut ctx.xer);
	// 82490CA8: 409A0018  bne cr6, 0x82490cc0
	if !ctx.cr[6].eq {
	pc = 0x82490CC0; continue 'dispatch;
	}
	// 82490CAC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82490CB0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82490CB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82490CB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490CBC: 4E800421  bctrl
	ctx.lr = 0x82490CC0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82490CC0 => {
    //   block [0x82490CC0..0x82490CC8)
	// 82490CC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82490CC4: 480A4448  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82490CC8 size=340
    let mut pc: u32 = 0x82490CC8;
    'dispatch: loop {
        match pc {
            0x82490CC8 => {
    //   block [0x82490CC8..0x82490E1C)
	// 82490CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490CCC: 480A43F1  bl 0x825350bc
	ctx.lr = 0x82490CD0;
	sub_82535080(ctx, base);
	// 82490CD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490CD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490CD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82490CDC: 394001A0  li r10, 0x1a0
	ctx.r[10].s64 = 416;
	// 82490CE0: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82490CE4: 390001B0  li r8, 0x1b0
	ctx.r[8].s64 = 432;
	// 82490CE8: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82490CEC: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82490CF0: 3BBF00D0  addi r29, r31, 0xd0
	ctx.r[29].s64 = ctx.r[31].s64 + 208;
	// 82490CF4: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82490CF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82490CFC: 997E00B6  stb r11, 0xb6(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(182 as u32), ctx.r[11].u8 ) };
	// 82490D00: 897F00BC  lbz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82490D04: 997E00B4  stb r11, 0xb4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[11].u8 ) };
	// 82490D08: 897F00BD  lbz r11, 0xbd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(189 as u32) ) } as u64;
	// 82490D0C: 997E00B5  stb r11, 0xb5(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(181 as u32), ctx.r[11].u8 ) };
	// 82490D10: A17F0096  lhz r11, 0x96(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(150 as u32) ) } as u64;
	// 82490D14: B17E000A  sth r11, 0xa(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82490D18: 897F00D9  lbz r11, 0xd9(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(217 as u32) ) } as u64;
	// 82490D1C: 396BFF01  addi r11, r11, -0xff
	ctx.r[11].s64 = ctx.r[11].s64 + -255;
	// 82490D20: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82490D24: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82490D28: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82490D2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82490D30: 997E00B1  stb r11, 0xb1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(177 as u32), ctx.r[11].u8 ) };
	// 82490D34: C01F0084  lfs f0, 0x84(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490D38: D01E009C  stfs f0, 0x9c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82490D3C: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82490D40: 997E0008  stb r11, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82490D44: C01F0088  lfs f0, 0x88(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490D48: D01E00A0  stfs f0, 0xa0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82490D4C: C01F0184  lfs f0, 0x184(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490D50: D01E0094  stfs f0, 0x94(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82490D54: C01F0188  lfs f0, 0x188(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82490D58: D01E0098  stfs f0, 0x98(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490E20 size=352
    let mut pc: u32 = 0x82490E20;
    'dispatch: loop {
        match pc {
            0x82490E20 => {
    //   block [0x82490E20..0x82490E8C)
	// 82490E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490E24: 480A4295  bl 0x825350b8
	ctx.lr = 0x82490E28;
	sub_82535080(ctx, base);
	// 82490E28: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490E30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82490E34: 480010C5  bl 0x82491ef8
	ctx.lr = 0x82490E38;
	sub_82491EF8(ctx, base);
	// 82490E38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82490E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490E40: 4BFFFE89  bl 0x82490cc8
	ctx.lr = 0x82490E44;
	sub_82490CC8(ctx, base);
	// 82490E44: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490E48: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82490E4C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82490E50: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 82490E54: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82490E58: 4BFD31E1  bl 0x82464038
	ctx.lr = 0x82490E5C;
	sub_82464038(ctx, base);
	// 82490E5C: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 82490E60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82490E64: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82490E68: 4BFFF1B1  bl 0x82490018
	ctx.lr = 0x82490E6C;
	sub_82490018(ctx, base);
	// 82490E6C: 39600012  li r11, 0x12
	ctx.r[11].s64 = 18;
	// 82490E70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82490E74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490E78: 397F00D0  addi r11, r31, 0xd0
	ctx.r[11].s64 = ctx.r[31].s64 + 208;
	// 82490E7C: 7D3EF850  subf r9, r30, r31
	ctx.r[9].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 82490E80: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82490E84: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82490E88: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	pc = 0x82490E8C; continue 'dispatch;
            }
            0x82490E8C => {
    //   block [0x82490E8C..0x82490F08)
	// 82490E8C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490E90: 910AFFF8  stw r8, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[8].u32 ) };
	// 82490E94: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82490E98: 910AFFFC  stw r8, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 82490E9C: 7D09502E  lwzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82490EA0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82490EA4: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82490EA8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82490EAC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82490EB0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82490EB4: 4200FFD8  bdnz 0x82490e8c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82490E8C; continue 'dispatch;
	}
	// 82490EB8: 817F01E8  lwz r11, 0x1e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82490EBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82490EC0: 419A007C  beq cr6, 0x82490f3c
	if ctx.cr[6].eq {
	pc = 0x82490F3C; continue 'dispatch;
	}
	// 82490EC4: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82490EC8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82490ECC: 38800120  li r4, 0x120
	ctx.r[4].s64 = 288;
	// 82490ED0: 4BFD3169  bl 0x82464038
	ctx.lr = 0x82490ED4;
	sub_82464038(ctx, base);
	// 82490ED4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82490ED8: 39400120  li r10, 0x120
	ctx.r[10].s64 = 288;
	// 82490EDC: 396BC9AC  addi r11, r11, -0x3654
	ctx.r[11].s64 = ctx.r[11].s64 + -13908;
	// 82490EE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82490EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82490EE8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82490EEC: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82490EF0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82490EF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82490EF8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82490EFC: 90E30118  stw r7, 0x118(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(280 as u32), ctx.r[7].u32 ) };
	// 82490F00: 907E01E8  stw r3, 0x1e8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(488 as u32), ctx.r[3].u32 ) };
	// 82490F04: 817F01E8  lwz r11, 0x1e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	pc = 0x82490F08; continue 'dispatch;
            }
            0x82490F08 => {
    //   block [0x82490F08..0x82490F3C)
	// 82490F08: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490F0C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82490F10: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82490F14: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82490F18: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82490F1C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82490F20: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82490F24: 910A0008  stw r8, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82490F28: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82490F2C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82490F30: 910A000C  stw r8, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82490F34: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82490F38: 4199FFD0  bgt cr6, 0x82490f08
	if ctx.cr[6].gt {
	pc = 0x82490F08; continue 'dispatch;
	}
	pc = 0x82490F3C; continue 'dispatch;
            }
            0x82490F3C => {
    //   block [0x82490F3C..0x82490F80)
	// 82490F3C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490F40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82490F44: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82490F48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82490F4C: 4E800421  bctrl
	ctx.lr = 0x82490F50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82490F50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82490F54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82490F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82490F5C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82490F60: 48000091  bl 0x82490ff0
	ctx.lr = 0x82490F64;
	sub_82490FF0(ctx, base);
	// 82490F64: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82490F68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82490F6C: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82490F70: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82490F74: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82490F78: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82490F7C: 480A418C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490F80 size=112
    let mut pc: u32 = 0x82490F80;
    'dispatch: loop {
        match pc {
            0x82490F80 => {
    //   block [0x82490F80..0x82490FD4)
	// 82490F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490F88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82490F8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82490F90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82490F94: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82490F98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82490F9C: 396BD8C4  addi r11, r11, -0x273c
	ctx.r[11].s64 = ctx.r[11].s64 + -10044;
	// 82490FA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82490FA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82490FA8: 4BFFE229  bl 0x8248f1d0
	ctx.lr = 0x82490FAC;
	sub_8248F1D0(ctx, base);
	// 82490FAC: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82490FB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82490FB4: 419A0020  beq cr6, 0x82490fd4
	if ctx.cr[6].eq {
	pc = 0x82490FD4; continue 'dispatch;
	}
	// 82490FB8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82490FBC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82490FC0: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82490FC4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82490FC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82490FCC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82490FD0: 4BFD30E9  bl 0x824640b8
	ctx.lr = 0x82490FD4;
	sub_824640B8(ctx, base);
	pc = 0x82490FD4; continue 'dispatch;
            }
            0x82490FD4 => {
    //   block [0x82490FD4..0x82490FF0)
	// 82490FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82490FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82490FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82490FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82490FE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82490FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82490FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82490FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82490FF0 size=232
    let mut pc: u32 = 0x82490FF0;
    'dispatch: loop {
        match pc {
            0x82490FF0 => {
    //   block [0x82490FF0..0x82491048)
	// 82490FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82490FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82490FF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82490FFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82491000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491004: 3BE30074  addi r31, r3, 0x74
	ctx.r[31].s64 = ctx.r[3].s64 + 116;
	// 82491008: 3BC40074  addi r30, r4, 0x74
	ctx.r[30].s64 = ctx.r[4].s64 + 116;
	// 8249100C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82491010: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82491014: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82491018: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8249101C: 40980060  bge cr6, 0x8249107c
	if !ctx.cr[6].lt {
	pc = 0x8249107C; continue 'dispatch;
	}
	// 82491020: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82491024: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82491028: 409A0020  bne cr6, 0x82491048
	if !ctx.cr[6].eq {
	pc = 0x82491048; continue 'dispatch;
	}
	// 8249102C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491030: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82491034: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82491038: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249103C: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82491040: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82491044: 4BFD3075  bl 0x824640b8
	ctx.lr = 0x82491048;
	sub_824640B8(ctx, base);
	pc = 0x82491048; continue 'dispatch;
            }
            0x82491048 => {
    //   block [0x82491048..0x8249107C)
	// 82491048: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249104C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82491050: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82491054: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82491058: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8249105C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82491060: 4BFD2FD9  bl 0x82464038
	ctx.lr = 0x82491064;
	sub_82464038(ctx, base);
	// 82491064: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82491068: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8249106C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82491070: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82491074: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82491078: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x8249107C; continue 'dispatch;
            }
            0x8249107C => {
    //   block [0x8249107C..0x82491098)
	// 8249107C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82491080: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491084: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82491088: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8249108C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491090: 40990030  ble cr6, 0x824910c0
	if !ctx.cr[6].gt {
	pc = 0x824910C0; continue 'dispatch;
	}
	// 82491094: 7CEB4850  subf r7, r11, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	pc = 0x82491098; continue 'dispatch;
            }
            0x82491098 => {
    //   block [0x82491098..0x824910C0)
	// 82491098: 7D075A14  add r8, r7, r11
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8249109C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 824910A0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824910A4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824910A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824910AC: E8C80000  ld r6, 0(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 824910B0: F8C90000  std r6, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 824910B4: E9080008  ld r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 824910B8: F9090008  std r8, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 824910BC: 409AFFDC  bne cr6, 0x82491098
	if !ctx.cr[6].eq {
	pc = 0x82491098; continue 'dispatch;
	}
	pc = 0x824910C0; continue 'dispatch;
            }
            0x824910C0 => {
    //   block [0x824910C0..0x824910D8)
	// 824910C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824910C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824910C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824910CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824910D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824910D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491108 size=44
    let mut pc: u32 = 0x82491108;
    'dispatch: loop {
        match pc {
            0x82491108 => {
    //   block [0x82491108..0x82491134)
	// 82491108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8249110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491110: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491114: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491118: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8249111C: 48005FAD  bl 0x824970c8
	ctx.lr = 0x82491120;
	sub_824970C8(ctx, base);
	// 82491120: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82491124: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 82491128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8249112C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491138 size=4
    let mut pc: u32 = 0x82491138;
    'dispatch: loop {
        match pc {
            0x82491138 => {
    //   block [0x82491138..0x8249113C)
	// 82491138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491140 size=20
    let mut pc: u32 = 0x82491140;
    'dispatch: loop {
        match pc {
            0x82491140 => {
    //   block [0x82491140..0x82491154)
	// 82491140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491144: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491148: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249114C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491150: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491158 size=4
    let mut pc: u32 = 0x82491158;
    'dispatch: loop {
        match pc {
            0x82491158 => {
    //   block [0x82491158..0x8249115C)
	// 82491158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491160 size=32
    let mut pc: u32 = 0x82491160;
    'dispatch: loop {
        match pc {
            0x82491160 => {
    //   block [0x82491160..0x82491180)
	// 82491160: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491164: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491168: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8249116C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491170: 396BE0EC  addi r11, r11, -0x1f14
	ctx.r[11].s64 = ctx.r[11].s64 + -7956;
	// 82491174: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491178: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8249117C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491180 size=12
    let mut pc: u32 = 0x82491180;
    'dispatch: loop {
        match pc {
            0x82491180 => {
    //   block [0x82491180..0x8249118C)
	// 82491180: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491184: 386BE0EC  addi r3, r11, -0x1f14
	ctx.r[3].s64 = ctx.r[11].s64 + -7956;
	// 82491188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491190 size=4
    let mut pc: u32 = 0x82491190;
    'dispatch: loop {
        match pc {
            0x82491190 => {
    //   block [0x82491190..0x82491194)
	// 82491190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491198 size=20
    let mut pc: u32 = 0x82491198;
    'dispatch: loop {
        match pc {
            0x82491198 => {
    //   block [0x82491198..0x824911AC)
	// 82491198: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249119C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824911A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824911A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824911A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824911B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824911B0 size=4
    let mut pc: u32 = 0x824911B0;
    'dispatch: loop {
        match pc {
            0x824911B0 => {
    //   block [0x824911B0..0x824911B4)
	// 824911B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824911B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824911B8 size=32
    let mut pc: u32 = 0x824911B8;
    'dispatch: loop {
        match pc {
            0x824911B8 => {
    //   block [0x824911B8..0x824911D8)
	// 824911B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824911BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824911C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824911C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824911C8: 396BE1B4  addi r11, r11, -0x1e4c
	ctx.r[11].s64 = ctx.r[11].s64 + -7756;
	// 824911CC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824911D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824911D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824911D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824911D8 size=12
    let mut pc: u32 = 0x824911D8;
    'dispatch: loop {
        match pc {
            0x824911D8 => {
    //   block [0x824911D8..0x824911E4)
	// 824911D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824911DC: 386BE1B4  addi r3, r11, -0x1e4c
	ctx.r[3].s64 = ctx.r[11].s64 + -7756;
	// 824911E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824911E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824911E8 size=4
    let mut pc: u32 = 0x824911E8;
    'dispatch: loop {
        match pc {
            0x824911E8 => {
    //   block [0x824911E8..0x824911EC)
	// 824911E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824911F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824911F0 size=20
    let mut pc: u32 = 0x824911F0;
    'dispatch: loop {
        match pc {
            0x824911F0 => {
    //   block [0x824911F0..0x82491204)
	// 824911F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824911F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824911F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824911FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491200: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491208 size=4
    let mut pc: u32 = 0x82491208;
    'dispatch: loop {
        match pc {
            0x82491208 => {
    //   block [0x82491208..0x8249120C)
	// 82491208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491210 size=16
    let mut pc: u32 = 0x82491210;
    'dispatch: loop {
        match pc {
            0x82491210 => {
    //   block [0x82491210..0x82491220)
	// 82491210: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82491214: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491218: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8249121C: 4800003C  b 0x82491258
	sub_82491258(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491220 size=4
    let mut pc: u32 = 0x82491220;
    'dispatch: loop {
        match pc {
            0x82491220 => {
    //   block [0x82491220..0x82491224)
	// 82491220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491228 size=44
    let mut pc: u32 = 0x82491228;
    'dispatch: loop {
        match pc {
            0x82491228 => {
    //   block [0x82491228..0x82491254)
	// 82491228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8249122C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491230: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491234: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8249123C: 4800001D  bl 0x82491258
	ctx.lr = 0x82491240;
	sub_82491258(ctx, base);
	// 82491240: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82491244: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82491248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8249124C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491258 size=100
    let mut pc: u32 = 0x82491258;
    'dispatch: loop {
        match pc {
            0x82491258 => {
    //   block [0x82491258..0x824912A8)
	// 82491258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8249125C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82491264: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491268: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8249126C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82491270: 396BE294  addi r11, r11, -0x1d6c
	ctx.r[11].s64 = ctx.r[11].s64 + -7532;
	// 82491274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491278: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8249127C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491280: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491284: 419A0024  beq cr6, 0x824912a8
	if ctx.cr[6].eq {
	pc = 0x824912A8; continue 'dispatch;
	}
	// 82491288: 897F009A  lbz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 8249128C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82491290: 419A0018  beq cr6, 0x824912a8
	if ctx.cr[6].eq {
	pc = 0x824912A8; continue 'dispatch;
	}
	// 82491294: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82491298: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8249129C: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824912A0: 48017499  bl 0x824a8738
	ctx.lr = 0x824912A4;
	sub_824A8738(ctx, base);
	// 824912A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x824912A8; continue 'dispatch;
            }
            0x824912A8 => {
    //   block [0x824912A8..0x824912BC)
	// 824912A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824912AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824912B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824912B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824912B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824912C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824912C0 size=20
    let mut pc: u32 = 0x824912C0;
    'dispatch: loop {
        match pc {
            0x824912C0 => {
    //   block [0x824912C0..0x824912D4)
	// 824912C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824912C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824912C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824912CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824912D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824912D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824912D8 size=32
    let mut pc: u32 = 0x824912D8;
    'dispatch: loop {
        match pc {
            0x824912D8 => {
    //   block [0x824912D8..0x824912F8)
	// 824912D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824912DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824912E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824912E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824912E8: 396BE320  addi r11, r11, -0x1ce0
	ctx.r[11].s64 = ctx.r[11].s64 + -7392;
	// 824912EC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824912F0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824912F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824912F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824912F8 size=12
    let mut pc: u32 = 0x824912F8;
    'dispatch: loop {
        match pc {
            0x824912F8 => {
    //   block [0x824912F8..0x82491304)
	// 824912F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824912FC: 386BE320  addi r3, r11, -0x1ce0
	ctx.r[3].s64 = ctx.r[11].s64 + -7392;
	// 82491300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491308 size=20
    let mut pc: u32 = 0x82491308;
    'dispatch: loop {
        match pc {
            0x82491308 => {
    //   block [0x82491308..0x8249131C)
	// 82491308: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249130C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491310: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491314: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491318: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491320 size=32
    let mut pc: u32 = 0x82491320;
    'dispatch: loop {
        match pc {
            0x82491320 => {
    //   block [0x82491320..0x82491340)
	// 82491320: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491324: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491328: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8249132C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491330: 396BE39C  addi r11, r11, -0x1c64
	ctx.r[11].s64 = ctx.r[11].s64 + -7268;
	// 82491334: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491338: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8249133C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491340 size=12
    let mut pc: u32 = 0x82491340;
    'dispatch: loop {
        match pc {
            0x82491340 => {
    //   block [0x82491340..0x8249134C)
	// 82491340: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491344: 386BE39C  addi r3, r11, -0x1c64
	ctx.r[3].s64 = ctx.r[11].s64 + -7268;
	// 82491348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491350 size=100
    let mut pc: u32 = 0x82491350;
    'dispatch: loop {
        match pc {
            0x82491350 => {
    //   block [0x82491350..0x82491398)
	// 82491350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8249135C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82491360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82491368: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8249136C: 480144D5  bl 0x824a5840
	ctx.lr = 0x82491370;
	sub_824A5840(ctx, base);
	// 82491370: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82491374: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82491378: 419A0020  beq cr6, 0x82491398
	if ctx.cr[6].eq {
	pc = 0x82491398; continue 'dispatch;
	}
	// 8249137C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491380: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82491384: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82491388: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8249138C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82491390: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82491394: 4BFD2D25  bl 0x824640b8
	ctx.lr = 0x82491398;
	sub_824640B8(ctx, base);
	pc = 0x82491398; continue 'dispatch;
            }
            0x82491398 => {
    //   block [0x82491398..0x824913B4)
	// 82491398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8249139C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824913A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824913A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824913A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824913AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824913B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824913B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824913B8 size=20
    let mut pc: u32 = 0x824913B8;
    'dispatch: loop {
        match pc {
            0x824913B8 => {
    //   block [0x824913B8..0x824913CC)
	// 824913B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824913BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824913C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824913C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824913C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824913D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824913D0 size=44
    let mut pc: u32 = 0x824913D0;
    'dispatch: loop {
        match pc {
            0x824913D0 => {
    //   block [0x824913D0..0x824913FC)
	// 824913D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824913D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824913D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824913DC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824913E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824913E4: 394AE418  addi r10, r10, -0x1be8
	ctx.r[10].s64 = ctx.r[10].s64 + -7144;
	// 824913E8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 824913EC: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824913F0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824913F4: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824913F8: 48011AC0  b 0x824a2eb8
	sub_824A2EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824913FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824913FC size=4
    let mut pc: u32 = 0x824913FC;
    'dispatch: loop {
        match pc {
            0x824913FC => {
    //   block [0x824913FC..0x82491400)
	// 824913FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491400 size=64
    let mut pc: u32 = 0x82491400;
    'dispatch: loop {
        match pc {
            0x82491400 => {
    //   block [0x82491400..0x82491440)
	// 82491400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491408: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8249140C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491410: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82491414: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491418: 396BE418  addi r11, r11, -0x1be8
	ctx.r[11].s64 = ctx.r[11].s64 + -7144;
	// 8249141C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82491420: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 82491424: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82491428: 48011A91  bl 0x824a2eb8
	ctx.lr = 0x8249142C;
	sub_824A2EB8(ctx, base);
	// 8249142C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82491430: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82491434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8249143C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491440 size=100
    let mut pc: u32 = 0x82491440;
    'dispatch: loop {
        match pc {
            0x82491440 => {
    //   block [0x82491440..0x82491488)
	// 82491440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8249144C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82491450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82491458: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8249145C: 480179F5  bl 0x824a8e50
	ctx.lr = 0x82491460;
	sub_824A8E50(ctx, base);
	// 82491460: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82491464: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82491468: 419A0020  beq cr6, 0x82491488
	if ctx.cr[6].eq {
	pc = 0x82491488; continue 'dispatch;
	}
	// 8249146C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491470: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82491474: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82491478: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8249147C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82491480: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82491484: 4BFD2C35  bl 0x824640b8
	ctx.lr = 0x82491488;
	sub_824640B8(ctx, base);
	pc = 0x82491488; continue 'dispatch;
            }
            0x82491488 => {
    //   block [0x82491488..0x824914A4)
	// 82491488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8249148C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82491490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491498: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8249149C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824914A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824914A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824914A8 size=20
    let mut pc: u32 = 0x824914A8;
    'dispatch: loop {
        match pc {
            0x824914A8 => {
    //   block [0x824914A8..0x824914BC)
	// 824914A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824914AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824914B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824914B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824914B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824914C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824914C0 size=20
    let mut pc: u32 = 0x824914C0;
    'dispatch: loop {
        match pc {
            0x824914C0 => {
    //   block [0x824914C0..0x824914D4)
	// 824914C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824914C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824914C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824914CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824914D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824914D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824914D8 size=32
    let mut pc: u32 = 0x824914D8;
    'dispatch: loop {
        match pc {
            0x824914D8 => {
    //   block [0x824914D8..0x824914F8)
	// 824914D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824914DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824914E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824914E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824914E8: 396BC93C  addi r11, r11, -0x36c4
	ctx.r[11].s64 = ctx.r[11].s64 + -14020;
	// 824914EC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824914F0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824914F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824914F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824914F8 size=12
    let mut pc: u32 = 0x824914F8;
    'dispatch: loop {
        match pc {
            0x824914F8 => {
    //   block [0x824914F8..0x82491504)
	// 824914F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824914FC: 386BC93C  addi r3, r11, -0x36c4
	ctx.r[3].s64 = ctx.r[11].s64 + -14020;
	// 82491500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491508 size=32
    let mut pc: u32 = 0x82491508;
    'dispatch: loop {
        match pc {
            0x82491508 => {
    //   block [0x82491508..0x82491528)
	// 82491508: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8249150C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491510: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491514: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491518: 396BC9AC  addi r11, r11, -0x3654
	ctx.r[11].s64 = ctx.r[11].s64 + -13908;
	// 8249151C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491520: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491528 size=12
    let mut pc: u32 = 0x82491528;
    'dispatch: loop {
        match pc {
            0x82491528 => {
    //   block [0x82491528..0x82491534)
	// 82491528: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8249152C: 386BC9AC  addi r3, r11, -0x3654
	ctx.r[3].s64 = ctx.r[11].s64 + -13908;
	// 82491530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491538 size=20
    let mut pc: u32 = 0x82491538;
    'dispatch: loop {
        match pc {
            0x82491538 => {
    //   block [0x82491538..0x8249154C)
	// 82491538: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249153C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491540: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491548: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491550 size=32
    let mut pc: u32 = 0x82491550;
    'dispatch: loop {
        match pc {
            0x82491550 => {
    //   block [0x82491550..0x82491570)
	// 82491550: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491554: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491558: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8249155C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491560: 396BCB54  addi r11, r11, -0x34ac
	ctx.r[11].s64 = ctx.r[11].s64 + -13484;
	// 82491564: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491568: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8249156C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491570 size=12
    let mut pc: u32 = 0x82491570;
    'dispatch: loop {
        match pc {
            0x82491570 => {
    //   block [0x82491570..0x8249157C)
	// 82491570: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491574: 386BCB54  addi r3, r11, -0x34ac
	ctx.r[3].s64 = ctx.r[11].s64 + -13484;
	// 82491578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491580 size=20
    let mut pc: u32 = 0x82491580;
    'dispatch: loop {
        match pc {
            0x82491580 => {
    //   block [0x82491580..0x82491594)
	// 82491580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491588: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249158C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491590: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491598 size=16
    let mut pc: u32 = 0x82491598;
    'dispatch: loop {
        match pc {
            0x82491598 => {
    //   block [0x82491598..0x824915A8)
	// 82491598: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8249159C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824915A0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824915A4: 480000A4  b 0x82491648
	sub_82491648(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824915A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824915A8 size=4
    let mut pc: u32 = 0x824915A8;
    'dispatch: loop {
        match pc {
            0x824915A8 => {
    //   block [0x824915A8..0x824915AC)
	// 824915A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824915B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824915B0 size=44
    let mut pc: u32 = 0x824915B0;
    'dispatch: loop {
        match pc {
            0x824915B0 => {
    //   block [0x824915B0..0x824915DC)
	// 824915B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824915B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824915B8: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824915BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824915C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824915C4: 48000085  bl 0x82491648
	ctx.lr = 0x824915C8;
	sub_82491648(ctx, base);
	// 824915C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824915CC: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 824915D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824915D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824915D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824915E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824915E0 size=100
    let mut pc: u32 = 0x824915E0;
    'dispatch: loop {
        match pc {
            0x824915E0 => {
    //   block [0x824915E0..0x82491628)
	// 824915E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824915E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824915E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824915EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824915F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824915F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824915F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824915FC: 480185A5  bl 0x824a9ba0
	ctx.lr = 0x82491600;
	sub_824A9BA0(ctx, base);
	// 82491600: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82491604: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82491608: 419A0020  beq cr6, 0x82491628
	if ctx.cr[6].eq {
	pc = 0x82491628; continue 'dispatch;
	}
	// 8249160C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491610: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82491614: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82491618: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8249161C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82491620: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82491624: 4BFD2A95  bl 0x824640b8
	ctx.lr = 0x82491628;
	sub_824640B8(ctx, base);
	pc = 0x82491628; continue 'dispatch;
            }
            0x82491628 => {
    //   block [0x82491628..0x82491644)
	// 82491628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8249162C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82491630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491638: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8249163C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82491640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491648 size=104
    let mut pc: u32 = 0x82491648;
    'dispatch: loop {
        match pc {
            0x82491648 => {
    //   block [0x82491648..0x824916B0)
	// 82491648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8249164C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82491654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491658: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8249165C: 4800A815  bl 0x8249be70
	ctx.lr = 0x82491660;
	sub_8249BE70(ctx, base);
	// 82491660: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491664: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82491668: 392BE4FC  addi r9, r11, -0x1b04
	ctx.r[9].s64 = ctx.r[11].s64 + -6916;
	// 8249166C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82491670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82491674: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82491678: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8249167C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82491680: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82491684: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82491688: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 8249168C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82491690: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82491694: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82491698: 915F0158  stw r10, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 8249169C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824916A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824916A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824916A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824916AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824916B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824916B0 size=100
    let mut pc: u32 = 0x824916B0;
    'dispatch: loop {
        match pc {
            0x824916B0 => {
    //   block [0x824916B0..0x824916F8)
	// 824916B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824916B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824916B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824916BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824916C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824916C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824916C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824916CC: 48018F0D  bl 0x824aa5d8
	ctx.lr = 0x824916D0;
	sub_824AA5D8(ctx, base);
	// 824916D0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824916D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824916D8: 419A0020  beq cr6, 0x824916f8
	if ctx.cr[6].eq {
	pc = 0x824916F8; continue 'dispatch;
	}
	// 824916DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824916E0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824916E4: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 824916E8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824916EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824916F0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824916F4: 4BFD29C5  bl 0x824640b8
	ctx.lr = 0x824916F8;
	sub_824640B8(ctx, base);
	pc = 0x824916F8; continue 'dispatch;
            }
            0x824916F8 => {
    //   block [0x824916F8..0x82491714)
	// 824916F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824916FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82491700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491708: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8249170C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82491710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491718 size=4
    let mut pc: u32 = 0x82491718;
    'dispatch: loop {
        match pc {
            0x82491718 => {
    //   block [0x82491718..0x8249171C)
	// 82491718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491720 size=4
    let mut pc: u32 = 0x82491720;
    'dispatch: loop {
        match pc {
            0x82491720 => {
    //   block [0x82491720..0x82491724)
	// 82491720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491728 size=20
    let mut pc: u32 = 0x82491728;
    'dispatch: loop {
        match pc {
            0x82491728 => {
    //   block [0x82491728..0x8249173C)
	// 82491728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249172C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491730: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491738: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491740 size=16
    let mut pc: u32 = 0x82491740;
    'dispatch: loop {
        match pc {
            0x82491740 => {
    //   block [0x82491740..0x82491750)
	// 82491740: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82491744: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491748: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8249174C: 4BFFD13C  b 0x8248e888
	sub_8248E888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491750 size=4
    let mut pc: u32 = 0x82491750;
    'dispatch: loop {
        match pc {
            0x82491750 => {
    //   block [0x82491750..0x82491754)
	// 82491750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491758 size=44
    let mut pc: u32 = 0x82491758;
    'dispatch: loop {
        match pc {
            0x82491758 => {
    //   block [0x82491758..0x82491784)
	// 82491758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8249175C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491760: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491764: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491768: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8249176C: 4BFFD11D  bl 0x8248e888
	ctx.lr = 0x82491770;
	sub_8248E888(ctx, base);
	// 82491770: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82491774: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 82491778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8249177C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491788 size=4
    let mut pc: u32 = 0x82491788;
    'dispatch: loop {
        match pc {
            0x82491788 => {
    //   block [0x82491788..0x8249178C)
	// 82491788: 4BFFDC58  b 0x8248f3e0
	sub_8248F3E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491790 size=4
    let mut pc: u32 = 0x82491790;
    'dispatch: loop {
        match pc {
            0x82491790 => {
    //   block [0x82491790..0x82491794)
	// 82491790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491798 size=4
    let mut pc: u32 = 0x82491798;
    'dispatch: loop {
        match pc {
            0x82491798 => {
    //   block [0x82491798..0x8249179C)
	// 82491798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824917A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824917A0 size=20
    let mut pc: u32 = 0x824917A0;
    'dispatch: loop {
        match pc {
            0x824917A0 => {
    //   block [0x824917A0..0x824917B4)
	// 824917A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824917A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824917A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824917AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824917B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824917B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824917B8 size=32
    let mut pc: u32 = 0x824917B8;
    'dispatch: loop {
        match pc {
            0x824917B8 => {
    //   block [0x824917B8..0x824917D8)
	// 824917B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824917BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824917C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824917C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824917C8: 396BE914  addi r11, r11, -0x16ec
	ctx.r[11].s64 = ctx.r[11].s64 + -5868;
	// 824917CC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824917D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824917D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824917D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824917D8 size=12
    let mut pc: u32 = 0x824917D8;
    'dispatch: loop {
        match pc {
            0x824917D8 => {
    //   block [0x824917D8..0x824917E4)
	// 824917D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824917DC: 386BE914  addi r3, r11, -0x16ec
	ctx.r[3].s64 = ctx.r[11].s64 + -5868;
	// 824917E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824917E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824917E8 size=16
    let mut pc: u32 = 0x824917E8;
    'dispatch: loop {
        match pc {
            0x824917E8 => {
    //   block [0x824917E8..0x824917F8)
	// 824917E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824917EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824917F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824917F4: 48009A9C  b 0x8249b290
	sub_8249B290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824917F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824917F8 size=4
    let mut pc: u32 = 0x824917F8;
    'dispatch: loop {
        match pc {
            0x824917F8 => {
    //   block [0x824917F8..0x824917FC)
	// 824917F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491800 size=20
    let mut pc: u32 = 0x82491800;
    'dispatch: loop {
        match pc {
            0x82491800 => {
    //   block [0x82491800..0x82491814)
	// 82491800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491804: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491808: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249180C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491810: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491818 size=44
    let mut pc: u32 = 0x82491818;
    'dispatch: loop {
        match pc {
            0x82491818 => {
    //   block [0x82491818..0x82491844)
	// 82491818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8249181C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491820: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491824: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491828: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8249182C: 48009A65  bl 0x8249b290
	ctx.lr = 0x82491830;
	sub_8249B290(ctx, base);
	// 82491830: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82491834: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82491838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8249183C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491848 size=4
    let mut pc: u32 = 0x82491848;
    'dispatch: loop {
        match pc {
            0x82491848 => {
    //   block [0x82491848..0x8249184C)
	// 82491848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491850 size=20
    let mut pc: u32 = 0x82491850;
    'dispatch: loop {
        match pc {
            0x82491850 => {
    //   block [0x82491850..0x82491864)
	// 82491850: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491854: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491858: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8249185C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491860: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491868 size=4
    let mut pc: u32 = 0x82491868;
    'dispatch: loop {
        match pc {
            0x82491868 => {
    //   block [0x82491868..0x8249186C)
	// 82491868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491870 size=32
    let mut pc: u32 = 0x82491870;
    'dispatch: loop {
        match pc {
            0x82491870 => {
    //   block [0x82491870..0x82491890)
	// 82491870: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491874: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491878: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8249187C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491880: 396BEBC4  addi r11, r11, -0x143c
	ctx.r[11].s64 = ctx.r[11].s64 + -5180;
	// 82491884: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491888: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8249188C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491890 size=12
    let mut pc: u32 = 0x82491890;
    'dispatch: loop {
        match pc {
            0x82491890 => {
    //   block [0x82491890..0x8249189C)
	// 82491890: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491894: 386BEBC4  addi r3, r11, -0x143c
	ctx.r[3].s64 = ctx.r[11].s64 + -5180;
	// 82491898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824918A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824918A0 size=4
    let mut pc: u32 = 0x824918A0;
    'dispatch: loop {
        match pc {
            0x824918A0 => {
    //   block [0x824918A0..0x824918A4)
	// 824918A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824918A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824918A8 size=16
    let mut pc: u32 = 0x824918A8;
    'dispatch: loop {
        match pc {
            0x824918A8 => {
    //   block [0x824918A8..0x824918B8)
	// 824918A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824918AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824918B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824918B4: 48019EF4  b 0x824ab7a8
	sub_824AB7A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824918B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824918B8 size=4
    let mut pc: u32 = 0x824918B8;
    'dispatch: loop {
        match pc {
            0x824918B8 => {
    //   block [0x824918B8..0x824918BC)
	// 824918B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824918C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824918C0 size=20
    let mut pc: u32 = 0x824918C0;
    'dispatch: loop {
        match pc {
            0x824918C0 => {
    //   block [0x824918C0..0x824918D4)
	// 824918C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824918C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824918C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824918CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824918D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824918D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824918D8 size=44
    let mut pc: u32 = 0x824918D8;
    'dispatch: loop {
        match pc {
            0x824918D8 => {
    //   block [0x824918D8..0x82491904)
	// 824918D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824918DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824918E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824918E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824918E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824918EC: 48019EBD  bl 0x824ab7a8
	ctx.lr = 0x824918F0;
	sub_824AB7A8(ctx, base);
	// 824918F0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824918F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824918F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824918FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491908 size=4
    let mut pc: u32 = 0x82491908;
    'dispatch: loop {
        match pc {
            0x82491908 => {
    //   block [0x82491908..0x8249190C)
	// 82491908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491940 size=44
    let mut pc: u32 = 0x82491940;
    'dispatch: loop {
        match pc {
            0x82491940 => {
    //   block [0x82491940..0x8249196C)
	// 82491940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491948: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8249194C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82491954: 4800001D  bl 0x82491970
	ctx.lr = 0x82491958;
	sub_82491970(ctx, base);
	// 82491958: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8249195C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82491960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491970 size=104
    let mut pc: u32 = 0x82491970;
    'dispatch: loop {
        match pc {
            0x82491970 => {
    //   block [0x82491970..0x824919D8)
	// 82491970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491978: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8249197C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491980: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82491984: 4800A4ED  bl 0x8249be70
	ctx.lr = 0x82491988;
	sub_8249BE70(ctx, base);
	// 82491988: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8249198C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82491990: 392BEDCC  addi r9, r11, -0x1234
	ctx.r[9].s64 = ctx.r[11].s64 + -4660;
	// 82491994: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82491998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8249199C: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 824919A0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824919A4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824919A8: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 824919AC: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 824919B0: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 824919B4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824919B8: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 824919BC: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 824919C0: 915F0158  stw r10, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 824919C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824919C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824919CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824919D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824919D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824919D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824919D8 size=100
    let mut pc: u32 = 0x824919D8;
    'dispatch: loop {
        match pc {
            0x824919D8 => {
    //   block [0x824919D8..0x82491A20)
	// 824919D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824919DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824919E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824919E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824919E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824919EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824919F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824919F4: 4801A9E5  bl 0x824ac3d8
	ctx.lr = 0x824919F8;
	sub_824AC3D8(ctx, base);
	// 824919F8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824919FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82491A00: 419A0020  beq cr6, 0x82491a20
	if ctx.cr[6].eq {
	pc = 0x82491A20; continue 'dispatch;
	}
	// 82491A04: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491A08: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82491A0C: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82491A10: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82491A14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82491A18: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82491A1C: 4BFD269D  bl 0x824640b8
	ctx.lr = 0x82491A20;
	sub_824640B8(ctx, base);
	pc = 0x82491A20; continue 'dispatch;
            }
            0x82491A20 => {
    //   block [0x82491A20..0x82491A3C)
	// 82491A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82491A24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82491A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491A30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82491A34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82491A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491A40 size=20
    let mut pc: u32 = 0x82491A40;
    'dispatch: loop {
        match pc {
            0x82491A40 => {
    //   block [0x82491A40..0x82491A54)
	// 82491A40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491A44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491A48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491A4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491A50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491A58 size=32
    let mut pc: u32 = 0x82491A58;
    'dispatch: loop {
        match pc {
            0x82491A58 => {
    //   block [0x82491A58..0x82491A78)
	// 82491A58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491A5C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491A60: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491A64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491A68: 396BCAEC  addi r11, r11, -0x3514
	ctx.r[11].s64 = ctx.r[11].s64 + -13588;
	// 82491A6C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491A70: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491A78 size=12
    let mut pc: u32 = 0x82491A78;
    'dispatch: loop {
        match pc {
            0x82491A78 => {
    //   block [0x82491A78..0x82491A84)
	// 82491A78: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491A7C: 386BCAEC  addi r3, r11, -0x3514
	ctx.r[3].s64 = ctx.r[11].s64 + -13588;
	// 82491A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491A88 size=20
    let mut pc: u32 = 0x82491A88;
    'dispatch: loop {
        match pc {
            0x82491A88 => {
    //   block [0x82491A88..0x82491A9C)
	// 82491A88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491A8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491A90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491A94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491A98: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491AA0 size=32
    let mut pc: u32 = 0x82491AA0;
    'dispatch: loop {
        match pc {
            0x82491AA0 => {
    //   block [0x82491AA0..0x82491AC0)
	// 82491AA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491AA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491AA8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491AAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491AB0: 396BCA1C  addi r11, r11, -0x35e4
	ctx.r[11].s64 = ctx.r[11].s64 + -13796;
	// 82491AB4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491AB8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491AC0 size=12
    let mut pc: u32 = 0x82491AC0;
    'dispatch: loop {
        match pc {
            0x82491AC0 => {
    //   block [0x82491AC0..0x82491ACC)
	// 82491AC0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491AC4: 386BCA1C  addi r3, r11, -0x35e4
	ctx.r[3].s64 = ctx.r[11].s64 + -13796;
	// 82491AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491AD0 size=4
    let mut pc: u32 = 0x82491AD0;
    'dispatch: loop {
        match pc {
            0x82491AD0 => {
    //   block [0x82491AD0..0x82491AD4)
	// 82491AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491AD8 size=16
    let mut pc: u32 = 0x82491AD8;
    'dispatch: loop {
        match pc {
            0x82491AD8 => {
    //   block [0x82491AD8..0x82491AE8)
	// 82491AD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82491ADC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491AE0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491AE4: 4801B194  b 0x824acc78
	sub_824ACC78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491AE8 size=4
    let mut pc: u32 = 0x82491AE8;
    'dispatch: loop {
        match pc {
            0x82491AE8 => {
    //   block [0x82491AE8..0x82491AEC)
	// 82491AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491AF0 size=20
    let mut pc: u32 = 0x82491AF0;
    'dispatch: loop {
        match pc {
            0x82491AF0 => {
    //   block [0x82491AF0..0x82491B04)
	// 82491AF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491AF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491AF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491B00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491B08 size=44
    let mut pc: u32 = 0x82491B08;
    'dispatch: loop {
        match pc {
            0x82491B08 => {
    //   block [0x82491B08..0x82491B34)
	// 82491B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491B10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491B14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491B18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82491B1C: 4801B15D  bl 0x824acc78
	ctx.lr = 0x82491B20;
	sub_824ACC78(ctx, base);
	// 82491B20: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82491B24: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82491B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491B38 size=4
    let mut pc: u32 = 0x82491B38;
    'dispatch: loop {
        match pc {
            0x82491B38 => {
    //   block [0x82491B38..0x82491B3C)
	// 82491B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491B70 size=44
    let mut pc: u32 = 0x82491B70;
    'dispatch: loop {
        match pc {
            0x82491B70 => {
    //   block [0x82491B70..0x82491B9C)
	// 82491B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491B78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491B7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491B80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82491B84: 4801B685  bl 0x824ad208
	ctx.lr = 0x82491B88;
	sub_824AD208(ctx, base);
	// 82491B88: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82491B8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82491B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491BA0 size=20
    let mut pc: u32 = 0x82491BA0;
    'dispatch: loop {
        match pc {
            0x82491BA0 => {
    //   block [0x82491BA0..0x82491BB4)
	// 82491BA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491BA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491BA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491BAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491BB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491BB8 size=32
    let mut pc: u32 = 0x82491BB8;
    'dispatch: loop {
        match pc {
            0x82491BB8 => {
    //   block [0x82491BB8..0x82491BD8)
	// 82491BB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491BBC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491BC0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491BC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491BC8: 396BF07C  addi r11, r11, -0xf84
	ctx.r[11].s64 = ctx.r[11].s64 + -3972;
	// 82491BCC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491BD0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491BD8 size=12
    let mut pc: u32 = 0x82491BD8;
    'dispatch: loop {
        match pc {
            0x82491BD8 => {
    //   block [0x82491BD8..0x82491BE4)
	// 82491BD8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491BDC: 386BF07C  addi r3, r11, -0xf84
	ctx.r[3].s64 = ctx.r[11].s64 + -3972;
	// 82491BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491BE8 size=100
    let mut pc: u32 = 0x82491BE8;
    'dispatch: loop {
        match pc {
            0x82491BE8 => {
    //   block [0x82491BE8..0x82491C30)
	// 82491BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491BF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82491BF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82491BF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491BFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82491C00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82491C04: 480006ED  bl 0x824922f0
	ctx.lr = 0x82491C08;
	sub_824922F0(ctx, base);
	// 82491C08: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82491C0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82491C10: 419A0020  beq cr6, 0x82491c30
	if ctx.cr[6].eq {
	pc = 0x82491C30; continue 'dispatch;
	}
	// 82491C14: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491C18: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82491C1C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82491C20: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82491C24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82491C28: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82491C2C: 4BFD248D  bl 0x824640b8
	ctx.lr = 0x82491C30;
	sub_824640B8(ctx, base);
	pc = 0x82491C30; continue 'dispatch;
            }
            0x82491C30 => {
    //   block [0x82491C30..0x82491C4C)
	// 82491C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82491C34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82491C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491C40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82491C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82491C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491C50 size=20
    let mut pc: u32 = 0x82491C50;
    'dispatch: loop {
        match pc {
            0x82491C50 => {
    //   block [0x82491C50..0x82491C64)
	// 82491C50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491C54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491C58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491C5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491C60: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491C68 size=32
    let mut pc: u32 = 0x82491C68;
    'dispatch: loop {
        match pc {
            0x82491C68 => {
    //   block [0x82491C68..0x82491C88)
	// 82491C68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491C6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491C70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491C74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491C78: 396BCC34  addi r11, r11, -0x33cc
	ctx.r[11].s64 = ctx.r[11].s64 + -13260;
	// 82491C7C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491C80: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491C88 size=12
    let mut pc: u32 = 0x82491C88;
    'dispatch: loop {
        match pc {
            0x82491C88 => {
    //   block [0x82491C88..0x82491C94)
	// 82491C88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491C8C: 386BCC34  addi r3, r11, -0x33cc
	ctx.r[3].s64 = ctx.r[11].s64 + -13260;
	// 82491C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491C98 size=20
    let mut pc: u32 = 0x82491C98;
    'dispatch: loop {
        match pc {
            0x82491C98 => {
    //   block [0x82491C98..0x82491CAC)
	// 82491C98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491C9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491CA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491CA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491CA8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491CB0 size=32
    let mut pc: u32 = 0x82491CB0;
    'dispatch: loop {
        match pc {
            0x82491CB0 => {
    //   block [0x82491CB0..0x82491CD0)
	// 82491CB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491CB4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491CB8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491CBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491CC0: 396BF0E4  addi r11, r11, -0xf1c
	ctx.r[11].s64 = ctx.r[11].s64 + -3868;
	// 82491CC4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491CC8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491CD0 size=12
    let mut pc: u32 = 0x82491CD0;
    'dispatch: loop {
        match pc {
            0x82491CD0 => {
    //   block [0x82491CD0..0x82491CDC)
	// 82491CD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491CD4: 386BF0E4  addi r3, r11, -0xf1c
	ctx.r[3].s64 = ctx.r[11].s64 + -3868;
	// 82491CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491CE0 size=20
    let mut pc: u32 = 0x82491CE0;
    'dispatch: loop {
        match pc {
            0x82491CE0 => {
    //   block [0x82491CE0..0x82491CF4)
	// 82491CE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491CE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491CE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491CF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491CF8 size=16
    let mut pc: u32 = 0x82491CF8;
    'dispatch: loop {
        match pc {
            0x82491CF8 => {
    //   block [0x82491CF8..0x82491D08)
	// 82491CF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82491CFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491D00: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491D04: 4801BBC4  b 0x824ad8c8
	sub_824AD8C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491D08 size=4
    let mut pc: u32 = 0x82491D08;
    'dispatch: loop {
        match pc {
            0x82491D08 => {
    //   block [0x82491D08..0x82491D0C)
	// 82491D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491D10 size=44
    let mut pc: u32 = 0x82491D10;
    'dispatch: loop {
        match pc {
            0x82491D10 => {
    //   block [0x82491D10..0x82491D3C)
	// 82491D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491D1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82491D24: 4801BBA5  bl 0x824ad8c8
	ctx.lr = 0x82491D28;
	sub_824AD8C8(ctx, base);
	// 82491D28: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82491D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82491D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491D40 size=4
    let mut pc: u32 = 0x82491D40;
    'dispatch: loop {
        match pc {
            0x82491D40 => {
    //   block [0x82491D40..0x82491D44)
	// 82491D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491D48 size=20
    let mut pc: u32 = 0x82491D48;
    'dispatch: loop {
        match pc {
            0x82491D48 => {
    //   block [0x82491D48..0x82491D5C)
	// 82491D48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491D4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491D50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491D54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491D58: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491D60 size=4
    let mut pc: u32 = 0x82491D60;
    'dispatch: loop {
        match pc {
            0x82491D60 => {
    //   block [0x82491D60..0x82491D64)
	// 82491D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491D68 size=32
    let mut pc: u32 = 0x82491D68;
    'dispatch: loop {
        match pc {
            0x82491D68 => {
    //   block [0x82491D68..0x82491D88)
	// 82491D68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491D6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491D70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491D74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491D78: 396BF1A4  addi r11, r11, -0xe5c
	ctx.r[11].s64 = ctx.r[11].s64 + -3676;
	// 82491D7C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491D80: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491D88 size=12
    let mut pc: u32 = 0x82491D88;
    'dispatch: loop {
        match pc {
            0x82491D88 => {
    //   block [0x82491D88..0x82491D94)
	// 82491D88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491D8C: 386BF1A4  addi r3, r11, -0xe5c
	ctx.r[3].s64 = ctx.r[11].s64 + -3676;
	// 82491D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491D98 size=20
    let mut pc: u32 = 0x82491D98;
    'dispatch: loop {
        match pc {
            0x82491D98 => {
    //   block [0x82491D98..0x82491DAC)
	// 82491D98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491D9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491DA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491DA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491DA8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491DB0 size=32
    let mut pc: u32 = 0x82491DB0;
    'dispatch: loop {
        match pc {
            0x82491DB0 => {
    //   block [0x82491DB0..0x82491DD0)
	// 82491DB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491DB4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491DB8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491DBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491DC0: 396BF214  addi r11, r11, -0xdec
	ctx.r[11].s64 = ctx.r[11].s64 + -3564;
	// 82491DC4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491DC8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491DD0 size=12
    let mut pc: u32 = 0x82491DD0;
    'dispatch: loop {
        match pc {
            0x82491DD0 => {
    //   block [0x82491DD0..0x82491DDC)
	// 82491DD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491DD4: 386BF214  addi r3, r11, -0xdec
	ctx.r[3].s64 = ctx.r[11].s64 + -3564;
	// 82491DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491DE0 size=20
    let mut pc: u32 = 0x82491DE0;
    'dispatch: loop {
        match pc {
            0x82491DE0 => {
    //   block [0x82491DE0..0x82491DF4)
	// 82491DE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491DE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491DE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491DF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491DF8 size=32
    let mut pc: u32 = 0x82491DF8;
    'dispatch: loop {
        match pc {
            0x82491DF8 => {
    //   block [0x82491DF8..0x82491E18)
	// 82491DF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491DFC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491E00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491E04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491E08: 396BF2F4  addi r11, r11, -0xd0c
	ctx.r[11].s64 = ctx.r[11].s64 + -3340;
	// 82491E0C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491E10: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491E18 size=12
    let mut pc: u32 = 0x82491E18;
    'dispatch: loop {
        match pc {
            0x82491E18 => {
    //   block [0x82491E18..0x82491E24)
	// 82491E18: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491E1C: 386BF2F4  addi r3, r11, -0xd0c
	ctx.r[3].s64 = ctx.r[11].s64 + -3340;
	// 82491E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491E28 size=12
    let mut pc: u32 = 0x82491E28;
    'dispatch: loop {
        match pc {
            0x82491E28 => {
    //   block [0x82491E28..0x82491E34)
	// 82491E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82491E2C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82491E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82491E38 size=100
    let mut pc: u32 = 0x82491E38;
    'dispatch: loop {
        match pc {
            0x82491E38 => {
    //   block [0x82491E38..0x82491E80)
	// 82491E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82491E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82491E40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82491E44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82491E48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82491E4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82491E50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82491E54: 4801C1E5  bl 0x824ae038
	ctx.lr = 0x82491E58;
	sub_824AE038(ctx, base);
	// 82491E58: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82491E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82491E60: 419A0020  beq cr6, 0x82491e80
	if ctx.cr[6].eq {
	pc = 0x82491E80; continue 'dispatch;
	}
	// 82491E64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491E68: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82491E6C: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82491E70: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82491E74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82491E78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82491E7C: 4BFD223D  bl 0x824640b8
	ctx.lr = 0x82491E80;
	sub_824640B8(ctx, base);
	pc = 0x82491E80; continue 'dispatch;
            }
            0x82491E80 => {
    //   block [0x82491E80..0x82491E9C)
	// 82491E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82491E84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82491E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82491E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82491E90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82491E94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82491E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491EA0 size=4
    let mut pc: u32 = 0x82491EA0;
    'dispatch: loop {
        match pc {
            0x82491EA0 => {
    //   block [0x82491EA0..0x82491EA4)
	// 82491EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491EA8 size=20
    let mut pc: u32 = 0x82491EA8;
    'dispatch: loop {
        match pc {
            0x82491EA8 => {
    //   block [0x82491EA8..0x82491EBC)
	// 82491EA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491EAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82491EB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82491EB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82491EB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491EC0 size=4
    let mut pc: u32 = 0x82491EC0;
    'dispatch: loop {
        match pc {
            0x82491EC0 => {
    //   block [0x82491EC0..0x82491EC4)
	// 82491EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491EC8 size=32
    let mut pc: u32 = 0x82491EC8;
    'dispatch: loop {
        match pc {
            0x82491EC8 => {
    //   block [0x82491EC8..0x82491EE8)
	// 82491EC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82491ECC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82491ED0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491ED4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82491ED8: 396BF384  addi r11, r11, -0xc7c
	ctx.r[11].s64 = ctx.r[11].s64 + -3196;
	// 82491EDC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82491EE0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82491EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82491EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82491EE8 size=12
    let mut pc: u32 = 0x82491EE8;
    'dispatch: loop {
        match pc {
            0x82491EE8 => {
    //   block [0x82491EE8..0x82491EF4)
	// 82491EE8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82491EEC: 386BF384  addi r3, r11, -0xc7c
	ctx.r[3].s64 = ctx.r[11].s64 + -3196;
	// 82491EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


