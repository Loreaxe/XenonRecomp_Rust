pub fn sub_832C1C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C1C08 size=556
    let mut pc: u32 = 0x832C1C08;
    'dispatch: loop {
        match pc {
            0x832C1C08 => {
    //   block [0x832C1C08..0x832C1C74)
	// 832C1C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C1C0C: 4B9E77F5  bl 0x82ca9400
	ctx.lr = 0x832C1C10;
	sub_82CA93D0(ctx, base);
	// 832C1C10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C1C14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1C18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 832C1C1C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C1C20: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 832C1C24: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C1C28: 409A0204  bne cr6, 0x832c1e2c
	if !ctx.cr[6].eq {
	pc = 0x832C1E2C; continue 'dispatch;
	}
	// 832C1C2C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 832C1C30: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 832C1C34: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C1C38: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1C3C: 83BC0004  lwz r29, 4(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C1C40: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832C1C44: 40980030  bge cr6, 0x832c1c74
	if !ctx.cr[6].lt {
	pc = 0x832C1C74; continue 'dispatch;
	}
	// 832C1C48: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1C4C: 20CB0020  subfic r6, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[6].s64 = (32 as i64) - ctx.r[11].s64;
	// 832C1C50: 7C8A5850  subf r4, r10, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 832C1C54: 7CE55030  slw r5, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[7].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1C58: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 832C1C5C: 7CA94B78  or r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 | ctx.r[9].u64;
	// 832C1C60: 7D063430  srw r6, r8, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1C64: 7CFE2430  srw r30, r7, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[7].u32) >> ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1C68: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 832C1C6C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832C1C70: 48000014  b 0x832c1c84
	pc = 0x832C1C84; continue 'dispatch;
            }
            0x832C1C74 => {
    //   block [0x832C1C74..0x832C1C84)
	// 832C1C74: 20EB0020  subfic r7, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[7].s64 = (32 as i64) - ctx.r[11].s64;
	// 832C1C78: 7FEB5050  subf r31, r11, r10
	ctx.r[31].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 832C1C7C: 7D063C30  srw r6, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1C80: 7D3E5C30  srw r30, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[9].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x832C1C84; continue 'dispatch;
            }
            0x832C1C84 => {
    //   block [0x832C1C84..0x832C1CC8)
	// 832C1C84: 7CC54838  and r5, r6, r9
	ctx.r[5].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 832C1C88: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 832C1C8C: 419A0184  beq cr6, 0x832c1e10
	if ctx.cr[6].eq {
	pc = 0x832C1E10; continue 'dispatch;
	}
	// 832C1C90: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 832C1C94: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832C1C98: 41990178  bgt cr6, 0x832c1e10
	if ctx.cr[6].gt {
	pc = 0x832C1E10; continue 'dispatch;
	}
	// 832C1C9C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C1CA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832C1CA4: 7D4B2A14  add r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 832C1CA8: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832C1CAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832C1CB0: 409A0018  bne cr6, 0x832c1cc8
	if !ctx.cr[6].eq {
	pc = 0x832C1CC8; continue 'dispatch;
	}
	// 832C1CB4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1CB8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832C1CBC: 3BE0001F  li r31, 0x1f
	ctx.r[31].s64 = 31;
	// 832C1CC0: 555EF87E  srwi r30, r10, 1
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C1CC4: 48000010  b 0x832c1cd4
	pc = 0x832C1CD4; continue 'dispatch;
            }
            0x832C1CC8 => {
    //   block [0x832C1CC8..0x832C1CD4)
	// 832C1CC8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 832C1CCC: 57DEF87E  srwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C1CD0: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	pc = 0x832C1CD4; continue 'dispatch;
            }
            0x832C1CD4 => {
    //   block [0x832C1CD4..0x832C1CEC)
	// 832C1CD4: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C1CD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C1CDC: 409A00C8  bne cr6, 0x832c1da4
	if !ctx.cr[6].eq {
	pc = 0x832C1DA4; continue 'dispatch;
	}
	// 832C1CE0: 80E30020  lwz r7, 0x20(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C1CE4: 38830010  addi r4, r3, 0x10
	ctx.r[4].s64 = ctx.r[3].s64 + 16;
	// 832C1CE8: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	pc = 0x832C1CEC; continue 'dispatch;
            }
            0x832C1CEC => {
    //   block [0x832C1CEC..0x832C1D28)
	// 832C1CEC: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 832C1CF0: 7F1F3840  cmplw cr6, r31, r7
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C1CF4: 41980034  blt cr6, 0x832c1d28
	if ctx.cr[6].lt {
	pc = 0x832C1D28; continue 'dispatch;
	}
	// 832C1CF8: 21470020  subfic r10, r7, 0x20
	ctx.xer.ca = ctx.r[7].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[7].s64;
	// 832C1CFC: 7D095430  srw r9, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1D00: 7D2AF038  and r10, r9, r30
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 832C1D04: 7D2A30AE  lbzx r9, r10, r6
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 832C1D08: 552A073E  clrlwi r10, r9, 0x1c
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 832C1D0C: 5529E13E  srwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C1D10: 7FDE4C30  srw r30, r30, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1D14: 7D4A20AE  lbzx r10, r10, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832C1D18: 7FE9F850  subf r31, r9, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[9].s64;
	// 832C1D1C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 832C1D20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C1D24: 48000064  b 0x832c1d88
	pc = 0x832C1D88; continue 'dispatch;
            }
            0x832C1D28 => {
    //   block [0x832C1D28..0x832C1D74)
	// 832C1D28: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 832C1D2C: 409800E4  bge cr6, 0x832c1e10
	if !ctx.cr[6].lt {
	pc = 0x832C1E10; continue 'dispatch;
	}
	// 832C1D30: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1D34: 21470020  subfic r10, r7, 0x20
	ctx.xer.ca = ctx.r[7].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[7].s64;
	// 832C1D38: 7D3AF830  slw r26, r9, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[26].u64 = 0;
	} else {
		ctx.r[26].u64 = ((ctx.r[9].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1D3C: 7D0A5430  srw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1D40: 7F5AF378  or r26, r26, r30
	ctx.r[26].u64 = ctx.r[26].u64 | ctx.r[30].u64;
	// 832C1D44: 7D4AD038  and r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[26].u64;
	// 832C1D48: 7D4A30AE  lbzx r10, r10, r6
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 832C1D4C: 555A073E  clrlwi r26, r10, 0x1c
	ctx.r[26].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 832C1D50: 554AE13E  srwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C1D54: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C1D58: 7F5A20AE  lbzx r26, r26, r4
	ctx.r[26].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832C1D5C: 9B4B0000  stb r26, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 832C1D60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C1D64: 41980010  blt cr6, 0x832c1d74
	if ctx.cr[6].lt {
	pc = 0x832C1D74; continue 'dispatch;
	}
	// 832C1D68: 7FDE5430  srw r30, r30, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1D6C: 7FEAF850  subf r31, r10, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 832C1D70: 48000018  b 0x832c1d88
	pc = 0x832C1D88; continue 'dispatch;
            }
            0x832C1D74 => {
    //   block [0x832C1D74..0x832C1D88)
	// 832C1D74: 7FDF5050  subf r30, r31, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[31].s64;
	// 832C1D78: 7D4AF850  subf r10, r10, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 832C1D7C: 7D3EF430  srw r30, r9, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[9].u32) >> ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1D80: 3BEA0020  addi r31, r10, 0x20
	ctx.r[31].s64 = ctx.r[10].s64 + 32;
	// 832C1D84: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	pc = 0x832C1D88; continue 'dispatch;
            }
            0x832C1D88 => {
    //   block [0x832C1D88..0x832C1DA4)
	// 832C1D88: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 832C1D8C: 409AFF60  bne cr6, 0x832c1cec
	if !ctx.cr[6].eq {
	pc = 0x832C1CEC; continue 'dispatch;
	}
	// 832C1D90: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832C1D94: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832C1D98: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 832C1D9C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832C1DA0: 4B9E76B0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C1DA4 => {
    //   block [0x832C1DA4..0x832C1DE8)
	// 832C1DA4: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 832C1DA8: 40980040  bge cr6, 0x832c1de8
	if !ctx.cr[6].lt {
	pc = 0x832C1DE8; continue 'dispatch;
	}
	// 832C1DAC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1DB0: 213F0004  subfic r9, r31, 4
	ctx.xer.ca = ctx.r[31].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[31].s64;
	// 832C1DB4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832C1DB8: 7D48F830  slw r8, r10, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[10].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1DBC: 7D07F378  or r7, r8, r30
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[30].u64;
	// 832C1DC0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832C1DC4: 54E4073E  clrlwi r4, r7, 0x1c
	ctx.r[4].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	// 832C1DC8: 7D5E4C30  srw r30, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1DCC: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 832C1DD0: 4B9E7BE1  bl 0x82ca99b0
	ctx.lr = 0x832C1DD4;
	sub_82CA99B0(ctx, base);
	// 832C1DD4: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832C1DD8: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832C1DDC: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 832C1DE0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832C1DE4: 4B9E766C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C1DE8 => {
    //   block [0x832C1DE8..0x832C1E10)
	// 832C1DE8: 57C4073E  clrlwi r4, r30, 0x1c
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000000Fu64;
	// 832C1DEC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832C1DF0: 57DEE13E  srwi r30, r30, 4
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shr(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C1DF4: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832C1DF8: 4B9E7BB9  bl 0x82ca99b0
	ctx.lr = 0x832C1DFC;
	sub_82CA99B0(ctx, base);
	// 832C1DFC: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832C1E00: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832C1E04: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 832C1E08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832C1E0C: 4B9E7644  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C1E10 => {
    //   block [0x832C1E10..0x832C1E2C)
	// 832C1E10: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C1E14: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 832C1E18: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832C1E1C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832C1E20: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832C1E24: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832C1E28: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	pc = 0x832C1E2C; continue 'dispatch;
            }
            0x832C1E2C => {
    //   block [0x832C1E2C..0x832C1E34)
	// 832C1E2C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832C1E30: 4B9E7620  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C1E38 size=496
    let mut pc: u32 = 0x832C1E38;
    'dispatch: loop {
        match pc {
            0x832C1E38 => {
    //   block [0x832C1E38..0x832C1E9C)
	// 832C1E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C1E3C: 4B9E75C1  bl 0x82ca93fc
	ctx.lr = 0x832C1E40;
	sub_82CA93D0(ctx, base);
	// 832C1E40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1E44: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C1E48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C1E4C: 409A01D8  bne cr6, 0x832c2024
	if !ctx.cr[6].eq {
	pc = 0x832C2024; continue 'dispatch;
	}
	// 832C1E50: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 832C1E54: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 832C1E58: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C1E5C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1E60: 80C40004  lwz r6, 4(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C1E64: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C1E68: 40980034  bge cr6, 0x832c1e9c
	if !ctx.cr[6].lt {
	pc = 0x832C1E9C; continue 'dispatch;
	}
	// 832C1E6C: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1E70: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 832C1E74: 20EA0020  subfic r7, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[7].s64 = (32 as i64) - ctx.r[10].s64;
	// 832C1E78: 7D1F5830  slw r31, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1E7C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 832C1E80: 7FEA4B78  or r10, r31, r9
	ctx.r[10].u64 = ctx.r[31].u64 | ctx.r[9].u64;
	// 832C1E84: 7FA93C30  srw r9, r29, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[29].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1E88: 7D3A5038  and r26, r9, r10
	ctx.r[26].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 832C1E8C: 7D0AF430  srw r10, r8, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1E90: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C1E94: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C1E98: 48000018  b 0x832c1eb0
	pc = 0x832C1EB0; continue 'dispatch;
            }
            0x832C1E9C => {
    //   block [0x832C1E9C..0x832C1EB0)
	// 832C1E9C: 210A0020  subfic r8, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[10].s64;
	// 832C1EA0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 832C1EA4: 7FA74430  srw r7, r29, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[29].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1EA8: 7CFA4838  and r26, r7, r9
	ctx.r[26].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 832C1EAC: 7D2A5430  srw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x832C1EB0; continue 'dispatch;
            }
            0x832C1EB0 => {
    //   block [0x832C1EB0..0x832C1EE4)
	// 832C1EB0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 832C1EB4: 419A0154  beq cr6, 0x832c2008
	if ctx.cr[6].eq {
	pc = 0x832C2008; continue 'dispatch;
	}
	// 832C1EB8: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 832C1EBC: 7F1A4840  cmplw cr6, r26, r9
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C1EC0: 41990148  bgt cr6, 0x832c2008
	if ctx.cr[6].gt {
	pc = 0x832C2008; continue 'dispatch;
	}
	// 832C1EC4: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C1EC8: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 832C1ECC: 83830020  lwz r28, 0x20(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C1ED0: 7D09D214  add r8, r9, r26
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[26].u64;
	// 832C1ED4: 83C30024  lwz r30, 0x24(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 832C1ED8: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 832C1EDC: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 832C1EE0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x832C1EE4; continue 'dispatch;
            }
            0x832C1EE4 => {
    //   block [0x832C1EE4..0x832C1F18)
	// 832C1EE4: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C1EE8: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 832C1EEC: 4198002C  blt cr6, 0x832c1f18
	if ctx.cr[6].lt {
	pc = 0x832C1F18; continue 'dispatch;
	}
	// 832C1EF0: 213C0020  subfic r9, r28, 0x20
	ctx.xer.ca = ctx.r[28].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[28].s64;
	// 832C1EF4: 7FA84C30  srw r8, r29, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[29].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1EF8: 7D075038  and r7, r8, r10
	ctx.r[7].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 832C1EFC: 7D27F0AE  lbzx r9, r7, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832C1F00: 5527073E  clrlwi r7, r9, 0x1c
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 832C1F04: 5529E13E  srwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C1F08: 7D4A4C30  srw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1F0C: 7F67F8AE  lbzx r27, r7, r31
	ctx.r[27].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 832C1F10: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C1F14: 4800005C  b 0x832c1f70
	pc = 0x832C1F70; continue 'dispatch;
            }
            0x832C1F18 => {
    //   block [0x832C1F18..0x832C1F5C)
	// 832C1F18: 7F062840  cmplw cr6, r6, r5
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[5].u32, &mut ctx.xer);
	// 832C1F1C: 409800EC  bge cr6, 0x832c2008
	if !ctx.cr[6].lt {
	pc = 0x832C2008; continue 'dispatch;
	}
	// 832C1F20: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1F24: 213C0020  subfic r9, r28, 0x20
	ctx.xer.ca = ctx.r[28].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[28].s64;
	// 832C1F28: 7CFB5830  slw r27, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[27].u64 = 0;
	} else {
		ctx.r[27].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1F2C: 7FA84C30  srw r8, r29, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[29].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1F30: 7F695378  or r9, r27, r10
	ctx.r[9].u64 = ctx.r[27].u64 | ctx.r[10].u64;
	// 832C1F34: 7D294038  and r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 832C1F38: 7D29F0AE  lbzx r9, r9, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832C1F3C: 553B073E  clrlwi r27, r9, 0x1c
	ctx.r[27].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 832C1F40: 5529E13E  srwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C1F44: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C1F48: 7F7BF8AE  lbzx r27, r27, r31
	ctx.r[27].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 832C1F4C: 41980010  blt cr6, 0x832c1f5c
	if ctx.cr[6].lt {
	pc = 0x832C1F5C; continue 'dispatch;
	}
	// 832C1F50: 7D4A4C30  srw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1F54: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C1F58: 48000018  b 0x832c1f70
	pc = 0x832C1F70; continue 'dispatch;
            }
            0x832C1F5C => {
    //   block [0x832C1F5C..0x832C1F70)
	// 832C1F5C: 7D4B4850  subf r10, r11, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 832C1F60: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C1F64: 7CEA5430  srw r10, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1F68: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C1F6C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	pc = 0x832C1F70; continue 'dispatch;
            }
            0x832C1F70 => {
    //   block [0x832C1F70..0x832C1F98)
	// 832C1F70: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 832C1F74: 41980024  blt cr6, 0x832c1f98
	if ctx.cr[6].lt {
	pc = 0x832C1F98; continue 'dispatch;
	}
	// 832C1F78: 7D095038  and r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 832C1F7C: 7D09F0AE  lbzx r8, r9, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832C1F80: 5507073E  clrlwi r7, r8, 0x1c
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 832C1F84: 5509E13E  srwi r9, r8, 4
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C1F88: 7D4A4C30  srw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1F8C: 7D07F8AE  lbzx r8, r7, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 832C1F90: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C1F94: 4800004C  b 0x832c1fe0
	pc = 0x832C1FE0; continue 'dispatch;
            }
            0x832C1F98 => {
    //   block [0x832C1F98..0x832C1FCC)
	// 832C1F98: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C1F9C: 7CE95830  slw r9, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1FA0: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 832C1FA4: 7D284038  and r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 832C1FA8: 7D28F0AE  lbzx r9, r8, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832C1FAC: 5528073E  clrlwi r8, r9, 0x1c
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 832C1FB0: 5529E13E  srwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C1FB4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C1FB8: 7D08F8AE  lbzx r8, r8, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 832C1FBC: 41980010  blt cr6, 0x832c1fcc
	if ctx.cr[6].lt {
	pc = 0x832C1FCC; continue 'dispatch;
	}
	// 832C1FC0: 7D4A4C30  srw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1FC4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C1FC8: 48000018  b 0x832c1fe0
	pc = 0x832C1FE0; continue 'dispatch;
            }
            0x832C1FCC => {
    //   block [0x832C1FCC..0x832C1FE0)
	// 832C1FCC: 7D4B4850  subf r10, r11, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 832C1FD0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C1FD4: 7CEA5430  srw r10, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 832C1FD8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C1FDC: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	pc = 0x832C1FE0; continue 'dispatch;
            }
            0x832C1FE0 => {
    //   block [0x832C1FE0..0x832C2008)
	// 832C1FE0: 55092036  slwi r9, r8, 4
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C1FE4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 832C1FE8: 7D28DB78  or r8, r9, r27
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[27].u64;
	// 832C1FEC: 99190000  stb r8, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 832C1FF0: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 832C1FF4: 409AFEF0  bne cr6, 0x832c1ee4
	if !ctx.cr[6].eq {
	pc = 0x832C1EE4; continue 'dispatch;
	}
	// 832C1FF8: 90C40004  stw r6, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C1FFC: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832C2000: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832C2004: 4B9E7448  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C2008 => {
    //   block [0x832C2008..0x832C2024)
	// 832C2008: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C200C: 39090004  addi r8, r9, 4
	ctx.r[8].s64 = ctx.r[9].s64 + 4;
	// 832C2010: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 832C2014: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832C2018: 90C40004  stw r6, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C201C: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832C2020: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x832C2024; continue 'dispatch;
            }
            0x832C2024 => {
    //   block [0x832C2024..0x832C2028)
	// 832C2024: 4B9E7428  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C2028 size=656
    let mut pc: u32 = 0x832C2028;
    'dispatch: loop {
        match pc {
            0x832C2028 => {
    //   block [0x832C2028..0x832C2094)
	// 832C2028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C202C: 4B9E73D1  bl 0x82ca93fc
	ctx.lr = 0x832C2030;
	sub_82CA93D0(ctx, base);
	// 832C2030: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C2034: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2038: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 832C203C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C2040: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 832C2044: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C2048: 409A0268  bne cr6, 0x832c22b0
	if !ctx.cr[6].eq {
	pc = 0x832C22B0; continue 'dispatch;
	}
	// 832C204C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 832C2050: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 832C2054: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C2058: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C205C: 83BB0004  lwz r29, 4(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C2060: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832C2064: 40980030  bge cr6, 0x832c2094
	if !ctx.cr[6].lt {
	pc = 0x832C2094; continue 'dispatch;
	}
	// 832C2068: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C206C: 20CB0020  subfic r6, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[6].s64 = (32 as i64) - ctx.r[11].s64;
	// 832C2070: 7C8A5850  subf r4, r10, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 832C2074: 7CE55030  slw r5, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[7].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2078: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 832C207C: 7CA94B78  or r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 | ctx.r[9].u64;
	// 832C2080: 7D063430  srw r6, r8, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2084: 7CFE2430  srw r30, r7, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[7].u32) >> ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2088: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 832C208C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832C2090: 48000014  b 0x832c20a4
	pc = 0x832C20A4; continue 'dispatch;
            }
            0x832C2094 => {
    //   block [0x832C2094..0x832C20A4)
	// 832C2094: 20EB0020  subfic r7, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[7].s64 = (32 as i64) - ctx.r[11].s64;
	// 832C2098: 7FEB5050  subf r31, r11, r10
	ctx.r[31].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 832C209C: 7D063C30  srw r6, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 832C20A0: 7D3E5C30  srw r30, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[9].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x832C20A4; continue 'dispatch;
            }
            0x832C20A4 => {
    //   block [0x832C20A4..0x832C20E8)
	// 832C20A4: 7CC54838  and r5, r6, r9
	ctx.r[5].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 832C20A8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 832C20AC: 419A01E8  beq cr6, 0x832c2294
	if ctx.cr[6].eq {
	pc = 0x832C2294; continue 'dispatch;
	}
	// 832C20B0: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 832C20B4: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832C20B8: 419901DC  bgt cr6, 0x832c2294
	if ctx.cr[6].gt {
	pc = 0x832C2294; continue 'dispatch;
	}
	// 832C20BC: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C20C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832C20C4: 7D6A2A14  add r11, r10, r5
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 832C20C8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832C20CC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832C20D0: 409A0018  bne cr6, 0x832c20e8
	if !ctx.cr[6].eq {
	pc = 0x832C20E8; continue 'dispatch;
	}
	// 832C20D4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C20D8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832C20DC: 3BE0001F  li r31, 0x1f
	ctx.r[31].s64 = 31;
	// 832C20E0: 557EF87E  srwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C20E4: 48000010  b 0x832c20f4
	pc = 0x832C20F4; continue 'dispatch;
            }
            0x832C20E8 => {
    //   block [0x832C20E8..0x832C20F4)
	// 832C20E8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832C20EC: 57DEF87E  srwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C20F0: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	pc = 0x832C20F4; continue 'dispatch;
            }
            0x832C20F4 => {
    //   block [0x832C20F4..0x832C2110)
	// 832C20F4: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 832C20F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C20FC: 409A0104  bne cr6, 0x832c2200
	if !ctx.cr[6].eq {
	pc = 0x832C2200; continue 'dispatch;
	}
	// 832C2100: 80E30020  lwz r7, 0x20(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C2104: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 832C2108: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 832C210C: 38830010  addi r4, r3, 0x10
	ctx.r[4].s64 = ctx.r[3].s64 + 16;
	pc = 0x832C2110; continue 'dispatch;
            }
            0x832C2110 => {
    //   block [0x832C2110..0x832C2144)
	// 832C2110: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 832C2114: 7F1F3840  cmplw cr6, r31, r7
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C2118: 4198002C  blt cr6, 0x832c2144
	if ctx.cr[6].lt {
	pc = 0x832C2144; continue 'dispatch;
	}
	// 832C211C: 21670020  subfic r11, r7, 0x20
	ctx.xer.ca = ctx.r[7].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[7].s64;
	// 832C2120: 7D0A5C30  srw r10, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2124: 7D49F038  and r9, r10, r30
	ctx.r[9].u64 = ctx.r[10].u64 & ctx.r[30].u64;
	// 832C2128: 7D6930AE  lbzx r11, r9, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 832C212C: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 832C2130: 556BE13E  srwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C2134: 7FDE5C30  srw r30, r30, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2138: 7D2A20AE  lbzx r9, r10, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832C213C: 7FEBF850  subf r31, r11, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 832C2140: 4800005C  b 0x832c219c
	pc = 0x832C219C; continue 'dispatch;
            }
            0x832C2144 => {
    //   block [0x832C2144..0x832C2188)
	// 832C2144: 7F1DD040  cmplw cr6, r29, r26
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[26].u32, &mut ctx.xer);
	// 832C2148: 4098014C  bge cr6, 0x832c2294
	if !ctx.cr[6].lt {
	pc = 0x832C2294; continue 'dispatch;
	}
	// 832C214C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2150: 21670020  subfic r11, r7, 0x20
	ctx.xer.ca = ctx.r[7].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[7].s64;
	// 832C2154: 7D49F830  slw r9, r10, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[10].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2158: 7D29F378  or r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[30].u64;
	// 832C215C: 7D0B5C30  srw r11, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2160: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 832C2164: 7D2B30AE  lbzx r9, r11, r6
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 832C2168: 5539073E  clrlwi r25, r9, 0x1c
	ctx.r[25].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 832C216C: 552BE13E  srwi r11, r9, 4
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C2170: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832C2174: 7D3920AE  lbzx r9, r25, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832C2178: 41980010  blt cr6, 0x832c2188
	if ctx.cr[6].lt {
	pc = 0x832C2188; continue 'dispatch;
	}
	// 832C217C: 7FDE5C30  srw r30, r30, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[30].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2180: 7FEBF850  subf r31, r11, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 832C2184: 48000018  b 0x832c219c
	pc = 0x832C219C; continue 'dispatch;
            }
            0x832C2188 => {
    //   block [0x832C2188..0x832C219C)
	// 832C2188: 7FDF5850  subf r30, r31, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 832C218C: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 832C2190: 7D5EF430  srw r30, r10, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2194: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 832C2198: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	pc = 0x832C219C; continue 'dispatch;
            }
            0x832C219C => {
    //   block [0x832C219C..0x832C21C0)
	// 832C219C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 832C21A0: 419A003C  beq cr6, 0x832c21dc
	if ctx.cr[6].eq {
	pc = 0x832C21DC; continue 'dispatch;
	}
	// 832C21A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832C21A8: 409A0018  bne cr6, 0x832c21c0
	if !ctx.cr[6].eq {
	pc = 0x832C21C0; continue 'dispatch;
	}
	// 832C21AC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C21B0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832C21B4: 3BE0001F  li r31, 0x1f
	ctx.r[31].s64 = 31;
	// 832C21B8: 557EF87E  srwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C21BC: 48000010  b 0x832c21cc
	pc = 0x832C21CC; continue 'dispatch;
            }
            0x832C21C0 => {
    //   block [0x832C21C0..0x832C21CC)
	// 832C21C0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832C21C4: 57DEF87E  srwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C21C8: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	pc = 0x832C21CC; continue 'dispatch;
            }
            0x832C21CC => {
    //   block [0x832C21CC..0x832C21DC)
	// 832C21CC: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 832C21D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C21D4: 419A0008  beq cr6, 0x832c21dc
	if ctx.cr[6].eq {
	pc = 0x832C21DC; continue 'dispatch;
	}
	// 832C21D8: 7D2900D0  neg r9, r9
	ctx.r[9].s64 = -ctx.r[9].s64;
	pc = 0x832C21DC; continue 'dispatch;
            }
            0x832C21DC => {
    //   block [0x832C21DC..0x832C2200)
	// 832C21DC: 993C0000  stb r9, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C21E0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 832C21E4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 832C21E8: 409AFF28  bne cr6, 0x832c2110
	if !ctx.cr[6].eq {
	pc = 0x832C2110; continue 'dispatch;
	}
	// 832C21EC: 93BB0004  stw r29, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832C21F0: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832C21F4: 93FB0008  stw r31, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 832C21F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832C21FC: 4B9E7250  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C2200 => {
    //   block [0x832C2200..0x832C222C)
	// 832C2200: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 832C2204: 40980028  bge cr6, 0x832c222c
	if !ctx.cr[6].lt {
	pc = 0x832C222C; continue 'dispatch;
	}
	// 832C2208: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C220C: 213F0004  subfic r9, r31, 4
	ctx.xer.ca = ctx.r[31].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[31].s64;
	// 832C2210: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832C2214: 7D68F830  slw r8, r11, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2218: 7D07F378  or r7, r8, r30
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[30].u64;
	// 832C221C: 7D7E4C30  srw r30, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[11].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2220: 54E4073E  clrlwi r4, r7, 0x1c
	ctx.r[4].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	// 832C2224: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 832C2228: 48000010  b 0x832c2238
	pc = 0x832C2238; continue 'dispatch;
            }
            0x832C222C => {
    //   block [0x832C222C..0x832C2238)
	// 832C222C: 57C4073E  clrlwi r4, r30, 0x1c
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000000Fu64;
	// 832C2230: 57DEE13E  srwi r30, r30, 4
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shr(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C2234: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	pc = 0x832C2238; continue 'dispatch;
            }
            0x832C2238 => {
    //   block [0x832C2238..0x832C225C)
	// 832C2238: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 832C223C: 419A003C  beq cr6, 0x832c2278
	if ctx.cr[6].eq {
	pc = 0x832C2278; continue 'dispatch;
	}
	// 832C2240: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832C2244: 409A0018  bne cr6, 0x832c225c
	if !ctx.cr[6].eq {
	pc = 0x832C225C; continue 'dispatch;
	}
	// 832C2248: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C224C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 832C2250: 3BE0001F  li r31, 0x1f
	ctx.r[31].s64 = 31;
	// 832C2254: 557EF87E  srwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C2258: 48000010  b 0x832c2268
	pc = 0x832C2268; continue 'dispatch;
            }
            0x832C225C => {
    //   block [0x832C225C..0x832C2268)
	// 832C225C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832C2260: 57DEF87E  srwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C2264: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	pc = 0x832C2268; continue 'dispatch;
            }
            0x832C2268 => {
    //   block [0x832C2268..0x832C2278)
	// 832C2268: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 832C226C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C2270: 419A0008  beq cr6, 0x832c2278
	if ctx.cr[6].eq {
	pc = 0x832C2278; continue 'dispatch;
	}
	// 832C2274: 7C8400D0  neg r4, r4
	ctx.r[4].s64 = -ctx.r[4].s64;
	pc = 0x832C2278; continue 'dispatch;
            }
            0x832C2278 => {
    //   block [0x832C2278..0x832C2294)
	// 832C2278: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 832C227C: 4B9E7735  bl 0x82ca99b0
	ctx.lr = 0x832C2280;
	sub_82CA99B0(ctx, base);
	// 832C2280: 93BB0004  stw r29, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832C2284: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832C2288: 93FB0008  stw r31, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 832C228C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832C2290: 4B9E71BC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C2294 => {
    //   block [0x832C2294..0x832C22B0)
	// 832C2294: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C2298: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 832C229C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832C22A0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832C22A4: 93BB0004  stw r29, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832C22A8: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832C22AC: 93FB0008  stw r31, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	pc = 0x832C22B0; continue 'dispatch;
            }
            0x832C22B0 => {
    //   block [0x832C22B0..0x832C22B8)
	// 832C22B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832C22B4: 4B9E7198  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C22B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C22B8 size=860
    let mut pc: u32 = 0x832C22B8;
    'dispatch: loop {
        match pc {
            0x832C22B8 => {
    //   block [0x832C22B8..0x832C231C)
	// 832C22B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C22BC: 4B9E713D  bl 0x82ca93f8
	ctx.lr = 0x832C22C0;
	sub_82CA93D0(ctx, base);
	// 832C22C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C22C4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C22C8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C22CC: 409A0344  bne cr6, 0x832c2610
	if !ctx.cr[6].eq {
	pc = 0x832C2610; continue 'dispatch;
	}
	// 832C22D0: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 832C22D4: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 832C22D8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C22DC: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C22E0: 80E40004  lwz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C22E4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C22E8: 40980034  bge cr6, 0x832c231c
	if !ctx.cr[6].lt {
	pc = 0x832C231C; continue 'dispatch;
	}
	// 832C22EC: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C22F0: 7FAB5050  subf r29, r11, r10
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 832C22F4: 20CA0020  subfic r6, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[6].s64 = (32 as i64) - ctx.r[10].s64;
	// 832C22F8: 7D1F5830  slw r31, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C22FC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 832C2300: 7FEA4B78  or r10, r31, r9
	ctx.r[10].u64 = ctx.r[31].u64 | ctx.r[9].u64;
	// 832C2304: 7F293430  srw r9, r25, r6
	if (ctx.r[6].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[25].u32) >> ((ctx.r[6].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2308: 7D3E5038  and r30, r9, r10
	ctx.r[30].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 832C230C: 7D0AEC30  srw r10, r8, r29
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2310: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C2314: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 832C2318: 48000018  b 0x832c2330
	pc = 0x832C2330; continue 'dispatch;
            }
            0x832C231C => {
    //   block [0x832C231C..0x832C2330)
	// 832C231C: 210A0020  subfic r8, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[10].s64;
	// 832C2320: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 832C2324: 7F264430  srw r6, r25, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[25].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2328: 7CDE4838  and r30, r6, r9
	ctx.r[30].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 832C232C: 7D2A5430  srw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x832C2330; continue 'dispatch;
            }
            0x832C2330 => {
    //   block [0x832C2330..0x832C239C)
	// 832C2330: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 832C2334: 419A02C0  beq cr6, 0x832c25f4
	if ctx.cr[6].eq {
	pc = 0x832C25F4; continue 'dispatch;
	}
	// 832C2338: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 832C233C: 5528F87E  srwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C2340: 7F1E4040  cmplw cr6, r30, r8
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[8].u32, &mut ctx.xer);
	// 832C2344: 419902B0  bgt cr6, 0x832c25f4
	if ctx.cr[6].gt {
	pc = 0x832C25F4; continue 'dispatch;
	}
	// 832C2348: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 832C234C: 80C30030  lwz r6, 0x30(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C2350: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 832C2354: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C2358: 419A00A0  beq cr6, 0x832c23f8
	if ctx.cr[6].eq {
	pc = 0x832C23F8; continue 'dispatch;
	}
	// 832C235C: 3909FFFF  addi r8, r9, -1
	ctx.r[8].s64 = ctx.r[9].s64 + -1;
	// 832C2360: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 832C2364: 40980038  bge cr6, 0x832c239c
	if !ctx.cr[6].lt {
	pc = 0x832C239C; continue 'dispatch;
	}
	// 832C2368: 83E70000  lwz r31, 0(r7)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C236C: 23A90021  subfic r29, r9, 0x21
	ctx.xer.ca = ctx.r[9].u32 <= 33 as u32;
	ctx.r[29].s64 = (33 as i64) - ctx.r[9].s64;
	// 832C2370: 7D0B4850  subf r8, r11, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 832C2374: 7FFC5830  slw r28, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[28].u64 = 0;
	} else {
		ctx.r[28].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2378: 7F8A5378  or r10, r28, r10
	ctx.r[10].u64 = ctx.r[28].u64 | ctx.r[10].u64;
	// 832C237C: 7F3DEC30  srw r29, r25, r29
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[29].u64 = 0;
	} else {
		ctx.r[29].u64 = ((ctx.r[25].u32) >> ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2380: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832C2384: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C2388: 7FBB5038  and r27, r29, r10
	ctx.r[27].u64 = ctx.r[29].u64 & ctx.r[10].u64;
	// 832C238C: 7FEA4430  srw r10, r31, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2390: 396B0021  addi r11, r11, 0x21
	ctx.r[11].s64 = ctx.r[11].s64 + 33;
	// 832C2394: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 832C2398: 4800001C  b 0x832c23b4
	pc = 0x832C23B4; continue 'dispatch;
            }
            0x832C239C => {
    //   block [0x832C239C..0x832C23B4)
	// 832C239C: 23E90021  subfic r31, r9, 0x21
	ctx.xer.ca = ctx.r[9].u32 <= 33 as u32;
	ctx.r[31].s64 = (33 as i64) - ctx.r[9].s64;
	// 832C23A0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C23A4: 7F29FC30  srw r9, r25, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[25].u32) >> ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 832C23A8: 7D3B5038  and r27, r9, r10
	ctx.r[27].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 832C23AC: 7D4A4430  srw r10, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 832C23B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	pc = 0x832C23B4; continue 'dispatch;
            }
            0x832C23B4 => {
    //   block [0x832C23B4..0x832C23D8)
	// 832C23B4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 832C23B8: 419A008C  beq cr6, 0x832c2444
	if ctx.cr[6].eq {
	pc = 0x832C2444; continue 'dispatch;
	}
	// 832C23BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C23C0: 409A0018  bne cr6, 0x832c23d8
	if !ctx.cr[6].eq {
	pc = 0x832C23D8; continue 'dispatch;
	}
	// 832C23C4: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C23C8: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 832C23CC: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C23D0: 552AF87E  srwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C23D4: 48000010  b 0x832c23e4
	pc = 0x832C23E4; continue 'dispatch;
            }
            0x832C23D8 => {
    //   block [0x832C23D8..0x832C23E4)
	// 832C23D8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 832C23DC: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C23E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C23E4; continue 'dispatch;
            }
            0x832C23E4 => {
    //   block [0x832C23E4..0x832C23F8)
	// 832C23E4: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 832C23E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C23EC: 419A0058  beq cr6, 0x832c2444
	if ctx.cr[6].eq {
	pc = 0x832C2444; continue 'dispatch;
	}
	// 832C23F0: 7F7B00D0  neg r27, r27
	ctx.r[27].s64 = -ctx.r[27].s64;
	// 832C23F4: 48000050  b 0x832c2444
	pc = 0x832C2444; continue 'dispatch;
            }
            0x832C23F8 => {
    //   block [0x832C23F8..0x832C2430)
	// 832C23F8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C23FC: 40980034  bge cr6, 0x832c2430
	if !ctx.cr[6].lt {
	pc = 0x832C2430; continue 'dispatch;
	}
	// 832C2400: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2404: 23E90020  subfic r31, r9, 0x20
	ctx.xer.ca = ctx.r[9].u32 <= 32 as u32;
	ctx.r[31].s64 = (32 as i64) - ctx.r[9].s64;
	// 832C2408: 7FAB4850  subf r29, r11, r9
	ctx.r[29].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 832C240C: 7D1C5830  slw r28, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[28].u64 = 0;
	} else {
		ctx.r[28].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2410: 7F8A5378  or r10, r28, r10
	ctx.r[10].u64 = ctx.r[28].u64 | ctx.r[10].u64;
	// 832C2414: 7F3FFC30  srw r31, r25, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[25].u32) >> ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2418: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C241C: 7FFB5038  and r27, r31, r10
	ctx.r[27].u64 = ctx.r[31].u64 & ctx.r[10].u64;
	// 832C2420: 7D0AEC30  srw r10, r8, r29
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2424: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C2428: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 832C242C: 48000018  b 0x832c2444
	pc = 0x832C2444; continue 'dispatch;
            }
            0x832C2430 => {
    //   block [0x832C2430..0x832C2444)
	// 832C2430: 21090020  subfic r8, r9, 0x20
	ctx.xer.ca = ctx.r[9].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[9].s64;
	// 832C2434: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C2438: 7F284430  srw r8, r25, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[25].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 832C243C: 7D1B5038  and r27, r8, r10
	ctx.r[27].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 832C2440: 7D4A4C30  srw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x832C2444; continue 'dispatch;
            }
            0x832C2444 => {
    //   block [0x832C2444..0x832C2468)
	// 832C2444: 57C9083C  slwi r9, r30, 1
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C2448: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 832C244C: 577F043E  clrlwi r31, r27, 0x10
	ctx.r[31].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 832C2450: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 832C2454: 371EFFFF  addic. r24, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[24].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 832C2458: B3E60000  sth r31, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 832C245C: 3B860002  addi r28, r6, 2
	ctx.r[28].s64 = ctx.r[6].s64 + 2;
	// 832C2460: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832C2464: 418201A0  beq 0x832c2604
	if ctx.cr[0].eq {
	pc = 0x832C2604; continue 'dispatch;
	}
	pc = 0x832C2468; continue 'dispatch;
            }
            0x832C2468 => {
    //   block [0x832C2468..0x832C2478)
	// 832C2468: 2B180008  cmplwi cr6, r24, 8
	ctx.cr[6].compare_u32(ctx.r[24].u32, 8 as u32, &mut ctx.xer);
	// 832C246C: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 832C2470: 41990008  bgt cr6, 0x832c2478
	if ctx.cr[6].gt {
	pc = 0x832C2478; continue 'dispatch;
	}
	// 832C2474: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	pc = 0x832C2478; continue 'dispatch;
            }
            0x832C2478 => {
    //   block [0x832C2478..0x832C24AC)
	// 832C2478: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832C247C: 40980030  bge cr6, 0x832c24ac
	if !ctx.cr[6].lt {
	pc = 0x832C24AC; continue 'dispatch;
	}
	// 832C2480: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 832C2484: 40980170  bge cr6, 0x832c25f4
	if !ctx.cr[6].lt {
	pc = 0x832C25F4; continue 'dispatch;
	}
	// 832C2488: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C248C: 210B0004  subfic r8, r11, 4
	ctx.xer.ca = ctx.r[11].u32 <= 4 as u32;
	ctx.r[8].s64 = (4 as i64) - ctx.r[11].s64;
	// 832C2490: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 832C2494: 7D265830  slw r6, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2498: 7CC65378  or r6, r6, r10
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[10].u64;
	// 832C249C: 7D2A4430  srw r10, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 832C24A0: 54C9073E  clrlwi r9, r6, 0x1c
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x0000000Fu64;
	// 832C24A4: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 832C24A8: 48000010  b 0x832c24b8
	pc = 0x832C24B8; continue 'dispatch;
            }
            0x832C24AC => {
    //   block [0x832C24AC..0x832C24B8)
	// 832C24AC: 5549073E  clrlwi r9, r10, 0x1c
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 832C24B0: 554AE13E  srwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C24B4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	pc = 0x832C24B8; continue 'dispatch;
            }
            0x832C24B8 => {
    //   block [0x832C24B8..0x832C24D4)
	// 832C24B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C24BC: 419A00B4  beq cr6, 0x832c2570
	if ctx.cr[6].eq {
	pc = 0x832C2570; continue 'dispatch;
	}
	// 832C24C0: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 832C24C4: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 832C24C8: 419A0114  beq cr6, 0x832c25dc
	if ctx.cr[6].eq {
	pc = 0x832C25DC; continue 'dispatch;
	}
	// 832C24CC: 21090020  subfic r8, r9, 0x20
	ctx.xer.ca = ctx.r[9].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[9].s64;
	// 832C24D0: 7F3E4430  srw r30, r25, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[25].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x832C24D4; continue 'dispatch;
            }
            0x832C24D4 => {
    //   block [0x832C24D4..0x832C2508)
	// 832C24D4: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 832C24D8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C24DC: 4098002C  bge cr6, 0x832c2508
	if !ctx.cr[6].lt {
	pc = 0x832C2508; continue 'dispatch;
	}
	// 832C24E0: 80C70000  lwz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C24E4: 7FEB4850  subf r31, r11, r9
	ctx.r[31].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 832C24E8: 7D095850  subf r8, r9, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 832C24EC: 7CCB5830  slw r11, r6, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[6].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C24F0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 832C24F4: 7CCAFC30  srw r10, r6, r31
	if (ctx.r[31].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[6].u32) >> ((ctx.r[31].u8 & 0x1F) as u32)) as u64;
	}
	// 832C24F8: 7D66F038  and r6, r11, r30
	ctx.r[6].u64 = ctx.r[11].u64 & ctx.r[30].u64;
	// 832C24FC: 39680020  addi r11, r8, 0x20
	ctx.r[11].s64 = ctx.r[8].s64 + 32;
	// 832C2500: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 832C2504: 48000010  b 0x832c2514
	pc = 0x832C2514; continue 'dispatch;
            }
            0x832C2508 => {
    //   block [0x832C2508..0x832C2514)
	// 832C2508: 7FC65038  and r6, r30, r10
	ctx.r[6].u64 = ctx.r[30].u64 & ctx.r[10].u64;
	// 832C250C: 7D4A4C30  srw r10, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2510: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	pc = 0x832C2514; continue 'dispatch;
            }
            0x832C2514 => {
    //   block [0x832C2514..0x832C2538)
	// 832C2514: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 832C2518: 419A003C  beq cr6, 0x832c2554
	if ctx.cr[6].eq {
	pc = 0x832C2554; continue 'dispatch;
	}
	// 832C251C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C2520: 409A0018  bne cr6, 0x832c2538
	if !ctx.cr[6].eq {
	pc = 0x832C2538; continue 'dispatch;
	}
	// 832C2524: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2528: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 832C252C: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C2530: 550AF87E  srwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C2534: 48000010  b 0x832c2544
	pc = 0x832C2544; continue 'dispatch;
            }
            0x832C2538 => {
    //   block [0x832C2538..0x832C2544)
	// 832C2538: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 832C253C: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C2540: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C2544; continue 'dispatch;
            }
            0x832C2544 => {
    //   block [0x832C2544..0x832C2554)
	// 832C2544: 550807FE  clrlwi r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 832C2548: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832C254C: 419A0008  beq cr6, 0x832c2554
	if ctx.cr[6].eq {
	pc = 0x832C2554; continue 'dispatch;
	}
	// 832C2550: 7CC600D0  neg r6, r6
	ctx.r[6].s64 = -ctx.r[6].s64;
	pc = 0x832C2554; continue 'dispatch;
            }
            0x832C2554 => {
    //   block [0x832C2554..0x832C2570)
	// 832C2554: 7F66DA14  add r27, r6, r27
	ctx.r[27].u64 = ctx.r[6].u64 + ctx.r[27].u64;
	// 832C2558: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832C255C: 577F043E  clrlwi r31, r27, 0x10
	ctx.r[31].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 832C2560: B3FC0000  sth r31, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 832C2564: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 832C2568: 409AFF6C  bne cr6, 0x832c24d4
	if !ctx.cr[6].eq {
	pc = 0x832C24D4; continue 'dispatch;
	}
	// 832C256C: 48000070  b 0x832c25dc
	pc = 0x832C25DC; continue 'dispatch;
            }
            0x832C2570 => {
    //   block [0x832C2570..0x832C259C)
	// 832C2570: 57E9043E  clrlwi r9, r31, 0x10
	ctx.r[9].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 832C2574: 57E8801E  slwi r8, r31, 0x10
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C2578: 578607BC  rlwinm r6, r28, 0, 0x1e, 0x1e
	ctx.r[6].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 832C257C: 7D1D4B78  or r29, r8, r9
	ctx.r[29].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 832C2580: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 832C2584: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 832C2588: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 832C258C: 419A0010  beq cr6, 0x832c259c
	if ctx.cr[6].eq {
	pc = 0x832C259C; continue 'dispatch;
	}
	// 832C2590: B3FC0000  sth r31, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 832C2594: 393C0002  addi r9, r28, 2
	ctx.r[9].s64 = ctx.r[28].s64 + 2;
	// 832C2598: 3BDAFFFF  addi r30, r26, -1
	ctx.r[30].s64 = ctx.r[26].s64 + -1;
	pc = 0x832C259C; continue 'dispatch;
            }
            0x832C259C => {
    //   block [0x832C259C..0x832C25B0)
	// 832C259C: 57C6F87E  srwi r6, r30, 1
	ctx.r[6].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832C25A0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 832C25A4: 419A0020  beq cr6, 0x832c25c4
	if ctx.cr[6].eq {
	pc = 0x832C25C4; continue 'dispatch;
	}
	// 832C25A8: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 832C25AC: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	pc = 0x832C25B0; continue 'dispatch;
            }
            0x832C25B0 => {
    //   block [0x832C25B0..0x832C25C4)
	// 832C25B0: 93A80000  stw r29, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832C25B4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 832C25B8: 4200FFF8  bdnz 0x832c25b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x832C25B0; continue 'dispatch;
	}
	// 832C25BC: 54C8103A  slwi r8, r6, 2
	ctx.r[8].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C25C0: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	pc = 0x832C25C4; continue 'dispatch;
            }
            0x832C25C4 => {
    //   block [0x832C25C4..0x832C25D4)
	// 832C25C4: 57C807FE  clrlwi r8, r30, 0x1f
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 832C25C8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832C25CC: 419A0008  beq cr6, 0x832c25d4
	if ctx.cr[6].eq {
	pc = 0x832C25D4; continue 'dispatch;
	}
	// 832C25D0: B3A90000  sth r29, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	pc = 0x832C25D4; continue 'dispatch;
            }
            0x832C25D4 => {
    //   block [0x832C25D4..0x832C25DC)
	// 832C25D4: 5749083C  slwi r9, r26, 1
	ctx.r[9].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C25D8: 7F89E214  add r28, r9, r28
	ctx.r[28].u64 = ctx.r[9].u64 + ctx.r[28].u64;
	pc = 0x832C25DC; continue 'dispatch;
            }
            0x832C25DC => {
    //   block [0x832C25DC..0x832C25F4)
	// 832C25DC: 7F1AC051  subf. r24, r26, r24
	ctx.r[24].s64 = ctx.r[24].s64 - ctx.r[26].s64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 832C25E0: 4082FE88  bne 0x832c2468
	if !ctx.cr[0].eq {
	pc = 0x832C2468; continue 'dispatch;
	}
	// 832C25E4: 90E40004  stw r7, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 832C25E8: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832C25EC: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832C25F0: 4B9E6E58  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C25F4 => {
    //   block [0x832C25F4..0x832C2604)
	// 832C25F4: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C25F8: 39090004  addi r8, r9, 4
	ctx.r[8].s64 = ctx.r[9].s64 + 4;
	// 832C25FC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 832C2600: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	pc = 0x832C2604; continue 'dispatch;
            }
            0x832C2604 => {
    //   block [0x832C2604..0x832C2610)
	// 832C2604: 90E40004  stw r7, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 832C2608: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832C260C: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x832C2610; continue 'dispatch;
            }
            0x832C2610 => {
    //   block [0x832C2610..0x832C2614)
	// 832C2610: 4B9E6E38  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C2618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C2618 size=9168
    let mut pc: u32 = 0x832C2618;
    'dispatch: loop {
        match pc {
            0x832C2618 => {
    //   block [0x832C2618..0x832C26C4)
	// 832C2618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C261C: 4B9E6DB5  bl 0x82ca93d0
	ctx.lr = 0x832C2620;
	sub_82CA93D0(ctx, base);
	// 832C2620: 9421FA20  stwu r1, -0x5e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1504 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C2624: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 832C2628: 81410654  lwz r10, 0x654(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1620 as u32) ) } as u64;
	// 832C262C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 832C2630: 82610634  lwz r19, 0x634(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1588 as u32) ) } as u64;
	// 832C2634: 3979FFF8  addi r11, r25, -8
	ctx.r[11].s64 = ctx.r[25].s64 + -8;
	// 832C2638: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 832C263C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 832C2640: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 832C2644: 7D6BF9D6  mullw r11, r11, r31
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 832C2648: 908105FC  stw r4, 0x5fc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1532 as u32), ctx.r[4].u32 ) };
	// 832C264C: 93410604  stw r26, 0x604(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1540 as u32), ctx.r[26].u32 ) };
	// 832C2650: 9321060C  stw r25, 0x60c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1548 as u32), ctx.r[25].u32 ) };
	// 832C2654: 92610080  stw r19, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[19].u32 ) };
	// 832C2658: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 832C265C: 7D2F4B78  mr r15, r9
	ctx.r[15].u64 = ctx.r[9].u64;
	// 832C2660: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 832C2664: 57E91838  slwi r9, r31, 3
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C2668: 91E10624  stw r15, 0x624(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1572 as u32), ctx.r[15].u32 ) };
	// 832C266C: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 832C2670: 92A10068  stw r21, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[21].u32 ) };
	// 832C2674: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832C2678: 92A10060  stw r21, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[21].u32 ) };
	// 832C267C: 7EB7AB78  mr r23, r21
	ctx.r[23].u64 = ctx.r[21].u64;
	// 832C2680: 55480420  rlwinm r8, r10, 0, 0x10, 0x10
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 832C2684: 93A105F4  stw r29, 0x5f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1524 as u32), ctx.r[29].u32 ) };
	// 832C2688: 7CFA4850  subf r7, r26, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[26].s64;
	// 832C268C: 92E10084  stw r23, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[23].u32 ) };
	// 832C2690: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 832C2694: 93A100A8  stw r29, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 832C2698: 38CBFFF8  addi r6, r11, -8
	ctx.r[6].s64 = ctx.r[11].s64 + -8;
	// 832C269C: 90E10134  stw r7, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[7].u32 ) };
	// 832C26A0: 7EB6AB78  mr r22, r21
	ctx.r[22].u64 = ctx.r[21].u64;
	// 832C26A4: 93010078  stw r24, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[24].u32 ) };
	// 832C26A8: 90C100E8  stw r6, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[6].u32 ) };
	// 832C26AC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832C26B0: 92C100E4  stw r22, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[22].u32 ) };
	// 832C26B4: 419A0010  beq cr6, 0x832c26c4
	if ctx.cr[6].eq {
	pc = 0x832C26C4; continue 'dispatch;
	}
	// 832C26B8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832C26BC: 396B16B0  addi r11, r11, 0x16b0
	ctx.r[11].s64 = ctx.r[11].s64 + 5808;
	// 832C26C0: 4800000C  b 0x832c26cc
	pc = 0x832C26CC; continue 'dispatch;
            }
            0x832C26C4 => {
    //   block [0x832C26C4..0x832C26CC)
	// 832C26C4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832C26C8: 396B1968  addi r11, r11, 0x1968
	ctx.r[11].s64 = ctx.r[11].s64 + 6504;
	pc = 0x832C26CC; continue 'dispatch;
            }
            0x832C26CC => {
    //   block [0x832C26CC..0x832C2914)
	// 832C26CC: 916100EC  stw r11, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 832C26D0: 5748E13E  srwi r8, r26, 4
	ctx.r[8].u32 = ctx.r[26].u32.wrapping_shr(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C26D4: 8161064C  lwz r11, 0x64c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1612 as u32) ) } as u64;
	// 832C26D8: 5749E8FE  srwi r9, r26, 3
	ctx.r[9].u32 = ctx.r[26].u32.wrapping_shr(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C26DC: 38A801FF  addi r5, r8, 0x1ff
	ctx.r[5].s64 = ctx.r[8].s64 + 511;
	// 832C26E0: 92A10240  stw r21, 0x240(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(576 as u32), ctx.r[21].u32 ) };
	// 832C26E4: 38C901FF  addi r6, r9, 0x1ff
	ctx.r[6].s64 = ctx.r[9].s64 + 511;
	// 832C26E8: 92A10244  stw r21, 0x244(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(580 as u32), ctx.r[21].u32 ) };
	// 832C26EC: 3BC8020F  addi r30, r8, 0x20f
	ctx.r[30].s64 = ctx.r[8].s64 + 527;
	// 832C26F0: 92A1024C  stw r21, 0x24c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(588 as u32), ctx.r[21].u32 ) };
	// 832C26F4: 54AE043E  clrlwi r14, r5, 0x10
	ctx.r[14].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 832C26F8: 92A10280  stw r21, 0x280(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(640 as u32), ctx.r[21].u32 ) };
	// 832C26FC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C2700: 574A1832  rlwinm r10, r26, 3, 0, 0x19
	ctx.r[10].u64 = ctx.r[26].u32 as u64 & 0x1FFFFFFFu64;
	// 832C2704: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2708: 57470038  rlwinm r7, r26, 0, 0, 0x1c
	ctx.r[7].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 832C270C: 574800F8  rlwinm r8, r26, 0, 3, 0x1c
	ctx.r[8].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 832C2710: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C2714: 80AB001C  lwz r5, 0x1c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 832C2718: 54D4043E  clrlwi r20, r6, 0x10
	ctx.r[20].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	// 832C271C: 80CB0018  lwz r6, 0x18(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 832C2720: 3908020F  addi r8, r8, 0x20f
	ctx.r[8].s64 = ctx.r[8].s64 + 527;
	// 832C2724: 394A01FF  addi r10, r10, 0x1ff
	ctx.r[10].s64 = ctx.r[10].s64 + 511;
	// 832C2728: 906102B0  stw r3, 0x2b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(688 as u32), ctx.r[3].u32 ) };
	// 832C272C: 3A2701FF  addi r17, r7, 0x1ff
	ctx.r[17].s64 = ctx.r[7].s64 + 511;
	// 832C2730: 910100A4  stw r8, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[8].u32 ) };
	// 832C2734: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 832C2738: 90810270  stw r4, 0x270(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(624 as u32), ctx.r[4].u32 ) };
	// 832C273C: 5623043E  clrlwi r3, r17, 0x10
	ctx.r[3].u64 = ctx.r[17].u32 as u64 & 0x0000FFFFu64;
	// 832C2740: 90A10090  stw r5, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[5].u32 ) };
	// 832C2744: 7E840034  cntlzw r4, r20
	ctx.r[4].u64 = if ctx.r[20].u32 == 0 { 32 } else { ctx.r[20].u32.leading_zeros() as u64 };
	// 832C2748: 938100E0  stw r28, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[28].u32 ) };
	// 832C274C: 5748F0BC  rlwinm r8, r26, 0x1e, 2, 0x1e
	ctx.r[8].u64 = ctx.r[26].u32 as u64 & 0x00000003u64;
	// 832C2750: 90C10088  stw r6, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[6].u32 ) };
	// 832C2754: 7DDC0034  cntlzw r28, r14
	ctx.r[28].u64 = if ctx.r[14].u32 == 0 { 32 } else { ctx.r[14].u32.leading_zeros() as u64 };
	// 832C2758: 836B000C  lwz r27, 0xc(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832C275C: 574518F2  rlwinm r5, r26, 3, 3, 0x19
	ctx.r[5].u64 = ctx.r[26].u32 as u64 & 0x1FFFFFFFu64;
	// 832C2760: 824B0010  lwz r18, 0x10(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C2764: 7C710034  cntlzw r17, r3
	ctx.r[17].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 832C2768: 820B0014  lwz r16, 0x14(r11)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 832C276C: 7D540034  cntlzw r20, r10
	ctx.r[20].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 832C2770: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C2774: 5746E8FE  srwi r6, r26, 3
	ctx.r[6].u32 = ctx.r[26].u32.wrapping_shr(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832C2778: 92A10284  stw r21, 0x284(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(644 as u32), ctx.r[21].u32 ) };
	// 832C277C: 21440020  subfic r10, r4, 0x20
	ctx.xer.ca = ctx.r[4].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[4].s64;
	// 832C2780: 92A1028C  stw r21, 0x28c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(652 as u32), ctx.r[21].u32 ) };
	// 832C2784: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 832C2788: 92A100B0  stw r21, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[21].u32 ) };
	// 832C278C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 832C2790: 92A100B4  stw r21, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[21].u32 ) };
	// 832C2794: 211C0020  subfic r8, r28, 0x20
	ctx.xer.ca = ctx.r[28].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[28].s64;
	// 832C2798: 92A100BC  stw r21, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[21].u32 ) };
	// 832C279C: 38A5020F  addi r5, r5, 0x20f
	ctx.r[5].s64 = ctx.r[5].s64 + 527;
	// 832C27A0: 906100B8  stw r3, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[3].u32 ) };
	// 832C27A4: 38C6020F  addi r6, r6, 0x20f
	ctx.r[6].s64 = ctx.r[6].s64 + 527;
	// 832C27A8: 90610208  stw r3, 0x208(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(520 as u32), ctx.r[3].u32 ) };
	// 832C27AC: 23940020  subfic r28, r20, 0x20
	ctx.xer.ca = ctx.r[20].u32 <= 32 as u32;
	ctx.r[28].s64 = (32 as i64) - ctx.r[20].s64;
	// 832C27B0: 910102A8  stw r8, 0x2a8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(680 as u32), ctx.r[8].u32 ) };
	// 832C27B4: 57C30036  rlwinm r3, r30, 0, 0, 0x1b
	ctx.r[3].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 832C27B8: 92A10200  stw r21, 0x200(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(512 as u32), ctx.r[21].u32 ) };
	// 832C27BC: 54A50036  rlwinm r5, r5, 0, 0, 0x1b
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 832C27C0: 938100D8  stw r28, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[28].u32 ) };
	// 832C27C4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 832C27C8: 906102AC  stw r3, 0x2ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(684 as u32), ctx.r[3].u32 ) };
	// 832C27CC: 54C80036  rlwinm r8, r6, 0, 0, 0x1b
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 832C27D0: 90A100DC  stw r5, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[5].u32 ) };
	// 832C27D4: 23D10020  subfic r30, r17, 0x20
	ctx.xer.ca = ctx.r[17].u32 <= 32 as u32;
	ctx.r[30].s64 = (32 as i64) - ctx.r[17].s64;
	// 832C27D8: 91410268  stw r10, 0x268(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(616 as u32), ctx.r[10].u32 ) };
	// 832C27DC: 92A10204  stw r21, 0x204(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(516 as u32), ctx.r[21].u32 ) };
	// 832C27E0: 57460836  rlwinm r6, r26, 1, 0, 0x1b
	ctx.r[6].u64 = ctx.r[26].u32 as u64 & 0x7FFFFFFFu64;
	// 832C27E4: 92A1020C  stw r21, 0x20c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(524 as u32), ctx.r[21].u32 ) };
	// 832C27E8: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C27EC: 93610230  stw r27, 0x230(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(560 as u32), ctx.r[27].u32 ) };
	// 832C27F0: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 832C27F4: 91410168  stw r10, 0x168(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(360 as u32), ctx.r[10].u32 ) };
	// 832C27F8: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 832C27FC: 9101026C  stw r8, 0x26c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(620 as u32), ctx.r[8].u32 ) };
	// 832C2800: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 832C2804: 90810248  stw r4, 0x248(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(584 as u32), ctx.r[4].u32 ) };
	// 832C2808: 90810288  stw r4, 0x288(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(648 as u32), ctx.r[4].u32 ) };
	// 832C280C: 93C10228  stw r30, 0x228(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(552 as u32), ctx.r[30].u32 ) };
	// 832C2810: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 832C2814: 574BF0FC  rlwinm r11, r26, 0x1e, 3, 0x1e
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x00000003u64;
	// 832C2818: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 832C281C: 83C100A4  lwz r30, 0xa4(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 832C2820: 38C901FF  addi r6, r9, 0x1ff
	ctx.r[6].s64 = ctx.r[9].s64 + 511;
	// 832C2824: 90A101C8  stw r5, 0x1c8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(456 as u32), ctx.r[5].u32 ) };
	// 832C2828: 392B040F  addi r9, r11, 0x40f
	ctx.r[9].s64 = ctx.r[11].s64 + 1039;
	// 832C282C: 90A10308  stw r5, 0x308(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(776 as u32), ctx.r[5].u32 ) };
	// 832C2830: 54EB08FC  rlwinm r11, r7, 1, 3, 0x1e
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x7FFFFFFFu64;
	// 832C2834: 82810088  lwz r20, 0x88(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 832C2838: 54C7043E  clrlwi r7, r6, 0x10
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	// 832C283C: 90610148  stw r3, 0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[3].u32 ) };
	// 832C2840: 38CB020F  addi r6, r11, 0x20f
	ctx.r[6].s64 = ctx.r[11].s64 + 527;
	// 832C2844: 90610188  stw r3, 0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), ctx.r[3].u32 ) };
	// 832C2848: 7CE50034  cntlzw r5, r7
	ctx.r[5].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 832C284C: 90810108  stw r4, 0x108(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[4].u32 ) };
	// 832C2850: 57C70036  rlwinm r7, r30, 0, 0, 0x1b
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 832C2854: 9101016C  stw r8, 0x16c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), ctx.r[8].u32 ) };
	// 832C2858: 3BC00400  li r30, 0x400
	ctx.r[30].s64 = 1024;
	// 832C285C: 9381014C  stw r28, 0x14c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), ctx.r[28].u32 ) };
	// 832C2860: 552B0036  rlwinm r11, r9, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 832C2864: 9381018C  stw r28, 0x18c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), ctx.r[28].u32 ) };
	// 832C2868: 93C1030C  stw r30, 0x30c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(780 as u32), ctx.r[30].u32 ) };
	// 832C286C: 21250020  subfic r9, r5, 0x20
	ctx.xer.ca = ctx.r[5].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[5].s64;
	// 832C2870: 83C10090  lwz r30, 0x90(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 832C2874: 54DB0036  rlwinm r27, r6, 0, 0, 0x1b
	ctx.r[27].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 832C2878: 38C10250  addi r6, r1, 0x250
	ctx.r[6].s64 = ctx.r[1].s64 + 592;
	// 832C287C: 92410170  stw r18, 0x170(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[18].u32 ) };
	// 832C2880: 38A10260  addi r5, r1, 0x260
	ctx.r[5].s64 = ctx.r[1].s64 + 608;
	// 832C2884: 914101A8  stw r10, 0x1a8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(424 as u32), ctx.r[10].u32 ) };
	// 832C2888: 38810264  addi r4, r1, 0x264
	ctx.r[4].s64 = ctx.r[1].s64 + 612;
	// 832C288C: 910101AC  stw r8, 0x1ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), ctx.r[8].u32 ) };
	// 832C2890: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832C2894: 920101B0  stw r16, 0x1b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), ctx.r[16].u32 ) };
	// 832C2898: 93C10330  stw r30, 0x330(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(816 as u32), ctx.r[30].u32 ) };
	// 832C289C: 914101E8  stw r10, 0x1e8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 832C28A0: 928101F0  stw r20, 0x1f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(496 as u32), ctx.r[20].u32 ) };
	// 832C28A4: 83C10098  lwz r30, 0x98(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 832C28A8: 91410328  stw r10, 0x328(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(808 as u32), ctx.r[10].u32 ) };
	// 832C28AC: 916101EC  stw r11, 0x1ec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(492 as u32), ctx.r[11].u32 ) };
	// 832C28B0: 90E1022C  stw r7, 0x22c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(556 as u32), ctx.r[7].u32 ) };
	// 832C28B4: 92A10140  stw r21, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[21].u32 ) };
	// 832C28B8: 92A10144  stw r21, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[21].u32 ) };
	// 832C28BC: 92A10180  stw r21, 0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[21].u32 ) };
	// 832C28C0: 92A10184  stw r21, 0x184(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), ctx.r[21].u32 ) };
	// 832C28C4: 92A101C0  stw r21, 0x1c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), ctx.r[21].u32 ) };
	// 832C28C8: 92A101C4  stw r21, 0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), ctx.r[21].u32 ) };
	// 832C28CC: 92A101CC  stw r21, 0x1cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(460 as u32), ctx.r[21].u32 ) };
	// 832C28D0: 9161032C  stw r11, 0x32c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(812 as u32), ctx.r[11].u32 ) };
	// 832C28D4: 92A10300  stw r21, 0x300(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(768 as u32), ctx.r[21].u32 ) };
	// 832C28D8: 92A10304  stw r21, 0x304(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(772 as u32), ctx.r[21].u32 ) };
	// 832C28DC: 91210128  stw r9, 0x128(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(296 as u32), ctx.r[9].u32 ) };
	// 832C28E0: 92A10100  stw r21, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[21].u32 ) };
	// 832C28E4: 92A10104  stw r21, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[21].u32 ) };
	// 832C28E8: 92A1010C  stw r21, 0x10c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), ctx.r[21].u32 ) };
	// 832C28EC: 9361012C  stw r27, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[27].u32 ) };
	// 832C28F0: 93C10130  stw r30, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[30].u32 ) };
	// 832C28F4: 4BFFE2BD  bl 0x832c0bb0
	ctx.lr = 0x832C28F8;
	sub_832C0BB0(ctx, base);
	// 832C28F8: 38C10290  addi r6, r1, 0x290
	ctx.r[6].s64 = ctx.r[1].s64 + 656;
	// 832C28FC: 38A102A0  addi r5, r1, 0x2a0
	ctx.r[5].s64 = ctx.r[1].s64 + 672;
	// 832C2900: 388102A4  addi r4, r1, 0x2a4
	ctx.r[4].s64 = ctx.r[1].s64 + 676;
	// 832C2904: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832C2908: 4BFFE2A9  bl 0x832c0bb0
	ctx.lr = 0x832C290C;
	sub_832C0BB0(ctx, base);
	// 832C290C: 3BC10500  addi r30, r1, 0x500
	ctx.r[30].s64 = ctx.r[1].s64 + 1280;
	// 832C2910: 3B6103C0  addi r27, r1, 0x3c0
	ctx.r[27].s64 = ctx.r[1].s64 + 960;
	pc = 0x832C2914; continue 'dispatch;
            }
            0x832C2914 => {
    //   block [0x832C2914..0x832C29DC)
	// 832C2914: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832C2918: 38BEFFC0  addi r5, r30, -0x40
	ctx.r[5].s64 = ctx.r[30].s64 + -64;
	// 832C291C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832C2920: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832C2924: 4BFFE28D  bl 0x832c0bb0
	ctx.lr = 0x832C2928;
	sub_832C0BB0(ctx, base);
	// 832C2928: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 832C292C: 3B7B0010  addi r27, r27, 0x10
	ctx.r[27].s64 = ctx.r[27].s64 + 16;
	// 832C2930: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 832C2934: 4082FFE0  bne 0x832c2914
	if !ctx.cr[0].eq {
	pc = 0x832C2914; continue 'dispatch;
	}
	// 832C2938: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 832C293C: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 832C2940: 388100D4  addi r4, r1, 0xd4
	ctx.r[4].s64 = ctx.r[1].s64 + 212;
	// 832C2944: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832C2948: 4BFFE269  bl 0x832c0bb0
	ctx.lr = 0x832C294C;
	sub_832C0BB0(ctx, base);
	// 832C294C: 92A10540  stw r21, 0x540(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1344 as u32), ctx.r[21].u32 ) };
	// 832C2950: 38C10210  addi r6, r1, 0x210
	ctx.r[6].s64 = ctx.r[1].s64 + 528;
	// 832C2954: 38A10220  addi r5, r1, 0x220
	ctx.r[5].s64 = ctx.r[1].s64 + 544;
	// 832C2958: 38810224  addi r4, r1, 0x224
	ctx.r[4].s64 = ctx.r[1].s64 + 548;
	// 832C295C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832C2960: 4BFFE251  bl 0x832c0bb0
	ctx.lr = 0x832C2964;
	sub_832C0BB0(ctx, base);
	// 832C2964: 38C10150  addi r6, r1, 0x150
	ctx.r[6].s64 = ctx.r[1].s64 + 336;
	// 832C2968: 38A10160  addi r5, r1, 0x160
	ctx.r[5].s64 = ctx.r[1].s64 + 352;
	// 832C296C: 38810164  addi r4, r1, 0x164
	ctx.r[4].s64 = ctx.r[1].s64 + 356;
	// 832C2970: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832C2974: 4BFFE23D  bl 0x832c0bb0
	ctx.lr = 0x832C2978;
	sub_832C0BB0(ctx, base);
	// 832C2978: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 832C297C: 38A101A0  addi r5, r1, 0x1a0
	ctx.r[5].s64 = ctx.r[1].s64 + 416;
	// 832C2980: 388101A4  addi r4, r1, 0x1a4
	ctx.r[4].s64 = ctx.r[1].s64 + 420;
	// 832C2984: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832C2988: 4BFFE229  bl 0x832c0bb0
	ctx.lr = 0x832C298C;
	sub_832C0BB0(ctx, base);
	// 832C298C: 38C10110  addi r6, r1, 0x110
	ctx.r[6].s64 = ctx.r[1].s64 + 272;
	// 832C2990: 38A10120  addi r5, r1, 0x120
	ctx.r[5].s64 = ctx.r[1].s64 + 288;
	// 832C2994: 38810124  addi r4, r1, 0x124
	ctx.r[4].s64 = ctx.r[1].s64 + 292;
	// 832C2998: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 832C299C: 4BFFE215  bl 0x832c0bb0
	ctx.lr = 0x832C29A0;
	sub_832C0BB0(ctx, base);
	// 832C29A0: 7EB1AB78  mr r17, r21
	ctx.r[17].u64 = ctx.r[21].u64;
	// 832C29A4: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 832C29A8: 9221007C  stw r17, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[17].u32 ) };
	// 832C29AC: 419A1FFC  beq cr6, 0x832c49a8
	if ctx.cr[6].eq {
	pc = 0x832C49A8; continue 'dispatch;
	}
	// 832C29B0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 832C29B4: 8381063C  lwz r28, 0x63c(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1596 as u32) ) } as u64;
	// 832C29B8: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 832C29BC: 3BCB6040  addi r30, r11, 0x6040
	ctx.r[30].s64 = ctx.r[11].s64 + 24640;
	// 832C29C0: 396A5C40  addi r11, r10, 0x5c40
	ctx.r[11].s64 = ctx.r[10].s64 + 23616;
	// 832C29C4: 93C10138  stw r30, 0x138(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[30].u32 ) };
	// 832C29C8: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 832C29CC: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 832C29D0: 4800000C  b 0x832c29dc
	pc = 0x832C29DC; continue 'dispatch;
	// 832C29D4: 83410604  lwz r26, 0x604(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1540 as u32) ) } as u64;
	// 832C29D8: 81E10624  lwz r15, 0x624(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1572 as u32) ) } as u64;
            }
            0x832C29DC => {
    //   block [0x832C29DC..0x832C2AB4)
	// 832C29DC: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 832C29E0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C29E4: 38610240  addi r3, r1, 0x240
	ctx.r[3].s64 = ctx.r[1].s64 + 576;
	// 832C29E8: 4BFFE9D1  bl 0x832c13b8
	ctx.lr = 0x832C29EC;
	sub_832C13B8(ctx, base);
	// 832C29EC: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 832C29F0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C29F4: 38610280  addi r3, r1, 0x280
	ctx.r[3].s64 = ctx.r[1].s64 + 640;
	// 832C29F8: 4BFFE9C1  bl 0x832c13b8
	ctx.lr = 0x832C29FC;
	sub_832C13B8(ctx, base);
	// 832C29FC: 816100EC  lwz r11, 0xec(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(236 as u32) ) } as u64;
	// 832C2A00: 7DE67B78  mr r6, r15
	ctx.r[6].u64 = ctx.r[15].u64;
	// 832C2A04: 38A103C0  addi r5, r1, 0x3c0
	ctx.r[5].s64 = ctx.r[1].s64 + 960;
	// 832C2A08: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C2A0C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 832C2A10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832C2A14: 4E800421  bctrl
	ctx.lr = 0x832C2A18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832C2A18: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 832C2A1C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C2A20: 38610200  addi r3, r1, 0x200
	ctx.r[3].s64 = ctx.r[1].s64 + 512;
	// 832C2A24: 4BFFF415  bl 0x832c1e38
	ctx.lr = 0x832C2A28;
	sub_832C1E38(ctx, base);
	// 832C2A28: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C2A2C: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 832C2A30: 4BFFF5F9  bl 0x832c2028
	ctx.lr = 0x832C2A34;
	sub_832C2028(ctx, base);
	// 832C2A34: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 832C2A38: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C2A3C: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 832C2A40: 4BFFF5E9  bl 0x832c2028
	ctx.lr = 0x832C2A44;
	sub_832C2028(ctx, base);
	// 832C2A44: 81410170  lwz r10, 0x170(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 832C2A48: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 832C2A4C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C2A50: 386101C0  addi r3, r1, 0x1c0
	ctx.r[3].s64 = ctx.r[1].s64 + 448;
	// 832C2A54: 81C101B0  lwz r14, 0x1b0(r1)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(432 as u32) ) } as u64;
	// 832C2A58: 83210180  lwz r25, 0x180(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 832C2A5C: 7D6EC850  subf r11, r14, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[14].s64;
	// 832C2A60: 7ECB5214  add r22, r11, r10
	ctx.r[22].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 832C2A64: 92C10140  stw r22, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[22].u32 ) };
	// 832C2A68: 4BFFF851  bl 0x832c22b8
	ctx.lr = 0x832C2A6C;
	sub_832C22B8(ctx, base);
	// 832C2A6C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C2A70: 38610300  addi r3, r1, 0x300
	ctx.r[3].s64 = ctx.r[1].s64 + 768;
	// 832C2A74: 4BFFF845  bl 0x832c22b8
	ctx.lr = 0x832C2A78;
	sub_832C22B8(ctx, base);
	// 832C2A78: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C2A7C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 832C2A80: 4BFFF189  bl 0x832c1c08
	ctx.lr = 0x832C2A84;
	sub_832C1C08(ctx, base);
	// 832C2A84: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 832C2A88: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C2A8C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C2A90: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 832C2A94: 83610200  lwz r27, 0x200(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(512 as u32) ) } as u64;
	// 832C2A98: 824101C0  lwz r18, 0x1c0(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(448 as u32) ) } as u64;
	// 832C2A9C: 90A100A0  stw r5, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[5].u32 ) };
	// 832C2AA0: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C2AA4: 82010100  lwz r16, 0x100(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 832C2AA8: 4800000C  b 0x832c2ab4
	pc = 0x832C2AB4; continue 'dispatch;
	// 832C2AAC: 81C101B0  lwz r14, 0x1b0(r1)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(432 as u32) ) } as u64;
	// 832C2AB0: 81E10624  lwz r15, 0x624(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1572 as u32) ) } as u64;
            }
            0x832C2AB4 => {
    //   block [0x832C2AB4..0x832C2B2C)
	// 832C2AB4: 80C101C4  lwz r6, 0x1c4(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 832C2AB8: 7F037840  cmplw cr6, r3, r15
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[15].u32, &mut ctx.xer);
	// 832C2ABC: 80E10204  lwz r7, 0x204(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(516 as u32) ) } as u64;
	// 832C2AC0: 81210184  lwz r9, 0x184(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(388 as u32) ) } as u64;
	// 832C2AC4: 40981E4C  bge cr6, 0x832c4910
	if !ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C2AC8: 81410244  lwz r10, 0x244(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(580 as u32) ) } as u64;
	// 832C2ACC: 81010240  lwz r8, 0x240(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(576 as u32) ) } as u64;
	// 832C2AD0: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C2AD4: 40981E3C  bge cr6, 0x832c4910
	if !ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C2AD8: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2ADC: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C2AE0: 91010240  stw r8, 0x240(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(576 as u32), ctx.r[8].u32 ) };
	// 832C2AE4: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 832C2AE8: 41991E28  bgt cr6, 0x832c4910
	if ctx.cr[6].gt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C2AEC: 3D80832C  lis r12, -0x7cd4
	ctx.r[12].s64 = -2094268416;
	// 832C2AF0: 398C2B04  addi r12, r12, 0x2b04
	ctx.r[12].s64 = ctx.r[12].s64 + 11012;
	// 832C2AF4: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 832C2AF8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 832C2AFC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 832C2B00: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x832C2B2C; continue 'dispatch;
		},
		1 => {
	pc = 0x832C2BC4; continue 'dispatch;
		},
		2 => {
	pc = 0x832C3818; continue 'dispatch;
		},
		3 => {
	pc = 0x832C3D18; continue 'dispatch;
		},
		4 => {
	pc = 0x832C3EDC; continue 'dispatch;
		},
		5 => {
	pc = 0x832C3FB0; continue 'dispatch;
		},
		6 => {
	pc = 0x832C406C; continue 'dispatch;
		},
		7 => {
	pc = 0x832C40E8; continue 'dispatch;
		},
		8 => {
	pc = 0x832C4204; continue 'dispatch;
		},
		9 => {
	pc = 0x832C4538; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 832C2B04: 832C2B2C  lwz r25, 0x2b2c(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(11052 as u32) ) } as u64;
	// 832C2B08: 832C2BC4  lwz r25, 0x2bc4(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(11204 as u32) ) } as u64;
	// 832C2B0C: 832C3818  lwz r25, 0x3818(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(14360 as u32) ) } as u64;
	// 832C2B10: 832C3D18  lwz r25, 0x3d18(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(15640 as u32) ) } as u64;
	// 832C2B14: 832C3EDC  lwz r25, 0x3edc(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(16092 as u32) ) } as u64;
	// 832C2B18: 832C3FB0  lwz r25, 0x3fb0(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(16304 as u32) ) } as u64;
	// 832C2B1C: 832C406C  lwz r25, 0x406c(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(16492 as u32) ) } as u64;
	// 832C2B20: 832C40E8  lwz r25, 0x40e8(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(16616 as u32) ) } as u64;
	// 832C2B24: 832C4204  lwz r25, 0x4204(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(16900 as u32) ) } as u64;
	// 832C2B28: 832C4538  lwz r25, 0x4538(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(17720 as u32) ) } as u64;
            }
            0x832C2B2C => {
    //   block [0x832C2B2C..0x832C2BB4)
	// 832C2B2C: 7D58FA14  add r10, r24, r31
	ctx.r[10].u64 = ctx.r[24].u64 + ctx.r[31].u64;
	// 832C2B30: 7C18FCAE  lfdx f0, r24, r31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[31].u32)) };
	// 832C2B34: 7D3DFA14  add r9, r29, r31
	ctx.r[9].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C2B38: 7C1DFDAE  stfdx f0, r29, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.f[0].u64) };
	// 832C2B3C: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C2B40: C9B80000  lfd f13, 0(r24)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) };
	// 832C2B44: 7D29FA14  add r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C2B48: D9BD0000  stfd f13, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.f[13].u64 ) };
	// 832C2B4C: 7D0AFA14  add r8, r10, r31
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C2B50: 7CE9FA14  add r7, r9, r31
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C2B54: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 832C2B58: C98A0000  lfd f12, 0(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 832C2B5C: 7D48FA14  add r10, r8, r31
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C2B60: D9890000  stfd f12, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.f[12].u64 ) };
	// 832C2B64: 7D27FA14  add r9, r7, r31
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C2B68: C9680000  lfd f11, 0(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 832C2B6C: 7D0AFA14  add r8, r10, r31
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C2B70: D9670000  stfd f11, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.f[11].u64 ) };
	// 832C2B74: 7CE9FA14  add r7, r9, r31
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C2B78: C94A0000  lfd f10, 0(r10)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 832C2B7C: 7D48FA14  add r10, r8, r31
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C2B80: D9490000  stfd f10, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.f[10].u64 ) };
	// 832C2B84: 7D27FA14  add r9, r7, r31
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C2B88: C9280000  lfd f9, 0(r8)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 832C2B8C: 7D08FCAE  lfdx f8, r8, r31
	ctx.f[8].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) };
	// 832C2B90: D9270000  stfd f9, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.f[9].u64 ) };
	// 832C2B94: 7CEAFCAE  lfdx f7, r10, r31
	ctx.f[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	// 832C2B98: 7D07FDAE  stfdx f8, r7, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32), ctx.f[8].u64) };
	// 832C2B9C: 7CE9FDAE  stfdx f7, r9, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.f[7].u64) };
	// 832C2BA0: 419A0014  beq cr6, 0x832c2bb4
	if ctx.cr[6].eq {
	pc = 0x832C2BB4; continue 'dispatch;
	}
	// 832C2BA4: 56EAF87E  srwi r10, r23, 1
	ctx.r[10].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C2BA8: 7D2A98AE  lbzx r9, r10, r19
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 832C2BAC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C2BB0: 409A1D60  bne cr6, 0x832c4910
	if !ctx.cr[6].eq {
	pc = 0x832C4910; continue 'dispatch;
	}
	pc = 0x832C2BB4; continue 'dispatch;
            }
            0x832C2BB4 => {
    //   block [0x832C2BB4..0x832C2BC4)
	// 832C2BB4: 814100E4  lwz r10, 0xe4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 832C2BB8: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 832C2BBC: 912100E4  stw r9, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 832C2BC0: 48001D50  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
            }
            0x832C2BC4 => {
    //   block [0x832C2BC4..0x832C2C34)
	// 832C2BC4: 2B050010  cmplwi cr6, r5, 0x10
	ctx.cr[6].compare_u32(ctx.r[5].u32, 16 as u32, &mut ctx.xer);
	// 832C2BC8: 41981D48  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C2BCC: 562A0738  rlwinm r10, r17, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[17].u32 as u64 & 0xFFFFFFFFu64;
	// 832C2BD0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C2BD4: 409A0C14  bne cr6, 0x832c37e8
	if !ctx.cr[6].eq {
	pc = 0x832C37E8; continue 'dispatch;
	}
	// 832C2BD8: 81410284  lwz r10, 0x284(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(644 as u32) ) } as u64;
	// 832C2BDC: 81210280  lwz r9, 0x280(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(640 as u32) ) } as u64;
	// 832C2BE0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C2BE4: 40980C04  bge cr6, 0x832c37e8
	if !ctx.cr[6].lt {
	pc = 0x832C37E8; continue 'dispatch;
	}
	// 832C2BE8: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2BEC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832C2BF0: 394AFFFD  addi r10, r10, -3
	ctx.r[10].s64 = ctx.r[10].s64 + -3;
	// 832C2BF4: 91210280  stw r9, 0x280(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(640 as u32), ctx.r[9].u32 ) };
	// 832C2BF8: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 832C2BFC: 41990BEC  bgt cr6, 0x832c37e8
	if ctx.cr[6].gt {
	pc = 0x832C37E8; continue 'dispatch;
	}
	// 832C2C00: 3D80832C  lis r12, -0x7cd4
	ctx.r[12].s64 = -2094268416;
	// 832C2C04: 398C2C18  addi r12, r12, 0x2c18
	ctx.r[12].s64 = ctx.r[12].s64 + 11288;
	// 832C2C08: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 832C2C0C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 832C2C10: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 832C2C14: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x832C2C34; continue 'dispatch;
		},
		1 => {
	pc = 0x832C37E8; continue 'dispatch;
		},
		2 => {
	pc = 0x832C2FB8; continue 'dispatch;
		},
		3 => {
	pc = 0x832C307C; continue 'dispatch;
		},
		4 => {
	pc = 0x832C37E8; continue 'dispatch;
		},
		5 => {
	pc = 0x832C3134; continue 'dispatch;
		},
		6 => {
	pc = 0x832C3498; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 832C2C18: 832C2C34  lwz r25, 0x2c34(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(11316 as u32) ) } as u64;
	// 832C2C1C: 832C37E8  lwz r25, 0x37e8(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(14312 as u32) ) } as u64;
	// 832C2C20: 832C2FB8  lwz r25, 0x2fb8(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(12216 as u32) ) } as u64;
	// 832C2C24: 832C307C  lwz r25, 0x307c(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(12412 as u32) ) } as u64;
	// 832C2C28: 832C37E8  lwz r25, 0x37e8(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(14312 as u32) ) } as u64;
	// 832C2C2C: 832C3134  lwz r25, 0x3134(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(12596 as u32) ) } as u64;
	// 832C2C30: 832C3498  lwz r25, 0x3498(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(13464 as u32) ) } as u64;
            }
            0x832C2C34 => {
    //   block [0x832C2C34..0x832C2C70)
	// 832C2C34: 8141060C  lwz r10, 0x60c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1548 as u32) ) } as u64;
	// 832C2C38: 7D315050  subf r9, r17, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[17].s64;
	// 832C2C3C: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 832C2C40: 41981CD0  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C2C44: 2B1A0004  cmplwi cr6, r26, 4
	ctx.cr[6].compare_u32(ctx.r[26].u32, 4 as u32, &mut ctx.xer);
	// 832C2C48: 40980028  bge cr6, 0x832c2c70
	if !ctx.cr[6].lt {
	pc = 0x832C2C70; continue 'dispatch;
	}
	// 832C2C4C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2C50: 213A0004  subfic r9, r26, 4
	ctx.xer.ca = ctx.r[26].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[26].s64;
	// 832C2C54: 38BA001C  addi r5, r26, 0x1c
	ctx.r[5].s64 = ctx.r[26].s64 + 28;
	// 832C2C58: 7D48D030  slw r8, r10, r26
	if (ctx.r[26].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[10].u32) << ((ctx.r[26].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2C5C: 7D072378  or r7, r8, r4
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[4].u64;
	// 832C2C60: 7D444C30  srw r4, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C2C64: 54EA073E  clrlwi r10, r7, 0x1c
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	// 832C2C68: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 832C2C6C: 48000010  b 0x832c2c7c
	pc = 0x832C2C7C; continue 'dispatch;
            }
            0x832C2C70 => {
    //   block [0x832C2C70..0x832C2C7C)
	// 832C2C70: 548A073E  clrlwi r10, r4, 0x1c
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000000Fu64;
	// 832C2C74: 5484E13E  srwi r4, r4, 4
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shr(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 832C2C78: 38BAFFFC  addi r5, r26, -4
	ctx.r[5].s64 = ctx.r[26].s64 + -4;
	pc = 0x832C2C7C; continue 'dispatch;
            }
            0x832C2C7C => {
    //   block [0x832C2C7C..0x832C2CA0)
	// 832C2C7C: 812100B4  lwz r9, 0xb4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 832C2C80: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C2C84: 810100A4  lwz r8, 0xa4(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 832C2C88: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C2C8C: 7CEA4214  add r7, r10, r8
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 832C2C90: 41980010  blt cr6, 0x832c2ca0
	if ctx.cr[6].lt {
	pc = 0x832C2CA0; continue 'dispatch;
	}
	// 832C2C94: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C2C98: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C2C9C: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	pc = 0x832C2CA0; continue 'dispatch;
            }
            0x832C2CA0 => {
    //   block [0x832C2CA0..0x832C2CB4)
	// 832C2CA0: 81410104  lwz r10, 0x104(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 832C2CA4: 7F105040  cmplw cr6, r16, r10
	ctx.cr[6].compare_u32(ctx.r[16].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C2CA8: 4198000C  blt cr6, 0x832c2cb4
	if ctx.cr[6].lt {
	pc = 0x832C2CB4; continue 'dispatch;
	}
	// 832C2CAC: 81410130  lwz r10, 0x130(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(304 as u32) ) } as u64;
	// 832C2CB0: 3A0A0004  addi r16, r10, 4
	ctx.r[16].s64 = ctx.r[10].s64 + 4;
	pc = 0x832C2CB4; continue 'dispatch;
            }
            0x832C2CB4 => {
    //   block [0x832C2CB4..0x832C2CBC)
	// 832C2CB4: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 832C2CB8: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	pc = 0x832C2CBC; continue 'dispatch;
            }
            0x832C2CBC => {
    //   block [0x832C2CBC..0x832C2CF0)
	// 832C2CBC: 89500000  lbz r10, 0(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2CC0: 3A100001  addi r16, r16, 1
	ctx.r[16].s64 = ctx.r[16].s64 + 1;
	// 832C2CC4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 832C2CC8: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 832C2CCC: 419900A0  bgt cr6, 0x832c2d6c
	if ctx.cr[6].gt {
	pc = 0x832C2D6C; continue 'dispatch;
	}
	// 832C2CD0: 7CCA3050  subf r6, r10, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 832C2CD4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 832C2CD8: 409A0018  bne cr6, 0x832c2cf0
	if !ctx.cr[6].eq {
	pc = 0x832C2CF0; continue 'dispatch;
	}
	// 832C2CDC: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2CE0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 832C2CE4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 832C2CE8: 5504F87E  srwi r4, r8, 1
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 832C2CEC: 48000010  b 0x832c2cfc
	pc = 0x832C2CFC; continue 'dispatch;
            }
            0x832C2CF0 => {
    //   block [0x832C2CF0..0x832C2CFC)
	// 832C2CF0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 832C2CF4: 5484F87E  srwi r4, r4, 1
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shr(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 832C2CF8: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	pc = 0x832C2CFC; continue 'dispatch;
            }
            0x832C2CFC => {
    //   block [0x832C2CFC..0x832C2D1C)
	// 832C2CFC: 550807FE  clrlwi r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 832C2D00: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832C2D04: 419A0034  beq cr6, 0x832c2d38
	if ctx.cr[6].eq {
	pc = 0x832C2D38; continue 'dispatch;
	}
	// 832C2D08: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2D0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C2D10: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C2D14: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C2D18: 419A004C  beq cr6, 0x832c2d64
	if ctx.cr[6].eq {
	pc = 0x832C2D64; continue 'dispatch;
	}
	pc = 0x832C2D1C; continue 'dispatch;
            }
            0x832C2D1C => {
    //   block [0x832C2D1C..0x832C2D38)
	// 832C2D1C: 7F8938AE  lbzx r28, r9, r7
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 832C2D20: 3B4102C0  addi r26, r1, 0x2c0
	ctx.r[26].s64 = ctx.r[1].s64 + 704;
	// 832C2D24: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C2D28: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832C2D2C: 7D1CD1AE  stbx r8, r28, r26
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32), ctx.r[8].u8) };
	// 832C2D30: 4082FFEC  bne 0x832c2d1c
	if !ctx.cr[0].eq {
	pc = 0x832C2D1C; continue 'dispatch;
	}
	// 832C2D34: 48000030  b 0x832c2d64
	pc = 0x832C2D64; continue 'dispatch;
            }
            0x832C2D38 => {
    //   block [0x832C2D38..0x832C2D40)
	// 832C2D38: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C2D3C: 419A0028  beq cr6, 0x832c2d64
	if ctx.cr[6].eq {
	pc = 0x832C2D64; continue 'dispatch;
	}
	pc = 0x832C2D40; continue 'dispatch;
            }
            0x832C2D40 => {
    //   block [0x832C2D40..0x832C2D64)
	// 832C2D40: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2D44: 3B8102C0  addi r28, r1, 0x2c0
	ctx.r[28].s64 = ctx.r[1].s64 + 704;
	// 832C2D48: 7F4938AE  lbzx r26, r9, r7
	ctx.r[26].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 832C2D4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C2D50: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C2D54: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832C2D58: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C2D5C: 7D1AE1AE  stbx r8, r26, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[28].u32), ctx.r[8].u8) };
	// 832C2D60: 4082FFE0  bne 0x832c2d40
	if !ctx.cr[0].eq {
	pc = 0x832C2D40; continue 'dispatch;
	}
	pc = 0x832C2D64; continue 'dispatch;
            }
            0x832C2D64 => {
    //   block [0x832C2D64..0x832C2D6C)
	// 832C2D64: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 832C2D68: 4199FF54  bgt cr6, 0x832c2cbc
	if ctx.cr[6].gt {
	pc = 0x832C2CBC; continue 'dispatch;
	}
	pc = 0x832C2D6C; continue 'dispatch;
            }
            0x832C2D6C => {
    //   block [0x832C2D6C..0x832C2D90)
	// 832C2D6C: 92010100  stw r16, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[16].u32 ) };
	// 832C2D70: 2F09003F  cmpwi cr6, r9, 0x3f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 63, &mut ctx.xer);
	// 832C2D74: 409A001C  bne cr6, 0x832c2d90
	if !ctx.cr[6].eq {
	pc = 0x832C2D90; continue 'dispatch;
	}
	// 832C2D78: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2D7C: 392102C0  addi r9, r1, 0x2c0
	ctx.r[9].s64 = ctx.r[1].s64 + 704;
	// 832C2D80: 8907003F  lbz r8, 0x3f(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(63 as u32) ) } as u64;
	// 832C2D84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C2D88: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C2D8C: 7D4849AE  stbx r10, r8, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	pc = 0x832C2D90; continue 'dispatch;
            }
            0x832C2D90 => {
    //   block [0x832C2D90..0x832C2DA0)
	// 832C2D90: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832C2D94: 7D1DFA14  add r8, r29, r31
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C2D98: 394102C4  addi r10, r1, 0x2c4
	ctx.r[10].s64 = ctx.r[1].s64 + 708;
	// 832C2D9C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	pc = 0x832C2DA0; continue 'dispatch;
            }
            0x832C2DA0 => {
    //   block [0x832C2DA0..0x832C2FB8)
	// 832C2DA0: A0CAFFFC  lhz r6, -4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 832C2DA4: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 832C2DA8: A34A0000  lhz r26, 0(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2DAC: 54D9401E  rlwinm r25, r6, 8, 0, 0xf
	ctx.r[25].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 832C2DB0: A38AFFFE  lhz r28, -2(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-2 as u32) ) } as u64;
	// 832C2DB4: A30A0002  lhz r24, 2(r10)
	ctx.r[24].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C2DB8: 5756401E  rlwinm r22, r26, 8, 0, 0xf
	ctx.r[22].u64 = ctx.r[26].u32 as u64 & 0x00FFFFFFu64;
	// 832C2DBC: 7CD23378  mr r18, r6
	ctx.r[18].u64 = ctx.r[6].u64;
	// 832C2DC0: A2AA0004  lhz r21, 4(r10)
	ctx.r[21].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C2DC4: 7F263378  or r6, r25, r6
	ctx.r[6].u64 = ctx.r[25].u64 | ctx.r[6].u64;
	// 832C2DC8: A32A000A  lhz r25, 0xa(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(10 as u32) ) } as u64;
	// 832C2DCC: 5797401E  rlwinm r23, r28, 8, 0, 0xf
	ctx.r[23].u64 = ctx.r[28].u32 as u64 & 0x00FFFFFFu64;
	// 832C2DD0: A26A0006  lhz r19, 6(r10)
	ctx.r[19].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C2DD4: 7F4ED378  mr r14, r26
	ctx.r[14].u64 = ctx.r[26].u64;
	// 832C2DD8: A22A0008  lhz r17, 8(r10)
	ctx.r[17].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C2DDC: 5714401E  rlwinm r20, r24, 8, 0, 0xf
	ctx.r[20].u64 = ctx.r[24].u32 as u64 & 0x00FFFFFFu64;
	// 832C2DE0: A1EA000C  lhz r15, 0xc(r10)
	ctx.r[15].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 832C2DE4: 7EDAD378  or r26, r22, r26
	ctx.r[26].u64 = ctx.r[22].u64 | ctx.r[26].u64;
	// 832C2DE8: 7F90E378  mr r16, r28
	ctx.r[16].u64 = ctx.r[28].u64;
	// 832C2DEC: 7F16C378  mr r22, r24
	ctx.r[22].u64 = ctx.r[24].u64;
	// 832C2DF0: 50D2402E  rlwimi r18, r6, 8, 0, 0x17
	ctx.r[18].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[18].u64 & 0xFFFFFFFF000000FF);
	// 832C2DF4: 7EFCE378  or r28, r23, r28
	ctx.r[28].u64 = ctx.r[23].u64 | ctx.r[28].u64;
	// 832C2DF8: A2EA000E  lhz r23, 0xe(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(14 as u32) ) } as u64;
	// 832C2DFC: 7E98C378  or r24, r20, r24
	ctx.r[24].u64 = ctx.r[20].u64 | ctx.r[24].u64;
	// 832C2E00: 92490000  stw r18, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 832C2E04: 56A6401E  rlwinm r6, r21, 8, 0, 0xf
	ctx.r[6].u64 = ctx.r[21].u32 as u64 & 0x00FFFFFFu64;
	// 832C2E08: 92480000  stw r18, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 832C2E0C: 5316402E  rlwimi r22, r24, 8, 0, 0x17
	ctx.r[22].u64 = (((ctx.r[24].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[22].u64 & 0xFFFFFFFF000000FF);
	// 832C2E10: 5390402E  rlwimi r16, r28, 8, 0, 0x17
	ctx.r[16].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[16].u64 & 0xFFFFFFFF000000FF);
	// 832C2E14: 534E402E  rlwimi r14, r26, 8, 0, 0x17
	ctx.r[14].u64 = (((ctx.r[26].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[14].u64 & 0xFFFFFFFF000000FF);
	// 832C2E18: 92C9000C  stw r22, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[22].u32 ) };
	// 832C2E1C: 7CC6AB78  or r6, r6, r21
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[21].u64;
	// 832C2E20: 92090004  stw r16, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[16].u32 ) };
	// 832C2E24: 7EB8AB78  mr r24, r21
	ctx.r[24].u64 = ctx.r[21].u64;
	// 832C2E28: 91C90008  stw r14, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[14].u32 ) };
	// 832C2E2C: 5735401E  rlwinm r21, r25, 8, 0, 0xf
	ctx.r[21].u64 = ctx.r[25].u32 as u64 & 0x00FFFFFFu64;
	// 832C2E30: 92C8000C  stw r22, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[22].u32 ) };
	// 832C2E34: 7D28FA14  add r9, r8, r31
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C2E38: 92080004  stw r16, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[16].u32 ) };
	// 832C2E3C: 50D8402E  rlwimi r24, r6, 8, 0, 0x17
	ctx.r[24].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[24].u64 & 0xFFFFFFFF000000FF);
	// 832C2E40: 91C80008  stw r14, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[14].u32 ) };
	// 832C2E44: 567C401E  rlwinm r28, r19, 8, 0, 0xf
	ctx.r[28].u64 = ctx.r[19].u32 as u64 & 0x00FFFFFFu64;
	// 832C2E48: 7EA6CB78  or r6, r21, r25
	ctx.r[6].u64 = ctx.r[21].u64 | ctx.r[25].u64;
	// 832C2E4C: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 832C2E50: 7F9C9B78  or r28, r28, r19
	ctx.r[28].u64 = ctx.r[28].u64 | ctx.r[19].u64;
	// 832C2E54: 93090000  stw r24, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 832C2E58: 563A401E  rlwinm r26, r17, 8, 0, 0xf
	ctx.r[26].u64 = ctx.r[17].u32 as u64 & 0x00FFFFFFu64;
	// 832C2E5C: 7F09F92E  stwx r24, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[24].u32) };
	// 832C2E60: 7E749B78  mr r20, r19
	ctx.r[20].u64 = ctx.r[19].u64;
	// 832C2E64: A30A0014  lhz r24, 0x14(r10)
	ctx.r[24].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 832C2E68: 50C8402E  rlwimi r8, r6, 8, 0, 0x17
	ctx.r[8].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[8].u64 & 0xFFFFFFFF000000FF);
	// 832C2E6C: 5394402E  rlwimi r20, r28, 8, 0, 0x17
	ctx.r[20].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[20].u64 & 0xFFFFFFFF000000FF);
	// 832C2E70: 7F5A8B78  or r26, r26, r17
	ctx.r[26].u64 = ctx.r[26].u64 | ctx.r[17].u64;
	// 832C2E74: 7E368B78  mr r22, r17
	ctx.r[22].u64 = ctx.r[17].u64;
	// 832C2E78: 92890004  stw r20, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[20].u32 ) };
	// 832C2E7C: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 832C2E80: 7D09FA14  add r8, r9, r31
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C2E84: 5356402E  rlwimi r22, r26, 8, 0, 0x17
	ctx.r[22].u64 = (((ctx.r[26].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[22].u64 & 0xFFFFFFFF000000FF);
	// 832C2E88: 55E6401E  rlwinm r6, r15, 8, 0, 0xf
	ctx.r[6].u64 = ctx.r[15].u32 as u64 & 0x00FFFFFFu64;
	// 832C2E8C: 92C90008  stw r22, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 832C2E90: 7DFA7B78  mr r26, r15
	ctx.r[26].u64 = ctx.r[15].u64;
	// 832C2E94: 9389000C  stw r28, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 832C2E98: 7CC67B78  or r6, r6, r15
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[15].u64;
	// 832C2E9C: 92880004  stw r20, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[20].u32 ) };
	// 832C2EA0: 7D28FA14  add r9, r8, r31
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C2EA4: 56F9401E  rlwinm r25, r23, 8, 0, 0xf
	ctx.r[25].u64 = ctx.r[23].u32 as u64 & 0x00FFFFFFu64;
	// 832C2EA8: 92C80008  stw r22, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 832C2EAC: 50DA402E  rlwimi r26, r6, 8, 0, 0x17
	ctx.r[26].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[26].u64 & 0xFFFFFFFF000000FF);
	// 832C2EB0: 9388000C  stw r28, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 832C2EB4: 7F3CBB78  or r28, r25, r23
	ctx.r[28].u64 = ctx.r[25].u64 | ctx.r[23].u64;
	// 832C2EB8: A2CA0016  lhz r22, 0x16(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(22 as u32) ) } as u64;
	// 832C2EBC: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 832C2EC0: 93490000  stw r26, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 832C2EC4: 7D09FA14  add r8, r9, r31
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C2EC8: 7F49F92E  stwx r26, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[26].u32) };
	// 832C2ECC: 5386402E  rlwimi r6, r28, 8, 0, 0x17
	ctx.r[6].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[6].u64 & 0xFFFFFFFF000000FF);
	// 832C2ED0: A38A0010  lhz r28, 0x10(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C2ED4: A34A0012  lhz r26, 0x12(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(18 as u32) ) } as u64;
	// 832C2ED8: 5799401E  rlwinm r25, r28, 8, 0, 0xf
	ctx.r[25].u64 = ctx.r[28].u32 as u64 & 0x00FFFFFFu64;
	// 832C2EDC: 5757401E  rlwinm r23, r26, 8, 0, 0xf
	ctx.r[23].u64 = ctx.r[26].u32 as u64 & 0x00FFFFFFu64;
	// 832C2EE0: 7F95E378  mr r21, r28
	ctx.r[21].u64 = ctx.r[28].u64;
	// 832C2EE4: 7F3CE378  or r28, r25, r28
	ctx.r[28].u64 = ctx.r[25].u64 | ctx.r[28].u64;
	// 832C2EE8: 90C80004  stw r6, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C2EEC: 7F53D378  mr r19, r26
	ctx.r[19].u64 = ctx.r[26].u64;
	// 832C2EF0: 90C90004  stw r6, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C2EF4: 7EFAD378  or r26, r23, r26
	ctx.r[26].u64 = ctx.r[23].u64 | ctx.r[26].u64;
	// 832C2EF8: A32A001A  lhz r25, 0x1a(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(26 as u32) ) } as u64;
	// 832C2EFC: 5706401E  rlwinm r6, r24, 8, 0, 0xf
	ctx.r[6].u64 = ctx.r[24].u32 as u64 & 0x00FFFFFFu64;
	// 832C2F00: A28A0018  lhz r20, 0x18(r10)
	ctx.r[20].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 832C2F04: 5395402E  rlwimi r21, r28, 8, 0, 0x17
	ctx.r[21].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[21].u64 & 0xFFFFFFFF000000FF);
	// 832C2F08: 5353402E  rlwimi r19, r26, 8, 0, 0x17
	ctx.r[19].u64 = (((ctx.r[26].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[19].u64 & 0xFFFFFFFF000000FF);
	// 832C2F0C: 7CC6C378  or r6, r6, r24
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[24].u64;
	// 832C2F10: 92A90008  stw r21, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[21].u32 ) };
	// 832C2F14: 7F17C378  mr r23, r24
	ctx.r[23].u64 = ctx.r[24].u64;
	// 832C2F18: 92A80008  stw r21, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[21].u32 ) };
	// 832C2F1C: 5738401E  rlwinm r24, r25, 8, 0, 0xf
	ctx.r[24].u64 = ctx.r[25].u32 as u64 & 0x00FFFFFFu64;
	// 832C2F20: 9269000C  stw r19, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[19].u32 ) };
	// 832C2F24: 56DC401E  rlwinm r28, r22, 8, 0, 0xf
	ctx.r[28].u64 = ctx.r[22].u32 as u64 & 0x00FFFFFFu64;
	// 832C2F28: 9268000C  stw r19, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[19].u32 ) };
	// 832C2F2C: 7D28FA14  add r9, r8, r31
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C2F30: 50D7402E  rlwimi r23, r6, 8, 0, 0x17
	ctx.r[23].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[23].u64 & 0xFFFFFFFF000000FF);
	// 832C2F34: 569A401E  rlwinm r26, r20, 8, 0, 0xf
	ctx.r[26].u64 = ctx.r[20].u32 as u64 & 0x00FFFFFFu64;
	// 832C2F38: 7F08CB78  or r8, r24, r25
	ctx.r[8].u64 = ctx.r[24].u64 | ctx.r[25].u64;
	// 832C2F3C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 832C2F40: 7ED5B378  mr r21, r22
	ctx.r[21].u64 = ctx.r[22].u64;
	// 832C2F44: 92E90000  stw r23, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 832C2F48: 7F9CB378  or r28, r28, r22
	ctx.r[28].u64 = ctx.r[28].u64 | ctx.r[22].u64;
	// 832C2F4C: 7EE9F92E  stwx r23, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[23].u32) };
	// 832C2F50: 7E96A378  mr r22, r20
	ctx.r[22].u64 = ctx.r[20].u64;
	// 832C2F54: 7F5AA378  or r26, r26, r20
	ctx.r[26].u64 = ctx.r[26].u64 | ctx.r[20].u64;
	// 832C2F58: 5106402E  rlwimi r6, r8, 8, 0, 0x17
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[6].u64 & 0xFFFFFFFF000000FF);
	// 832C2F5C: 7D09FA14  add r8, r9, r31
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C2F60: 5395402E  rlwimi r21, r28, 8, 0, 0x17
	ctx.r[21].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[21].u64 & 0xFFFFFFFF000000FF);
	// 832C2F64: 5356402E  rlwimi r22, r26, 8, 0, 0x17
	ctx.r[22].u64 = (((ctx.r[26].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[22].u64 & 0xFFFFFFFF000000FF);
	// 832C2F68: 92A90004  stw r21, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[21].u32 ) };
	// 832C2F6C: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 832C2F70: 92C90008  stw r22, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 832C2F74: 90C9000C  stw r6, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 832C2F78: 7D28FA14  add r9, r8, r31
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C2F7C: 92A80004  stw r21, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[21].u32 ) };
	// 832C2F80: 92C80008  stw r22, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 832C2F84: 90C8000C  stw r6, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 832C2F88: 7D09FA14  add r8, r9, r31
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C2F8C: 4082FE14  bne 0x832c2da0
	if !ctx.cr[0].eq {
	pc = 0x832C2DA0; continue 'dispatch;
	}
	// 832C2F90: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 832C2F94: 83010078  lwz r24, 0x78(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 832C2F98: 82E10084  lwz r23, 0x84(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 832C2F9C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 832C2FA0: 83210180  lwz r25, 0x180(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 832C2FA4: 82C10140  lwz r22, 0x140(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) } as u64;
	// 832C2FA8: 90610064  stw r3, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 832C2FAC: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 832C2FB0: 93410068  stw r26, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[26].u32 ) };
	// 832C2FB4: 48000820  b 0x832c37d4
	pc = 0x832C37D4; continue 'dispatch;
            }
            0x832C2FB8 => {
    //   block [0x832C2FB8..0x832C2FD8)
	// 832C2FB8: 8141060C  lwz r10, 0x60c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1548 as u32) ) } as u64;
	// 832C2FBC: 7D315050  subf r9, r17, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[17].s64;
	// 832C2FC0: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 832C2FC4: 4198194C  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C2FC8: 7F123040  cmplw cr6, r18, r6
	ctx.cr[6].compare_u32(ctx.r[18].u32, ctx.r[6].u32, &mut ctx.xer);
	// 832C2FCC: 4198000C  blt cr6, 0x832c2fd8
	if ctx.cr[6].lt {
	pc = 0x832C2FD8; continue 'dispatch;
	}
	// 832C2FD0: 816101F0  lwz r11, 0x1f0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(496 as u32) ) } as u64;
	// 832C2FD4: 3A4B0004  addi r18, r11, 4
	ctx.r[18].s64 = ctx.r[11].s64 + 4;
	pc = 0x832C2FD8; continue 'dispatch;
            }
            0x832C2FD8 => {
    //   block [0x832C2FD8..0x832C3038)
	// 832C2FD8: A1720000  lhz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C2FDC: 3A520002  addi r18, r18, 2
	ctx.r[18].s64 = ctx.r[18].s64 + 2;
	// 832C2FE0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C2FE4: 38610340  addi r3, r1, 0x340
	ctx.r[3].s64 = ctx.r[1].s64 + 832;
	// 832C2FE8: 924101C0  stw r18, 0x1c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), ctx.r[18].u32 ) };
	// 832C2FEC: B1610340  sth r11, 0x340(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(832 as u32), ctx.r[11].u16 ) };
	// 832C2FF0: 480059B9  bl 0x832c89a8
	ctx.lr = 0x832C2FF4;
	sub_832C89A8(ctx, base);
	// 832C2FF4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C2FF8: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832C2FFC: 4098003C  bge cr6, 0x832c3038
	if !ctx.cr[6].lt {
	pc = 0x832C3038; continue 'dispatch;
	}
	// 832C3000: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C3004: 212B0004  subfic r9, r11, 4
	ctx.xer.ca = ctx.r[11].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[11].s64;
	// 832C3008: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C300C: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 832C3010: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 832C3014: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3018: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 832C301C: 7CA45830  slw r4, r5, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[5].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C3020: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 832C3024: 7CAB4C30  srw r11, r5, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[5].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C3028: 7C834378  or r3, r4, r8
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 832C302C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 832C3030: 546B073E  clrlwi r11, r3, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 832C3034: 4800001C  b 0x832c3050
	pc = 0x832C3050; continue 'dispatch;
            }
            0x832C3038 => {
    //   block [0x832C3038..0x832C3050)
	// 832C3038: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C303C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 832C3040: 5549E13E  srwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C3044: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832C3048: 554B073E  clrlwi r11, r10, 0x1c
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 832C304C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	pc = 0x832C3050; continue 'dispatch;
            }
            0x832C3050 => {
    //   block [0x832C3050..0x832C307C)
	// 832C3050: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C3054: 38A10340  addi r5, r1, 0x340
	ctx.r[5].s64 = ctx.r[1].s64 + 832;
	// 832C3058: 7CCBF214  add r6, r11, r30
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C305C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832C3060: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832C3064: 48005255  bl 0x832c82b8
	ctx.lr = 0x832C3068;
	sub_832C82B8(ctx, base);
	// 832C3068: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C306C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C3070: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 832C3074: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C3078: 48000770  b 0x832c37e8
	pc = 0x832C37E8; continue 'dispatch;
            }
            0x832C307C => {
    //   block [0x832C307C..0x832C30A4)
	// 832C307C: 8141060C  lwz r10, 0x60c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1548 as u32) ) } as u64;
	// 832C3080: 7D315050  subf r9, r17, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[17].s64;
	// 832C3084: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 832C3088: 41981888  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C308C: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 832C3090: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C3094: 41980010  blt cr6, 0x832c30a4
	if ctx.cr[6].lt {
	pc = 0x832C30A4; continue 'dispatch;
	}
	// 832C3098: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C309C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C30A0: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	pc = 0x832C30A4; continue 'dispatch;
            }
            0x832C30A4 => {
    //   block [0x832C30A4..0x832C3134)
	pc = 0x832C3134; continue 'dispatch;
            }
            0x832C3134 => {
    //   block [0x832C3134..0x832C3498)
	// 832C3134: 8141060C  lwz r10, 0x60c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1548 as u32) ) } as u64;
	// 832C3138: 7D315050  subf r9, r17, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[17].s64;
	// 832C313C: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 832C3140: 419817D0  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C3144: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 832C3148: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C314C: 41980010  blt cr6, 0x832c315c
	if ctx.cr[6].lt {
	pc = 0x832C315C; continue 'dispatch;
	}
	// 832C3150: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C3154: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C3158: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C315C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3160: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C3164: 7F1B3840  cmplw cr6, r27, r7
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C3168: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C316C: 5549403E  rotlwi r9, r10, 8
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 832C3170: 7D285378  or r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 832C3174: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3178: 54E6403E  rotlwi r6, r7, 8
	ctx.r[6].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 832C317C: 7CC53B78  or r5, r6, r7
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[7].u64;
	// 832C3180: 5504801E  slwi r4, r8, 0x10
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(16);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 832C3184: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C3188: 54A9801E  slwi r9, r5, 0x10
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C318C: 7C8A4378  or r10, r4, r8
	ctx.r[10].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 832C3190: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C3194: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 832C3198: 4198000C  blt cr6, 0x832c31a4
	if ctx.cr[6].lt {
	pc = 0x832C31A4; continue 'dispatch;
	}
	// 832C319C: 81010230  lwz r8, 0x230(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(560 as u32) ) } as u64;
	// 832C31A0: 3B680004  addi r27, r8, 4
	ctx.r[27].s64 = ctx.r[8].s64 + 4;
	// 832C31A4: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 832C31A8: 7CFDFA14  add r7, r29, r31
	ctx.r[7].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C31AC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 832C31B0: 88BB0000  lbz r5, 0(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C31B4: 389E24A8  addi r4, r30, 0x24a8
	ctx.r[4].s64 = ctx.r[30].s64 + 9384;
	// 832C31B8: 3B9E24B8  addi r28, r30, 0x24b8
	ctx.r[28].s64 = ctx.r[30].s64 + 9400;
	// 832C31BC: 8B3B0001  lbz r25, 1(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C31C0: 54B8173A  rlwinm r24, r5, 2, 0x1c, 0x1d
	ctx.r[24].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 832C31C4: 54B6F73A  rlwinm r22, r5, 0x1e, 0x1c, 0x1d
	ctx.r[22].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 832C31C8: 54B7073A  rlwinm r23, r5, 0, 0x1c, 0x1d
	ctx.r[23].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 832C31CC: 3A7E24A8  addi r19, r30, 0x24a8
	ctx.r[19].s64 = ctx.r[30].s64 + 9384;
	// 832C31D0: 54A5E13A  rlwinm r5, r5, 0x1c, 4, 0x1d
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 832C31D4: 7C98202E  lwzx r4, r24, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832C31D8: 3A5E24A8  addi r18, r30, 0x24a8
	ctx.r[18].s64 = ctx.r[30].s64 + 9384;
	// 832C31DC: 7F98E02E  lwzx r28, r24, r28
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 832C31E0: 3B1E24B8  addi r24, r30, 0x24b8
	ctx.r[24].s64 = ctx.r[30].s64 + 9400;
	// 832C31E4: 3ABE24A8  addi r21, r30, 0x24a8
	ctx.r[21].s64 = ctx.r[30].s64 + 9384;
	// 832C31E8: 3A9E24B8  addi r20, r30, 0x24b8
	ctx.r[20].s64 = ctx.r[30].s64 + 9400;
	// 832C31EC: 7E76982E  lwzx r19, r22, r19
	ctx.r[19].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 832C31F0: 3A3E24B8  addi r17, r30, 0x24b8
	ctx.r[17].s64 = ctx.r[30].s64 + 9400;
	// 832C31F4: 7C844838  and r4, r4, r9
	ctx.r[4].u64 = ctx.r[4].u64 & ctx.r[9].u64;
	// 832C31F8: 7F16C02E  lwzx r24, r22, r24
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 832C31FC: 7F9C5038  and r28, r28, r10
	ctx.r[28].u64 = ctx.r[28].u64 & ctx.r[10].u64;
	// 832C3200: 7EB7A82E  lwzx r21, r23, r21
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 832C3204: 7E734838  and r19, r19, r9
	ctx.r[19].u64 = ctx.r[19].u64 & ctx.r[9].u64;
	// 832C3208: 7EC5902E  lwzx r22, r5, r18
	ctx.r[22].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C320C: 3A5E24A8  addi r18, r30, 0x24a8
	ctx.r[18].s64 = ctx.r[30].s64 + 9384;
	// 832C3210: 7EF7A02E  lwzx r23, r23, r20
	ctx.r[23].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 832C3214: 5734173A  rlwinm r20, r25, 2, 0x1c, 0x1d
	ctx.r[20].u64 = ctx.r[25].u32 as u64 & 0x3FFFFFFFu64;
	// 832C3218: 7CA5882E  lwzx r5, r5, r17
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 832C321C: 3A3E24B8  addi r17, r30, 0x24b8
	ctx.r[17].s64 = ctx.r[30].s64 + 9400;
	// 832C3220: 7EB54838  and r21, r21, r9
	ctx.r[21].u64 = ctx.r[21].u64 & ctx.r[9].u64;
	// 832C3224: 7EF75038  and r23, r23, r10
	ctx.r[23].u64 = ctx.r[23].u64 & ctx.r[10].u64;
	// 832C3228: 7F185038  and r24, r24, r10
	ctx.r[24].u64 = ctx.r[24].u64 & ctx.r[10].u64;
	// 832C322C: 7ED64838  and r22, r22, r9
	ctx.r[22].u64 = ctx.r[22].u64 & ctx.r[9].u64;
	// 832C3230: 7CA55038  and r5, r5, r10
	ctx.r[5].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	// 832C3234: 7C84E378  or r4, r4, r28
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[28].u64;
	// 832C3238: 7F94902E  lwzx r28, r20, r18
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C323C: 7EB7BB78  or r23, r21, r23
	ctx.r[23].u64 = ctx.r[21].u64 | ctx.r[23].u64;
	// 832C3240: 7EB4882E  lwzx r21, r20, r17
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 832C3244: 5730073A  rlwinm r16, r25, 0, 0x1c, 0x1d
	ctx.r[16].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	// 832C3248: 90880000  stw r4, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 832C324C: 39FE24A8  addi r15, r30, 0x24a8
	ctx.r[15].s64 = ctx.r[30].s64 + 9384;
	// 832C3250: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 832C3254: 39DE24B8  addi r14, r30, 0x24b8
	ctx.r[14].s64 = ctx.r[30].s64 + 9400;
	// 832C3258: 92E80004  stw r23, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 832C325C: 7E78C378  or r24, r19, r24
	ctx.r[24].u64 = ctx.r[19].u64 | ctx.r[24].u64;
	// 832C3260: 92E70004  stw r23, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 832C3264: 7ED62B78  or r22, r22, r5
	ctx.r[22].u64 = ctx.r[22].u64 | ctx.r[5].u64;
	// 832C3268: 5734F73A  rlwinm r20, r25, 0x1e, 0x1c, 0x1d
	ctx.r[20].u64 = ctx.r[25].u32 as u64 & 0x00000003u64;
	// 832C326C: 93080008  stw r24, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 832C3270: 3A5E24A8  addi r18, r30, 0x24a8
	ctx.r[18].s64 = ctx.r[30].s64 + 9384;
	// 832C3274: 92C8000C  stw r22, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[22].u32 ) };
	// 832C3278: 3A3E24B8  addi r17, r30, 0x24b8
	ctx.r[17].s64 = ctx.r[30].s64 + 9400;
	// 832C327C: 7E70782E  lwzx r19, r16, r15
	ctx.r[19].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 832C3280: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 832C3284: 7F70702E  lwzx r27, r16, r14
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 832C3288: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C328C: 93070008  stw r24, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 832C3290: 7F844838  and r4, r28, r9
	ctx.r[4].u64 = ctx.r[28].u64 & ctx.r[9].u64;
	// 832C3294: 92C7000C  stw r22, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[22].u32 ) };
	// 832C3298: 7EBC5038  and r28, r21, r10
	ctx.r[28].u64 = ctx.r[21].u64 & ctx.r[10].u64;
	// 832C329C: 7F14902E  lwzx r24, r20, r18
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C32A0: 7EF4882E  lwzx r23, r20, r17
	ctx.r[23].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 832C32A4: 5739E13A  rlwinm r25, r25, 0x1c, 4, 0x1d
	ctx.r[25].u64 = ctx.r[25].u32 as u64 & 0x0000000Fu64;
	// 832C32A8: 3ABE24A8  addi r21, r30, 0x24a8
	ctx.r[21].s64 = ctx.r[30].s64 + 9384;
	// 832C32AC: 7E764838  and r22, r19, r9
	ctx.r[22].u64 = ctx.r[19].u64 & ctx.r[9].u64;
	// 832C32B0: 7F7B5038  and r27, r27, r10
	ctx.r[27].u64 = ctx.r[27].u64 & ctx.r[10].u64;
	// 832C32B4: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 832C32B8: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C32BC: 3A9E24B8  addi r20, r30, 0x24b8
	ctx.r[20].s64 = ctx.r[30].s64 + 9400;
	// 832C32C0: 7C84E378  or r4, r4, r28
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[28].u64;
	// 832C32C4: 7EDCDB78  or r28, r22, r27
	ctx.r[28].u64 = ctx.r[22].u64 | ctx.r[27].u64;
	// 832C32C8: 7F1B4838  and r27, r24, r9
	ctx.r[27].u64 = ctx.r[24].u64 & ctx.r[9].u64;
	// 832C32CC: 7F19A82E  lwzx r24, r25, r21
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 832C32D0: 90880000  stw r4, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 832C32D4: 7EF75038  and r23, r23, r10
	ctx.r[23].u64 = ctx.r[23].u64 & ctx.r[10].u64;
	// 832C32D8: 7C88F92E  stwx r4, r8, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.r[4].u32) };
	// 832C32DC: 389E24A8  addi r4, r30, 0x24a8
	ctx.r[4].s64 = ctx.r[30].s64 + 9384;
	// 832C32E0: 8A650000  lbz r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C32E4: 3ADE24B8  addi r22, r30, 0x24b8
	ctx.r[22].s64 = ctx.r[30].s64 + 9400;
	// 832C32E8: 7F39A02E  lwzx r25, r25, r20
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 832C32EC: 93880004  stw r28, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832C32F0: 93870004  stw r28, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832C32F4: 567C173A  rlwinm r28, r19, 2, 0x1c, 0x1d
	ctx.r[28].u64 = ctx.r[19].u32 as u64 & 0x3FFFFFFFu64;
	// 832C32F8: 5675073A  rlwinm r21, r19, 0, 0x1c, 0x1d
	ctx.r[21].u64 = ctx.r[19].u32 as u64 & 0xFFFFFFFFu64;
	// 832C32FC: 3A9E24A8  addi r20, r30, 0x24a8
	ctx.r[20].s64 = ctx.r[30].s64 + 9384;
	// 832C3300: 3A5E24B8  addi r18, r30, 0x24b8
	ctx.r[18].s64 = ctx.r[30].s64 + 9400;
	// 832C3304: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 832C3308: 7F184838  and r24, r24, r9
	ctx.r[24].u64 = ctx.r[24].u64 & ctx.r[9].u64;
	// 832C330C: 7C9C202E  lwzx r4, r28, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832C3310: 7F395038  and r25, r25, r10
	ctx.r[25].u64 = ctx.r[25].u64 & ctx.r[10].u64;
	// 832C3314: 7F9CB02E  lwzx r28, r28, r22
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 832C3318: 5671F73A  rlwinm r17, r19, 0x1e, 0x1c, 0x1d
	ctx.r[17].u64 = ctx.r[19].u32 as u64 & 0x00000003u64;
	// 832C331C: 7E95A02E  lwzx r20, r21, r20
	ctx.r[20].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 832C3320: 3A1E24A8  addi r16, r30, 0x24a8
	ctx.r[16].s64 = ctx.r[30].s64 + 9384;
	// 832C3324: 7EB5902E  lwzx r21, r21, r18
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C3328: 39FE24B8  addi r15, r30, 0x24b8
	ctx.r[15].s64 = ctx.r[30].s64 + 9400;
	// 832C332C: 8AC50000  lbz r22, 0(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3330: 7F7BBB78  or r27, r27, r23
	ctx.r[27].u64 = ctx.r[27].u64 | ctx.r[23].u64;
	// 832C3334: 7F19CB78  or r25, r24, r25
	ctx.r[25].u64 = ctx.r[24].u64 | ctx.r[25].u64;
	// 832C3338: 93680008  stw r27, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 832C333C: 5673E13A  rlwinm r19, r19, 0x1c, 4, 0x1d
	ctx.r[19].u64 = ctx.r[19].u32 as u64 & 0x0000000Fu64;
	// 832C3340: 3A5E24A8  addi r18, r30, 0x24a8
	ctx.r[18].s64 = ctx.r[30].s64 + 9384;
	// 832C3344: 93670008  stw r27, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 832C3348: 9328000C  stw r25, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 832C334C: 39DE24B8  addi r14, r30, 0x24b8
	ctx.r[14].s64 = ctx.r[30].s64 + 9400;
	// 832C3350: 9327000C  stw r25, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 832C3354: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C3358: 7EF1802E  lwzx r23, r17, r16
	ctx.r[23].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 832C335C: 38FE24A8  addi r7, r30, 0x24a8
	ctx.r[7].s64 = ctx.r[30].s64 + 9384;
	// 832C3360: 7F11782E  lwzx r24, r17, r15
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 832C3364: 56D1173A  rlwinm r17, r22, 2, 0x1c, 0x1d
	ctx.r[17].u64 = ctx.r[22].u32 as u64 & 0x3FFFFFFFu64;
	// 832C3368: 7E53902E  lwzx r18, r19, r18
	ctx.r[18].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[19].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C336C: 7C844838  and r4, r4, r9
	ctx.r[4].u64 = ctx.r[4].u64 & ctx.r[9].u64;
	// 832C3370: 7F9C5038  and r28, r28, r10
	ctx.r[28].u64 = ctx.r[28].u64 & ctx.r[10].u64;
	// 832C3374: 7E73702E  lwzx r19, r19, r14
	ctx.r[19].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[19].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 832C3378: 7EBB5038  and r27, r21, r10
	ctx.r[27].u64 = ctx.r[21].u64 & ctx.r[10].u64;
	// 832C337C: 7C84E378  or r4, r4, r28
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[28].u64;
	// 832C3380: 7EB1382E  lwzx r21, r17, r7
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 832C3384: 7E675038  and r7, r19, r10
	ctx.r[7].u64 = ctx.r[19].u64 & ctx.r[10].u64;
	// 832C3388: 7E5C4838  and r28, r18, r9
	ctx.r[28].u64 = ctx.r[18].u64 & ctx.r[9].u64;
	// 832C338C: 90880000  stw r4, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 832C3390: 7E944838  and r20, r20, r9
	ctx.r[20].u64 = ctx.r[20].u64 & ctx.r[9].u64;
	// 832C3394: 7C88F92E  stwx r4, r8, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.r[4].u32) };
	// 832C3398: 3B3E24B8  addi r25, r30, 0x24b8
	ctx.r[25].s64 = ctx.r[30].s64 + 9400;
	// 832C339C: 7F9C3B78  or r28, r28, r7
	ctx.r[28].u64 = ctx.r[28].u64 | ctx.r[7].u64;
	// 832C33A0: 7EF74838  and r23, r23, r9
	ctx.r[23].u64 = ctx.r[23].u64 & ctx.r[9].u64;
	// 832C33A4: 7F185038  and r24, r24, r10
	ctx.r[24].u64 = ctx.r[24].u64 & ctx.r[10].u64;
	// 832C33A8: 7E9BDB78  or r27, r20, r27
	ctx.r[27].u64 = ctx.r[20].u64 | ctx.r[27].u64;
	// 832C33AC: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C33B0: 7F31C82E  lwzx r25, r17, r25
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 832C33B4: 56D4073A  rlwinm r20, r22, 0, 0x1c, 0x1d
	ctx.r[20].u64 = ctx.r[22].u32 as u64 & 0xFFFFFFFFu64;
	// 832C33B8: 93680004  stw r27, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 832C33BC: 389E24A8  addi r4, r30, 0x24a8
	ctx.r[4].s64 = ctx.r[30].s64 + 9384;
	// 832C33C0: 7EF8C378  or r24, r23, r24
	ctx.r[24].u64 = ctx.r[23].u64 | ctx.r[24].u64;
	// 832C33C4: 3A7E24B8  addi r19, r30, 0x24b8
	ctx.r[19].s64 = ctx.r[30].s64 + 9400;
	// 832C33C8: 93080008  stw r24, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 832C33CC: 56D7F73A  rlwinm r23, r22, 0x1e, 0x1c, 0x1d
	ctx.r[23].u64 = ctx.r[22].u32 as u64 & 0x00000003u64;
	// 832C33D0: 93670004  stw r27, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 832C33D4: 3A5E24A8  addi r18, r30, 0x24a8
	ctx.r[18].s64 = ctx.r[30].s64 + 9384;
	// 832C33D8: 9388000C  stw r28, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 832C33DC: 7F3B5038  and r27, r25, r10
	ctx.r[27].u64 = ctx.r[25].u64 & ctx.r[10].u64;
	// 832C33E0: 7C94202E  lwzx r4, r20, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832C33E4: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C33E8: 7EB54838  and r21, r21, r9
	ctx.r[21].u64 = ctx.r[21].u64 & ctx.r[9].u64;
	// 832C33EC: 7E94982E  lwzx r20, r20, r19
	ctx.r[20].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 832C33F0: 93070008  stw r24, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 832C33F4: 3B3E24A8  addi r25, r30, 0x24a8
	ctx.r[25].s64 = ctx.r[30].s64 + 9384;
	// 832C33F8: 56D6E13A  rlwinm r22, r22, 0x1c, 4, 0x1d
	ctx.r[22].u64 = ctx.r[22].u32 as u64 & 0x0000000Fu64;
	// 832C33FC: 9387000C  stw r28, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 832C3400: 3A7E24B8  addi r19, r30, 0x24b8
	ctx.r[19].s64 = ctx.r[30].s64 + 9400;
	// 832C3404: 7F17902E  lwzx r24, r23, r18
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C3408: 7EBCDB78  or r28, r21, r27
	ctx.r[28].u64 = ctx.r[21].u64 | ctx.r[27].u64;
	// 832C340C: 7C844838  and r4, r4, r9
	ctx.r[4].u64 = ctx.r[4].u64 & ctx.r[9].u64;
	// 832C3410: 7E955038  and r21, r20, r10
	ctx.r[21].u64 = ctx.r[20].u64 & ctx.r[10].u64;
	// 832C3414: 93880000  stw r28, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832C3418: 7F88F92E  stwx r28, r8, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u32) };
	// 832C341C: 3B650001  addi r27, r5, 1
	ctx.r[27].s64 = ctx.r[5].s64 + 1;
	// 832C3420: 7F96C82E  lwzx r28, r22, r25
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 832C3424: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C3428: 7EF7982E  lwzx r23, r23, r19
	ctx.r[23].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 832C342C: 38BE24B8  addi r5, r30, 0x24b8
	ctx.r[5].s64 = ctx.r[30].s64 + 9400;
	// 832C3430: 7F194838  and r25, r24, r9
	ctx.r[25].u64 = ctx.r[24].u64 & ctx.r[9].u64;
	// 832C3434: 7C84AB78  or r4, r4, r21
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[21].u64;
	// 832C3438: 7CB6282E  lwzx r5, r22, r5
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 832C343C: 7EF85038  and r24, r23, r10
	ctx.r[24].u64 = ctx.r[23].u64 & ctx.r[10].u64;
	// 832C3440: 7F9C4838  and r28, r28, r9
	ctx.r[28].u64 = ctx.r[28].u64 & ctx.r[9].u64;
	// 832C3444: 90880004  stw r4, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 832C3448: 7CA55038  and r5, r5, r10
	ctx.r[5].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	// 832C344C: 90870004  stw r4, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 832C3450: 7F24C378  or r4, r25, r24
	ctx.r[4].u64 = ctx.r[25].u64 | ctx.r[24].u64;
	// 832C3454: 7F852B78  or r5, r28, r5
	ctx.r[5].u64 = ctx.r[28].u64 | ctx.r[5].u64;
	// 832C3458: 90880008  stw r4, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 832C345C: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 832C3460: 90A8000C  stw r5, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 832C3464: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C3468: 90870008  stw r4, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 832C346C: 90A7000C  stw r5, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 832C3470: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C3474: 4082FD3C  bne 0x832c31b0
	if !ctx.cr[0].eq {
	pc = 0x832C31B0; continue 'dispatch;
	}
	// 832C3478: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C347C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 832C3480: 82C10140  lwz r22, 0x140(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) } as u64;
	// 832C3484: 83210180  lwz r25, 0x180(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 832C3488: 82E10084  lwz r23, 0x84(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 832C348C: 83010078  lwz r24, 0x78(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 832C3490: 93610200  stw r27, 0x200(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(512 as u32), ctx.r[27].u32 ) };
	// 832C3494: 48000340  b 0x832c37d4
	pc = 0x832C37D4; continue 'dispatch;
            }
            0x832C3498 => {
    //   block [0x832C3498..0x832C37D4)
	// 832C3498: 8141060C  lwz r10, 0x60c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1548 as u32) ) } as u64;
	// 832C349C: 7D315050  subf r9, r17, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[17].s64;
	// 832C34A0: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 832C34A4: 4198146C  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C34A8: 80E100B4  lwz r7, 0xb4(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 832C34AC: 7D1DFA14  add r8, r29, r31
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C34B0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 832C34B4: 57E9083C  slwi r9, r31, 1
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C34B8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C34BC: 41980010  blt cr6, 0x832c34cc
	if ctx.cr[6].lt {
	pc = 0x832C34CC; continue 'dispatch;
	}
	// 832C34C0: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C34C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C34C8: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C34CC: 556707BE  clrlwi r7, r11, 0x1e
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 832C34D0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 832C34D4: 419A00CC  beq cr6, 0x832c35a0
	if ctx.cr[6].eq {
	pc = 0x832C35A0; continue 'dispatch;
	}
	// 832C34D8: 7CDD4050  subf r6, r29, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[29].s64;
	// 832C34DC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 832C34E0: 8AAB0004  lbz r21, 4(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C34E4: 7D065214  add r8, r6, r10
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	// 832C34E8: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C34EC: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 832C34F0: 8B8B0002  lbz r28, 2(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C34F4: 56B1403E  rotlwi r17, r21, 8
	ctx.r[17].u64 = ((ctx.r[21].u32).rotate_left(8)) as u64;
	// 832C34F8: 8A6B0006  lbz r19, 6(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C34FC: 54B4403E  rotlwi r20, r5, 8
	ctx.r[20].u64 = ((ctx.r[5].u32).rotate_left(8)) as u64;
	// 832C3500: 5792403E  rotlwi r18, r28, 8
	ctx.r[18].u64 = ((ctx.r[28].u32).rotate_left(8)) as u64;
	// 832C3504: 8A0B0001  lbz r16, 1(r11)
	ctx.r[16].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C3508: 566F403E  rotlwi r15, r19, 8
	ctx.r[15].u64 = ((ctx.r[19].u32).rotate_left(8)) as u64;
	// 832C350C: 89CB0003  lbz r14, 3(r11)
	ctx.r[14].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C3510: 7E35AB78  or r21, r17, r21
	ctx.r[21].u64 = ctx.r[17].u64 | ctx.r[21].u64;
	// 832C3514: 7E852B78  or r5, r20, r5
	ctx.r[5].u64 = ctx.r[20].u64 | ctx.r[5].u64;
	// 832C3518: 8A8B0005  lbz r20, 5(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C351C: 7E5CE378  or r28, r18, r28
	ctx.r[28].u64 = ctx.r[18].u64 | ctx.r[28].u64;
	// 832C3520: 8A4B0007  lbz r18, 7(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C3524: 7DF39B78  or r19, r15, r19
	ctx.r[19].u64 = ctx.r[15].u64 | ctx.r[19].u64;
	// 832C3528: 56B5402E  slwi r21, r21, 8
	ctx.r[21].u32 = ctx.r[21].u32.wrapping_shl(8);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 832C352C: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C3530: 579C402E  slwi r28, r28, 8
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(8);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C3534: 5673402E  slwi r19, r19, 8
	ctx.r[19].u32 = ctx.r[19].u32.wrapping_shl(8);
	ctx.r[19].u64 = ctx.r[19].u32 as u64;
	// 832C3538: 7EB5A378  or r21, r21, r20
	ctx.r[21].u64 = ctx.r[21].u64 | ctx.r[20].u64;
	// 832C353C: 7CA58378  or r5, r5, r16
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[16].u64;
	// 832C3540: 7F9C7378  or r28, r28, r14
	ctx.r[28].u64 = ctx.r[28].u64 | ctx.r[14].u64;
	// 832C3544: 7E739378  or r19, r19, r18
	ctx.r[19].u64 = ctx.r[19].u64 | ctx.r[18].u64;
	// 832C3548: 56B5402E  slwi r21, r21, 8
	ctx.r[21].u32 = ctx.r[21].u32.wrapping_shl(8);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 832C354C: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C3550: 579C402E  slwi r28, r28, 8
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(8);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C3554: 5673402E  slwi r19, r19, 8
	ctx.r[19].u32 = ctx.r[19].u32.wrapping_shl(8);
	ctx.r[19].u64 = ctx.r[19].u32 as u64;
	// 832C3558: 7EB5A378  or r21, r21, r20
	ctx.r[21].u64 = ctx.r[21].u64 | ctx.r[20].u64;
	// 832C355C: 7CA58378  or r5, r5, r16
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[16].u64;
	// 832C3560: 7F9C7378  or r28, r28, r14
	ctx.r[28].u64 = ctx.r[28].u64 | ctx.r[14].u64;
	// 832C3564: 92AA0008  stw r21, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[21].u32 ) };
	// 832C3568: 7E749378  or r20, r19, r18
	ctx.r[20].u64 = ctx.r[19].u64 | ctx.r[18].u64;
	// 832C356C: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 832C3570: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 832C3574: 7CA6512E  stwx r5, r6, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	// 832C3578: 938A0004  stw r28, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832C357C: 928A000C  stw r20, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[20].u32 ) };
	// 832C3580: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 832C3584: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C3588: 93880004  stw r28, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832C358C: 92A80008  stw r21, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[21].u32 ) };
	// 832C3590: 9288000C  stw r20, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[20].u32 ) };
	// 832C3594: 4082FF4C  bne 0x832c34e0
	if !ctx.cr[0].eq {
	pc = 0x832C34E0; continue 'dispatch;
	}
	// 832C3598: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 832C359C: 48000238  b 0x832c37d4
	pc = 0x832C37D4; continue 'dispatch;
	// 832C35A0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832C35A4: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C35A8: A38B0002  lhz r28, 2(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C35AC: A28B0004  lhz r20, 4(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C35B0: 54D3401E  rlwinm r19, r6, 8, 0, 0xf
	ctx.r[19].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 832C35B4: A24B0006  lhz r18, 6(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C35B8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 832C35BC: 5791401E  rlwinm r17, r28, 8, 0, 0xf
	ctx.r[17].u64 = ctx.r[28].u32 as u64 & 0x00FFFFFFu64;
	// 832C35C0: F8610098  std r3, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[3].u64 ) };
	// 832C35C4: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C35C8: 7CCE3378  mr r14, r6
	ctx.r[14].u64 = ctx.r[6].u64;
	// 832C35CC: 7E663378  or r6, r19, r6
	ctx.r[6].u64 = ctx.r[19].u64 | ctx.r[6].u64;
	// 832C35D0: F8810090  std r4, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 832C35D4: 5690401E  rlwinm r16, r20, 8, 0, 0xf
	ctx.r[16].u64 = ctx.r[20].u32 as u64 & 0x00FFFFFFu64;
	// 832C35D8: 7F93E378  mr r19, r28
	ctx.r[19].u64 = ctx.r[28].u64;
	// 832C35DC: 7E3CE378  or r28, r17, r28
	ctx.r[28].u64 = ctx.r[17].u64 | ctx.r[28].u64;
	// 832C35E0: 7E91A378  mr r17, r20
	ctx.r[17].u64 = ctx.r[20].u64;
	// 832C35E4: 7E14A378  or r20, r16, r20
	ctx.r[20].u64 = ctx.r[16].u64 | ctx.r[20].u64;
	// 832C35E8: 564F401E  rlwinm r15, r18, 8, 0, 0xf
	ctx.r[15].u64 = ctx.r[18].u32 as u64 & 0x00FFFFFFu64;
	// 832C35EC: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 832C35F0: 7E509378  mr r16, r18
	ctx.r[16].u64 = ctx.r[18].u64;
	// 832C35F4: 5393402E  rlwimi r19, r28, 8, 0, 0x17
	ctx.r[19].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[19].u64 & 0xFFFFFFFF000000FF);
	// 832C35F8: 5291402E  rlwimi r17, r20, 8, 0, 0x17
	ctx.r[17].u64 = (((ctx.r[20].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[17].u64 & 0xFFFFFFFF000000FF);
	// 832C35FC: 50CE402E  rlwimi r14, r6, 8, 0, 0x17
	ctx.r[14].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[14].u64 & 0xFFFFFFFF000000FF);
	// 832C3600: 92680004  stw r19, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[19].u32 ) };
	// 832C3604: 7DF29378  or r18, r15, r18
	ctx.r[18].u64 = ctx.r[15].u64 | ctx.r[18].u64;
	// 832C3608: 926A0004  stw r19, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[19].u32 ) };
	// 832C360C: 92280008  stw r17, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[17].u32 ) };
	// 832C3610: 5250402E  rlwimi r16, r18, 8, 0, 0x17
	ctx.r[16].u64 = (((ctx.r[18].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[16].u64 & 0xFFFFFFFF000000FF);
	// 832C3614: 91CA0000  stw r14, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[14].u32 ) };
	// 832C3618: 922A0008  stw r17, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[17].u32 ) };
	// 832C361C: 920A000C  stw r16, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[16].u32 ) };
	// 832C3620: 91C80000  stw r14, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[14].u32 ) };
	// 832C3624: 9208000C  stw r16, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[16].u32 ) };
	// 832C3628: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C362C: A28B0004  lhz r20, 4(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3630: A1EB0006  lhz r15, 6(r11)
	ctx.r[15].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3634: 54D2401E  rlwinm r18, r6, 8, 0, 0xf
	ctx.r[18].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 832C3638: A38B0002  lhz r28, 2(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C363C: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 832C3640: 90E100B0  stw r7, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[7].u32 ) };
	// 832C3644: 7E463378  or r6, r18, r6
	ctx.r[6].u64 = ctx.r[18].u64 | ctx.r[6].u64;
	// 832C3648: 578B401E  rlwinm r11, r28, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00FFFFFFu64;
	// 832C364C: 7F93E378  mr r19, r28
	ctx.r[19].u64 = ctx.r[28].u64;
	// 832C3650: 7D7CE378  or r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 832C3654: 50C3402E  rlwimi r3, r6, 8, 0, 0x17
	ctx.r[3].u64 = (((ctx.r[6].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[3].u64 & 0xFFFFFFFF000000FF);
	// 832C3658: 5393402E  rlwimi r19, r28, 8, 0, 0x17
	ctx.r[19].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[19].u64 & 0xFFFFFFFF000000FF);
	// 832C365C: 5684401E  rlwinm r4, r20, 8, 0, 0xf
	ctx.r[4].u64 = ctx.r[20].u32 as u64 & 0x00FFFFFFu64;
	// 832C3660: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 832C3664: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 832C3668: 55F2401E  rlwinm r18, r15, 8, 0, 0xf
	ctx.r[18].u64 = ctx.r[15].u32 as u64 & 0x00FFFFFFu64;
	// 832C366C: 7D494214  add r10, r9, r8
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 832C3670: 7F89412E  stwx r28, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[28].u32) };
	// 832C3674: 7E91A378  mr r17, r20
	ctx.r[17].u64 = ctx.r[20].u64;
	// 832C3678: 7C94A378  or r20, r4, r20
	ctx.r[20].u64 = ctx.r[4].u64 | ctx.r[20].u64;
	// 832C367C: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832C3680: 7DE67B78  mr r6, r15
	ctx.r[6].u64 = ctx.r[15].u64;
	// 832C3684: 926B0004  stw r19, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[19].u32 ) };
	// 832C3688: 7E527B78  or r18, r18, r15
	ctx.r[18].u64 = ctx.r[18].u64 | ctx.r[15].u64;
	// 832C368C: 5291402E  rlwimi r17, r20, 8, 0, 0x17
	ctx.r[17].u64 = (((ctx.r[20].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[17].u64 & 0xFFFFFFFF000000FF);
	// 832C3690: 926A0004  stw r19, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[19].u32 ) };
	// 832C3694: 5246402E  rlwimi r6, r18, 8, 0, 0x17
	ctx.r[6].u64 = (((ctx.r[18].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[6].u64 & 0xFFFFFFFF000000FF);
	// 832C3698: 922B0008  stw r17, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[17].u32 ) };
	// 832C369C: 922A0008  stw r17, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[17].u32 ) };
	// 832C36A0: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 832C36A4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 832C36A8: A1070000  lhz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C36AC: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 832C36B0: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 832C36B4: 551C401E  rlwinm r28, r8, 8, 0, 0xf
	ctx.r[28].u64 = ctx.r[8].u32 as u64 & 0x00FFFFFFu64;
	// 832C36B8: A0C70002  lhz r6, 2(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C36BC: 7D144378  mr r20, r8
	ctx.r[20].u64 = ctx.r[8].u64;
	// 832C36C0: 7F884378  or r8, r28, r8
	ctx.r[8].u64 = ctx.r[28].u64 | ctx.r[8].u64;
	// 832C36C4: A3870004  lhz r28, 4(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C36C8: 5114402E  rlwimi r20, r8, 8, 0, 0x17
	ctx.r[20].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[20].u64 & 0xFFFFFFFF000000FF);
	// 832C36CC: 39070008  addi r8, r7, 8
	ctx.r[8].s64 = ctx.r[7].s64 + 8;
	// 832C36D0: A0E70006  lhz r7, 6(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C36D4: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 832C36D8: 5793401E  rlwinm r19, r28, 8, 0, 0xf
	ctx.r[19].u64 = ctx.r[28].u32 as u64 & 0x00FFFFFFu64;
	// 832C36DC: 928A0000  stw r20, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 832C36E0: 54D4401E  rlwinm r20, r6, 8, 0, 0xf
	ctx.r[20].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 832C36E4: 910100B0  stw r8, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[8].u32 ) };
	// 832C36E8: 54F2401E  rlwinm r18, r7, 8, 0, 0xf
	ctx.r[18].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 832C36EC: A2280002  lhz r17, 2(r8)
	ctx.r[17].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C36F0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 832C36F4: FBE100F0  std r31, 0xf0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[31].u64 ) };
	// 832C36F8: 7E473B78  or r7, r18, r7
	ctx.r[7].u64 = ctx.r[18].u64 | ctx.r[7].u64;
	// 832C36FC: A2080004  lhz r16, 4(r8)
	ctx.r[16].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3700: 7F8FE378  mr r15, r28
	ctx.r[15].u64 = ctx.r[28].u64;
	// 832C3704: A1C80006  lhz r14, 6(r8)
	ctx.r[14].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3708: 50E3402E  rlwimi r3, r7, 8, 0, 0x17
	ctx.r[3].u64 = (((ctx.r[7].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[3].u64 & 0xFFFFFFFF000000FF);
	// 832C370C: 7E278B78  mr r7, r17
	ctx.r[7].u64 = ctx.r[17].u64;
	// 832C3710: 7E7CE378  or r28, r19, r28
	ctx.r[28].u64 = ctx.r[19].u64 | ctx.r[28].u64;
	// 832C3714: A2680000  lhz r19, 0(r8)
	ctx.r[19].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3718: 90E10088  stw r7, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[7].u32 ) };
	// 832C371C: 7E943378  or r20, r20, r6
	ctx.r[20].u64 = ctx.r[20].u64 | ctx.r[6].u64;
	// 832C3720: 5664401E  rlwinm r4, r19, 8, 0, 0xf
	ctx.r[4].u64 = ctx.r[19].u32 as u64 & 0x00FFFFFFu64;
	// 832C3724: 5286402E  rlwimi r6, r20, 8, 0, 0x17
	ctx.r[6].u64 = (((ctx.r[20].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[6].u64 & 0xFFFFFFFF000000FF);
	// 832C3728: 538F402E  rlwimi r15, r28, 8, 0, 0x17
	ctx.r[15].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[15].u64 & 0xFFFFFFFF000000FF);
	// 832C372C: 7E7F9B78  mr r31, r19
	ctx.r[31].u64 = ctx.r[19].u64;
	// 832C3730: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C3734: 5632401E  rlwinm r18, r17, 8, 0, 0xf
	ctx.r[18].u64 = ctx.r[17].u32 as u64 & 0x00FFFFFFu64;
	// 832C3738: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C373C: 7C939B78  or r19, r4, r19
	ctx.r[19].u64 = ctx.r[4].u64 | ctx.r[19].u64;
	// 832C3740: 91EB0008  stw r15, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[15].u32 ) };
	// 832C3744: 91EA0008  stw r15, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[15].u32 ) };
	// 832C3748: 5614401E  rlwinm r20, r16, 8, 0, 0xf
	ctx.r[20].u64 = ctx.r[16].u32 as u64 & 0x00FFFFFFu64;
	// 832C374C: 7E528B78  or r18, r18, r17
	ctx.r[18].u64 = ctx.r[18].u64 | ctx.r[17].u64;
	// 832C3750: E8810090  ld r4, 0x90(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	// 832C3754: 527F402E  rlwimi r31, r19, 8, 0, 0x17
	ctx.r[31].u64 = (((ctx.r[19].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[31].u64 & 0xFFFFFFFF000000FF);
	// 832C3758: 7E118378  mr r17, r16
	ctx.r[17].u64 = ctx.r[16].u64;
	// 832C375C: 7E948378  or r20, r20, r16
	ctx.r[20].u64 = ctx.r[20].u64 | ctx.r[16].u64;
	// 832C3760: 55DC401E  rlwinm r28, r14, 8, 0, 0xf
	ctx.r[28].u64 = ctx.r[14].u32 as u64 & 0x00FFFFFFu64;
	// 832C3764: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 832C3768: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 832C376C: 7C701B78  mr r16, r3
	ctx.r[16].u64 = ctx.r[3].u64;
	// 832C3770: E8610098  ld r3, 0x98(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) };
	// 832C3774: 7DD37378  mr r19, r14
	ctx.r[19].u64 = ctx.r[14].u64;
	// 832C3778: 7F9C7378  or r28, r28, r14
	ctx.r[28].u64 = ctx.r[28].u64 | ctx.r[14].u64;
	// 832C377C: 920B000C  stw r16, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[16].u32 ) };
	// 832C3780: 920A000C  stw r16, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[16].u32 ) };
	// 832C3784: 5291402E  rlwimi r17, r20, 8, 0, 0x17
	ctx.r[17].u64 = (((ctx.r[20].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[17].u64 & 0xFFFFFFFF000000FF);
	// 832C3788: 5393402E  rlwimi r19, r28, 8, 0, 0x17
	ctx.r[19].u64 = (((ctx.r[28].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[19].u64 & 0xFFFFFFFF000000FF);
	// 832C378C: 81E10088  lwz r15, 0x88(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 832C3790: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 832C3794: 524F402E  rlwimi r15, r18, 8, 0, 0x17
	ctx.r[15].u64 = (((ctx.r[18].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[15].u64 & 0xFFFFFFFF000000FF);
	// 832C3798: 7FF2FB78  mr r18, r31
	ctx.r[18].u64 = ctx.r[31].u64;
	// 832C379C: EBE100F0  ld r31, 0xf0(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) };
	// 832C37A0: 7E49592E  stwx r18, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[18].u32) };
	// 832C37A4: 39680008  addi r11, r8, 8
	ctx.r[11].s64 = ctx.r[8].s64 + 8;
	// 832C37A8: 7E49512E  stwx r18, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[18].u32) };
	// 832C37AC: 7D493A14  add r10, r9, r7
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 832C37B0: 91E70004  stw r15, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[15].u32 ) };
	// 832C37B4: 7D093214  add r8, r9, r6
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 832C37B8: 91E60004  stw r15, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[15].u32 ) };
	// 832C37BC: 92270008  stw r17, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[17].u32 ) };
	// 832C37C0: 92260008  stw r17, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[17].u32 ) };
	// 832C37C4: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C37C8: 9267000C  stw r19, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[19].u32 ) };
	// 832C37CC: 9266000C  stw r19, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[19].u32 ) };
	// 832C37D0: 4082FDD4  bne 0x832c35a4
	if !ctx.cr[0].eq {
	pc = 0x832C35A4; continue 'dispatch;
	}
            }
            0x832C37D4 => {
    //   block [0x832C37D4..0x832C37E8)
	// 832C37D4: 82610080  lwz r19, 0x80(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C37D8: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 832C37DC: 8221007C  lwz r17, 0x7c(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 832C37E0: 82010100  lwz r16, 0x100(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 832C37E4: 824101C0  lwz r18, 0x1c0(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(448 as u32) ) } as u64;
	pc = 0x832C37E8; continue 'dispatch;
            }
            0x832C37E8 => {
    //   block [0x832C37E8..0x832C3818)
	// 832C37E8: 81210644  lwz r9, 0x644(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1604 as u32) ) } as u64;
	// 832C37EC: 56E8F87E  srwi r8, r23, 1
	ctx.r[8].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C37F0: 80E100A0  lwz r7, 0xa0(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 832C37F4: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 832C37F8: 7D574A14  add r10, r23, r9
	ctx.r[10].u64 = ctx.r[23].u64 + ctx.r[9].u64;
	// 832C37FC: 3B180010  addi r24, r24, 0x10
	ctx.r[24].s64 = ctx.r[24].s64 + 16;
	// 832C3800: 5546F87E  srwi r6, r10, 1
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832C3804: 7E8899AE  stbx r20, r8, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C3808: 38A7FFF0  addi r5, r7, -0x10
	ctx.r[5].s64 = ctx.r[7].s64 + -16;
	// 832C380C: 7EEA4A14  add r23, r10, r9
	ctx.r[23].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 832C3810: 7E8699AE  stbx r20, r6, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C3814: 48001114  b 0x832c4928
	pc = 0x832C4928; continue 'dispatch;
            }
            0x832C3818 => {
    //   block [0x832C3818..0x832C3D18)
	// 832C3818: 56EAF87E  srwi r10, r23, 1
	ctx.r[10].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C381C: 7F194840  cmplw cr6, r25, r9
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C3820: 7E8A99AE  stbx r20, r10, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C3824: 41980010  blt cr6, 0x832c3834
	if ctx.cr[6].lt {
	pc = 0x832C3834; continue 'dispatch;
	}
	// 832C3828: 81410170  lwz r10, 0x170(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 832C382C: 3B2E0004  addi r25, r14, 4
	ctx.r[25].s64 = ctx.r[14].s64 + 4;
	// 832C3830: 3ACA0004  addi r22, r10, 4
	ctx.r[22].s64 = ctx.r[10].s64 + 4;
	// 832C3834: 89590000  lbz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3838: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 832C383C: 89360000  lbz r9, 0(r22)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3840: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 832C3844: 7D480774  extsb r8, r10
	ctx.r[8].s64 = ctx.r[10].s8 as i64;
	// 832C3848: 80E105FC  lwz r7, 0x5fc(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1532 as u32) ) } as u64;
	// 832C384C: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 832C3850: 93210180  stw r25, 0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[25].u32 ) };
	// 832C3854: 7D28F9D6  mullw r9, r8, r31
	ctx.r[9].s32 = ((ctx.r[8].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 832C3858: 92C10140  stw r22, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[22].u32 ) };
	// 832C385C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 832C3860: 7D2AC214  add r9, r10, r24
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 832C3864: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C3868: 419810A8  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C386C: 814100E8  lwz r10, 0xe8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 832C3870: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C3874: 4199109C  bgt cr6, 0x832c4910
	if ctx.cr[6].gt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C3878: 7D27EB78  or r7, r9, r29
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[29].u64;
	// 832C387C: 7D1DFA14  add r8, r29, r31
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C3880: 54E6077E  clrlwi r6, r7, 0x1d
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x00000007u64;
	// 832C3884: 910100F0  stw r8, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[8].u32 ) };
	// 832C3888: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C388C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 832C3890: 409A0070  bne cr6, 0x832c3900
	if !ctx.cr[6].eq {
	pc = 0x832C3900; continue 'dispatch;
	}
	// 832C3894: C8090000  lfd f0, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 832C3898: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C389C: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C38A0: C9AA0000  lfd f13, 0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 832C38A4: D9A80000  stfd f13, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.f[13].u64 ) };
	// 832C38A8: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C38AC: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C38B0: D81D0000  stfd f0, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 832C38B4: C9890000  lfd f12, 0(r9)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 832C38B8: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C38BC: D9870000  stfd f12, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.f[12].u64 ) };
	// 832C38C0: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C38C4: C96A0000  lfd f11, 0(r10)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 832C38C8: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C38CC: D9680000  stfd f11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.f[11].u64 ) };
	// 832C38D0: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C38D4: C9490000  lfd f10, 0(r9)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 832C38D8: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C38DC: D9470000  stfd f10, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.f[10].u64 ) };
	// 832C38E0: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C38E4: C92A0000  lfd f9, 0(r10)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 832C38E8: 7D0AFCAE  lfdx f8, r10, r31
	ctx.f[8].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	// 832C38EC: D9280000  stfd f9, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.f[9].u64 ) };
	// 832C38F0: 7CE9FCAE  lfdx f7, r9, r31
	ctx.f[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) };
	// 832C38F4: 7D08FDAE  stfdx f8, r8, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.f[8].u64) };
	// 832C38F8: 7CE7FDAE  stfdx f7, r7, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32), ctx.f[7].u64) };
	// 832C38FC: 48001014  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
	// 832C3900: 54E707BE  clrlwi r7, r7, 0x1e
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 832C3904: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 832C3908: 409A00D0  bne cr6, 0x832c39d8
	if !ctx.cr[6].eq {
	pc = 0x832C39D8; continue 'dispatch;
	}
	// 832C390C: 80C90000  lwz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3910: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C3914: 80A90004  lwz r5, 4(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3918: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C391C: 838A0000  lwz r28, 0(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3920: 81EA0004  lwz r15, 4(r10)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3924: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C3928: F9610090  std r11, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u64 ) };
	// 832C392C: 90DD0000  stw r6, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 832C3930: 80C90000  lwz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3934: 81C90004  lwz r14, 4(r9)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3938: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C393C: 93880000  stw r28, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832C3940: 91E80004  stw r15, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[15].u32 ) };
	// 832C3944: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C3948: 838A0000  lwz r28, 0(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C394C: 81EA0004  lwz r15, 4(r10)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3950: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C3954: 91C70004  stw r14, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[14].u32 ) };
	// 832C3958: 81C90000  lwz r14, 0(r9)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C395C: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3960: F8810098  std r4, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[4].u64 ) };
	// 832C3964: 90C70000  stw r6, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 832C3968: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C396C: 93880000  stw r28, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832C3970: 91E80004  stw r15, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[15].u32 ) };
	// 832C3974: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C3978: 91210088  stw r9, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[9].u32 ) };
	// 832C397C: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3980: 838A0004  lwz r28, 4(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3984: 7CC9FA14  add r6, r9, r31
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C3988: 81EA0000  lwz r15, 0(r10)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C398C: 7D48FA14  add r10, r8, r31
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C3990: 91C70000  stw r14, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[14].u32 ) };
	// 832C3994: 90BD0004  stw r5, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 832C3998: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C399C: 80890004  lwz r4, 4(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C39A0: 93880004  stw r28, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832C39A4: 7D0AFA14  add r8, r10, r31
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C39A8: 7D29F82E  lwzx r9, r9, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 832C39AC: 80C60004  lwz r6, 4(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C39B0: 83810088  lwz r28, 0x88(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 832C39B4: 93870004  stw r28, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832C39B8: 7DE7F92E  stwx r15, r7, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32), ctx.r[15].u32) };
	// 832C39BC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832C39C0: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 832C39C4: E9610090  ld r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	// 832C39C8: E8810098  ld r4, 0x98(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) };
	// 832C39CC: 7D2AF92E  stwx r9, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u32) };
	// 832C39D0: 90C80004  stw r6, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C39D4: 48000F3C  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
	// 832C39D8: 7D08FA14  add r8, r8, r31
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C39DC: 8B490004  lbz r26, 4(r9)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C39E0: 8B090005  lbz r24, 5(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C39E4: 8AE90006  lbz r23, 6(r9)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C39E8: 8AA90007  lbz r21, 7(r9)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C39EC: 88A90000  lbz r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C39F0: 88890001  lbz r4, 1(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C39F4: 88690002  lbz r3, 2(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C39F8: 8B890003  lbz r28, 3(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C39FC: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3A00: 91010088  stw r8, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[8].u32 ) };
	// 832C3A04: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3A08: 88EA0001  lbz r7, 1(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C3A0C: 88CA0002  lbz r6, 2(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C3A10: 8BCA0003  lbz r30, 3(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C3A14: 8BAA0004  lbz r29, 4(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3A18: 8B6A0005  lbz r27, 5(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C3A1C: 8B2A0006  lbz r25, 6(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3A20: 8ACA0007  lbz r22, 7(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C3A24: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C3A28: F96102B8  std r11, 0x2b8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(696 as u32), ctx.r[11].u64 ) };
	// 832C3A2C: 91410098  stw r10, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 832C3A30: 7E6AFA14  add r19, r10, r31
	ctx.r[19].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3A34: 8A890000  lbz r20, 0(r9)
	ctx.r[20].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3A38: 8A490001  lbz r18, 1(r9)
	ctx.r[18].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C3A3C: 8A290002  lbz r17, 2(r9)
	ctx.r[17].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C3A40: 8A090003  lbz r16, 3(r9)
	ctx.r[16].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C3A44: 89E90004  lbz r15, 4(r9)
	ctx.r[15].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3A48: 89C90005  lbz r14, 5(r9)
	ctx.r[14].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C3A4C: 89690006  lbz r11, 6(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3A50: 89290007  lbz r9, 7(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C3A54: 92610090  stw r19, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[19].u32 ) };
	// 832C3A58: 814100F0  lwz r10, 0xf0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 832C3A5C: 9AE10070  stb r23, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[23].u8 ) };
	// 832C3A60: 9B010077  stb r24, 0x77(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(119 as u32), ctx.r[24].u8 ) };
	// 832C3A64: 99210052  stb r9, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[9].u8 ) };
	// 832C3A68: 5669003E  slwi r9, r19, 0
	ctx.r[9].u32 = ctx.r[19].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C3A6C: 82610088  lwz r19, 0x88(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 832C3A70: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 832C3A74: 7E73FA14  add r19, r19, r31
	ctx.r[19].u64 = ctx.r[19].u64 + ctx.r[31].u64;
	// 832C3A78: 98EA0001  stb r7, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 832C3A7C: 98CA0002  stb r6, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 832C3A80: 9BCA0003  stb r30, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[30].u8 ) };
	// 832C3A84: 9BAA0004  stb r29, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 832C3A88: 9B6A0005  stb r27, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[27].u8 ) };
	// 832C3A8C: 9B2A0006  stb r25, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[25].u8 ) };
	// 832C3A90: 9ACA0007  stb r22, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[22].u8 ) };
	// 832C3A94: 81410098  lwz r10, 0x98(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 832C3A98: 926100F0  stw r19, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[19].u32 ) };
	// 832C3A9C: 9AA10051  stb r21, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[21].u8 ) };
	// 832C3AA0: 9B410076  stb r26, 0x76(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[26].u8 ) };
	// 832C3AA4: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3AA8: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C3AAC: 8B2A0000  lbz r25, 0(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3AB0: 8ACA0001  lbz r22, 1(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C3AB4: 8A6A0002  lbz r19, 2(r10)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C3AB8: 88EA0004  lbz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3ABC: 8AEA0005  lbz r23, 5(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C3AC0: 8B0A0006  lbz r24, 6(r10)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3AC4: 894A0007  lbz r10, 7(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C3AC8: 82A10090  lwz r21, 0x90(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 832C3ACC: 8BC90001  lbz r30, 1(r9)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C3AD0: 7EB5FA14  add r21, r21, r31
	ctx.r[21].u64 = ctx.r[21].u64 + ctx.r[31].u64;
	// 832C3AD4: 8BA90002  lbz r29, 2(r9)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C3AD8: 8B690003  lbz r27, 3(r9)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C3ADC: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 832C3AE0: 92A10098  stw r21, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[21].u32 ) };
	// 832C3AE4: 81410088  lwz r10, 0x88(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 832C3AE8: 8AA10052  lbz r21, 0x52(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 832C3AEC: 99010052  stb r8, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[8].u8 ) };
	// 832C3AF0: 8B490004  lbz r26, 4(r9)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3AF4: 9A8A0000  stb r20, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[20].u8 ) };
	// 832C3AF8: 9A4A0001  stb r18, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[18].u8 ) };
	// 832C3AFC: 9A2A0002  stb r17, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[17].u8 ) };
	// 832C3B00: 9A0A0003  stb r16, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[16].u8 ) };
	// 832C3B04: 99EA0004  stb r15, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[15].u8 ) };
	// 832C3B08: 99CA0005  stb r14, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[14].u8 ) };
	// 832C3B0C: 996A0006  stb r11, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[11].u8 ) };
	// 832C3B10: 9AAA0007  stb r21, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[21].u8 ) };
	// 832C3B14: 814100F0  lwz r10, 0xf0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 832C3B18: 7D0AFA14  add r8, r10, r31
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3B1C: 9B2A0000  stb r25, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 832C3B20: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 832C3B24: 8B210052  lbz r25, 0x52(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 832C3B28: 98E10072  stb r7, 0x72(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(114 as u32), ctx.r[7].u8 ) };
	// 832C3B2C: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C3B30: 9ACA0001  stb r22, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[22].u8 ) };
	// 832C3B34: 9AEA0005  stb r23, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[23].u8 ) };
	// 832C3B38: 9B0A0006  stb r24, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[24].u8 ) };
	// 832C3B3C: 9B2A0003  stb r25, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[25].u8 ) };
	// 832C3B40: 8B210072  lbz r25, 0x72(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(114 as u32) ) } as u64;
	// 832C3B44: 9B2A0004  stb r25, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[25].u8 ) };
	// 832C3B48: 8B210050  lbz r25, 0x50(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832C3B4C: 9A6A0002  stb r19, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[19].u8 ) };
	// 832C3B50: 7CCAF9AE  stbx r6, r10, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u8) };
	// 832C3B54: 9BC80001  stb r30, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 832C3B58: 8B090006  lbz r24, 6(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3B5C: 9B2A0007  stb r25, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[25].u8 ) };
	// 832C3B60: 81410098  lwz r10, 0x98(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 832C3B64: 8B290005  lbz r25, 5(r9)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C3B68: 8AE90007  lbz r23, 7(r9)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C3B6C: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3B70: 9B680003  stb r27, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[27].u8 ) };
	// 832C3B74: 7CC9FA14  add r6, r9, r31
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C3B78: 9BA80002  stb r29, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 832C3B7C: 8BCA0000  lbz r30, 0(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3B80: 8ACA0001  lbz r22, 1(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C3B84: 8A4A0004  lbz r18, 4(r10)
	ctx.r[18].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3B88: 8A2A0005  lbz r17, 5(r10)
	ctx.r[17].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C3B8C: 8A0A0006  lbz r16, 6(r10)
	ctx.r[16].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3B90: 8AAA0002  lbz r21, 2(r10)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C3B94: 8A6A0003  lbz r19, 3(r10)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C3B98: 89EA0007  lbz r15, 7(r10)
	ctx.r[15].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C3B9C: 7D47FA14  add r10, r7, r31
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C3BA0: 9B280005  stb r25, 5(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(5 as u32), ctx.r[25].u8 ) };
	// 832C3BA4: 7FC8F9AE  stbx r30, r8, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u8) };
	// 832C3BA8: 9B480004  stb r26, 4(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 832C3BAC: 9B080006  stb r24, 6(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(6 as u32), ctx.r[24].u8 ) };
	// 832C3BB0: 9AE80007  stb r23, 7(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(7 as u32), ctx.r[23].u8 ) };
	// 832C3BB4: 7D0AFA14  add r8, r10, r31
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3BB8: 8BC90005  lbz r30, 5(r9)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C3BBC: F9010088  std r8, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[8].u64 ) };
	// 832C3BC0: 89060002  lbz r8, 2(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C3BC4: 8B690006  lbz r27, 6(r9)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3BC8: 8B290007  lbz r25, 7(r9)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C3BCC: 83A100A8  lwz r29, 0xa8(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 832C3BD0: FA810090  std r20, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[20].u64 ) };
	// 832C3BD4: 99010074  stb r8, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u8 ) };
	// 832C3BD8: 89060003  lbz r8, 3(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C3BDC: 8B490000  lbz r26, 0(r9)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3BE0: 8B090001  lbz r24, 1(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C3BE4: 8AE90002  lbz r23, 2(r9)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C3BE8: 89C90003  lbz r14, 3(r9)
	ctx.r[14].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C3BEC: 99010071  stb r8, 0x71(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(113 as u32), ctx.r[8].u8 ) };
	// 832C3BF0: 89060004  lbz r8, 4(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3BF4: 89690004  lbz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C3BF8: 8A860001  lbz r20, 1(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C3BFC: 9AC70001  stb r22, 1(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(1 as u32), ctx.r[22].u8 ) };
	// 832C3C00: 9A470004  stb r18, 4(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[18].u8 ) };
	// 832C3C04: 99010073  stb r8, 0x73(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(115 as u32), ctx.r[8].u8 ) };
	// 832C3C08: 89060005  lbz r8, 5(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C3C0C: 9A270005  stb r17, 5(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(5 as u32), ctx.r[17].u8 ) };
	// 832C3C10: 9A070006  stb r16, 6(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(6 as u32), ctx.r[16].u8 ) };
	// 832C3C14: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 832C3C18: 9B610072  stb r27, 0x72(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(114 as u32), ctx.r[27].u8 ) };
	// 832C3C1C: 99010075  stb r8, 0x75(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(117 as u32), ctx.r[8].u8 ) };
	// 832C3C20: 89060006  lbz r8, 6(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C3C24: 9B210052  stb r25, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[25].u8 ) };
	// 832C3C28: 7D29F8AE  lbzx r9, r9, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 832C3C2C: 88C60007  lbz r6, 7(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C3C30: 83C10138  lwz r30, 0x138(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(312 as u32) ) } as u64;
	// 832C3C34: 83610200  lwz r27, 0x200(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(512 as u32) ) } as u64;
	// 832C3C38: 824101C0  lwz r18, 0x1c0(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(448 as u32) ) } as u64;
	// 832C3C3C: 82010100  lwz r16, 0x100(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 832C3C40: 82C10140  lwz r22, 0x140(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) } as u64;
	// 832C3C44: 83210180  lwz r25, 0x180(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 832C3C48: 8221007C  lwz r17, 0x7c(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 832C3C4C: 9AA70002  stb r21, 2(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(2 as u32), ctx.r[21].u8 ) };
	// 832C3C50: 9A670003  stb r19, 3(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(3 as u32), ctx.r[19].u8 ) };
	// 832C3C54: 99E70007  stb r15, 7(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(7 as u32), ctx.r[15].u8 ) };
	// 832C3C58: 98BD0000  stb r5, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 832C3C5C: 989D0001  stb r4, 1(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 832C3C60: 88A10076  lbz r5, 0x76(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(118 as u32) ) } as u64;
	// 832C3C64: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 832C3C68: 88810077  lbz r4, 0x77(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(119 as u32) ) } as u64;
	// 832C3C6C: 987D0002  stb r3, 2(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 832C3C70: 88610070  lbz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 832C3C74: 7D2AF9AE  stbx r9, r10, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u8) };
	// 832C3C78: 98BD0004  stb r5, 4(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 832C3C7C: 88A10051  lbz r5, 0x51(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 832C3C80: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 832C3C84: 989D0005  stb r4, 5(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(5 as u32), ctx.r[4].u8 ) };
	// 832C3C88: 88810050  lbz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832C3C8C: 987D0006  stb r3, 6(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[3].u8 ) };
	// 832C3C90: E9010088  ld r8, 0x88(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	// 832C3C94: 88610072  lbz r3, 0x72(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(114 as u32) ) } as u64;
	// 832C3C98: 98BD0007  stb r5, 7(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(7 as u32), ctx.r[5].u8 ) };
	// 832C3C9C: 988A0005  stb r4, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[4].u8 ) };
	// 832C3CA0: 88810074  lbz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 832C3CA4: 88A10052  lbz r5, 0x52(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 832C3CA8: 986A0006  stb r3, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[3].u8 ) };
	// 832C3CAC: 88610071  lbz r3, 0x71(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(113 as u32) ) } as u64;
	// 832C3CB0: 89210073  lbz r9, 0x73(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(115 as u32) ) } as u64;
	// 832C3CB4: 98880002  stb r4, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[4].u8 ) };
	// 832C3CB8: 98AA0007  stb r5, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[5].u8 ) };
	// 832C3CBC: 88A10075  lbz r5, 0x75(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(117 as u32) ) } as u64;
	// 832C3CC0: 7F47F9AE  stbx r26, r7, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32), ctx.r[26].u8) };
	// 832C3CC4: 9B0A0001  stb r24, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 832C3CC8: 9AEA0002  stb r23, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[23].u8 ) };
	// 832C3CCC: 996A0004  stb r11, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 832C3CD0: 88810051  lbz r4, 0x51(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 832C3CD4: 9A880001  stb r20, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[20].u8 ) };
	// 832C3CD8: 98680003  stb r3, 3(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(3 as u32), ctx.r[3].u8 ) };
	// 832C3CDC: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C3CE0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C3CE4: 98880006  stb r4, 6(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(6 as u32), ctx.r[4].u8 ) };
	// 832C3CE8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C3CEC: 82610080  lwz r19, 0x80(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C3CF0: 82E10084  lwz r23, 0x84(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 832C3CF4: 83010078  lwz r24, 0x78(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 832C3CF8: E96102B8  ld r11, 0x2b8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(696 as u32) ) };
	// 832C3CFC: EA810090  ld r20, 0x90(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	// 832C3D00: 9B9D0003  stb r28, 3(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(3 as u32), ctx.r[28].u8 ) };
	// 832C3D04: 99CA0003  stb r14, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[14].u8 ) };
	// 832C3D08: 99280004  stb r9, 4(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u8 ) };
	// 832C3D0C: 98A80005  stb r5, 5(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(5 as u32), ctx.r[5].u8 ) };
	// 832C3D10: 98C80007  stb r6, 7(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(7 as u32), ctx.r[6].u8 ) };
	// 832C3D14: 48000BFC  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
            }
            0x832C3D18 => {
    //   block [0x832C3D18..0x832C3EDC)
	// 832C3D18: 2B1A0004  cmplwi cr6, r26, 4
	ctx.cr[6].compare_u32(ctx.r[26].u32, 4 as u32, &mut ctx.xer);
	// 832C3D1C: 40980028  bge cr6, 0x832c3d44
	if !ctx.cr[6].lt {
	pc = 0x832C3D44; continue 'dispatch;
	}
	// 832C3D20: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3D24: 213A0004  subfic r9, r26, 4
	ctx.xer.ca = ctx.r[26].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[26].s64;
	// 832C3D28: 38DA001C  addi r6, r26, 0x1c
	ctx.r[6].s64 = ctx.r[26].s64 + 28;
	// 832C3D2C: 7D48D030  slw r8, r10, r26
	if (ctx.r[26].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[10].u32) << ((ctx.r[26].u8 & 0x1F) as u32)) as u64;
	}
	// 832C3D30: 7D072378  or r7, r8, r4
	ctx.r[7].u64 = ctx.r[8].u64 | ctx.r[4].u64;
	// 832C3D34: 7D454C30  srw r5, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C3D38: 54EA073E  clrlwi r10, r7, 0x1c
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	// 832C3D3C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 832C3D40: 48000010  b 0x832c3d50
	pc = 0x832C3D50; continue 'dispatch;
	// 832C3D44: 548A073E  clrlwi r10, r4, 0x1c
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000000Fu64;
	// 832C3D48: 5485E13E  srwi r5, r4, 4
	ctx.r[5].u32 = ctx.r[4].u32.wrapping_shr(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C3D4C: 38DAFFFC  addi r6, r26, -4
	ctx.r[6].s64 = ctx.r[26].s64 + -4;
	// 832C3D50: 812100B4  lwz r9, 0xb4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 832C3D54: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C3D58: 810100A4  lwz r8, 0xa4(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 832C3D5C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C3D60: 7CEA4214  add r7, r10, r8
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 832C3D64: 41980010  blt cr6, 0x832c3d74
	if ctx.cr[6].lt {
	pc = 0x832C3D74; continue 'dispatch;
	}
	// 832C3D68: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C3D6C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C3D70: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C3D74: 81410104  lwz r10, 0x104(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 832C3D78: 7F105040  cmplw cr6, r16, r10
	ctx.cr[6].compare_u32(ctx.r[16].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C3D7C: 4198000C  blt cr6, 0x832c3d88
	if ctx.cr[6].lt {
	pc = 0x832C3D88; continue 'dispatch;
	}
	// 832C3D80: 81410130  lwz r10, 0x130(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(304 as u32) ) } as u64;
	// 832C3D84: 3A0A0004  addi r16, r10, 4
	ctx.r[16].s64 = ctx.r[10].s64 + 4;
	// 832C3D88: 56EAF87E  srwi r10, r23, 1
	ctx.r[10].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C3D8C: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 832C3D90: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 832C3D94: 7E8A99AE  stbx r20, r10, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C3D98: 89500000  lbz r10, 0(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3D9C: 3A100001  addi r16, r16, 1
	ctx.r[16].s64 = ctx.r[16].s64 + 1;
	// 832C3DA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 832C3DA4: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 832C3DA8: 419900A0  bgt cr6, 0x832c3e48
	if ctx.cr[6].gt {
	pc = 0x832C3E48; continue 'dispatch;
	}
	// 832C3DAC: 7C8A2050  subf r4, r10, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 832C3DB0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 832C3DB4: 409A0018  bne cr6, 0x832c3dcc
	if !ctx.cr[6].eq {
	pc = 0x832C3DCC; continue 'dispatch;
	}
	// 832C3DB8: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3DBC: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 832C3DC0: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 832C3DC4: 5505F87E  srwi r5, r8, 1
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C3DC8: 48000010  b 0x832c3dd8
	pc = 0x832C3DD8; continue 'dispatch;
	// 832C3DCC: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 832C3DD0: 54A5F87E  srwi r5, r5, 1
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shr(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C3DD4: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 832C3DD8: 550807FE  clrlwi r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 832C3DDC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832C3DE0: 419A0034  beq cr6, 0x832c3e14
	if ctx.cr[6].eq {
	pc = 0x832C3E14; continue 'dispatch;
	}
	// 832C3DE4: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3DE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C3DEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C3DF0: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C3DF4: 419A004C  beq cr6, 0x832c3e40
	if ctx.cr[6].eq {
	pc = 0x832C3E40; continue 'dispatch;
	}
	// 832C3DF8: 7F8938AE  lbzx r28, r9, r7
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 832C3DFC: 3B4102C0  addi r26, r1, 0x2c0
	ctx.r[26].s64 = ctx.r[1].s64 + 704;
	// 832C3E00: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C3E04: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832C3E08: 7D1CD1AE  stbx r8, r28, r26
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32), ctx.r[8].u8) };
	// 832C3E0C: 4082FFEC  bne 0x832c3df8
	if !ctx.cr[0].eq {
	pc = 0x832C3DF8; continue 'dispatch;
	}
	// 832C3E10: 48000030  b 0x832c3e40
	pc = 0x832C3E40; continue 'dispatch;
	// 832C3E14: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C3E18: 419A0028  beq cr6, 0x832c3e40
	if ctx.cr[6].eq {
	pc = 0x832C3E40; continue 'dispatch;
	}
	// 832C3E1C: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3E20: 3B8102C0  addi r28, r1, 0x2c0
	ctx.r[28].s64 = ctx.r[1].s64 + 704;
	// 832C3E24: 7F4938AE  lbzx r26, r9, r7
	ctx.r[26].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 832C3E28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C3E2C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C3E30: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832C3E34: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C3E38: 7D1AE1AE  stbx r8, r26, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[28].u32), ctx.r[8].u8) };
	// 832C3E3C: 4082FFE0  bne 0x832c3e1c
	if !ctx.cr[0].eq {
	pc = 0x832C3E1C; continue 'dispatch;
	}
	// 832C3E40: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 832C3E44: 4199FF54  bgt cr6, 0x832c3d98
	if ctx.cr[6].gt {
	pc = 0x832C3D98; continue 'dispatch;
	}
	// 832C3E48: 92010100  stw r16, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[16].u32 ) };
	// 832C3E4C: 2F09003F  cmpwi cr6, r9, 0x3f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 63, &mut ctx.xer);
	// 832C3E50: 409A001C  bne cr6, 0x832c3e6c
	if !ctx.cr[6].eq {
	pc = 0x832C3E6C; continue 'dispatch;
	}
	// 832C3E54: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3E58: 392102C0  addi r9, r1, 0x2c0
	ctx.r[9].s64 = ctx.r[1].s64 + 704;
	// 832C3E5C: 8907003F  lbz r8, 0x3f(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(63 as u32) ) } as u64;
	// 832C3E60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C3E64: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C3E68: 7D4849AE  stbx r10, r8, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 832C3E6C: 7D5DFA14  add r10, r29, r31
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C3E70: C80102D0  lfd f0, 0x2d0(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(720 as u32) ) };
	// 832C3E74: C9A102C8  lfd f13, 0x2c8(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(712 as u32) ) };
	// 832C3E78: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 832C3E7C: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3E80: C98102D8  lfd f12, 0x2d8(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(728 as u32) ) };
	// 832C3E84: 7DBDFDAE  stfdx f13, r29, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.f[13].u64) };
	// 832C3E88: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 832C3E8C: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3E90: C96102E0  lfd f11, 0x2e0(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(736 as u32) ) };
	// 832C3E94: C94102E8  lfd f10, 0x2e8(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(744 as u32) ) };
	// 832C3E98: 90610064  stw r3, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 832C3E9C: C92102C0  lfd f9, 0x2c0(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(704 as u32) ) };
	// 832C3EA0: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 832C3EA4: D80A0000  stfd f0, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 832C3EA8: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C3EAC: C90102F0  lfd f8, 0x2f0(r1)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(752 as u32) ) };
	// 832C3EB0: 93410068  stw r26, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[26].u32 ) };
	// 832C3EB4: D9890000  stfd f12, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.f[12].u64 ) };
	// 832C3EB8: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C3EBC: C8E102F8  lfd f7, 0x2f8(r1)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(760 as u32) ) };
	// 832C3EC0: D93D0000  stfd f9, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.f[9].u64 ) };
	// 832C3EC4: D96A0000  stfd f11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[11].u64 ) };
	// 832C3EC8: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C3ECC: D9490000  stfd f10, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.f[10].u64 ) };
	// 832C3ED0: 7D09FDAE  stfdx f8, r9, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.f[8].u64) };
	// 832C3ED4: 7CEAFDAE  stfdx f7, r10, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.f[7].u64) };
	// 832C3ED8: 48000A38  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
            }
            0x832C3EDC => {
    //   block [0x832C3EDC..0x832C3FB0)
	// 832C3EDC: 56EAF87E  srwi r10, r23, 1
	ctx.r[10].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C3EE0: 7F194840  cmplw cr6, r25, r9
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C3EE4: 7E8A99AE  stbx r20, r10, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C3EE8: 41980010  blt cr6, 0x832c3ef8
	if ctx.cr[6].lt {
	pc = 0x832C3EF8; continue 'dispatch;
	}
	// 832C3EEC: 81410170  lwz r10, 0x170(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 832C3EF0: 3B2E0004  addi r25, r14, 4
	ctx.r[25].s64 = ctx.r[14].s64 + 4;
	// 832C3EF4: 3ACA0004  addi r22, r10, 4
	ctx.r[22].s64 = ctx.r[10].s64 + 4;
	// 832C3EF8: 89590000  lbz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3EFC: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 832C3F00: 89360000  lbz r9, 0(r22)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3F04: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 832C3F08: 7D480774  extsb r8, r10
	ctx.r[8].s64 = ctx.r[10].s8 as i64;
	// 832C3F0C: 80C105FC  lwz r6, 0x5fc(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1532 as u32) ) } as u64;
	// 832C3F10: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 832C3F14: 93210180  stw r25, 0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[25].u32 ) };
	// 832C3F18: 7D28F9D6  mullw r9, r8, r31
	ctx.r[9].s32 = ((ctx.r[8].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 832C3F1C: 92C10140  stw r22, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[22].u32 ) };
	// 832C3F20: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 832C3F24: 7CEAC214  add r7, r10, r24
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 832C3F28: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 832C3F2C: 419809E4  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C3F30: 814100E8  lwz r10, 0xe8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 832C3F34: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C3F38: 419909D8  bgt cr6, 0x832c4910
	if ctx.cr[6].gt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C3F3C: 2B1A0007  cmplwi cr6, r26, 7
	ctx.cr[6].compare_u32(ctx.r[26].u32, 7 as u32, &mut ctx.xer);
	// 832C3F40: 40980034  bge cr6, 0x832c3f74
	if !ctx.cr[6].lt {
	pc = 0x832C3F74; continue 'dispatch;
	}
	// 832C3F44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3F48: 215A0007  subfic r10, r26, 7
	ctx.xer.ca = ctx.r[26].u32 <= 7 as u32;
	ctx.r[10].s64 = (7 as i64) - ctx.r[26].s64;
	// 832C3F4C: 393A0019  addi r9, r26, 0x19
	ctx.r[9].s64 = ctx.r[26].s64 + 25;
	// 832C3F50: 7D68D030  slw r8, r11, r26
	if (ctx.r[26].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[26].u8 & 0x1F) as u32)) as u64;
	}
	// 832C3F54: 7D062378  or r6, r8, r4
	ctx.r[6].u64 = ctx.r[8].u64 | ctx.r[4].u64;
	// 832C3F58: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 832C3F5C: 7D655430  srw r5, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[11].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 832C3F60: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 832C3F64: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 832C3F68: 54C6067E  clrlwi r6, r6, 0x19
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x0000007Fu64;
	// 832C3F6C: 90810064  stw r4, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[4].u32 ) };
	// 832C3F70: 48000018  b 0x832c3f88
	pc = 0x832C3F88; continue 'dispatch;
	// 832C3F74: 548BC9FE  srwi r11, r4, 7
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C3F78: 395AFFF9  addi r10, r26, -7
	ctx.r[10].s64 = ctx.r[26].s64 + -7;
	// 832C3F7C: 5486067E  clrlwi r6, r4, 0x19
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000007Fu64;
	// 832C3F80: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 832C3F84: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 832C3F88: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 832C3F8C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 832C3F90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832C3F94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832C3F98: 48005A11  bl 0x832c99a8
	ctx.lr = 0x832C3F9C;
	sub_832C99A8(ctx, base);
	// 832C3F9C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C3FA0: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C3FA4: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 832C3FA8: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C3FAC: 48000964  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
            }
            0x832C3FB0 => {
    //   block [0x832C3FB0..0x832C406C)
	// 832C3FB0: 56EBF87E  srwi r11, r23, 1
	ctx.r[11].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C3FB4: 7F123040  cmplw cr6, r18, r6
	ctx.cr[6].compare_u32(ctx.r[18].u32, ctx.r[6].u32, &mut ctx.xer);
	// 832C3FB8: 7E8B99AE  stbx r20, r11, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C3FBC: 4198000C  blt cr6, 0x832c3fc8
	if ctx.cr[6].lt {
	pc = 0x832C3FC8; continue 'dispatch;
	}
	// 832C3FC0: 816101F0  lwz r11, 0x1f0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(496 as u32) ) } as u64;
	// 832C3FC4: 3A4B0004  addi r18, r11, 4
	ctx.r[18].s64 = ctx.r[11].s64 + 4;
	// 832C3FC8: A1720000  lhz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C3FCC: 3A520002  addi r18, r18, 2
	ctx.r[18].s64 = ctx.r[18].s64 + 2;
	// 832C3FD0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C3FD4: 38610340  addi r3, r1, 0x340
	ctx.r[3].s64 = ctx.r[1].s64 + 832;
	// 832C3FD8: 924101C0  stw r18, 0x1c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), ctx.r[18].u32 ) };
	// 832C3FDC: B1610340  sth r11, 0x340(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(832 as u32), ctx.r[11].u16 ) };
	// 832C3FE0: 480049C9  bl 0x832c89a8
	ctx.lr = 0x832C3FE4;
	sub_832C89A8(ctx, base);
	// 832C3FE4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C3FE8: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832C3FEC: 4098003C  bge cr6, 0x832c4028
	if !ctx.cr[6].lt {
	pc = 0x832C4028; continue 'dispatch;
	}
	// 832C3FF0: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C3FF4: 212B0004  subfic r9, r11, 4
	ctx.xer.ca = ctx.r[11].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[11].s64;
	// 832C3FF8: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C3FFC: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 832C4000: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 832C4004: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4008: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 832C400C: 7CA45830  slw r4, r5, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[5].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C4010: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 832C4014: 7CAB4C30  srw r11, r5, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[5].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C4018: 7C834378  or r3, r4, r8
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 832C401C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 832C4020: 546B073E  clrlwi r11, r3, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 832C4024: 4800001C  b 0x832c4040
	pc = 0x832C4040; continue 'dispatch;
	// 832C4028: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C402C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 832C4030: 5549E13E  srwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C4034: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832C4038: 554B073E  clrlwi r11, r10, 0x1c
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 832C403C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 832C4040: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4044: 38A10340  addi r5, r1, 0x340
	ctx.r[5].s64 = ctx.r[1].s64 + 832;
	// 832C4048: 7CCBF214  add r6, r11, r30
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C404C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832C4050: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832C4054: 48003F45  bl 0x832c7f98
	ctx.lr = 0x832C4058;
	sub_832C7F98(ctx, base);
	// 832C4058: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C405C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 832C4060: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C4064: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C4068: 480008A8  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
            }
            0x832C406C => {
    //   block [0x832C406C..0x832C40E8)
	// 832C406C: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 832C4070: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C4074: 41980010  blt cr6, 0x832c4084
	if ctx.cr[6].lt {
	pc = 0x832C4084; continue 'dispatch;
	}
	// 832C4078: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C407C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C4080: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C4084: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4088: 56E8F87E  srwi r8, r23, 1
	ctx.r[8].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C408C: 7D5DFA14  add r10, r29, r31
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C4090: 792747E4  rldicr r7, r9, 8, 0x3f
	ctx.r[7].u64 = (ctx.r[9].u64).rotate_left(8) & 0xFFFFFFFFFFFFFFFF;
	// 832C4094: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4098: 7CE64B78  or r6, r7, r9
	ctx.r[6].u64 = ctx.r[7].u64 | ctx.r[9].u64;
	// 832C409C: 7E8899AE  stbx r20, r8, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C40A0: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C40A4: 78C583E4  sldi r5, r6, 0x10
	ctx.r[5].u64 = ctx.r[6].u64.wrapping_shl(16);
	ctx.r[5].u32 = ctx.r[5].u64 as u32;
	// 832C40A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C40AC: 7CA83378  or r8, r5, r6
	ctx.r[8].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 832C40B0: 790707C6  sldi r7, r8, 0x20
	ctx.r[7].u64 = ctx.r[8].u64.wrapping_shl(32);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 832C40B4: 7CE64378  or r6, r7, r8
	ctx.r[6].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 832C40B8: F8CA0000  std r6, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 832C40BC: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C40C0: 7CDDF92A  stdx r6, r29, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u64) };
	// 832C40C4: F8C90000  std r6, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 832C40C8: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C40CC: F8DD0000  std r6, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 832C40D0: F8CA0000  std r6, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 832C40D4: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C40D8: F8C90000  std r6, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 832C40DC: 7CC9F92A  stdx r6, r9, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u64) };
	// 832C40E0: 7CCAF92A  stdx r6, r10, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u64) };
	// 832C40E4: 48000828  b 0x832c490c
	pc = 0x832C490C; continue 'dispatch;
            }
            0x832C40E8 => {
    //   block [0x832C40E8..0x832C4204)
	// 832C40E8: 56EAF87E  srwi r10, r23, 1
	ctx.r[10].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C40EC: 7F194840  cmplw cr6, r25, r9
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C40F0: 7E8A99AE  stbx r20, r10, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C40F4: 41980010  blt cr6, 0x832c4104
	if ctx.cr[6].lt {
	pc = 0x832C4104; continue 'dispatch;
	}
	// 832C40F8: 81410170  lwz r10, 0x170(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 832C40FC: 3B2E0004  addi r25, r14, 4
	ctx.r[25].s64 = ctx.r[14].s64 + 4;
	// 832C4100: 3ACA0004  addi r22, r10, 4
	ctx.r[22].s64 = ctx.r[10].s64 + 4;
	// 832C4104: 89590000  lbz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4108: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 832C410C: 89360000  lbz r9, 0(r22)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4110: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 832C4114: 7D480774  extsb r8, r10
	ctx.r[8].s64 = ctx.r[10].s8 as i64;
	// 832C4118: 80E105FC  lwz r7, 0x5fc(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1532 as u32) ) } as u64;
	// 832C411C: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 832C4120: 93210180  stw r25, 0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[25].u32 ) };
	// 832C4124: 7D28F9D6  mullw r9, r8, r31
	ctx.r[9].s32 = ((ctx.r[8].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 832C4128: 92C10140  stw r22, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[22].u32 ) };
	// 832C412C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 832C4130: 7F8AC214  add r28, r10, r24
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 832C4134: 7F1C3840  cmplw cr6, r28, r7
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C4138: 419807D8  blt cr6, 0x832c4910
	if ctx.cr[6].lt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C413C: 814100E8  lwz r10, 0xe8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 832C4140: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C4144: 419907CC  bgt cr6, 0x832c4910
	if ctx.cr[6].gt {
	pc = 0x832C4910; continue 'dispatch;
	}
	// 832C4148: 81410304  lwz r10, 0x304(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(772 as u32) ) } as u64;
	// 832C414C: 81610300  lwz r11, 0x300(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(768 as u32) ) } as u64;
	// 832C4150: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C4154: 4198000C  blt cr6, 0x832c4160
	if ctx.cr[6].lt {
	pc = 0x832C4160; continue 'dispatch;
	}
	// 832C4158: 81610330  lwz r11, 0x330(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(816 as u32) ) } as u64;
	// 832C415C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C4160: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4164: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 832C4168: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 832C416C: 38610340  addi r3, r1, 0x340
	ctx.r[3].s64 = ctx.r[1].s64 + 832;
	// 832C4170: 91210300  stw r9, 0x300(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(768 as u32), ctx.r[9].u32 ) };
	// 832C4174: B1410340  sth r10, 0x340(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(832 as u32), ctx.r[10].u16 ) };
	// 832C4178: 48004831  bl 0x832c89a8
	ctx.lr = 0x832C417C;
	sub_832C89A8(ctx, base);
	// 832C417C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C4180: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 832C4184: 4098003C  bge cr6, 0x832c41c0
	if !ctx.cr[6].lt {
	pc = 0x832C41C0; continue 'dispatch;
	}
	// 832C4188: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C418C: 212B0004  subfic r9, r11, 4
	ctx.xer.ca = ctx.r[11].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[11].s64;
	// 832C4190: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C4194: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 832C4198: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 832C419C: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C41A0: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 832C41A4: 7CA45830  slw r4, r5, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[5].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C41A8: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 832C41AC: 7C834378  or r3, r4, r8
	ctx.r[3].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 832C41B0: 7CAB4C30  srw r11, r5, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[5].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C41B4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 832C41B8: 5466073E  clrlwi r6, r3, 0x1c
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 832C41BC: 4800001C  b 0x832c41d8
	pc = 0x832C41D8; continue 'dispatch;
	// 832C41C0: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C41C4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 832C41C8: 5549E13E  srwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C41CC: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832C41D0: 5546073E  clrlwi r6, r10, 0x1c
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 832C41D4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 832C41D8: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 832C41DC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 832C41E0: 38A10340  addi r5, r1, 0x340
	ctx.r[5].s64 = ctx.r[1].s64 + 832;
	// 832C41E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832C41E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832C41EC: 48004445  bl 0x832c8630
	ctx.lr = 0x832C41F0;
	sub_832C8630(ctx, base);
	// 832C41F0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C41F4: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 832C41F8: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C41FC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C4200: 48000710  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
            }
            0x832C4204 => {
    //   block [0x832C4204..0x832C4538)
	// 832C4204: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 832C4208: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C420C: 41980010  blt cr6, 0x832c421c
	if ctx.cr[6].lt {
	pc = 0x832C421C; continue 'dispatch;
	}
	// 832C4210: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C4214: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C4218: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C421C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4220: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C4224: 56E9F87E  srwi r9, r23, 1
	ctx.r[9].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C4228: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C422C: 5546403E  rotlwi r6, r10, 8
	ctx.r[6].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 832C4230: 7D1DFA14  add r8, r29, r31
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C4234: 7CC55378  or r5, r6, r10
	ctx.r[5].u64 = ctx.r[6].u64 | ctx.r[10].u64;
	// 832C4238: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C423C: 5483403E  rotlwi r3, r4, 8
	ctx.r[3].u64 = ((ctx.r[4].u32).rotate_left(8)) as u64;
	// 832C4240: 7C662378  or r6, r3, r4
	ctx.r[6].u64 = ctx.r[3].u64 | ctx.r[4].u64;
	// 832C4244: 7E8999AE  stbx r20, r9, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C4248: 54A4801E  slwi r4, r5, 0x10
	ctx.r[4].u32 = ctx.r[5].u32.wrapping_shl(16);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 832C424C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832C4250: 54C3801E  slwi r3, r6, 0x10
	ctx.r[3].u32 = ctx.r[6].u32.wrapping_shl(16);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 832C4254: 7F1B3840  cmplw cr6, r27, r7
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C4258: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C425C: 7C8A2B78  or r10, r4, r5
	ctx.r[10].u64 = ctx.r[4].u64 | ctx.r[5].u64;
	// 832C4260: 7C693378  or r9, r3, r6
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[6].u64;
	// 832C4264: 41980010  blt cr6, 0x832c4274
	if ctx.cr[6].lt {
	pc = 0x832C4274; continue 'dispatch;
	}
	// 832C4268: 80E10230  lwz r7, 0x230(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(560 as u32) ) } as u64;
	// 832C426C: 3B670004  addi r27, r7, 4
	ctx.r[27].s64 = ctx.r[7].s64 + 4;
	// 832C4270: 93610200  stw r27, 0x200(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(512 as u32), ctx.r[27].u32 ) };
	// 832C4274: 88FB0001  lbz r7, 1(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C4278: 38DE2428  addi r6, r30, 0x2428
	ctx.r[6].s64 = ctx.r[30].s64 + 9256;
	// 832C427C: 389E2468  addi r4, r30, 0x2468
	ctx.r[4].s64 = ctx.r[30].s64 + 9320;
	// 832C4280: 88BB0002  lbz r5, 2(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C4284: 54E316BA  rlwinm r3, r7, 2, 0x1a, 0x1d
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 832C4288: 8B9B0000  lbz r28, 0(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C428C: 54B916BA  rlwinm r25, r5, 2, 0x1a, 0x1d
	ctx.r[25].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 832C4290: 8B5B0003  lbz r26, 3(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C4294: 54E7F0BA  rlwinm r7, r7, 0x1e, 2, 0x1d
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 832C4298: 8B1B0004  lbz r24, 4(r27)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C429C: 3AFE2428  addi r23, r30, 0x2428
	ctx.r[23].s64 = ctx.r[30].s64 + 9256;
	// 832C42A0: 8ADB0005  lbz r22, 5(r27)
	ctx.r[22].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C42A4: 3A9E2428  addi r20, r30, 0x2428
	ctx.r[20].s64 = ctx.r[30].s64 + 9256;
	// 832C42A8: 7CC3302E  lwzx r6, r3, r6
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 832C42AC: 3ABE2468  addi r21, r30, 0x2468
	ctx.r[21].s64 = ctx.r[30].s64 + 9320;
	// 832C42B0: 7C83202E  lwzx r4, r3, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 832C42B4: 54A3F0BA  rlwinm r3, r5, 0x1e, 2, 0x1d
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 832C42B8: 3A7E2468  addi r19, r30, 0x2468
	ctx.r[19].s64 = ctx.r[30].s64 + 9320;
	// 832C42BC: 38BE2428  addi r5, r30, 0x2428
	ctx.r[5].s64 = ctx.r[30].s64 + 9256;
	// 832C42C0: 7EE7B82E  lwzx r23, r7, r23
	ctx.r[23].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 832C42C4: 3A5E2468  addi r18, r30, 0x2468
	ctx.r[18].s64 = ctx.r[30].s64 + 9320;
	// 832C42C8: 7E99A02E  lwzx r20, r25, r20
	ctx.r[20].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 832C42CC: 7CE7A82E  lwzx r7, r7, r21
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 832C42D0: 579516BA  rlwinm r21, r28, 2, 0x1a, 0x1d
	ctx.r[21].u64 = ctx.r[28].u32 as u64 & 0x3FFFFFFFu64;
	// 832C42D4: 579CF0BA  rlwinm r28, r28, 0x1e, 2, 0x1d
	ctx.r[28].u64 = ctx.r[28].u32 as u64 & 0x00000003u64;
	// 832C42D8: 7F39982E  lwzx r25, r25, r19
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 832C42DC: 3A7E2428  addi r19, r30, 0x2428
	ctx.r[19].s64 = ctx.r[30].s64 + 9256;
	// 832C42E0: 7CA3282E  lwzx r5, r3, r5
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 832C42E4: 7CC64838  and r6, r6, r9
	ctx.r[6].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 832C42E8: 7C63902E  lwzx r3, r3, r18
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C42EC: 7C845038  and r4, r4, r10
	ctx.r[4].u64 = ctx.r[4].u64 & ctx.r[10].u64;
	// 832C42F0: 3A3E2468  addi r17, r30, 0x2468
	ctx.r[17].s64 = ctx.r[30].s64 + 9320;
	// 832C42F4: 3A1E2428  addi r16, r30, 0x2428
	ctx.r[16].s64 = ctx.r[30].s64 + 9256;
	// 832C42F8: 3A5E2468  addi r18, r30, 0x2468
	ctx.r[18].s64 = ctx.r[30].s64 + 9320;
	// 832C42FC: 7E75982E  lwzx r19, r21, r19
	ctx.r[19].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 832C4300: 7CC62378  or r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[4].u64;
	// 832C4304: 7F395038  and r25, r25, r10
	ctx.r[25].u64 = ctx.r[25].u64 & ctx.r[10].u64;
	// 832C4308: 7E944838  and r20, r20, r9
	ctx.r[20].u64 = ctx.r[20].u64 & ctx.r[9].u64;
	// 832C430C: 7EB5882E  lwzx r21, r21, r17
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 832C4310: 7E1C802E  lwzx r16, r28, r16
	ctx.r[16].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 832C4314: 7EF74838  and r23, r23, r9
	ctx.r[23].u64 = ctx.r[23].u64 & ctx.r[9].u64;
	// 832C4318: 90C80000  stw r6, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 832C431C: 7E86CB78  or r6, r20, r25
	ctx.r[6].u64 = ctx.r[20].u64 | ctx.r[25].u64;
	// 832C4320: 7F9C902E  lwzx r28, r28, r18
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C4324: 7CF15038  and r17, r7, r10
	ctx.r[17].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 832C4328: 575216BA  rlwinm r18, r26, 2, 0x1a, 0x1d
	ctx.r[18].u64 = ctx.r[26].u32 as u64 & 0x3FFFFFFFu64;
	// 832C432C: 7CC8F92E  stwx r6, r8, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u32) };
	// 832C4330: 39FE2428  addi r15, r30, 0x2428
	ctx.r[15].s64 = ctx.r[30].s64 + 9256;
	// 832C4334: 3B3E2468  addi r25, r30, 0x2468
	ctx.r[25].s64 = ctx.r[30].s64 + 9320;
	// 832C4338: 7EE48B78  or r4, r23, r17
	ctx.r[4].u64 = ctx.r[23].u64 | ctx.r[17].u64;
	// 832C433C: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C4340: 90880004  stw r4, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 832C4344: 7C635038  and r3, r3, r10
	ctx.r[3].u64 = ctx.r[3].u64 & ctx.r[10].u64;
	// 832C4348: 7EA65038  and r6, r21, r10
	ctx.r[6].u64 = ctx.r[21].u64 & ctx.r[10].u64;
	// 832C434C: 7C92782E  lwzx r4, r18, r15
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 832C4350: 7CA54838  and r5, r5, r9
	ctx.r[5].u64 = ctx.r[5].u64 & ctx.r[9].u64;
	// 832C4354: 7F32C82E  lwzx r25, r18, r25
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 832C4358: 575AF0BA  rlwinm r26, r26, 0x1e, 2, 0x1d
	ctx.r[26].u64 = ctx.r[26].u32 as u64 & 0x00000003u64;
	// 832C435C: 7E684838  and r8, r19, r9
	ctx.r[8].u64 = ctx.r[19].u64 & ctx.r[9].u64;
	// 832C4360: 7E174838  and r23, r16, r9
	ctx.r[23].u64 = ctx.r[16].u64 & ctx.r[9].u64;
	// 832C4364: 7F9C5038  and r28, r28, r10
	ctx.r[28].u64 = ctx.r[28].u64 & ctx.r[10].u64;
	// 832C4368: 3ABE2428  addi r21, r30, 0x2428
	ctx.r[21].s64 = ctx.r[30].s64 + 9256;
	// 832C436C: 3A9E2468  addi r20, r30, 0x2468
	ctx.r[20].s64 = ctx.r[30].s64 + 9320;
	// 832C4370: 571216BA  rlwinm r18, r24, 2, 0x1a, 0x1d
	ctx.r[18].u64 = ctx.r[24].u32 as u64 & 0x3FFFFFFFu64;
	// 832C4374: 3A7E2428  addi r19, r30, 0x2428
	ctx.r[19].s64 = ctx.r[30].s64 + 9256;
	// 832C4378: 7CA51B78  or r5, r5, r3
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[3].u64;
	// 832C437C: 7D033378  or r3, r8, r6
	ctx.r[3].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 832C4380: 7CDAA82E  lwzx r6, r26, r21
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 832C4384: 7EFCE378  or r28, r23, r28
	ctx.r[28].u64 = ctx.r[23].u64 | ctx.r[28].u64;
	// 832C4388: 90A70004  stw r5, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 832C438C: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C4390: 7CBAA02E  lwzx r5, r26, r20
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 832C4394: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 832C4398: 7C844838  and r4, r4, r9
	ctx.r[4].u64 = ctx.r[4].u64 & ctx.r[9].u64;
	// 832C439C: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832C43A0: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C43A4: 7F52982E  lwzx r26, r18, r19
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 832C43A8: 387E2468  addi r3, r30, 0x2468
	ctx.r[3].s64 = ctx.r[30].s64 + 9320;
	// 832C43AC: 3B9E2428  addi r28, r30, 0x2428
	ctx.r[28].s64 = ctx.r[30].s64 + 9256;
	// 832C43B0: 3AFE2468  addi r23, r30, 0x2468
	ctx.r[23].s64 = ctx.r[30].s64 + 9320;
	// 832C43B4: 5718F0BA  rlwinm r24, r24, 0x1e, 2, 0x1d
	ctx.r[24].u64 = ctx.r[24].u32 as u64 & 0x00000003u64;
	// 832C43B8: 7F98E02E  lwzx r28, r24, r28
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 832C43BC: 56D516BA  rlwinm r21, r22, 2, 0x1a, 0x1d
	ctx.r[21].u64 = ctx.r[22].u32 as u64 & 0x3FFFFFFFu64;
	// 832C43C0: 7C72182E  lwzx r3, r18, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 832C43C4: 7CC64838  and r6, r6, r9
	ctx.r[6].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 832C43C8: 7F18B82E  lwzx r24, r24, r23
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 832C43CC: 7CA55038  and r5, r5, r10
	ctx.r[5].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	// 832C43D0: 56D6F0BA  rlwinm r22, r22, 0x1e, 2, 0x1d
	ctx.r[22].u64 = ctx.r[22].u32 as u64 & 0x00000003u64;
	// 832C43D4: 8A7B0006  lbz r19, 6(r27)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C43D8: 7F395038  and r25, r25, r10
	ctx.r[25].u64 = ctx.r[25].u64 & ctx.r[10].u64;
	// 832C43DC: 8B7B0007  lbz r27, 7(r27)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C43E0: 3A5E2428  addi r18, r30, 0x2428
	ctx.r[18].s64 = ctx.r[30].s64 + 9256;
	// 832C43E4: 82010200  lwz r16, 0x200(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(512 as u32) ) } as u64;
	// 832C43E8: 3A3E2468  addi r17, r30, 0x2468
	ctx.r[17].s64 = ctx.r[30].s64 + 9320;
	// 832C43EC: 3A9E2428  addi r20, r30, 0x2428
	ctx.r[20].s64 = ctx.r[30].s64 + 9256;
	// 832C43F0: 3AFE2468  addi r23, r30, 0x2468
	ctx.r[23].s64 = ctx.r[30].s64 + 9320;
	// 832C43F4: 7C84CB78  or r4, r4, r25
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[25].u64;
	// 832C43F8: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 832C43FC: 7F36902E  lwzx r25, r22, r18
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C4400: 90880000  stw r4, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 832C4404: 7C645038  and r4, r3, r10
	ctx.r[4].u64 = ctx.r[3].u64 & ctx.r[10].u64;
	// 832C4408: 90C80004  stw r6, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C440C: 566816BA  rlwinm r8, r19, 2, 0x1a, 0x1d
	ctx.r[8].u64 = ctx.r[19].u32 as u64 & 0x3FFFFFFFu64;
	// 832C4410: 7CB6882E  lwzx r5, r22, r17
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 832C4414: 3ADE2428  addi r22, r30, 0x2428
	ctx.r[22].s64 = ctx.r[30].s64 + 9256;
	// 832C4418: 7E95A02E  lwzx r20, r21, r20
	ctx.r[20].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 832C441C: 7F834838  and r3, r28, r9
	ctx.r[3].u64 = ctx.r[28].u64 & ctx.r[9].u64;
	// 832C4420: 7EF5B82E  lwzx r23, r21, r23
	ctx.r[23].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 832C4424: 3ABE2468  addi r21, r30, 0x2468
	ctx.r[21].s64 = ctx.r[30].s64 + 9320;
	// 832C4428: 7F5A4838  and r26, r26, r9
	ctx.r[26].u64 = ctx.r[26].u64 & ctx.r[9].u64;
	// 832C442C: 7F065038  and r6, r24, r10
	ctx.r[6].u64 = ctx.r[24].u64 & ctx.r[10].u64;
	// 832C4430: 7EC8B02E  lwzx r22, r8, r22
	ctx.r[22].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 832C4434: 577816BA  rlwinm r24, r27, 2, 0x1a, 0x1d
	ctx.r[24].u64 = ctx.r[27].u32 as u64 & 0x3FFFFFFFu64;
	// 832C4438: 3A3E2428  addi r17, r30, 0x2428
	ctx.r[17].s64 = ctx.r[30].s64 + 9256;
	// 832C443C: 7EA8A82E  lwzx r21, r8, r21
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 832C4440: 391E2468  addi r8, r30, 0x2468
	ctx.r[8].s64 = ctx.r[30].s64 + 9320;
	// 832C4444: 39FE2468  addi r15, r30, 0x2468
	ctx.r[15].s64 = ctx.r[30].s64 + 9320;
	// 832C4448: 91010088  stw r8, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[8].u32 ) };
	// 832C444C: 7F442378  or r4, r26, r4
	ctx.r[4].u64 = ctx.r[26].u64 | ctx.r[4].u64;
	// 832C4450: 7C633378  or r3, r3, r6
	ctx.r[3].u64 = ctx.r[3].u64 | ctx.r[6].u64;
	// 832C4454: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 832C4458: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C445C: 90670004  stw r3, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 832C4460: 567CF0BA  rlwinm r28, r19, 0x1e, 2, 0x1d
	ctx.r[28].u64 = ctx.r[19].u32 as u64 & 0x00000003u64;
	// 832C4464: 7CA75038  and r7, r5, r10
	ctx.r[7].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	// 832C4468: 7F58882E  lwzx r26, r24, r17
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 832C446C: 3A7E2428  addi r19, r30, 0x2428
	ctx.r[19].s64 = ctx.r[30].s64 + 9256;
	// 832C4470: 7CD8782E  lwzx r6, r24, r15
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 832C4474: 577BF0BA  rlwinm r27, r27, 0x1e, 2, 0x1d
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0x00000003u64;
	// 832C4478: 3A5E2468  addi r18, r30, 0x2468
	ctx.r[18].s64 = ctx.r[30].s64 + 9320;
	// 832C447C: 39DE2428  addi r14, r30, 0x2428
	ctx.r[14].s64 = ctx.r[30].s64 + 9256;
	// 832C4480: 7F234838  and r3, r25, r9
	ctx.r[3].u64 = ctx.r[25].u64 & ctx.r[9].u64;
	// 832C4484: 7E984838  and r24, r20, r9
	ctx.r[24].u64 = ctx.r[20].u64 & ctx.r[9].u64;
	// 832C4488: 7E7C982E  lwzx r19, r28, r19
	ctx.r[19].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 832C448C: 7EE45038  and r4, r23, r10
	ctx.r[4].u64 = ctx.r[23].u64 & ctx.r[10].u64;
	// 832C4490: 7C633B78  or r3, r3, r7
	ctx.r[3].u64 = ctx.r[3].u64 | ctx.r[7].u64;
	// 832C4494: 7F9C902E  lwzx r28, r28, r18
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 832C4498: 7F042378  or r4, r24, r4
	ctx.r[4].u64 = ctx.r[24].u64 | ctx.r[4].u64;
	// 832C449C: 7F3B702E  lwzx r25, r27, r14
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 832C44A0: 7CE8FA14  add r7, r8, r31
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C44A4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 832C44A8: 90880000  stw r4, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 832C44AC: 7EC44838  and r4, r22, r9
	ctx.r[4].u64 = ctx.r[22].u64 & ctx.r[9].u64;
	// 832C44B0: 7EA35038  and r3, r21, r10
	ctx.r[3].u64 = ctx.r[21].u64 & ctx.r[10].u64;
	// 832C44B4: 7F5A4838  and r26, r26, r9
	ctx.r[26].u64 = ctx.r[26].u64 & ctx.r[9].u64;
	// 832C44B8: 7F9C5038  and r28, r28, r10
	ctx.r[28].u64 = ctx.r[28].u64 & ctx.r[10].u64;
	// 832C44BC: 7D07FA14  add r8, r7, r31
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C44C0: 80A10088  lwz r5, 0x88(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 832C44C4: 7CC65038  and r6, r6, r10
	ctx.r[6].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 832C44C8: 7C841B78  or r4, r4, r3
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[3].u64;
	// 832C44CC: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 832C44D0: 7CBB282E  lwzx r5, r27, r5
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 832C44D4: 7E7B4838  and r27, r19, r9
	ctx.r[27].u64 = ctx.r[19].u64 & ctx.r[9].u64;
	// 832C44D8: 7F294838  and r9, r25, r9
	ctx.r[9].u64 = ctx.r[25].u64 & ctx.r[9].u64;
	// 832C44DC: 7CA55038  and r5, r5, r10
	ctx.r[5].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	// 832C44E0: 7F63E378  or r3, r27, r28
	ctx.r[3].u64 = ctx.r[27].u64 | ctx.r[28].u64;
	// 832C44E4: 7F4A3378  or r10, r26, r6
	ctx.r[10].u64 = ctx.r[26].u64 | ctx.r[6].u64;
	// 832C44E8: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 832C44EC: 90670004  stw r3, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 832C44F0: 3B700008  addi r27, r16, 8
	ctx.r[27].s64 = ctx.r[16].s64 + 8;
	// 832C44F4: 7D47F92E  stwx r10, r7, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 832C44F8: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832C44FC: 824101C0  lwz r18, 0x1c0(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(448 as u32) ) } as u64;
	// 832C4500: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 832C4504: 82010100  lwz r16, 0x100(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 832C4508: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 832C450C: 82C10140  lwz r22, 0x140(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) } as u64;
	// 832C4510: 83210180  lwz r25, 0x180(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 832C4514: 8221007C  lwz r17, 0x7c(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 832C4518: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C451C: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C4520: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C4524: 82610080  lwz r19, 0x80(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C4528: 82E10084  lwz r23, 0x84(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 832C452C: 83010078  lwz r24, 0x78(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 832C4530: 93610200  stw r27, 0x200(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(512 as u32), ctx.r[27].u32 ) };
	// 832C4534: 480003DC  b 0x832c4910
	pc = 0x832C4910; continue 'dispatch;
            }
            0x832C4538 => {
    //   block [0x832C4538..0x832C4910)
	// 832C4538: 56EAF87E  srwi r10, r23, 1
	ctx.r[10].u32 = ctx.r[23].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C453C: 812100B4  lwz r9, 0xb4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 832C4540: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832C4544: 7E8A99AE  stbx r20, r10, r19
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[19].u32), ctx.r[20].u8) };
	// 832C4548: 41980010  blt cr6, 0x832c4558
	if ctx.cr[6].lt {
	pc = 0x832C4558; continue 'dispatch;
	}
	// 832C454C: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C4550: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C4554: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C4558: 7D69EB78  or r9, r11, r29
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[29].u64;
	// 832C455C: 7D5DFA14  add r10, r29, r31
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C4560: 5528077E  clrlwi r8, r9, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 832C4564: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832C4568: 409A005C  bne cr6, 0x832c45c4
	if !ctx.cr[6].eq {
	pc = 0x832C45C4; continue 'dispatch;
	}
	// 832C456C: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4570: C80B0008  lfd f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 832C4574: D80A0000  stfd f0, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 832C4578: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C457C: C9AB0010  lfd f13, 0x10(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 832C4580: C98B0018  lfd f12, 0x18(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 832C4584: C96B0020  lfd f11, 0x20(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 832C4588: D9A90000  stfd f13, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.f[13].u64 ) };
	// 832C458C: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4590: C94B0028  lfd f10, 0x28(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 832C4594: D98A0000  stfd f12, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[12].u64 ) };
	// 832C4598: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C459C: C92B0000  lfd f9, 0(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 832C45A0: C90B0030  lfd f8, 0x30(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 832C45A4: D9690000  stfd f11, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.f[11].u64 ) };
	// 832C45A8: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C45AC: C8EB0038  lfd f7, 0x38(r11)
	ctx.f[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 832C45B0: D94A0000  stfd f10, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.f[10].u64 ) };
	// 832C45B4: D93D0000  stfd f9, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.f[9].u64 ) };
	// 832C45B8: 7D0AFDAE  stfdx f8, r10, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.f[8].u64) };
	// 832C45BC: 7CE9FDAE  stfdx f7, r9, r31
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.f[7].u64) };
	// 832C45C0: 48000348  b 0x832c4908
	pc = 0x832C4908; continue 'dispatch;
	// 832C45C4: 552907BE  clrlwi r9, r9, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 832C45C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C45CC: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C45D0: 409A009C  bne cr6, 0x832c466c
	if !ctx.cr[6].eq {
	pc = 0x832C466C; continue 'dispatch;
	}
	// 832C45D4: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C45D8: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832C45DC: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C45E0: 80AB0014  lwz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 832C45E4: 838B0018  lwz r28, 0x18(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 832C45E8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 832C45EC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 832C45F0: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C45F4: 810B001C  lwz r8, 0x1c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 832C45F8: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 832C45FC: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 832C4600: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4604: 80EB0020  lwz r7, 0x20(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C4608: 80CB0024  lwz r6, 0x24(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 832C460C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 832C4610: 938A0000  stw r28, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832C4614: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C4618: 80AB0028  lwz r5, 0x28(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 832C461C: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 832C4620: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 832C4624: 90C90004  stw r6, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C4628: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C462C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4630: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 832C4634: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 832C4638: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C463C: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C4640: 80AB0030  lwz r5, 0x30(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C4644: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 832C4648: 838B0038  lwz r28, 0x38(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 832C464C: 81EB003C  lwz r15, 0x3c(r11)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 832C4650: 90FD0000  stw r7, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 832C4654: 90DD0004  stw r6, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C4658: 90A90000  stw r5, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 832C465C: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 832C4660: 7F89F92E  stwx r28, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u32) };
	// 832C4664: 91EA0004  stw r15, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[15].u32 ) };
	// 832C4668: 480002A0  b 0x832c4908
	pc = 0x832C4908; continue 'dispatch;
	// 832C466C: 890B0008  lbz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C4670: 88EB0009  lbz r7, 9(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 832C4674: 88CB000A  lbz r6, 0xa(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 832C4678: 88AB000B  lbz r5, 0xb(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 832C467C: 888B000C  lbz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 832C4680: 886B000D  lbz r3, 0xd(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(13 as u32) ) } as u64;
	// 832C4684: 8B8B000E  lbz r28, 0xe(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(14 as u32) ) } as u64;
	// 832C4688: 8B6B000F  lbz r27, 0xf(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(15 as u32) ) } as u64;
	// 832C468C: 98EA0001  stb r7, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 832C4690: 98CA0002  stb r6, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 832C4694: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 832C4698: 98AA0003  stb r5, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 832C469C: 988A0004  stb r4, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u8 ) };
	// 832C46A0: 986A0005  stb r3, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[3].u8 ) };
	// 832C46A4: 9B8A0006  stb r28, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[28].u8 ) };
	// 832C46A8: 9B6A0007  stb r27, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[27].u8 ) };
	// 832C46AC: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C46B0: 88CB001A  lbz r6, 0x1a(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 832C46B4: 7CEAFA14  add r7, r10, r31
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C46B8: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 832C46BC: 88AB001B  lbz r5, 0x1b(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(27 as u32) ) } as u64;
	// 832C46C0: 90E10088  stw r7, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[7].u32 ) };
	// 832C46C4: 888B001C  lbz r4, 0x1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 832C46C8: 886B001D  lbz r3, 0x1d(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 832C46CC: 88EB001E  lbz r7, 0x1e(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(30 as u32) ) } as u64;
	// 832C46D0: 8B8B001F  lbz r28, 0x1f(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(31 as u32) ) } as u64;
	// 832C46D4: 98CA0002  stb r6, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 832C46D8: 8B6B0010  lbz r27, 0x10(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C46DC: 8B4B0011  lbz r26, 0x11(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 832C46E0: 8B2B0012  lbz r25, 0x12(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 832C46E4: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C46E8: 8B0B0013  lbz r24, 0x13(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19 as u32) ) } as u64;
	// 832C46EC: 8AEB0014  lbz r23, 0x14(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 832C46F0: 8ACB0015  lbz r22, 0x15(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 832C46F4: 8AAB0016  lbz r21, 0x16(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(22 as u32) ) } as u64;
	// 832C46F8: 8A8B0017  lbz r20, 0x17(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(23 as u32) ) } as u64;
	// 832C46FC: 8A6B0018  lbz r19, 0x18(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 832C4700: 990A0001  stb r8, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 832C4704: 98AA0003  stb r5, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 832C4708: 988A0004  stb r4, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u8 ) };
	// 832C470C: 986A0005  stb r3, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[3].u8 ) };
	// 832C4710: 98EA0006  stb r7, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[7].u8 ) };
	// 832C4714: 9B8A0007  stb r28, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[28].u8 ) };
	// 832C4718: 81410088  lwz r10, 0x88(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 832C471C: 88AB0001  lbz r5, 1(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C4720: 888B0002  lbz r4, 2(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C4724: 886B0003  lbz r3, 3(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C4728: 890B0004  lbz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C472C: 88EB0005  lbz r7, 5(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C4730: 8B8B0006  lbz r28, 6(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C4734: 8A4B0007  lbz r18, 7(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 832C4738: 8A2B0020  lbz r17, 0x20(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C473C: 8A0B0021  lbz r16, 0x21(r11)
	ctx.r[16].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 832C4740: 89EB0022  lbz r15, 0x22(r11)
	ctx.r[15].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(34 as u32) ) } as u64;
	// 832C4744: 89CB0023  lbz r14, 0x23(r11)
	ctx.r[14].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(35 as u32) ) } as u64;
	// 832C4748: 9B690000  stb r27, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 832C474C: 9B490001  stb r26, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[26].u8 ) };
	// 832C4750: 9B290002  stb r25, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[25].u8 ) };
	// 832C4754: 9B090003  stb r24, 3(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(3 as u32), ctx.r[24].u8 ) };
	// 832C4758: 9AE90004  stb r23, 4(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[23].u8 ) };
	// 832C475C: 9AC90005  stb r22, 5(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(5 as u32), ctx.r[22].u8 ) };
	// 832C4760: 9AA90006  stb r21, 6(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(6 as u32), ctx.r[21].u8 ) };
	// 832C4764: 9A890007  stb r20, 7(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(7 as u32), ctx.r[20].u8 ) };
	// 832C4768: 7E69F9AE  stbx r19, r9, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[19].u8) };
	// 832C476C: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4770: 98DD0000  stb r6, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 832C4774: 8B6B0024  lbz r27, 0x24(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 832C4778: 8B4B0025  lbz r26, 0x25(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 832C477C: 8B2B0026  lbz r25, 0x26(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(38 as u32) ) } as u64;
	// 832C4780: 88CB0027  lbz r6, 0x27(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(39 as u32) ) } as u64;
	// 832C4784: 98BD0001  stb r5, 1(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(1 as u32), ctx.r[5].u8 ) };
	// 832C4788: 989D0002  stb r4, 2(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(2 as u32), ctx.r[4].u8 ) };
	// 832C478C: 987D0003  stb r3, 3(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(3 as u32), ctx.r[3].u8 ) };
	// 832C4790: 991D0004  stb r8, 4(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[8].u8 ) };
	// 832C4794: 98FD0005  stb r7, 5(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(5 as u32), ctx.r[7].u8 ) };
	// 832C4798: 9B9D0006  stb r28, 6(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[28].u8 ) };
	// 832C479C: 9A5D0007  stb r18, 7(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(7 as u32), ctx.r[18].u8 ) };
	// 832C47A0: 9A2A0000  stb r17, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[17].u8 ) };
	// 832C47A4: 9A0A0001  stb r16, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[16].u8 ) };
	// 832C47A8: 99EA0002  stb r15, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[15].u8 ) };
	// 832C47AC: 99CA0003  stb r14, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[14].u8 ) };
	// 832C47B0: 888B0029  lbz r4, 0x29(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 832C47B4: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 832C47B8: 886B002A  lbz r3, 0x2a(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(42 as u32) ) } as u64;
	// 832C47BC: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 832C47C0: 8B0B002E  lbz r24, 0x2e(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(46 as u32) ) } as u64;
	// 832C47C4: 9B6A0004  stb r27, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[27].u8 ) };
	// 832C47C8: 9B4A0005  stb r26, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[26].u8 ) };
	// 832C47CC: 98890001  stb r4, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 832C47D0: 98690002  stb r3, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[3].u8 ) };
	// 832C47D4: 888B0037  lbz r4, 0x37(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(55 as u32) ) } as u64;
	// 832C47D8: 886B0038  lbz r3, 0x38(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 832C47DC: 9B2A0006  stb r25, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[25].u8 ) };
	// 832C47E0: 9B090006  stb r24, 6(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(6 as u32), ctx.r[24].u8 ) };
	// 832C47E4: 8AEB002F  lbz r23, 0x2f(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(47 as u32) ) } as u64;
	// 832C47E8: 8B6B0039  lbz r27, 0x39(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 832C47EC: 98610073  stb r3, 0x73(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(115 as u32), ctx.r[3].u8 ) };
	// 832C47F0: 98810075  stb r4, 0x75(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(117 as u32), ctx.r[4].u8 ) };
	// 832C47F4: 8B4B003A  lbz r26, 0x3a(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(58 as u32) ) } as u64;
	// 832C47F8: 8B2B003B  lbz r25, 0x3b(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(59 as u32) ) } as u64;
	// 832C47FC: 8B0B003C  lbz r24, 0x3c(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 832C4800: 9AE90007  stb r23, 7(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(7 as u32), ctx.r[23].u8 ) };
	// 832C4804: 9B610071  stb r27, 0x71(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(113 as u32), ctx.r[27].u8 ) };
	// 832C4808: 88AB0028  lbz r5, 0x28(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 832C480C: 8AEB003D  lbz r23, 0x3d(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(61 as u32) ) } as u64;
	// 832C4810: 9B010070  stb r24, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[24].u8 ) };
	// 832C4814: 9B410074  stb r26, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[26].u8 ) };
	// 832C4818: 9B210051  stb r25, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[25].u8 ) };
	// 832C481C: 98CA0007  stb r6, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[6].u8 ) };
	// 832C4820: 7D49FA14  add r10, r9, r31
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C4824: 98A90000  stb r5, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 832C4828: 890B002B  lbz r8, 0x2b(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(43 as u32) ) } as u64;
	// 832C482C: 9AE10077  stb r23, 0x77(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(119 as u32), ctx.r[23].u8 ) };
	// 832C4830: 88EB002C  lbz r7, 0x2c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 832C4834: 88CB0030  lbz r6, 0x30(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C4838: 88AB0031  lbz r5, 0x31(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(49 as u32) ) } as u64;
	// 832C483C: 99090003  stb r8, 3(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(3 as u32), ctx.r[8].u8 ) };
	// 832C4840: 8B8B002D  lbz r28, 0x2d(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 832C4844: 98E90004  stb r7, 4(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u8 ) };
	// 832C4848: 890B0032  lbz r8, 0x32(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(50 as u32) ) } as u64;
	// 832C484C: 98CA0000  stb r6, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 832C4850: 98AA0001  stb r5, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[5].u8 ) };
	// 832C4854: 88EB0033  lbz r7, 0x33(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(51 as u32) ) } as u64;
	// 832C4858: 88C10075  lbz r6, 0x75(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(117 as u32) ) } as u64;
	// 832C485C: 88A10073  lbz r5, 0x73(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(115 as u32) ) } as u64;
	// 832C4860: 9B890005  stb r28, 5(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(5 as u32), ctx.r[28].u8 ) };
	// 832C4864: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4868: 990A0002  stb r8, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[8].u8 ) };
	// 832C486C: 8ACB003E  lbz r22, 0x3e(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(62 as u32) ) } as u64;
	// 832C4870: 8A6B003F  lbz r19, 0x3f(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(63 as u32) ) } as u64;
	// 832C4874: 89010071  lbz r8, 0x71(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(113 as u32) ) } as u64;
	// 832C4878: 98EA0003  stb r7, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 832C487C: 98CA0007  stb r6, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[6].u8 ) };
	// 832C4880: 7CAAF9AE  stbx r5, r10, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u8) };
	// 832C4884: 8B8B0034  lbz r28, 0x34(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 832C4888: 89EB0035  lbz r15, 0x35(r11)
	ctx.r[15].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(53 as u32) ) } as u64;
	// 832C488C: 89CB0036  lbz r14, 0x36(r11)
	ctx.r[14].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(54 as u32) ) } as u64;
	// 832C4890: 88E10074  lbz r7, 0x74(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 832C4894: 88C10051  lbz r6, 0x51(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 832C4898: 88A10070  lbz r5, 0x70(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 832C489C: 9A610050  stb r19, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[19].u8 ) };
	// 832C48A0: 9AC10076  stb r22, 0x76(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[22].u8 ) };
	// 832C48A4: 99090001  stb r8, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 832C48A8: 83010078  lwz r24, 0x78(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 832C48AC: 82E10084  lwz r23, 0x84(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 832C48B0: 82610080  lwz r19, 0x80(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C48B4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C48B8: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 832C48BC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C48C0: 8221007C  lwz r17, 0x7c(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 832C48C4: 83210180  lwz r25, 0x180(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 832C48C8: 82C10140  lwz r22, 0x140(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) } as u64;
	// 832C48CC: 82010100  lwz r16, 0x100(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 832C48D0: 824101C0  lwz r18, 0x1c0(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(448 as u32) ) } as u64;
	// 832C48D4: 83610200  lwz r27, 0x200(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(512 as u32) ) } as u64;
	// 832C48D8: 89010077  lbz r8, 0x77(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(119 as u32) ) } as u64;
	// 832C48DC: 9B8A0004  stb r28, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[28].u8 ) };
	// 832C48E0: 99EA0005  stb r15, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[15].u8 ) };
	// 832C48E4: 99CA0006  stb r14, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[14].u8 ) };
	// 832C48E8: 98E90002  stb r7, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[7].u8 ) };
	// 832C48EC: 98C90003  stb r6, 3(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(3 as u32), ctx.r[6].u8 ) };
	// 832C48F0: 98A90004  stb r5, 4(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 832C48F4: 88E10076  lbz r7, 0x76(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(118 as u32) ) } as u64;
	// 832C48F8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832C48FC: 99090005  stb r8, 5(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(5 as u32), ctx.r[8].u8 ) };
	// 832C4900: 98E90006  stb r7, 6(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(6 as u32), ctx.r[7].u8 ) };
	// 832C4904: 98C90007  stb r6, 7(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(7 as u32), ctx.r[6].u8 ) };
	// 832C4908: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 832C490C: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
            }
            0x832C4910 => {
    //   block [0x832C4910..0x832C49A8)
	// 832C4910: 814100A0  lwz r10, 0xa0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 832C4914: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 832C4918: 81210644  lwz r9, 0x644(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1604 as u32) ) } as u64;
	// 832C491C: 3B180008  addi r24, r24, 8
	ctx.r[24].s64 = ctx.r[24].s64 + 8;
	// 832C4920: 38AAFFF8  addi r5, r10, -8
	ctx.r[5].s64 = ctx.r[10].s64 + -8;
	// 832C4924: 7EF74A14  add r23, r23, r9
	ctx.r[23].u64 = ctx.r[23].u64 + ctx.r[9].u64;
	// 832C4928: 8381063C  lwz r28, 0x63c(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1596 as u32) ) } as u64;
	// 832C492C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 832C4930: 92E10084  stw r23, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[23].u32 ) };
	// 832C4934: 90A100A0  stw r5, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[5].u32 ) };
	// 832C4938: 93010078  stw r24, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[24].u32 ) };
	// 832C493C: 93A100A8  stw r29, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 832C4940: 409AE16C  bne cr6, 0x832c2aac
	if !ctx.cr[6].eq {
	pc = 0x832C2AAC; continue 'dispatch;
	}
	// 832C4944: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 832C4948: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 832C494C: 7FBD5A14  add r29, r29, r11
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C4950: 7F185A14  add r24, r24, r11
	ctx.r[24].u64 = ctx.r[24].u64 + ctx.r[11].u64;
	// 832C4954: 93A100A8  stw r29, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 832C4958: 93010078  stw r24, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[24].u32 ) };
	// 832C495C: 419A0020  beq cr6, 0x832c497c
	if ctx.cr[6].eq {
	pc = 0x832C497C; continue 'dispatch;
	}
	// 832C4960: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 832C4964: 409A0010  bne cr6, 0x832c4974
	if !ctx.cr[6].eq {
	pc = 0x832C4974; continue 'dispatch;
	}
	// 832C4968: 562B0738  rlwinm r11, r17, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[17].u32 as u64 & 0xFFFFFFFFu64;
	// 832C496C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C4970: 419A000C  beq cr6, 0x832c497c
	if ctx.cr[6].eq {
	pc = 0x832C497C; continue 'dispatch;
	}
	// 832C4974: 7E73E214  add r19, r19, r28
	ctx.r[19].u64 = ctx.r[19].u64 + ctx.r[28].u64;
	// 832C4978: 92610080  stw r19, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[19].u32 ) };
	// 832C497C: 8161060C  lwz r11, 0x60c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1548 as u32) ) } as u64;
	// 832C4980: 3A310008  addi r17, r17, 8
	ctx.r[17].s64 = ctx.r[17].s64 + 8;
	// 832C4984: 7EB7AB78  mr r23, r21
	ctx.r[23].u64 = ctx.r[21].u64;
	// 832C4988: 9221007C  stw r17, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[17].u32 ) };
	// 832C498C: 7F115840  cmplw cr6, r17, r11
	ctx.cr[6].compare_u32(ctx.r[17].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832C4990: 92E10084  stw r23, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[23].u32 ) };
	// 832C4994: 4198E040  blt cr6, 0x832c29d4
	if ctx.cr[6].lt {
	pc = 0x832C29D4; continue 'dispatch;
	}
	// 832C4998: 83A105F4  lwz r29, 0x5f4(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1524 as u32) ) } as u64;
	// 832C499C: 5579003E  slwi r25, r11, 0
	ctx.r[25].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 832C49A0: 82C100E4  lwz r22, 0xe4(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 832C49A4: 48000008  b 0x832c49ac
	pc = 0x832C49AC; continue 'dispatch;
            }
            0x832C49A8 => {
    //   block [0x832C49A8..0x832C49E8)
	// 832C49A8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 832C49AC: 7D79F9D6  mullw r11, r25, r31
	ctx.r[11].s32 = ((ctx.r[25].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 832C49B0: 7D4BEA14  add r10, r11, r29
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 832C49B4: 57AB0030  rlwinm r11, r29, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 832C49B8: 394A007F  addi r10, r10, 0x7f
	ctx.r[10].s64 = ctx.r[10].s64 + 127;
	// 832C49BC: 554A0030  rlwinm r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 832C49C0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C49C4: 40980014  bge cr6, 0x832c49d8
	if !ctx.cr[6].lt {
	pc = 0x832C49D8; continue 'dispatch;
	}
	// 832C49C8: 7C00586C  dcbst 0, r11
	// 832C49CC: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 832C49D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832C49D4: 4198FFF4  blt cr6, 0x832c49c8
	if ctx.cr[6].lt {
	pc = 0x832C49C8; continue 'dispatch;
	}
	// 832C49D8: 8161065C  lwz r11, 0x65c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1628 as u32) ) } as u64;
	// 832C49DC: 92CB0000  stw r22, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 832C49E0: 382105E0  addi r1, r1, 0x5e0
	ctx.r[1].s64 = ctx.r[1].s64 + 1504;
	// 832C49E4: 4B9E4A3C  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C49E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C49E8 size=968
    let mut pc: u32 = 0x832C49E8;
    'dispatch: loop {
        match pc {
            0x832C49E8 => {
    //   block [0x832C49E8..0x832C4AE0)
	// 832C49E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C49EC: 4B9E49F5  bl 0x82ca93e0
	ctx.lr = 0x832C49F0;
	sub_82CA93D0(ctx, base);
	// 832C49F0: 3D80FFFF  lis r12, -1
	ctx.r[12].s64 = -65536;
	// 832C49F4: 618C4EC0  ori r12, r12, 0x4ec0
	ctx.r[12].u64 = ctx.r[12].u64 | 20160;
	// 832C49F8: 4B9EBC5D  bl 0x82cb0654
	ctx.lr = 0x832C49FC;
	sub_82CB0654(ctx, base);
	// 832C49FC: 7C21616E  stwux r1, r1, r12
	ea = ctx.r[1].u32.wrapping_add(ctx.r[12].u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C4A00: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 832C4A04: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 832C4A08: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 832C4A0C: 392100C0  addi r9, r1, 0xc0
	ctx.r[9].s64 = ctx.r[1].s64 + 192;
	// 832C4A10: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 832C4A14: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4A18: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 832C4A1C: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C4A20: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 832C4A24: 80D80008  lwz r6, 8(r24)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C4A28: 3B4100C0  addi r26, r1, 0xc0
	ctx.r[26].s64 = ctx.r[1].s64 + 192;
	// 832C4A2C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 832C4A30: 80B8000C  lwz r5, 0xc(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 832C4A34: 83B80010  lwz r29, 0x10(r24)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C4A38: 3BC100C0  addi r30, r1, 0xc0
	ctx.r[30].s64 = ctx.r[1].s64 + 192;
	// 832C4A3C: 7ECB4A14  add r22, r11, r9
	ctx.r[22].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 832C4A40: 81380014  lwz r9, 0x14(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 832C4A44: 7D665A14  add r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 832C4A48: 80D8001C  lwz r6, 0x1c(r24)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 832C4A4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832C4A50: 92C10098  stw r22, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[22].u32 ) };
	// 832C4A54: 7EAB3A14  add r21, r11, r7
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 832C4A58: 80F80020  lwz r7, 0x20(r24)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C4A5C: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 832C4A60: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 832C4A64: 80980018  lwz r4, 0x18(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 832C4A68: 7F4BD214  add r26, r11, r26
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 832C4A6C: 92A1009C  stw r21, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[21].u32 ) };
	// 832C4A70: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C4A74: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 832C4A78: 934100A0  stw r26, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[26].u32 ) };
	// 832C4A7C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C4A80: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 832C4A84: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 832C4A88: 93C100A4  stw r30, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 832C4A8C: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 832C4A90: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 832C4A94: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 832C4A98: 906100A8  stw r3, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[3].u32 ) };
	// 832C4A9C: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 832C4AA0: 7D665A14  add r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 832C4AA4: 392100C0  addi r9, r1, 0xc0
	ctx.r[9].s64 = ctx.r[1].s64 + 192;
	// 832C4AA8: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 832C4AAC: 7C675A14  add r3, r7, r11
	ctx.r[3].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 832C4AB0: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 832C4AB4: 91410094  stw r10, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 832C4AB8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 832C4ABC: 90A100AC  stw r5, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[5].u32 ) };
	// 832C4AC0: 90810090  stw r4, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 832C4AC4: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 832C4AC8: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832C4ACC: 2B03B000  cmplwi cr6, r3, 0xb000
	ctx.cr[6].compare_u32(ctx.r[3].u32, 45056 as u32, &mut ctx.xer);
	// 832C4AD0: 40990010  ble cr6, 0x832c4ae0
	if !ctx.cr[6].gt {
	pc = 0x832C4AE0; continue 'dispatch;
	}
	// 832C4AD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832C4AD8: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4ADC: 4B9E4954  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C4AE0 => {
    //   block [0x832C4AE0..0x832C4AFC)
	// 832C4AE0: 3AC80004  addi r22, r8, 4
	ctx.r[22].s64 = ctx.r[8].s64 + 4;
	// 832C4AE4: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 832C4AE8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 832C4AEC: 409A0010  bne cr6, 0x832c4afc
	if !ctx.cr[6].eq {
	pc = 0x832C4AFC; continue 'dispatch;
	}
	// 832C4AF0: 3B6100B4  addi r27, r1, 0xb4
	ctx.r[27].s64 = ctx.r[1].s64 + 180;
	// 832C4AF4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 832C4AF8: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	pc = 0x832C4AFC; continue 'dispatch;
            }
            0x832C4AFC => {
    //   block [0x832C4AFC..0x832C4B40)
	// 832C4AFC: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 832C4B00: 7E98A378  mr r24, r20
	ctx.r[24].u64 = ctx.r[20].u64;
	// 832C4B04: 6BCB0001  xori r11, r30, 1
	ctx.r[11].u64 = ctx.r[30].u64 ^ 1;
	// 832C4B08: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 832C4B0C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4B10: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 832C4B14: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4B18: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4B1C: 8109001C  lwz r8, 0x1c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 832C4B20: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832C4B24: 419A001C  beq cr6, 0x832c4b40
	if ctx.cr[6].eq {
	pc = 0x832C4B40; continue 'dispatch;
	}
	// 832C4B28: 578A05EE  rlwinm r10, r28, 0, 0x17, 0x17
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 832C4B2C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 832C4B30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C4B34: 409A000C  bne cr6, 0x832c4b40
	if !ctx.cr[6].eq {
	pc = 0x832C4B40; continue 'dispatch;
	}
	// 832C4B38: 6BCB0001  xori r11, r30, 1
	ctx.r[11].u64 = ctx.r[30].u64 ^ 1;
	// 832C4B3C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	pc = 0x832C4B40; continue 'dispatch;
            }
            0x832C4B40 => {
    //   block [0x832C4B40..0x832C4BCC)
	// 832C4B40: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 832C4B44: 616BB194  ori r11, r11, 0xb194
	ctx.r[11].u64 = ctx.r[11].u64 | 45460;
	// 832C4B48: 7D61582E  lwzx r11, r1, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[1].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832C4B4C: 556A02D6  rlwinm r10, r11, 0, 0xb, 0xb
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 832C4B50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C4B54: 419A0088  beq cr6, 0x832c4bdc
	if ctx.cr[6].eq {
	pc = 0x832C4BDC; continue 'dispatch;
	}
	// 832C4B58: 578B02D6  rlwinm r11, r28, 0, 0xb, 0xb
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 832C4B5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C4B60: 419A006C  beq cr6, 0x832c4bcc
	if ctx.cr[6].eq {
	pc = 0x832C4BCC; continue 'dispatch;
	}
	// 832C4B64: 57CA083C  slwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4B68: 93810074  stw r28, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[28].u32 ) };
	// 832C4B6C: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4B70: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 832C4B74: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 832C4B78: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 832C4B7C: 7D3D5A14  add r9, r29, r11
	ctx.r[9].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C4B80: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 832C4B84: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4B88: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C4B8C: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4B90: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C4B94: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 832C4B98: 7C6AFA14  add r3, r10, r31
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4B9C: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 832C4BA0: 3B410090  addi r26, r1, 0x90
	ctx.r[26].s64 = ctx.r[1].s64 + 144;
	// 832C4BA4: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 832C4BA8: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 832C4BAC: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 832C4BB0: 80EB0044  lwz r7, 0x44(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 832C4BB4: 39140004  addi r8, r20, 4
	ctx.r[8].s64 = ctx.r[20].s64 + 4;
	// 832C4BB8: 808B0040  lwz r4, 0x40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 832C4BBC: 80630040  lwz r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 832C4BC0: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 832C4BC4: 4BFFDA55  bl 0x832c2618
	ctx.lr = 0x832C4BC8;
	sub_832C2618(ctx, base);
	// 832C4BC8: 83410080  lwz r26, 0x80(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	pc = 0x832C4BCC; continue 'dispatch;
            }
            0x832C4BCC => {
    //   block [0x832C4BCC..0x832C4BDC)
	// 832C4BCC: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4BD0: 7F0BA214  add r24, r11, r20
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	// 832C4BD4: 7F18A040  cmplw cr6, r24, r20
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[20].u32, &mut ctx.xer);
	// 832C4BD8: 419801B4  blt cr6, 0x832c4d8c
	if ctx.cr[6].lt {
	pc = 0x832C4D8C; continue 'dispatch;
	}
	pc = 0x832C4BDC; continue 'dispatch;
            }
            0x832C4BDC => {
    //   block [0x832C4BDC..0x832C4BF4)
	// 832C4BDC: 7F18B040  cmplw cr6, r24, r22
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[22].u32, &mut ctx.xer);
	// 832C4BE0: 409801AC  bge cr6, 0x832c4d8c
	if !ctx.cr[6].lt {
	pc = 0x832C4D8C; continue 'dispatch;
	}
	// 832C4BE4: 57930420  rlwinm r19, r28, 0, 0x10, 0x10
	ctx.r[19].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 832C4BE8: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 832C4BEC: 409A0008  bne cr6, 0x832c4bf4
	if !ctx.cr[6].eq {
	pc = 0x832C4BF4; continue 'dispatch;
	}
	// 832C4BF0: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	pc = 0x832C4BF4; continue 'dispatch;
            }
            0x832C4BF4 => {
    //   block [0x832C4BF4..0x832C4C78)
	// 832C4BF4: 578B05AC  rlwinm r11, r28, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 832C4BF8: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 832C4BFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C4C00: 409A0078  bne cr6, 0x832c4c78
	if !ctx.cr[6].eq {
	pc = 0x832C4C78; continue 'dispatch;
	}
	// 832C4C04: 57CA083C  slwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4C08: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 832C4C0C: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4C10: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 832C4C14: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 832C4C18: 93810074  stw r28, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[28].u32 ) };
	// 832C4C1C: 7D3D5A14  add r9, r29, r11
	ctx.r[9].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C4C20: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 832C4C24: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4C28: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C4C2C: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4C30: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C4C34: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 832C4C38: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 832C4C3C: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 832C4C40: 7C6AFA14  add r3, r10, r31
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4C44: 3A410090  addi r18, r1, 0x90
	ctx.r[18].s64 = ctx.r[1].s64 + 144;
	// 832C4C48: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 832C4C4C: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 832C4C50: 9241006C  stw r18, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[18].u32 ) };
	// 832C4C54: 80EB0020  lwz r7, 0x20(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C4C58: 808B001C  lwz r4, 0x1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 832C4C5C: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 832C4C60: 4BFFD9B9  bl 0x832c2618
	ctx.lr = 0x832C4C64;
	sub_832C2618(ctx, base);
	// 832C4C64: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C4C68: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 832C4C6C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 832C4C70: 40980008  bge cr6, 0x832c4c78
	if !ctx.cr[6].lt {
	pc = 0x832C4C78; continue 'dispatch;
	}
	// 832C4C74: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	pc = 0x832C4C78; continue 'dispatch;
            }
            0x832C4C78 => {
    //   block [0x832C4C78..0x832C4C8C)
	// 832C4C78: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 832C4C7C: 409A0010  bne cr6, 0x832c4c8c
	if !ctx.cr[6].eq {
	pc = 0x832C4C8C; continue 'dispatch;
	}
	// 832C4C80: 8178FFFC  lwz r11, -4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-4 as u32) ) } as u64;
	// 832C4C84: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 832C4C88: 390BFFFC  addi r8, r11, -4
	ctx.r[8].s64 = ctx.r[11].s64 + -4;
	pc = 0x832C4C8C; continue 'dispatch;
            }
            0x832C4C8C => {
    //   block [0x832C4C8C..0x832C4D28)
	// 832C4C8C: 578B039C  rlwinm r11, r28, 0, 0xe, 0xe
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 832C4C90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C4C94: 409A00F8  bne cr6, 0x832c4d8c
	if !ctx.cr[6].eq {
	pc = 0x832C4D8C; continue 'dispatch;
	}
	// 832C4C98: 7F08A040  cmplw cr6, r8, r20
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[20].u32, &mut ctx.xer);
	// 832C4C9C: 419800F0  blt cr6, 0x832c4d8c
	if ctx.cr[6].lt {
	pc = 0x832C4D8C; continue 'dispatch;
	}
	// 832C4CA0: 7F08B040  cmplw cr6, r8, r22
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[22].u32, &mut ctx.xer);
	// 832C4CA4: 409800E8  bge cr6, 0x832c4d8c
	if !ctx.cr[6].lt {
	pc = 0x832C4D8C; continue 'dispatch;
	}
	// 832C4CA8: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 832C4CAC: 56B8083C  slwi r24, r21, 1
	ctx.r[24].u32 = ctx.r[21].u32.wrapping_shl(1);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 832C4CB0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 832C4CB4: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 832C4CB8: 93010064  stw r24, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[24].u32 ) };
	// 832C4CBC: 57CA083C  slwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832C4CC4: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4CC8: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 832C4CCC: 93810074  stw r28, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[28].u32 ) };
	// 832C4CD0: 7D3D5A14  add r9, r29, r11
	ctx.r[9].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C4CD4: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C4CD8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4CDC: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832C4CE0: 552B2036  slwi r11, r9, 4
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4CE4: 7FCAFA14  add r30, r10, r31
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 832C4CE8: 7FABFA14  add r29, r11, r31
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 832C4CEC: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 832C4CF0: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 832C4CF4: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 832C4CF8: 90E1007C  stw r7, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[7].u32 ) };
	// 832C4CFC: 80FE002C  lwz r7, 0x2c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 832C4D00: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 832C4D04: 807D0028  lwz r3, 0x28(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 832C4D08: 4BFFD911  bl 0x832c2618
	ctx.lr = 0x832C4D0C;
	sub_832C2618(ctx, base);
	// 832C4D0C: 80C10080  lwz r6, 0x80(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C4D10: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 832C4D14: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4D18: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 832C4D1C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 832C4D20: 40980008  bge cr6, 0x832c4d28
	if !ctx.cr[6].lt {
	pc = 0x832C4D28; continue 'dispatch;
	}
	// 832C4D24: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	pc = 0x832C4D28; continue 'dispatch;
            }
            0x832C4D28 => {
    //   block [0x832C4D28..0x832C4D8C)
	// 832C4D28: 7F08A040  cmplw cr6, r8, r20
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[20].u32, &mut ctx.xer);
	// 832C4D2C: 41980060  blt cr6, 0x832c4d8c
	if ctx.cr[6].lt {
	pc = 0x832C4D8C; continue 'dispatch;
	}
	// 832C4D30: 7F08B040  cmplw cr6, r8, r22
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[22].u32, &mut ctx.xer);
	// 832C4D34: 40980058  bge cr6, 0x832c4d8c
	if !ctx.cr[6].lt {
	pc = 0x832C4D8C; continue 'dispatch;
	}
	// 832C4D38: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 832C4D3C: 93810074  stw r28, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[28].u32 ) };
	// 832C4D40: 3AA10090  addi r21, r1, 0x90
	ctx.r[21].s64 = ctx.r[1].s64 + 144;
	// 832C4D44: 93010064  stw r24, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[24].u32 ) };
	// 832C4D48: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 832C4D4C: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 832C4D50: 92A1006C  stw r21, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[21].u32 ) };
	// 832C4D54: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 832C4D58: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 832C4D5C: 80FE0038  lwz r7, 0x38(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 832C4D60: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C4D64: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832C4D68: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 832C4D6C: 807D0034  lwz r3, 0x34(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 832C4D70: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 832C4D74: 4BFFD8A5  bl 0x832c2618
	ctx.lr = 0x832C4D78;
	sub_832C2618(ctx, base);
	// 832C4D78: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C4D7C: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4D80: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 832C4D84: 40980008  bge cr6, 0x832c4d8c
	if !ctx.cr[6].lt {
	pc = 0x832C4D8C; continue 'dispatch;
	}
	// 832C4D88: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	pc = 0x832C4D8C; continue 'dispatch;
            }
            0x832C4D8C => {
    //   block [0x832C4D8C..0x832C4DB0)
	// 832C4D8C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C4D90: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C4D94: 7D2B51D6  mullw r9, r11, r10
	ctx.r[9].s32 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 832C4D98: 5528D1BE  srwi r8, r9, 6
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(6);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C4D9C: 7CFA4050  subf r7, r26, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[26].s64;
	// 832C4DA0: 1CC70064  mulli r6, r7, 0x64
	ctx.r[6].s32 = ((ctx.r[7].s32 as i64 * 100 as i64) as i32);
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 832C4DA4: 7C664396  divwu r3, r6, r8
	ctx.r[3].u32 = ctx.r[6].u32 / ctx.r[8].u32;
	// 832C4DA8: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4DAC: 4B9E4684  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C4DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832C4DB0 size=508
    let mut pc: u32 = 0x832C4DB0;
    'dispatch: loop {
        match pc {
            0x832C4DB0 => {
    //   block [0x832C4DB0..0x832C4E00)
	// 832C4DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C4DB4: 4B9E4649  bl 0x82ca93fc
	ctx.lr = 0x832C4DB8;
	sub_82CA93D0(ctx, base);
	// 832C4DB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C4DBC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 832C4DC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832C4DC4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 832C4DC8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 832C4DCC: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C4DD0: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4DD4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832C4DD8: 40990028  ble cr6, 0x832c4e00
	if !ctx.cr[6].gt {
	pc = 0x832C4E00; continue 'dispatch;
	}
	// 832C4DDC: 7FFD1670  srawi r29, r31, 2
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[31].s32 >> 2) as i64;
	// 832C4DE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832C4DE4: 93BA0000  stw r29, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832C4DE8: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 832C4DEC: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832C4DF0: 40990010  ble cr6, 0x832c4e00
	if !ctx.cr[6].gt {
	pc = 0x832C4E00; continue 'dispatch;
	}
	// 832C4DF4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 832C4DF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832C4DFC: 480001B5  bl 0x832c4fb0
	ctx.lr = 0x832C4E00;
	sub_832C4FB0(ctx, base);
	pc = 0x832C4E00; continue 'dispatch;
            }
            0x832C4E00 => {
    //   block [0x832C4E00..0x832C4E2C)
	// 832C4E00: 839A0004  lwz r28, 4(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C4E04: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 832C4E08: 40990024  ble cr6, 0x832c4e2c
	if !ctx.cr[6].gt {
	pc = 0x832C4E2C; continue 'dispatch;
	}
	// 832C4E0C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 832C4E10: 93FA0004  stw r31, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 832C4E14: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 832C4E18: 40990014  ble cr6, 0x832c4e2c
	if !ctx.cr[6].gt {
	pc = 0x832C4E2C; continue 'dispatch;
	}
	// 832C4E1C: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C4E24: 7C8BCA14  add r4, r11, r25
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 832C4E28: 48000379  bl 0x832c51a0
	ctx.lr = 0x832C4E2C;
	sub_832C51A0(ctx, base);
	pc = 0x832C4E2C; continue 'dispatch;
            }
            0x832C4E2C => {
    //   block [0x832C4E2C..0x832C4EB4)
	// 832C4E2C: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4E30: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C4E34: 7FCBCA14  add r30, r11, r25
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 832C4E38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832C4E3C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 832C4E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C4E44: 48002F55  bl 0x832c7d98
	ctx.lr = 0x832C4E48;
	sub_832C7D98(ctx, base);
	// 832C4E48: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 832C4E4C: 409900E0  ble cr6, 0x832c4f2c
	if !ctx.cr[6].gt {
	pc = 0x832C4F2C; continue 'dispatch;
	}
	// 832C4E50: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 832C4E54: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832C4E58: 38BA0008  addi r5, r26, 8
	ctx.r[5].s64 = ctx.r[26].s64 + 8;
	// 832C4E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C4E60: 48000431  bl 0x832c5290
	ctx.lr = 0x832C4E64;
	sub_832C5290(ctx, base);
	// 832C4E64: 7FEB0E70  srawi r11, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 832C4E68: 578A083C  slwi r10, r28, 1
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4E6C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 832C4E70: 7D4A5BD6  divw r10, r10, r11
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 832C4E74: 409900D4  ble cr6, 0x832c4f48
	if !ctx.cr[6].gt {
	pc = 0x832C4F48; continue 'dispatch;
	}
	// 832C4E78: 38FFFFFE  addi r7, r31, -2
	ctx.r[7].s64 = ctx.r[31].s64 + -2;
	// 832C4E7C: 390BFFFD  addi r8, r11, -3
	ctx.r[8].s64 = ctx.r[11].s64 + -3;
	// 832C4E80: 3C80820A  lis r4, -0x7df6
	ctx.r[4].s64 = -2113273856;
	// 832C4E84: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4E88: 7CAA00D0  neg r5, r10
	ctx.r[5].s64 = -ctx.r[10].s64;
	// 832C4E8C: 5507F87E  srwi r7, r8, 1
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C4E90: 5789103A  slwi r9, r28, 2
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C4E94: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832C4E98: C00492D4  lfs f0, -0x6d2c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C4E9C: 395B0008  addi r10, r27, 8
	ctx.r[10].s64 = ctx.r[27].s64 + 8;
	// 832C4EA0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 832C4EA4: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C4EA8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 832C4EAC: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 832C4EB0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	pc = 0x832C4EB4; continue 'dispatch;
            }
            0x832C4EB4 => {
    //   block [0x832C4EB4..0x832C4F2C)
	// 832C4EB4: 7D254A14  add r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 832C4EB8: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C4EBC: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C4EC0: 7D064214  add r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 832C4EC4: ED6D602A  fadds f11, f13, f12
	ctx.f[11].f64 = ((ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C4EC8: C14A0000  lfs f10, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C4ECC: C12B0000  lfs f9, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C4ED0: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 832C4ED4: ED0A4828  fsubs f8, f10, f9
	ctx.f[8].f64 = (((ctx.f[10].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C4ED8: C0E90000  lfs f7, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C4EDC: ECC03828  fsubs f6, f0, f7
	ctx.f[6].f64 = (((ctx.f[0].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C4EE0: C0A80000  lfs f5, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C4EE4: EC8B0172  fmuls f4, f11, f5
	ctx.f[4].f64 = (((ctx.f[11].f64 * ctx.f[5].f64) as f32) as f64);
	// 832C4EE8: EC6B01B2  fmuls f3, f11, f6
	ctx.f[3].f64 = (((ctx.f[11].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C4EEC: EC4821B8  fmsubs f2, f8, f6, f4
	ctx.f[2].f64 = (((ctx.f[8].f64 * ctx.f[6].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C4EF0: EC28197A  fmadds f1, f8, f5, f3
	ctx.f[1].f64 = (((ctx.f[8].f64 * ctx.f[5].f64 + ctx.f[3].f64) as f32) as f64);
	// 832C4EF4: ED8A1028  fsubs f12, f10, f2
	ctx.f[12].f64 = (((ctx.f[10].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C4EF8: D18A0000  stfs f12, 0(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C4EFC: ED6D0828  fsubs f11, f13, f1
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C4F00: D16A0004  stfs f11, 4(r10)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C4F04: C12B0000  lfs f9, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C4F08: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 832C4F0C: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C4F10: ECEA0828  fsubs f7, f10, f1
	ctx.f[7].f64 = (((ctx.f[10].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C4F14: ED09102A  fadds f8, f9, f2
	ctx.f[8].f64 = ((ctx.f[9].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C4F18: D10B0000  stfs f8, 0(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C4F1C: D0EB0004  stfs f7, 4(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C4F20: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 832C4F24: 4082FF90  bne 0x832c4eb4
	if !ctx.cr[0].eq {
	pc = 0x832C4EB4; continue 'dispatch;
	}
	// 832C4F28: 48000020  b 0x832c4f48
	pc = 0x832C4F48; continue 'dispatch;
            }
            0x832C4F2C => {
    //   block [0x832C4F2C..0x832C4F48)
	// 832C4F2C: 409A001C  bne cr6, 0x832c4f48
	if !ctx.cr[6].eq {
	pc = 0x832C4F48; continue 'dispatch;
	}
	// 832C4F30: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 832C4F34: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832C4F38: 38BA0008  addi r5, r26, 8
	ctx.r[5].s64 = ctx.r[26].s64 + 8;
	// 832C4F3C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832C4F40: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 832C4F44: 4800034D  bl 0x832c5290
	ctx.lr = 0x832C4F48;
	sub_832C5290(ctx, base);
	pc = 0x832C4F48; continue 'dispatch;
            }
            0x832C4F48 => {
    //   block [0x832C4F48..0x832C4F74)
	// 832C4F48: C01B0000  lfs f0, 0(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C4F4C: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 832C4F50: C1BB0004  lfs f13, 4(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C4F54: ED8D002A  fadds f12, f13, f0
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C4F58: D19B0000  stfs f12, 0(r27)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C4F5C: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C4F60: 40990038  ble cr6, 0x832c4f98
	if !ctx.cr[6].gt {
	pc = 0x832C4F98; continue 'dispatch;
	}
	// 832C4F64: 395FFFFD  addi r10, r31, -3
	ctx.r[10].s64 = ctx.r[31].s64 + -3;
	// 832C4F68: 397B0008  addi r11, r27, 8
	ctx.r[11].s64 = ctx.r[27].s64 + 8;
	// 832C4F6C: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C4F70: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	pc = 0x832C4F74; continue 'dispatch;
            }
            0x832C4F74 => {
    //   block [0x832C4F74..0x832C4F98)
	// 832C4F74: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C4F78: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C4F7C: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C4F80: ED6D6028  fsubs f11, f13, f12
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C4F84: D16BFFFC  stfs f11, -4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C4F88: ED4C682A  fadds f10, f12, f13
	ctx.f[10].f64 = ((ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C4F8C: D14B0000  stfs f10, 0(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C4F90: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 832C4F94: 4082FFE0  bne 0x832c4f74
	if !ctx.cr[0].eq {
	pc = 0x832C4F74; continue 'dispatch;
	}
	pc = 0x832C4F98; continue 'dispatch;
            }
            0x832C4F98 => {
    //   block [0x832C4F98..0x832C4FAC)
	// 832C4F98: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C4F9C: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 832C4FA0: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C4FA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832C4FA8: 4B9E44A4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C4FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832C4FB0 size=496
    let mut pc: u32 = 0x832C4FB0;
    'dispatch: loop {
        match pc {
            0x832C4FB0 => {
    //   block [0x832C4FB0..0x832C5048)
	// 832C4FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C4FB4: 4B9E4451  bl 0x82ca9404
	ctx.lr = 0x832C4FB8;
	sub_82CA93D0(ctx, base);
	// 832C4FB8: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 832C4FBC: 4B9E8D15  bl 0x82cadcd0
	ctx.lr = 0x832C4FC0;
	sub_82CADCA0(ctx, base);
	// 832C4FC0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C4FC4: 7C7F0E70  srawi r31, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 832C4FC8: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 832C4FCC: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 832C4FD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832C4FD4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 832C4FD8: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832C4FDC: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 832C4FE0: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 832C4FE4: C00A0E60  lfs f0, 0xe60(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3680 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C4FE8: EFE06024  fdivs f31, f0, f12
	ctx.f[31].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 832C4FEC: EC2C07F2  fmuls f1, f12, f31
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C4FF0: 4AF74EC1  bl 0x82239eb0
	ctx.lr = 0x832C4FF4;
	sub_82239EB0(ctx, base);
	// 832C4FF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832C4FF8: FF800818  frsp f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C4FFC: D39E0004  stfs f28, 4(r30)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5000: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 832C5004: 3B6BB7A4  addi r27, r11, -0x485c
	ctx.r[27].s64 = ctx.r[11].s64 + -18524;
	// 832C5008: C37BDCEC  lfs f27, -0x2314(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-8980 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 832C500C: C3DBDB30  lfs f30, -0x24d0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-9424 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C5010: D37E0000  stfs f27, 0(r30)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5014: 41980034  blt cr6, 0x832c5048
	if ctx.cr[6].lt {
	pc = 0x832C5048; continue 'dispatch;
	}
	// 832C5018: C01BFCDC  lfs f0, -0x324(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-804 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C501C: EC3F0032  fmuls f1, f31, f0
	ctx.f[1].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C5020: 4AF74E91  bl 0x82239eb0
	ctx.lr = 0x832C5024;
	sub_82239EB0(ctx, base);
	// 832C5024: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C5028: C01BDB24  lfs f0, -0x24dc(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-9436 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C502C: EC3F0032  fmuls f1, f31, f0
	ctx.f[1].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C5030: ED9E6824  fdivs f12, f30, f13
	ctx.f[12].f64 = ((ctx.f[30].f64 / ctx.f[13].f64) as f32) as f64;
	// 832C5034: D19E0008  stfs f12, 8(r30)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C5038: 4AF74E79  bl 0x82239eb0
	ctx.lr = 0x832C503C;
	sub_82239EB0(ctx, base);
	// 832C503C: FD600818  frsp f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C5040: ED5E5824  fdivs f10, f30, f11
	ctx.f[10].f64 = ((ctx.f[30].f64 / ctx.f[11].f64) as f32) as f64;
	// 832C5044: D15E000C  stfs f10, 0xc(r30)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	pc = 0x832C5048; continue 'dispatch;
            }
            0x832C5048 => {
    //   block [0x832C5048..0x832C505C)
	// 832C5048: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 832C504C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 832C5050: 40990078  ble cr6, 0x832c50c8
	if !ctx.cr[6].gt {
	pc = 0x832C50C8; continue 'dispatch;
	}
	// 832C5054: C3BB0000  lfs f29, 0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 832C5058: 3BBE0018  addi r29, r30, 0x18
	ctx.r[29].s64 = ctx.r[30].s64 + 24;
	pc = 0x832C505C; continue 'dispatch;
            }
            0x832C505C => {
    //   block [0x832C505C..0x832C50C8)
	// 832C505C: 7F8B07B4  extsw r11, r28
	ctx.r[11].s64 = ctx.r[28].s32 as i64;
	// 832C5060: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 832C5064: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832C5068: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 832C506C: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 832C5070: EF4C07F2  fmuls f26, f12, f31
	ctx.f[26].f64 = (((ctx.f[12].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C5074: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 832C5078: 4AF74E39  bl 0x82239eb0
	ctx.lr = 0x832C507C;
	sub_82239EB0(ctx, base);
	// 832C507C: FD600818  frsp f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C5080: D17DFFF8  stfs f11, -8(r29)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 832C5084: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 832C5088: 4AF74F09  bl 0x82239f90
	ctx.lr = 0x832C508C;
	sub_82239F90(ctx, base);
	// 832C508C: FD400818  frsp f10, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[10].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C5090: D15DFFFC  stfs f10, -4(r29)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C5094: EF5A0772  fmuls f26, f26, f29
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[29].f64) as f32) as f64);
	// 832C5098: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 832C509C: 4AF74E15  bl 0x82239eb0
	ctx.lr = 0x832C50A0;
	sub_82239EB0(ctx, base);
	// 832C50A0: FD200818  frsp f9, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[9].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C50A4: D13D0000  stfs f9, 0(r29)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C50A8: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 832C50AC: 4AF74EE5  bl 0x82239f90
	ctx.lr = 0x832C50B0;
	sub_82239F90(ctx, base);
	// 832C50B0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 832C50B4: FD000818  frsp f8, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[8].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C50B8: D11D0004  stfs f8, 4(r29)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C50BC: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 832C50C0: 7F1CF800  cmpw cr6, r28, r31
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[31].s32, &mut ctx.xer);
	// 832C50C4: 4198FF98  blt cr6, 0x832c505c
	if ctx.cr[6].lt {
	pc = 0x832C505C; continue 'dispatch;
	}
	pc = 0x832C50C8; continue 'dispatch;
            }
            0x832C50C8 => {
    //   block [0x832C50C8..0x832C50D4)
	// 832C50C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832C50CC: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 832C50D0: 409900C0  ble cr6, 0x832c5190
	if !ctx.cr[6].gt {
	pc = 0x832C5190; continue 'dispatch;
	}
	pc = 0x832C50D4; continue 'dispatch;
            }
            0x832C50D4 => {
    //   block [0x832C50D4..0x832C5154)
	// 832C50D4: 7D0BFA14  add r8, r11, r31
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 832C50D8: 7FFF0E70  srawi r31, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 832C50DC: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C50E0: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 832C50E4: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 832C50E8: D36A0000  stfs f27, 0(r10)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C50EC: D38A0004  stfs f28, 4(r10)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C50F0: 41980094  blt cr6, 0x832c5184
	if ctx.cr[6].lt {
	pc = 0x832C5184; continue 'dispatch;
	}
	// 832C50F4: 394B0006  addi r10, r11, 6
	ctx.r[10].s64 = ctx.r[11].s64 + 6;
	// 832C50F8: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 832C50FC: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C5100: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832C5104: 38A80002  addi r5, r8, 2
	ctx.r[5].s64 = ctx.r[8].s64 + 2;
	// 832C5108: 38880003  addi r4, r8, 3
	ctx.r[4].s64 = ctx.r[8].s64 + 3;
	// 832C510C: 54A3103A  slwi r3, r5, 2
	ctx.r[3].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 832C5110: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5114: 7C07F42E  lfsx f0, r7, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5118: 7DA6F42E  lfsx f13, r6, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C511C: ED9E0024  fdivs f12, f30, f0
	ctx.f[12].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 832C5120: ED7E6824  fdivs f11, f30, f13
	ctx.f[11].f64 = ((ctx.f[30].f64 / ctx.f[13].f64) as f32) as f64;
	// 832C5124: 7D63F52E  stfsx f11, r3, r30
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 832C5128: 7D8AF52E  stfsx f12, r10, r30
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 832C512C: 40990058  ble cr6, 0x832c5184
	if !ctx.cr[6].gt {
	pc = 0x832C5184; continue 'dispatch;
	}
	// 832C5130: 39480004  addi r10, r8, 4
	ctx.r[10].s64 = ctx.r[8].s64 + 4;
	// 832C5134: 392B000A  addi r9, r11, 0xa
	ctx.r[9].s64 = ctx.r[11].s64 + 10;
	// 832C5138: 38FFFFFB  addi r7, r31, -5
	ctx.r[7].s64 = ctx.r[31].s64 + -5;
	// 832C513C: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5140: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C5144: 54E7F0BE  srwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C5148: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C514C: 7D69F214  add r11, r9, r30
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 832C5150: 39270001  addi r9, r7, 1
	ctx.r[9].s64 = ctx.r[7].s64 + 1;
	pc = 0x832C5154; continue 'dispatch;
            }
            0x832C5154 => {
    //   block [0x832C5154..0x832C5184)
	// 832C5154: C00BFFFC  lfs f0, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5158: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 832C515C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5160: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5164: C16BFFF8  lfs f11, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C5168: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C516C: D16A0000  stfs f11, 0(r10)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5170: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5174: D1AA0008  stfs f13, 8(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C5178: D18A000C  stfs f12, 0xc(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C517C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 832C5180: 4082FFD4  bne 0x832c5154
	if !ctx.cr[0].eq {
	pc = 0x832C5154; continue 'dispatch;
	}
	pc = 0x832C5184; continue 'dispatch;
            }
            0x832C5184 => {
    //   block [0x832C5184..0x832C5190)
	// 832C5184: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 832C5188: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 832C518C: 4199FF48  bgt cr6, 0x832c50d4
	if ctx.cr[6].gt {
	pc = 0x832C50D4; continue 'dispatch;
	}
	pc = 0x832C5190; continue 'dispatch;
            }
            0x832C5190 => {
    //   block [0x832C5190..0x832C51A0)
	// 832C5190: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 832C5194: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 832C5198: 4B9E8B85  bl 0x82cadd1c
	ctx.lr = 0x832C519C;
	sub_82CADCEC(ctx, base);
	// 832C519C: 4B9E42B8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C51A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832C51A0 size=236
    let mut pc: u32 = 0x832C51A0;
    'dispatch: loop {
        match pc {
            0x832C51A0 => {
    //   block [0x832C51A0..0x832C5224)
	// 832C51A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C51A4: 4B9E4265  bl 0x82ca9408
	ctx.lr = 0x832C51A8;
	sub_82CA93D0(ctx, base);
	// 832C51A8: DBA1FFC0  stfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[29].u64 ) };
	// 832C51AC: DBC1FFC8  stfd f30, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 832C51B0: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 832C51B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C51B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832C51BC: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 832C51C0: 7FDC0E70  srawi r28, r30, 1
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[30].s32 >> 1) as i64;
	// 832C51C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 832C51C8: 7F8B07B4  extsw r11, r28
	ctx.r[11].s64 = ctx.r[28].s32 as i64;
	// 832C51CC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 832C51D0: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832C51D4: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 832C51D8: C00A0E60  lfs f0, 0xe60(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3680 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C51DC: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 832C51E0: EFC06024  fdivs f30, f0, f12
	ctx.f[30].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 832C51E4: EC2C07B2  fmuls f1, f12, f30
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[30].f64) as f32) as f64);
	// 832C51E8: 4AF74CC9  bl 0x82239eb0
	ctx.lr = 0x832C51EC;
	sub_82239EB0(ctx, base);
	// 832C51EC: FD600818  frsp f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C51F0: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 832C51F4: 5788103A  slwi r8, r28, 2
	ctx.r[8].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C51F8: D17D0000  stfs f11, 0(r29)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C51FC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 832C5200: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 832C5204: C3E992D4  lfs f31, -0x6d2c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C5208: ED4B07F2  fmuls f10, f11, f31
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C520C: 7D48ED2E  stfsx f10, r8, r29
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 832C5210: 40990068  ble cr6, 0x832c5278
	if !ctx.cr[6].gt {
	pc = 0x832C5278; continue 'dispatch;
	}
	// 832C5214: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5218: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 832C521C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 832C5220: 3BABFFFC  addi r29, r11, -4
	ctx.r[29].s64 = ctx.r[11].s64 + -4;
	pc = 0x832C5224; continue 'dispatch;
            }
            0x832C5224 => {
    //   block [0x832C5224..0x832C5278)
	// 832C5224: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 832C5228: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 832C522C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832C5230: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 832C5234: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 832C5238: EFAC07B2  fmuls f29, f12, f30
	ctx.f[29].f64 = (((ctx.f[12].f64 * ctx.f[30].f64) as f32) as f64);
	// 832C523C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 832C5240: 4AF74C71  bl 0x82239eb0
	ctx.lr = 0x832C5244;
	sub_82239EB0(ctx, base);
	// 832C5244: FD600818  frsp f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C5248: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 832C524C: ED4B07F2  fmuls f10, f11, f31
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C5250: D15E0000  stfs f10, 0(r30)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5254: 4AF74D3D  bl 0x82239f90
	ctx.lr = 0x832C5258;
	sub_82239F90(ctx, base);
	// 832C5258: FD200818  frsp f9, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[9].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832C525C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 832C5260: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 832C5264: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 832C5268: ED0907F2  fmuls f8, f9, f31
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C526C: D11D0000  stfs f8, 0(r29)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5270: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 832C5274: 4198FFB0  blt cr6, 0x832c5224
	if ctx.cr[6].lt {
	pc = 0x832C5224; continue 'dispatch;
	}
	pc = 0x832C5278; continue 'dispatch;
            }
            0x832C5278 => {
    //   block [0x832C5278..0x832C528C)
	// 832C5278: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 832C527C: CBA1FFC0  lfd f29, -0x40(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 832C5280: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 832C5284: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 832C5288: 4B9E41D0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C5290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832C5290 size=840
    let mut pc: u32 = 0x832C5290;
    'dispatch: loop {
        match pc {
            0x832C5290 => {
    //   block [0x832C5290..0x832C535C)
	// 832C5290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C5294: 4B9E416D  bl 0x82ca9400
	ctx.lr = 0x832C5298;
	sub_82CA93D0(ctx, base);
	// 832C5298: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 832C529C: 4B9E8A25  bl 0x82cadcc0
	ctx.lr = 0x832C52A0;
	sub_82CADCA0(ctx, base);
	// 832C52A0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C52A4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 832C52A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832C52AC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 832C52B0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 832C52B4: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 832C52B8: 2F1B0020  cmpwi cr6, r27, 0x20
	ctx.cr[6].compare_i32(ctx.r[27].s32, 32, &mut ctx.xer);
	// 832C52BC: 409900F4  ble cr6, 0x832c53b0
	if !ctx.cr[6].gt {
	pc = 0x832C53B0; continue 'dispatch;
	}
	// 832C52C0: 7F7E1670  srawi r30, r27, 2
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[27].s32 >> 2) as i64;
	// 832C52C4: 7D7EE050  subf r11, r30, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 832C52C8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C52CC: 7CABEA14  add r5, r11, r29
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 832C52D0: 48000829  bl 0x832c5af8
	ctx.lr = 0x832C52D4;
	sub_832C5AF8(ctx, base);
	// 832C52D4: 2F1B0200  cmpwi cr6, r27, 0x200
	ctx.cr[6].compare_i32(ctx.r[27].s32, 512, &mut ctx.xer);
	// 832C52D8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832C52DC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C52E0: 4099007C  ble cr6, 0x832c535c
	if !ctx.cr[6].gt {
	pc = 0x832C535C; continue 'dispatch;
	}
	// 832C52E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832C52E8: 48000F01  bl 0x832c61e8
	ctx.lr = 0x832C52EC;
	sub_832C61E8(ctx, base);
	// 832C52EC: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C52F0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832C52F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C52F8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 832C52FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832C5300: 48000FC1  bl 0x832c62c0
	ctx.lr = 0x832C5304;
	sub_832C62C0(ctx, base);
	// 832C5304: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5308: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832C530C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C5310: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 832C5314: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832C5318: 48000ED1  bl 0x832c61e8
	ctx.lr = 0x832C531C;
	sub_832C61E8(ctx, base);
	// 832C531C: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5320: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832C5324: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 832C5328: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C532C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5330: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832C5334: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 832C5338: 48000EB1  bl 0x832c61e8
	ctx.lr = 0x832C533C;
	sub_832C61E8(ctx, base);
	// 832C533C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832C5340: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 832C5344: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832C5348: 48000291  bl 0x832c55d8
	ctx.lr = 0x832C534C;
	sub_832C55D8(ctx, base);
	// 832C534C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 832C5350: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 832C5354: 4B9E89B9  bl 0x82cadd0c
	ctx.lr = 0x832C5358;
	sub_82CADCEC(ctx, base);
	// 832C5358: 4B9E40F8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C535C => {
    //   block [0x832C535C..0x832C538C)
	// 832C535C: 2F1E0020  cmpwi cr6, r30, 0x20
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32, &mut ctx.xer);
	// 832C5360: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832C5364: 40990028  ble cr6, 0x832c538c
	if !ctx.cr[6].gt {
	pc = 0x832C538C; continue 'dispatch;
	}
	// 832C5368: 48001029  bl 0x832c6390
	ctx.lr = 0x832C536C;
	sub_832C6390(ctx, base);
	// 832C536C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832C5370: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 832C5374: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832C5378: 48000261  bl 0x832c55d8
	ctx.lr = 0x832C537C;
	sub_832C55D8(ctx, base);
	// 832C537C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 832C5380: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 832C5384: 4B9E8989  bl 0x82cadd0c
	ctx.lr = 0x832C5388;
	sub_82CADCEC(ctx, base);
	// 832C5388: 4B9E40C8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C538C => {
    //   block [0x832C538C..0x832C53B0)
	// 832C538C: 48001C45  bl 0x832c6fd0
	ctx.lr = 0x832C5390;
	sub_832C6FD0(ctx, base);
	// 832C5390: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832C5394: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 832C5398: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832C539C: 4800023D  bl 0x832c55d8
	ctx.lr = 0x832C53A0;
	sub_832C55D8(ctx, base);
	// 832C53A0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 832C53A4: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 832C53A8: 4B9E8965  bl 0x82cadd0c
	ctx.lr = 0x832C53AC;
	sub_82CADCEC(ctx, base);
	// 832C53AC: 4B9E40A4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C53B0 => {
    //   block [0x832C53B0..0x832C54A4)
	// 832C53B0: 2F1B0008  cmpwi cr6, r27, 8
	ctx.cr[6].compare_i32(ctx.r[27].s32, 8, &mut ctx.xer);
	// 832C53B4: 40990148  ble cr6, 0x832c54fc
	if !ctx.cr[6].gt {
	pc = 0x832C54FC; continue 'dispatch;
	}
	// 832C53B8: 2F1B0020  cmpwi cr6, r27, 0x20
	ctx.cr[6].compare_i32(ctx.r[27].s32, 32, &mut ctx.xer);
	// 832C53BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C53C0: 409A00E4  bne cr6, 0x832c54a4
	if !ctx.cr[6].eq {
	pc = 0x832C54A4; continue 'dispatch;
	}
	// 832C53C4: 397CFFF8  addi r11, r28, -8
	ctx.r[11].s64 = ctx.r[28].s64 + -8;
	// 832C53C8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C53CC: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 832C53D0: 48001D49  bl 0x832c7118
	ctx.lr = 0x832C53D4;
	sub_832C7118(ctx, base);
	// 832C53D4: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C53D8: C1BF000C  lfs f13, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C53DC: C19F0010  lfs f12, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C53E0: C17F0014  lfs f11, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C53E4: C15F0018  lfs f10, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C53E8: C13F001C  lfs f9, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C53EC: C11F0028  lfs f8, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C53F0: C0FF002C  lfs f7, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C53F4: C0DF0038  lfs f6, 0x38(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C53F8: C0BF003C  lfs f5, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C53FC: C09F0058  lfs f4, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C5400: C07F005C  lfs f3, 0x5c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C5404: C05F0040  lfs f2, 0x40(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C5408: C03F0044  lfs f1, 0x44(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C540C: D05F0008  stfs f2, 8(r31)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C5410: D03F000C  stfs f1, 0xc(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C5414: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 832C5418: D1BF0044  stfs f13, 0x44(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 832C541C: C3FF0020  lfs f31, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C5420: C3DF0024  lfs f30, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C5424: C3BF0060  lfs f29, 0x60(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 832C5428: C39F0064  lfs f28, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C542C: C37F0050  lfs f27, 0x50(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 832C5430: C35F0054  lfs f26, 0x54(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 832C5434: C33F0070  lfs f25, 0x70(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 832C5438: C31F0074  lfs f24, 0x74(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 832C543C: C2FF0068  lfs f23, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 832C5440: C2DF006C  lfs f22, 0x6c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 832C5444: D3FF0010  stfs f31, 0x10(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 832C5448: D3DF0014  stfs f30, 0x14(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832C544C: D3BF0018  stfs f29, 0x18(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 832C5450: D39F001C  stfs f28, 0x1c(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 832C5454: D19F0020  stfs f12, 0x20(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 832C5458: D17F0024  stfs f11, 0x24(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 832C545C: D37F0028  stfs f27, 0x28(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 832C5460: D35F002C  stfs f26, 0x2c(r31)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 832C5464: D33F0038  stfs f25, 0x38(r31)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 832C5468: D31F003C  stfs f24, 0x3c(r31)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 832C546C: D11F0050  stfs f8, 0x50(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 832C5470: D0FF0054  stfs f7, 0x54(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 832C5474: D2FF0058  stfs f23, 0x58(r31)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 832C5478: D2DF005C  stfs f22, 0x5c(r31)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 832C547C: D15F0060  stfs f10, 0x60(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 832C5480: D13F0064  stfs f9, 0x64(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 832C5484: D09F0068  stfs f4, 0x68(r31)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 832C5488: D07F006C  stfs f3, 0x6c(r31)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 832C548C: D0DF0070  stfs f6, 0x70(r31)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 832C5490: D0BF0074  stfs f5, 0x74(r31)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 832C5494: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 832C5498: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 832C549C: 4B9E8871  bl 0x82cadd0c
	ctx.lr = 0x832C54A0;
	sub_82CADCEC(ctx, base);
	// 832C54A0: 4B9E3FB0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C54A4 => {
    //   block [0x832C54A4..0x832C54FC)
	// 832C54A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832C54A8: 48002599  bl 0x832c7a40
	ctx.lr = 0x832C54AC;
	sub_832C7A40(ctx, base);
	// 832C54AC: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C54B0: C1BF000C  lfs f13, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C54B4: C19F0018  lfs f12, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C54B8: C17F001C  lfs f11, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C54BC: C15F0020  lfs f10, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C54C0: C13F0024  lfs f9, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C54C4: C11F0030  lfs f8, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C54C8: C0FF0034  lfs f7, 0x34(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C54CC: D15F0008  stfs f10, 8(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C54D0: D13F000C  stfs f9, 0xc(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C54D4: D11F0018  stfs f8, 0x18(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 832C54D8: D0FF001C  stfs f7, 0x1c(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 832C54DC: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 832C54E0: D1BF0024  stfs f13, 0x24(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 832C54E4: D19F0030  stfs f12, 0x30(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 832C54E8: D17F0034  stfs f11, 0x34(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 832C54EC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 832C54F0: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 832C54F4: 4B9E8819  bl 0x82cadd0c
	ctx.lr = 0x832C54F8;
	sub_82CADCEC(ctx, base);
	// 832C54F8: 4B9E3F58  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C54FC => {
    //   block [0x832C54FC..0x832C5590)
	// 832C54FC: 409A0094  bne cr6, 0x832c5590
	if !ctx.cr[6].eq {
	pc = 0x832C5590; continue 'dispatch;
	}
	// 832C5500: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5504: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5508: C11F0018  lfs f8, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C550C: ED60682A  fadds f11, f0, f13
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C5510: C0DF0008  lfs f6, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C5514: ED206828  fsubs f9, f0, f13
	ctx.f[9].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C5518: C19F0014  lfs f12, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C551C: EC68302A  fadds f3, f8, f6
	ctx.f[3].f64 = ((ctx.f[8].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C5520: C15F0004  lfs f10, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C5524: EC264028  fsubs f1, f6, f8
	ctx.f[1].f64 = (((ctx.f[6].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C5528: C09F001C  lfs f4, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C552C: ECEA602A  fadds f7, f10, f12
	ctx.f[7].f64 = ((ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C5530: C05F000C  lfs f2, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C5534: ECAA6028  fsubs f5, f10, f12
	ctx.f[5].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C5538: EC04102A  fadds f0, f4, f2
	ctx.f[0].f64 = ((ctx.f[4].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C553C: EDA22028  fsubs f13, f2, f4
	ctx.f[13].f64 = (((ctx.f[2].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C5540: ED83582A  fadds f12, f3, f11
	ctx.f[12].f64 = ((ctx.f[3].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C5544: D19F0000  stfs f12, 0(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5548: ED6B1828  fsubs f11, f11, f3
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C554C: D17F0010  stfs f11, 0x10(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 832C5550: ECC1282A  fadds f6, f1, f5
	ctx.f[6].f64 = ((ctx.f[1].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C5554: D0DF000C  stfs f6, 0xc(r31)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C5558: ED40382A  fadds f10, f0, f7
	ctx.f[10].f64 = ((ctx.f[0].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C555C: D15F0004  stfs f10, 4(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5560: ED070028  fsubs f8, f7, f0
	ctx.f[8].f64 = (((ctx.f[7].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C5564: D11F0014  stfs f8, 0x14(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832C5568: ECE96828  fsubs f7, f9, f13
	ctx.f[7].f64 = (((ctx.f[9].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C556C: D0FF0008  stfs f7, 8(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C5570: EC8D482A  fadds f4, f13, f9
	ctx.f[4].f64 = ((ctx.f[13].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C5574: D09F0018  stfs f4, 0x18(r31)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 832C5578: EC650828  fsubs f3, f5, f1
	ctx.f[3].f64 = (((ctx.f[5].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C557C: D07F001C  stfs f3, 0x1c(r31)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 832C5580: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 832C5584: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 832C5588: 4B9E8785  bl 0x82cadd0c
	ctx.lr = 0x832C558C;
	sub_82CADCEC(ctx, base);
	// 832C558C: 4B9E3EC4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C5590 => {
    //   block [0x832C5590..0x832C55C8)
	// 832C5590: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 832C5594: 409A0034  bne cr6, 0x832c55c8
	if !ctx.cr[6].eq {
	pc = 0x832C55C8; continue 'dispatch;
	}
	// 832C5598: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C559C: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C55A0: C19F0004  lfs f12, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C55A4: ED606828  fsubs f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C55A8: C15F000C  lfs f10, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C55AC: ED20682A  fadds f9, f0, f13
	ctx.f[9].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C55B0: ED0C5028  fsubs f8, f12, f10
	ctx.f[8].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C55B4: D13F0000  stfs f9, 0(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C55B8: ECEC502A  fadds f7, f12, f10
	ctx.f[7].f64 = ((ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C55BC: D0FF0004  stfs f7, 4(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C55C0: D17F0008  stfs f11, 8(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C55C4: D11F000C  stfs f8, 0xc(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	pc = 0x832C55C8; continue 'dispatch;
            }
            0x832C55C8 => {
    //   block [0x832C55C8..0x832C55D8)
	// 832C55C8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 832C55CC: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 832C55D0: 4B9E873D  bl 0x82cadd0c
	ctx.lr = 0x832C55D4;
	sub_82CADCEC(ctx, base);
	// 832C55D4: 4B9E3E7C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C55D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C55D8 size=1312
    let mut pc: u32 = 0x832C55D8;
    'dispatch: loop {
        match pc {
            0x832C55D8 => {
    //   block [0x832C55D8..0x832C55F8)
	// 832C55D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C55DC: 4B9E3E11  bl 0x82ca93ec
	ctx.lr = 0x832C55E0;
	sub_82CA93D0(ctx, base);
	// 832C55E0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 832C55E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832C55E8: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 832C55EC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832C55F0: 2F070008  cmpwi cr6, r7, 8
	ctx.cr[6].compare_i32(ctx.r[7].s32, 8, &mut ctx.xer);
	// 832C55F4: 4099004C  ble cr6, 0x832c5640
	if !ctx.cr[6].gt {
	pc = 0x832C5640; continue 'dispatch;
	}
	pc = 0x832C55F8; continue 'dispatch;
            }
            0x832C55F8 => {
    //   block [0x832C55F8..0x832C5614)
	// 832C55F8: 7CE70E70  srawi r7, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 832C55FC: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 832C5600: 40990030  ble cr6, 0x832c5630
	if !ctx.cr[6].gt {
	pc = 0x832C5630; continue 'dispatch;
	}
	// 832C5604: 56AB103A  slwi r11, r21, 2
	ctx.r[11].u32 = ctx.r[21].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5608: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 832C560C: 7D2B2214  add r9, r11, r4
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 832C5610: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	pc = 0x832C5614; continue 'dispatch;
            }
            0x832C5614 => {
    //   block [0x832C5614..0x832C5630)
	// 832C5614: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C5618: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832C561C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 832C5620: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 832C5624: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 832C5628: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 832C562C: 4082FFE8  bne 0x832c5614
	if !ctx.cr[0].eq {
	pc = 0x832C5614; continue 'dispatch;
	}
	pc = 0x832C5630; continue 'dispatch;
            }
            0x832C5630 => {
    //   block [0x832C5630..0x832C5640)
	// 832C5630: 56B5083C  slwi r21, r21, 1
	ctx.r[21].u32 = ctx.r[21].u32.wrapping_shl(1);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 832C5634: 56AB1838  slwi r11, r21, 3
	ctx.r[11].u32 = ctx.r[21].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5638: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 832C563C: 4198FFBC  blt cr6, 0x832c55f8
	if ctx.cr[6].lt {
	pc = 0x832C55F8; continue 'dispatch;
	}
	pc = 0x832C5640; continue 'dispatch;
            }
            0x832C5640 => {
    //   block [0x832C5640..0x832C5664)
	// 832C5640: 56AA1838  slwi r10, r21, 3
	ctx.r[10].u32 = ctx.r[21].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5644: 56AB083C  slwi r11, r21, 1
	ctx.r[11].u32 = ctx.r[21].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5648: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 832C564C: 409A0198  bne cr6, 0x832c57e4
	if !ctx.cr[6].eq {
	pc = 0x832C57E4; continue 'dispatch;
	}
	// 832C5650: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 832C5654: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 832C5658: 4099049C  ble cr6, 0x832c5af4
	if !ctx.cr[6].gt {
	pc = 0x832C5AF4; continue 'dispatch;
	}
	// 832C565C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 832C5660: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	pc = 0x832C5664; continue 'dispatch;
            }
            0x832C5664 => {
    //   block [0x832C5664..0x832C567C)
	// 832C5664: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 832C5668: 40990124  ble cr6, 0x832c578c
	if !ctx.cr[6].gt {
	pc = 0x832C578C; continue 'dispatch;
	}
	// 832C566C: 557C083C  slwi r28, r11, 1
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C5670: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832C5674: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 832C5678: 7EC7B378  mr r7, r22
	ctx.r[7].u64 = ctx.r[22].u64;
	pc = 0x832C567C; continue 'dispatch;
            }
            0x832C567C => {
    //   block [0x832C567C..0x832C578C)
	// 832C567C: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C5680: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 832C5684: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C5688: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C568C: 7D175214  add r8, r23, r10
	ctx.r[8].u64 = ctx.r[23].u64 + ctx.r[10].u64;
	// 832C5690: 7FE34A14  add r31, r3, r9
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[9].u64;
	// 832C5694: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5698: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C569C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 832C56A0: 7D292A14  add r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[5].u64;
	// 832C56A4: 7FBF5A14  add r29, r31, r11
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 832C56A8: 3BE90004  addi r31, r9, 4
	ctx.r[31].s64 = ctx.r[9].s64 + 4;
	// 832C56AC: 57BB103A  slwi r27, r29, 2
	ctx.r[27].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 832C56B0: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C56B4: 3BCA0004  addi r30, r10, 4
	ctx.r[30].s64 = ctx.r[10].s64 + 4;
	// 832C56B8: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C56BC: 7D1C4214  add r8, r28, r8
	ctx.r[8].u64 = ctx.r[28].u64 + ctx.r[8].u64;
	// 832C56C0: C18A0004  lfs f12, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C56C4: 7FBD5A14  add r29, r29, r11
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C56C8: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C56CC: 7D3B2A14  add r9, r27, r5
	ctx.r[9].u64 = ctx.r[27].u64 + ctx.r[5].u64;
	// 832C56D0: C17F0000  lfs f11, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C56D4: 551B103A  slwi r27, r8, 2
	ctx.r[27].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 832C56D8: D19F0000  stfs f12, 0(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C56DC: 3BE90004  addi r31, r9, 4
	ctx.r[31].s64 = ctx.r[9].s64 + 4;
	// 832C56E0: D1AA0000  stfs f13, 0(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C56E4: 7D5B2A14  add r10, r27, r5
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[5].u64;
	// 832C56E8: D17E0000  stfs f11, 0(r30)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C56EC: 57BB103A  slwi r27, r29, 2
	ctx.r[27].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 832C56F0: C1490004  lfs f10, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C56F4: 7D0B4050  subf r8, r11, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 832C56F8: C1290000  lfs f9, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C56FC: 7FBD5A14  add r29, r29, r11
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C5700: 551A103A  slwi r26, r8, 2
	ctx.r[26].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 832C5704: C10A0000  lfs f8, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C5708: 57B9103A  slwi r25, r29, 2
	ctx.r[25].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 832C570C: C0EA0004  lfs f7, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C5710: 7FBC4214  add r29, r28, r8
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[8].u64;
	// 832C5714: D1090000  stfs f8, 0(r9)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5718: 7D3B2A14  add r9, r27, r5
	ctx.r[9].u64 = ctx.r[27].u64 + ctx.r[5].u64;
	// 832C571C: D0FF0000  stfs f7, 0(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5720: 7D1A2A14  add r8, r26, r5
	ctx.r[8].u64 = ctx.r[26].u64 + ctx.r[5].u64;
	// 832C5724: D12A0000  stfs f9, 0(r10)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5728: 3BCA0004  addi r30, r10, 4
	ctx.r[30].s64 = ctx.r[10].s64 + 4;
	// 832C572C: D14A0004  stfs f10, 4(r10)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5730: 7D592A14  add r10, r25, r5
	ctx.r[10].u64 = ctx.r[25].u64 + ctx.r[5].u64;
	// 832C5734: 7CDB2C2E  lfsx f6, r27, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C5738: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 832C573C: C0A90004  lfs f5, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C5740: 3BE90004  addi r31, r9, 4
	ctx.r[31].s64 = ctx.r[9].s64 + 4;
	// 832C5744: 7C9A2C2E  lfsx f4, r26, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C5748: 3BC80004  addi r30, r8, 4
	ctx.r[30].s64 = ctx.r[8].s64 + 4;
	// 832C574C: C0680004  lfs f3, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C5750: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 832C5754: D0690004  stfs f3, 4(r9)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5758: 7D3D2A14  add r9, r29, r5
	ctx.r[9].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	// 832C575C: 7C9B2D2E  stfsx f4, r27, r5
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5760: 7CDA2D2E  stfsx f6, r26, r5
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5764: D0A80004  stfs f5, 4(r8)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5768: C04A0004  lfs f2, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C576C: 7C392C2E  lfsx f1, r25, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C5770: 7C1D2C2E  lfsx f0, r29, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5774: C1A90004  lfs f13, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5778: 7C192D2E  stfsx f0, r25, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C577C: D1AA0004  stfs f13, 4(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5780: 7C3D2D2E  stfsx f1, r29, r5
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5784: D0490004  stfs f2, 4(r9)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5788: 4082FEF4  bne 0x832c567c
	if !ctx.cr[0].eq {
	pc = 0x832C567C; continue 'dispatch;
	}
	pc = 0x832C578C; continue 'dispatch;
            }
            0x832C578C => {
    //   block [0x832C578C..0x832C57E4)
	// 832C578C: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C5790: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 832C5794: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 832C5798: 7D575214  add r10, r23, r10
	ctx.r[10].u64 = ctx.r[23].u64 + ctx.r[10].u64;
	// 832C579C: 3AF70002  addi r23, r23, 2
	ctx.r[23].s64 = ctx.r[23].s64 + 2;
	// 832C57A0: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832C57A4: 7F16A800  cmpw cr6, r22, r21
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[21].s32, &mut ctx.xer);
	// 832C57A8: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832C57AC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C57B0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C57B4: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 832C57B8: 7D292A14  add r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[5].u64;
	// 832C57BC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C57C0: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C57C4: C18A0004  lfs f12, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C57C8: C1690004  lfs f11, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C57CC: D1AA0000  stfs f13, 0(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C57D0: D16A0004  stfs f11, 4(r10)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C57D4: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C57D8: D1890004  stfs f12, 4(r9)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C57DC: 4198FE88  blt cr6, 0x832c5664
	if ctx.cr[6].lt {
	pc = 0x832C5664; continue 'dispatch;
	}
	// 832C57E0: 4B9E3C5C  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            0x832C57E4 => {
    //   block [0x832C57E4..0x832C57F8)
	// 832C57E4: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 832C57E8: 2F150001  cmpwi cr6, r21, 1
	ctx.cr[6].compare_i32(ctx.r[21].s32, 1, &mut ctx.xer);
	// 832C57EC: 40990308  ble cr6, 0x832c5af4
	if !ctx.cr[6].gt {
	pc = 0x832C5AF4; continue 'dispatch;
	}
	// 832C57F0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 832C57F4: 38C40004  addi r6, r4, 4
	ctx.r[6].s64 = ctx.r[4].s64 + 4;
	pc = 0x832C57F8; continue 'dispatch;
            }
            0x832C57F8 => {
    //   block [0x832C57F8..0x832C581C)
	// 832C57F8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 832C57FC: 2F180004  cmpwi cr6, r24, 4
	ctx.cr[6].compare_i32(ctx.r[24].s32, 4, &mut ctx.xer);
	// 832C5800: 41980238  blt cr6, 0x832c5a38
	if ctx.cr[6].lt {
	pc = 0x832C5A38; continue 'dispatch;
	}
	// 832C5804: 3958FFFC  addi r10, r24, -4
	ctx.r[10].s64 = ctx.r[24].s64 + -4;
	// 832C5808: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832C580C: 5548F0BE  srwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C5810: 39440008  addi r10, r4, 8
	ctx.r[10].s64 = ctx.r[4].s64 + 8;
	// 832C5814: 3BE80001  addi r31, r8, 1
	ctx.r[31].s64 = ctx.r[8].s64 + 1;
	// 832C5818: 57F9103A  slwi r25, r31, 2
	ctx.r[25].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	pc = 0x832C581C; continue 'dispatch;
            }
            0x832C581C => {
    //   block [0x832C581C..0x832C5A38)
	// 832C581C: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C5820: 3BC90006  addi r30, r9, 6
	ctx.r[30].s64 = ctx.r[9].s64 + 6;
	// 832C5824: 80EAFFF8  lwz r7, -8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C5828: 7FA84A14  add r29, r8, r9
	ctx.r[29].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 832C582C: 7F871A14  add r28, r7, r3
	ctx.r[28].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 832C5830: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C5834: 5787103A  slwi r7, r28, 2
	ctx.r[7].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C5838: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 832C583C: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 832C5840: 3B680004  addi r27, r8, 4
	ctx.r[27].s64 = ctx.r[8].s64 + 4;
	// 832C5844: 3B470004  addi r26, r7, 4
	ctx.r[26].s64 = ctx.r[7].s64 + 4;
	// 832C5848: 7FBD5A14  add r29, r29, r11
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C584C: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5850: 7F9C5A14  add r28, r28, r11
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 832C5854: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5858: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 832C585C: C1870000  lfs f12, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5860: 579C103A  slwi r28, r28, 2
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C5864: C1670004  lfs f11, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C5868: D1880000  stfs f12, 0(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C586C: 7D1D2A14  add r8, r29, r5
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	// 832C5870: D17B0000  stfs f11, 0(r27)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5874: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5878: 7CFC2A14  add r7, r28, r5
	ctx.r[7].u64 = ctx.r[28].u64 + ctx.r[5].u64;
	// 832C587C: D1BA0000  stfs f13, 0(r26)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5880: 7D5C2C2E  lfsx f10, r28, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C5884: C1270004  lfs f9, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C5888: C0E80004  lfs f7, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C588C: 7D1D2C2E  lfsx f8, r29, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C5890: 7D5D2D2E  stfsx f10, r29, r5
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5894: D1280004  stfs f9, 4(r8)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5898: 7D1C2D2E  stfsx f8, r28, r5
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C589C: D0E70004  stfs f7, 4(r7)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C58A0: 80EAFFFC  lwz r7, -4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 832C58A4: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C58A8: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 832C58AC: 3BA80002  addi r29, r8, 2
	ctx.r[29].s64 = ctx.r[8].s64 + 2;
	// 832C58B0: 7F871A14  add r28, r7, r3
	ctx.r[28].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 832C58B4: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C58B8: 5787103A  slwi r7, r28, 2
	ctx.r[7].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C58BC: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 832C58C0: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 832C58C4: 3B680004  addi r27, r8, 4
	ctx.r[27].s64 = ctx.r[8].s64 + 4;
	// 832C58C8: 7FBD5A14  add r29, r29, r11
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C58CC: 7F9C5A14  add r28, r28, r11
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 832C58D0: C0C80004  lfs f6, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C58D4: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 832C58D8: C0A80000  lfs f5, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C58DC: 579C103A  slwi r28, r28, 2
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C58E0: C0870000  lfs f4, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C58E4: 3B470004  addi r26, r7, 4
	ctx.r[26].s64 = ctx.r[7].s64 + 4;
	// 832C58E8: C0670004  lfs f3, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C58EC: D0880000  stfs f4, 0(r8)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C58F0: 7D1D2A14  add r8, r29, r5
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	// 832C58F4: D07B0000  stfs f3, 0(r27)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C58F8: D0A70000  stfs f5, 0(r7)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C58FC: 7CFC2A14  add r7, r28, r5
	ctx.r[7].u64 = ctx.r[28].u64 + ctx.r[5].u64;
	// 832C5900: D0DA0000  stfs f6, 0(r26)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5904: C0480004  lfs f2, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C5908: 7C3C2C2E  lfsx f1, r28, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C590C: 7C1D2C2E  lfsx f0, r29, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5910: C1A70004  lfs f13, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5914: 7C3D2D2E  stfsx f1, r29, r5
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5918: D1A80004  stfs f13, 4(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C591C: 7C1C2D2E  stfsx f0, r28, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5920: D0470004  stfs f2, 4(r7)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5924: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C5928: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C592C: 7D08F214  add r8, r8, r30
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 832C5930: 3BA8FFFE  addi r29, r8, -2
	ctx.r[29].s64 = ctx.r[8].s64 + -2;
	// 832C5934: 7F871A14  add r28, r7, r3
	ctx.r[28].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 832C5938: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C593C: 5787103A  slwi r7, r28, 2
	ctx.r[7].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C5940: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 832C5944: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 832C5948: 7F5D5A14  add r26, r29, r11
	ctx.r[26].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C594C: 3B680004  addi r27, r8, 4
	ctx.r[27].s64 = ctx.r[8].s64 + 4;
	// 832C5950: 3BA70004  addi r29, r7, 4
	ctx.r[29].s64 = ctx.r[7].s64 + 4;
	// 832C5954: C1880000  lfs f12, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5958: 7F9C5A14  add r28, r28, r11
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 832C595C: C1680004  lfs f11, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C5960: C1470000  lfs f10, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C5964: 579C103A  slwi r28, r28, 2
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C5968: C1270004  lfs f9, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C596C: 575A103A  slwi r26, r26, 2
	ctx.r[26].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 832C5970: D1480000  stfs f10, 0(r8)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5974: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832C5978: D13B0000  stfs f9, 0(r27)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C597C: 7D1A2A14  add r8, r26, r5
	ctx.r[8].u64 = ctx.r[26].u64 + ctx.r[5].u64;
	// 832C5980: D1870000  stfs f12, 0(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5984: 7CFC2A14  add r7, r28, r5
	ctx.r[7].u64 = ctx.r[28].u64 + ctx.r[5].u64;
	// 832C5988: D17D0000  stfs f11, 0(r29)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C598C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 832C5990: 7D1C2C2E  lfsx f8, r28, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C5994: 7CFA2C2E  lfsx f7, r26, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C5998: C0C80004  lfs f6, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C599C: C0A70004  lfs f5, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C59A0: 7D1A2D2E  stfsx f8, r26, r5
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C59A4: D0A80004  stfs f5, 4(r8)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C59A8: 7CFC2D2E  stfsx f7, r28, r5
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C59AC: D0C70004  stfs f6, 4(r7)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C59B0: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C59B4: 7FC7F214  add r30, r7, r30
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 832C59B8: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C59BC: 7FA81A14  add r29, r8, r3
	ctx.r[29].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 832C59C0: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C59C4: 57DC103A  slwi r28, r30, 2
	ctx.r[28].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C59C8: 7CE82A14  add r7, r8, r5
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 832C59CC: 7D1C2A14  add r8, r28, r5
	ctx.r[8].u64 = ctx.r[28].u64 + ctx.r[5].u64;
	// 832C59D0: 3B870004  addi r28, r7, 4
	ctx.r[28].s64 = ctx.r[7].s64 + 4;
	// 832C59D4: 3B680004  addi r27, r8, 4
	ctx.r[27].s64 = ctx.r[8].s64 + 4;
	// 832C59D8: 7FDE5A14  add r30, r30, r11
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 832C59DC: C0870004  lfs f4, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C59E0: 7FBD5A14  add r29, r29, r11
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C59E4: C0680000  lfs f3, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C59E8: 57DE103A  slwi r30, r30, 2
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C59EC: C0470000  lfs f2, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C59F0: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 832C59F4: C0280004  lfs f1, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C59F8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 832C59FC: D0480000  stfs f2, 0(r8)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5A00: 7D1E2A14  add r8, r30, r5
	ctx.r[8].u64 = ctx.r[30].u64 + ctx.r[5].u64;
	// 832C5A04: D09B0000  stfs f4, 0(r27)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5A08: D0670000  stfs f3, 0(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5A0C: 7CFD2A14  add r7, r29, r5
	ctx.r[7].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	// 832C5A10: D03C0000  stfs f1, 0(r28)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5A14: 7DBD2C2E  lfsx f13, r29, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5A18: 7D7E2C2E  lfsx f11, r30, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C5A1C: C1870004  lfs f12, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5A20: C0080004  lfs f0, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5A24: 7DBE2D2E  stfsx f13, r30, r5
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5A28: D1880004  stfs f12, 4(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5A2C: 7D7D2D2E  stfsx f11, r29, r5
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5A30: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5A34: 4082FDE8  bne 0x832c581c
	if !ctx.cr[0].eq {
	pc = 0x832C581C; continue 'dispatch;
	}
	pc = 0x832C5A38; continue 'dispatch;
            }
            0x832C5A38 => {
    //   block [0x832C5A38..0x832C5A50)
	// 832C5A38: 7F19C000  cmpw cr6, r25, r24
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[24].s32, &mut ctx.xer);
	// 832C5A3C: 409800A4  bge cr6, 0x832c5ae0
	if !ctx.cr[6].lt {
	pc = 0x832C5AE0; continue 'dispatch;
	}
	// 832C5A40: 572A103A  slwi r10, r25, 2
	ctx.r[10].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5A44: 573F083C  slwi r31, r25, 1
	ctx.r[31].u32 = ctx.r[25].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 832C5A48: 7CEA2214  add r7, r10, r4
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C5A4C: 7D19C050  subf r8, r25, r24
	ctx.r[8].s64 = ctx.r[24].s64 - ctx.r[25].s64;
	pc = 0x832C5A50; continue 'dispatch;
            }
            0x832C5A50 => {
    //   block [0x832C5A50..0x832C5AE0)
	// 832C5A50: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C5A54: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 832C5A58: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C5A5C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 832C5A60: 7FC35214  add r30, r3, r10
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 832C5A64: 7FA9FA14  add r29, r9, r31
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C5A68: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5A6C: 57BC103A  slwi r28, r29, 2
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C5A70: 7D2A2A14  add r9, r10, r5
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 832C5A74: 7D5C2A14  add r10, r28, r5
	ctx.r[10].u64 = ctx.r[28].u64 + ctx.r[5].u64;
	// 832C5A78: 7F7D5A14  add r27, r29, r11
	ctx.r[27].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 832C5A7C: 3BAA0004  addi r29, r10, 4
	ctx.r[29].s64 = ctx.r[10].s64 + 4;
	// 832C5A80: 577B103A  slwi r27, r27, 2
	ctx.r[27].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 832C5A84: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5A88: 3B890004  addi r28, r9, 4
	ctx.r[28].s64 = ctx.r[9].s64 + 4;
	// 832C5A8C: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5A90: 7FDE5A14  add r30, r30, r11
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 832C5A94: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5A98: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 832C5A9C: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5AA0: 7D5B2A14  add r10, r27, r5
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[5].u64;
	// 832C5AA4: C17D0000  lfs f11, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C5AA8: 57DE103A  slwi r30, r30, 2
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C5AAC: D19D0000  stfs f12, 0(r29)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5AB0: D1A90000  stfs f13, 0(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5AB4: 7D3E2A14  add r9, r30, r5
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[5].u64;
	// 832C5AB8: D17C0000  stfs f11, 0(r28)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5ABC: C14A0004  lfs f10, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C5AC0: 7D3E2C2E  lfsx f9, r30, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C5AC4: 7D1B2C2E  lfsx f8, r27, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C5AC8: C0E90004  lfs f7, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C5ACC: 7D3B2D2E  stfsx f9, r27, r5
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5AD0: D0EA0004  stfs f7, 4(r10)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5AD4: 7D1E2D2E  stfsx f8, r30, r5
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 832C5AD8: D1490004  stfs f10, 4(r9)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5ADC: 4082FF74  bne 0x832c5a50
	if !ctx.cr[0].eq {
	pc = 0x832C5A50; continue 'dispatch;
	}
	pc = 0x832C5AE0; continue 'dispatch;
            }
            0x832C5AE0 => {
    //   block [0x832C5AE0..0x832C5AF4)
	// 832C5AE0: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 832C5AE4: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C5AE8: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 832C5AEC: 7F18A800  cmpw cr6, r24, r21
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[21].s32, &mut ctx.xer);
	// 832C5AF0: 4198FD08  blt cr6, 0x832c57f8
	if ctx.cr[6].lt {
	pc = 0x832C57F8; continue 'dispatch;
	}
	pc = 0x832C5AF4; continue 'dispatch;
            }
            0x832C5AF4 => {
    //   block [0x832C5AF4..0x832C5AF8)
	// 832C5AF4: 4B9E3948  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C5AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C5AF8 size=1776
    let mut pc: u32 = 0x832C5AF8;
    'dispatch: loop {
        match pc {
            0x832C5AF8 => {
    //   block [0x832C5AF8..0x832C5C50)
	// 832C5AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C5AFC: 4B9E38FD  bl 0x82ca93f8
	ctx.lr = 0x832C5B00;
	sub_82CA93D0(ctx, base);
	// 832C5B00: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 832C5B04: 4B9E819D  bl 0x82cadca0
	ctx.lr = 0x832C5B08;
	sub_82CADCA0(ctx, base);
	// 832C5B08: 7C7E1E70  srawi r30, r3, 3
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[3].s32 >> 3) as i64;
	// 832C5B0C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5B10: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5B14: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 832C5B18: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5B1C: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C5B20: 57C82036  slwi r8, r30, 4
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C5B24: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832C5B28: 7D482214  add r10, r8, r4
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 832C5B2C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C5B30: 57C81838  slwi r8, r30, 3
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C5B34: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C5B38: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 832C5B3C: C18A0000  lfs f12, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5B40: 38C79484  addi r6, r7, -0x6b7c
	ctx.r[6].s64 = ctx.r[7].s64 + -27516;
	// 832C5B44: C16A0004  lfs f11, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C5B48: ED40602A  fadds f10, f0, f12
	ctx.f[10].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C5B4C: ECE06028  fsubs f7, f0, f12
	ctx.f[7].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C5B50: 3B7EFFFE  addi r27, r30, -2
	ctx.r[27].s64 = ctx.r[30].s64 + -2;
	// 832C5B54: C1090000  lfs f8, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C5B58: ED2D582A  fadds f9, f13, f11
	ctx.f[9].f64 = ((ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C5B5C: C0C80000  lfs f6, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C5B60: ECAD5828  fsubs f5, f13, f11
	ctx.f[5].f64 = (((ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C5B64: C0890004  lfs f4, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C5B68: EC66402A  fadds f3, f6, f8
	ctx.f[3].f64 = ((ctx.f[6].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C5B6C: C0480004  lfs f2, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C5B70: EC264028  fsubs f1, f6, f8
	ctx.f[1].f64 = (((ctx.f[6].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C5B74: EC02202A  fadds f0, f2, f4
	ctx.f[0].f64 = ((ctx.f[2].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C5B78: C186000C  lfs f12, 0xc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5B7C: ED622028  fsubs f11, f2, f4
	ctx.f[11].f64 = (((ctx.f[2].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C5B80: C1A79484  lfs f13, -0x6b7c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5B84: 3BA50008  addi r29, r5, 8
	ctx.r[29].s64 = ctx.r[5].s64 + 8;
	// 832C5B88: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 832C5B8C: ED03502A  fadds f8, f3, f10
	ctx.f[8].f64 = ((ctx.f[3].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C5B90: D1040000  stfs f8, 0(r4)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5B94: ECCA1828  fsubs f6, f10, f3
	ctx.f[6].f64 = (((ctx.f[10].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C5B98: EC80482A  fadds f4, f0, f9
	ctx.f[4].f64 = ((ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C5B9C: D0840004  stfs f4, 4(r4)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5BA0: EC690028  fsubs f3, f9, f0
	ctx.f[3].f64 = (((ctx.f[9].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C5BA4: D0C80000  stfs f6, 0(r8)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5BA8: D0680004  stfs f3, 4(r8)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5BAC: EC475828  fsubs f2, f7, f11
	ctx.f[2].f64 = (((ctx.f[7].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C5BB0: EC01282A  fadds f0, f1, f5
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C5BB4: D04A0000  stfs f2, 0(r10)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5BB8: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5BBC: ED6B382A  fadds f11, f11, f7
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C5BC0: ED450828  fsubs f10, f5, f1
	ctx.f[10].f64 = (((ctx.f[5].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C5BC4: D1690000  stfs f11, 0(r9)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5BC8: D1490004  stfs f10, 4(r9)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5BCC: FD006090  fmr f8, f12
	ctx.f[8].f64 = ctx.f[12].f64;
	// 832C5BD0: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5BD4: FD606890  fmr f11, f13
	ctx.f[11].f64 = ctx.f[13].f64;
	// 832C5BD8: C125000C  lfs f9, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C5BDC: C1450008  lfs f10, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C5BE0: D121FF1C  stfs f9, -0xe4(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-228 as u32), tmp.u32 ) };
	// 832C5BE4: D001FF20  stfs f0, -0xe0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), tmp.u32 ) };
	// 832C5BE8: 40990368  ble cr6, 0x832c5f50
	if !ctx.cr[6].gt {
	pc = 0x832C5F50; continue 'dispatch;
	}
	// 832C5BEC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5BF0: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 832C5BF4: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 832C5BF8: 38CBFFFC  addi r6, r11, -4
	ctx.r[6].s64 = ctx.r[11].s64 + -4;
	// 832C5BFC: 386BFFFE  addi r3, r11, -2
	ctx.r[3].s64 = ctx.r[11].s64 + -2;
	// 832C5C00: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5C04: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 832C5C08: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C5C0C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C5C10: 3B5BFFFD  addi r26, r27, -3
	ctx.r[26].s64 = ctx.r[27].s64 + -3;
	// 832C5C14: 54C5103A  slwi r5, r6, 2
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C5C18: 547C1838  slwi r28, r3, 3
	ctx.r[28].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C5C1C: 7CEA2214  add r7, r10, r4
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C5C20: 57E31838  slwi r3, r31, 3
	ctx.r[3].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 832C5C24: 7CC92214  add r6, r9, r4
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C5C28: 575AF0BE  srwi r26, r26, 2
	ctx.r[26].u32 = ctx.r[26].u32.wrapping_shr(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 832C5C2C: 7FE82214  add r31, r8, r4
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 832C5C30: 7D052214  add r8, r5, r4
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[4].u64;
	// 832C5C34: 7D3C2214  add r9, r28, r4
	ctx.r[9].u64 = ctx.r[28].u64 + ctx.r[4].u64;
	// 832C5C38: 38A70010  addi r5, r7, 0x10
	ctx.r[5].s64 = ctx.r[7].s64 + 16;
	// 832C5C3C: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 832C5C40: 7C632214  add r3, r3, r4
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 832C5C44: 38E7FFF0  addi r7, r7, -0x10
	ctx.r[7].s64 = ctx.r[7].s64 + -16;
	// 832C5C48: 38C6FFF0  addi r6, r6, -0x10
	ctx.r[6].s64 = ctx.r[6].s64 + -16;
	// 832C5C4C: 3B9A0001  addi r28, r26, 1
	ctx.r[28].s64 = ctx.r[26].s64 + 1;
	pc = 0x832C5C50; continue 'dispatch;
            }
            0x832C5C50 => {
    //   block [0x832C5C50..0x832C5F50)
	// 832C5C50: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 832C5C54: C0C5FFF8  lfs f6, -8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C5C58: C0A3FFFC  lfs f5, -4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C5C5C: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 832C5C60: C3DFFFF8  lfs f30, -8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C5C64: C06AFFFC  lfs f3, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C5C68: EF7E3028  fsubs f27, f30, f6
	ctx.f[27].f64 = (((ctx.f[30].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C5C6C: C0E5FFFC  lfs f7, -4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C5C70: EC432828  fsubs f2, f3, f5
	ctx.f[2].f64 = (((ctx.f[3].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C5C74: C01DFFFC  lfs f0, -4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C5C78: ECA5182A  fadds f5, f5, f3
	ctx.f[5].f64 = ((ctx.f[5].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C5C7C: C083FFF8  lfs f4, -8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C5C80: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C5C84: C03FFFFC  lfs f1, -4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C5C88: ECDE302A  fadds f6, f30, f6
	ctx.f[6].f64 = ((ctx.f[30].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C5C8C: C38AFFF8  lfs f28, -8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C5C90: EFA13828  fsubs f29, f1, f7
	ctx.f[29].f64 = (((ctx.f[1].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C5C94: EF5C2028  fsubs f26, f28, f4
	ctx.f[26].f64 = (((ctx.f[28].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C5C98: C2E30000  lfs f23, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 832C5C9C: EC21382A  fadds f1, f1, f7
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C5CA0: C06A0000  lfs f3, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C5CA4: C2850000  lfs f20, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 832C5CA8: EC84E02A  fadds f4, f4, f28
	ctx.f[4].f64 = ((ctx.f[4].f64 + ctx.f[28].f64) as f32) as f64;
	// 832C5CAC: C25F0000  lfs f18, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 832C5CB0: EEC3B828  fsubs f22, f3, f23
	ctx.f[22].f64 = (((ctx.f[3].f64 - ctx.f[23].f64) as f32) as f64);
	// 832C5CB4: C3250004  lfs f25, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 832C5CB8: EC77182A  fadds f3, f23, f3
	ctx.f[3].f64 = ((ctx.f[23].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C5CBC: C0FF0004  lfs f7, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C5CC0: EDF2A02A  fadds f15, f18, f20
	ctx.f[15].f64 = ((ctx.f[18].f64 + ctx.f[20].f64) as f32) as f64;
	// 832C5CC4: C1BDFFF8  lfs f13, -8(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5CC8: EEA7C828  fsubs f21, f7, f25
	ctx.f[21].f64 = (((ctx.f[7].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C5CCC: C2630004  lfs f19, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 832C5CD0: ECE7C82A  fadds f7, f7, f25
	ctx.f[7].f64 = ((ctx.f[7].f64 + ctx.f[25].f64) as f32) as f64;
	// 832C5CD4: C3CA0004  lfs f30, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C5CD8: EF0C682A  fadds f24, f12, f13
	ctx.f[24].f64 = ((ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C5CDC: EEF3F02A  fadds f23, f19, f30
	ctx.f[23].f64 = ((ctx.f[19].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C5CE0: C19D0000  lfs f12, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5CE4: EFFF02B2  fmuls f31, f31, f10
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C5CE8: C21D0004  lfs f16, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[16].f64 = (tmp.f32 as f64);
	// 832C5CEC: EF9B102A  fadds f28, f27, f2
	ctx.f[28].f64 = ((ctx.f[27].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C5CF0: D1A1FF18  stfs f13, -0xe8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-232 as u32), tmp.u32 ) };
	// 832C5CF4: EE3AE828  fsubs f17, f26, f29
	ctx.f[17].f64 = (((ctx.f[26].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C5CF8: D001FF14  stfs f0, -0xec(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-236 as u32), tmp.u32 ) };
	// 832C5CFC: EFDE9828  fsubs f30, f30, f19
	ctx.f[30].f64 = (((ctx.f[30].f64 - ctx.f[19].f64) as f32) as f64);
	// 832C5D00: D181FF10  stfs f12, -0xf0(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-240 as u32), tmp.u32 ) };
	// 832C5D04: EE92A028  fsubs f20, f18, f20
	ctx.f[20].f64 = (((ctx.f[18].f64 - ctx.f[20].f64) as f32) as f64);
	// 832C5D08: EDC1282A  fadds f14, f1, f5
	ctx.f[14].f64 = ((ctx.f[1].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C5D0C: D1CAFFFC  stfs f14, -4(r10)
	tmp.f32 = (ctx.f[14].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C5D10: EDC6202A  fadds f14, f6, f4
	ctx.f[14].f64 = ((ctx.f[6].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C5D14: D1CAFFF8  stfs f14, -8(r10)
	tmp.f32 = (ctx.f[14].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 832C5D18: EDCF182A  fadds f14, f15, f3
	ctx.f[14].f64 = ((ctx.f[15].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C5D1C: D1CA0000  stfs f14, 0(r10)
	tmp.f32 = (ctx.f[14].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5D20: EF3802B2  fmuls f25, f24, f10
	ctx.f[25].f64 = (((ctx.f[24].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C5D24: EDC7B82A  fadds f14, f7, f23
	ctx.f[14].f64 = ((ctx.f[7].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C5D28: D1CA0004  stfs f14, 4(r10)
	tmp.f32 = (ctx.f[14].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5D2C: EC637828  fsubs f3, f3, f15
	ctx.f[3].f64 = (((ctx.f[3].f64 - ctx.f[15].f64) as f32) as f64);
	// 832C5D30: D07F0000  stfs f3, 0(r31)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5D34: EF1F0732  fmuls f24, f31, f28
	ctx.f[24].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 832C5D38: ED6B8028  fsubs f11, f11, f16
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[16].f64) as f32) as f64);
	// 832C5D3C: ED0C402A  fadds f8, f12, f8
	ctx.f[8].f64 = ((ctx.f[12].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C5D40: EE7F0472  fmuls f19, f31, f17
	ctx.f[19].f64 = (((ctx.f[31].f64 * ctx.f[17].f64) as f32) as f64);
	// 832C5D44: EE56A828  fsubs f18, f22, f21
	ctx.f[18].f64 = (((ctx.f[22].f64 - ctx.f[21].f64) as f32) as f64);
	// 832C5D48: EC74F02A  fadds f3, f20, f30
	ctx.f[3].f64 = ((ctx.f[20].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C5D4C: ECA50828  fsubs f5, f5, f1
	ctx.f[5].f64 = (((ctx.f[5].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C5D50: D0BFFFFC  stfs f5, -4(r31)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C5D54: EC373828  fsubs f1, f23, f7
	ctx.f[1].f64 = (((ctx.f[23].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C5D58: D03F0004  stfs f1, 4(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5D5C: EC843028  fsubs f4, f4, f6
	ctx.f[4].f64 = (((ctx.f[4].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C5D60: D09FFFF8  stfs f4, -8(r31)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 832C5D64: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 832C5D68: ECF9C478  fmsubs f7, f25, f17, f24
	ctx.f[7].f64 = (((ctx.f[25].f64 * ctx.f[17].f64 - ctx.f[24].f64) as f32) as f64);
	// 832C5D6C: D0E3FFF8  stfs f7, -8(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 832C5D70: EC8B0272  fmuls f4, f11, f9
	ctx.f[4].f64 = (((ctx.f[11].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C5D74: ECA80272  fmuls f5, f8, f9
	ctx.f[5].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C5D78: ECD99F3A  fmadds f6, f25, f28, f19
	ctx.f[6].f64 = (((ctx.f[25].f64 * ctx.f[28].f64 + ctx.f[19].f64) as f32) as f64);
	// 832C5D7C: D0C3FFFC  stfs f6, -4(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C5D80: FD608050  fneg f11, f16
	ctx.f[11].u64 = ctx.f[16].u64 ^ 0x8000_0000_0000_0000u64;
	// 832C5D84: EC2004B2  fmuls f1, f0, f18
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[18].f64) as f32) as f64);
	// 832C5D88: ED3DD02A  fadds f9, f29, f26
	ctx.f[9].f64 = ((ctx.f[29].f64 + ctx.f[26].f64) as f32) as f64;
	// 832C5D8C: ED15B02A  fadds f8, f21, f22
	ctx.f[8].f64 = ((ctx.f[21].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C5D90: ECE000F2  fmuls f7, f0, f3
	ctx.f[7].f64 = (((ctx.f[0].f64 * ctx.f[3].f64) as f32) as f64);
	// 832C5D94: EC2D08FA  fmadds f1, f13, f3, f1
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[3].f64 + ctx.f[1].f64) as f32) as f64);
	// 832C5D98: D0230004  stfs f1, 4(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5D9C: EC650272  fmuls f3, f5, f9
	ctx.f[3].f64 = (((ctx.f[5].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C5DA0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 832C5DA4: EC240272  fmuls f1, f4, f9
	ctx.f[1].f64 = (((ctx.f[4].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C5DA8: ECC2D828  fsubs f6, f2, f27
	ctx.f[6].f64 = (((ctx.f[2].f64 - ctx.f[27].f64) as f32) as f64);
	// 832C5DAC: ED2C0232  fmuls f9, f12, f8
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C5DB0: EC5EA028  fsubs f2, f30, f20
	ctx.f[2].f64 = (((ctx.f[30].f64 - ctx.f[20].f64) as f32) as f64);
	// 832C5DB4: ED0B0232  fmuls f8, f11, f8
	ctx.f[8].f64 = (((ctx.f[11].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C5DB8: ECED3CB8  fmsubs f7, f13, f18, f7
	ctx.f[7].f64 = (((ctx.f[13].f64 * ctx.f[18].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C5DBC: D0E30000  stfs f7, 0(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5DC0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 832C5DC4: EC6419BA  fmadds f3, f4, f6, f3
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[6].f64 + ctx.f[3].f64) as f32) as f64);
	// 832C5DC8: D065FFF8  stfs f3, -8(r5)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 832C5DCC: EC2509B8  fmsubs f1, f5, f6, f1
	ctx.f[1].f64 = (((ctx.f[5].f64 * ctx.f[6].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C5DD0: D025FFFC  stfs f1, -4(r5)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C5DD4: ED2B48BA  fmadds f9, f11, f2, f9
	ctx.f[9].f64 = (((ctx.f[11].f64 * ctx.f[2].f64 + ctx.f[9].f64) as f32) as f64);
	// 832C5DD8: D1250000  stfs f9, 0(r5)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5DDC: ED0C40B8  fmsubs f8, f12, f2, f8
	ctx.f[8].f64 = (((ctx.f[12].f64 * ctx.f[2].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C5DE0: D1050004  stfs f8, 4(r5)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5DE4: C0C60008  lfs f6, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C5DE8: 38A50010  addi r5, r5, 0x10
	ctx.r[5].s64 = ctx.r[5].s64 + 16;
	// 832C5DEC: C066000C  lfs f3, 0xc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C5DF0: C2C70008  lfs f22, 8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 832C5DF4: C129000C  lfs f9, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C5DF8: C2A80008  lfs f21, 8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 832C5DFC: C0290008  lfs f1, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C5E00: C1070000  lfs f8, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C5E04: C3A60000  lfs f29, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 832C5E08: C3690000  lfs f27, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 832C5E0C: C3080000  lfs f24, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 832C5E10: C3C70004  lfs f30, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C5E14: C3860004  lfs f28, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C5E18: C3490004  lfs f26, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 832C5E1C: C2E80004  lfs f23, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 832C5E20: C0E7000C  lfs f7, 0xc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C5E24: C048000C  lfs f2, 0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C5E28: EE423828  fsubs f18, f2, f7
	ctx.f[18].f64 = (((ctx.f[2].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C5E2C: EE213028  fsubs f17, f1, f6
	ctx.f[17].f64 = (((ctx.f[1].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C5E30: EE691828  fsubs f19, f9, f3
	ctx.f[19].f64 = (((ctx.f[9].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C5E34: EE95B028  fsubs f20, f21, f22
	ctx.f[20].f64 = (((ctx.f[21].f64 - ctx.f[22].f64) as f32) as f64);
	// 832C5E38: EC63482A  fadds f3, f3, f9
	ctx.f[3].f64 = ((ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C5E3C: ECE2382A  fadds f7, f2, f7
	ctx.f[7].f64 = ((ctx.f[2].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C5E40: ED3DD82A  fadds f9, f29, f27
	ctx.f[9].f64 = ((ctx.f[29].f64 + ctx.f[27].f64) as f32) as f64;
	// 832C5E44: EC48C02A  fadds f2, f8, f24
	ctx.f[2].f64 = ((ctx.f[8].f64 + ctx.f[24].f64) as f32) as f64;
	// 832C5E48: EC26082A  fadds f1, f6, f1
	ctx.f[1].f64 = ((ctx.f[6].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C5E4C: EED5B02A  fadds f22, f21, f22
	ctx.f[22].f64 = ((ctx.f[21].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C5E50: ECD7F02A  fadds f6, f23, f30
	ctx.f[6].f64 = ((ctx.f[23].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C5E54: EE1CD02A  fadds f16, f28, f26
	ctx.f[16].f64 = ((ctx.f[28].f64 + ctx.f[26].f64) as f32) as f64;
	// 832C5E58: EFBBE828  fsubs f29, f27, f29
	ctx.f[29].f64 = (((ctx.f[27].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C5E5C: ED184028  fsubs f8, f24, f8
	ctx.f[8].f64 = (((ctx.f[24].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C5E60: EF9AE028  fsubs f28, f26, f28
	ctx.f[28].f64 = (((ctx.f[26].f64 - ctx.f[28].f64) as f32) as f64);
	// 832C5E64: EFD7F028  fsubs f30, f23, f30
	ctx.f[30].f64 = (((ctx.f[23].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C5E68: EEB49828  fsubs f21, f20, f19
	ctx.f[21].f64 = (((ctx.f[20].f64 - ctx.f[19].f64) as f32) as f64);
	// 832C5E6C: EDF1902A  fadds f15, f17, f18
	ctx.f[15].f64 = ((ctx.f[17].f64 + ctx.f[18].f64) as f32) as f64;
	// 832C5E70: EF63382A  fadds f27, f3, f7
	ctx.f[27].f64 = ((ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C5E74: D368000C  stfs f27, 0xc(r8)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C5E78: EF69102A  fadds f27, f9, f2
	ctx.f[27].f64 = ((ctx.f[9].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C5E7C: D3680000  stfs f27, 0(r8)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5E80: EF61B02A  fadds f27, f1, f22
	ctx.f[27].f64 = ((ctx.f[1].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C5E84: D3680008  stfs f27, 8(r8)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C5E88: ECE71828  fsubs f7, f7, f3
	ctx.f[7].f64 = (((ctx.f[7].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C5E8C: EC624828  fsubs f3, f2, f9
	ctx.f[3].f64 = (((ctx.f[2].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C5E90: EF70302A  fadds f27, f16, f6
	ctx.f[27].f64 = ((ctx.f[16].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C5E94: D3680004  stfs f27, 4(r8)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5E98: EC468028  fsubs f2, f6, f16
	ctx.f[2].f64 = (((ctx.f[6].f64 - ctx.f[16].f64) as f32) as f64);
	// 832C5E9C: D0E9000C  stfs f7, 0xc(r9)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C5EA0: D0490004  stfs f2, 4(r9)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5EA4: EC360828  fsubs f1, f22, f1
	ctx.f[1].f64 = (((ctx.f[22].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C5EA8: D0290008  stfs f1, 8(r9)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C5EAC: ECFDF02A  fadds f7, f29, f30
	ctx.f[7].f64 = ((ctx.f[29].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C5EB0: D0690000  stfs f3, 0(r9)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5EB4: ECC8E028  fsubs f6, f8, f28
	ctx.f[6].f64 = (((ctx.f[8].f64 - ctx.f[28].f64) as f32) as f64);
	// 832C5EB8: EC5C402A  fadds f2, f28, f8
	ctx.f[2].f64 = ((ctx.f[28].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C5EBC: 3929FFF0  addi r9, r9, -0x10
	ctx.r[9].s64 = ctx.r[9].s64 + -16;
	// 832C5EC0: EC390572  fmuls f1, f25, f21
	ctx.f[1].f64 = (((ctx.f[25].f64 * ctx.f[21].f64) as f32) as f64);
	// 832C5EC4: 3908FFF0  addi r8, r8, -0x10
	ctx.r[8].s64 = ctx.r[8].s64 + -16;
	// 832C5EC8: ED3903F2  fmuls f9, f25, f15
	ctx.f[9].f64 = (((ctx.f[25].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C5ECC: EC73A02A  fadds f3, f19, f20
	ctx.f[3].f64 = ((ctx.f[19].f64 + ctx.f[20].f64) as f32) as f64;
	// 832C5ED0: ED128828  fsubs f8, f18, f17
	ctx.f[8].f64 = (((ctx.f[18].f64 - ctx.f[17].f64) as f32) as f64);
	// 832C5ED4: EFDEE828  fsubs f30, f30, f29
	ctx.f[30].f64 = (((ctx.f[30].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C5ED8: EC3F0BFA  fmadds f1, f31, f15, f1
	ctx.f[1].f64 = (((ctx.f[31].f64 * ctx.f[15].f64 + ctx.f[1].f64) as f32) as f64);
	// 832C5EDC: D027000C  stfs f1, 0xc(r7)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C5EE0: ED3F4D78  fmsubs f9, f31, f21, f9
	ctx.f[9].f64 = (((ctx.f[31].f64 * ctx.f[21].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C5EE4: D1270008  stfs f9, 8(r7)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C5EE8: EC2D01F2  fmuls f1, f13, f7
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C5EEC: EDAD01B2  fmuls f13, f13, f6
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C5EF0: ED2400F2  fmuls f9, f4, f3
	ctx.f[9].f64 = (((ctx.f[4].f64 * ctx.f[3].f64) as f32) as f64);
	// 832C5EF4: EC6500F2  fmuls f3, f5, f3
	ctx.f[3].f64 = (((ctx.f[5].f64 * ctx.f[3].f64) as f32) as f64);
	// 832C5EF8: EFEB00B2  fmuls f31, f11, f2
	ctx.f[31].f64 = (((ctx.f[11].f64 * ctx.f[2].f64) as f32) as f64);
	// 832C5EFC: EC4C00B2  fmuls f2, f12, f2
	ctx.f[2].f64 = (((ctx.f[12].f64 * ctx.f[2].f64) as f32) as f64);
	// 832C5F00: EC2009B8  fmsubs f1, f0, f6, f1
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[6].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C5F04: D0270000  stfs f1, 0(r7)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5F08: EC0069FA  fmadds f0, f0, f7, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64);
	// 832C5F0C: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5F10: EDA54A3A  fmadds f13, f5, f8, f9
	ctx.f[13].f64 = (((ctx.f[5].f64 * ctx.f[8].f64 + ctx.f[9].f64) as f32) as f64);
	// 832C5F14: D1A60008  stfs f13, 8(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C5F18: ED241A38  fmsubs f9, f4, f8, f3
	ctx.f[9].f64 = (((ctx.f[4].f64 * ctx.f[8].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C5F1C: D126000C  stfs f9, 0xc(r6)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C5F20: ED0CFFBA  fmadds f8, f12, f30, f31
	ctx.f[8].f64 = (((ctx.f[12].f64 * ctx.f[30].f64 + ctx.f[31].f64) as f32) as f64);
	// 832C5F24: D1060000  stfs f8, 0(r6)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C5F28: ECEB17B8  fmsubs f7, f11, f30, f2
	ctx.f[7].f64 = (((ctx.f[11].f64 * ctx.f[30].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C5F2C: D0E60004  stfs f7, 4(r6)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C5F30: FD006090  fmr f8, f12
	ctx.f[8].f64 = ctx.f[12].f64;
	// 832C5F34: C1A1FF14  lfs f13, -0xec(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-236 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5F38: C181FF18  lfs f12, -0xe8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-232 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5F3C: 38E7FFF0  addi r7, r7, -0x10
	ctx.r[7].s64 = ctx.r[7].s64 + -16;
	// 832C5F40: C121FF1C  lfs f9, -0xe4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-228 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C5F44: 38C6FFF0  addi r6, r6, -0x10
	ctx.r[6].s64 = ctx.r[6].s64 + -16;
	// 832C5F48: 4082FD08  bne 0x832c5c50
	if !ctx.cr[0].eq {
	pc = 0x832C5C50; continue 'dispatch;
	}
	// 832C5F4C: C001FF20  lfs f0, -0xe0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x832C5F50; continue 'dispatch;
            }
            0x832C5F50 => {
    //   block [0x832C5F50..0x832C61E8)
	// 832C5F50: 7CEBF214  add r7, r11, r30
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C5F54: EDAD002A  fadds f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C5F58: ECEB0028  fsubs f7, f11, f0
	ctx.f[7].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C5F5C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C5F60: 7CC75A14  add r6, r7, r11
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 832C5F64: ED080028  fsubs f8, f8, f0
	ctx.f[8].f64 = (((ctx.f[8].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C5F68: ECCC002A  fadds f6, f12, f0
	ctx.f[6].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C5F6C: 54FA103A  slwi r26, r7, 2
	ctx.r[26].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 832C5F70: 7CA65A14  add r5, r6, r11
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 832C5F74: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C5F78: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C5F7C: 7D6A2214  add r11, r10, r4
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C5F80: 3BE7FFFE  addi r31, r7, -2
	ctx.r[31].s64 = ctx.r[7].s64 + -2;
	// 832C5F84: 3865FFFE  addi r3, r5, -2
	ctx.r[3].s64 = ctx.r[5].s64 + -2;
	// 832C5F88: 7D492214  add r10, r9, r4
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C5F8C: ECAD02B2  fmuls f5, f13, f10
	ctx.f[5].f64 = (((ctx.f[13].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C5F90: 7D282214  add r9, r8, r4
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 832C5F94: EC470272  fmuls f2, f7, f9
	ctx.f[2].f64 = (((ctx.f[7].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C5F98: 3B86FFFE  addi r28, r6, -2
	ctx.r[28].s64 = ctx.r[6].s64 + -2;
	// 832C5F9C: C06BFFFC  lfs f3, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C5FA0: 7D1A2214  add r8, r26, r4
	ctx.r[8].u64 = ctx.r[26].u64 + ctx.r[4].u64;
	// 832C5FA4: EC880272  fmuls f4, f8, f9
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C5FA8: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 832C5FAC: EC2602B2  fmuls f1, f6, f10
	ctx.f[1].f64 = (((ctx.f[6].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C5FB0: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 832C5FB4: C1AAFFFC  lfs f13, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C5FB8: 577D103A  slwi r29, r27, 2
	ctx.r[29].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 832C5FBC: C189FFFC  lfs f12, -4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C5FC0: 579C103A  slwi r28, r28, 2
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C5FC4: ECC3682A  fadds f6, f3, f13
	ctx.f[6].f64 = ((ctx.f[3].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C5FC8: C168FFFC  lfs f11, -4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C5FCC: ED436828  fsubs f10, f3, f13
	ctx.f[10].f64 = (((ctx.f[3].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C5FD0: 7D23242E  lfsx f9, r3, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C5FD4: ED0B6028  fsubs f8, f11, f12
	ctx.f[8].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C5FD8: 7CFF242E  lfsx f7, r31, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C5FDC: ED8B602A  fadds f12, f11, f12
	ctx.f[12].f64 = ((ctx.f[11].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C5FE0: 7C7D242E  lfsx f3, r29, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C5FE4: EDA74828  fsubs f13, f7, f9
	ctx.f[13].f64 = (((ctx.f[7].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C5FE8: 7FFC242E  lfsx f31, r28, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C5FEC: ED27482A  fadds f9, f7, f9
	ctx.f[9].f64 = ((ctx.f[7].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C5FF0: ED63F828  fsubs f11, f3, f31
	ctx.f[11].f64 = (((ctx.f[3].f64 - ctx.f[31].f64) as f32) as f64);
	// 832C5FF4: 3B650002  addi r27, r5, 2
	ctx.r[27].s64 = ctx.r[5].s64 + 2;
	// 832C5FF8: ECE3F82A  fadds f7, f3, f31
	ctx.f[7].f64 = ((ctx.f[3].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C5FFC: 3B270002  addi r25, r7, 2
	ctx.r[25].s64 = ctx.r[7].s64 + 2;
	// 832C6000: EC6D502A  fadds f3, f13, f10
	ctx.f[3].f64 = ((ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6004: EDAA6828  fsubs f13, f10, f13
	ctx.f[13].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C6008: ED4B4028  fsubs f10, f11, f8
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C600C: ED08582A  fadds f8, f8, f11
	ctx.f[8].f64 = ((ctx.f[8].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C6010: ED6C302A  fadds f11, f12, f6
	ctx.f[11].f64 = ((ctx.f[12].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C6014: D16BFFFC  stfs f11, -4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C6018: ED69382A  fadds f11, f9, f7
	ctx.f[11].f64 = ((ctx.f[9].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C601C: 7D7D252E  stfsx f11, r29, r4
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6020: ECC66028  fsubs f6, f6, f12
	ctx.f[6].f64 = (((ctx.f[6].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C6024: D0C8FFFC  stfs f6, -4(r8)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C6028: ED874828  fsubs f12, f7, f9
	ctx.f[12].f64 = (((ctx.f[7].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C602C: 7D9F252E  stfsx f12, r31, r4
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6030: ED6500F2  fmuls f11, f5, f3
	ctx.f[11].f64 = (((ctx.f[5].f64 * ctx.f[3].f64) as f32) as f64);
	// 832C6034: ED2502B2  fmuls f9, f5, f10
	ctx.f[9].f64 = (((ctx.f[5].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C6038: ECE40232  fmuls f7, f4, f8
	ctx.f[7].f64 = (((ctx.f[4].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C603C: ECC20232  fmuls f6, f2, f8
	ctx.f[6].f64 = (((ctx.f[2].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C6040: ED815AB8  fmsubs f12, f1, f10, f11
	ctx.f[12].f64 = (((ctx.f[1].f64 * ctx.f[10].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C6044: 7D9C252E  stfsx f12, r28, r4
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6048: ED6148FA  fmadds f11, f1, f3, f9
	ctx.f[11].f64 = (((ctx.f[1].f64 * ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64);
	// 832C604C: D16AFFFC  stfs f11, -4(r10)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C6050: ED423B7A  fmadds f10, f2, f13, f7
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[13].f64 + ctx.f[7].f64) as f32) as f64);
	// 832C6054: 7D43252E  stfsx f10, r3, r4
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6058: ED243378  fmsubs f9, f4, f13, f6
	ctx.f[9].f64 = (((ctx.f[4].f64 * ctx.f[13].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C605C: D129FFFC  stfs f9, -4(r9)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C6060: C10B0004  lfs f8, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C6064: C0EA0004  lfs f7, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C6068: C0C90000  lfs f6, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C606C: 7C7A242E  lfsx f3, r26, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C6070: C18A0000  lfs f12, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6074: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6078: ED6D602A  fadds f11, f13, f12
	ctx.f[11].f64 = ((ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C607C: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C6080: C1480004  lfs f10, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C6084: ED28382A  fadds f9, f8, f7
	ctx.f[9].f64 = ((ctx.f[8].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C6088: C3E90004  lfs f31, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C608C: ED883828  fsubs f12, f8, f7
	ctx.f[12].f64 = (((ctx.f[8].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C6090: ED033028  fsubs f8, f3, f6
	ctx.f[8].f64 = (((ctx.f[3].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C6094: ECEAF828  fsubs f7, f10, f31
	ctx.f[7].f64 = (((ctx.f[10].f64 - ctx.f[31].f64) as f32) as f64);
	// 832C6098: 3BE50003  addi r31, r5, 3
	ctx.r[31].s64 = ctx.r[5].s64 + 3;
	// 832C609C: EFC8602A  fadds f30, f8, f12
	ctx.f[30].f64 = ((ctx.f[8].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C60A0: 3BA70003  addi r29, r7, 3
	ctx.r[29].s64 = ctx.r[7].s64 + 3;
	// 832C60A4: ED8C4028  fsubs f12, f12, f8
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C60A8: 387E0003  addi r3, r30, 3
	ctx.r[3].s64 = ctx.r[30].s64 + 3;
	// 832C60AC: ED03302A  fadds f8, f3, f6
	ctx.f[8].f64 = ((ctx.f[3].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C60B0: 3B060002  addi r24, r6, 2
	ctx.r[24].s64 = ctx.r[6].s64 + 2;
	// 832C60B4: ECCAF82A  fadds f6, f10, f31
	ctx.f[6].f64 = ((ctx.f[10].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C60B8: 5767103A  slwi r7, r27, 2
	ctx.r[7].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C60BC: FC600050  fneg f3, f0
	ctx.f[3].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 832C60C0: 5725103A  slwi r5, r25, 2
	ctx.r[5].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C60C4: 3B860003  addi r28, r6, 3
	ctx.r[28].s64 = ctx.r[6].s64 + 3;
	// 832C60C8: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 832C60CC: 5786103A  slwi r6, r28, 2
	ctx.r[6].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832C60D0: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 832C60D4: ED4D3828  fsubs f10, f13, f7
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C60D8: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 832C60DC: ECE7682A  fadds f7, f7, f13
	ctx.f[7].f64 = ((ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C60E0: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 832C60E4: EDA8582A  fadds f13, f8, f11
	ctx.f[13].f64 = ((ctx.f[8].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C60E8: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C60EC: EDA6482A  fadds f13, f6, f9
	ctx.f[13].f64 = ((ctx.f[6].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C60F0: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C60F4: ED293028  fsubs f9, f9, f6
	ctx.f[9].f64 = (((ctx.f[9].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C60F8: D1280004  stfs f9, 4(r8)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C60FC: ED6B4028  fsubs f11, f11, f8
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C6100: 7D7A252E  stfsx f11, r26, r4
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6104: 5708103A  slwi r8, r24, 2
	ctx.r[8].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C6108: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C610C: ED0AF028  fsubs f8, f10, f30
	ctx.f[8].f64 = (((ctx.f[10].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C6110: EDAC382A  fadds f13, f12, f7
	ctx.f[13].f64 = ((ctx.f[12].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C6114: ECDE502A  fadds f6, f30, f10
	ctx.f[6].f64 = ((ctx.f[30].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6118: ED8C3828  fsubs f12, f12, f7
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C611C: ED680032  fmuls f11, f8, f0
	ctx.f[11].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C6120: D16A0000  stfs f11, 0(r10)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6124: ED2D00F2  fmuls f9, f13, f3
	ctx.f[9].f64 = (((ctx.f[13].f64 * ctx.f[3].f64) as f32) as f64);
	// 832C6128: ED460032  fmuls f10, f6, f0
	ctx.f[10].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C612C: D14A0004  stfs f10, 4(r10)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6130: ED0C00F2  fmuls f8, f12, f3
	ctx.f[8].f64 = (((ctx.f[12].f64 * ctx.f[3].f64) as f32) as f64);
	// 832C6134: D1290000  stfs f9, 0(r9)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6138: D1090004  stfs f8, 4(r9)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C613C: 7DBD242E  lfsx f13, r29, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6140: 7D63242E  lfsx f11, r3, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C6144: 7CE7242E  lfsx f7, r7, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C6148: 7D26242E  lfsx f9, r6, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C614C: 7C05242E  lfsx f0, r5, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C6150: ED00382A  fadds f8, f0, f7
	ctx.f[8].f64 = ((ctx.f[0].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C6154: 7C6B242E  lfsx f3, r11, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C6158: ED803828  fsubs f12, f0, f7
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C615C: 7CE8242E  lfsx f7, r8, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C6160: 7CDF242E  lfsx f6, r31, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C6164: ED4D3028  fsubs f10, f13, f6
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C6168: ECCD302A  fadds f6, f13, f6
	ctx.f[6].f64 = ((ctx.f[13].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C616C: EC0B4828  fsubs f0, f11, f9
	ctx.f[0].f64 = (((ctx.f[11].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C6170: EFE33828  fsubs f31, f3, f7
	ctx.f[31].f64 = (((ctx.f[3].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C6174: ECE3382A  fadds f7, f3, f7
	ctx.f[7].f64 = ((ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C6178: EC6B482A  fadds f3, f11, f9
	ctx.f[3].f64 = ((ctx.f[11].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C617C: EFCC002A  fadds f30, f12, f0
	ctx.f[30].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C6180: EC006028  fsubs f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C6184: EFBF5028  fsubs f29, f31, f10
	ctx.f[29].f64 = (((ctx.f[31].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C6188: ED4AF82A  fadds f10, f10, f31
	ctx.f[10].f64 = ((ctx.f[10].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C618C: ED28382A  fadds f9, f8, f7
	ctx.f[9].f64 = ((ctx.f[8].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C6190: 7D2B252E  stfsx f9, r11, r4
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6194: ED274028  fsubs f9, f7, f8
	ctx.f[9].f64 = (((ctx.f[7].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C6198: ED033028  fsubs f8, f3, f6
	ctx.f[8].f64 = (((ctx.f[3].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C619C: EDA107B2  fmuls f13, f1, f30
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[30].f64) as f32) as f64);
	// 832C61A0: ED810772  fmuls f12, f1, f29
	ctx.f[12].f64 = (((ctx.f[1].f64 * ctx.f[29].f64) as f32) as f64);
	// 832C61A4: ED6202B2  fmuls f11, f2, f10
	ctx.f[11].f64 = (((ctx.f[2].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C61A8: ED4402B2  fmuls f10, f4, f10
	ctx.f[10].f64 = (((ctx.f[4].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C61AC: EC26182A  fadds f1, f6, f3
	ctx.f[1].f64 = ((ctx.f[6].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C61B0: 7C23252E  stfsx f1, r3, r4
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C61B4: 7D25252E  stfsx f9, r5, r4
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C61B8: 7D1D252E  stfsx f8, r29, r4
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C61BC: ECE56F78  fmsubs f7, f5, f29, f13
	ctx.f[7].f64 = (((ctx.f[5].f64 * ctx.f[29].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C61C0: 7CE8252E  stfsx f7, r8, r4
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C61C4: ECC567BA  fmadds f6, f5, f30, f12
	ctx.f[6].f64 = (((ctx.f[5].f64 * ctx.f[30].f64 + ctx.f[12].f64) as f32) as f64);
	// 832C61C8: 7CC6252E  stfsx f6, r6, r4
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C61CC: ECA4583A  fmadds f5, f4, f0, f11
	ctx.f[5].f64 = (((ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 832C61D0: 7CA7252E  stfsx f5, r7, r4
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C61D4: EC825038  fmsubs f4, f2, f0, f10
	ctx.f[4].f64 = (((ctx.f[2].f64 * ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C61D8: 7C9F252E  stfsx f4, r31, r4
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C61DC: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 832C61E0: 4B9E7B0D  bl 0x82cadcec
	ctx.lr = 0x832C61E4;
	sub_82CADCEC(ctx, base);
	// 832C61E4: 4B9E3264  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C61E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C61E8 size=216
    let mut pc: u32 = 0x832C61E8;
    'dispatch: loop {
        match pc {
            0x832C61E8 => {
    //   block [0x832C61E8..0x832C6224)
	// 832C61E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C61EC: 4B9E3219  bl 0x82ca9404
	ctx.lr = 0x832C61F0;
	sub_82CA93D0(ctx, base);
	// 832C61F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C61F4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 832C61F8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 832C61FC: 7F7F1670  srawi r31, r27, 2
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[27].s32 >> 2) as i64;
	// 832C6200: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 832C6204: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6208: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832C620C: 7D4BE850  subf r10, r11, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 832C6210: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6214: 7CABE214  add r5, r11, r28
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 832C6218: 48000601  bl 0x832c6818
	ctx.lr = 0x832C621C;
	sub_832C6818(ctx, base);
	// 832C621C: 2F1B0200  cmpwi cr6, r27, 0x200
	ctx.cr[6].compare_i32(ctx.r[27].s32, 512, &mut ctx.xer);
	// 832C6220: 40990084  ble cr6, 0x832c62a4
	if !ctx.cr[6].gt {
	pc = 0x832C62A4; continue 'dispatch;
	}
	pc = 0x832C6224; continue 'dispatch;
            }
            0x832C6224 => {
    //   block [0x832C6224..0x832C62A4)
	// 832C6224: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832C6228: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 832C622C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 832C6230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6234: 4BFFFFB5  bl 0x832c61e8
	ctx.lr = 0x832C6238;
	sub_832C61E8(ctx, base);
	// 832C6238: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C623C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 832C6240: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C6244: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 832C6248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C624C: 48000075  bl 0x832c62c0
	ctx.lr = 0x832C6250;
	sub_832C62C0(ctx, base);
	// 832C6250: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6254: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 832C6258: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C625C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 832C6260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6264: 4BFFFF85  bl 0x832c61e8
	ctx.lr = 0x832C6268;
	sub_832C61E8(ctx, base);
	// 832C6268: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C626C: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 832C6270: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 832C6274: 7FFF1670  srawi r31, r31, 2
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 2) as i64;
	// 832C6278: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C627C: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6280: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C6284: 7D2AE850  subf r9, r10, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[10].s64;
	// 832C6288: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832C628C: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6290: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832C6294: 7CABE214  add r5, r11, r28
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 832C6298: 48000581  bl 0x832c6818
	ctx.lr = 0x832C629C;
	sub_832C6818(ctx, base);
	// 832C629C: 2F1B0200  cmpwi cr6, r27, 0x200
	ctx.cr[6].compare_i32(ctx.r[27].s32, 512, &mut ctx.xer);
	// 832C62A0: 4199FF84  bgt cr6, 0x832c6224
	if ctx.cr[6].gt {
	pc = 0x832C6224; continue 'dispatch;
	}
	pc = 0x832C62A4; continue 'dispatch;
            }
            0x832C62A4 => {
    //   block [0x832C62A4..0x832C62C0)
	// 832C62A4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 832C62A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 832C62AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832C62B0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832C62B4: 480000DD  bl 0x832c6390
	ctx.lr = 0x832C62B8;
	sub_832C6390(ctx, base);
	// 832C62B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832C62BC: 4B9E3198  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C62C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C62C0 size=208
    let mut pc: u32 = 0x832C62C0;
    'dispatch: loop {
        match pc {
            0x832C62C0 => {
    //   block [0x832C62C0..0x832C62F8)
	// 832C62C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C62C4: 4B9E3141  bl 0x82ca9404
	ctx.lr = 0x832C62C8;
	sub_82CA93D0(ctx, base);
	// 832C62C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C62CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832C62D0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 832C62D4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 832C62D8: 7D7DE050  subf r11, r29, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 832C62DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832C62E0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C62E4: 7FBF1670  srawi r31, r29, 2
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[29].s32 >> 2) as i64;
	// 832C62E8: 7CABDA14  add r5, r11, r27
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 832C62EC: 480008B5  bl 0x832c6ba0
	ctx.lr = 0x832C62F0;
	sub_832C6BA0(ctx, base);
	// 832C62F0: 2F1D0200  cmpwi cr6, r29, 0x200
	ctx.cr[6].compare_i32(ctx.r[29].s32, 512, &mut ctx.xer);
	// 832C62F4: 40990080  ble cr6, 0x832c6374
	if !ctx.cr[6].gt {
	pc = 0x832C6374; continue 'dispatch;
	}
	pc = 0x832C62F8; continue 'dispatch;
            }
            0x832C62F8 => {
    //   block [0x832C62F8..0x832C6374)
	// 832C62F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832C62FC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832C6300: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C6304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6308: 4BFFFEE1  bl 0x832c61e8
	ctx.lr = 0x832C630C;
	sub_832C61E8(ctx, base);
	// 832C630C: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6310: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832C6314: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C6318: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C631C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6320: 4BFFFFA1  bl 0x832c62c0
	ctx.lr = 0x832C6324;
	sub_832C62C0(ctx, base);
	// 832C6324: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6328: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832C632C: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C6330: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C6334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6338: 4BFFFEB1  bl 0x832c61e8
	ctx.lr = 0x832C633C;
	sub_832C61E8(ctx, base);
	// 832C633C: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6340: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 832C6344: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 832C6348: 7D3DE050  subf r9, r29, r28
	ctx.r[9].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 832C634C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6350: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6354: 7FCAF214  add r30, r10, r30
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 832C6358: 7CABDA14  add r5, r11, r27
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 832C635C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832C6360: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832C6364: 7FFF1670  srawi r31, r31, 2
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 2) as i64;
	// 832C6368: 48000839  bl 0x832c6ba0
	ctx.lr = 0x832C636C;
	sub_832C6BA0(ctx, base);
	// 832C636C: 2F1D0200  cmpwi cr6, r29, 0x200
	ctx.cr[6].compare_i32(ctx.r[29].s32, 512, &mut ctx.xer);
	// 832C6370: 4199FF88  bgt cr6, 0x832c62f8
	if ctx.cr[6].gt {
	pc = 0x832C62F8; continue 'dispatch;
	}
	pc = 0x832C6374; continue 'dispatch;
            }
            0x832C6374 => {
    //   block [0x832C6374..0x832C6390)
	// 832C6374: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832C6378: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832C637C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832C6380: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832C6384: 4800023D  bl 0x832c65c0
	ctx.lr = 0x832C6388;
	sub_832C65C0(ctx, base);
	// 832C6388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832C638C: 4B9E30C8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C6390 size=556
    let mut pc: u32 = 0x832C6390;
    'dispatch: loop {
        match pc {
            0x832C6390 => {
    //   block [0x832C6390..0x832C63B8)
	// 832C6390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C6394: 4B9E304D  bl 0x82ca93e0
	ctx.lr = 0x832C6398;
	sub_82CA93D0(ctx, base);
	// 832C6398: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C639C: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 832C63A0: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 832C63A4: 7E5F1670  srawi r31, r18, 2
	ctx.xer.ca = (ctx.r[18].s32 < 0) && ((ctx.r[18].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[18].s32 >> 2) as i64;
	// 832C63A8: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 832C63AC: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 832C63B0: 2F1F0080  cmpwi cr6, r31, 0x80
	ctx.cr[6].compare_i32(ctx.r[31].s32, 128, &mut ctx.xer);
	// 832C63B4: 409900E8  ble cr6, 0x832c649c
	if !ctx.cr[6].gt {
	pc = 0x832C649C; continue 'dispatch;
	}
	pc = 0x832C63B8; continue 'dispatch;
            }
            0x832C63B8 => {
    //   block [0x832C63B8..0x832C63C4)
	// 832C63B8: 7FF7FB78  mr r23, r31
	ctx.r[23].u64 = ctx.r[31].u64;
	// 832C63BC: 7F1F9000  cmpw cr6, r31, r18
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[18].s32, &mut ctx.xer);
	// 832C63C0: 409800AC  bge cr6, 0x832c646c
	if !ctx.cr[6].lt {
	pc = 0x832C646C; continue 'dispatch;
	}
	pc = 0x832C63C4; continue 'dispatch;
            }
            0x832C63C4 => {
    //   block [0x832C63C4..0x832C6418)
	// 832C63C4: 7FDFB850  subf r30, r31, r23
	ctx.r[30].s64 = ctx.r[23].s64 - ctx.r[31].s64;
	// 832C63C8: 7F1E9000  cmpw cr6, r30, r18
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[18].s32, &mut ctx.xer);
	// 832C63CC: 40980094  bge cr6, 0x832c6460
	if !ctx.cr[6].lt {
	pc = 0x832C6460; continue 'dispatch;
	}
	// 832C63D0: 56EB083C  slwi r11, r23, 1
	ctx.r[11].u32 = ctx.r[23].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C63D4: 7FEA0E70  srawi r10, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 832C63D8: 7CCBF214  add r6, r11, r30
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C63DC: 7C8AC850  subf r4, r10, r25
	ctx.r[4].s64 = ctx.r[25].s64 - ctx.r[10].s64;
	// 832C63E0: 7CBFC850  subf r5, r31, r25
	ctx.r[5].s64 = ctx.r[25].s64 - ctx.r[31].s64;
	// 832C63E4: 7C7EBA14  add r3, r30, r23
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[23].u64;
	// 832C63E8: 54A7103A  slwi r7, r5, 2
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C63EC: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C63F0: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C63F4: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C63F8: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C63FC: 7E87C214  add r20, r7, r24
	ctx.r[20].u64 = ctx.r[7].u64 + ctx.r[24].u64;
	// 832C6400: 7EC8C214  add r22, r8, r24
	ctx.r[22].u64 = ctx.r[8].u64 + ctx.r[24].u64;
	// 832C6404: 56F3103A  slwi r19, r23, 2
	ctx.r[19].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[19].u64 = ctx.r[19].u32 as u64;
	// 832C6408: 56FA2036  slwi r26, r23, 4
	ctx.r[26].u32 = ctx.r[23].u32.wrapping_shl(4);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 832C640C: 7FA9AA14  add r29, r9, r21
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[21].u64;
	// 832C6410: 7F6AAA14  add r27, r10, r21
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 832C6414: 7F8BAA14  add r28, r11, r21
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	pc = 0x832C6418; continue 'dispatch;
            }
            0x832C6418 => {
    //   block [0x832C6418..0x832C6460)
	// 832C6418: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 832C641C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832C6420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6424: 480003F5  bl 0x832c6818
	ctx.lr = 0x832C6428;
	sub_832C6818(ctx, base);
	// 832C6428: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 832C642C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832C6430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6434: 4800076D  bl 0x832c6ba0
	ctx.lr = 0x832C6438;
	sub_832C6BA0(ctx, base);
	// 832C6438: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 832C643C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832C6440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6444: 480003D5  bl 0x832c6818
	ctx.lr = 0x832C6448;
	sub_832C6818(ctx, base);
	// 832C6448: 7FD3F214  add r30, r19, r30
	ctx.r[30].u64 = ctx.r[19].u64 + ctx.r[30].u64;
	// 832C644C: 7FBAEA14  add r29, r26, r29
	ctx.r[29].u64 = ctx.r[26].u64 + ctx.r[29].u64;
	// 832C6450: 7F9AE214  add r28, r26, r28
	ctx.r[28].u64 = ctx.r[26].u64 + ctx.r[28].u64;
	// 832C6454: 7F7ADA14  add r27, r26, r27
	ctx.r[27].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 832C6458: 7F1E9000  cmpw cr6, r30, r18
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[18].s32, &mut ctx.xer);
	// 832C645C: 4198FFBC  blt cr6, 0x832c6418
	if ctx.cr[6].lt {
	pc = 0x832C6418; continue 'dispatch;
	}
	pc = 0x832C6460; continue 'dispatch;
            }
            0x832C6460 => {
    //   block [0x832C6460..0x832C646C)
	// 832C6460: 56F7103A  slwi r23, r23, 2
	ctx.r[23].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 832C6464: 7F179000  cmpw cr6, r23, r18
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[18].s32, &mut ctx.xer);
	// 832C6468: 4198FF5C  blt cr6, 0x832c63c4
	if ctx.cr[6].lt {
	pc = 0x832C63C4; continue 'dispatch;
	}
	pc = 0x832C646C; continue 'dispatch;
            }
            0x832C646C => {
    //   block [0x832C646C..0x832C649C)
	// 832C646C: 7FEB0E70  srawi r11, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 832C6470: 7D5F9050  subf r10, r31, r18
	ctx.r[10].s64 = ctx.r[18].s64 - ctx.r[31].s64;
	// 832C6474: 7D2BC850  subf r9, r11, r25
	ctx.r[9].s64 = ctx.r[25].s64 - ctx.r[11].s64;
	// 832C6478: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C647C: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6480: 7C8AAA14  add r4, r10, r21
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 832C6484: 7CABC214  add r5, r11, r24
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 832C6488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C648C: 4800038D  bl 0x832c6818
	ctx.lr = 0x832C6490;
	sub_832C6818(ctx, base);
	// 832C6490: 7FFF1670  srawi r31, r31, 2
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 2) as i64;
	// 832C6494: 2F1F0080  cmpwi cr6, r31, 0x80
	ctx.cr[6].compare_i32(ctx.r[31].s32, 128, &mut ctx.xer);
	// 832C6498: 4199FF20  bgt cr6, 0x832c63b8
	if ctx.cr[6].gt {
	pc = 0x832C63B8; continue 'dispatch;
	}
	pc = 0x832C649C; continue 'dispatch;
            }
            0x832C649C => {
    //   block [0x832C649C..0x832C64A8)
	// 832C649C: 7FF6FB78  mr r22, r31
	ctx.r[22].u64 = ctx.r[31].u64;
	// 832C64A0: 7F1F9000  cmpw cr6, r31, r18
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[18].s32, &mut ctx.xer);
	// 832C64A4: 409800DC  bge cr6, 0x832c6580
	if !ctx.cr[6].lt {
	pc = 0x832C6580; continue 'dispatch;
	}
	pc = 0x832C64A8; continue 'dispatch;
            }
            0x832C64A8 => {
    //   block [0x832C64A8..0x832C64FC)
	// 832C64A8: 7FDFB050  subf r30, r31, r22
	ctx.r[30].s64 = ctx.r[22].s64 - ctx.r[31].s64;
	// 832C64AC: 7F1E9000  cmpw cr6, r30, r18
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[18].s32, &mut ctx.xer);
	// 832C64B0: 409800C4  bge cr6, 0x832c6574
	if !ctx.cr[6].lt {
	pc = 0x832C6574; continue 'dispatch;
	}
	// 832C64B4: 56CB083C  slwi r11, r22, 1
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C64B8: 7FEA0E70  srawi r10, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 832C64BC: 7CCBF214  add r6, r11, r30
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832C64C0: 7C8AC850  subf r4, r10, r25
	ctx.r[4].s64 = ctx.r[25].s64 - ctx.r[10].s64;
	// 832C64C4: 7CBFC850  subf r5, r31, r25
	ctx.r[5].s64 = ctx.r[25].s64 - ctx.r[31].s64;
	// 832C64C8: 7C7EB214  add r3, r30, r22
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[22].u64;
	// 832C64CC: 54A7103A  slwi r7, r5, 2
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C64D0: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C64D4: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C64D8: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C64DC: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C64E0: 7E87C214  add r20, r7, r24
	ctx.r[20].u64 = ctx.r[7].u64 + ctx.r[24].u64;
	// 832C64E4: 7EE8C214  add r23, r8, r24
	ctx.r[23].u64 = ctx.r[8].u64 + ctx.r[24].u64;
	// 832C64E8: 56D3103A  slwi r19, r22, 2
	ctx.r[19].u32 = ctx.r[22].u32.wrapping_shl(2);
	ctx.r[19].u64 = ctx.r[19].u32 as u64;
	// 832C64EC: 56DA2036  slwi r26, r22, 4
	ctx.r[26].u32 = ctx.r[22].u32.wrapping_shl(4);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 832C64F0: 7FA9AA14  add r29, r9, r21
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[21].u64;
	// 832C64F4: 7F6AAA14  add r27, r10, r21
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 832C64F8: 7F8BAA14  add r28, r11, r21
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	pc = 0x832C64FC; continue 'dispatch;
            }
            0x832C64FC => {
    //   block [0x832C64FC..0x832C6574)
	// 832C64FC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 832C6500: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832C6504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6508: 48000311  bl 0x832c6818
	ctx.lr = 0x832C650C;
	sub_832C6818(ctx, base);
	// 832C650C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 832C6510: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 832C6514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6518: 48000AB9  bl 0x832c6fd0
	ctx.lr = 0x832C651C;
	sub_832C6FD0(ctx, base);
	// 832C651C: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 832C6520: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832C6524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6528: 48000679  bl 0x832c6ba0
	ctx.lr = 0x832C652C;
	sub_832C6BA0(ctx, base);
	// 832C652C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 832C6530: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 832C6534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6538: 48000B39  bl 0x832c7070
	ctx.lr = 0x832C653C;
	sub_832C7070(ctx, base);
	// 832C653C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 832C6540: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832C6544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6548: 480002D1  bl 0x832c6818
	ctx.lr = 0x832C654C;
	sub_832C6818(ctx, base);
	// 832C654C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 832C6550: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 832C6554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6558: 48000A79  bl 0x832c6fd0
	ctx.lr = 0x832C655C;
	sub_832C6FD0(ctx, base);
	// 832C655C: 7FD3F214  add r30, r19, r30
	ctx.r[30].u64 = ctx.r[19].u64 + ctx.r[30].u64;
	// 832C6560: 7FBDD214  add r29, r29, r26
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[26].u64;
	// 832C6564: 7F9CD214  add r28, r28, r26
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[26].u64;
	// 832C6568: 7F7BD214  add r27, r27, r26
	ctx.r[27].u64 = ctx.r[27].u64 + ctx.r[26].u64;
	// 832C656C: 7F1E9000  cmpw cr6, r30, r18
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[18].s32, &mut ctx.xer);
	// 832C6570: 4198FF8C  blt cr6, 0x832c64fc
	if ctx.cr[6].lt {
	pc = 0x832C64FC; continue 'dispatch;
	}
	pc = 0x832C6574; continue 'dispatch;
            }
            0x832C6574 => {
    //   block [0x832C6574..0x832C6580)
	// 832C6574: 56D6103A  slwi r22, r22, 2
	ctx.r[22].u32 = ctx.r[22].u32.wrapping_shl(2);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 832C6578: 7F169000  cmpw cr6, r22, r18
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[18].s32, &mut ctx.xer);
	// 832C657C: 4198FF2C  blt cr6, 0x832c64a8
	if ctx.cr[6].lt {
	pc = 0x832C64A8; continue 'dispatch;
	}
	pc = 0x832C6580; continue 'dispatch;
            }
            0x832C6580 => {
    //   block [0x832C6580..0x832C65BC)
	// 832C6580: 7FEB0E70  srawi r11, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 832C6584: 7D5F9050  subf r10, r31, r18
	ctx.r[10].s64 = ctx.r[18].s64 - ctx.r[31].s64;
	// 832C6588: 7D2BC850  subf r9, r11, r25
	ctx.r[9].s64 = ctx.r[25].s64 - ctx.r[11].s64;
	// 832C658C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6590: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6594: 7C8AAA14  add r4, r10, r21
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 832C6598: 7CABC214  add r5, r11, r24
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 832C659C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C65A0: 48000279  bl 0x832c6818
	ctx.lr = 0x832C65A4;
	sub_832C6818(ctx, base);
	// 832C65A4: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 832C65A8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 832C65AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C65B0: 48000A21  bl 0x832c6fd0
	ctx.lr = 0x832C65B4;
	sub_832C6FD0(ctx, base);
	// 832C65B4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 832C65B8: 4B9E2E78  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C65C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C65C0 size=596
    let mut pc: u32 = 0x832C65C0;
    'dispatch: loop {
        match pc {
            0x832C65C0 => {
    //   block [0x832C65C0..0x832C65E8)
	// 832C65C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C65C4: 4B9E2E25  bl 0x82ca93e8
	ctx.lr = 0x832C65C8;
	sub_82CA93D0(ctx, base);
	// 832C65C8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C65CC: 7C7E0E70  srawi r30, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 832C65D0: 7C7F1670  srawi r31, r3, 2
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[3].s32 >> 2) as i64;
	// 832C65D4: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 832C65D8: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 832C65DC: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 832C65E0: 2F1F0080  cmpwi cr6, r31, 0x80
	ctx.cr[6].compare_i32(ctx.r[31].s32, 128, &mut ctx.xer);
	// 832C65E4: 409900F8  ble cr6, 0x832c66dc
	if !ctx.cr[6].gt {
	pc = 0x832C66DC; continue 'dispatch;
	}
	pc = 0x832C65E8; continue 'dispatch;
            }
            0x832C65E8 => {
    //   block [0x832C65E8..0x832C65F4)
	// 832C65E8: 7FF8FB78  mr r24, r31
	ctx.r[24].u64 = ctx.r[31].u64;
	// 832C65EC: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C65F0: 409800E0  bge cr6, 0x832c66d0
	if !ctx.cr[6].lt {
	pc = 0x832C66D0; continue 'dispatch;
	}
	pc = 0x832C65F4; continue 'dispatch;
            }
            0x832C65F4 => {
    //   block [0x832C65F4..0x832C662C)
	// 832C65F4: 7FBFC050  subf r29, r31, r24
	ctx.r[29].s64 = ctx.r[24].s64 - ctx.r[31].s64;
	// 832C65F8: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C65FC: 40980064  bge cr6, 0x832c6660
	if !ctx.cr[6].lt {
	pc = 0x832C6660; continue 'dispatch;
	}
	// 832C6600: 7FEB0E70  srawi r11, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 832C6604: 7D1DF214  add r8, r29, r30
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 832C6608: 7CEBB850  subf r7, r11, r23
	ctx.r[7].s64 = ctx.r[23].s64 - ctx.r[11].s64;
	// 832C660C: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6610: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C6614: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6618: 7F49B214  add r26, r9, r22
	ctx.r[26].u64 = ctx.r[9].u64 + ctx.r[22].u64;
	// 832C661C: 5715083C  slwi r21, r24, 1
	ctx.r[21].u32 = ctx.r[24].u32.wrapping_shl(1);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 832C6620: 57191838  slwi r25, r24, 3
	ctx.r[25].u32 = ctx.r[24].u32.wrapping_shl(3);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 832C6624: 7F8AA214  add r28, r10, r20
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[20].u64;
	// 832C6628: 7F6BA214  add r27, r11, r20
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	pc = 0x832C662C; continue 'dispatch;
            }
            0x832C662C => {
    //   block [0x832C662C..0x832C6660)
	// 832C662C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 832C6630: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832C6634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6638: 480001E1  bl 0x832c6818
	ctx.lr = 0x832C663C;
	sub_832C6818(ctx, base);
	// 832C663C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 832C6640: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832C6644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C6648: 480001D1  bl 0x832c6818
	ctx.lr = 0x832C664C;
	sub_832C6818(ctx, base);
	// 832C664C: 7FB5EA14  add r29, r21, r29
	ctx.r[29].u64 = ctx.r[21].u64 + ctx.r[29].u64;
	// 832C6650: 7F99E214  add r28, r25, r28
	ctx.r[28].u64 = ctx.r[25].u64 + ctx.r[28].u64;
	// 832C6654: 7F79DA14  add r27, r25, r27
	ctx.r[27].u64 = ctx.r[25].u64 + ctx.r[27].u64;
	// 832C6658: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C665C: 4198FFD0  blt cr6, 0x832c662c
	if ctx.cr[6].lt {
	pc = 0x832C662C; continue 'dispatch;
	}
	pc = 0x832C6660; continue 'dispatch;
            }
            0x832C6660 => {
    //   block [0x832C6660..0x832C6698)
	// 832C6660: 570B083C  slwi r11, r24, 1
	ctx.r[11].u32 = ctx.r[24].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6664: 7FBF5850  subf r29, r31, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 832C6668: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C666C: 40980058  bge cr6, 0x832c66c4
	if !ctx.cr[6].lt {
	pc = 0x832C66C4; continue 'dispatch;
	}
	// 832C6670: 7D7FB850  subf r11, r31, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[31].s64;
	// 832C6674: 7D1DF214  add r8, r29, r30
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 832C6678: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C667C: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6680: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6684: 7CA9B214  add r5, r9, r22
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[22].u64;
	// 832C6688: 5719103A  slwi r25, r24, 2
	ctx.r[25].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 832C668C: 571A2036  slwi r26, r24, 4
	ctx.r[26].u32 = ctx.r[24].u32.wrapping_shl(4);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 832C6690: 7F8AA214  add r28, r10, r20
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[20].u64;
	// 832C6694: 7F6BA214  add r27, r11, r20
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	pc = 0x832C6698; continue 'dispatch;
            }
            0x832C6698 => {
    //   block [0x832C6698..0x832C66C4)
	// 832C6698: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832C669C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C66A0: 48000501  bl 0x832c6ba0
	ctx.lr = 0x832C66A4;
	sub_832C6BA0(ctx, base);
	// 832C66A4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832C66A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C66AC: 480004F5  bl 0x832c6ba0
	ctx.lr = 0x832C66B0;
	sub_832C6BA0(ctx, base);
	// 832C66B0: 7FB9EA14  add r29, r25, r29
	ctx.r[29].u64 = ctx.r[25].u64 + ctx.r[29].u64;
	// 832C66B4: 7F9AE214  add r28, r26, r28
	ctx.r[28].u64 = ctx.r[26].u64 + ctx.r[28].u64;
	// 832C66B8: 7F7ADA14  add r27, r26, r27
	ctx.r[27].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 832C66BC: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C66C0: 4198FFD8  blt cr6, 0x832c6698
	if ctx.cr[6].lt {
	pc = 0x832C6698; continue 'dispatch;
	}
	pc = 0x832C66C4; continue 'dispatch;
            }
            0x832C66C4 => {
    //   block [0x832C66C4..0x832C66D0)
	// 832C66C4: 5718103A  slwi r24, r24, 2
	ctx.r[24].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 832C66C8: 7F18F000  cmpw cr6, r24, r30
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C66CC: 4198FF28  blt cr6, 0x832c65f4
	if ctx.cr[6].lt {
	pc = 0x832C65F4; continue 'dispatch;
	}
	pc = 0x832C66D0; continue 'dispatch;
            }
            0x832C66D0 => {
    //   block [0x832C66D0..0x832C66DC)
	// 832C66D0: 7FFF1670  srawi r31, r31, 2
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 2) as i64;
	// 832C66D4: 2F1F0080  cmpwi cr6, r31, 0x80
	ctx.cr[6].compare_i32(ctx.r[31].s32, 128, &mut ctx.xer);
	// 832C66D8: 4199FF10  bgt cr6, 0x832c65e8
	if ctx.cr[6].gt {
	pc = 0x832C65E8; continue 'dispatch;
	}
	pc = 0x832C66DC; continue 'dispatch;
            }
            0x832C66DC => {
    //   block [0x832C66DC..0x832C66E8)
	// 832C66DC: 7FF8FB78  mr r24, r31
	ctx.r[24].u64 = ctx.r[31].u64;
	// 832C66E0: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C66E4: 40980128  bge cr6, 0x832c680c
	if !ctx.cr[6].lt {
	pc = 0x832C680C; continue 'dispatch;
	}
	pc = 0x832C66E8; continue 'dispatch;
            }
            0x832C66E8 => {
    //   block [0x832C66E8..0x832C6720)
	// 832C66E8: 7FBFC050  subf r29, r31, r24
	ctx.r[29].s64 = ctx.r[24].s64 - ctx.r[31].s64;
	// 832C66EC: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C66F0: 40980084  bge cr6, 0x832c6774
	if !ctx.cr[6].lt {
	pc = 0x832C6774; continue 'dispatch;
	}
	// 832C66F4: 7FEB0E70  srawi r11, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 832C66F8: 7D1DF214  add r8, r29, r30
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 832C66FC: 7CEBB850  subf r7, r11, r23
	ctx.r[7].s64 = ctx.r[23].s64 - ctx.r[11].s64;
	// 832C6700: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6704: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C6708: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C670C: 7F49B214  add r26, r9, r22
	ctx.r[26].u64 = ctx.r[9].u64 + ctx.r[22].u64;
	// 832C6710: 5715083C  slwi r21, r24, 1
	ctx.r[21].u32 = ctx.r[24].u32.wrapping_shl(1);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 832C6714: 57191838  slwi r25, r24, 3
	ctx.r[25].u32 = ctx.r[24].u32.wrapping_shl(3);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 832C6718: 7F8AA214  add r28, r10, r20
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[20].u64;
	// 832C671C: 7F6BA214  add r27, r11, r20
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	pc = 0x832C6720; continue 'dispatch;
            }
            0x832C6720 => {
    //   block [0x832C6720..0x832C6774)
	// 832C6720: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 832C6724: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832C6728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C672C: 480000ED  bl 0x832c6818
	ctx.lr = 0x832C6730;
	sub_832C6818(ctx, base);
	// 832C6730: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 832C6734: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 832C6738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C673C: 48000895  bl 0x832c6fd0
	ctx.lr = 0x832C6740;
	sub_832C6FD0(ctx, base);
	// 832C6740: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 832C6744: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832C6748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C674C: 480000CD  bl 0x832c6818
	ctx.lr = 0x832C6750;
	sub_832C6818(ctx, base);
	// 832C6750: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 832C6754: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 832C6758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C675C: 48000875  bl 0x832c6fd0
	ctx.lr = 0x832C6760;
	sub_832C6FD0(ctx, base);
	// 832C6760: 7FB5EA14  add r29, r21, r29
	ctx.r[29].u64 = ctx.r[21].u64 + ctx.r[29].u64;
	// 832C6764: 7F9CCA14  add r28, r28, r25
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[25].u64;
	// 832C6768: 7F7BCA14  add r27, r27, r25
	ctx.r[27].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 832C676C: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C6770: 4198FFB0  blt cr6, 0x832c6720
	if ctx.cr[6].lt {
	pc = 0x832C6720; continue 'dispatch;
	}
	pc = 0x832C6774; continue 'dispatch;
            }
            0x832C6774 => {
    //   block [0x832C6774..0x832C67AC)
	// 832C6774: 570B083C  slwi r11, r24, 1
	ctx.r[11].u32 = ctx.r[24].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6778: 7FBF5850  subf r29, r31, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 832C677C: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C6780: 40980080  bge cr6, 0x832c6800
	if !ctx.cr[6].lt {
	pc = 0x832C6800; continue 'dispatch;
	}
	// 832C6784: 7D7FB850  subf r11, r31, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[31].s64;
	// 832C6788: 7D1DF214  add r8, r29, r30
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 832C678C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C6790: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6794: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6798: 7F49B214  add r26, r9, r22
	ctx.r[26].u64 = ctx.r[9].u64 + ctx.r[22].u64;
	// 832C679C: 5715103A  slwi r21, r24, 2
	ctx.r[21].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 832C67A0: 57192036  slwi r25, r24, 4
	ctx.r[25].u32 = ctx.r[24].u32.wrapping_shl(4);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 832C67A4: 7F8AA214  add r28, r10, r20
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[20].u64;
	// 832C67A8: 7F6BA214  add r27, r11, r20
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	pc = 0x832C67AC; continue 'dispatch;
            }
            0x832C67AC => {
    //   block [0x832C67AC..0x832C6800)
	// 832C67AC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 832C67B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832C67B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C67B8: 480003E9  bl 0x832c6ba0
	ctx.lr = 0x832C67BC;
	sub_832C6BA0(ctx, base);
	// 832C67BC: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 832C67C0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 832C67C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C67C8: 480008A9  bl 0x832c7070
	ctx.lr = 0x832C67CC;
	sub_832C7070(ctx, base);
	// 832C67CC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 832C67D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832C67D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C67D8: 480003C9  bl 0x832c6ba0
	ctx.lr = 0x832C67DC;
	sub_832C6BA0(ctx, base);
	// 832C67DC: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 832C67E0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 832C67E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832C67E8: 48000889  bl 0x832c7070
	ctx.lr = 0x832C67EC;
	sub_832C7070(ctx, base);
	// 832C67EC: 7FB5EA14  add r29, r21, r29
	ctx.r[29].u64 = ctx.r[21].u64 + ctx.r[29].u64;
	// 832C67F0: 7F9CCA14  add r28, r28, r25
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[25].u64;
	// 832C67F4: 7F7BCA14  add r27, r27, r25
	ctx.r[27].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 832C67F8: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C67FC: 4198FFB0  blt cr6, 0x832c67ac
	if ctx.cr[6].lt {
	pc = 0x832C67AC; continue 'dispatch;
	}
	pc = 0x832C6800; continue 'dispatch;
            }
            0x832C6800 => {
    //   block [0x832C6800..0x832C680C)
	// 832C6800: 5718103A  slwi r24, r24, 2
	ctx.r[24].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 832C6804: 7F18F000  cmpw cr6, r24, r30
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[30].s32, &mut ctx.xer);
	// 832C6808: 4198FEE0  blt cr6, 0x832c66e8
	if ctx.cr[6].lt {
	pc = 0x832C66E8; continue 'dispatch;
	}
	pc = 0x832C680C; continue 'dispatch;
            }
            0x832C680C => {
    //   block [0x832C680C..0x832C6814)
	// 832C680C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 832C6810: 4B9E2C28  b 0x82ca9438
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C6818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C6818 size=904
    let mut pc: u32 = 0x832C6818;
    'dispatch: loop {
        match pc {
            0x832C6818 => {
    //   block [0x832C6818..0x832C6940)
	// 832C6818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C681C: 4B9E2BED  bl 0x82ca9408
	ctx.lr = 0x832C6820;
	sub_82CA93D0(ctx, base);
	// 832C6820: DBA1FFC0  stfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[29].u64 ) };
	// 832C6824: DBC1FFC8  stfd f30, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 832C6828: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 832C682C: 7C7D1E70  srawi r29, r3, 3
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[3].s32 >> 3) as i64;
	// 832C6830: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C6834: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6838: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C683C: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6840: 57A71838  slwi r7, r29, 3
	ctx.r[7].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C6844: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832C6848: 57A92036  slwi r9, r29, 4
	ctx.r[9].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C684C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6850: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C6854: 7D0A2214  add r8, r10, r4
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C6858: 7D87242E  lfsx f12, r7, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C685C: 7D472214  add r10, r7, r4
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 832C6860: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 832C6864: C1690000  lfs f11, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C6868: C1480000  lfs f10, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C686C: ED20582A  fadds f9, f0, f11
	ctx.f[9].f64 = ((ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C6870: C1090004  lfs f8, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C6874: ECEC502A  fadds f7, f12, f10
	ctx.f[7].f64 = ((ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6878: C0C80004  lfs f6, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C687C: ECAD402A  fadds f5, f13, f8
	ctx.f[5].f64 = ((ctx.f[13].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C6880: C08A0004  lfs f4, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C6884: EC605828  fsubs f3, f0, f11
	ctx.f[3].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C6888: EC44302A  fadds f2, f4, f6
	ctx.f[2].f64 = ((ctx.f[4].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C688C: EC2D4028  fsubs f1, f13, f8
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C6890: EC0C5028  fsubs f0, f12, f10
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C6894: EDA43028  fsubs f13, f4, f6
	ctx.f[13].f64 = (((ctx.f[4].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C6898: ED87482A  fadds f12, f7, f9
	ctx.f[12].f64 = ((ctx.f[7].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C689C: D1840000  stfs f12, 0(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C68A0: ED693828  fsubs f11, f9, f7
	ctx.f[11].f64 = (((ctx.f[9].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C68A4: ED42282A  fadds f10, f2, f5
	ctx.f[10].f64 = ((ctx.f[2].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C68A8: D1440004  stfs f10, 4(r4)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C68AC: ED251028  fsubs f9, f5, f2
	ctx.f[9].f64 = (((ctx.f[5].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C68B0: 7D67252E  stfsx f11, r7, r4
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C68B4: D12A0004  stfs f9, 4(r10)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C68B8: ED00082A  fadds f8, f0, f1
	ctx.f[8].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C68BC: ECE36828  fsubs f7, f3, f13
	ctx.f[7].f64 = (((ctx.f[3].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C68C0: D0E90000  stfs f7, 0(r9)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C68C4: D1090004  stfs f8, 4(r9)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C68C8: ECA10028  fsubs f5, f1, f0
	ctx.f[5].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C68CC: ECCD182A  fadds f6, f13, f3
	ctx.f[6].f64 = ((ctx.f[13].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C68D0: D0C80000  stfs f6, 0(r8)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C68D4: D0A80004  stfs f5, 4(r8)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C68D8: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C68DC: 409901E4  ble cr6, 0x832c6ac0
	if !ctx.cr[6].gt {
	pc = 0x832C6AC0; continue 'dispatch;
	}
	// 832C68E0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C68E4: 38EB0002  addi r7, r11, 2
	ctx.r[7].s64 = ctx.r[11].s64 + 2;
	// 832C68E8: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 832C68EC: 386BFFFE  addi r3, r11, -2
	ctx.r[3].s64 = ctx.r[11].s64 + -2;
	// 832C68F0: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C68F4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C68F8: 7D092214  add r8, r9, r4
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C68FC: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C6900: 38DDFFFD  addi r6, r29, -3
	ctx.r[6].s64 = ctx.r[29].s64 + -3;
	// 832C6904: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C6908: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 832C690C: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C6910: 54DCF87E  srwi r28, r6, 1
	ctx.r[28].u32 = ctx.r[6].u32.wrapping_shr(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832C6914: 7CE72214  add r7, r7, r4
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 832C6918: 7FE92214  add r31, r9, r4
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C691C: 38C50008  addi r6, r5, 8
	ctx.r[6].s64 = ctx.r[5].s64 + 8;
	// 832C6920: 7D232214  add r9, r3, r4
	ctx.r[9].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 832C6924: 386A0008  addi r3, r10, 8
	ctx.r[3].s64 = ctx.r[10].s64 + 8;
	// 832C6928: 38A80008  addi r5, r8, 8
	ctx.r[5].s64 = ctx.r[8].s64 + 8;
	// 832C692C: 3BC40008  addi r30, r4, 8
	ctx.r[30].s64 = ctx.r[4].s64 + 8;
	// 832C6930: 394AFFF8  addi r10, r10, -8
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	// 832C6934: 3908FFF8  addi r8, r8, -8
	ctx.r[8].s64 = ctx.r[8].s64 + -8;
	// 832C6938: 38E7FFF8  addi r7, r7, -8
	ctx.r[7].s64 = ctx.r[7].s64 + -8;
	// 832C693C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	pc = 0x832C6940; continue 'dispatch;
            }
            0x832C6940 => {
    //   block [0x832C6940..0x832C6AC0)
	// 832C6940: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6944: 38C60010  addi r6, r6, 0x10
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	// 832C6948: C1650004  lfs f11, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C694C: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 832C6950: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6954: C13F0004  lfs f9, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C6958: ED0C682A  fadds f8, f12, f13
	ctx.f[8].f64 = ((ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C695C: C0E50000  lfs f7, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C6960: EC8B482A  fadds f4, f11, f9
	ctx.f[4].f64 = ((ctx.f[11].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C6964: C0BF0000  lfs f5, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C6968: ED4D6028  fsubs f10, f13, f12
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C696C: C0230000  lfs f1, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C6970: EC453828  fsubs f2, f5, f7
	ctx.f[2].f64 = (((ctx.f[5].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C6974: C07E0000  lfs f3, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C6978: ECC95828  fsubs f6, f9, f11
	ctx.f[6].f64 = (((ctx.f[9].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C697C: ED830828  fsubs f12, f3, f1
	ctx.f[12].f64 = (((ctx.f[3].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C6980: C1660004  lfs f11, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C6984: EDA7282A  fadds f13, f7, f5
	ctx.f[13].f64 = ((ctx.f[7].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C6988: C0E6FFFC  lfs f7, -4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C698C: FD205850  fneg f9, f11
	ctx.f[9].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 832C6990: ECA1182A  fadds f5, f1, f3
	ctx.f[5].f64 = ((ctx.f[1].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C6994: C026FFF8  lfs f1, -8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C6998: C0660000  lfs f3, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C699C: ED64402A  fadds f11, f4, f8
	ctx.f[11].f64 = ((ctx.f[4].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C69A0: D17E0004  stfs f11, 4(r30)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C69A4: EC882028  fsubs f4, f8, f4
	ctx.f[4].f64 = (((ctx.f[8].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C69A8: ED62502A  fadds f11, f2, f10
	ctx.f[11].f64 = ((ctx.f[2].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C69AC: ED4A1028  fsubs f10, f10, f2
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C69B0: EC4C3028  fsubs f2, f12, f6
	ctx.f[2].f64 = (((ctx.f[12].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C69B4: ED86602A  fadds f12, f6, f12
	ctx.f[12].f64 = ((ctx.f[6].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C69B8: ECCD282A  fadds f6, f13, f5
	ctx.f[6].f64 = ((ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C69BC: D0DE0000  stfs f6, 0(r30)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C69C0: D09F0004  stfs f4, 4(r31)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C69C4: EDA56828  fsubs f13, f5, f13
	ctx.f[13].f64 = (((ctx.f[5].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C69C8: D1BF0000  stfs f13, 0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C69CC: ED0702F2  fmuls f8, f7, f11
	ctx.f[8].f64 = (((ctx.f[7].f64 * ctx.f[11].f64) as f32) as f64);
	// 832C69D0: ECC700B2  fmuls f6, f7, f2
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[2].f64) as f32) as f64);
	// 832C69D4: EC890332  fmuls f4, f9, f12
	ctx.f[4].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 832C69D8: ECA30332  fmuls f5, f3, f12
	ctx.f[5].f64 = (((ctx.f[3].f64 * ctx.f[12].f64) as f32) as f64);
	// 832C69DC: EC4140B8  fmsubs f2, f1, f2, f8
	ctx.f[2].f64 = (((ctx.f[1].f64 * ctx.f[2].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C69E0: D0430000  stfs f2, 0(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C69E4: EDA132FA  fmadds f13, f1, f11, f6
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[11].f64 + ctx.f[6].f64) as f32) as f64);
	// 832C69E8: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C69EC: ED6322B8  fmsubs f11, f3, f10, f4
	ctx.f[11].f64 = (((ctx.f[3].f64 * ctx.f[10].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C69F0: D1650004  stfs f11, 4(r5)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C69F4: ED892ABA  fmadds f12, f9, f10, f5
	ctx.f[12].f64 = (((ctx.f[9].f64 * ctx.f[10].f64 + ctx.f[5].f64) as f32) as f64);
	// 832C69F8: D1850000  stfs f12, 0(r5)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C69FC: C1090004  lfs f8, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C6A00: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 832C6A04: C1480004  lfs f10, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C6A08: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 832C6A0C: C0C70004  lfs f6, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C6A10: C0A80000  lfs f5, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C6A14: C08A0004  lfs f4, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C6A18: C0490000  lfs f2, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C6A1C: C1A70000  lfs f13, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6A20: C18A0000  lfs f12, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6A24: ED6C6828  fsubs f11, f12, f13
	ctx.f[11].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C6A28: EFE85028  fsubs f31, f8, f10
	ctx.f[31].f64 = (((ctx.f[8].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C6A2C: ED4A402A  fadds f10, f10, f8
	ctx.f[10].f64 = ((ctx.f[10].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C6A30: ED06202A  fadds f8, f6, f4
	ctx.f[8].f64 = ((ctx.f[6].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C6A34: EFC22828  fsubs f30, f2, f5
	ctx.f[30].f64 = (((ctx.f[2].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C6A38: EFA43028  fsubs f29, f4, f6
	ctx.f[29].f64 = (((ctx.f[4].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C6A3C: ECCD602A  fadds f6, f13, f12
	ctx.f[6].f64 = ((ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C6A40: ECA5102A  fadds f5, f5, f2
	ctx.f[5].f64 = ((ctx.f[5].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C6A44: EC8BF82A  fadds f4, f11, f31
	ctx.f[4].f64 = ((ctx.f[11].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C6A48: EDA8502A  fadds f13, f8, f10
	ctx.f[13].f64 = ((ctx.f[8].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6A4C: D1A90004  stfs f13, 4(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6A50: ED8A4028  fsubs f12, f10, f8
	ctx.f[12].f64 = (((ctx.f[10].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C6A54: EC5EE828  fsubs f2, f30, f29
	ctx.f[2].f64 = (((ctx.f[30].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C6A58: EDBF5828  fsubs f13, f31, f11
	ctx.f[13].f64 = (((ctx.f[31].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C6A5C: ED46282A  fadds f10, f6, f5
	ctx.f[10].f64 = ((ctx.f[6].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C6A60: D1490000  stfs f10, 0(r9)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6A64: ED053028  fsubs f8, f5, f6
	ctx.f[8].f64 = (((ctx.f[5].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C6A68: D10A0000  stfs f8, 0(r10)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6A6C: D18A0004  stfs f12, 4(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6A70: ECC10132  fmuls f6, f1, f4
	ctx.f[6].f64 = (((ctx.f[1].f64 * ctx.f[4].f64) as f32) as f64);
	// 832C6A74: ECA100B2  fmuls f5, f1, f2
	ctx.f[5].f64 = (((ctx.f[1].f64 * ctx.f[2].f64) as f32) as f64);
	// 832C6A78: EC3DF02A  fadds f1, f29, f30
	ctx.f[1].f64 = ((ctx.f[29].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C6A7C: EC4730B8  fmsubs f2, f7, f2, f6
	ctx.f[2].f64 = (((ctx.f[7].f64 * ctx.f[2].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C6A80: D0480000  stfs f2, 0(r8)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6A84: ED690072  fmuls f11, f9, f1
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[1].f64) as f32) as f64);
	// 832C6A88: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 832C6A8C: ED430072  fmuls f10, f3, f1
	ctx.f[10].f64 = (((ctx.f[3].f64 * ctx.f[1].f64) as f32) as f64);
	// 832C6A90: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 832C6A94: ED87293A  fmadds f12, f7, f4, f5
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[4].f64 + ctx.f[5].f64) as f32) as f64);
	// 832C6A98: D1880004  stfs f12, 4(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6A9C: 394AFFF8  addi r10, r10, -8
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	// 832C6AA0: 3929FFF8  addi r9, r9, -8
	ctx.r[9].s64 = ctx.r[9].s64 + -8;
	// 832C6AA4: 3908FFF8  addi r8, r8, -8
	ctx.r[8].s64 = ctx.r[8].s64 + -8;
	// 832C6AA8: ED035B7A  fmadds f8, f3, f13, f11
	ctx.f[8].f64 = (((ctx.f[3].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 832C6AAC: D1070000  stfs f8, 0(r7)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6AB0: ECE95378  fmsubs f7, f9, f13, f10
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C6AB4: D0E70004  stfs f7, 4(r7)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6AB8: 38E7FFF8  addi r7, r7, -8
	ctx.r[7].s64 = ctx.r[7].s64 + -8;
	// 832C6ABC: 4082FE84  bne 0x832c6940
	if !ctx.cr[0].eq {
	pc = 0x832C6940; continue 'dispatch;
	}
	pc = 0x832C6AC0; continue 'dispatch;
            }
            0x832C6AC0 => {
    //   block [0x832C6AC0..0x832C6BA0)
	// 832C6AC0: 7D4BEA14  add r10, r11, r29
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 832C6AC4: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 832C6AC8: 57A7103A  slwi r7, r29, 2
	ctx.r[7].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C6ACC: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832C6AD0: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832C6AD4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 832C6AD8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6ADC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6AE0: 7D87242E  lfsx f12, r7, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6AE4: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C6AE8: 7D0B2214  add r8, r11, r4
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 832C6AEC: 7D66242E  lfsx f11, r6, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C6AF0: 7D672214  add r11, r7, r4
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 832C6AF4: 7D462214  add r10, r6, r4
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 832C6AF8: C1490000  lfs f10, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C6AFC: C0A90004  lfs f5, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C6B00: ED0C5028  fsubs f8, f12, f10
	ctx.f[8].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C6B04: C0EB0004  lfs f7, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C6B08: EC8C502A  fadds f4, f12, f10
	ctx.f[4].f64 = ((ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6B0C: C1280000  lfs f9, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C6B10: EC472828  fsubs f2, f7, f5
	ctx.f[2].f64 = (((ctx.f[7].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C6B14: C0680004  lfs f3, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C6B18: ECCB4828  fsubs f6, f11, f9
	ctx.f[6].f64 = (((ctx.f[11].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C6B1C: C02A0004  lfs f1, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C6B20: ED87282A  fadds f12, f7, f5
	ctx.f[12].f64 = ((ctx.f[7].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C6B24: ED411828  fsubs f10, f1, f3
	ctx.f[10].f64 = (((ctx.f[1].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C6B28: ECE1182A  fadds f7, f1, f3
	ctx.f[7].f64 = ((ctx.f[1].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C6B2C: ED2B482A  fadds f9, f11, f9
	ctx.f[9].f64 = ((ctx.f[11].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C6B30: EC623028  fsubs f3, f2, f6
	ctx.f[3].f64 = (((ctx.f[2].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C6B34: ECA6102A  fadds f5, f6, f2
	ctx.f[5].f64 = ((ctx.f[6].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C6B38: EC485028  fsubs f2, f8, f10
	ctx.f[2].f64 = (((ctx.f[8].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C6B3C: EC2A402A  fadds f1, f10, f8
	ctx.f[1].f64 = ((ctx.f[10].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C6B40: ED69202A  fadds f11, f9, f4
	ctx.f[11].f64 = ((ctx.f[9].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C6B44: 7D67252E  stfsx f11, r7, r4
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6B48: ED47602A  fadds f10, f7, f12
	ctx.f[10].f64 = ((ctx.f[7].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C6B4C: D14B0004  stfs f10, 4(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6B50: ED0C3828  fsubs f8, f12, f7
	ctx.f[8].f64 = (((ctx.f[12].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C6B54: D10A0004  stfs f8, 4(r10)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6B58: ED244828  fsubs f9, f4, f9
	ctx.f[9].f64 = (((ctx.f[4].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C6B5C: 7D26252E  stfsx f9, r6, r4
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6B60: ECE22828  fsubs f7, f2, f5
	ctx.f[7].f64 = (((ctx.f[2].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C6B64: ECC5102A  fadds f6, f5, f2
	ctx.f[6].f64 = ((ctx.f[5].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C6B68: EC830828  fsubs f4, f3, f1
	ctx.f[4].f64 = (((ctx.f[3].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C6B6C: ECA3082A  fadds f5, f3, f1
	ctx.f[5].f64 = ((ctx.f[3].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C6B70: EC670032  fmuls f3, f7, f0
	ctx.f[3].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C6B74: D0690000  stfs f3, 0(r9)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6B78: EC460032  fmuls f2, f6, f0
	ctx.f[2].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C6B7C: D0490004  stfs f2, 4(r9)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6B80: EC040372  fmuls f0, f4, f13
	ctx.f[0].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C6B84: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6B88: EC250372  fmuls f1, f5, f13
	ctx.f[1].f64 = (((ctx.f[5].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C6B8C: D0280000  stfs f1, 0(r8)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6B90: CBA1FFC0  lfd f29, -0x40(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 832C6B94: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 832C6B98: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 832C6B9C: 4B9E28BC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C6BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C6BA0 size=1072
    let mut pc: u32 = 0x832C6BA0;
    'dispatch: loop {
        match pc {
            0x832C6BA0 => {
    //   block [0x832C6BA0..0x832C6CF4)
	// 832C6BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C6BA4: 4B9E2855  bl 0x82ca93f8
	ctx.lr = 0x832C6BA8;
	sub_82CA93D0(ctx, base);
	// 832C6BA8: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 832C6BAC: 4B9E711D  bl 0x82cadcc8
	ctx.lr = 0x832C6BB0;
	sub_82CADCA0(ctx, base);
	// 832C6BB0: 7C7B1E70  srawi r27, r3, 3
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[3].s32 >> 3) as i64;
	// 832C6BB4: C1840004  lfs f12, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6BB8: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6BBC: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6BC0: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C6BC4: 576A103A  slwi r10, r27, 2
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6BC8: 57781838  slwi r24, r27, 3
	ctx.r[24].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 832C6BCC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832C6BD0: 7CF82214  add r7, r24, r4
	ctx.r[7].u64 = ctx.r[24].u64 + ctx.r[4].u64;
	// 832C6BD4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6BD8: 57692036  slwi r9, r27, 4
	ctx.r[9].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C6BDC: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C6BE0: 7D78242E  lfsx f11, r24, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C6BE4: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C6BE8: C1470004  lfs f10, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C6BEC: 5768103A  slwi r8, r27, 2
	ctx.r[8].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C6BF0: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 832C6BF4: C12A0000  lfs f9, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C6BF8: C10A0004  lfs f8, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C6BFC: ECE9502A  fadds f7, f9, f10
	ctx.f[7].f64 = ((ctx.f[9].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6C00: ECCB4028  fsubs f6, f11, f8
	ctx.f[6].f64 = (((ctx.f[11].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C6C04: C0A90000  lfs f5, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C6C08: EC48582A  fadds f2, f8, f11
	ctx.f[2].f64 = ((ctx.f[8].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C6C0C: C0690004  lfs f3, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C6C10: EC8A4828  fsubs f4, f10, f9
	ctx.f[4].f64 = (((ctx.f[10].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C6C14: ED2C2828  fsubs f9, f12, f5
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C6C18: ED6C282A  fadds f11, f12, f5
	ctx.f[11].f64 = ((ctx.f[12].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C6C1C: ED43682A  fadds f10, f3, f13
	ctx.f[10].f64 = ((ctx.f[3].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C6C20: EC2D1828  fsubs f1, f13, f3
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C6C24: ED063828  fsubs f8, f6, f7
	ctx.f[8].f64 = (((ctx.f[6].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C6C28: ECE7302A  fadds f7, f7, f6
	ctx.f[7].f64 = ((ctx.f[7].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C6C2C: ECC22028  fsubs f6, f2, f4
	ctx.f[6].f64 = (((ctx.f[2].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C6C30: ECA4102A  fadds f5, f4, f2
	ctx.f[5].f64 = ((ctx.f[4].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C6C34: EC880032  fmuls f4, f8, f0
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C6C38: EC670032  fmuls f3, f7, f0
	ctx.f[3].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C6C3C: EC460032  fmuls f2, f6, f0
	ctx.f[2].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C6C40: EC050032  fmuls f0, f5, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C6C44: EDA4082A  fadds f13, f4, f1
	ctx.f[13].f64 = ((ctx.f[4].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C6C48: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6C4C: ED83582A  fadds f12, f3, f11
	ctx.f[12].f64 = ((ctx.f[3].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C6C50: D1840004  stfs f12, 4(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6C54: ED012028  fsubs f8, f1, f4
	ctx.f[8].f64 = (((ctx.f[1].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C6C58: 7D18252E  stfsx f8, r24, r4
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C6C5C: ECEB1828  fsubs f7, f11, f3
	ctx.f[7].f64 = (((ctx.f[11].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C6C60: D0E70004  stfs f7, 4(r7)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6C64: ECC2482A  fadds f6, f2, f9
	ctx.f[6].f64 = ((ctx.f[2].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C6C68: D0C90004  stfs f6, 4(r9)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6C6C: ECAA0028  fsubs f5, f10, f0
	ctx.f[5].f64 = (((ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C6C70: D0A90000  stfs f5, 0(r9)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6C74: EC80502A  fadds f4, f0, f10
	ctx.f[4].f64 = ((ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6C78: D08A0000  stfs f4, 0(r10)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6C7C: EC691028  fsubs f3, f9, f2
	ctx.f[3].f64 = (((ctx.f[9].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C6C80: D06A0004  stfs f3, 4(r10)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6C84: 40990248  ble cr6, 0x832c6ecc
	if !ctx.cr[6].gt {
	pc = 0x832C6ECC; continue 'dispatch;
	}
	// 832C6C88: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6C8C: 38CB0002  addi r6, r11, 2
	ctx.r[6].s64 = ctx.r[11].s64 + 2;
	// 832C6C90: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 832C6C94: 3BC80002  addi r30, r8, 2
	ctx.r[30].s64 = ctx.r[8].s64 + 2;
	// 832C6C98: 5469103A  slwi r9, r3, 2
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C6C9C: 386BFFFE  addi r3, r11, -2
	ctx.r[3].s64 = ctx.r[11].s64 + -2;
	// 832C6CA0: 7D092214  add r8, r9, r4
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C6CA4: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C6CA8: 547F103A  slwi r31, r3, 2
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 832C6CAC: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C6CB0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6CB4: 387BFFFD  addi r3, r27, -3
	ctx.r[3].s64 = ctx.r[27].s64 + -3;
	// 832C6CB8: 7F472214  add r26, r7, r4
	ctx.r[26].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 832C6CBC: 57C6103A  slwi r6, r30, 2
	ctx.r[6].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 832C6CC0: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C6CC4: 7FA92214  add r29, r9, r4
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C6CC8: 5479F87E  srwi r25, r3, 1
	ctx.r[25].u32 = ctx.r[3].u32.wrapping_shr(1);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 832C6CCC: 7D3F2214  add r9, r31, r4
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 832C6CD0: 3BE80008  addi r31, r8, 8
	ctx.r[31].s64 = ctx.r[8].s64 + 8;
	// 832C6CD4: 38E8FFF8  addi r7, r8, -8
	ctx.r[7].s64 = ctx.r[8].s64 + -8;
	// 832C6CD8: 7C662A14  add r3, r6, r5
	ctx.r[3].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 832C6CDC: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 832C6CE0: 391AFFF8  addi r8, r26, -8
	ctx.r[8].s64 = ctx.r[26].s64 + -8;
	// 832C6CE4: 3B840008  addi r28, r4, 8
	ctx.r[28].s64 = ctx.r[4].s64 + 8;
	// 832C6CE8: 38C50008  addi r6, r5, 8
	ctx.r[6].s64 = ctx.r[5].s64 + 8;
	// 832C6CEC: 394AFFF8  addi r10, r10, -8
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	// 832C6CF0: 3B590001  addi r26, r25, 1
	ctx.r[26].s64 = ctx.r[25].s64 + 1;
	pc = 0x832C6CF4; continue 'dispatch;
            }
            0x832C6CF4 => {
    //   block [0x832C6CF4..0x832C6ECC)
	// 832C6CF4: 38C60010  addi r6, r6, 0x10
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	// 832C6CF8: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C6CFC: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 832C6D00: C17E0004  lfs f11, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C6D04: C0DD0000  lfs f6, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C6D08: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 832C6D0C: C19C0000  lfs f12, 0(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6D10: EC660028  fsubs f3, f6, f0
	ctx.f[3].f64 = (((ctx.f[6].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C6D14: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6D18: ED2C5828  fsubs f9, f12, f11
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C6D1C: C11E0000  lfs f8, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C6D20: ECEB602A  fadds f7, f11, f12
	ctx.f[7].f64 = ((ctx.f[11].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C6D24: C09D0004  lfs f4, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C6D28: ED60302A  fadds f11, f0, f6
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C6D2C: C15C0004  lfs f10, 4(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C6D30: EC2D202A  fadds f1, f13, f4
	ctx.f[1].f64 = ((ctx.f[13].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C6D34: C0460004  lfs f2, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C6D38: ECA8502A  fadds f5, f8, f10
	ctx.f[5].f64 = ((ctx.f[8].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6D3C: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6D40: FCC01050  fneg f6, f2
	ctx.f[6].u64 = ctx.f[2].u64 ^ 0x8000_0000_0000_0000u64;
	// 832C6D44: FC006050  fneg f0, f12
	ctx.f[0].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 832C6D48: C046FFFC  lfs f2, -4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C6D4C: C183FFF8  lfs f12, -8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6D50: EC846828  fsubs f4, f4, f13
	ctx.f[4].f64 = (((ctx.f[4].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C6D54: C3E60000  lfs f31, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C6D58: ED4A4028  fsubs f10, f10, f8
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C6D5C: C1030000  lfs f8, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C6D60: EF4C00F2  fmuls f26, f12, f3
	ctx.f[26].f64 = (((ctx.f[12].f64 * ctx.f[3].f64) as f32) as f64);
	// 832C6D64: EDA20272  fmuls f13, f2, f9
	ctx.f[13].f64 = (((ctx.f[2].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C6D68: C3C3FFFC  lfs f30, -4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C6D6C: EFBF01F2  fmuls f29, f31, f7
	ctx.f[29].f64 = (((ctx.f[31].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C6D70: C386FFF8  lfs f28, -8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C6D74: EF0802F2  fmuls f24, f8, f11
	ctx.f[24].f64 = (((ctx.f[8].f64 * ctx.f[11].f64) as f32) as f64);
	// 832C6D78: EF2C0072  fmuls f25, f12, f1
	ctx.f[25].f64 = (((ctx.f[12].f64 * ctx.f[1].f64) as f32) as f64);
	// 832C6D7C: EF620172  fmuls f27, f2, f5
	ctx.f[27].f64 = (((ctx.f[2].f64 * ctx.f[5].f64) as f32) as f64);
	// 832C6D80: ECE601F2  fmuls f7, f6, f7
	ctx.f[7].f64 = (((ctx.f[6].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C6D84: ED6002F2  fmuls f11, f0, f11
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 832C6D88: EC3ED07A  fmadds f1, f30, f1, f26
	ctx.f[1].f64 = (((ctx.f[30].f64 * ctx.f[1].f64 + ctx.f[26].f64) as f32) as f64);
	// 832C6D8C: ECBC697A  fmadds f5, f28, f5, f13
	ctx.f[5].f64 = (((ctx.f[28].f64 * ctx.f[5].f64 + ctx.f[13].f64) as f32) as f64);
	// 832C6D90: EDA6EABA  fmadds f13, f6, f10, f29
	ctx.f[13].f64 = (((ctx.f[6].f64 * ctx.f[10].f64 + ctx.f[29].f64) as f32) as f64);
	// 832C6D94: EFA0C138  fmsubs f29, f0, f4, f24
	ctx.f[29].f64 = (((ctx.f[0].f64 * ctx.f[4].f64 - ctx.f[24].f64) as f32) as f64);
	// 832C6D98: EC7EC8F8  fmsubs f3, f30, f3, f25
	ctx.f[3].f64 = (((ctx.f[30].f64 * ctx.f[3].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C6D9C: ED3CDA78  fmsubs f9, f28, f9, f27
	ctx.f[9].f64 = (((ctx.f[28].f64 * ctx.f[9].f64 - ctx.f[27].f64) as f32) as f64);
	// 832C6DA0: ED5F3AB8  fmsubs f10, f31, f10, f7
	ctx.f[10].f64 = (((ctx.f[31].f64 * ctx.f[10].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C6DA4: ECE8593A  fmadds f7, f8, f4, f11
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[4].f64 + ctx.f[11].f64) as f32) as f64);
	// 832C6DA8: EC81282A  fadds f4, f1, f5
	ctx.f[4].f64 = ((ctx.f[1].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C6DAC: D09C0004  stfs f4, 4(r28)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6DB0: ECA50828  fsubs f5, f5, f1
	ctx.f[5].f64 = (((ctx.f[5].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C6DB4: ED63482A  fadds f11, f3, f9
	ctx.f[11].f64 = ((ctx.f[3].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C6DB8: D17C0000  stfs f11, 0(r28)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6DBC: EC891828  fsubs f4, f9, f3
	ctx.f[4].f64 = (((ctx.f[9].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C6DC0: D0BD0004  stfs f5, 4(r29)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6DC4: D09D0000  stfs f4, 0(r29)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6DC8: EC67682A  fadds f3, f7, f13
	ctx.f[3].f64 = ((ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C6DCC: EC3D502A  fadds f1, f29, f10
	ctx.f[1].f64 = ((ctx.f[29].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6DD0: D03E0004  stfs f1, 4(r30)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6DD4: D07E0000  stfs f3, 0(r30)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6DD8: EDAD3828  fsubs f13, f13, f7
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C6DDC: ED6AE828  fsubs f11, f10, f29
	ctx.f[11].f64 = (((ctx.f[10].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C6DE0: D1BF0000  stfs f13, 0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6DE4: D17F0004  stfs f11, 4(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6DE8: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 832C6DEC: C1270004  lfs f9, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C6DF0: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 832C6DF4: C1490000  lfs f10, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C6DF8: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 832C6DFC: C0A80004  lfs f5, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C6E00: C0EA0000  lfs f7, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C6E04: C0880000  lfs f4, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C6E08: C06A0004  lfs f3, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C6E0C: C0290004  lfs f1, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C6E10: C1A70000  lfs f13, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6E14: ED6D082A  fadds f11, f13, f1
	ctx.f[11].f64 = ((ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C6E18: EFAA4828  fsubs f29, f10, f9
	ctx.f[29].f64 = (((ctx.f[10].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C6E1C: ED49502A  fadds f10, f9, f10
	ctx.f[10].f64 = ((ctx.f[9].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6E20: ED216828  fsubs f9, f1, f13
	ctx.f[9].f64 = (((ctx.f[1].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C6E24: EC272828  fsubs f1, f7, f5
	ctx.f[1].f64 = (((ctx.f[7].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C6E28: ECE5382A  fadds f7, f5, f7
	ctx.f[7].f64 = ((ctx.f[5].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C6E2C: EDA4182A  fadds f13, f4, f3
	ctx.f[13].f64 = ((ctx.f[4].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C6E30: EF7E02F2  fmuls f27, f30, f11
	ctx.f[27].f64 = (((ctx.f[30].f64 * ctx.f[11].f64) as f32) as f64);
	// 832C6E34: ECBE0772  fmuls f5, f30, f29
	ctx.f[5].f64 = (((ctx.f[30].f64 * ctx.f[29].f64) as f32) as f64);
	// 832C6E38: EC832028  fsubs f4, f3, f4
	ctx.f[4].f64 = (((ctx.f[3].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C6E3C: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 832C6E40: EFDC0072  fmuls f30, f28, f1
	ctx.f[30].f64 = (((ctx.f[28].f64 * ctx.f[1].f64) as f32) as f64);
	// 832C6E44: EC6802B2  fmuls f3, f8, f10
	ctx.f[3].f64 = (((ctx.f[8].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C6E48: EFACDF78  fmsubs f29, f12, f29, f27
	ctx.f[29].f64 = (((ctx.f[12].f64 * ctx.f[29].f64 - ctx.f[27].f64) as f32) as f64);
	// 832C6E4C: EF9C0372  fmuls f28, f28, f13
	ctx.f[28].f64 = (((ctx.f[28].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C6E50: EF6601F2  fmuls f27, f6, f7
	ctx.f[27].f64 = (((ctx.f[6].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C6E54: ED4002B2  fmuls f10, f0, f10
	ctx.f[10].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C6E58: ECFF01F2  fmuls f7, f31, f7
	ctx.f[7].f64 = (((ctx.f[31].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C6E5C: ECAC2AFA  fmadds f5, f12, f11, f5
	ctx.f[5].f64 = (((ctx.f[12].f64 * ctx.f[11].f64 + ctx.f[5].f64) as f32) as f64);
	// 832C6E60: EDA2F37A  fmadds f13, f2, f13, f30
	ctx.f[13].f64 = (((ctx.f[2].f64 * ctx.f[13].f64 + ctx.f[30].f64) as f32) as f64);
	// 832C6E64: ED601A7A  fmadds f11, f0, f9, f3
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[9].f64 + ctx.f[3].f64) as f32) as f64);
	// 832C6E68: ED82E078  fmsubs f12, f2, f1, f28
	ctx.f[12].f64 = (((ctx.f[2].f64 * ctx.f[1].f64 - ctx.f[28].f64) as f32) as f64);
	// 832C6E6C: ED485278  fmsubs f10, f8, f9, f10
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[9].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C6E70: ED063938  fmsubs f8, f6, f4, f7
	ctx.f[8].f64 = (((ctx.f[6].f64 * ctx.f[4].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C6E74: ED3FD93A  fmadds f9, f31, f4, f27
	ctx.f[9].f64 = (((ctx.f[31].f64 * ctx.f[4].f64 + ctx.f[27].f64) as f32) as f64);
	// 832C6E78: ECED282A  fadds f7, f13, f5
	ctx.f[7].f64 = ((ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C6E7C: D0E90004  stfs f7, 4(r9)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6E80: ECA56828  fsubs f5, f5, f13
	ctx.f[5].f64 = (((ctx.f[5].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C6E84: ECCCE82A  fadds f6, f12, f29
	ctx.f[6].f64 = ((ctx.f[12].f64 + ctx.f[29].f64) as f32) as f64;
	// 832C6E88: D0C90000  stfs f6, 0(r9)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6E8C: EC9D6028  fsubs f4, f29, f12
	ctx.f[4].f64 = (((ctx.f[29].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C6E90: D0AA0004  stfs f5, 4(r10)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6E94: D08A0000  stfs f4, 0(r10)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6E98: 394AFFF8  addi r10, r10, -8
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	// 832C6E9C: EC48502A  fadds f2, f8, f10
	ctx.f[2].f64 = ((ctx.f[8].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6EA0: D0470004  stfs f2, 4(r7)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6EA4: EC69582A  fadds f3, f9, f11
	ctx.f[3].f64 = ((ctx.f[9].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C6EA8: D0670000  stfs f3, 0(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6EAC: EC2A4028  fsubs f1, f10, f8
	ctx.f[1].f64 = (((ctx.f[10].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C6EB0: D0280004  stfs f1, 4(r8)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6EB4: EC0B4828  fsubs f0, f11, f9
	ctx.f[0].f64 = (((ctx.f[11].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C6EB8: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6EBC: 3929FFF8  addi r9, r9, -8
	ctx.r[9].s64 = ctx.r[9].s64 + -8;
	// 832C6EC0: 3908FFF8  addi r8, r8, -8
	ctx.r[8].s64 = ctx.r[8].s64 + -8;
	// 832C6EC4: 38E7FFF8  addi r7, r7, -8
	ctx.r[7].s64 = ctx.r[7].s64 + -8;
	// 832C6EC8: 4082FE2C  bne 0x832c6cf4
	if !ctx.cr[0].eq {
	pc = 0x832C6CF4; continue 'dispatch;
	}
	pc = 0x832C6ECC; continue 'dispatch;
            }
            0x832C6ECC => {
    //   block [0x832C6ECC..0x832C6FD0)
	// 832C6ECC: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 832C6ED0: 7C182C2E  lfsx f0, r24, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C6ED4: 5768103A  slwi r8, r27, 2
	ctx.r[8].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C6ED8: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832C6EDC: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C6EE0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 832C6EE4: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C6EE8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6EEC: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C6EF0: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 832C6EF4: 7D2B2214  add r9, r11, r4
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 832C6EF8: 7D672214  add r11, r7, r4
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 832C6EFC: 7CF82A14  add r7, r24, r5
	ctx.r[7].u64 = ctx.r[24].u64 + ctx.r[5].u64;
	// 832C6F00: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C6F04: C1280004  lfs f9, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C6F08: C10A0004  lfs f8, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C6F0C: ECED482A  fadds f7, f13, f9
	ctx.f[7].f64 = ((ctx.f[13].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C6F10: C1480000  lfs f10, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C6F14: ED296828  fsubs f9, f9, f13
	ctx.f[9].f64 = (((ctx.f[9].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C6F18: C1890000  lfs f12, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C6F1C: ECAA4028  fsubs f5, f10, f8
	ctx.f[5].f64 = (((ctx.f[10].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C6F20: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C6F24: ED48502A  fadds f10, f8, f10
	ctx.f[10].f64 = ((ctx.f[8].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C6F28: C0C90004  lfs f6, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C6F2C: C08B0004  lfs f4, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C6F30: EC6B3028  fsubs f3, f11, f6
	ctx.f[3].f64 = (((ctx.f[11].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C6F34: EC44602A  fadds f2, f4, f12
	ctx.f[2].f64 = ((ctx.f[4].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C6F38: C0270004  lfs f1, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C6F3C: ED06582A  fadds f8, f6, f11
	ctx.f[8].f64 = ((ctx.f[6].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C6F40: ECC46028  fsubs f6, f4, f12
	ctx.f[6].f64 = (((ctx.f[4].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C6F44: EC8101F2  fmuls f4, f1, f7
	ctx.f[4].f64 = (((ctx.f[1].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C6F48: EFC00272  fmuls f30, f0, f9
	ctx.f[30].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C6F4C: EDA10172  fmuls f13, f1, f5
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[5].f64) as f32) as f64);
	// 832C6F50: EFE002B2  fmuls f31, f0, f10
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C6F54: ED8000F2  fmuls f12, f0, f3
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[3].f64) as f32) as f64);
	// 832C6F58: ED6000B2  fmuls f11, f0, f2
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 832C6F5C: EFA10232  fmuls f29, f1, f8
	ctx.f[29].f64 = (((ctx.f[1].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C6F60: EF8101B2  fmuls f28, f1, f6
	ctx.f[28].f64 = (((ctx.f[1].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C6F64: ECA02178  fmsubs f5, f0, f5, f4
	ctx.f[5].f64 = (((ctx.f[0].f64 * ctx.f[5].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C6F68: EC8069FA  fmadds f4, f0, f7, f13
	ctx.f[4].f64 = (((ctx.f[0].f64 * ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64);
	// 832C6F6C: EC4160BA  fmadds f2, f1, f2, f12
	ctx.f[2].f64 = (((ctx.f[1].f64 * ctx.f[2].f64 + ctx.f[12].f64) as f32) as f64);
	// 832C6F70: EDA158F8  fmsubs f13, f1, f3, f11
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[3].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C6F74: ED61F2B8  fmsubs f11, f1, f10, f30
	ctx.f[11].f64 = (((ctx.f[1].f64 * ctx.f[10].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C6F78: ED81FA7A  fmadds f12, f1, f9, f31
	ctx.f[12].f64 = (((ctx.f[1].f64 * ctx.f[9].f64 + ctx.f[31].f64) as f32) as f64);
	// 832C6F7C: ED40E9BA  fmadds f10, f0, f6, f29
	ctx.f[10].f64 = (((ctx.f[0].f64 * ctx.f[6].f64 + ctx.f[29].f64) as f32) as f64);
	// 832C6F80: ED20E238  fmsubs f9, f0, f8, f28
	ctx.f[9].f64 = (((ctx.f[0].f64 * ctx.f[8].f64 - ctx.f[28].f64) as f32) as f64);
	// 832C6F84: ED02202A  fadds f8, f2, f4
	ctx.f[8].f64 = ((ctx.f[2].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C6F88: D1080004  stfs f8, 4(r8)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6F8C: ECED282A  fadds f7, f13, f5
	ctx.f[7].f64 = ((ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C6F90: D0E80000  stfs f7, 0(r8)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6F94: ECC56828  fsubs f6, f5, f13
	ctx.f[6].f64 = (((ctx.f[5].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C6F98: D0CB0000  stfs f6, 0(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6F9C: ECA41028  fsubs f5, f4, f2
	ctx.f[5].f64 = (((ctx.f[4].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C6FA0: D0AB0004  stfs f5, 4(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6FA4: EC8C5028  fsubs f4, f12, f10
	ctx.f[4].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C6FA8: D08A0004  stfs f4, 4(r10)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6FAC: EC6B4828  fsubs f3, f11, f9
	ctx.f[3].f64 = (((ctx.f[11].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C6FB0: D06A0000  stfs f3, 0(r10)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6FB4: EC49582A  fadds f2, f9, f11
	ctx.f[2].f64 = ((ctx.f[9].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C6FB8: D0490000  stfs f2, 0(r9)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C6FBC: EC2A602A  fadds f1, f10, f12
	ctx.f[1].f64 = ((ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C6FC0: D0290004  stfs f1, 4(r9)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C6FC4: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 832C6FC8: 4B9E6D4D  bl 0x82cadd14
	ctx.lr = 0x832C6FCC;
	sub_82CADCEC(ctx, base);
	// 832C6FCC: 4B9E247C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C6FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C6FD0 size=160
    let mut pc: u32 = 0x832C6FD0;
    'dispatch: loop {
        match pc {
            0x832C6FD0 => {
    //   block [0x832C6FD0..0x832C7038)
	// 832C6FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C6FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832C6FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C6FDC: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 832C6FE0: 2F030080  cmpwi cr6, r3, 0x80
	ctx.cr[6].compare_i32(ctx.r[3].s32, 128, &mut ctx.xer);
	// 832C6FE4: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 832C6FE8: 409A0050  bne cr6, 0x832c7038
	if !ctx.cr[6].eq {
	pc = 0x832C7038; continue 'dispatch;
	}
	// 832C6FEC: 3965FFF8  addi r11, r5, -8
	ctx.r[11].s64 = ctx.r[5].s64 + -8;
	// 832C6FF0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C6FF4: 7D2B3214  add r9, r11, r6
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 832C6FF8: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 832C6FFC: 4800011D  bl 0x832c7118
	ctx.lr = 0x832C7000;
	sub_832C7118(ctx, base);
	// 832C7000: 3905FFE0  addi r8, r5, -0x20
	ctx.r[8].s64 = ctx.r[5].s64 + -32;
	// 832C7004: 386A0080  addi r3, r10, 0x80
	ctx.r[3].s64 = ctx.r[10].s64 + 128;
	// 832C7008: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C700C: 7C8B3214  add r4, r11, r6
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 832C7010: 48000519  bl 0x832c7528
	ctx.lr = 0x832C7014;
	sub_832C7528(ctx, base);
	// 832C7014: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 832C7018: 386A0100  addi r3, r10, 0x100
	ctx.r[3].s64 = ctx.r[10].s64 + 256;
	// 832C701C: 480000FD  bl 0x832c7118
	ctx.lr = 0x832C7020;
	sub_832C7118(ctx, base);
	// 832C7020: 386A0180  addi r3, r10, 0x180
	ctx.r[3].s64 = ctx.r[10].s64 + 384;
	// 832C7024: 480000F5  bl 0x832c7118
	ctx.lr = 0x832C7028;
	sub_832C7118(ctx, base);
	// 832C7028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832C702C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C7030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832C7034: 4E800020  blr
	return;
            }
            0x832C7038 => {
    //   block [0x832C7038..0x832C7070)
	// 832C7038: 3965FFF0  addi r11, r5, -0x10
	ctx.r[11].s64 = ctx.r[5].s64 + -16;
	// 832C703C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C7040: 7C8B3214  add r4, r11, r6
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 832C7044: 480009FD  bl 0x832c7a40
	ctx.lr = 0x832C7048;
	sub_832C7A40(ctx, base);
	// 832C7048: 386A0040  addi r3, r10, 0x40
	ctx.r[3].s64 = ctx.r[10].s64 + 64;
	// 832C704C: 48000B7D  bl 0x832c7bc8
	ctx.lr = 0x832C7050;
	sub_832C7BC8(ctx, base);
	// 832C7050: 386A0080  addi r3, r10, 0x80
	ctx.r[3].s64 = ctx.r[10].s64 + 128;
	// 832C7054: 480009ED  bl 0x832c7a40
	ctx.lr = 0x832C7058;
	sub_832C7A40(ctx, base);
	// 832C7058: 386A00C0  addi r3, r10, 0xc0
	ctx.r[3].s64 = ctx.r[10].s64 + 192;
	// 832C705C: 480009E5  bl 0x832c7a40
	ctx.lr = 0x832C7060;
	sub_832C7A40(ctx, base);
	// 832C7060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832C7064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C7068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832C706C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C7070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C7070 size=168
    let mut pc: u32 = 0x832C7070;
    'dispatch: loop {
        match pc {
            0x832C7070 => {
    //   block [0x832C7070..0x832C70E0)
	// 832C7070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C7074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832C7078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C707C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 832C7080: 2F030080  cmpwi cr6, r3, 0x80
	ctx.cr[6].compare_i32(ctx.r[3].s32, 128, &mut ctx.xer);
	// 832C7084: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 832C7088: 409A0058  bne cr6, 0x832c70e0
	if !ctx.cr[6].eq {
	pc = 0x832C70E0; continue 'dispatch;
	}
	// 832C708C: 3965FFF8  addi r11, r5, -8
	ctx.r[11].s64 = ctx.r[5].s64 + -8;
	// 832C7090: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C7094: 7D2B3214  add r9, r11, r6
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 832C7098: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 832C709C: 4800007D  bl 0x832c7118
	ctx.lr = 0x832C70A0;
	sub_832C7118(ctx, base);
	// 832C70A0: 3905FFE0  addi r8, r5, -0x20
	ctx.r[8].s64 = ctx.r[5].s64 + -32;
	// 832C70A4: 386A0080  addi r3, r10, 0x80
	ctx.r[3].s64 = ctx.r[10].s64 + 128;
	// 832C70A8: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C70AC: 7D0B3214  add r8, r11, r6
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 832C70B0: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 832C70B4: 48000475  bl 0x832c7528
	ctx.lr = 0x832C70B8;
	sub_832C7528(ctx, base);
	// 832C70B8: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 832C70BC: 386A0100  addi r3, r10, 0x100
	ctx.r[3].s64 = ctx.r[10].s64 + 256;
	// 832C70C0: 48000059  bl 0x832c7118
	ctx.lr = 0x832C70C4;
	sub_832C7118(ctx, base);
	// 832C70C4: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 832C70C8: 386A0180  addi r3, r10, 0x180
	ctx.r[3].s64 = ctx.r[10].s64 + 384;
	// 832C70CC: 4800045D  bl 0x832c7528
	ctx.lr = 0x832C70D0;
	sub_832C7528(ctx, base);
	// 832C70D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832C70D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C70D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832C70DC: 4E800020  blr
	return;
            }
            0x832C70E0 => {
    //   block [0x832C70E0..0x832C7118)
	// 832C70E0: 3965FFF0  addi r11, r5, -0x10
	ctx.r[11].s64 = ctx.r[5].s64 + -16;
	// 832C70E4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C70E8: 7C8B3214  add r4, r11, r6
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 832C70EC: 48000955  bl 0x832c7a40
	ctx.lr = 0x832C70F0;
	sub_832C7A40(ctx, base);
	// 832C70F0: 386A0040  addi r3, r10, 0x40
	ctx.r[3].s64 = ctx.r[10].s64 + 64;
	// 832C70F4: 48000AD5  bl 0x832c7bc8
	ctx.lr = 0x832C70F8;
	sub_832C7BC8(ctx, base);
	// 832C70F8: 386A0080  addi r3, r10, 0x80
	ctx.r[3].s64 = ctx.r[10].s64 + 128;
	// 832C70FC: 48000945  bl 0x832c7a40
	ctx.lr = 0x832C7100;
	sub_832C7A40(ctx, base);
	// 832C7100: 386A00C0  addi r3, r10, 0xc0
	ctx.r[3].s64 = ctx.r[10].s64 + 192;
	// 832C7104: 48000AC5  bl 0x832c7bc8
	ctx.lr = 0x832C7108;
	sub_832C7BC8(ctx, base);
	// 832C7108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832C710C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C7110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832C7114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C7118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C7118 size=1036
    let mut pc: u32 = 0x832C7118;
    'dispatch: loop {
        match pc {
            0x832C7118 => {
    //   block [0x832C7118..0x832C7524)
	// 832C7118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C711C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832C7120: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 832C7124: 4B9E6B7D  bl 0x82cadca0
	ctx.lr = 0x832C7128;
	sub_82CADCA0(ctx, base);
	// 832C7128: C0030048  lfs f0, 0x48(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C712C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7130: C1830068  lfs f12, 0x68(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7134: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C7138: C1430028  lfs f10, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C713C: ED2D002A  fadds f9, f13, f0
	ctx.f[9].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C7140: C083004C  lfs f4, 0x4c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C7144: ECEA6028  fsubs f7, f10, f12
	ctx.f[7].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C7148: C043000C  lfs f2, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C714C: ECAA602A  fadds f5, f10, f12
	ctx.f[5].f64 = ((ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C7150: C103006C  lfs f8, 0x6c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C7154: EDA22028  fsubs f13, f2, f4
	ctx.f[13].f64 = (((ctx.f[2].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C7158: C0C3002C  lfs f6, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C715C: ED42202A  fadds f10, f2, f4
	ctx.f[10].f64 = ((ctx.f[2].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C7160: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7164: EC664028  fsubs f3, f6, f8
	ctx.f[3].f64 = (((ctx.f[6].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C7168: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C716C: EC26402A  fadds f1, f6, f8
	ctx.f[1].f64 = ((ctx.f[6].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C7170: ED000332  fmuls f8, f0, f12
	ctx.f[8].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 832C7174: C0830000  lfs f4, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C7178: C3C30020  lfs f30, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C717C: C0C30040  lfs f6, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C7180: C0430060  lfs f2, 0x60(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C7184: EFE4302A  fadds f31, f4, f6
	ctx.f[31].f64 = ((ctx.f[4].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7188: EFBE102A  fadds f29, f30, f2
	ctx.f[29].f64 = ((ctx.f[30].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C718C: C3830024  lfs f28, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C7190: ECC43028  fsubs f6, f4, f6
	ctx.f[6].f64 = (((ctx.f[4].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C7194: C0830064  lfs f4, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C7198: EF07682A  fadds f24, f7, f13
	ctx.f[24].f64 = ((ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C719C: C3430004  lfs f26, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 832C71A0: EC5E1028  fsubs f2, f30, f2
	ctx.f[2].f64 = (((ctx.f[30].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C71A4: C3630044  lfs f27, 0x44(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 832C71A8: EFCB1828  fsubs f30, f11, f3
	ctx.f[30].f64 = (((ctx.f[11].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C71AC: C2E30050  lfs f23, 0x50(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 832C71B0: EDAD3828  fsubs f13, f13, f7
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C71B4: C0E30010  lfs f7, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C71B8: EF3C2028  fsubs f25, f28, f4
	ctx.f[25].f64 = (((ctx.f[28].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C71BC: C2C30054  lfs f22, 0x54(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 832C71C0: EC00402A  fadds f0, f0, f8
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C71C4: D181FF44  stfs f12, -0xbc(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 832C71C8: EEBAD82A  fadds f21, f26, f27
	ctx.f[21].f64 = ((ctx.f[26].f64 + ctx.f[27].f64) as f32) as f64;
	// 832C71CC: C2830070  lfs f20, 0x70(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 832C71D0: EC9C202A  fadds f4, f28, f4
	ctx.f[4].f64 = ((ctx.f[28].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C71D4: C3830014  lfs f28, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C71D8: EF7AD828  fsubs f27, f26, f27
	ctx.f[27].f64 = (((ctx.f[26].f64 - ctx.f[27].f64) as f32) as f64);
	// 832C71DC: C3430030  lfs f26, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 832C71E0: EE5DF82A  fadds f18, f29, f31
	ctx.f[18].f64 = ((ctx.f[29].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C71E4: D241FF54  stfs f18, -0xac(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 832C71E8: EFFFE828  fsubs f31, f31, f29
	ctx.f[31].f64 = (((ctx.f[31].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C71EC: D3E1FF50  stfs f31, -0xb0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 832C71F0: C3E3003C  lfs f31, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C71F4: EFB80232  fmuls f29, f24, f8
	ctx.f[29].f64 = (((ctx.f[24].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C71F8: EE3E0232  fmuls f17, f30, f8
	ctx.f[17].f64 = (((ctx.f[30].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C71FC: D3E1FF40  stfs f31, -0xc0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 832C7200: EE4D0232  fmuls f18, f13, f8
	ctx.f[18].f64 = (((ctx.f[13].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C7204: C2630034  lfs f19, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 832C7208: EC63582A  fadds f3, f3, f11
	ctx.f[3].f64 = ((ctx.f[3].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C720C: C1630074  lfs f11, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C7210: EFE6C828  fsubs f31, f6, f25
	ctx.f[31].f64 = (((ctx.f[6].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C7214: C203005C  lfs f16, 0x5c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) };
	ctx.f[16].f64 = (tmp.f32 as f64);
	// 832C7218: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C721C: C1E3001C  lfs f15, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[15].f64 = (tmp.f32 as f64);
	// 832C7220: ECD9302A  fadds f6, f25, f6
	ctx.f[6].f64 = ((ctx.f[25].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7224: C1C3007C  lfs f14, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[14].f64 = (tmp.f32 as f64);
	// 832C7228: EF24A82A  fadds f25, f4, f21
	ctx.f[25].f64 = ((ctx.f[4].f64 + ctx.f[21].f64) as f32) as f64;
	// 832C722C: C1830078  lfs f12, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7230: EC952028  fsubs f4, f21, f4
	ctx.f[4].f64 = (((ctx.f[21].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C7234: EEA2D82A  fadds f21, f2, f27
	ctx.f[21].f64 = ((ctx.f[2].f64 + ctx.f[27].f64) as f32) as f64;
	// 832C7238: EC5B1028  fsubs f2, f27, f2
	ctx.f[2].f64 = (((ctx.f[27].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C723C: EF65482A  fadds f27, f5, f9
	ctx.f[27].f64 = ((ctx.f[5].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7240: ED292828  fsubs f9, f9, f5
	ctx.f[9].f64 = (((ctx.f[9].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7244: ECA1502A  fadds f5, f1, f10
	ctx.f[5].f64 = ((ctx.f[1].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7248: EC2A0828  fsubs f1, f10, f1
	ctx.f[1].f64 = (((ctx.f[10].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C724C: ED58883A  fmadds f10, f24, f0, f17
	ctx.f[10].f64 = (((ctx.f[24].f64 * ctx.f[0].f64 + ctx.f[17].f64) as f32) as f64);
	// 832C7250: EFDEE838  fmsubs f30, f30, f0, f29
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[0].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C7254: EDA36A38  fmsubs f13, f3, f8, f13
	ctx.f[13].f64 = (((ctx.f[3].f64 * ctx.f[8].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C7258: EFA7B82A  fadds f29, f7, f23
	ctx.f[29].f64 = ((ctx.f[7].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C725C: EF1CB02A  fadds f24, f28, f22
	ctx.f[24].f64 = ((ctx.f[28].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C7260: EC63903A  fmadds f3, f3, f0, f18
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[0].f64 + ctx.f[18].f64) as f32) as f64);
	// 832C7264: ECE7B828  fsubs f7, f7, f23
	ctx.f[7].f64 = (((ctx.f[7].f64 - ctx.f[23].f64) as f32) as f64);
	// 832C7268: EF9CB028  fsubs f28, f28, f22
	ctx.f[28].f64 = (((ctx.f[28].f64 - ctx.f[22].f64) as f32) as f64);
	// 832C726C: EEFAA02A  fadds f23, f26, f20
	ctx.f[23].f64 = ((ctx.f[26].f64 + ctx.f[20].f64) as f32) as f64;
	// 832C7270: D361FF5C  stfs f27, -0xa4(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 832C7274: EED3582A  fadds f22, f19, f11
	ctx.f[22].f64 = ((ctx.f[19].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C7278: D0A1FF60  stfs f5, -0xa0(r1)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 832C727C: EF5AA028  fsubs f26, f26, f20
	ctx.f[26].f64 = (((ctx.f[26].f64 - ctx.f[20].f64) as f32) as f64);
	// 832C7280: C2830038  lfs f20, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 832C7284: ED735828  fsubs f11, f19, f11
	ctx.f[11].f64 = (((ctx.f[19].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7288: C361FF40  lfs f27, -0xc0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-192 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 832C728C: EE6F8028  fsubs f19, f15, f16
	ctx.f[19].f64 = (((ctx.f[15].f64 - ctx.f[16].f64) as f32) as f64);
	// 832C7290: C2430058  lfs f18, 0x58(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 832C7294: EE346028  fsubs f17, f20, f12
	ctx.f[17].f64 = (((ctx.f[20].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C7298: C0A30018  lfs f5, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C729C: D321FF58  stfs f25, -0xa8(r1)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 832C72A0: EF3B7028  fsubs f25, f27, f14
	ctx.f[25].f64 = (((ctx.f[27].f64 - ctx.f[14].f64) as f32) as f64);
	// 832C72A4: D081FF4C  stfs f4, -0xb4(r1)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 832C72A8: EC859028  fsubs f4, f5, f18
	ctx.f[4].f64 = (((ctx.f[5].f64 - ctx.f[18].f64) as f32) as f64);
	// 832C72AC: D121FF40  stfs f9, -0xc0(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 832C72B0: ED94602A  fadds f12, f20, f12
	ctx.f[12].f64 = ((ctx.f[20].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C72B4: D021FF48  stfs f1, -0xb8(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 832C72B8: ECA5902A  fadds f5, f5, f18
	ctx.f[5].f64 = ((ctx.f[5].f64 + ctx.f[18].f64) as f32) as f64;
	// 832C72BC: EC3AE02A  fadds f1, f26, f28
	ctx.f[1].f64 = ((ctx.f[26].f64 + ctx.f[28].f64) as f32) as f64;
	// 832C72C0: ED275828  fsubs f9, f7, f11
	ctx.f[9].f64 = (((ctx.f[7].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C72C4: ECEB382A  fadds f7, f11, f7
	ctx.f[7].f64 = ((ctx.f[11].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C72C8: EF9CD028  fsubs f28, f28, f26
	ctx.f[28].f64 = (((ctx.f[28].f64 - ctx.f[26].f64) as f32) as f64);
	// 832C72CC: EF51982A  fadds f26, f17, f19
	ctx.f[26].f64 = ((ctx.f[17].f64 + ctx.f[19].f64) as f32) as f64;
	// 832C72D0: EE938828  fsubs f20, f19, f17
	ctx.f[20].f64 = (((ctx.f[19].f64 - ctx.f[17].f64) as f32) as f64);
	// 832C72D4: EE79202A  fadds f19, f25, f4
	ctx.f[19].f64 = ((ctx.f[25].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C72D8: EC84C828  fsubs f4, f4, f25
	ctx.f[4].f64 = (((ctx.f[4].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C72DC: ED7B702A  fadds f11, f27, f14
	ctx.f[11].f64 = ((ctx.f[27].f64 + ctx.f[14].f64) as f32) as f64;
	// 832C72E0: EF6F802A  fadds f27, f15, f16
	ctx.f[27].f64 = ((ctx.f[15].f64 + ctx.f[16].f64) as f32) as f64;
	// 832C72E4: C1E1FF44  lfs f15, -0xbc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-188 as u32) ) };
	ctx.f[15].f64 = (tmp.f32 as f64);
	// 832C72E8: D1E1FF44  stfs f15, -0xbc(r1)
	tmp.f32 = (ctx.f[15].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 832C72EC: EDC56028  fsubs f14, f5, f12
	ctx.f[14].f64 = (((ctx.f[5].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C72F0: EF290828  fsubs f25, f9, f1
	ctx.f[25].f64 = (((ctx.f[9].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C72F4: EC21482A  fadds f1, f1, f9
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C72F8: ED3C382A  fadds f9, f28, f7
	ctx.f[9].f64 = ((ctx.f[28].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C72FC: ECFC3828  fsubs f7, f28, f7
	ctx.f[7].f64 = (((ctx.f[28].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7300: EE5A0032  fmuls f18, f26, f0
	ctx.f[18].f64 = (((ctx.f[26].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C7304: EF5A0232  fmuls f26, f26, f8
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C7308: EE340232  fmuls f17, f20, f8
	ctx.f[17].f64 = (((ctx.f[20].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C730C: EE130232  fmuls f16, f19, f8
	ctx.f[16].f64 = (((ctx.f[19].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C7310: EF9B5828  fsubs f28, f27, f11
	ctx.f[28].f64 = (((ctx.f[27].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7314: ED6BD82A  fadds f11, f11, f27
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[27].f64) as f32) as f64;
	// 832C7318: EF3903F2  fmuls f25, f25, f15
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C731C: EC2103F2  fmuls f1, f1, f15
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7320: ED2903F2  fmuls f9, f9, f15
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7324: ECE703F2  fmuls f7, f7, f15
	ctx.f[7].f64 = (((ctx.f[7].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7328: ED049238  fmsubs f8, f4, f8, f18
	ctx.f[8].f64 = (((ctx.f[4].f64 * ctx.f[8].f64 - ctx.f[18].f64) as f32) as f64);
	// 832C732C: EC84D03A  fmadds f4, f4, f0, f26
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[26].f64) as f32) as f64);
	// 832C7330: EF538838  fmsubs f26, f19, f0, f17
	ctx.f[26].f64 = (((ctx.f[19].f64 * ctx.f[0].f64 - ctx.f[17].f64) as f32) as f64);
	// 832C7334: EC14803A  fmadds f0, f20, f0, f16
	ctx.f[0].f64 = (((ctx.f[20].f64 * ctx.f[0].f64 + ctx.f[16].f64) as f32) as f64);
	// 832C7338: EDF7E82A  fadds f15, f23, f29
	ctx.f[15].f64 = ((ctx.f[23].f64 + ctx.f[29].f64) as f32) as f64;
	// 832C733C: EE99F82A  fadds f20, f25, f31
	ctx.f[20].f64 = ((ctx.f[25].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C7340: EE61A82A  fadds f19, f1, f21
	ctx.f[19].f64 = ((ctx.f[1].f64 + ctx.f[21].f64) as f32) as f64;
	// 832C7344: EE464828  fsubs f18, f6, f9
	ctx.f[18].f64 = (((ctx.f[6].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C7348: EE27102A  fadds f17, f7, f2
	ctx.f[17].f64 = ((ctx.f[7].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C734C: ECE23828  fsubs f7, f2, f7
	ctx.f[7].f64 = (((ctx.f[2].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7350: EC48F02A  fadds f2, f8, f30
	ctx.f[2].f64 = ((ctx.f[8].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C7354: ECC9302A  fadds f6, f9, f6
	ctx.f[6].f64 = ((ctx.f[9].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7358: ED24502A  fadds f9, f4, f10
	ctx.f[9].f64 = ((ctx.f[4].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C735C: EE0DD028  fsubs f16, f13, f26
	ctx.f[16].f64 = (((ctx.f[13].f64 - ctx.f[26].f64) as f32) as f64);
	// 832C7360: EDBA682A  fadds f13, f26, f13
	ctx.f[13].f64 = ((ctx.f[26].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C7364: EF430028  fsubs f26, f3, f0
	ctx.f[26].f64 = (((ctx.f[3].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C7368: EC60182A  fadds f3, f0, f3
	ctx.f[3].f64 = ((ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C736C: EC1DB828  fsubs f0, f29, f23
	ctx.f[0].f64 = (((ctx.f[29].f64 - ctx.f[23].f64) as f32) as f64);
	// 832C7370: EFB6C02A  fadds f29, f22, f24
	ctx.f[29].f64 = ((ctx.f[22].f64 + ctx.f[24].f64) as f32) as f64;
	// 832C7374: EF18B028  fsubs f24, f24, f22
	ctx.f[24].f64 = (((ctx.f[24].f64 - ctx.f[22].f64) as f32) as f64);
	// 832C7378: EF62A02A  fadds f27, f2, f20
	ctx.f[27].f64 = ((ctx.f[2].f64 + ctx.f[20].f64) as f32) as f64;
	// 832C737C: D3630040  stfs f27, 0x40(r3)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 832C7380: EF69982A  fadds f27, f9, f19
	ctx.f[27].f64 = ((ctx.f[9].f64 + ctx.f[19].f64) as f32) as f64;
	// 832C7384: D3630044  stfs f27, 0x44(r3)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 832C7388: EF70902A  fadds f27, f16, f18
	ctx.f[27].f64 = ((ctx.f[16].f64 + ctx.f[18].f64) as f32) as f64;
	// 832C738C: D3630060  stfs f27, 0x60(r3)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 832C7390: EF728028  fsubs f27, f18, f16
	ctx.f[27].f64 = (((ctx.f[18].f64 - ctx.f[16].f64) as f32) as f64);
	// 832C7394: D3630068  stfs f27, 0x68(r3)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 832C7398: EF7A382A  fadds f27, f26, f7
	ctx.f[27].f64 = ((ctx.f[26].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C739C: D3630064  stfs f27, 0x64(r3)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 832C73A0: ECE7D028  fsubs f7, f7, f26
	ctx.f[7].f64 = (((ctx.f[7].f64 - ctx.f[26].f64) as f32) as f64);
	// 832C73A4: D0E3006C  stfs f7, 0x6c(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 832C73A8: ECED882A  fadds f7, f13, f17
	ctx.f[7].f64 = ((ctx.f[13].f64 + ctx.f[17].f64) as f32) as f64;
	// 832C73AC: D0E30074  stfs f7, 0x74(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 832C73B0: ECF41028  fsubs f7, f20, f2
	ctx.f[7].f64 = (((ctx.f[20].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C73B4: D0E30048  stfs f7, 0x48(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 832C73B8: EDB16828  fsubs f13, f17, f13
	ctx.f[13].f64 = (((ctx.f[17].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C73BC: D1A3007C  stfs f13, 0x7c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 832C73C0: EC534828  fsubs f2, f19, f9
	ctx.f[2].f64 = (((ctx.f[19].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C73C4: D043004C  stfs f2, 0x4c(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 832C73C8: EDA61828  fsubs f13, f6, f3
	ctx.f[13].f64 = (((ctx.f[6].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C73CC: C0E1FF40  lfs f7, -0xc0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-192 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C73D0: C041FF48  lfs f2, -0xb8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-184 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C73D4: ED350828  fsubs f9, f21, f1
	ctx.f[9].f64 = (((ctx.f[21].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C73D8: ED8C282A  fadds f12, f12, f5
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C73DC: D1A30070  stfs f13, 0x70(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 832C73E0: ECA7E028  fsubs f5, f7, f28
	ctx.f[5].f64 = (((ctx.f[7].f64 - ctx.f[28].f64) as f32) as f64);
	// 832C73E4: C2E1FF5C  lfs f23, -0xa4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-164 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 832C73E8: EDBC382A  fadds f13, f28, f7
	ctx.f[13].f64 = ((ctx.f[28].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C73EC: C2A1FF60  lfs f21, -0xa0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 832C73F0: EC2E102A  fadds f1, f14, f2
	ctx.f[1].f64 = ((ctx.f[14].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C73F4: ECE27028  fsubs f7, f2, f14
	ctx.f[7].f64 = (((ctx.f[2].f64 - ctx.f[14].f64) as f32) as f64);
	// 832C73F8: EC8A2028  fsubs f4, f10, f4
	ctx.f[4].f64 = (((ctx.f[10].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C73FC: C141FF4C  lfs f10, -0xb4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-180 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C7400: ED1E4028  fsubs f8, f30, f8
	ctx.f[8].f64 = (((ctx.f[30].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C7404: C3C1FF50  lfs f30, -0xb0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-176 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C7408: EF98F02A  fadds f28, f24, f30
	ctx.f[28].f64 = ((ctx.f[24].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C740C: EFDEC028  fsubs f30, f30, f24
	ctx.f[30].f64 = (((ctx.f[30].f64 - ctx.f[24].f64) as f32) as f64);
	// 832C7410: EE8BA82A  fadds f20, f11, f21
	ctx.f[20].f64 = ((ctx.f[11].f64 + ctx.f[21].f64) as f32) as f64;
	// 832C7414: EECCB82A  fadds f22, f12, f23
	ctx.f[22].f64 = ((ctx.f[12].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C7418: ED976028  fsubs f12, f23, f12
	ctx.f[12].f64 = (((ctx.f[23].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C741C: EC450828  fsubs f2, f5, f1
	ctx.f[2].f64 = (((ctx.f[5].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C7420: EF4D3828  fsubs f26, f13, f7
	ctx.f[26].f64 = (((ctx.f[13].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7424: EC21282A  fadds f1, f1, f5
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C7428: ECE7682A  fadds f7, f7, f13
	ctx.f[7].f64 = ((ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C742C: C1A1FF44  lfs f13, -0xbc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-188 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7430: ECBFC828  fsubs f5, f31, f25
	ctx.f[5].f64 = (((ctx.f[31].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C7434: C321FF58  lfs f25, -0xa8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-168 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 832C7438: EFEA0028  fsubs f31, f10, f0
	ctx.f[31].f64 = (((ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C743C: EC00502A  fadds f0, f0, f10
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7440: C141FF54  lfs f10, -0xac(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-172 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C7444: EF6F502A  fadds f27, f15, f10
	ctx.f[27].f64 = ((ctx.f[15].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7448: EF1DC82A  fadds f24, f29, f25
	ctx.f[24].f64 = ((ctx.f[29].f64 + ctx.f[25].f64) as f32) as f64;
	// 832C744C: ED4A7828  fsubs f10, f10, f15
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[15].f64) as f32) as f64);
	// 832C7450: EC420372  fmuls f2, f2, f13
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C7454: EF5A0372  fmuls f26, f26, f13
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C7458: EC210372  fmuls f1, f1, f13
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C745C: ECE70372  fmuls f7, f7, f13
	ctx.f[7].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C7460: EDB55828  fsubs f13, f21, f11
	ctx.f[13].f64 = (((ctx.f[21].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7464: ED63302A  fadds f11, f3, f6
	ctx.f[11].f64 = ((ctx.f[3].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7468: D1630078  stfs f11, 0x78(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 832C746C: ECC8482A  fadds f6, f8, f9
	ctx.f[6].f64 = ((ctx.f[8].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7470: D0C30054  stfs f6, 0x54(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 832C7474: EC694028  fsubs f3, f9, f8
	ctx.f[3].f64 = (((ctx.f[9].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C7478: D063005C  stfs f3, 0x5c(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 832C747C: ED652028  fsubs f11, f5, f4
	ctx.f[11].f64 = (((ctx.f[5].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C7480: D1630050  stfs f11, 0x50(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 832C7484: ED24282A  fadds f9, f4, f5
	ctx.f[9].f64 = ((ctx.f[4].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C7488: D1230058  stfs f9, 0x58(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 832C748C: ED02F02A  fadds f8, f2, f30
	ctx.f[8].f64 = ((ctx.f[2].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C7490: D1030020  stfs f8, 0x20(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 832C7494: ECBE1028  fsubs f5, f30, f2
	ctx.f[5].f64 = (((ctx.f[30].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C7498: D0A30028  stfs f5, 0x28(r3)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 832C749C: ECC1002A  fadds f6, f1, f0
	ctx.f[6].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C74A0: D0C30024  stfs f6, 0x24(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 832C74A4: EC800828  fsubs f4, f0, f1
	ctx.f[4].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C74A8: D083002C  stfs f4, 0x2c(r3)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 832C74AC: EC5C3828  fsubs f2, f28, f7
	ctx.f[2].f64 = (((ctx.f[28].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C74B0: D0430030  stfs f2, 0x30(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 832C74B4: EC27E02A  fadds f1, f7, f28
	ctx.f[1].f64 = ((ctx.f[7].f64 + ctx.f[28].f64) as f32) as f64;
	// 832C74B8: D0230038  stfs f1, 0x38(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 832C74BC: EC7AF82A  fadds f3, f26, f31
	ctx.f[3].f64 = ((ctx.f[26].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C74C0: D0630034  stfs f3, 0x34(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 832C74C4: EC1FD028  fsubs f0, f31, f26
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[26].f64) as f32) as f64);
	// 832C74C8: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 832C74CC: ED76D82A  fadds f11, f22, f27
	ctx.f[11].f64 = ((ctx.f[22].f64 + ctx.f[27].f64) as f32) as f64;
	// 832C74D0: D1630000  stfs f11, 0(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C74D4: ED34C02A  fadds f9, f20, f24
	ctx.f[9].f64 = ((ctx.f[20].f64 + ctx.f[24].f64) as f32) as f64;
	// 832C74D8: D1230004  stfs f9, 4(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C74DC: ED1BB028  fsubs f8, f27, f22
	ctx.f[8].f64 = (((ctx.f[27].f64 - ctx.f[22].f64) as f32) as f64);
	// 832C74E0: D1030008  stfs f8, 8(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C74E4: ECF8A028  fsubs f7, f24, f20
	ctx.f[7].f64 = (((ctx.f[24].f64 - ctx.f[20].f64) as f32) as f64);
	// 832C74E8: D0E3000C  stfs f7, 0xc(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C74EC: EFB9E828  fsubs f29, f25, f29
	ctx.f[29].f64 = (((ctx.f[25].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C74F0: ECCA6828  fsubs f6, f10, f13
	ctx.f[6].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C74F4: ECACE82A  fadds f5, f12, f29
	ctx.f[5].f64 = ((ctx.f[12].f64 + ctx.f[29].f64) as f32) as f64;
	// 832C74F8: D0C30010  stfs f6, 0x10(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 832C74FC: EC8D502A  fadds f4, f13, f10
	ctx.f[4].f64 = ((ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7500: D0A30014  stfs f5, 0x14(r3)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832C7504: EC7D6028  fsubs f3, f29, f12
	ctx.f[3].f64 = (((ctx.f[29].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C7508: D0830018  stfs f4, 0x18(r3)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 832C750C: D063001C  stfs f3, 0x1c(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 832C7510: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 832C7514: 4B9E67D9  bl 0x82cadcec
	ctx.lr = 0x832C7518;
	sub_82CADCEC(ctx, base);
	// 832C7518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C751C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832C7520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C7528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C7528 size=1304
    let mut pc: u32 = 0x832C7528;
    'dispatch: loop {
        match pc {
            0x832C7528 => {
    //   block [0x832C7528..0x832C7A40)
	// 832C7528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C752C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832C7530: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 832C7534: 4B9E676D  bl 0x82cadca0
	ctx.lr = 0x832C7538;
	sub_82CADCA0(ctx, base);
	// 832C7538: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C753C: C1A30064  lfs f13, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7540: C1830060  lfs f12, 0x60(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7544: ED606828  fsubs f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C7548: C1430024  lfs f10, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C754C: ED2D002A  fadds f9, f13, f0
	ctx.f[9].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C7550: ECEA602A  fadds f7, f10, f12
	ctx.f[7].f64 = ((ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C7554: C1030048  lfs f8, 0x48(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C7558: ECAA6028  fsubs f5, f10, f12
	ctx.f[5].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C755C: C0C3000C  lfs f6, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C7560: C0030028  lfs f0, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7564: EC66402A  fadds f3, f6, f8
	ctx.f[3].f64 = ((ctx.f[6].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C7568: C183006C  lfs f12, 0x6c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C756C: EC264028  fsubs f1, f6, f8
	ctx.f[1].f64 = (((ctx.f[6].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C7570: ED006028  fsubs f8, f0, f12
	ctx.f[8].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C7574: C0830068  lfs f4, 0x68(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C7578: C043002C  lfs f2, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C757C: EDA2202A  fadds f13, f2, f4
	ctx.f[13].f64 = ((ctx.f[2].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C7580: C0C40018  lfs f6, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C7584: C3640004  lfs f27, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 832C7588: ED422028  fsubs f10, f2, f4
	ctx.f[10].f64 = (((ctx.f[2].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C758C: C3230000  lfs f25, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 832C7590: C2C30004  lfs f22, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 832C7594: EFAB3828  fsubs f29, f11, f7
	ctx.f[29].f64 = (((ctx.f[11].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7598: C2E30040  lfs f23, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 832C759C: ED67582A  fadds f11, f7, f11
	ctx.f[11].f64 = ((ctx.f[7].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C75A0: C0440014  lfs f2, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C75A4: EF492828  fsubs f26, f9, f5
	ctx.f[26].f64 = (((ctx.f[9].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C75A8: C0840010  lfs f4, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C75AC: ECA5482A  fadds f5, f5, f9
	ctx.f[5].f64 = ((ctx.f[5].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C75B0: C1230044  lfs f9, 0x44(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C75B4: EE6801B2  fmuls f19, f8, f6
	ctx.f[19].f64 = (((ctx.f[8].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C75B8: C3C3004C  lfs f30, 0x4c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C75BC: EE394828  fsubs f17, f25, f9
	ctx.f[17].f64 = (((ctx.f[25].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C75C0: C3E30008  lfs f31, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C75C4: EDF6B82A  fadds f15, f22, f23
	ctx.f[15].f64 = ((ctx.f[22].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C75C8: C0E4001C  lfs f7, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C75CC: EF0300B2  fmuls f24, f3, f2
	ctx.f[24].f64 = (((ctx.f[3].f64 * ctx.f[2].f64) as f32) as f64);
	// 832C75D0: C2840020  lfs f20, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 832C75D4: ED29C82A  fadds f9, f9, f25
	ctx.f[9].f64 = ((ctx.f[9].f64 + ctx.f[25].f64) as f32) as f64;
	// 832C75D8: C2440024  lfs f18, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 832C75DC: EF9FF028  fsubs f28, f31, f30
	ctx.f[28].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C75E0: D141FF2C  stfs f10, -0xd4(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), tmp.u32 ) };
	// 832C75E4: EFBD06F2  fmuls f29, f29, f27
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[27].f64) as f32) as f64);
	// 832C75E8: D241FF28  stfs f18, -0xd8(r1)
	tmp.f32 = (ctx.f[18].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), tmp.u32 ) };
	// 832C75EC: ED6B06F2  fmuls f11, f11, f27
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[27].f64) as f32) as f64);
	// 832C75F0: D281FF20  stfs f20, -0xe0(r1)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), tmp.u32 ) };
	// 832C75F4: EEAD01B2  fmuls f21, f13, f6
	ctx.f[21].f64 = (((ctx.f[13].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C75F8: D361FF50  stfs f27, -0xb0(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 832C75FC: ECA506F2  fmuls f5, f5, f27
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[27].f64) as f32) as f64);
	// 832C7600: D081FF24  stfs f4, -0xdc(r1)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), tmp.u32 ) };
	// 832C7604: EC630132  fmuls f3, f3, f4
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[4].f64) as f32) as f64);
	// 832C7608: C2030050  lfs f16, 0x50(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[16].f64 = (tmp.f32 as f64);
	// 832C760C: EEF6B828  fsubs f23, f22, f23
	ctx.f[23].f64 = (((ctx.f[22].f64 - ctx.f[23].f64) as f32) as f64);
	// 832C7610: C1C30014  lfs f14, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[14].f64 = (tmp.f32 as f64);
	// 832C7614: EF5A06F2  fmuls f26, f26, f27
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[27].f64) as f32) as f64);
	// 832C7618: C1430010  lfs f10, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C761C: EDAD99FA  fmadds f13, f13, f7, f19
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[7].f64 + ctx.f[19].f64) as f32) as f64);
	// 832C7620: C2430054  lfs f18, 0x54(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 832C7624: EFFEF82A  fadds f31, f30, f31
	ctx.f[31].f64 = ((ctx.f[30].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C7628: C3230070  lfs f25, 0x70(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 832C762C: EF1CC138  fmsubs f24, f28, f4, f24
	ctx.f[24].f64 = (((ctx.f[28].f64 * ctx.f[4].f64 - ctx.f[24].f64) as f32) as f64);
	// 832C7630: C2C30034  lfs f22, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 832C7634: EE7D882A  fadds f19, f29, f17
	ctx.f[19].f64 = ((ctx.f[29].f64 + ctx.f[17].f64) as f32) as f64;
	// 832C7638: D261FF40  stfs f19, -0xc0(r1)
	tmp.f32 = (ctx.f[19].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 832C763C: EE6B782A  fadds f19, f11, f15
	ctx.f[19].f64 = ((ctx.f[11].f64 + ctx.f[15].f64) as f32) as f64;
	// 832C7640: C2830030  lfs f20, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 832C7644: ED6F5828  fsubs f11, f15, f11
	ctx.f[11].f64 = (((ctx.f[15].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7648: C3630074  lfs f27, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 832C764C: EDE92828  fsubs f15, f9, f5
	ctx.f[15].f64 = (((ctx.f[9].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7650: C0830058  lfs f4, 0x58(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C7654: EC7C18BA  fmadds f3, f28, f2, f3
	ctx.f[3].f64 = (((ctx.f[28].f64 * ctx.f[2].f64 + ctx.f[3].f64) as f32) as f64);
	// 832C7658: C383001C  lfs f28, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C765C: ED08A9F8  fmsubs f8, f8, f7, f21
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[7].f64 - ctx.f[21].f64) as f32) as f64);
	// 832C7660: C2A30018  lfs f21, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 832C7664: EFC101F2  fmuls f30, f1, f7
	ctx.f[30].f64 = (((ctx.f[1].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C7668: EFB1E828  fsubs f29, f17, f29
	ctx.f[29].f64 = (((ctx.f[17].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C766C: ED25482A  fadds f9, f5, f9
	ctx.f[9].f64 = ((ctx.f[5].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7670: EC2101B2  fmuls f1, f1, f6
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C7674: EE3AB82A  fadds f17, f26, f23
	ctx.f[17].f64 = ((ctx.f[26].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C7678: ECB7D028  fsubs f5, f23, f26
	ctx.f[5].f64 = (((ctx.f[23].f64 - ctx.f[26].f64) as f32) as f64);
	// 832C767C: EF48C02A  fadds f26, f8, f24
	ctx.f[26].f64 = ((ctx.f[8].f64 + ctx.f[24].f64) as f32) as f64;
	// 832C7680: D121FF5C  stfs f9, -0xa4(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 832C7684: ED184028  fsubs f8, f24, f8
	ctx.f[8].f64 = (((ctx.f[24].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C7688: D1E1FF54  stfs f15, -0xac(r1)
	tmp.f32 = (ctx.f[15].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 832C768C: EF0D182A  fadds f24, f13, f3
	ctx.f[24].f64 = ((ctx.f[13].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C7690: C2E3007C  lfs f23, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 832C7694: EC636828  fsubs f3, f3, f13
	ctx.f[3].f64 = (((ctx.f[3].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C7698: C1E3003C  lfs f15, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[15].f64 = (tmp.f32 as f64);
	// 832C769C: EDBFF1B8  fmsubs f13, f31, f6, f30
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[6].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C76A0: C1230078  lfs f9, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C76A4: EC3F09FA  fmadds f1, f31, f7, f1
	ctx.f[1].f64 = (((ctx.f[31].f64 * ctx.f[7].f64 + ctx.f[1].f64) as f32) as f64);
	// 832C76A8: C3E30038  lfs f31, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C76AC: D221FF58  stfs f17, -0xa8(r1)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 832C76B0: EE3C202A  fadds f17, f28, f4
	ctx.f[17].f64 = ((ctx.f[28].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C76B4: D0A1FF60  stfs f5, -0xa0(r1)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 832C76B8: ECB4D828  fsubs f5, f20, f27
	ctx.f[5].f64 = (((ctx.f[20].f64 - ctx.f[27].f64) as f32) as f64);
	// 832C76BC: D161FF4C  stfs f11, -0xb4(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 832C76C0: ED7FB828  fsubs f11, f31, f23
	ctx.f[11].f64 = (((ctx.f[31].f64 - ctx.f[23].f64) as f32) as f64);
	// 832C76C4: D3A1FF48  stfs f29, -0xb8(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 832C76C8: EFAF482A  fadds f29, f15, f9
	ctx.f[29].f64 = ((ctx.f[15].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C76CC: EFCE802A  fadds f30, f14, f16
	ctx.f[30].f64 = ((ctx.f[14].f64 + ctx.f[16].f64) as f32) as f64;
	// 832C76D0: D341FF44  stfs f26, -0xbc(r1)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 832C76D4: D301FF3C  stfs f24, -0xc4(r1)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), tmp.u32 ) };
	// 832C76D8: EF16C82A  fadds f24, f22, f25
	ctx.f[24].f64 = ((ctx.f[22].f64 + ctx.f[25].f64) as f32) as f64;
	// 832C76DC: C343005C  lfs f26, 0x5c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 832C76E0: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C76E4: EF36C828  fsubs f25, f22, f25
	ctx.f[25].f64 = (((ctx.f[22].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C76E8: D261FF38  stfs f19, -0xc8(r1)
	tmp.f32 = (ctx.f[19].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), tmp.u32 ) };
	// 832C76EC: EC9C2028  fsubs f4, f28, f4
	ctx.f[4].f64 = (((ctx.f[28].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C76F0: D101FF34  stfs f8, -0xcc(r1)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), tmp.u32 ) };
	// 832C76F4: ED8A9028  fsubs f12, f10, f18
	ctx.f[12].f64 = (((ctx.f[10].f64 - ctx.f[18].f64) as f32) as f64);
	// 832C76F8: C101FF28  lfs f8, -0xd8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-216 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C76FC: EED5D028  fsubs f22, f21, f26
	ctx.f[22].f64 = (((ctx.f[21].f64 - ctx.f[26].f64) as f32) as f64);
	// 832C7700: D061FF30  stfs f3, -0xd0(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), tmp.u32 ) };
	// 832C7704: EF9AA82A  fadds f28, f26, f21
	ctx.f[28].f64 = ((ctx.f[26].f64 + ctx.f[21].f64) as f32) as f64;
	// 832C7708: C341FF20  lfs f26, -0xe0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-224 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 832C770C: ED52502A  fadds f10, f18, f10
	ctx.f[10].f64 = ((ctx.f[18].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7710: D021FF20  stfs f1, -0xe0(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), tmp.u32 ) };
	// 832C7714: ED2F4828  fsubs f9, f15, f9
	ctx.f[9].f64 = (((ctx.f[15].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C7718: C1E1FF24  lfs f15, -0xdc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-220 as u32) ) };
	ctx.f[15].f64 = (tmp.f32 as f64);
	// 832C771C: EE4E8028  fsubs f18, f14, f16
	ctx.f[18].f64 = (((ctx.f[14].f64 - ctx.f[16].f64) as f32) as f64);
	// 832C7720: D1A1FF24  stfs f13, -0xdc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), tmp.u32 ) };
	// 832C7724: EE1101F2  fmuls f16, f17, f7
	ctx.f[16].f64 = (((ctx.f[17].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C7728: EEA506B2  fmuls f21, f5, f26
	ctx.f[21].f64 = (((ctx.f[5].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C772C: EE3101B2  fmuls f17, f17, f6
	ctx.f[17].f64 = (((ctx.f[17].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C7730: EDCB03F2  fmuls f14, f11, f15
	ctx.f[14].f64 = (((ctx.f[11].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7734: EE7D03F2  fmuls f19, f29, f15
	ctx.f[19].f64 = (((ctx.f[29].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7738: EFF7F82A  fadds f31, f23, f31
	ctx.f[31].f64 = ((ctx.f[23].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C773C: EEFE06B2  fmuls f23, f30, f26
	ctx.f[23].f64 = (((ctx.f[30].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C7740: EF7BA02A  fadds f27, f27, f20
	ctx.f[27].f64 = ((ctx.f[27].f64 + ctx.f[20].f64) as f32) as f64;
	// 832C7744: EE8003F2  fmuls f20, f0, f15
	ctx.f[20].f64 = (((ctx.f[0].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7748: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 832C774C: EE1681B8  fmsubs f16, f22, f6, f16
	ctx.f[16].f64 = (((ctx.f[22].f64 * ctx.f[6].f64 - ctx.f[16].f64) as f32) as f64);
	// 832C7750: EEB8AA3A  fmadds f21, f24, f8, f21
	ctx.f[21].f64 = (((ctx.f[24].f64 * ctx.f[8].f64 + ctx.f[21].f64) as f32) as f64);
	// 832C7754: EC3C03F2  fmuls f1, f28, f15
	ctx.f[1].f64 = (((ctx.f[28].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7758: EED689FA  fmadds f22, f22, f7, f17
	ctx.f[22].f64 = (((ctx.f[22].f64 * ctx.f[7].f64 + ctx.f[17].f64) as f32) as f64);
	// 832C775C: EFBD70BA  fmadds f29, f29, f2, f14
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[2].f64 + ctx.f[14].f64) as f32) as f64);
	// 832C7760: ED6B98B8  fmsubs f11, f11, f2, f19
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[2].f64 - ctx.f[19].f64) as f32) as f64);
	// 832C7764: EF1806B2  fmuls f24, f24, f26
	ctx.f[24].f64 = (((ctx.f[24].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C7768: EDBF01B2  fmuls f13, f31, f6
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C776C: EFDE0232  fmuls f30, f30, f8
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C7770: EDD90232  fmuls f14, f25, f8
	ctx.f[14].f64 = (((ctx.f[25].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C7774: EE6A06B2  fmuls f19, f10, f26
	ctx.f[19].f64 = (((ctx.f[10].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C7778: EE3206B2  fmuls f17, f18, f26
	ctx.f[17].f64 = (((ctx.f[18].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C777C: EEECBA3A  fmadds f23, f12, f8, f23
	ctx.f[23].f64 = (((ctx.f[12].f64 * ctx.f[8].f64 + ctx.f[23].f64) as f32) as f64);
	// 832C7780: EF3906B2  fmuls f25, f25, f26
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C7784: ECC901B2  fmuls f6, f9, f6
	ctx.f[6].f64 = (((ctx.f[9].f64 * ctx.f[6].f64) as f32) as f64);
	// 832C7788: D0C1FF28  stfs f6, -0xd8(r1)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), tmp.u32 ) };
	// 832C778C: C0C1FF2C  lfs f6, -0xd4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-212 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C7790: EC6403F2  fmuls f3, f4, f15
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7794: EE86A0BA  fmadds f20, f6, f2, f20
	ctx.f[20].f64 = (((ctx.f[6].f64 * ctx.f[2].f64 + ctx.f[20].f64) as f32) as f64);
	// 832C7798: EC0603F8  fmsubs f0, f6, f15, f0
	ctx.f[0].f64 = (((ctx.f[6].f64 * ctx.f[15].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C779C: ECC5C238  fmsubs f6, f5, f8, f24
	ctx.f[6].f64 = (((ctx.f[5].f64 * ctx.f[8].f64 - ctx.f[24].f64) as f32) as f64);
	// 832C77A0: EC8408B8  fmsubs f4, f4, f2, f1
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[2].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C77A4: ED8CF6B8  fmsubs f12, f12, f26, f30
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[26].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C77A8: ECB29A3A  fmadds f5, f18, f8, f19
	ctx.f[5].f64 = (((ctx.f[18].f64 * ctx.f[8].f64 + ctx.f[19].f64) as f32) as f64);
	// 832C77AC: ED4A8A38  fmsubs f10, f10, f8, f17
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[8].f64 - ctx.f[17].f64) as f32) as f64);
	// 832C77B0: EC2969FA  fmadds f1, f9, f7, f13
	ctx.f[1].f64 = (((ctx.f[9].f64 * ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64);
	// 832C77B4: EFDB76B8  fmsubs f30, f27, f26, f14
	ctx.f[30].f64 = (((ctx.f[27].f64 * ctx.f[26].f64 - ctx.f[14].f64) as f32) as f64);
	// 832C77B8: ED1BCA3A  fmadds f8, f27, f8, f25
	ctx.f[8].f64 = (((ctx.f[27].f64 * ctx.f[8].f64 + ctx.f[25].f64) as f32) as f64);
	// 832C77BC: EDB5B82A  fadds f13, f21, f23
	ctx.f[13].f64 = ((ctx.f[21].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C77C0: ED36E828  fsubs f9, f22, f29
	ctx.f[9].f64 = (((ctx.f[22].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C77C4: C261FF3C  lfs f19, -0xc4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-196 as u32) ) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 832C77C8: EF705828  fsubs f27, f16, f11
	ctx.f[27].f64 = (((ctx.f[16].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C77CC: C1E1FF50  lfs f15, -0xb0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-176 as u32) ) };
	ctx.f[15].f64 = (tmp.f32 as f64);
	// 832C77D0: EC7C18BA  fmadds f3, f28, f2, f3
	ctx.f[3].f64 = (((ctx.f[28].f64 * ctx.f[2].f64 + ctx.f[3].f64) as f32) as f64);
	// 832C77D4: C381FF20  lfs f28, -0xe0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-224 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C77D8: EF5C0028  fsubs f26, f28, f0
	ctx.f[26].f64 = (((ctx.f[28].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C77DC: C041FF28  lfs f2, -0xd8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C77E0: EC00E02A  fadds f0, f0, f28
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[28].f64) as f32) as f64;
	// 832C77E4: D1E1FF50  stfs f15, -0xb0(r1)
	tmp.f32 = (ctx.f[15].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 832C77E8: EF86602A  fadds f28, f6, f12
	ctx.f[28].f64 = ((ctx.f[6].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C77EC: C1C1FF58  lfs f14, -0xa8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-168 as u32) ) };
	ctx.f[14].f64 = (tmp.f32 as f64);
	// 832C77F0: EF054028  fsubs f24, f5, f8
	ctx.f[24].f64 = (((ctx.f[5].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C77F4: ED8C3028  fsubs f12, f12, f6
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C77F8: ED08282A  fadds f8, f8, f5
	ctx.f[8].f64 = ((ctx.f[8].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C77FC: D101FF58  stfs f8, -0xa8(r1)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 832C7800: ECD7A828  fsubs f6, f23, f21
	ctx.f[6].f64 = (((ctx.f[23].f64 - ctx.f[21].f64) as f32) as f64);
	// 832C7804: C2E1FF34  lfs f23, -0xcc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-204 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 832C7808: ECAB802A  fadds f5, f11, f16
	ctx.f[5].f64 = ((ctx.f[11].f64 + ctx.f[16].f64) as f32) as f64;
	// 832C780C: C161FF30  lfs f11, -0xd0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-208 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C7810: EF2AF028  fsubs f25, f10, f30
	ctx.f[25].f64 = (((ctx.f[10].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C7814: EFBDB02A  fadds f29, f29, f22
	ctx.f[29].f64 = ((ctx.f[29].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C7818: ED5E502A  fadds f10, f30, f10
	ctx.f[10].f64 = ((ctx.f[30].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C781C: EED74828  fsubs f22, f23, f9
	ctx.f[22].f64 = (((ctx.f[23].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C7820: EFDB582A  fadds f30, f27, f11
	ctx.f[30].f64 = ((ctx.f[27].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C7824: ED29B82A  fadds f9, f9, f23
	ctx.f[9].f64 = ((ctx.f[9].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C7828: ED6BD828  fsubs f11, f11, f27
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[27].f64) as f32) as f64);
	// 832C782C: ECFF11F8  fmsubs f7, f31, f7, f2
	ctx.f[7].f64 = (((ctx.f[31].f64 * ctx.f[7].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C7830: C041FF24  lfs f2, -0xdc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-220 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C7834: EEA1202A  fadds f21, f1, f4
	ctx.f[21].f64 = ((ctx.f[1].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C7838: EC840828  fsubs f4, f4, f1
	ctx.f[4].f64 = (((ctx.f[4].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C783C: C021FF38  lfs f1, -0xc8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-200 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C7840: EFE2A028  fsubs f31, f2, f20
	ctx.f[31].f64 = (((ctx.f[2].f64 - ctx.f[20].f64) as f32) as f64);
	// 832C7844: EC54102A  fadds f2, f20, f2
	ctx.f[2].f64 = ((ctx.f[20].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C7848: EE8D082A  fadds f20, f13, f1
	ctx.f[20].f64 = ((ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C784C: EC216828  fsubs f1, f1, f13
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C7850: EE5D982A  fadds f18, f29, f19
	ctx.f[18].f64 = ((ctx.f[29].f64 + ctx.f[19].f64) as f32) as f64;
	// 832C7854: EDB3E828  fsubs f13, f19, f29
	ctx.f[13].f64 = (((ctx.f[19].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C7858: EF76F028  fsubs f27, f22, f30
	ctx.f[27].f64 = (((ctx.f[22].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C785C: EFA95828  fsubs f29, f9, f11
	ctx.f[29].f64 = (((ctx.f[9].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7860: EFDEB02A  fadds f30, f30, f22
	ctx.f[30].f64 = ((ctx.f[30].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C7864: C2C1FF44  lfs f22, -0xbc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-188 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 832C7868: ED6B482A  fadds f11, f11, f9
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C786C: C121FF48  lfs f9, -0xb8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-184 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C7870: EE27182A  fadds f17, f7, f3
	ctx.f[17].f64 = ((ctx.f[7].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C7874: ECE33828  fsubs f7, f3, f7
	ctx.f[7].f64 = (((ctx.f[3].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7878: C061FF40  lfs f3, -0xc0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-192 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C787C: EE05B02A  fadds f16, f5, f22
	ctx.f[16].f64 = ((ctx.f[5].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C7880: EEFC182A  fadds f23, f28, f3
	ctx.f[23].f64 = ((ctx.f[28].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C7884: EC63E028  fsubs f3, f3, f28
	ctx.f[3].f64 = (((ctx.f[3].f64 - ctx.f[28].f64) as f32) as f64);
	// 832C7888: ECB62828  fsubs f5, f22, f5
	ctx.f[5].f64 = (((ctx.f[22].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C788C: C2C1FF4C  lfs f22, -0xb4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-180 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 832C7890: EF7B03F2  fmuls f27, f27, f15
	ctx.f[27].f64 = (((ctx.f[27].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7894: EFBD03F2  fmuls f29, f29, f15
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C7898: EFDE03F2  fmuls f30, f30, f15
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C789C: ED6B03F2  fmuls f11, f11, f15
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[15].f64) as f32) as f64);
	// 832C78A0: EDFF8828  fsubs f15, f31, f17
	ctx.f[15].f64 = (((ctx.f[31].f64 - ctx.f[17].f64) as f32) as f64);
	// 832C78A4: EF893028  fsubs f28, f9, f6
	ctx.f[28].f64 = (((ctx.f[9].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C78A8: EFF1F82A  fadds f31, f17, f31
	ctx.f[31].f64 = ((ctx.f[17].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C78AC: EE32A02A  fadds f17, f18, f20
	ctx.f[17].f64 = ((ctx.f[18].f64 + ctx.f[20].f64) as f32) as f64;
	// 832C78B0: D2230004  stfs f17, 4(r3)
	tmp.f32 = (ctx.f[17].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C78B4: ECC6482A  fadds f6, f6, f9
	ctx.f[6].f64 = ((ctx.f[6].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C78B8: C121FF54  lfs f9, -0xac(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-172 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C78BC: EE6CB02A  fadds f19, f12, f22
	ctx.f[19].f64 = ((ctx.f[12].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C78C0: D141FF54  stfs f10, -0xac(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 832C78C4: EE949028  fsubs f20, f20, f18
	ctx.f[20].f64 = (((ctx.f[20].f64 - ctx.f[18].f64) as f32) as f64);
	// 832C78C8: D283000C  stfs f20, 0xc(r3)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C78CC: ED966028  fsubs f12, f22, f12
	ctx.f[12].f64 = (((ctx.f[22].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C78D0: EE90B82A  fadds f20, f16, f23
	ctx.f[20].f64 = ((ctx.f[16].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C78D4: D2830000  stfs f20, 0(r3)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C78D8: EED9482A  fadds f22, f25, f9
	ctx.f[22].f64 = ((ctx.f[25].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C78DC: ED1AA828  fsubs f8, f26, f21
	ctx.f[8].f64 = (((ctx.f[26].f64 - ctx.f[21].f64) as f32) as f64);
	// 832C78E0: EEF78028  fsubs f23, f23, f16
	ctx.f[23].f64 = (((ctx.f[23].f64 - ctx.f[16].f64) as f32) as f64);
	// 832C78E4: D2E30008  stfs f23, 8(r3)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C78E8: ED29C828  fsubs f9, f9, f25
	ctx.f[9].f64 = (((ctx.f[9].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C78EC: ED58702A  fadds f10, f24, f14
	ctx.f[10].f64 = ((ctx.f[24].f64 + ctx.f[14].f64) as f32) as f64;
	// 832C78F0: EF55D02A  fadds f26, f21, f26
	ctx.f[26].f64 = ((ctx.f[21].f64 + ctx.f[26].f64) as f32) as f64;
	// 832C78F4: EF2EC028  fsubs f25, f14, f24
	ctx.f[25].f64 = (((ctx.f[14].f64 - ctx.f[24].f64) as f32) as f64);
	// 832C78F8: EF04102A  fadds f24, f4, f2
	ctx.f[24].f64 = ((ctx.f[4].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C78FC: EEA03828  fsubs f21, f0, f7
	ctx.f[21].f64 = (((ctx.f[0].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7900: EEE36828  fsubs f23, f3, f13
	ctx.f[23].f64 = (((ctx.f[3].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C7904: D2E30010  stfs f23, 0x10(r3)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 832C7908: EEE5082A  fadds f23, f5, f1
	ctx.f[23].f64 = ((ctx.f[5].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C790C: EC212828  fsubs f1, f1, f5
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7910: D023001C  stfs f1, 0x1c(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 832C7914: ECBE982A  fadds f5, f30, f19
	ctx.f[5].f64 = ((ctx.f[30].f64 + ctx.f[19].f64) as f32) as f64;
	// 832C7918: D0A30024  stfs f5, 0x24(r3)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 832C791C: ECACE828  fsubs f5, f12, f29
	ctx.f[5].f64 = (((ctx.f[12].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C7920: D0A3003C  stfs f5, 0x3c(r3)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 832C7924: ECA22028  fsubs f5, f2, f4
	ctx.f[5].f64 = (((ctx.f[2].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C7928: D2E30014  stfs f23, 0x14(r3)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832C792C: EC87002A  fadds f4, f7, f0
	ctx.f[4].f64 = ((ctx.f[7].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C7930: C001FF58  lfs f0, -0xa8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-168 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7934: EC6D182A  fadds f3, f13, f3
	ctx.f[3].f64 = ((ctx.f[13].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C7938: D0630018  stfs f3, 0x18(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 832C793C: EDBBE02A  fadds f13, f27, f28
	ctx.f[13].f64 = ((ctx.f[27].f64 + ctx.f[28].f64) as f32) as f64;
	// 832C7940: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 832C7944: EDA65828  fsubs f13, f6, f11
	ctx.f[13].f64 = (((ctx.f[6].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7948: D1A30030  stfs f13, 0x30(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 832C794C: ED6B302A  fadds f11, f11, f6
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7950: D1630038  stfs f11, 0x38(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 832C7954: EC33F028  fsubs f1, f19, f30
	ctx.f[1].f64 = (((ctx.f[19].f64 - ctx.f[30].f64) as f32) as f64);
	// 832C7958: D023002C  stfs f1, 0x2c(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 832C795C: ECDD602A  fadds f6, f29, f12
	ctx.f[6].f64 = ((ctx.f[29].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C7960: D0C30034  stfs f6, 0x34(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 832C7964: EC7CD828  fsubs f3, f28, f27
	ctx.f[3].f64 = (((ctx.f[28].f64 - ctx.f[27].f64) as f32) as f64);
	// 832C7968: D0630028  stfs f3, 0x28(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 832C796C: EC28502A  fadds f1, f8, f10
	ctx.f[1].f64 = ((ctx.f[8].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7970: D0230044  stfs f1, 0x44(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 832C7974: ED8A4028  fsubs f12, f10, f8
	ctx.f[12].f64 = (((ctx.f[10].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C7978: C021FF5C  lfs f1, -0xa4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C797C: EC6FB02A  fadds f3, f15, f22
	ctx.f[3].f64 = ((ctx.f[15].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C7980: D0630040  stfs f3, 0x40(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 832C7984: ED18A828  fsubs f8, f24, f21
	ctx.f[8].f64 = (((ctx.f[24].f64 - ctx.f[21].f64) as f32) as f64);
	// 832C7988: D183004C  stfs f12, 0x4c(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 832C798C: ECD5C02A  fadds f6, f21, f24
	ctx.f[6].f64 = ((ctx.f[21].f64 + ctx.f[24].f64) as f32) as f64;
	// 832C7990: C181FF60  lfs f12, -0xa0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7994: EC652028  fsubs f3, f5, f4
	ctx.f[3].f64 = (((ctx.f[5].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C7998: EC44282A  fadds f2, f4, f5
	ctx.f[2].f64 = ((ctx.f[4].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C799C: ED49D028  fsubs f10, f9, f26
	ctx.f[10].f64 = (((ctx.f[9].f64 - ctx.f[26].f64) as f32) as f64);
	// 832C79A0: D1430050  stfs f10, 0x50(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 832C79A4: ED3A482A  fadds f9, f26, f9
	ctx.f[9].f64 = ((ctx.f[26].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C79A8: D1230058  stfs f9, 0x58(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 832C79AC: ED7FC82A  fadds f11, f31, f25
	ctx.f[11].f64 = ((ctx.f[31].f64 + ctx.f[25].f64) as f32) as f64;
	// 832C79B0: C121FF50  lfs f9, -0xb0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-176 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C79B4: D1630054  stfs f11, 0x54(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 832C79B8: EDB67828  fsubs f13, f22, f15
	ctx.f[13].f64 = (((ctx.f[22].f64 - ctx.f[15].f64) as f32) as f64);
	// 832C79BC: C161FF54  lfs f11, -0xac(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-172 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C79C0: EC99F828  fsubs f4, f25, f31
	ctx.f[4].f64 = (((ctx.f[25].f64 - ctx.f[31].f64) as f32) as f64);
	// 832C79C4: D1A30048  stfs f13, 0x48(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 832C79C8: EDA10028  fsubs f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C79CC: ED080272  fmuls f8, f8, f9
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C79D0: D083005C  stfs f4, 0x5c(r3)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 832C79D4: ED4B602A  fadds f10, f11, f12
	ctx.f[10].f64 = ((ctx.f[11].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C79D8: ECE60272  fmuls f7, f6, f9
	ctx.f[7].f64 = (((ctx.f[6].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C79DC: ECC0082A  fadds f6, f0, f1
	ctx.f[6].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C79E0: ECAC5828  fsubs f5, f12, f11
	ctx.f[5].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C79E4: EC630272  fmuls f3, f3, f9
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C79E8: EC420272  fmuls f2, f2, f9
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C79EC: EC28682A  fadds f1, f8, f13
	ctx.f[1].f64 = ((ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C79F0: D0230060  stfs f1, 0x60(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 832C79F4: EDAD4028  fsubs f13, f13, f8
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C79F8: D1A30068  stfs f13, 0x68(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 832C79FC: EC07502A  fadds f0, f7, f10
	ctx.f[0].f64 = ((ctx.f[7].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7A00: D0030064  stfs f0, 0x64(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 832C7A04: ED8A3828  fsubs f12, f10, f7
	ctx.f[12].f64 = (((ctx.f[10].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7A08: D183006C  stfs f12, 0x6c(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 832C7A0C: ED63282A  fadds f11, f3, f5
	ctx.f[11].f64 = ((ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C7A10: D1630074  stfs f11, 0x74(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 832C7A14: ED461028  fsubs f10, f6, f2
	ctx.f[10].f64 = (((ctx.f[6].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C7A18: D1430070  stfs f10, 0x70(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 832C7A1C: ED22302A  fadds f9, f2, f6
	ctx.f[9].f64 = ((ctx.f[2].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7A20: D1230078  stfs f9, 0x78(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 832C7A24: ED051828  fsubs f8, f5, f3
	ctx.f[8].f64 = (((ctx.f[5].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C7A28: D103007C  stfs f8, 0x7c(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 832C7A2C: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 832C7A30: 4B9E62BD  bl 0x82cadcec
	ctx.lr = 0x832C7A34;
	sub_82CADCEC(ctx, base);
	// 832C7A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C7A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832C7A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C7A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C7A40 size=392
    let mut pc: u32 = 0x832C7A40;
    'dispatch: loop {
        match pc {
            0x832C7A40 => {
    //   block [0x832C7A40..0x832C7BC8)
	// 832C7A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C7A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832C7A48: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 832C7A4C: 4B9E627D  bl 0x82cadcc8
	ctx.lr = 0x832C7A50;
	sub_82CADCA0(ctx, base);
	// 832C7A50: C0030028  lfs f0, 0x28(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7A54: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7A58: C123003C  lfs f9, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C7A5C: ED4C0028  fsubs f10, f12, f0
	ctx.f[10].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C7A60: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7A64: ED0C002A  fadds f8, f12, f0
	ctx.f[8].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C7A68: C1630038  lfs f11, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C7A6C: C0E3000C  lfs f7, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C7A70: C0C30018  lfs f6, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C7A74: ECA76828  fsubs f5, f7, f13
	ctx.f[5].f64 = (((ctx.f[7].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C7A78: C083001C  lfs f4, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C7A7C: EC665828  fsubs f3, f6, f11
	ctx.f[3].f64 = (((ctx.f[6].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7A80: EC444828  fsubs f2, f4, f9
	ctx.f[2].f64 = (((ctx.f[4].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C7A84: C1830000  lfs f12, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7A88: C0230020  lfs f1, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C7A8C: ED66582A  fadds f11, f6, f11
	ctx.f[11].f64 = ((ctx.f[6].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C7A90: ECCC082A  fadds f6, f12, f1
	ctx.f[6].f64 = ((ctx.f[12].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C7A94: C3E30030  lfs f31, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C7A98: EC2C0828  fsubs f1, f12, f1
	ctx.f[1].f64 = (((ctx.f[12].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C7A9C: C1830010  lfs f12, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7AA0: EF8CF82A  fadds f28, f12, f31
	ctx.f[28].f64 = ((ctx.f[12].f64 + ctx.f[31].f64) as f32) as f64;
	// 832C7AA4: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7AA8: EDA7682A  fadds f13, f7, f13
	ctx.f[13].f64 = ((ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C7AAC: C0E30004  lfs f7, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C7AB0: EFC7002A  fadds f30, f7, f0
	ctx.f[30].f64 = ((ctx.f[7].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C7AB4: C3A30034  lfs f29, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 832C7AB8: ECE70028  fsubs f7, f7, f0
	ctx.f[7].f64 = (((ctx.f[7].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C7ABC: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7AC0: EF60E82A  fadds f27, f0, f29
	ctx.f[27].f64 = ((ctx.f[0].f64 + ctx.f[29].f64) as f32) as f64;
	// 832C7AC4: C3440004  lfs f26, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 832C7AC8: EF23282A  fadds f25, f3, f5
	ctx.f[25].f64 = ((ctx.f[3].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C7ACC: EF0A1028  fsubs f24, f10, f2
	ctx.f[24].f64 = (((ctx.f[10].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C7AD0: ED42502A  fadds f10, f2, f10
	ctx.f[10].f64 = ((ctx.f[2].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7AD4: ECA51828  fsubs f5, f5, f3
	ctx.f[5].f64 = (((ctx.f[5].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C7AD8: EC40E828  fsubs f2, f0, f29
	ctx.f[2].f64 = (((ctx.f[0].f64 - ctx.f[29].f64) as f32) as f64);
	// 832C7ADC: EC6CF828  fsubs f3, f12, f31
	ctx.f[3].f64 = (((ctx.f[12].f64 - ctx.f[31].f64) as f32) as f64);
	// 832C7AE0: EC04482A  fadds f0, f4, f9
	ctx.f[0].f64 = ((ctx.f[4].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7AE4: ED3C302A  fadds f9, f28, f6
	ctx.f[9].f64 = ((ctx.f[28].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7AE8: ECC6E028  fsubs f6, f6, f28
	ctx.f[6].f64 = (((ctx.f[6].f64 - ctx.f[28].f64) as f32) as f64);
	// 832C7AEC: ED8B402A  fadds f12, f11, f8
	ctx.f[12].f64 = ((ctx.f[11].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C7AF0: EC9BF02A  fadds f4, f27, f30
	ctx.f[4].f64 = ((ctx.f[27].f64 + ctx.f[30].f64) as f32) as f64;
	// 832C7AF4: EFF8C828  fsubs f31, f24, f25
	ctx.f[31].f64 = (((ctx.f[24].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C7AF8: EFB9C02A  fadds f29, f25, f24
	ctx.f[29].f64 = ((ctx.f[25].f64 + ctx.f[24].f64) as f32) as f64;
	// 832C7AFC: EF85502A  fadds f28, f5, f10
	ctx.f[28].f64 = ((ctx.f[5].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7B00: ECAA2828  fsubs f5, f10, f5
	ctx.f[5].f64 = (((ctx.f[10].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7B04: ED411028  fsubs f10, f1, f2
	ctx.f[10].f64 = (((ctx.f[1].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C7B08: EC42082A  fadds f2, f2, f1
	ctx.f[2].f64 = ((ctx.f[2].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C7B0C: EF23382A  fadds f25, f3, f7
	ctx.f[25].f64 = ((ctx.f[3].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C7B10: EC271828  fsubs f1, f7, f3
	ctx.f[1].f64 = (((ctx.f[7].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C7B14: ECE0682A  fadds f7, f0, f13
	ctx.f[7].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C7B18: EC6D0028  fsubs f3, f13, f0
	ctx.f[3].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C7B1C: EC1F06B2  fmuls f0, f31, f26
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C7B20: EDBD06B2  fmuls f13, f29, f26
	ctx.f[13].f64 = (((ctx.f[29].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C7B24: EFFC06B2  fmuls f31, f28, f26
	ctx.f[31].f64 = (((ctx.f[28].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C7B28: ECA506B2  fmuls f5, f5, f26
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[26].f64) as f32) as f64);
	// 832C7B2C: EFAC482A  fadds f29, f12, f9
	ctx.f[29].f64 = ((ctx.f[12].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7B30: D3A30000  stfs f29, 0(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C7B34: ED896028  fsubs f12, f9, f12
	ctx.f[12].f64 = (((ctx.f[9].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C7B38: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C7B3C: ED085828  fsubs f8, f8, f11
	ctx.f[8].f64 = (((ctx.f[8].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7B40: ED3ED828  fsubs f9, f30, f27
	ctx.f[9].f64 = (((ctx.f[30].f64 - ctx.f[27].f64) as f32) as f64);
	// 832C7B44: ED87202A  fadds f12, f7, f4
	ctx.f[12].f64 = ((ctx.f[7].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C7B48: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C7B4C: ED643828  fsubs f11, f4, f7
	ctx.f[11].f64 = (((ctx.f[4].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7B50: ECE0502A  fadds f7, f0, f10
	ctx.f[7].f64 = ((ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64;
	// 832C7B54: D0E30020  stfs f7, 0x20(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 832C7B58: EC0A0028  fsubs f0, f10, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C7B5C: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 832C7B60: EC8DC82A  fadds f4, f13, f25
	ctx.f[4].f64 = ((ctx.f[13].f64 + ctx.f[25].f64) as f32) as f64;
	// 832C7B64: D0830024  stfs f4, 0x24(r3)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 832C7B68: ED45082A  fadds f10, f5, f1
	ctx.f[10].f64 = ((ctx.f[5].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C7B6C: D1430034  stfs f10, 0x34(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 832C7B70: EDB96828  fsubs f13, f25, f13
	ctx.f[13].f64 = (((ctx.f[25].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C7B74: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 832C7B78: ED82F828  fsubs f12, f2, f31
	ctx.f[12].f64 = (((ctx.f[2].f64 - ctx.f[31].f64) as f32) as f64);
	// 832C7B7C: D1830030  stfs f12, 0x30(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 832C7B80: ECFF102A  fadds f7, f31, f2
	ctx.f[7].f64 = ((ctx.f[31].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C7B84: D0E30038  stfs f7, 0x38(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 832C7B88: ECA12828  fsubs f5, f1, f5
	ctx.f[5].f64 = (((ctx.f[1].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7B8C: D0A3003C  stfs f5, 0x3c(r3)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 832C7B90: EC861828  fsubs f4, f6, f3
	ctx.f[4].f64 = (((ctx.f[6].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C7B94: EC48482A  fadds f2, f8, f9
	ctx.f[2].f64 = ((ctx.f[8].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7B98: D163000C  stfs f11, 0xc(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C7B9C: EC23302A  fadds f1, f3, f6
	ctx.f[1].f64 = ((ctx.f[3].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7BA0: D0830010  stfs f4, 0x10(r3)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 832C7BA4: EC094028  fsubs f0, f9, f8
	ctx.f[0].f64 = (((ctx.f[9].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C7BA8: D0430014  stfs f2, 0x14(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832C7BAC: D0230018  stfs f1, 0x18(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 832C7BB0: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 832C7BB4: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 832C7BB8: 4B9E615D  bl 0x82cadd14
	ctx.lr = 0x832C7BBC;
	sub_82CADCEC(ctx, base);
	// 832C7BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C7BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832C7BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C7BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C7BC8 size=464
    let mut pc: u32 = 0x832C7BC8;
    'dispatch: loop {
        match pc {
            0x832C7BC8 => {
    //   block [0x832C7BC8..0x832C7D98)
	// 832C7BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C7BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832C7BD0: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 832C7BD4: 4B9E60E5  bl 0x82cadcb8
	ctx.lr = 0x832C7BD8;
	sub_82CADCA0(ctx, base);
	// 832C7BD8: C1A30014  lfs f13, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7BDC: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7BE0: C183000C  lfs f12, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7BE4: C1430034  lfs f10, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C7BE8: C0E30028  lfs f7, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C7BEC: ED005028  fsubs f8, f0, f10
	ctx.f[8].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C7BF0: C1230030  lfs f9, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C7BF4: EC8C382A  fadds f4, f12, f7
	ctx.f[4].f64 = ((ctx.f[12].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C7BF8: ECCD482A  fadds f6, f13, f9
	ctx.f[6].f64 = ((ctx.f[13].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7BFC: C1630018  lfs f11, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C7C00: C0A3003C  lfs f5, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C7C04: EC0A002A  fadds f0, f10, f0
	ctx.f[0].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C7C08: C023001C  lfs f1, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C7C0C: EC4B2828  fsubs f2, f11, f5
	ctx.f[2].f64 = (((ctx.f[11].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7C10: C0630038  lfs f3, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C7C14: ED2D4828  fsubs f9, f13, f9
	ctx.f[9].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C7C18: ED41182A  fadds f10, f1, f3
	ctx.f[10].f64 = ((ctx.f[1].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C7C1C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7C20: ED8C3828  fsubs f12, f12, f7
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7C24: C0E3002C  lfs f7, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C7C28: EC611828  fsubs f3, f1, f3
	ctx.f[3].f64 = (((ctx.f[1].f64 - ctx.f[3].f64) as f32) as f64);
	// 832C7C2C: C3C40014  lfs f30, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 832C7C30: EC2D3828  fsubs f1, f13, f7
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7C34: C3E40010  lfs f31, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 832C7C38: EDA7682A  fadds f13, f7, f13
	ctx.f[13].f64 = ((ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64;
	// 832C7C3C: C3A40004  lfs f29, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 832C7C40: EF4407B2  fmuls f26, f4, f30
	ctx.f[26].f64 = (((ctx.f[4].f64 * ctx.f[30].f64) as f32) as f64);
	// 832C7C44: C3830000  lfs f28, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 832C7C48: ECE83028  fsubs f7, f8, f6
	ctx.f[7].f64 = (((ctx.f[8].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C7C4C: C3630004  lfs f27, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 832C7C50: EC8407F2  fmuls f4, f4, f31
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C7C54: C3230020  lfs f25, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 832C7C58: ECC6402A  fadds f6, f6, f8
	ctx.f[6].f64 = ((ctx.f[6].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C7C5C: C1030024  lfs f8, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C7C60: EF0207F2  fmuls f24, f2, f31
	ctx.f[24].f64 = (((ctx.f[2].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C7C64: EEEA07F2  fmuls f23, f10, f31
	ctx.f[23].f64 = (((ctx.f[10].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C7C68: EEDC4028  fsubs f22, f28, f8
	ctx.f[22].f64 = (((ctx.f[28].f64 - ctx.f[8].f64) as f32) as f64);
	// 832C7C6C: EEBBC82A  fadds f21, f27, f25
	ctx.f[21].f64 = ((ctx.f[27].f64 + ctx.f[25].f64) as f32) as f64;
	// 832C7C70: ED65582A  fadds f11, f5, f11
	ctx.f[11].f64 = ((ctx.f[5].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C7C74: ECA04828  fsubs f5, f0, f9
	ctx.f[5].f64 = (((ctx.f[0].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C7C78: EF41D7F8  fmsubs f26, f1, f31, f26
	ctx.f[26].f64 = (((ctx.f[1].f64 * ctx.f[31].f64 - ctx.f[26].f64) as f32) as f64);
	// 832C7C7C: ECE70772  fmuls f7, f7, f29
	ctx.f[7].f64 = (((ctx.f[7].f64 * ctx.f[29].f64) as f32) as f64);
	// 832C7C80: EC8127BA  fmadds f4, f1, f30, f4
	ctx.f[4].f64 = (((ctx.f[1].f64 * ctx.f[30].f64 + ctx.f[4].f64) as f32) as f64);
	// 832C7C84: ECC60772  fmuls f6, f6, f29
	ctx.f[6].f64 = (((ctx.f[6].f64 * ctx.f[29].f64) as f32) as f64);
	// 832C7C88: EC2AC7BA  fmadds f1, f10, f30, f24
	ctx.f[1].f64 = (((ctx.f[10].f64 * ctx.f[30].f64 + ctx.f[24].f64) as f32) as f64);
	// 832C7C8C: ED42BFB8  fmsubs f10, f2, f30, f23
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[30].f64 - ctx.f[23].f64) as f32) as f64);
	// 832C7C90: EC4C07F2  fmuls f2, f12, f31
	ctx.f[2].f64 = (((ctx.f[12].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C7C94: ED29002A  fadds f9, f9, f0
	ctx.f[9].f64 = ((ctx.f[9].f64 + ctx.f[0].f64) as f32) as f64;
	// 832C7C98: EC0D07F2  fmuls f0, f13, f31
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C7C9C: EF0307B2  fmuls f24, f3, f30
	ctx.f[24].f64 = (((ctx.f[3].f64 * ctx.f[30].f64) as f32) as f64);
	// 832C7CA0: ED08E02A  fadds f8, f8, f28
	ctx.f[8].f64 = ((ctx.f[8].f64 + ctx.f[28].f64) as f32) as f64;
	// 832C7CA4: EEE7B02A  fadds f23, f7, f22
	ctx.f[23].f64 = ((ctx.f[7].f64 + ctx.f[22].f64) as f32) as f64;
	// 832C7CA8: ECF63828  fsubs f7, f22, f7
	ctx.f[7].f64 = (((ctx.f[22].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7CAC: EE86A82A  fadds f20, f6, f21
	ctx.f[20].f64 = ((ctx.f[6].f64 + ctx.f[21].f64) as f32) as f64;
	// 832C7CB0: EEC1202A  fadds f22, f1, f4
	ctx.f[22].f64 = ((ctx.f[1].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C7CB4: ECD53028  fsubs f6, f21, f6
	ctx.f[6].f64 = (((ctx.f[21].f64 - ctx.f[6].f64) as f32) as f64);
	// 832C7CB8: EEAAD02A  fadds f21, f10, f26
	ctx.f[21].f64 = ((ctx.f[10].f64 + ctx.f[26].f64) as f32) as f64;
	// 832C7CBC: EC840828  fsubs f4, f4, f1
	ctx.f[4].f64 = (((ctx.f[4].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C7CC0: EC3A5028  fsubs f1, f26, f10
	ctx.f[1].f64 = (((ctx.f[26].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C7CC4: EC4D17B8  fmsubs f2, f13, f30, f2
	ctx.f[2].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 - ctx.f[2].f64) as f32) as f64);
	// 832C7CC8: ED4307F2  fmuls f10, f3, f31
	ctx.f[10].f64 = (((ctx.f[3].f64 * ctx.f[31].f64) as f32) as f64);
	// 832C7CCC: EC7BC828  fsubs f3, f27, f25
	ctx.f[3].f64 = (((ctx.f[27].f64 - ctx.f[25].f64) as f32) as f64);
	// 832C7CD0: ECA50772  fmuls f5, f5, f29
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[29].f64) as f32) as f64);
	// 832C7CD4: ED290772  fmuls f9, f9, f29
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[29].f64) as f32) as f64);
	// 832C7CD8: EDB6A02A  fadds f13, f22, f20
	ctx.f[13].f64 = ((ctx.f[22].f64 + ctx.f[20].f64) as f32) as f64;
	// 832C7CDC: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C7CE0: EDB5B82A  fadds f13, f21, f23
	ctx.f[13].f64 = ((ctx.f[21].f64 + ctx.f[23].f64) as f32) as f64;
	// 832C7CE4: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C7CE8: EDB7A828  fsubs f13, f23, f21
	ctx.f[13].f64 = (((ctx.f[23].f64 - ctx.f[21].f64) as f32) as f64);
	// 832C7CEC: D1A30008  stfs f13, 8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C7CF0: EDB4B028  fsubs f13, f20, f22
	ctx.f[13].f64 = (((ctx.f[20].f64 - ctx.f[22].f64) as f32) as f64);
	// 832C7CF4: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832C7CF8: EDA1302A  fadds f13, f1, f6
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[6].f64) as f32) as f64;
	// 832C7CFC: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832C7D00: EDA72028  fsubs f13, f7, f4
	ctx.f[13].f64 = (((ctx.f[7].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C7D04: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 832C7D08: ECE4382A  fadds f7, f4, f7
	ctx.f[7].f64 = ((ctx.f[4].f64 + ctx.f[7].f64) as f32) as f64;
	// 832C7D0C: D0E30018  stfs f7, 0x18(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 832C7D10: EC8C07BA  fmadds f4, f12, f30, f0
	ctx.f[4].f64 = (((ctx.f[12].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 832C7D14: EC0BC7F8  fmsubs f0, f11, f31, f24
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[31].f64 - ctx.f[24].f64) as f32) as f64);
	// 832C7D18: EDAB57BA  fmadds f13, f11, f30, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[30].f64 + ctx.f[10].f64) as f32) as f64);
	// 832C7D1C: ED884828  fsubs f12, f8, f9
	ctx.f[12].f64 = (((ctx.f[8].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C7D20: ED65182A  fadds f11, f5, f3
	ctx.f[11].f64 = ((ctx.f[5].f64 + ctx.f[3].f64) as f32) as f64;
	// 832C7D24: ED29402A  fadds f9, f9, f8
	ctx.f[9].f64 = ((ctx.f[9].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C7D28: ED420028  fsubs f10, f2, f0
	ctx.f[10].f64 = (((ctx.f[2].f64 - ctx.f[0].f64) as f32) as f64);
	// 832C7D2C: ECE46828  fsubs f7, f4, f13
	ctx.f[7].f64 = (((ctx.f[4].f64 - ctx.f[13].f64) as f32) as f64);
	// 832C7D30: ED032828  fsubs f8, f3, f5
	ctx.f[8].f64 = (((ctx.f[3].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7D34: EC8D202A  fadds f4, f13, f4
	ctx.f[4].f64 = ((ctx.f[13].f64 + ctx.f[4].f64) as f32) as f64;
	// 832C7D38: ECA0102A  fadds f5, f0, f2
	ctx.f[5].f64 = ((ctx.f[0].f64 + ctx.f[2].f64) as f32) as f64;
	// 832C7D3C: EC660828  fsubs f3, f6, f1
	ctx.f[3].f64 = (((ctx.f[6].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C7D40: D063001C  stfs f3, 0x1c(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 832C7D44: EC4A602A  fadds f2, f10, f12
	ctx.f[2].f64 = ((ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C7D48: D0430020  stfs f2, 0x20(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 832C7D4C: EC0C5028  fsubs f0, f12, f10
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 832C7D50: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 832C7D54: EC27582A  fadds f1, f7, f11
	ctx.f[1].f64 = ((ctx.f[7].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C7D58: D0230024  stfs f1, 0x24(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 832C7D5C: EDAB3828  fsubs f13, f11, f7
	ctx.f[13].f64 = (((ctx.f[11].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7D60: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 832C7D64: ED692028  fsubs f11, f9, f4
	ctx.f[11].f64 = (((ctx.f[9].f64 - ctx.f[4].f64) as f32) as f64);
	// 832C7D68: D1630030  stfs f11, 0x30(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 832C7D6C: ED44482A  fadds f10, f4, f9
	ctx.f[10].f64 = ((ctx.f[4].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7D70: D1430038  stfs f10, 0x38(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 832C7D74: ED85402A  fadds f12, f5, f8
	ctx.f[12].f64 = ((ctx.f[5].f64 + ctx.f[8].f64) as f32) as f64;
	// 832C7D78: D1830034  stfs f12, 0x34(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 832C7D7C: ED282828  fsubs f9, f8, f5
	ctx.f[9].f64 = (((ctx.f[8].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7D80: D123003C  stfs f9, 0x3c(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 832C7D84: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 832C7D88: 4B9E5F7D  bl 0x82cadd04
	ctx.lr = 0x832C7D8C;
	sub_82CADCEC(ctx, base);
	// 832C7D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C7D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832C7D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C7D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832C7D98 size=512
    let mut pc: u32 = 0x832C7D98;
    'dispatch: loop {
        match pc {
            0x832C7D98 => {
    //   block [0x832C7D98..0x832C7DF8)
	// 832C7D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C7D9C: 4B9E1661  bl 0x82ca93fc
	ctx.lr = 0x832C7DA0;
	sub_82CA93D0(ctx, base);
	// 832C7DA0: 7C790E70  srawi r25, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 832C7DA4: 7D251BD6  divw r9, r5, r3
	ctx.r[9].s32 = ctx.r[5].s32 / ctx.r[3].s32;
	// 832C7DA8: 3979FFFF  addi r11, r25, -1
	ctx.r[11].s64 = ctx.r[25].s64 + -1;
	// 832C7DAC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832C7DB0: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 832C7DB4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 832C7DB8: 41980140  blt cr6, 0x832c7ef8
	if ctx.cr[6].lt {
	pc = 0x832C7EF8; continue 'dispatch;
	}
	// 832C7DBC: 3979FFFB  addi r11, r25, -5
	ctx.r[11].s64 = ctx.r[25].s64 + -5;
	// 832C7DC0: 3903FFFD  addi r8, r3, -3
	ctx.r[8].s64 = ctx.r[3].s64 + -3;
	// 832C7DC4: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C7DC8: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C7DCC: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 832C7DD0: 551B103A  slwi r27, r8, 2
	ctx.r[27].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 832C7DD4: 7CE900D0  neg r7, r9
	ctx.r[7].s64 = -ctx.r[9].s64;
	// 832C7DD8: 57FA103A  slwi r26, r31, 2
	ctx.r[26].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 832C7DDC: 7FAA3214  add r29, r10, r6
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 832C7DE0: 7D5B2214  add r10, r27, r4
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[4].u64;
	// 832C7DE4: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C7DE8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 832C7DEC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C7DF0: 3964000C  addi r11, r4, 0xc
	ctx.r[11].s64 = ctx.r[4].s64 + 12;
	// 832C7DF4: 3B7A0001  addi r27, r26, 1
	ctx.r[27].s64 = ctx.r[26].s64 + 1;
	pc = 0x832C7DF8; continue 'dispatch;
            }
            0x832C7DF8 => {
    //   block [0x832C7DF8..0x832C7EF8)
	// 832C7DF8: 7FA7EA14  add r29, r7, r29
	ctx.r[29].u64 = ctx.r[7].u64 + ctx.r[29].u64;
	// 832C7DFC: C00A0008  lfs f0, 8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7E00: 7FDE4214  add r30, r30, r8
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[8].u64;
	// 832C7E04: C1ABFFF8  lfs f13, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7E08: 7F9C4A14  add r28, r28, r9
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[9].u64;
	// 832C7E0C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832C7E10: 7F9C4A14  add r28, r28, r9
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[9].u64;
	// 832C7E14: C19D0000  lfs f12, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7E18: 7FA7EA14  add r29, r7, r29
	ctx.r[29].u64 = ctx.r[7].u64 + ctx.r[29].u64;
	// 832C7E1C: C17E0000  lfs f11, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C7E20: 7FDE4214  add r30, r30, r8
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[8].u64;
	// 832C7E24: ED4B602A  fadds f10, f11, f12
	ctx.f[10].f64 = ((ctx.f[11].f64 + ctx.f[12].f64) as f32) as f64;
	// 832C7E28: 7F9C4A14  add r28, r28, r9
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[9].u64;
	// 832C7E2C: ED2B6028  fsubs f9, f11, f12
	ctx.f[9].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C7E30: 7F9C4A14  add r28, r28, r9
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[9].u64;
	// 832C7E34: ED0002B2  fmuls f8, f0, f10
	ctx.f[8].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C7E38: ECE00272  fmuls f7, f0, f9
	ctx.f[7].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 832C7E3C: ECCD427A  fmadds f6, f13, f9, f8
	ctx.f[6].f64 = (((ctx.f[13].f64 * ctx.f[9].f64 + ctx.f[8].f64) as f32) as f64);
	// 832C7E40: D0CBFFF8  stfs f6, -8(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 832C7E44: ECAD3AB8  fmsubs f5, f13, f10, f7
	ctx.f[5].f64 = (((ctx.f[13].f64 * ctx.f[10].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7E48: D0AA0008  stfs f5, 8(r10)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832C7E4C: C08A0004  lfs f4, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 832C7E50: C06BFFFC  lfs f3, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 832C7E54: C05E0000  lfs f2, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 832C7E58: 7FDE4214  add r30, r30, r8
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[8].u64;
	// 832C7E5C: C03D0000  lfs f1, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832C7E60: EC02082A  fadds f0, f2, f1
	ctx.f[0].f64 = ((ctx.f[2].f64 + ctx.f[1].f64) as f32) as f64;
	// 832C7E64: EDA20828  fsubs f13, f2, f1
	ctx.f[13].f64 = (((ctx.f[2].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C7E68: 7FA7EA14  add r29, r7, r29
	ctx.r[29].u64 = ctx.r[7].u64 + ctx.r[29].u64;
	// 832C7E6C: ED840032  fmuls f12, f4, f0
	ctx.f[12].f64 = (((ctx.f[4].f64 * ctx.f[0].f64) as f32) as f64);
	// 832C7E70: ED640372  fmuls f11, f4, f13
	ctx.f[11].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C7E74: ED43637A  fmadds f10, f3, f13, f12
	ctx.f[10].f64 = (((ctx.f[3].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 832C7E78: D14BFFFC  stfs f10, -4(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C7E7C: ED235838  fmsubs f9, f3, f0, f11
	ctx.f[9].f64 = (((ctx.f[3].f64 * ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 832C7E80: D12A0004  stfs f9, 4(r10)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C7E84: C10B0000  lfs f8, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 832C7E88: C0EA0000  lfs f7, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 832C7E8C: C0DE0000  lfs f6, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 832C7E90: 7FDE4214  add r30, r30, r8
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[8].u64;
	// 832C7E94: C0BD0000  lfs f5, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 832C7E98: EC862828  fsubs f4, f6, f5
	ctx.f[4].f64 = (((ctx.f[6].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7E9C: EC66282A  fadds f3, f6, f5
	ctx.f[3].f64 = ((ctx.f[6].f64 + ctx.f[5].f64) as f32) as f64;
	// 832C7EA0: 7FA7EA14  add r29, r7, r29
	ctx.r[29].u64 = ctx.r[7].u64 + ctx.r[29].u64;
	// 832C7EA4: EC480132  fmuls f2, f8, f4
	ctx.f[2].f64 = (((ctx.f[8].f64 * ctx.f[4].f64) as f32) as f64);
	// 832C7EA8: EC270132  fmuls f1, f7, f4
	ctx.f[1].f64 = (((ctx.f[7].f64 * ctx.f[4].f64) as f32) as f64);
	// 832C7EAC: EC0710FA  fmadds f0, f7, f3, f2
	ctx.f[0].f64 = (((ctx.f[7].f64 * ctx.f[3].f64 + ctx.f[2].f64) as f32) as f64);
	// 832C7EB0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C7EB4: EDA808F8  fmsubs f13, f8, f3, f1
	ctx.f[13].f64 = (((ctx.f[8].f64 * ctx.f[3].f64 - ctx.f[1].f64) as f32) as f64);
	// 832C7EB8: D1AA0000  stfs f13, 0(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C7EBC: C18AFFFC  lfs f12, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7EC0: C16B0004  lfs f11, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C7EC4: C15E0000  lfs f10, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832C7EC8: C13D0000  lfs f9, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 832C7ECC: ED0A482A  fadds f8, f10, f9
	ctx.f[8].f64 = ((ctx.f[10].f64 + ctx.f[9].f64) as f32) as f64;
	// 832C7ED0: ECEA4828  fsubs f7, f10, f9
	ctx.f[7].f64 = (((ctx.f[10].f64 - ctx.f[9].f64) as f32) as f64);
	// 832C7ED4: ECCC0232  fmuls f6, f12, f8
	ctx.f[6].f64 = (((ctx.f[12].f64 * ctx.f[8].f64) as f32) as f64);
	// 832C7ED8: ECAC01F2  fmuls f5, f12, f7
	ctx.f[5].f64 = (((ctx.f[12].f64 * ctx.f[7].f64) as f32) as f64);
	// 832C7EDC: EC8B31FA  fmadds f4, f11, f7, f6
	ctx.f[4].f64 = (((ctx.f[11].f64 * ctx.f[7].f64 + ctx.f[6].f64) as f32) as f64);
	// 832C7EE0: D08B0004  stfs f4, 4(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832C7EE4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 832C7EE8: EC6B2A38  fmsubs f3, f11, f8, f5
	ctx.f[3].f64 = (((ctx.f[11].f64 * ctx.f[8].f64 - ctx.f[5].f64) as f32) as f64);
	// 832C7EEC: D06AFFFC  stfs f3, -4(r10)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832C7EF0: 394AFFF0  addi r10, r10, -0x10
	ctx.r[10].s64 = ctx.r[10].s64 + -16;
	// 832C7EF4: 4082FF04  bne 0x832c7df8
	if !ctx.cr[0].eq {
	pc = 0x832C7DF8; continue 'dispatch;
	}
	pc = 0x832C7EF8; continue 'dispatch;
            }
            0x832C7EF8 => {
    //   block [0x832C7EF8..0x832C7F38)
	// 832C7EF8: 7F1BC800  cmpw cr6, r27, r25
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[25].s32, &mut ctx.xer);
	// 832C7EFC: 40980084  bge cr6, 0x832c7f80
	if !ctx.cr[6].lt {
	pc = 0x832C7F80; continue 'dispatch;
	}
	// 832C7F00: 7D5B1850  subf r10, r27, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[27].s64;
	// 832C7F04: 7CBC2850  subf r5, r28, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[28].s64;
	// 832C7F08: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C7F0C: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C7F10: 7FE900D0  neg r31, r9
	ctx.r[31].s64 = -ctx.r[9].s64;
	// 832C7F14: 5788103A  slwi r8, r28, 2
	ctx.r[8].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C7F18: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C7F1C: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 832C7F20: 5523103A  slwi r3, r9, 2
	ctx.r[3].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 832C7F24: 7D672214  add r11, r7, r4
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 832C7F28: 7D083214  add r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 832C7F2C: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 832C7F30: 7D253214  add r9, r5, r6
	ctx.r[9].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 832C7F34: 7CFBC850  subf r7, r27, r25
	ctx.r[7].s64 = ctx.r[25].s64 - ctx.r[27].s64;
	pc = 0x832C7F38; continue 'dispatch;
            }
            0x832C7F38 => {
    //   block [0x832C7F38..0x832C7F80)
	// 832C7F38: 7D29FA14  add r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 832C7F3C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7F40: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 832C7F44: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7F48: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 832C7F4C: C1890000  lfs f12, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832C7F50: C1680000  lfs f11, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832C7F54: ED4B6028  fsubs f10, f11, f12
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 832C7F58: ED2C582A  fadds f9, f12, f11
	ctx.f[9].f64 = ((ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64;
	// 832C7F5C: ED0002B2  fmuls f8, f0, f10
	ctx.f[8].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C7F60: ECED02B2  fmuls f7, f13, f10
	ctx.f[7].f64 = (((ctx.f[13].f64 * ctx.f[10].f64) as f32) as f64);
	// 832C7F64: ECCD427A  fmadds f6, f13, f9, f8
	ctx.f[6].f64 = (((ctx.f[13].f64 * ctx.f[9].f64 + ctx.f[8].f64) as f32) as f64);
	// 832C7F68: D0CA0000  stfs f6, 0(r10)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C7F6C: ECA03A78  fmsubs f5, f0, f9, f7
	ctx.f[5].f64 = (((ctx.f[0].f64 * ctx.f[9].f64 - ctx.f[7].f64) as f32) as f64);
	// 832C7F70: D0AB0000  stfs f5, 0(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 832C7F74: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 832C7F78: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 832C7F7C: 4082FFBC  bne 0x832c7f38
	if !ctx.cr[0].eq {
	pc = 0x832C7F38; continue 'dispatch;
	}
	pc = 0x832C7F80; continue 'dispatch;
            }
            0x832C7F80 => {
    //   block [0x832C7F80..0x832C7F98)
	// 832C7F80: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C7F84: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832C7F88: 7DAB242E  lfsx f13, r11, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832C7F8C: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 832C7F90: 7D8B252E  stfsx f12, r11, r4
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 832C7F94: 4B9E14B8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C7F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C7F98 size=796
    let mut pc: u32 = 0x832C7F98;
    'dispatch: loop {
        match pc {
            0x832C7F98 => {
    //   block [0x832C7F98..0x832C7FAC)
	// 832C7F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C7F9C: 4B9E1449  bl 0x82ca93e4
	ctx.lr = 0x832C7FA0;
	sub_82CA93D0(ctx, base);
	// 832C7FA0: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 832C7FA4: 3961FE90  addi r11, r1, -0x170
	ctx.r[11].s64 = ctx.r[1].s64 + -368;
	// 832C7FA8: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	pc = 0x832C7FAC; continue 'dispatch;
            }
            0x832C7FAC => {
    //   block [0x832C7FAC..0x832C8040)
	// 832C7FAC: A10A0010  lhz r8, 0x10(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C7FB0: A12A0020  lhz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C7FB4: A0EA0030  lhz r7, 0x30(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C7FB8: 7D050734  extsh r5, r8
	ctx.r[5].s64 = ctx.r[8].s16 as i64;
	// 832C7FBC: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 832C7FC0: A10A0040  lhz r8, 0x40(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 832C7FC4: 7CFD0734  extsh r29, r7
	ctx.r[29].s64 = ctx.r[7].s16 as i64;
	// 832C7FC8: A0EA0050  lhz r7, 0x50(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 832C7FCC: 7CBF4B78  or r31, r5, r9
	ctx.r[31].u64 = ctx.r[5].u64 | ctx.r[9].u64;
	// 832C7FD0: A3CA0060  lhz r30, 0x60(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C7FD4: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 832C7FD8: A36A0070  lhz r27, 0x70(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(112 as u32) ) } as u64;
	// 832C7FDC: 7FFFEB78  or r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[29].u64;
	// 832C7FE0: 7CFC0734  extsh r28, r7
	ctx.r[28].s64 = ctx.r[7].s16 as i64;
	// 832C7FE4: 7FFF4378  or r31, r31, r8
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[8].u64;
	// 832C7FE8: 7FC70734  extsh r7, r30
	ctx.r[7].s64 = ctx.r[30].s16 as i64;
	// 832C7FEC: 7FFFE378  or r31, r31, r28
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[28].u64;
	// 832C7FF0: 7F7B0734  extsh r27, r27
	ctx.r[27].s64 = ctx.r[27].s16 as i64;
	// 832C7FF4: 7FFF3B78  or r31, r31, r7
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[7].u64;
	// 832C7FF8: 7FFFDB78  or r31, r31, r27
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[27].u64;
	// 832C7FFC: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 832C8000: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832C8004: 409A003C  bne cr6, 0x832c8040
	if !ctx.cr[6].eq {
	pc = 0x832C8040; continue 'dispatch;
	}
	// 832C8008: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C800C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 832C8010: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8014: 7D270734  extsh r7, r9
	ctx.r[7].s64 = ctx.r[9].s16 as i64;
	// 832C8018: 7CA741D6  mullw r5, r7, r8
	ctx.r[5].s32 = ((ctx.r[7].s32 as i64 * ctx.r[8].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 832C801C: 7CA95E70  srawi r9, r5, 0xb
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[5].s32 >> 11) as i64;
	// 832C8020: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832C8024: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 832C8028: 912B0040  stw r9, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[9].u32 ) };
	// 832C802C: 912B0060  stw r9, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 832C8030: 912B00A0  stw r9, 0xa0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 832C8034: 912B00C0  stw r9, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[9].u32 ) };
	// 832C8038: 912B00E0  stw r9, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[9].u32 ) };
	// 832C803C: 48000128  b 0x832c8164
	pc = 0x832C8164; continue 'dispatch;
            }
            0x832C8040 => {
    //   block [0x832C8040..0x832C8164)
	// 832C8040: A3EA0000  lhz r31, 0(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8044: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 832C8048: 83C60000  lwz r30, 0(r6)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C804C: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 832C8050: 83460040  lwz r26, 0x40(r6)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(64 as u32) ) } as u64;
	// 832C8054: 83260080  lwz r25, 0x80(r6)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C8058: 7FFFF1D6  mullw r31, r31, r30
	ctx.r[31].s32 = ((ctx.r[31].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[31].s64 = ctx.r[31].s32 as i64;
	// 832C805C: 82E600C0  lwz r23, 0xc0(r6)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(192 as u32) ) } as u64;
	// 832C8060: 82C60020  lwz r22, 0x20(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C8064: 82A60060  lwz r21, 0x60(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C8068: 828600A0  lwz r20, 0xa0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(160 as u32) ) } as u64;
	// 832C806C: 826600E0  lwz r19, 0xe0(r6)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C8070: 7D3A49D6  mullw r9, r26, r9
	ctx.r[9].s32 = ((ctx.r[26].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 832C8074: 7FD941D6  mullw r30, r25, r8
	ctx.r[30].s32 = ((ctx.r[25].s32 as i64 * ctx.r[8].s32 as i64) as i32);
	ctx.r[30].s64 = ctx.r[30].s32 as i64;
	// 832C8078: 7FFF5E70  srawi r31, r31, 0xb
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 11) as i64;
	// 832C807C: 7CF739D6  mullw r7, r23, r7
	ctx.r[7].s32 = ((ctx.r[23].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 832C8080: 7D285E70  srawi r8, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 832C8084: 7FDE5E70  srawi r30, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 832C8088: 7CE75E70  srawi r7, r7, 0xb
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 11) as i64;
	// 832C808C: 7CB629D6  mullw r5, r22, r5
	ctx.r[5].s32 = ((ctx.r[22].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 832C8090: 7D274050  subf r9, r7, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 832C8094: 7FB5E9D6  mullw r29, r21, r29
	ctx.r[29].s32 = ((ctx.r[21].s32 as i64 * ctx.r[29].s32 as i64) as i32);
	ctx.r[29].s64 = ctx.r[29].s32 as i64;
	// 832C8098: 1D290B50  mulli r9, r9, 0xb50
	ctx.r[9].s32 = ((ctx.r[9].s32 as i64 * 2896 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 832C809C: 7D395E70  srawi r25, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 832C80A0: 7F94E1D6  mullw r28, r20, r28
	ctx.r[28].s32 = ((ctx.r[20].s32 as i64 * ctx.r[28].s32 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 832C80A4: 7CA95E70  srawi r9, r5, 0xb
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[5].s32 >> 11) as i64;
	// 832C80A8: 7F73D9D6  mullw r27, r19, r27
	ctx.r[27].s32 = ((ctx.r[19].s32 as i64 * ctx.r[27].s32 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C80AC: 7FA55E70  srawi r5, r29, 0xb
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[29].s32 >> 11) as i64;
	// 832C80B0: 7F9D5E70  srawi r29, r28, 0xb
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[28].s32 >> 11) as i64;
	// 832C80B4: 7F7C5E70  srawi r28, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C80B8: 7F65E850  subf r27, r5, r29
	ctx.r[27].s64 = ctx.r[29].s64 - ctx.r[5].s64;
	// 832C80BC: 7F5C4850  subf r26, r28, r9
	ctx.r[26].s64 = ctx.r[9].s64 - ctx.r[28].s64;
	// 832C80C0: 7CBD2A14  add r5, r29, r5
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	// 832C80C4: 7FBADA14  add r29, r26, r27
	ctx.r[29].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 832C80C8: 7D3C4A14  add r9, r28, r9
	ctx.r[9].u64 = ctx.r[28].u64 + ctx.r[9].u64;
	// 832C80CC: 1FBD0EC8  mulli r29, r29, 0xec8
	ctx.r[29].s32 = ((ctx.r[29].s32 as i64 * 3784 as i64) as i32);
	ctx.r[29].s64 = ctx.r[29].s32 as i64;
	// 832C80D0: 1F9BEB18  mulli r28, r27, -0x14e8
	ctx.r[28].s32 = ((ctx.r[27].s32 as i64 * -5352 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 832C80D4: 7F654850  subf r27, r5, r9
	ctx.r[27].s64 = ctx.r[9].s64 - ctx.r[5].s64;
	// 832C80D8: 7D292A14  add r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[5].u64;
	// 832C80DC: 7FBD5E70  srawi r29, r29, 0xb
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 11) as i64;
	// 832C80E0: 7F855E70  srawi r5, r28, 0xb
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[28].s32 >> 11) as i64;
	// 832C80E4: 1F9B0B50  mulli r28, r27, 0xb50
	ctx.r[28].s32 = ((ctx.r[27].s32 as i64 * 2896 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 832C80E8: 7CA92850  subf r5, r9, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 832C80EC: 1F7A08A9  mulli r27, r26, 0x8a9
	ctx.r[27].s32 = ((ctx.r[26].s32 as i64 * 2217 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C80F0: 7F9A5E70  srawi r26, r28, 0xb
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[28].s32 >> 11) as i64;
	// 832C80F4: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 832C80F8: 7F9EF850  subf r28, r30, r31
	ctx.r[28].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 832C80FC: 7CE5EA14  add r7, r5, r29
	ctx.r[7].u64 = ctx.r[5].u64 + ctx.r[29].u64;
	// 832C8100: 7FFEFA14  add r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 832C8104: 7F775E70  srawi r23, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[23].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C8108: 7F68C850  subf r27, r8, r25
	ctx.r[27].s64 = ctx.r[25].s64 - ctx.r[8].s64;
	// 832C810C: 7FC8FA14  add r30, r8, r31
	ctx.r[30].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C8110: 7CA7D050  subf r5, r7, r26
	ctx.r[5].s64 = ctx.r[26].s64 - ctx.r[7].s64;
	// 832C8114: 7F48F850  subf r26, r8, r31
	ctx.r[26].s64 = ctx.r[31].s64 - ctx.r[8].s64;
	// 832C8118: 7F3DB850  subf r25, r29, r23
	ctx.r[25].s64 = ctx.r[23].s64 - ctx.r[29].s64;
	// 832C811C: 7D1BE214  add r8, r27, r28
	ctx.r[8].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 832C8120: 7FBBE050  subf r29, r27, r28
	ctx.r[29].s64 = ctx.r[28].s64 - ctx.r[27].s64;
	// 832C8124: 7F89F214  add r28, r9, r30
	ctx.r[28].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 832C8128: 7D29F050  subf r9, r9, r30
	ctx.r[9].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	// 832C812C: 7FC74214  add r30, r7, r8
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 832C8130: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832C8134: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 832C8138: 912B00E0  stw r9, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[9].u32 ) };
	// 832C813C: 7FF92A14  add r31, r25, r5
	ctx.r[31].u64 = ctx.r[25].u64 + ctx.r[5].u64;
	// 832C8140: 93CB0020  stw r30, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 832C8144: 7CE5EA14  add r7, r5, r29
	ctx.r[7].u64 = ctx.r[5].u64 + ctx.r[29].u64;
	// 832C8148: 910B00C0  stw r8, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[8].u32 ) };
	// 832C814C: 7CA5E850  subf r5, r5, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[5].s64;
	// 832C8150: 7D1FD050  subf r8, r31, r26
	ctx.r[8].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 832C8154: 90EB0040  stw r7, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[7].u32 ) };
	// 832C8158: 7D3FD214  add r9, r31, r26
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[26].u64;
	// 832C815C: 90AB00A0  stw r5, 0xa0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[5].u32 ) };
	// 832C8160: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	pc = 0x832C8164; continue 'dispatch;
            }
            0x832C8164 => {
    //   block [0x832C8164..0x832C8184)
	// 832C8164: 912B0080  stw r9, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 832C8168: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C816C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8170: 3718FFFF  addic. r24, r24, -1
	ctx.xer.ca = (ctx.r[24].u32 > (!(-1 as u32)));
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 832C8174: 4181FE38  bgt 0x832c7fac
	if ctx.cr[0].gt {
	pc = 0x832C7FAC; continue 'dispatch;
	}
	// 832C8178: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 832C817C: 3961FEA8  addi r11, r1, -0x158
	ctx.r[11].s64 = ctx.r[1].s64 + -344;
	// 832C8180: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	pc = 0x832C8184; continue 'dispatch;
            }
            0x832C8184 => {
    //   block [0x832C8184..0x832C82B4)
	// 832C8184: 810BFFF4  lwz r8, -0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 832C8188: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C818C: 80CBFFEC  lwz r6, -0x14(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) } as u64;
	// 832C8190: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 832C8194: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8198: 7FC73050  subf r30, r7, r6
	ctx.r[30].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 832C819C: 83EBFFF0  lwz r31, -0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) } as u64;
	// 832C81A0: 7C684850  subf r3, r8, r9
	ctx.r[3].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 832C81A4: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 832C81A8: 7F9DF850  subf r28, r29, r31
	ctx.r[28].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 832C81AC: 7F7E1A14  add r27, r30, r3
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 832C81B0: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 832C81B4: 80EBFFF8  lwz r7, -8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C81B8: 1F9C0B50  mulli r28, r28, 0xb50
	ctx.r[28].s32 = ((ctx.r[28].s32 as i64 * 2896 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 832C81BC: 80CBFFE8  lwz r6, -0x18(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) } as u64;
	// 832C81C0: 1F7B0EC8  mulli r27, r27, 0xec8
	ctx.r[27].s32 = ((ctx.r[27].s32 as i64 * 3784 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C81C4: 1F23EB18  mulli r25, r3, -0x14e8
	ctx.r[25].s32 = ((ctx.r[3].s32 as i64 * -5352 as i64) as i32);
	ctx.r[25].s64 = ctx.r[25].s32 as i64;
	// 832C81C8: 7F494050  subf r26, r9, r8
	ctx.r[26].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 832C81CC: 7F985E70  srawi r24, r28, 0xb
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[24].s64 = (ctx.r[28].s32 >> 11) as i64;
	// 832C81D0: 7F635E70  srawi r3, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C81D4: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 832C81D8: 7F3C5E70  srawi r28, r25, 0xb
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[25].s32 >> 11) as i64;
	// 832C81DC: 1F7A0B50  mulli r27, r26, 0xb50
	ctx.r[27].s32 = ((ctx.r[26].s32 as i64 * 2896 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C81E0: 7D1DFA14  add r8, r29, r31
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 832C81E4: 1FDE08A9  mulli r30, r30, 0x8a9
	ctx.r[30].s32 = ((ctx.r[30].s32 as i64 * 2217 as i64) as i32);
	ctx.r[30].s64 = ctx.r[30].s32 as i64;
	// 832C81E8: 7FA9E050  subf r29, r9, r28
	ctx.r[29].s64 = ctx.r[28].s64 - ctx.r[9].s64;
	// 832C81EC: 7FE63A14  add r31, r6, r7
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 832C81F0: 7F873050  subf r28, r7, r6
	ctx.r[28].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 832C81F4: 7F7A5E70  srawi r26, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C81F8: 7FC65E70  srawi r6, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 832C81FC: 7F68C050  subf r27, r8, r24
	ctx.r[27].s64 = ctx.r[24].s64 - ctx.r[8].s64;
	// 832C8200: 7CFD1A14  add r7, r29, r3
	ctx.r[7].u64 = ctx.r[29].u64 + ctx.r[3].u64;
	// 832C8204: 7FC8FA14  add r30, r8, r31
	ctx.r[30].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 832C8208: 7F233050  subf r25, r3, r6
	ctx.r[25].s64 = ctx.r[6].s64 - ctx.r[3].s64;
	// 832C820C: 7FBBE214  add r29, r27, r28
	ctx.r[29].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 832C8210: 7CC7D050  subf r6, r7, r26
	ctx.r[6].s64 = ctx.r[26].s64 - ctx.r[7].s64;
	// 832C8214: 7C7BE050  subf r3, r27, r28
	ctx.r[3].s64 = ctx.r[28].s64 - ctx.r[27].s64;
	// 832C8218: 7F49F214  add r26, r9, r30
	ctx.r[26].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 832C821C: 7F89F050  subf r28, r9, r30
	ctx.r[28].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	// 832C8220: 7FC7EA14  add r30, r7, r29
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[29].u64;
	// 832C8224: 7CE7E850  subf r7, r7, r29
	ctx.r[7].s64 = ctx.r[29].s64 - ctx.r[7].s64;
	// 832C8228: 7D08F850  subf r8, r8, r31
	ctx.r[8].s64 = ctx.r[31].s64 - ctx.r[8].s64;
	// 832C822C: 7D393214  add r9, r25, r6
	ctx.r[9].u64 = ctx.r[25].u64 + ctx.r[6].u64;
	// 832C8230: 3BBA007F  addi r29, r26, 0x7f
	ctx.r[29].s64 = ctx.r[26].s64 + 127;
	// 832C8234: 7FE61A14  add r31, r6, r3
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 832C8238: 3B9C007F  addi r28, r28, 0x7f
	ctx.r[28].s64 = ctx.r[28].s64 + 127;
	// 832C823C: 3B67007F  addi r27, r7, 0x7f
	ctx.r[27].s64 = ctx.r[7].s64 + 127;
	// 832C8240: 7CC61850  subf r6, r6, r3
	ctx.r[6].s64 = ctx.r[3].s64 - ctx.r[6].s64;
	// 832C8244: 3BDE007F  addi r30, r30, 0x7f
	ctx.r[30].s64 = ctx.r[30].s64 + 127;
	// 832C8248: 7CE94214  add r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 832C824C: 7FA34670  srawi r3, r29, 8
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[29].s32 >> 8) as i64;
	// 832C8250: 7D294050  subf r9, r9, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 832C8254: 7F9D4670  srawi r29, r28, 8
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[28].s32 >> 8) as i64;
	// 832C8258: 986AFFFF  stb r3, -1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-1 as u32), ctx.r[3].u8 ) };
	// 832C825C: 3BFF007F  addi r31, r31, 0x7f
	ctx.r[31].s64 = ctx.r[31].s64 + 127;
	// 832C8260: 7FC84670  srawi r8, r30, 8
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[30].s32 >> 8) as i64;
	// 832C8264: 9BAA0006  stb r29, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[29].u8 ) };
	// 832C8268: 38C6007F  addi r6, r6, 0x7f
	ctx.r[6].s64 = ctx.r[6].s64 + 127;
	// 832C826C: 7F7E4670  srawi r30, r27, 8
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[27].s32 >> 8) as i64;
	// 832C8270: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 832C8274: 38E7007F  addi r7, r7, 0x7f
	ctx.r[7].s64 = ctx.r[7].s64 + 127;
	// 832C8278: 7FFF4670  srawi r31, r31, 8
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 8) as i64;
	// 832C827C: 9BCA0005  stb r30, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[30].u8 ) };
	// 832C8280: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 832C8284: 7CC64670  srawi r6, r6, 8
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[6].s32 >> 8) as i64;
	// 832C8288: 9BEA0001  stb r31, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[31].u8 ) };
	// 832C828C: 7CE74670  srawi r7, r7, 8
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 8) as i64;
	// 832C8290: 7D294670  srawi r9, r9, 8
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 8) as i64;
	// 832C8294: 98CA0004  stb r6, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u8 ) };
	// 832C8298: 98EA0003  stb r7, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 832C829C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C82A0: 992A0002  stb r9, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u8 ) };
	// 832C82A4: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 832C82A8: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 832C82AC: 4082FED8  bne 0x832c8184
	if !ctx.cr[0].eq {
	pc = 0x832C8184; continue 'dispatch;
	}
	// 832C82B0: 4B9E1184  b 0x82ca9434
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C82B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C82B8 size=888
    let mut pc: u32 = 0x832C82B8;
    'dispatch: loop {
        match pc {
            0x832C82B8 => {
    //   block [0x832C82B8..0x832C82D4)
	// 832C82B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C82BC: 4B9E1125  bl 0x82ca93e0
	ctx.lr = 0x832C82C0;
	sub_82CA93D0(ctx, base);
	// 832C82C0: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 832C82C4: 7D232214  add r9, r3, r4
	ctx.r[9].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 832C82C8: 5498083C  slwi r24, r4, 1
	ctx.r[24].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 832C82CC: 3961FE80  addi r11, r1, -0x180
	ctx.r[11].s64 = ctx.r[1].s64 + -384;
	// 832C82D0: 3AE00008  li r23, 8
	ctx.r[23].s64 = 8;
	pc = 0x832C82D4; continue 'dispatch;
            }
            0x832C82D4 => {
    //   block [0x832C82D4..0x832C8368)
	// 832C82D4: A0EA0010  lhz r7, 0x10(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C82D8: A10A0020  lhz r8, 0x20(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C82DC: A0AA0030  lhz r5, 0x30(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C82E0: 7CE40734  extsh r4, r7
	ctx.r[4].s64 = ctx.r[7].s16 as i64;
	// 832C82E4: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 832C82E8: A0EA0040  lhz r7, 0x40(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 832C82EC: 7CBD0734  extsh r29, r5
	ctx.r[29].s64 = ctx.r[5].s16 as i64;
	// 832C82F0: A0AA0050  lhz r5, 0x50(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 832C82F4: 7C9F4378  or r31, r4, r8
	ctx.r[31].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 832C82F8: A3CA0060  lhz r30, 0x60(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C82FC: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 832C8300: A36A0070  lhz r27, 0x70(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(112 as u32) ) } as u64;
	// 832C8304: 7FFFEB78  or r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[29].u64;
	// 832C8308: 7CBC0734  extsh r28, r5
	ctx.r[28].s64 = ctx.r[5].s16 as i64;
	// 832C830C: 7FFF3B78  or r31, r31, r7
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[7].u64;
	// 832C8310: 7FC50734  extsh r5, r30
	ctx.r[5].s64 = ctx.r[30].s16 as i64;
	// 832C8314: 7FFFE378  or r31, r31, r28
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[28].u64;
	// 832C8318: 7F7B0734  extsh r27, r27
	ctx.r[27].s64 = ctx.r[27].s16 as i64;
	// 832C831C: 7FFF2B78  or r31, r31, r5
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[5].u64;
	// 832C8320: 7FFFDB78  or r31, r31, r27
	ctx.r[31].u64 = ctx.r[31].u64 | ctx.r[27].u64;
	// 832C8324: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 832C8328: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832C832C: 409A003C  bne cr6, 0x832c8368
	if !ctx.cr[6].eq {
	pc = 0x832C8368; continue 'dispatch;
	}
	// 832C8330: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8334: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 832C8338: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C833C: 7D050734  extsh r5, r8
	ctx.r[5].s64 = ctx.r[8].s16 as i64;
	// 832C8340: 7C8539D6  mullw r4, r5, r7
	ctx.r[4].s32 = ((ctx.r[5].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 832C8344: 7C885E70  srawi r8, r4, 0xb
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[4].s32 >> 11) as i64;
	// 832C8348: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 832C834C: 910B0020  stw r8, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 832C8350: 910B0040  stw r8, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 832C8354: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 832C8358: 910B00A0  stw r8, 0xa0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[8].u32 ) };
	// 832C835C: 910B00C0  stw r8, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[8].u32 ) };
	// 832C8360: 910B00E0  stw r8, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[8].u32 ) };
	// 832C8364: 48000128  b 0x832c848c
	pc = 0x832C848C; continue 'dispatch;
            }
            0x832C8368 => {
    //   block [0x832C8368..0x832C848C)
	// 832C8368: A3EA0000  lhz r31, 0(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C836C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 832C8370: 83C60000  lwz r30, 0(r6)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8374: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 832C8378: 83460040  lwz r26, 0x40(r6)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(64 as u32) ) } as u64;
	// 832C837C: 83260080  lwz r25, 0x80(r6)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C8380: 7FFFF1D6  mullw r31, r31, r30
	ctx.r[31].s32 = ((ctx.r[31].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[31].s64 = ctx.r[31].s32 as i64;
	// 832C8384: 82C600C0  lwz r22, 0xc0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(192 as u32) ) } as u64;
	// 832C8388: 82A60020  lwz r21, 0x20(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C838C: 82860060  lwz r20, 0x60(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C8390: 826600A0  lwz r19, 0xa0(r6)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(160 as u32) ) } as u64;
	// 832C8394: 824600E0  lwz r18, 0xe0(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C8398: 7D1A41D6  mullw r8, r26, r8
	ctx.r[8].s32 = ((ctx.r[26].s32 as i64 * ctx.r[8].s32 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 832C839C: 7FD939D6  mullw r30, r25, r7
	ctx.r[30].s32 = ((ctx.r[25].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[30].s64 = ctx.r[30].s32 as i64;
	// 832C83A0: 7FFF5E70  srawi r31, r31, 0xb
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 11) as i64;
	// 832C83A4: 7CB629D6  mullw r5, r22, r5
	ctx.r[5].s32 = ((ctx.r[22].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 832C83A8: 7D075E70  srawi r7, r8, 0xb
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[8].s32 >> 11) as i64;
	// 832C83AC: 7FDE5E70  srawi r30, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 832C83B0: 7CA55E70  srawi r5, r5, 0xb
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[5].s32 >> 11) as i64;
	// 832C83B4: 7C9521D6  mullw r4, r21, r4
	ctx.r[4].s32 = ((ctx.r[21].s32 as i64 * ctx.r[4].s32 as i64) as i32);
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 832C83B8: 7D053850  subf r8, r5, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 832C83BC: 7FB4E9D6  mullw r29, r20, r29
	ctx.r[29].s32 = ((ctx.r[20].s32 as i64 * ctx.r[29].s32 as i64) as i32);
	ctx.r[29].s64 = ctx.r[29].s32 as i64;
	// 832C83C0: 1D080B50  mulli r8, r8, 0xb50
	ctx.r[8].s32 = ((ctx.r[8].s32 as i64 * 2896 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 832C83C4: 7D195E70  srawi r25, r8, 0xb
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[8].s32 >> 11) as i64;
	// 832C83C8: 7F93E1D6  mullw r28, r19, r28
	ctx.r[28].s32 = ((ctx.r[19].s32 as i64 * ctx.r[28].s32 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 832C83CC: 7C885E70  srawi r8, r4, 0xb
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[4].s32 >> 11) as i64;
	// 832C83D0: 7F72D9D6  mullw r27, r18, r27
	ctx.r[27].s32 = ((ctx.r[18].s32 as i64 * ctx.r[27].s32 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C83D4: 7FA45E70  srawi r4, r29, 0xb
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[29].s32 >> 11) as i64;
	// 832C83D8: 7F9D5E70  srawi r29, r28, 0xb
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[28].s32 >> 11) as i64;
	// 832C83DC: 7F7C5E70  srawi r28, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C83E0: 7F64E850  subf r27, r4, r29
	ctx.r[27].s64 = ctx.r[29].s64 - ctx.r[4].s64;
	// 832C83E4: 7F5C4050  subf r26, r28, r8
	ctx.r[26].s64 = ctx.r[8].s64 - ctx.r[28].s64;
	// 832C83E8: 7C9D2214  add r4, r29, r4
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[4].u64;
	// 832C83EC: 7FBADA14  add r29, r26, r27
	ctx.r[29].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 832C83F0: 7D1C4214  add r8, r28, r8
	ctx.r[8].u64 = ctx.r[28].u64 + ctx.r[8].u64;
	// 832C83F4: 1FBD0EC8  mulli r29, r29, 0xec8
	ctx.r[29].s32 = ((ctx.r[29].s32 as i64 * 3784 as i64) as i32);
	ctx.r[29].s64 = ctx.r[29].s32 as i64;
	// 832C83F8: 1F9BEB18  mulli r28, r27, -0x14e8
	ctx.r[28].s32 = ((ctx.r[27].s32 as i64 * -5352 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 832C83FC: 7F644050  subf r27, r4, r8
	ctx.r[27].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 832C8400: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 832C8404: 7FBD5E70  srawi r29, r29, 0xb
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 11) as i64;
	// 832C8408: 7F845E70  srawi r4, r28, 0xb
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[28].s32 >> 11) as i64;
	// 832C840C: 1F9B0B50  mulli r28, r27, 0xb50
	ctx.r[28].s32 = ((ctx.r[27].s32 as i64 * 2896 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 832C8410: 7C882050  subf r4, r8, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[8].s64;
	// 832C8414: 1F7A08A9  mulli r27, r26, 0x8a9
	ctx.r[27].s32 = ((ctx.r[26].s32 as i64 * 2217 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C8418: 7F9A5E70  srawi r26, r28, 0xb
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[28].s32 >> 11) as i64;
	// 832C841C: 7CE53A14  add r7, r5, r7
	ctx.r[7].u64 = ctx.r[5].u64 + ctx.r[7].u64;
	// 832C8420: 7F9EF850  subf r28, r30, r31
	ctx.r[28].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 832C8424: 7CA4EA14  add r5, r4, r29
	ctx.r[5].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 832C8428: 7FFEFA14  add r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 832C842C: 7F765E70  srawi r22, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[22].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C8430: 7F67C850  subf r27, r7, r25
	ctx.r[27].s64 = ctx.r[25].s64 - ctx.r[7].s64;
	// 832C8434: 7FC7FA14  add r30, r7, r31
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C8438: 7C85D050  subf r4, r5, r26
	ctx.r[4].s64 = ctx.r[26].s64 - ctx.r[5].s64;
	// 832C843C: 7F47F850  subf r26, r7, r31
	ctx.r[26].s64 = ctx.r[31].s64 - ctx.r[7].s64;
	// 832C8440: 7F3DB050  subf r25, r29, r22
	ctx.r[25].s64 = ctx.r[22].s64 - ctx.r[29].s64;
	// 832C8444: 7CFBE214  add r7, r27, r28
	ctx.r[7].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 832C8448: 7FBBE050  subf r29, r27, r28
	ctx.r[29].s64 = ctx.r[28].s64 - ctx.r[27].s64;
	// 832C844C: 7F88F214  add r28, r8, r30
	ctx.r[28].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 832C8450: 7D08F050  subf r8, r8, r30
	ctx.r[8].s64 = ctx.r[30].s64 - ctx.r[8].s64;
	// 832C8454: 7FC53A14  add r30, r5, r7
	ctx.r[30].u64 = ctx.r[5].u64 + ctx.r[7].u64;
	// 832C8458: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832C845C: 7CE53850  subf r7, r5, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 832C8460: 910B00E0  stw r8, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[8].u32 ) };
	// 832C8464: 7FF92214  add r31, r25, r4
	ctx.r[31].u64 = ctx.r[25].u64 + ctx.r[4].u64;
	// 832C8468: 93CB0020  stw r30, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 832C846C: 7CA4EA14  add r5, r4, r29
	ctx.r[5].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 832C8470: 90EB00C0  stw r7, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[7].u32 ) };
	// 832C8474: 7C84E850  subf r4, r4, r29
	ctx.r[4].s64 = ctx.r[29].s64 - ctx.r[4].s64;
	// 832C8478: 7CFFD050  subf r7, r31, r26
	ctx.r[7].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 832C847C: 90AB0040  stw r5, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[5].u32 ) };
	// 832C8480: 7D1FD214  add r8, r31, r26
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[26].u64;
	// 832C8484: 908B00A0  stw r4, 0xa0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[4].u32 ) };
	// 832C8488: 90EB0060  stw r7, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	pc = 0x832C848C; continue 'dispatch;
            }
            0x832C848C => {
    //   block [0x832C848C..0x832C84AC)
	// 832C848C: 910B0080  stw r8, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 832C8490: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C8494: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8498: 36F7FFFF  addic. r23, r23, -1
	ctx.xer.ca = (ctx.r[23].u32 > (!(-1 as u32)));
	ctx.r[23].s64 = ctx.r[23].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 832C849C: 4181FE38  bgt 0x832c82d4
	if ctx.cr[0].gt {
	pc = 0x832C82D4; continue 'dispatch;
	}
	// 832C84A0: 3961FE98  addi r11, r1, -0x168
	ctx.r[11].s64 = ctx.r[1].s64 + -360;
	// 832C84A4: 39430008  addi r10, r3, 8
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	// 832C84A8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	pc = 0x832C84AC; continue 'dispatch;
            }
            0x832C84AC => {
    //   block [0x832C84AC..0x832C8630)
	// 832C84AC: 80EBFFF4  lwz r7, -0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 832C84B0: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 832C84B4: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C84B8: 80ABFFEC  lwz r5, -0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) } as u64;
	// 832C84BC: 7C674050  subf r3, r7, r8
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 832C84C0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C84C4: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 832C84C8: 83ABFFF0  lwz r29, -0x10(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) } as u64;
	// 832C84CC: 7FE62850  subf r31, r6, r5
	ctx.r[31].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 832C84D0: 838BFFE8  lwz r28, -0x18(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) } as u64;
	// 832C84D4: 7CFEE850  subf r7, r30, r29
	ctx.r[7].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 832C84D8: 7F7F1A14  add r27, r31, r3
	ctx.r[27].u64 = ctx.r[31].u64 + ctx.r[3].u64;
	// 832C84DC: 1F470B50  mulli r26, r7, 0xb50
	ctx.r[26].s32 = ((ctx.r[7].s32 as i64 * 2896 as i64) as i32);
	ctx.r[26].s64 = ctx.r[26].s32 as i64;
	// 832C84E0: 7CE53214  add r7, r5, r6
	ctx.r[7].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 832C84E4: 80ABFFF8  lwz r5, -8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C84E8: 1CDB0EC8  mulli r6, r27, 0xec8
	ctx.r[6].s32 = ((ctx.r[27].s32 as i64 * 3784 as i64) as i32);
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 832C84EC: 1F63EB18  mulli r27, r3, -0x14e8
	ctx.r[27].s32 = ((ctx.r[3].s32 as i64 * -5352 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C84F0: 7F5A5E70  srawi r26, r26, 0xb
	ctx.xer.ca = (ctx.r[26].s32 < 0) && ((ctx.r[26].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[26].s32 >> 11) as i64;
	// 832C84F4: 7F283850  subf r25, r8, r7
	ctx.r[25].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 832C84F8: 7CC35E70  srawi r3, r6, 0xb
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[6].s32 >> 11) as i64;
	// 832C84FC: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 832C8500: 7F665E70  srawi r6, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C8504: 1F790B50  mulli r27, r25, 0xb50
	ctx.r[27].s32 = ((ctx.r[25].s32 as i64 * 2896 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C8508: 7CFDF214  add r7, r29, r30
	ctx.r[7].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 832C850C: 7CC83050  subf r6, r8, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[8].s64;
	// 832C8510: 1FDF08A9  mulli r30, r31, 0x8a9
	ctx.r[30].s32 = ((ctx.r[31].s32 as i64 * 2217 as i64) as i32);
	ctx.r[30].s64 = ctx.r[30].s32 as i64;
	// 832C8514: 7F7D5E70  srawi r29, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C8518: 7CC61A14  add r6, r6, r3
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 832C851C: 7F67D050  subf r27, r7, r26
	ctx.r[27].s64 = ctx.r[26].s64 - ctx.r[7].s64;
	// 832C8520: 7FFC2A14  add r31, r28, r5
	ctx.r[31].u64 = ctx.r[28].u64 + ctx.r[5].u64;
	// 832C8524: 7FDA5E70  srawi r26, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 832C8528: 7F85E050  subf r28, r5, r28
	ctx.r[28].s64 = ctx.r[28].s64 - ctx.r[5].s64;
	// 832C852C: 7CA6E850  subf r5, r6, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	// 832C8530: 7FC7FA14  add r30, r7, r31
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C8534: 7C63D050  subf r3, r3, r26
	ctx.r[3].s64 = ctx.r[26].s64 - ctx.r[3].s64;
	// 832C8538: 7FBBE214  add r29, r27, r28
	ctx.r[29].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 832C853C: 7F48F214  add r26, r8, r30
	ctx.r[26].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 832C8540: 7FE7F850  subf r31, r7, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[7].s64;
	// 832C8544: 7F26EA14  add r25, r6, r29
	ctx.r[25].u64 = ctx.r[6].u64 + ctx.r[29].u64;
	// 832C8548: 7CE32A14  add r7, r3, r5
	ctx.r[7].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 832C854C: 7CC6E850  subf r6, r6, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	// 832C8550: 7C7BE050  subf r3, r27, r28
	ctx.r[3].s64 = ctx.r[28].s64 - ctx.r[27].s64;
	// 832C8554: 7FC8F050  subf r30, r8, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[8].s64;
	// 832C8558: 3B9A007F  addi r28, r26, 0x7f
	ctx.r[28].s64 = ctx.r[26].s64 + 127;
	// 832C855C: 7FA7FA14  add r29, r7, r31
	ctx.r[29].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 832C8560: 3B79007F  addi r27, r25, 0x7f
	ctx.r[27].s64 = ctx.r[25].s64 + 127;
	// 832C8564: 7D051850  subf r8, r5, r3
	ctx.r[8].s64 = ctx.r[3].s64 - ctx.r[5].s64;
	// 832C8568: 38C6007F  addi r6, r6, 0x7f
	ctx.r[6].s64 = ctx.r[6].s64 + 127;
	// 832C856C: 3BDE007F  addi r30, r30, 0x7f
	ctx.r[30].s64 = ctx.r[30].s64 + 127;
	// 832C8570: 7F9C4670  srawi r28, r28, 8
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[28].s32 >> 8) as i64;
	// 832C8574: 3BBD007F  addi r29, r29, 0x7f
	ctx.r[29].s64 = ctx.r[29].s64 + 127;
	// 832C8578: 7F7B4670  srawi r27, r27, 8
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[27].s32 >> 8) as i64;
	// 832C857C: 7CDA4670  srawi r26, r6, 8
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[6].s32 >> 8) as i64;
	// 832C8580: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 832C8584: 7FDE4670  srawi r30, r30, 8
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[30].s32 >> 8) as i64;
	// 832C8588: 7CC51A14  add r6, r5, r3
	ctx.r[6].u64 = ctx.r[5].u64 + ctx.r[3].u64;
	// 832C858C: 7FBD4670  srawi r29, r29, 8
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 8) as i64;
	// 832C8590: 7D054670  srawi r5, r8, 8
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[8].s32 >> 8) as i64;
	// 832C8594: 7D07F850  subf r8, r7, r31
	ctx.r[8].s64 = ctx.r[31].s64 - ctx.r[7].s64;
	// 832C8598: 3866007F  addi r3, r6, 0x7f
	ctx.r[3].s64 = ctx.r[6].s64 + 127;
	// 832C859C: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 832C85A0: 5766063E  clrlwi r6, r27, 0x18
	ctx.r[6].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 832C85A4: 5787821E  rlwinm r7, r28, 0x10, 8, 0xf
	ctx.r[7].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 832C85A8: 7C634670  srawi r3, r3, 8
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 8) as i64;
	// 832C85AC: 575F821E  rlwinm r31, r26, 0x10, 8, 0xf
	ctx.r[31].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 832C85B0: 57DE063E  clrlwi r30, r30, 0x18
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 832C85B4: 57BD821E  rlwinm r29, r29, 0x10, 8, 0xf
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 832C85B8: 7D084670  srawi r8, r8, 8
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 8) as i64;
	// 832C85BC: 54A5063E  clrlwi r5, r5, 0x18
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 832C85C0: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 832C85C4: 7FE6F378  or r6, r31, r30
	ctx.r[6].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 832C85C8: 7FA52B78  or r5, r29, r5
	ctx.r[5].u64 = ctx.r[29].u64 | ctx.r[5].u64;
	// 832C85CC: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 832C85D0: 5463821E  rlwinm r3, r3, 0x10, 8, 0xf
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 832C85D4: 54FF402E  slwi r31, r7, 8
	ctx.r[31].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 832C85D8: 54DE402E  slwi r30, r6, 8
	ctx.r[30].u32 = ctx.r[6].u32.wrapping_shl(8);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832C85DC: 54BD402E  slwi r29, r5, 8
	ctx.r[29].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 832C85E0: 7C634378  or r3, r3, r8
	ctx.r[3].u64 = ctx.r[3].u64 | ctx.r[8].u64;
	// 832C85E4: 7FE83B78  or r8, r31, r7
	ctx.r[8].u64 = ctx.r[31].u64 | ctx.r[7].u64;
	// 832C85E8: 7FC73378  or r7, r30, r6
	ctx.r[7].u64 = ctx.r[30].u64 | ctx.r[6].u64;
	// 832C85EC: 7FA62B78  or r6, r29, r5
	ctx.r[6].u64 = ctx.r[29].u64 | ctx.r[5].u64;
	// 832C85F0: 5465402E  slwi r5, r3, 8
	ctx.r[5].u32 = ctx.r[3].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832C85F4: 910AFFF8  stw r8, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[8].u32 ) };
	// 832C85F8: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 832C85FC: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 832C8600: 7CA31B78  or r3, r5, r3
	ctx.r[3].u64 = ctx.r[5].u64 | ctx.r[3].u64;
	// 832C8604: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 832C8608: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C860C: 906AFFFC  stw r3, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[3].u32 ) };
	// 832C8610: 7D4AC214  add r10, r10, r24
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 832C8614: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 832C8618: 90690004  stw r3, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 832C861C: 90C90008  stw r6, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 832C8620: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 832C8624: 7D384A14  add r9, r24, r9
	ctx.r[9].u64 = ctx.r[24].u64 + ctx.r[9].u64;
	// 832C8628: 4082FE84  bne 0x832c84ac
	if !ctx.cr[0].eq {
	pc = 0x832C84AC; continue 'dispatch;
	}
	// 832C862C: 4B9E0E04  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C8630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C8630 size=888
    let mut pc: u32 = 0x832C8630;
    'dispatch: loop {
        match pc {
            0x832C8630 => {
    //   block [0x832C8630..0x832C8654)
	// 832C8630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C8634: 4B9E0DA9  bl 0x82ca93dc
	ctx.lr = 0x832C8638;
	sub_82CA93D0(ctx, base);
	// 832C8638: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 832C863C: 54CA402E  slwi r10, r6, 8
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8640: 396B7040  addi r11, r11, 0x7040
	ctx.r[11].s64 = ctx.r[11].s64 + 28736;
	// 832C8644: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 832C8648: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832C864C: 3961FE80  addi r11, r1, -0x180
	ctx.r[11].s64 = ctx.r[1].s64 + -384;
	// 832C8650: 3AC00008  li r22, 8
	ctx.r[22].s64 = 8;
	pc = 0x832C8654; continue 'dispatch;
            }
            0x832C8654 => {
    //   block [0x832C8654..0x832C86E8)
	// 832C8654: A0A90010  lhz r5, 0x10(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 832C8658: A0C90020  lhz r6, 0x20(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C865C: A3E90030  lhz r31, 0x30(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 832C8660: 7CBE0734  extsh r30, r5
	ctx.r[30].s64 = ctx.r[5].s16 as i64;
	// 832C8664: 7CC60734  extsh r6, r6
	ctx.r[6].s64 = ctx.r[6].s16 as i64;
	// 832C8668: A0A90040  lhz r5, 0x40(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(64 as u32) ) } as u64;
	// 832C866C: 7FFB0734  extsh r27, r31
	ctx.r[27].s64 = ctx.r[31].s16 as i64;
	// 832C8670: A3E90050  lhz r31, 0x50(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(80 as u32) ) } as u64;
	// 832C8674: 7FDD3378  or r29, r30, r6
	ctx.r[29].u64 = ctx.r[30].u64 | ctx.r[6].u64;
	// 832C8678: A3890060  lhz r28, 0x60(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C867C: 7CA50734  extsh r5, r5
	ctx.r[5].s64 = ctx.r[5].s16 as i64;
	// 832C8680: A3290070  lhz r25, 0x70(r9)
	ctx.r[25].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(112 as u32) ) } as u64;
	// 832C8684: 7FBDDB78  or r29, r29, r27
	ctx.r[29].u64 = ctx.r[29].u64 | ctx.r[27].u64;
	// 832C8688: 7FFA0734  extsh r26, r31
	ctx.r[26].s64 = ctx.r[31].s16 as i64;
	// 832C868C: 7FBD2B78  or r29, r29, r5
	ctx.r[29].u64 = ctx.r[29].u64 | ctx.r[5].u64;
	// 832C8690: 7F9F0734  extsh r31, r28
	ctx.r[31].s64 = ctx.r[28].s16 as i64;
	// 832C8694: 7FBDD378  or r29, r29, r26
	ctx.r[29].u64 = ctx.r[29].u64 | ctx.r[26].u64;
	// 832C8698: 7F390734  extsh r25, r25
	ctx.r[25].s64 = ctx.r[25].s16 as i64;
	// 832C869C: 7FBDFB78  or r29, r29, r31
	ctx.r[29].u64 = ctx.r[29].u64 | ctx.r[31].u64;
	// 832C86A0: 7FBDCB78  or r29, r29, r25
	ctx.r[29].u64 = ctx.r[29].u64 | ctx.r[25].u64;
	// 832C86A4: 7FBD0734  extsh r29, r29
	ctx.r[29].s64 = ctx.r[29].s16 as i64;
	// 832C86A8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832C86AC: 409A003C  bne cr6, 0x832c86e8
	if !ctx.cr[6].eq {
	pc = 0x832C86E8; continue 'dispatch;
	}
	// 832C86B0: A0C90000  lhz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C86B4: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 832C86B8: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C86BC: 7CC60734  extsh r6, r6
	ctx.r[6].s64 = ctx.r[6].s16 as i64;
	// 832C86C0: 7CA629D6  mullw r5, r6, r5
	ctx.r[5].s32 = ((ctx.r[6].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 832C86C4: 7CA65E70  srawi r6, r5, 0xb
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[5].s32 >> 11) as i64;
	// 832C86C8: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 832C86CC: 90CB0020  stw r6, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 832C86D0: 90CB0040  stw r6, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[6].u32 ) };
	// 832C86D4: 90CB0080  stw r6, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[6].u32 ) };
	// 832C86D8: 90CB00A0  stw r6, 0xa0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[6].u32 ) };
	// 832C86DC: 90CB00C0  stw r6, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[6].u32 ) };
	// 832C86E0: 90CB00E0  stw r6, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[6].u32 ) };
	// 832C86E4: 48000128  b 0x832c880c
	pc = 0x832C880C; continue 'dispatch;
            }
            0x832C86E8 => {
    //   block [0x832C86E8..0x832C880C)
	// 832C86E8: A3A90000  lhz r29, 0(r9)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C86EC: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 832C86F0: 838A0000  lwz r28, 0(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C86F4: 7FBD0734  extsh r29, r29
	ctx.r[29].s64 = ctx.r[29].s16 as i64;
	// 832C86F8: 830A0040  lwz r24, 0x40(r10)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 832C86FC: 82EA0080  lwz r23, 0x80(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 832C8700: 7FBDE1D6  mullw r29, r29, r28
	ctx.r[29].s32 = ((ctx.r[29].s32 as i64 * ctx.r[28].s32 as i64) as i32);
	ctx.r[29].s64 = ctx.r[29].s32 as i64;
	// 832C8704: 82AA00C0  lwz r21, 0xc0(r10)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(192 as u32) ) } as u64;
	// 832C8708: 828A0020  lwz r20, 0x20(r10)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 832C870C: 826A0060  lwz r19, 0x60(r10)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 832C8710: 824A00A0  lwz r18, 0xa0(r10)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(160 as u32) ) } as u64;
	// 832C8714: 822A00E0  lwz r17, 0xe0(r10)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(224 as u32) ) } as u64;
	// 832C8718: 7CD831D6  mullw r6, r24, r6
	ctx.r[6].s32 = ((ctx.r[24].s32 as i64 * ctx.r[6].s32 as i64) as i32);
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 832C871C: 7F9729D6  mullw r28, r23, r5
	ctx.r[28].s32 = ((ctx.r[23].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 832C8720: 7FBD5E70  srawi r29, r29, 0xb
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 11) as i64;
	// 832C8724: 7FF5F9D6  mullw r31, r21, r31
	ctx.r[31].s32 = ((ctx.r[21].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[31].s64 = ctx.r[31].s32 as i64;
	// 832C8728: 7CC55E70  srawi r5, r6, 0xb
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[6].s32 >> 11) as i64;
	// 832C872C: 7F9C5E70  srawi r28, r28, 0xb
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[28].s32 >> 11) as i64;
	// 832C8730: 7FFF5E70  srawi r31, r31, 0xb
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 11) as i64;
	// 832C8734: 7CD4F1D6  mullw r6, r20, r30
	ctx.r[6].s32 = ((ctx.r[20].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 832C8738: 7FDF2850  subf r30, r31, r5
	ctx.r[30].s64 = ctx.r[5].s64 - ctx.r[31].s64;
	// 832C873C: 7F73D9D6  mullw r27, r19, r27
	ctx.r[27].s32 = ((ctx.r[19].s32 as i64 * ctx.r[27].s32 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C8740: 1FDE0B50  mulli r30, r30, 0xb50
	ctx.r[30].s32 = ((ctx.r[30].s32 as i64 * 2896 as i64) as i32);
	ctx.r[30].s64 = ctx.r[30].s32 as i64;
	// 832C8744: 7FD75E70  srawi r23, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[23].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 832C8748: 7F52D1D6  mullw r26, r18, r26
	ctx.r[26].s32 = ((ctx.r[18].s32 as i64 * ctx.r[26].s32 as i64) as i32);
	ctx.r[26].s64 = ctx.r[26].s32 as i64;
	// 832C874C: 7CC65E70  srawi r6, r6, 0xb
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[6].s32 >> 11) as i64;
	// 832C8750: 7F31C9D6  mullw r25, r17, r25
	ctx.r[25].s32 = ((ctx.r[17].s32 as i64 * ctx.r[25].s32 as i64) as i32);
	ctx.r[25].s64 = ctx.r[25].s32 as i64;
	// 832C8754: 7F7E5E70  srawi r30, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C8758: 7F5B5E70  srawi r27, r26, 0xb
	ctx.xer.ca = (ctx.r[26].s32 < 0) && ((ctx.r[26].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[26].s32 >> 11) as i64;
	// 832C875C: 7F3A5E70  srawi r26, r25, 0xb
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[25].s32 >> 11) as i64;
	// 832C8760: 7F3ED850  subf r25, r30, r27
	ctx.r[25].s64 = ctx.r[27].s64 - ctx.r[30].s64;
	// 832C8764: 7F1A3050  subf r24, r26, r6
	ctx.r[24].s64 = ctx.r[6].s64 - ctx.r[26].s64;
	// 832C8768: 7FDBF214  add r30, r27, r30
	ctx.r[30].u64 = ctx.r[27].u64 + ctx.r[30].u64;
	// 832C876C: 7F78CA14  add r27, r24, r25
	ctx.r[27].u64 = ctx.r[24].u64 + ctx.r[25].u64;
	// 832C8770: 7CDA3214  add r6, r26, r6
	ctx.r[6].u64 = ctx.r[26].u64 + ctx.r[6].u64;
	// 832C8774: 1F7B0EC8  mulli r27, r27, 0xec8
	ctx.r[27].s32 = ((ctx.r[27].s32 as i64 * 3784 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	// 832C8778: 1F59EB18  mulli r26, r25, -0x14e8
	ctx.r[26].s32 = ((ctx.r[25].s32 as i64 * -5352 as i64) as i32);
	ctx.r[26].s64 = ctx.r[26].s32 as i64;
	// 832C877C: 7F3E3050  subf r25, r30, r6
	ctx.r[25].s64 = ctx.r[6].s64 - ctx.r[30].s64;
	// 832C8780: 7CC6F214  add r6, r6, r30
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[30].u64;
	// 832C8784: 7F7B5E70  srawi r27, r27, 0xb
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[27].s32 >> 11) as i64;
	// 832C8788: 7F5E5E70  srawi r30, r26, 0xb
	ctx.xer.ca = (ctx.r[26].s32 < 0) && ((ctx.r[26].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[26].s32 >> 11) as i64;
	// 832C878C: 1F590B50  mulli r26, r25, 0xb50
	ctx.r[26].s32 = ((ctx.r[25].s32 as i64 * 2896 as i64) as i32);
	ctx.r[26].s64 = ctx.r[26].s32 as i64;
	// 832C8790: 7FC6F050  subf r30, r6, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[6].s64;
	// 832C8794: 1F3808A9  mulli r25, r24, 0x8a9
	ctx.r[25].s32 = ((ctx.r[24].s32 as i64 * 2217 as i64) as i32);
	ctx.r[25].s64 = ctx.r[25].s32 as i64;
	// 832C8798: 7F585E70  srawi r24, r26, 0xb
	ctx.xer.ca = (ctx.r[26].s32 < 0) && ((ctx.r[26].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[24].s64 = (ctx.r[26].s32 >> 11) as i64;
	// 832C879C: 7CBF2A14  add r5, r31, r5
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 832C87A0: 7F5CE850  subf r26, r28, r29
	ctx.r[26].s64 = ctx.r[29].s64 - ctx.r[28].s64;
	// 832C87A4: 7FFEDA14  add r31, r30, r27
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 832C87A8: 7FBCEA14  add r29, r28, r29
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 832C87AC: 7F355E70  srawi r21, r25, 0xb
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[21].s64 = (ctx.r[25].s32 >> 11) as i64;
	// 832C87B0: 7F25B850  subf r25, r5, r23
	ctx.r[25].s64 = ctx.r[23].s64 - ctx.r[5].s64;
	// 832C87B4: 7F85EA14  add r28, r5, r29
	ctx.r[28].u64 = ctx.r[5].u64 + ctx.r[29].u64;
	// 832C87B8: 7FDFC050  subf r30, r31, r24
	ctx.r[30].s64 = ctx.r[24].s64 - ctx.r[31].s64;
	// 832C87BC: 7F05E850  subf r24, r5, r29
	ctx.r[24].s64 = ctx.r[29].s64 - ctx.r[5].s64;
	// 832C87C0: 7EFBA850  subf r23, r27, r21
	ctx.r[23].s64 = ctx.r[21].s64 - ctx.r[27].s64;
	// 832C87C4: 7CB9D214  add r5, r25, r26
	ctx.r[5].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 832C87C8: 7F79D050  subf r27, r25, r26
	ctx.r[27].s64 = ctx.r[26].s64 - ctx.r[25].s64;
	// 832C87CC: 7F46E214  add r26, r6, r28
	ctx.r[26].u64 = ctx.r[6].u64 + ctx.r[28].u64;
	// 832C87D0: 7CC6E050  subf r6, r6, r28
	ctx.r[6].s64 = ctx.r[28].s64 - ctx.r[6].s64;
	// 832C87D4: 7F9F2A14  add r28, r31, r5
	ctx.r[28].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 832C87D8: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 832C87DC: 7CBF2850  subf r5, r31, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[31].s64;
	// 832C87E0: 90CB00E0  stw r6, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[6].u32 ) };
	// 832C87E4: 7FB7F214  add r29, r23, r30
	ctx.r[29].u64 = ctx.r[23].u64 + ctx.r[30].u64;
	// 832C87E8: 938B0020  stw r28, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 832C87EC: 7CDEDA14  add r6, r30, r27
	ctx.r[6].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 832C87F0: 90AB00C0  stw r5, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[5].u32 ) };
	// 832C87F4: 7FFED850  subf r31, r30, r27
	ctx.r[31].s64 = ctx.r[27].s64 - ctx.r[30].s64;
	// 832C87F8: 7CBDC214  add r5, r29, r24
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[24].u64;
	// 832C87FC: 90CB0040  stw r6, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[6].u32 ) };
	// 832C8800: 7CDDC050  subf r6, r29, r24
	ctx.r[6].s64 = ctx.r[24].s64 - ctx.r[29].s64;
	// 832C8804: 93EB00A0  stw r31, 0xa0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[31].u32 ) };
	// 832C8808: 90AB0080  stw r5, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[5].u32 ) };
	pc = 0x832C880C; continue 'dispatch;
            }
            0x832C880C => {
    //   block [0x832C880C..0x832C8830)
	// 832C880C: 90CB0060  stw r6, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 832C8810: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832C8814: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 832C8818: 36D6FFFF  addic. r22, r22, -1
	ctx.xer.ca = (ctx.r[22].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 832C881C: 4181FE38  bgt 0x832c8654
	if ctx.cr[0].gt {
	pc = 0x832C8654; continue 'dispatch;
	}
	// 832C8820: 39230001  addi r9, r3, 1
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	// 832C8824: 39470001  addi r10, r7, 1
	ctx.r[10].s64 = ctx.r[7].s64 + 1;
	// 832C8828: 3961FE98  addi r11, r1, -0x168
	ctx.r[11].s64 = ctx.r[1].s64 + -360;
	// 832C882C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	pc = 0x832C8830; continue 'dispatch;
            }
            0x832C8830 => {
    //   block [0x832C8830..0x832C89A8)
	// 832C8830: 80EBFFFC  lwz r7, -4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 832C8834: 80CBFFF4  lwz r6, -0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 832C8838: 83CBFFEC  lwz r30, -0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) } as u64;
	// 832C883C: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C8840: 7F463850  subf r26, r6, r7
	ctx.r[26].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	// 832C8844: 7CA63A14  add r5, r6, r7
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 832C8848: 838BFFF8  lwz r28, -8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832C884C: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8850: 7F3FF050  subf r25, r31, r30
	ctx.r[25].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 832C8854: 83ABFFF0  lwz r29, -0x10(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) } as u64;
	// 832C8858: 7FFEFA14  add r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 832C885C: 836BFFE8  lwz r27, -0x18(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) } as u64;
	// 832C8860: 7F19D214  add r24, r25, r26
	ctx.r[24].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 832C8864: 7CFD3214  add r7, r29, r6
	ctx.r[7].u64 = ctx.r[29].u64 + ctx.r[6].u64;
	// 832C8868: 8AEAFFFF  lbz r23, -1(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-1 as u32) ) } as u64;
	// 832C886C: 7FDBE214  add r30, r27, r28
	ctx.r[30].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 832C8870: 7EC6E850  subf r22, r6, r29
	ctx.r[22].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	// 832C8874: 7CDF2A14  add r6, r31, r5
	ctx.r[6].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 832C8878: 7FA7F214  add r29, r7, r30
	ctx.r[29].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 832C887C: 7FE5F850  subf r31, r5, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[5].s64;
	// 832C8880: 1ED60B50  mulli r22, r22, 0xb50
	ctx.r[22].s32 = ((ctx.r[22].s32 as i64 * 2896 as i64) as i32);
	ctx.r[22].s64 = ctx.r[22].s32 as i64;
	// 832C8884: 1F180EC8  mulli r24, r24, 0xec8
	ctx.r[24].s32 = ((ctx.r[24].s32 as i64 * 3784 as i64) as i32);
	ctx.r[24].s64 = ctx.r[24].s32 as i64;
	// 832C8888: 7CA6EA14  add r5, r6, r29
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[29].u64;
	// 832C888C: 1F5AEB18  mulli r26, r26, -0x14e8
	ctx.r[26].s32 = ((ctx.r[26].s32 as i64 * -5352 as i64) as i32);
	ctx.r[26].s64 = ctx.r[26].s32 as i64;
	// 832C8890: 1EBF0B50  mulli r21, r31, 0xb50
	ctx.r[21].s32 = ((ctx.r[31].s32 as i64 * 2896 as i64) as i32);
	ctx.r[21].s64 = ctx.r[21].s32 as i64;
	// 832C8894: 7ED65E70  srawi r22, r22, 0xb
	ctx.xer.ca = (ctx.r[22].s32 < 0) && ((ctx.r[22].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[22].s64 = (ctx.r[22].s32 >> 11) as i64;
	// 832C8898: 7F1F5E70  srawi r31, r24, 0xb
	ctx.xer.ca = (ctx.r[24].s32 < 0) && ((ctx.r[24].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[24].s32 >> 11) as i64;
	// 832C889C: 1F3908A9  mulli r25, r25, 0x8a9
	ctx.r[25].s32 = ((ctx.r[25].s32 as i64 * 2217 as i64) as i32);
	ctx.r[25].s64 = ctx.r[25].s32 as i64;
	// 832C88A0: 38A5007F  addi r5, r5, 0x7f
	ctx.r[5].s64 = ctx.r[5].s64 + 127;
	// 832C88A4: 7F585E70  srawi r24, r26, 0xb
	ctx.xer.ca = (ctx.r[26].s32 < 0) && ((ctx.r[26].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[24].s64 = (ctx.r[26].s32 >> 11) as i64;
	// 832C88A8: 7EB55E70  srawi r21, r21, 0xb
	ctx.xer.ca = (ctx.r[21].s32 < 0) && ((ctx.r[21].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[21].s64 = (ctx.r[21].s32 >> 11) as i64;
	// 832C88AC: 7F345E70  srawi r20, r25, 0xb
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[20].s64 = (ctx.r[25].s32 >> 11) as i64;
	// 832C88B0: 7CB94670  srawi r25, r5, 8
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[5].s32 >> 8) as i64;
	// 832C88B4: 7CA6E850  subf r5, r6, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	// 832C88B8: 7FB9BA14  add r29, r25, r23
	ctx.r[29].u64 = ctx.r[25].u64 + ctx.r[23].u64;
	// 832C88BC: 7F7CD850  subf r27, r28, r27
	ctx.r[27].s64 = ctx.r[27].s64 - ctx.r[28].s64;
	// 832C88C0: 9BA9FFFF  stb r29, -1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-1 as u32), ctx.r[29].u8 ) };
	// 832C88C4: 7F87B050  subf r28, r7, r22
	ctx.r[28].s64 = ctx.r[22].s64 - ctx.r[7].s64;
	// 832C88C8: 7CC6C050  subf r6, r6, r24
	ctx.r[6].s64 = ctx.r[24].s64 - ctx.r[6].s64;
	// 832C88CC: 7FBCDA14  add r29, r28, r27
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 832C88D0: 7CC6FA14  add r6, r6, r31
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[31].u64;
	// 832C88D4: 3B25007F  addi r25, r5, 0x7f
	ctx.r[25].s64 = ctx.r[5].s64 + 127;
	// 832C88D8: 7CA6A850  subf r5, r6, r21
	ctx.r[5].s64 = ctx.r[21].s64 - ctx.r[6].s64;
	// 832C88DC: 7F46EA14  add r26, r6, r29
	ctx.r[26].u64 = ctx.r[6].u64 + ctx.r[29].u64;
	// 832C88E0: 7CC6E850  subf r6, r6, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	// 832C88E4: 7F394670  srawi r25, r25, 8
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[25].s32 >> 8) as i64;
	// 832C88E8: 3BA6007F  addi r29, r6, 0x7f
	ctx.r[29].s64 = ctx.r[6].s64 + 127;
	// 832C88EC: 7CDCD850  subf r6, r28, r27
	ctx.r[6].s64 = ctx.r[27].s64 - ctx.r[28].s64;
	// 832C88F0: 7F7FA050  subf r27, r31, r20
	ctx.r[27].s64 = ctx.r[20].s64 - ctx.r[31].s64;
	// 832C88F4: 7FE53214  add r31, r5, r6
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 832C88F8: 7F853050  subf r28, r5, r6
	ctx.r[28].s64 = ctx.r[6].s64 - ctx.r[5].s64;
	// 832C88FC: 7CC7F050  subf r6, r7, r30
	ctx.r[6].s64 = ctx.r[30].s64 - ctx.r[7].s64;
	// 832C8900: 7CFB2A14  add r7, r27, r5
	ctx.r[7].u64 = ctx.r[27].u64 + ctx.r[5].u64;
	// 832C8904: 38BA007F  addi r5, r26, 0x7f
	ctx.r[5].s64 = ctx.r[26].s64 + 127;
	// 832C8908: 3BFF007F  addi r31, r31, 0x7f
	ctx.r[31].s64 = ctx.r[31].s64 + 127;
	// 832C890C: 7CBA4670  srawi r26, r5, 8
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[5].s32 >> 8) as i64;
	// 832C8910: 7FBD4670  srawi r29, r29, 8
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 8) as i64;
	// 832C8914: 88AA0006  lbz r5, 6(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 832C8918: 7FFF4670  srawi r31, r31, 8
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[31].s32 >> 8) as i64;
	// 832C891C: 3BDC007F  addi r30, r28, 0x7f
	ctx.r[30].s64 = ctx.r[28].s64 + 127;
	// 832C8920: 7CB92A14  add r5, r25, r5
	ctx.r[5].u64 = ctx.r[25].u64 + ctx.r[5].u64;
	// 832C8924: 98A90006  stb r5, 6(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(6 as u32), ctx.r[5].u8 ) };
	// 832C8928: 88AA0000  lbz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C892C: 7CBA2A14  add r5, r26, r5
	ctx.r[5].u64 = ctx.r[26].u64 + ctx.r[5].u64;
	// 832C8930: 98A90000  stb r5, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 832C8934: 88AA0005  lbz r5, 5(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 832C8938: 7CBD2A14  add r5, r29, r5
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[5].u64;
	// 832C893C: 98A90005  stb r5, 5(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(5 as u32), ctx.r[5].u8 ) };
	// 832C8940: 88AA0001  lbz r5, 1(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 832C8944: 7CBF2A14  add r5, r31, r5
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 832C8948: 98A90001  stb r5, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[5].u8 ) };
	// 832C894C: 7FC54670  srawi r5, r30, 8
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[30].s32 >> 8) as i64;
	// 832C8950: 8BEA0004  lbz r31, 4(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C8954: 7FC73050  subf r30, r7, r6
	ctx.r[30].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 832C8958: 7CA5FA14  add r5, r5, r31
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[31].u64;
	// 832C895C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 832C8960: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 832C8964: 3BDE007F  addi r30, r30, 0x7f
	ctx.r[30].s64 = ctx.r[30].s64 + 127;
	// 832C8968: 98C90004  stb r6, 4(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[6].u8 ) };
	// 832C896C: 38E7007F  addi r7, r7, 0x7f
	ctx.r[7].s64 = ctx.r[7].s64 + 127;
	// 832C8970: 7FC54670  srawi r5, r30, 8
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[30].s32 >> 8) as i64;
	// 832C8974: 88CA0002  lbz r6, 2(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 832C8978: 7CE74670  srawi r7, r7, 8
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 8) as i64;
	// 832C897C: 3463FFFF  addic. r3, r3, -1
	ctx.xer.ca = (ctx.r[3].u32 > (!(-1 as u32)));
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832C8980: 7CA53214  add r5, r5, r6
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 832C8984: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C8988: 98A90002  stb r5, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[5].u8 ) };
	// 832C898C: 88CA0003  lbz r6, 3(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 832C8990: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 832C8994: 98E90003  stb r7, 3(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 832C8998: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 832C899C: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 832C89A0: 4082FE90  bne 0x832c8830
	if !ctx.cr[0].eq {
	pc = 0x832C8830; continue 'dispatch;
	}
	// 832C89A4: 4B9E0A88  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C89A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C89A8 size=2784
    //   switch @ 0x832C8B20: r9 with 4 label(s)
    //       case  0 → 0x832C8B34
    //       case  1 → 0x832C8B48
    //       case  2 → 0x832C8B84
    //       case  3 → 0x832C8EE0
    let mut pc: u32 = 0x832C89A8;
    'dispatch: loop {
        match pc {
            0x832C89A8 => {
    //   block [0x832C89A8..0x832C8B34)
	// 832C89A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C89AC: 4B9E0A25  bl 0x82ca93d0
	ctx.lr = 0x832C89B0;
	sub_82CA93D0(ctx, base);
	// 832C89B0: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 832C89B4: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 832C89B8: 3921FE60  addi r9, r1, -0x1a0
	ctx.r[9].s64 = ctx.r[1].s64 + -416;
	// 832C89BC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C89C0: 3901FE70  addi r8, r1, -0x190
	ctx.r[8].s64 = ctx.r[1].s64 + -400;
	// 832C89C4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C89C8: 38E1FE80  addi r7, r1, -0x180
	ctx.r[7].s64 = ctx.r[1].s64 + -384;
	// 832C89CC: 80C40004  lwz r6, 4(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C89D0: 38A1FE90  addi r5, r1, -0x170
	ctx.r[5].s64 = ctx.r[1].s64 + -368;
	// 832C89D4: 3861FEA0  addi r3, r1, -0x160
	ctx.r[3].s64 = ctx.r[1].s64 + -352;
	// 832C89D8: 3BE1FEB0  addi r31, r1, -0x150
	ctx.r[31].s64 = ctx.r[1].s64 + -336;
	// 832C89DC: 3BC1FEC0  addi r30, r1, -0x140
	ctx.r[30].s64 = ctx.r[1].s64 + -320;
	// 832C89E0: 3BA1FED0  addi r29, r1, -0x130
	ctx.r[29].s64 = ctx.r[1].s64 + -304;
	pc = 0x832C8B34; continue 'dispatch;
            }
            0x832C8B34 => {
    //   block [0x832C8B34..0x832C8B48)
	// 832C8B34: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C8B38: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8B3C: 39290011  addi r9, r9, 0x11
	ctx.r[9].s64 = ctx.r[9].s64 + 17;
	// 832C8B40: 99230000  stb r9, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8B44: 4800004C  b 0x832c8b90
	pc = 0x832C8B90; continue 'dispatch;
            }
            0x832C8B48 => {
    //   block [0x832C8B48..0x832C8B84)
	// 832C8B48: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 832C8B4C: 39190001  addi r8, r25, 1
	ctx.r[8].s64 = ctx.r[25].s64 + 1;
	// 832C8B50: 5529003A  rlwinm r9, r9, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 832C8B54: 3AC90002  addi r22, r9, 2
	ctx.r[22].s64 = ctx.r[9].s64 + 2;
	// 832C8B58: 3AA90012  addi r21, r9, 0x12
	ctx.r[21].s64 = ctx.r[9].s64 + 18;
	// 832C8B5C: 3A890022  addi r20, r9, 0x22
	ctx.r[20].s64 = ctx.r[9].s64 + 34;
	// 832C8B60: 9AC30000  stb r22, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[22].u8 ) };
	// 832C8B64: 39290032  addi r9, r9, 0x32
	ctx.r[9].s64 = ctx.r[9].s64 + 50;
	// 832C8B68: 9AB90000  stb r21, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[21].u8 ) };
	// 832C8B6C: 9A990001  stb r20, 1(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(1 as u32), ctx.r[20].u8 ) };
	// 832C8B70: 7D364B78  mr r22, r9
	ctx.r[22].u64 = ctx.r[9].u64;
	// 832C8B74: 39280001  addi r9, r8, 1
	ctx.r[9].s64 = ctx.r[8].s64 + 1;
	// 832C8B78: 9AC80001  stb r22, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[22].u8 ) };
	// 832C8B7C: 3B290001  addi r25, r9, 1
	ctx.r[25].s64 = ctx.r[9].s64 + 1;
	// 832C8B80: 480003D4  b 0x832c8f54
	pc = 0x832C8F54; continue 'dispatch;
            }
            0x832C8B84 => {
    //   block [0x832C8B84..0x832C8EE0)
	// 832C8B84: 9AE30000  stb r23, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u8 ) };
	// 832C8B88: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C8B8C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 832C8B90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C8B94: 409A0158  bne cr6, 0x832c8cec
	if !ctx.cr[6].eq {
	pc = 0x832C8CEC; continue 'dispatch;
	}
	// 832C8B98: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8B9C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8BA0: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 832C8BA4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C8BA8: 409A0128  bne cr6, 0x832c8cd0
	if !ctx.cr[6].eq {
	pc = 0x832C8CD0; continue 'dispatch;
	}
	// 832C8BAC: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8BB0: 7D6A2C30  srw r10, r11, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8BB4: 2167001F  subfic r11, r7, 0x1f
	ctx.xer.ca = ctx.r[7].u32 <= 31 as u32;
	ctx.r[11].s64 = (31 as i64) - ctx.r[7].s64;
	// 832C8BB8: 7D36F838  and r22, r9, r31
	ctx.r[22].u64 = ctx.r[9].u64 & ctx.r[31].u64;
	// 832C8BBC: 7D29F038  and r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 832C8BC0: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 832C8BC4: 7D29EB78  or r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[29].u64;
	// 832C8BC8: 419A0008  beq cr6, 0x832c8bd0
	if ctx.cr[6].eq {
	pc = 0x832C8BD0; continue 'dispatch;
	}
	// 832C8BCC: 7D2900D0  neg r9, r9
	ctx.r[9].s64 = -ctx.r[9].s64;
	// 832C8BD0: 5516083C  slwi r22, r8, 1
	ctx.r[22].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 832C8BD4: 3AA1FE60  addi r21, r1, -0x1a0
	ctx.r[21].s64 = ctx.r[1].s64 + -416;
	// 832C8BD8: 7D36AB2E  sthx r9, r22, r21
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32), ctx.r[9].u16) };
	// 832C8BDC: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C8BE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C8BE4: 409A018C  bne cr6, 0x832c8d70
	if !ctx.cr[6].eq {
	pc = 0x832C8D70; continue 'dispatch;
	}
	// 832C8BE8: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8BEC: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8BF0: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 832C8BF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C8BF8: 409A015C  bne cr6, 0x832c8d54
	if !ctx.cr[6].eq {
	pc = 0x832C8D54; continue 'dispatch;
	}
	// 832C8BFC: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8C00: 7D6A2C30  srw r10, r11, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8C04: 2167001F  subfic r11, r7, 0x1f
	ctx.xer.ca = ctx.r[7].u32 <= 31 as u32;
	ctx.r[11].s64 = (31 as i64) - ctx.r[7].s64;
	// 832C8C08: 7D36F838  and r22, r9, r31
	ctx.r[22].u64 = ctx.r[9].u64 & ctx.r[31].u64;
	// 832C8C0C: 7D29F038  and r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 832C8C10: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 832C8C14: 7D29EB78  or r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[29].u64;
	// 832C8C18: 419A0008  beq cr6, 0x832c8c20
	if ctx.cr[6].eq {
	pc = 0x832C8C20; continue 'dispatch;
	}
	// 832C8C1C: 7D2900D0  neg r9, r9
	ctx.r[9].s64 = -ctx.r[9].s64;
	// 832C8C20: 5516083C  slwi r22, r8, 1
	ctx.r[22].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 832C8C24: 3AA1FE60  addi r21, r1, -0x1a0
	ctx.r[21].s64 = ctx.r[1].s64 + -416;
	// 832C8C28: 7D36AB2E  sthx r9, r22, r21
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32), ctx.r[9].u16) };
	// 832C8C2C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C8C30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C8C34: 409A01C0  bne cr6, 0x832c8df4
	if !ctx.cr[6].eq {
	pc = 0x832C8DF4; continue 'dispatch;
	}
	// 832C8C38: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8C3C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8C40: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 832C8C44: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C8C48: 409A0190  bne cr6, 0x832c8dd8
	if !ctx.cr[6].eq {
	pc = 0x832C8DD8; continue 'dispatch;
	}
	// 832C8C4C: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8C50: 7D6A2C30  srw r10, r11, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8C54: 2167001F  subfic r11, r7, 0x1f
	ctx.xer.ca = ctx.r[7].u32 <= 31 as u32;
	ctx.r[11].s64 = (31 as i64) - ctx.r[7].s64;
	// 832C8C58: 7D36F838  and r22, r9, r31
	ctx.r[22].u64 = ctx.r[9].u64 & ctx.r[31].u64;
	// 832C8C5C: 7D29F038  and r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 832C8C60: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 832C8C64: 7D29EB78  or r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[29].u64;
	// 832C8C68: 419A0008  beq cr6, 0x832c8c70
	if ctx.cr[6].eq {
	pc = 0x832C8C70; continue 'dispatch;
	}
	// 832C8C6C: 7D2900D0  neg r9, r9
	ctx.r[9].s64 = -ctx.r[9].s64;
	// 832C8C70: 5516083C  slwi r22, r8, 1
	ctx.r[22].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 832C8C74: 3AA1FE60  addi r21, r1, -0x1a0
	ctx.r[21].s64 = ctx.r[1].s64 + -416;
	// 832C8C78: 7D36AB2E  sthx r9, r22, r21
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32), ctx.r[9].u16) };
	// 832C8C7C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C8C80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C8C84: 409A01F4  bne cr6, 0x832c8e78
	if !ctx.cr[6].eq {
	pc = 0x832C8E78; continue 'dispatch;
	}
	// 832C8C88: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8C8C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8C90: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 832C8C94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C8C98: 409A01C4  bne cr6, 0x832c8e5c
	if !ctx.cr[6].eq {
	pc = 0x832C8E5C; continue 'dispatch;
	}
	// 832C8C9C: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8CA0: 7D6A2C30  srw r10, r11, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8CA4: 2167001F  subfic r11, r7, 0x1f
	ctx.xer.ca = ctx.r[7].u32 <= 31 as u32;
	ctx.r[11].s64 = (31 as i64) - ctx.r[7].s64;
	// 832C8CA8: 7D36F838  and r22, r9, r31
	ctx.r[22].u64 = ctx.r[9].u64 & ctx.r[31].u64;
	// 832C8CAC: 7D29F038  and r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 832C8CB0: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 832C8CB4: 7D29EB78  or r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[29].u64;
	// 832C8CB8: 419A0008  beq cr6, 0x832c8cc0
	if ctx.cr[6].eq {
	pc = 0x832C8CC0; continue 'dispatch;
	}
	// 832C8CBC: 7D2900D0  neg r9, r9
	ctx.r[9].s64 = -ctx.r[9].s64;
	// 832C8CC0: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C8CC4: 3AC1FE60  addi r22, r1, -0x1a0
	ctx.r[22].s64 = ctx.r[1].s64 + -416;
	// 832C8CC8: 7D28B32E  sthx r9, r8, r22
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[22].u32), ctx.r[9].u16) };
	// 832C8CCC: 48000288  b 0x832c8f54
	pc = 0x832C8F54; continue 'dispatch;
	// 832C8CD0: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8CD4: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C8CD8: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C8CDC: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8CE0: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C8CE4: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8CE8: 4BFFFEF4  b 0x832c8bdc
	pc = 0x832C8BDC; continue 'dispatch;
	// 832C8CEC: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C8CF0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832C8CF4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C8CF8: 409A0044  bne cr6, 0x832c8d3c
	if !ctx.cr[6].eq {
	pc = 0x832C8D3C; continue 'dispatch;
	}
	// 832C8CFC: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C8D00: 4098002C  bge cr6, 0x832c8d2c
	if !ctx.cr[6].lt {
	pc = 0x832C8D2C; continue 'dispatch;
	}
	// 832C8D04: 82C60000  lwz r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8D08: 7EAB3850  subf r21, r11, r7
	ctx.r[21].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 832C8D0C: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8D10: 7EC95830  slw r9, r22, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[22].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8D14: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8D18: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 832C8D1C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8D20: 7ECAAC30  srw r10, r22, r21
	if (ctx.r[21].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[22].u32) >> ((ctx.r[21].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8D24: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C8D28: 4BFFFE90  b 0x832c8bb8
	pc = 0x832C8BB8; continue 'dispatch;
	// 832C8D2C: 5549F87E  srwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8D30: 7D4A2C30  srw r10, r10, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8D34: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8D38: 4BFFFE80  b 0x832c8bb8
	pc = 0x832C8BB8; continue 'dispatch;
	// 832C8D3C: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8D40: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C8D44: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C8D48: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8D4C: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8D50: 4BFFFE8C  b 0x832c8bdc
	pc = 0x832C8BDC; continue 'dispatch;
	// 832C8D54: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8D58: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C8D5C: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C8D60: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8D64: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C8D68: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8D6C: 4BFFFEC0  b 0x832c8c2c
	pc = 0x832C8C2C; continue 'dispatch;
	// 832C8D70: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C8D74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832C8D78: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C8D7C: 409A0044  bne cr6, 0x832c8dc0
	if !ctx.cr[6].eq {
	pc = 0x832C8DC0; continue 'dispatch;
	}
	// 832C8D80: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C8D84: 4098002C  bge cr6, 0x832c8db0
	if !ctx.cr[6].lt {
	pc = 0x832C8DB0; continue 'dispatch;
	}
	// 832C8D88: 82C60000  lwz r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8D8C: 7EAB3850  subf r21, r11, r7
	ctx.r[21].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 832C8D90: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8D94: 7EC95830  slw r9, r22, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[22].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8D98: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8D9C: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 832C8DA0: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8DA4: 7ECAAC30  srw r10, r22, r21
	if (ctx.r[21].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[22].u32) >> ((ctx.r[21].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8DA8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C8DAC: 4BFFFE5C  b 0x832c8c08
	pc = 0x832C8C08; continue 'dispatch;
	// 832C8DB0: 5549F87E  srwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8DB4: 7D4A2C30  srw r10, r10, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8DB8: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8DBC: 4BFFFE4C  b 0x832c8c08
	pc = 0x832C8C08; continue 'dispatch;
	// 832C8DC0: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8DC4: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C8DC8: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C8DCC: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8DD0: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8DD4: 4BFFFE58  b 0x832c8c2c
	pc = 0x832C8C2C; continue 'dispatch;
	// 832C8DD8: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8DDC: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C8DE0: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C8DE4: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8DE8: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C8DEC: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8DF0: 4BFFFE8C  b 0x832c8c7c
	pc = 0x832C8C7C; continue 'dispatch;
	// 832C8DF4: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C8DF8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832C8DFC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C8E00: 409A0044  bne cr6, 0x832c8e44
	if !ctx.cr[6].eq {
	pc = 0x832C8E44; continue 'dispatch;
	}
	// 832C8E04: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C8E08: 4098002C  bge cr6, 0x832c8e34
	if !ctx.cr[6].lt {
	pc = 0x832C8E34; continue 'dispatch;
	}
	// 832C8E0C: 82C60000  lwz r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8E10: 7EAB3850  subf r21, r11, r7
	ctx.r[21].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 832C8E14: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8E18: 7EC95830  slw r9, r22, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[22].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8E1C: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8E20: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 832C8E24: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8E28: 7ECAAC30  srw r10, r22, r21
	if (ctx.r[21].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[22].u32) >> ((ctx.r[21].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8E2C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C8E30: 4BFFFE28  b 0x832c8c58
	pc = 0x832C8C58; continue 'dispatch;
	// 832C8E34: 5549F87E  srwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8E38: 7D4A2C30  srw r10, r10, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8E3C: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8E40: 4BFFFE18  b 0x832c8c58
	pc = 0x832C8C58; continue 'dispatch;
	// 832C8E44: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8E48: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C8E4C: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C8E50: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8E54: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8E58: 4BFFFE24  b 0x832c8c7c
	pc = 0x832C8C7C; continue 'dispatch;
	// 832C8E5C: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8E60: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C8E64: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C8E68: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8E6C: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C8E70: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8E74: 480000E0  b 0x832c8f54
	pc = 0x832C8F54; continue 'dispatch;
	// 832C8E78: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C8E7C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832C8E80: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C8E84: 409A0044  bne cr6, 0x832c8ec8
	if !ctx.cr[6].eq {
	pc = 0x832C8EC8; continue 'dispatch;
	}
	// 832C8E88: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C8E8C: 4098002C  bge cr6, 0x832c8eb8
	if !ctx.cr[6].lt {
	pc = 0x832C8EB8; continue 'dispatch;
	}
	// 832C8E90: 82C60000  lwz r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8E94: 7EAB3850  subf r21, r11, r7
	ctx.r[21].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 832C8E98: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8E9C: 7EC95830  slw r9, r22, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[22].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8EA0: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8EA4: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 832C8EA8: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8EAC: 7ECAAC30  srw r10, r22, r21
	if (ctx.r[21].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[22].u32) >> ((ctx.r[21].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8EB0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C8EB4: 4BFFFDF4  b 0x832c8ca8
	pc = 0x832C8CA8; continue 'dispatch;
	// 832C8EB8: 5549F87E  srwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8EBC: 7D4A2C30  srw r10, r10, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8EC0: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8EC4: 4BFFFDE4  b 0x832c8ca8
	pc = 0x832C8CA8; continue 'dispatch;
	// 832C8EC8: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C8ECC: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C8ED0: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C8ED4: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8ED8: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C8EDC: 48000078  b 0x832c8f54
	pc = 0x832C8F54; continue 'dispatch;
            }
            0x832C8EE0 => {
    //   block [0x832C8EE0..0x832C8FFC)
	// 832C8EE0: 21270020  subfic r9, r7, 0x20
	ctx.xer.ca = ctx.r[7].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[7].s64;
	// 832C8EE4: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C8EE8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832C8EEC: 7F894C30  srw r9, r28, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[28].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8EF0: 4098002C  bge cr6, 0x832c8f1c
	if !ctx.cr[6].lt {
	pc = 0x832C8F1C; continue 'dispatch;
	}
	// 832C8EF4: 82C60000  lwz r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8EF8: 7EAB3850  subf r21, r11, r7
	ctx.r[21].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 832C8EFC: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8F00: 7ED45830  slw r20, r22, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[20].u64 = 0;
	} else {
		ctx.r[20].u64 = ((ctx.r[22].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8F04: 7E8A5378  or r10, r20, r10
	ctx.r[10].u64 = ctx.r[20].u64 | ctx.r[10].u64;
	// 832C8F08: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8F0C: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 832C8F10: 7ECAAC30  srw r10, r22, r21
	if (ctx.r[21].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[22].u32) >> ((ctx.r[21].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8F14: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 832C8F18: 48000010  b 0x832c8f28
	pc = 0x832C8F28; continue 'dispatch;
	// 832C8F1C: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 832C8F20: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 832C8F24: 7D4A3C30  srw r10, r10, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 832C8F28: 7D36F838  and r22, r9, r31
	ctx.r[22].u64 = ctx.r[9].u64 & ctx.r[31].u64;
	// 832C8F2C: 7D29F038  and r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 832C8F30: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 832C8F34: 7D29EB78  or r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[29].u64;
	// 832C8F38: 419A0008  beq cr6, 0x832c8f40
	if ctx.cr[6].eq {
	pc = 0x832C8F40; continue 'dispatch;
	}
	// 832C8F3C: 7D2900D0  neg r9, r9
	ctx.r[9].s64 = -ctx.r[9].s64;
	// 832C8F40: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C8F44: 9AE30000  stb r23, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u8 ) };
	// 832C8F48: 3AC1FE60  addi r22, r1, -0x1a0
	ctx.r[22].s64 = ctx.r[1].s64 + -416;
	// 832C8F4C: 7D28B32E  sthx r9, r8, r22
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[22].u32), ctx.r[9].u16) };
	// 832C8F50: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 832C8F54: 7F03C840  cmplw cr6, r3, r25
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[25].u32, &mut ctx.xer);
	// 832C8F58: 4198FB68  blt cr6, 0x832c8ac0
	if ctx.cr[6].lt {
	pc = 0x832C8AC0; continue 'dispatch;
	}
	// 832C8F5C: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 832C8F60: 7FBD0E70  srawi r29, r29, 1
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[29].s32 >> 1) as i64;
	// 832C8F64: 2B070001  cmplwi cr6, r7, 1
	ctx.cr[6].compare_u32(ctx.r[7].u32, 1 as u32, &mut ctx.xer);
	// 832C8F68: 4199FB3C  bgt cr6, 0x832c8aa4
	if ctx.cr[6].gt {
	pc = 0x832C8AA4; continue 'dispatch;
	}
	// 832C8F6C: 9141FE34  stw r10, -0x1cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-460 as u32), ctx.r[10].u32 ) };
	// 832C8F70: 9161FE30  stw r11, -0x1d0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-464 as u32), ctx.r[11].u32 ) };
	// 832C8F74: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 832C8F78: 419A036C  beq cr6, 0x832c92e4
	if ctx.cr[6].eq {
	pc = 0x832C92E4; continue 'dispatch;
	}
	// 832C8F7C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 832C8F80: 7F1AC840  cmplw cr6, r26, r25
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[25].u32, &mut ctx.xer);
	// 832C8F84: 40980360  bge cr6, 0x832c92e4
	if !ctx.cr[6].lt {
	pc = 0x832C92E4; continue 'dispatch;
	}
	// 832C8F88: 89050000  lbz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8F8C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 832C8F90: 419A0340  beq cr6, 0x832c92d0
	if ctx.cr[6].eq {
	pc = 0x832C92D0; continue 'dispatch;
	}
	// 832C8F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C8F98: 409A0018  bne cr6, 0x832c8fb0
	if !ctx.cr[6].eq {
	pc = 0x832C8FB0; continue 'dispatch;
	}
	// 832C8F9C: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C8FA0: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C8FA4: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C8FA8: 552AF87E  srwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8FAC: 48000010  b 0x832c8fbc
	pc = 0x832C8FBC; continue 'dispatch;
	// 832C8FB0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 832C8FB4: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C8FB8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832C8FBC: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 832C8FC0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C8FC4: 419A030C  beq cr6, 0x832c92d0
	if ctx.cr[6].eq {
	pc = 0x832C92D0; continue 'dispatch;
	}
	// 832C8FC8: 550907BE  clrlwi r9, r8, 0x1e
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 832C8FCC: 2B090003  cmplwi cr6, r9, 3
	ctx.cr[6].compare_u32(ctx.r[9].u32, 3 as u32, &mut ctx.xer);
	// 832C8FD0: 41990300  bgt cr6, 0x832c92d0
	if ctx.cr[6].gt {
	pc = 0x832C92D0; continue 'dispatch;
	}
	// 832C8FD4: 3D80832D  lis r12, -0x7cd3
	ctx.r[12].s64 = -2094202880;
	// 832C8FD8: 398C8FEC  addi r12, r12, -0x7014
	ctx.r[12].s64 = ctx.r[12].s64 + -28692;
	// 832C8FDC: 5520103A  slwi r0, r9, 2
	ctx.r[0].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 832C8FE0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 832C8FE4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 832C8FE8: 4E800420  bctr
	match ctx.r[9].u64 {
		0 => {
	pc = 0x832C8FFC; continue 'dispatch;
		},
		1 => {
	pc = 0x832C9010; continue 'dispatch;
		},
		2 => {
	pc = 0x832C904C; continue 'dispatch;
		},
		3 => {
	pc = 0x832C9288; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 832C8FEC: 832C8FFC  lwz r25, -0x7004(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-28676 as u32) ) } as u64;
	// 832C8FF0: 832C9010  lwz r25, -0x6ff0(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-28656 as u32) ) } as u64;
	// 832C8FF4: 832C904C  lwz r25, -0x6fb4(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-28596 as u32) ) } as u64;
	// 832C8FF8: 832C9288  lwz r25, -0x6d78(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-28024 as u32) ) } as u64;
            }
            0x832C8FFC => {
    //   block [0x832C8FFC..0x832C9010)
	// 832C8FFC: 5507F0BE  srwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9000: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C9004: 39290011  addi r9, r9, 0x11
	ctx.r[9].s64 = ctx.r[9].s64 + 17;
	// 832C9008: 99250000  stb r9, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C900C: 4800004C  b 0x832c9058
	pc = 0x832C9058; continue 'dispatch;
            }
            0x832C9010 => {
    //   block [0x832C9010..0x832C904C)
	// 832C9010: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 832C9014: 39190001  addi r8, r25, 1
	ctx.r[8].s64 = ctx.r[25].s64 + 1;
	// 832C9018: 5529003A  rlwinm r9, r9, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 832C901C: 38E90002  addi r7, r9, 2
	ctx.r[7].s64 = ctx.r[9].s64 + 2;
	// 832C9020: 38690012  addi r3, r9, 0x12
	ctx.r[3].s64 = ctx.r[9].s64 + 18;
	// 832C9024: 3BE90022  addi r31, r9, 0x22
	ctx.r[31].s64 = ctx.r[9].s64 + 34;
	// 832C9028: 98E50000  stb r7, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 832C902C: 39290032  addi r9, r9, 0x32
	ctx.r[9].s64 = ctx.r[9].s64 + 50;
	// 832C9030: 98790000  stb r3, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 832C9034: 9BF90001  stb r31, 1(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(1 as u32), ctx.r[31].u8 ) };
	// 832C9038: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 832C903C: 39280001  addi r9, r8, 1
	ctx.r[9].s64 = ctx.r[8].s64 + 1;
	// 832C9040: 98E80001  stb r7, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 832C9044: 3B290001  addi r25, r9, 1
	ctx.r[25].s64 = ctx.r[9].s64 + 1;
	// 832C9048: 4800028C  b 0x832c92d4
	pc = 0x832C92D4; continue 'dispatch;
            }
            0x832C904C => {
    //   block [0x832C904C..0x832C9288)
	// 832C904C: 9AE50000  stb r23, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[23].u8 ) };
	// 832C9050: 5507F0BE  srwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9054: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 832C9058: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C905C: 409A0018  bne cr6, 0x832c9074
	if !ctx.cr[6].eq {
	pc = 0x832C9074; continue 'dispatch;
	}
	// 832C9060: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9064: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C9068: 3920001F  li r9, 0x1f
	ctx.r[9].s64 = 31;
	// 832C906C: 550AF87E  srwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C9070: 48000010  b 0x832c9080
	pc = 0x832C9080; continue 'dispatch;
	// 832C9074: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 832C9078: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C907C: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 832C9080: 550B07FE  clrlwi r11, r8, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 832C9084: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C9088: 419A0018  beq cr6, 0x832c90a0
	if ctx.cr[6].eq {
	pc = 0x832C90A0; continue 'dispatch;
	}
	// 832C908C: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C9090: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C9094: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 832C9098: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 832C909C: 48000044  b 0x832c90e0
	pc = 0x832C90E0; continue 'dispatch;
	// 832C90A0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C90A4: 409A0018  bne cr6, 0x832c90bc
	if !ctx.cr[6].eq {
	pc = 0x832C90BC; continue 'dispatch;
	}
	// 832C90A8: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C90AC: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C90B0: 3920001F  li r9, 0x1f
	ctx.r[9].s64 = 31;
	// 832C90B4: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C90B8: 48000010  b 0x832c90c8
	pc = 0x832C90C8; continue 'dispatch;
	// 832C90BC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 832C90C0: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C90C4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 832C90C8: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 832C90CC: 54E8083C  slwi r8, r7, 1
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C90D0: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 832C90D4: 3961FE60  addi r11, r1, -0x1a0
	ctx.r[11].s64 = ctx.r[1].s64 + -416;
	// 832C90D8: 530307DE  rlwimi r3, r24, 0, 0x1f, 0xf
	ctx.r[3].u64 = (((ctx.r[24].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFF0001) | (ctx.r[3].u64 & 0x000000000000FFFE);
	// 832C90DC: 7C685B2E  sthx r3, r8, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u16) };
	// 832C90E0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 832C90E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C90E8: 409A0018  bne cr6, 0x832c9100
	if !ctx.cr[6].eq {
	pc = 0x832C9100; continue 'dispatch;
	}
	// 832C90EC: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C90F0: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C90F4: 3940001F  li r10, 0x1f
	ctx.r[10].s64 = 31;
	// 832C90F8: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C90FC: 48000010  b 0x832c910c
	pc = 0x832C910C; continue 'dispatch;
	// 832C9100: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 832C9104: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C9108: 3949FFFF  addi r10, r9, -1
	ctx.r[10].s64 = ctx.r[9].s64 + -1;
	// 832C910C: 550907FE  clrlwi r9, r8, 0x1f
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 832C9110: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C9114: 419A0018  beq cr6, 0x832c912c
	if ctx.cr[6].eq {
	pc = 0x832C912C; continue 'dispatch;
	}
	// 832C9118: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C911C: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C9120: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C9124: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C9128: 48000044  b 0x832c916c
	pc = 0x832C916C; continue 'dispatch;
	// 832C912C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C9130: 409A0018  bne cr6, 0x832c9148
	if !ctx.cr[6].eq {
	pc = 0x832C9148; continue 'dispatch;
	}
	// 832C9134: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9138: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C913C: 3940001F  li r10, 0x1f
	ctx.r[10].s64 = 31;
	// 832C9140: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C9144: 48000010  b 0x832c9154
	pc = 0x832C9154; continue 'dispatch;
	// 832C9148: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 832C914C: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C9150: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 832C9154: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 832C9158: 54E8083C  slwi r8, r7, 1
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C915C: 7C6900D0  neg r3, r9
	ctx.r[3].s64 = -ctx.r[9].s64;
	// 832C9160: 3921FE60  addi r9, r1, -0x1a0
	ctx.r[9].s64 = ctx.r[1].s64 + -416;
	// 832C9164: 530307DE  rlwimi r3, r24, 0, 0x1f, 0xf
	ctx.r[3].u64 = (((ctx.r[24].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFF0001) | (ctx.r[3].u64 & 0x000000000000FFFE);
	// 832C9168: 7C684B2E  sthx r3, r8, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[3].u16) };
	// 832C916C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 832C9170: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C9174: 409A0018  bne cr6, 0x832c918c
	if !ctx.cr[6].eq {
	pc = 0x832C918C; continue 'dispatch;
	}
	// 832C9178: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C917C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C9180: 3920001F  li r9, 0x1f
	ctx.r[9].s64 = 31;
	// 832C9184: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C9188: 48000010  b 0x832c9198
	pc = 0x832C9198; continue 'dispatch;
	// 832C918C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 832C9190: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C9194: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 832C9198: 550A07FE  clrlwi r10, r8, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 832C919C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C91A0: 419A0018  beq cr6, 0x832c91b8
	if ctx.cr[6].eq {
	pc = 0x832C91B8; continue 'dispatch;
	}
	// 832C91A4: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C91A8: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C91AC: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 832C91B0: 995A0000  stb r10, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 832C91B4: 48000044  b 0x832c91f8
	pc = 0x832C91F8; continue 'dispatch;
	// 832C91B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C91BC: 409A0018  bne cr6, 0x832c91d4
	if !ctx.cr[6].eq {
	pc = 0x832C91D4; continue 'dispatch;
	}
	// 832C91C0: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C91C4: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C91C8: 3920001F  li r9, 0x1f
	ctx.r[9].s64 = 31;
	// 832C91CC: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C91D0: 48000010  b 0x832c91e0
	pc = 0x832C91E0; continue 'dispatch;
	// 832C91D4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 832C91D8: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832C91DC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 832C91E0: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C91E4: 54E8083C  slwi r8, r7, 1
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C91E8: 7C6A00D0  neg r3, r10
	ctx.r[3].s64 = -ctx.r[10].s64;
	// 832C91EC: 3941FE60  addi r10, r1, -0x1a0
	ctx.r[10].s64 = ctx.r[1].s64 + -416;
	// 832C91F0: 530307DE  rlwimi r3, r24, 0, 0x1f, 0xf
	ctx.r[3].u64 = (((ctx.r[24].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFF0001) | (ctx.r[3].u64 & 0x000000000000FFFE);
	// 832C91F4: 7C68532E  sthx r3, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u16) };
	// 832C91F8: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 832C91FC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C9200: 409A0018  bne cr6, 0x832c9218
	if !ctx.cr[6].eq {
	pc = 0x832C9218; continue 'dispatch;
	}
	// 832C9204: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9208: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C920C: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C9210: 550AF87E  srwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C9214: 48000010  b 0x832c9224
	pc = 0x832C9224; continue 'dispatch;
	// 832C9218: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 832C921C: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C9220: 3969FFFF  addi r11, r9, -1
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	// 832C9224: 550907FE  clrlwi r9, r8, 0x1f
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 832C9228: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C922C: 419A0018  beq cr6, 0x832c9244
	if ctx.cr[6].eq {
	pc = 0x832C9244; continue 'dispatch;
	}
	// 832C9230: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C9234: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 832C9238: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 832C923C: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C9240: 48000094  b 0x832c92d4
	pc = 0x832C92D4; continue 'dispatch;
	// 832C9244: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C9248: 409A0018  bne cr6, 0x832c9260
	if !ctx.cr[6].eq {
	pc = 0x832C9260; continue 'dispatch;
	}
	// 832C924C: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9250: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C9254: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C9258: 552AF87E  srwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C925C: 48000010  b 0x832c926c
	pc = 0x832C926C; continue 'dispatch;
	// 832C9260: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 832C9264: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C9268: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832C926C: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 832C9270: 54E8083C  slwi r8, r7, 1
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C9274: 7CE900D0  neg r7, r9
	ctx.r[7].s64 = -ctx.r[9].s64;
	// 832C9278: 3861FE60  addi r3, r1, -0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + -416;
	// 832C927C: 530707DE  rlwimi r7, r24, 0, 0x1f, 0xf
	ctx.r[7].u64 = (((ctx.r[24].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFF0001) | (ctx.r[7].u64 & 0x000000000000FFFE);
	// 832C9280: 7CE81B2E  sthx r7, r8, r3
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[7].u16) };
	// 832C9284: 48000050  b 0x832c92d4
	pc = 0x832C92D4; continue 'dispatch;
            }
            0x832C9288 => {
    //   block [0x832C9288..0x832C9488)
	// 832C9288: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C928C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C9290: 409A0018  bne cr6, 0x832c92a8
	if !ctx.cr[6].eq {
	pc = 0x832C92A8; continue 'dispatch;
	}
	// 832C9294: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9298: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 832C929C: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C92A0: 552AF87E  srwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C92A4: 48000010  b 0x832c92b4
	pc = 0x832C92B4; continue 'dispatch;
	// 832C92A8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 832C92AC: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C92B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832C92B4: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 832C92B8: 9AE50000  stb r23, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[23].u8 ) };
	// 832C92BC: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 832C92C0: 7CE900D0  neg r7, r9
	ctx.r[7].s64 = -ctx.r[9].s64;
	// 832C92C4: 3861FE60  addi r3, r1, -0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + -416;
	// 832C92C8: 530707DE  rlwimi r7, r24, 0, 0x1f, 0xf
	ctx.r[7].u64 = (((ctx.r[24].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFF0001) | (ctx.r[7].u64 & 0x000000000000FFFE);
	// 832C92CC: 7CE81B2E  sthx r7, r8, r3
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[7].u16) };
	// 832C92D0: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 832C92D4: 7F05C840  cmplw cr6, r5, r25
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[25].u32, &mut ctx.xer);
	// 832C92D8: 4198FCB0  blt cr6, 0x832c8f88
	if ctx.cr[6].lt {
	pc = 0x832C8F88; continue 'dispatch;
	}
	// 832C92DC: 9141FE34  stw r10, -0x1cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-460 as u32), ctx.r[10].u32 ) };
	// 832C92E0: 9161FE30  stw r11, -0x1d0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-464 as u32), ctx.r[11].u32 ) };
	// 832C92E4: 8141FE68  lwz r10, -0x198(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-408 as u32) ) } as u64;
	// 832C92E8: 8121FE70  lwz r9, -0x190(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-400 as u32) ) } as u64;
	// 832C92EC: 8101FE78  lwz r8, -0x188(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-392 as u32) ) } as u64;
	// 832C92F0: 80E1FE64  lwz r7, -0x19c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-412 as u32) ) } as u64;
	// 832C92F4: 80A1FE6C  lwz r5, -0x194(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-404 as u32) ) } as u64;
	// 832C92F8: 9141FE40  stw r10, -0x1c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-448 as u32), ctx.r[10].u32 ) };
	// 832C92FC: 9121FE4C  stw r9, -0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-436 as u32), ctx.r[9].u32 ) };
	// 832C9300: 8061FE74  lwz r3, -0x18c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-396 as u32) ) } as u64;
	// 832C9304: 9101FE38  stw r8, -0x1c8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-456 as u32), ctx.r[8].u32 ) };
	// 832C9308: 90E1FE50  stw r7, -0x1b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-432 as u32), ctx.r[7].u32 ) };
	// 832C930C: 90A1FE3C  stw r5, -0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-452 as u32), ctx.r[5].u32 ) };
	// 832C9310: A161FE62  lhz r11, -0x19e(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-414 as u32) ) } as u64;
	// 832C9314: 9061FE44  stw r3, -0x1bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-444 as u32), ctx.r[3].u32 ) };
	// 832C9318: 8221FED8  lwz r17, -0x128(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-296 as u32) ) } as u64;
	// 832C931C: 8241FED0  lwz r18, -0x130(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-304 as u32) ) } as u64;
	// 832C9320: 8261FEB0  lwz r19, -0x150(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-336 as u32) ) } as u64;
	// 832C9324: B161FE48  sth r11, -0x1b8(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-440 as u32), ctx.r[11].u16 ) };
	// 832C9328: 81C1FE30  lwz r14, -0x1d0(r1)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-464 as u32) ) } as u64;
	// 832C932C: 8281FEA8  lwz r20, -0x158(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-344 as u32) ) } as u64;
	// 832C9330: 9221FE30  stw r17, -0x1d0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-464 as u32), ctx.r[17].u32 ) };
	// 832C9334: 82A1FECC  lwz r21, -0x134(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-308 as u32) ) } as u64;
	// 832C9338: 82C1FEC4  lwz r22, -0x13c(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-316 as u32) ) } as u64;
	// 832C933C: 82E1FEA4  lwz r23, -0x15c(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-348 as u32) ) } as u64;
	// 832C9340: 8201FEAC  lwz r16, -0x154(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-340 as u32) ) } as u64;
	// 832C9344: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 832C9348: 8221FE40  lwz r17, -0x1c0(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-448 as u32) ) } as u64;
	// 832C934C: 9241FE40  stw r18, -0x1c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-448 as u32), ctx.r[18].u32 ) };
	// 832C9350: 8241FE4C  lwz r18, -0x1b4(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-436 as u32) ) } as u64;
	// 832C9354: 9261FE4C  stw r19, -0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-436 as u32), ctx.r[19].u32 ) };
	// 832C9358: 8261FE38  lwz r19, -0x1c8(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-456 as u32) ) } as u64;
	// 832C935C: 9281FE38  stw r20, -0x1c8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-456 as u32), ctx.r[20].u32 ) };
	// 832C9360: 8281FE50  lwz r20, -0x1b0(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-432 as u32) ) } as u64;
	// 832C9364: 92A1FE50  stw r21, -0x1b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-432 as u32), ctx.r[21].u32 ) };
	// 832C9368: 82A1FE3C  lwz r21, -0x1c4(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-452 as u32) ) } as u64;
	// 832C936C: 92C1FE3C  stw r22, -0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-452 as u32), ctx.r[22].u32 ) };
	// 832C9370: 82C1FE44  lwz r22, -0x1bc(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-444 as u32) ) } as u64;
	// 832C9374: 92E1FE44  stw r23, -0x1bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-444 as u32), ctx.r[23].u32 ) };
	// 832C9378: 82E1FEB4  lwz r23, -0x14c(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-332 as u32) ) } as u64;
	// 832C937C: 81E1FE34  lwz r15, -0x1cc(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-460 as u32) ) } as u64;
	// 832C9380: 9201FE34  stw r16, -0x1cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-460 as u32), ctx.r[16].u32 ) };
	// 832C9384: A201FE48  lhz r16, -0x1b8(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-440 as u32) ) } as u64;
	// 832C9388: 8141FE7C  lwz r10, -0x184(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-388 as u32) ) } as u64;
	// 832C938C: 92E1FE48  stw r23, -0x1b8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-440 as u32), ctx.r[23].u32 ) };
	// 832C9390: 82E1FED4  lwz r23, -0x12c(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-300 as u32) ) } as u64;
	// 832C9394: 8121FE90  lwz r9, -0x170(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-368 as u32) ) } as u64;
	// 832C9398: 8101FEB8  lwz r8, -0x148(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-328 as u32) ) } as u64;
	// 832C939C: 80E1FE80  lwz r7, -0x180(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-384 as u32) ) } as u64;
	// 832C93A0: 80A1FE88  lwz r5, -0x178(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-376 as u32) ) } as u64;
	// 832C93A4: 8061FE94  lwz r3, -0x16c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-364 as u32) ) } as u64;
	// 832C93A8: 83E1FEBC  lwz r31, -0x144(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-324 as u32) ) } as u64;
	// 832C93AC: 83C1FE84  lwz r30, -0x17c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-380 as u32) ) } as u64;
	// 832C93B0: 83A1FE8C  lwz r29, -0x174(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-372 as u32) ) } as u64;
	// 832C93B4: 8381FE98  lwz r28, -0x168(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-360 as u32) ) } as u64;
	// 832C93B8: 8361FEA0  lwz r27, -0x160(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-352 as u32) ) } as u64;
	// 832C93BC: 8341FEC0  lwz r26, -0x140(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-320 as u32) ) } as u64;
	// 832C93C0: 92E1FE54  stw r23, -0x1ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-428 as u32), ctx.r[23].u32 ) };
	// 832C93C4: 8321FEC8  lwz r25, -0x138(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-312 as u32) ) } as u64;
	// 832C93C8: 8301FE9C  lwz r24, -0x164(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-356 as u32) ) } as u64;
	// 832C93CC: 82E1FEDC  lwz r23, -0x124(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-292 as u32) ) } as u64;
	// 832C93D0: 90C40004  stw r6, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 832C93D4: 91E40000  stw r15, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[15].u32 ) };
	// 832C93D8: 91C40008  stw r14, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[14].u32 ) };
	// 832C93DC: B20B0002  sth r16, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[16].u16 ) };
	// 832C93E0: 922B0004  stw r17, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[17].u32 ) };
	// 832C93E4: 924B0008  stw r18, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[18].u32 ) };
	// 832C93E8: 926B000C  stw r19, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[19].u32 ) };
	// 832C93EC: 928B0010  stw r20, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[20].u32 ) };
	// 832C93F0: 92AB0014  stw r21, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[21].u32 ) };
	// 832C93F4: 92CB0018  stw r22, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[22].u32 ) };
	// 832C93F8: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 832C93FC: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 832C9400: 910B0024  stw r8, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 832C9404: 90EB0028  stw r7, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 832C9408: 90AB002C  stw r5, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 832C940C: 906B0030  stw r3, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 832C9410: 93EB0034  stw r31, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[31].u32 ) };
	// 832C9414: 93CB0038  stw r30, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 832C9418: 93AB003C  stw r29, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 832C941C: 938B0040  stw r28, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[28].u32 ) };
	// 832C9420: 936B0044  stw r27, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[27].u32 ) };
	// 832C9424: 934B0048  stw r26, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 832C9428: 8141FE44  lwz r10, -0x1bc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-444 as u32) ) } as u64;
	// 832C942C: 8121FE3C  lwz r9, -0x1c4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-452 as u32) ) } as u64;
	// 832C9430: 8101FE50  lwz r8, -0x1b0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-432 as u32) ) } as u64;
	// 832C9434: 80E1FE38  lwz r7, -0x1c8(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-456 as u32) ) } as u64;
	// 832C9438: 80A1FE4C  lwz r5, -0x1b4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-436 as u32) ) } as u64;
	// 832C943C: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 832C9440: 912B0058  stw r9, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 832C9444: 910B005C  stw r8, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832C9448: 90EB0060  stw r7, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 832C944C: 8061FE40  lwz r3, -0x1c0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-448 as u32) ) } as u64;
	// 832C9450: 8141FE30  lwz r10, -0x1d0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-464 as u32) ) } as u64;
	// 832C9454: 8121FE34  lwz r9, -0x1cc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-460 as u32) ) } as u64;
	// 832C9458: 8101FE48  lwz r8, -0x1b8(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-440 as u32) ) } as u64;
	// 832C945C: 80E1FE54  lwz r7, -0x1ac(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-428 as u32) ) } as u64;
	// 832C9460: 932B004C  stw r25, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[25].u32 ) };
	// 832C9464: 930B0050  stw r24, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 832C9468: 90AB0064  stw r5, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 832C946C: 906B0068  stw r3, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 832C9470: 914B006C  stw r10, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 832C9474: 912B0070  stw r9, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 832C9478: 910B0074  stw r8, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 832C947C: 90EB0078  stw r7, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 832C9480: 92EB007C  stw r23, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[23].u32 ) };
	// 832C9484: 4B9DFF9C  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C9488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832C9488 size=1308
    let mut pc: u32 = 0x832C9488;
    'dispatch: loop {
        match pc {
            0x832C9488 => {
    //   block [0x832C9488..0x832C94D8)
	// 832C9488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C948C: 4B9DFF61  bl 0x82ca93ec
	ctx.lr = 0x832C9490;
	sub_82CA93D0(ctx, base);
	// 832C9490: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 832C9494: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 832C9498: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C949C: 83E40004  lwz r31, 4(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 832C94A0: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 832C94A4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 832C94A8: 40980030  bge cr6, 0x832c94d8
	if !ctx.cr[6].lt {
	pc = 0x832C94D8; continue 'dispatch;
	}
	// 832C94AC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C94B0: 5548063E  clrlwi r8, r10, 0x18
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 832C94B4: 20EB0003  subfic r7, r11, 3
	ctx.xer.ca = ctx.r[11].u32 <= 3 as u32;
	ctx.r[7].s64 = (3 as i64) - ctx.r[11].s64;
	// 832C94B8: 7D265830  slw r6, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 832C94BC: 54CA063E  clrlwi r10, r6, 0x18
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 832C94C0: 7D273C30  srw r7, r9, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[9].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 832C94C4: 7D494378  or r9, r10, r8
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 832C94C8: 396B001D  addi r11, r11, 0x1d
	ctx.r[11].s64 = ctx.r[11].s64 + 29;
	// 832C94CC: 5529077E  clrlwi r9, r9, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000007u64;
	// 832C94D0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C94D4: 48000010  b 0x832c94e4
	pc = 0x832C94E4; continue 'dispatch;
            }
            0x832C94D8 => {
    //   block [0x832C94D8..0x832C94E4)
	// 832C94D8: 5549077E  clrlwi r9, r10, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 832C94DC: 5547E8FE  srwi r7, r10, 3
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C94E0: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	pc = 0x832C94E4; continue 'dispatch;
            }
            0x832C94E4 => {
    //   block [0x832C94E4..0x832C9528)
	// 832C94E4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832C94E8: 35490001  addic. r10, r9, 1
	ctx.xer.ca = (ctx.r[9].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832C94EC: 3BA00060  li r29, 0x60
	ctx.r[29].s64 = 96;
	// 832C94F0: 98C1FF14  stb r6, -0xec(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-236 as u32), ctx.r[6].u8 ) };
	// 832C94F4: 3B6000B0  li r27, 0xb0
	ctx.r[27].s64 = 176;
	// 832C94F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832C94FC: 9BA1FF15  stb r29, -0xeb(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-235 as u32), ctx.r[29].u8 ) };
	// 832C9500: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 832C9504: 9B61FF16  stb r27, -0xea(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-234 as u32), ctx.r[27].u8 ) };
	// 832C9508: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 832C950C: 3B81FF14  addi r28, r1, -0xec
	ctx.r[28].s64 = ctx.r[1].s64 + -236;
	// 832C9510: 98C1FF17  stb r6, -0xe9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-233 as u32), ctx.r[6].u8 ) };
	// 832C9514: 3B41FF18  addi r26, r1, -0xe8
	ctx.r[26].s64 = ctx.r[1].s64 + -232;
	// 832C9518: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 832C951C: 7D3B4030  slw r27, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[27].u64 = 0;
	} else {
		ctx.r[27].u64 = ((ctx.r[9].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 832C9520: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 832C9524: 41820470  beq 0x832c9994
	if ctx.cr[0].eq {
	pc = 0x832C9994; continue 'dispatch;
	}
	pc = 0x832C9528; continue 'dispatch;
            }
            0x832C9528 => {
    //   block [0x832C9528..0x832C9534)
	// 832C9528: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 832C952C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832C9530: 40990078  ble cr6, 0x832c95a8
	if !ctx.cr[6].gt {
	pc = 0x832C95A8; continue 'dispatch;
	}
	pc = 0x832C9534; continue 'dispatch;
            }
            0x832C9534 => {
    //   block [0x832C9534..0x832C9550)
	// 832C9534: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C9538: 409A0018  bne cr6, 0x832c9550
	if !ctx.cr[6].eq {
	pc = 0x832C9550; continue 'dispatch;
	}
	// 832C953C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9540: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C9544: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C9548: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C954C: 48000010  b 0x832c955c
	pc = 0x832C955C; continue 'dispatch;
            }
            0x832C9550 => {
    //   block [0x832C9550..0x832C955C)
	// 832C9550: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C9554: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9558: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C955C; continue 'dispatch;
            }
            0x832C955C => {
    //   block [0x832C955C..0x832C9588)
	// 832C955C: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C9560: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C9564: 419A0038  beq cr6, 0x832c959c
	if ctx.cr[6].eq {
	pc = 0x832C959C; continue 'dispatch;
	}
	// 832C9568: 3941FF60  addi r10, r1, -0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + -160;
	// 832C956C: 7D0650AE  lbzx r8, r6, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 832C9570: 7D5B00D0  neg r10, r27
	ctx.r[10].s64 = -ctx.r[27].s64;
	// 832C9574: 7D2818AE  lbzx r9, r8, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 832C9578: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 832C957C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 832C9580: 41980008  blt cr6, 0x832c9588
	if ctx.cr[6].lt {
	pc = 0x832C9588; continue 'dispatch;
	}
	// 832C9584: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	pc = 0x832C9588; continue 'dispatch;
            }
            0x832C9588 => {
    //   block [0x832C9588..0x832C959C)
	// 832C9588: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 832C958C: 7F1E2840  cmplw cr6, r30, r5
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[5].u32, &mut ctx.xer);
	// 832C9590: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832C9594: 7D4819AE  stbx r10, r8, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u8) };
	// 832C9598: 419A03FC  beq cr6, 0x832c9994
	if ctx.cr[6].eq {
	pc = 0x832C9994; continue 'dispatch;
	}
	pc = 0x832C959C; continue 'dispatch;
            }
            0x832C959C => {
    //   block [0x832C959C..0x832C95A8)
	// 832C959C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 832C95A0: 7F06E800  cmpw cr6, r6, r29
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[29].s32, &mut ctx.xer);
	// 832C95A4: 4198FF90  blt cr6, 0x832c9534
	if ctx.cr[6].lt {
	pc = 0x832C9534; continue 'dispatch;
	}
	pc = 0x832C95A8; continue 'dispatch;
            }
            0x832C95A8 => {
    //   block [0x832C95A8..0x832C95BC)
	// 832C95A8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 832C95AC: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 832C95B0: 409803D8  bge cr6, 0x832c9988
	if !ctx.cr[6].lt {
	pc = 0x832C9988; continue 'dispatch;
	}
	// 832C95B4: 3941FF60  addi r10, r1, -0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + -160;
	// 832C95B8: 7D1D5214  add r8, r29, r10
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	pc = 0x832C95BC; continue 'dispatch;
            }
            0x832C95BC => {
    //   block [0x832C95BC..0x832C95E4)
	// 832C95BC: 89260000  lbz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C95C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832C95C4: 419A03B8  beq cr6, 0x832c997c
	if ctx.cr[6].eq {
	pc = 0x832C997C; continue 'dispatch;
	}
	// 832C95C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C95CC: 409A0018  bne cr6, 0x832c95e4
	if !ctx.cr[6].eq {
	pc = 0x832C95E4; continue 'dispatch;
	}
	// 832C95D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C95D4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C95D8: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C95DC: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C95E0: 48000010  b 0x832c95f0
	pc = 0x832C95F0; continue 'dispatch;
            }
            0x832C95E4 => {
    //   block [0x832C95E4..0x832C95F0)
	// 832C95E4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C95E8: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C95EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C95F0; continue 'dispatch;
            }
            0x832C95F0 => {
    //   block [0x832C95F0..0x832C9630)
	// 832C95F0: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C95F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C95F8: 419A0384  beq cr6, 0x832c997c
	if ctx.cr[6].eq {
	pc = 0x832C997C; continue 'dispatch;
	}
	// 832C95FC: 552A07BE  clrlwi r10, r9, 0x1e
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 832C9600: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 832C9604: 41990378  bgt cr6, 0x832c997c
	if ctx.cr[6].gt {
	pc = 0x832C997C; continue 'dispatch;
	}
	// 832C9608: 3D80832D  lis r12, -0x7cd3
	ctx.r[12].s64 = -2094202880;
	// 832C960C: 398C9620  addi r12, r12, -0x69e0
	ctx.r[12].s64 = ctx.r[12].s64 + -27104;
	// 832C9610: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 832C9614: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 832C9618: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 832C961C: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x832C9630; continue 'dispatch;
		},
		1 => {
	pc = 0x832C9644; continue 'dispatch;
		},
		2 => {
	pc = 0x832C9680; continue 'dispatch;
		},
		3 => {
	pc = 0x832C991C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 832C9620: 832C9630  lwz r25, -0x69d0(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-27088 as u32) ) } as u64;
	// 832C9624: 832C9644  lwz r25, -0x69bc(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-27068 as u32) ) } as u64;
	// 832C9628: 832C9680  lwz r25, -0x6980(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-27008 as u32) ) } as u64;
	// 832C962C: 832C991C  lwz r25, -0x66e4(r12)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-26340 as u32) ) } as u64;
            }
            0x832C9630 => {
    //   block [0x832C9630..0x832C9644)
	// 832C9630: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C9634: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C9638: 394A0011  addi r10, r10, 0x11
	ctx.r[10].s64 = ctx.r[10].s64 + 17;
	// 832C963C: 99460000  stb r10, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 832C9640: 4800004C  b 0x832c968c
	pc = 0x832C968C; continue 'dispatch;
            }
            0x832C9644 => {
    //   block [0x832C9644..0x832C9680)
	// 832C9644: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 832C9648: 393A0001  addi r9, r26, 1
	ctx.r[9].s64 = ctx.r[26].s64 + 1;
	// 832C964C: 554A003A  rlwinm r10, r10, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 832C9650: 3AEA0002  addi r23, r10, 2
	ctx.r[23].s64 = ctx.r[10].s64 + 2;
	// 832C9654: 3ACA0012  addi r22, r10, 0x12
	ctx.r[22].s64 = ctx.r[10].s64 + 18;
	// 832C9658: 3AAA0022  addi r21, r10, 0x22
	ctx.r[21].s64 = ctx.r[10].s64 + 34;
	// 832C965C: 9AE60000  stb r23, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[23].u8 ) };
	// 832C9660: 394A0032  addi r10, r10, 0x32
	ctx.r[10].s64 = ctx.r[10].s64 + 50;
	// 832C9664: 9ADA0000  stb r22, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[22].u8 ) };
	// 832C9668: 9ABA0001  stb r21, 1(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(1 as u32), ctx.r[21].u8 ) };
	// 832C966C: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 832C9670: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 832C9674: 9AE90001  stb r23, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[23].u8 ) };
	// 832C9678: 3B4A0001  addi r26, r10, 1
	ctx.r[26].s64 = ctx.r[10].s64 + 1;
	// 832C967C: 48000304  b 0x832c9980
	pc = 0x832C9980; continue 'dispatch;
            }
            0x832C9680 => {
    //   block [0x832C9680..0x832C968C)
	// 832C9680: 9B260000  stb r25, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 832C9684: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C9688: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	pc = 0x832C968C; continue 'dispatch;
            }
            0x832C968C => {
    //   block [0x832C968C..0x832C96A8)
	// 832C968C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C9690: 409A0018  bne cr6, 0x832c96a8
	if !ctx.cr[6].eq {
	pc = 0x832C96A8; continue 'dispatch;
	}
	// 832C9694: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9698: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C969C: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C96A0: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C96A4: 48000010  b 0x832c96b4
	pc = 0x832C96B4; continue 'dispatch;
            }
            0x832C96A8 => {
    //   block [0x832C96A8..0x832C96B4)
	// 832C96A8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C96AC: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C96B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C96B4; continue 'dispatch;
            }
            0x832C96B4 => {
    //   block [0x832C96B4..0x832C96D4)
	// 832C96B4: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C96B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C96BC: 419A0018  beq cr6, 0x832c96d4
	if ctx.cr[6].eq {
	pc = 0x832C96D4; continue 'dispatch;
	}
	// 832C96C0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C96C4: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 832C96C8: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 832C96CC: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 832C96D0: 4800005C  b 0x832c972c
	pc = 0x832C972C; continue 'dispatch;
            }
            0x832C96D4 => {
    //   block [0x832C96D4..0x832C96FC)
	// 832C96D4: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C96D8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 832C96DC: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C96E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C96E4: 409A0018  bne cr6, 0x832c96fc
	if !ctx.cr[6].eq {
	pc = 0x832C96FC; continue 'dispatch;
	}
	// 832C96E8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C96EC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C96F0: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C96F4: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C96F8: 48000010  b 0x832c9708
	pc = 0x832C9708; continue 'dispatch;
            }
            0x832C96FC => {
    //   block [0x832C96FC..0x832C9708)
	// 832C96FC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C9700: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9704: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C9708; continue 'dispatch;
            }
            0x832C9708 => {
    //   block [0x832C9708..0x832C971C)
	// 832C9708: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C970C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C9710: 7D5B00D0  neg r10, r27
	ctx.r[10].s64 = -ctx.r[27].s64;
	// 832C9714: 409A0008  bne cr6, 0x832c971c
	if !ctx.cr[6].eq {
	pc = 0x832C971C; continue 'dispatch;
	}
	// 832C9718: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	pc = 0x832C971C; continue 'dispatch;
            }
            0x832C971C => {
    //   block [0x832C971C..0x832C972C)
	// 832C971C: 7F1E2840  cmplw cr6, r30, r5
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[5].u32, &mut ctx.xer);
	// 832C9720: 7D4919AE  stbx r10, r9, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u8) };
	// 832C9724: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832C9728: 419A026C  beq cr6, 0x832c9994
	if ctx.cr[6].eq {
	pc = 0x832C9994; continue 'dispatch;
	}
	pc = 0x832C972C; continue 'dispatch;
            }
            0x832C972C => {
    //   block [0x832C972C..0x832C974C)
	// 832C972C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832C9730: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C9734: 409A0018  bne cr6, 0x832c974c
	if !ctx.cr[6].eq {
	pc = 0x832C974C; continue 'dispatch;
	}
	// 832C9738: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C973C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C9740: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C9744: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9748: 48000010  b 0x832c9758
	pc = 0x832C9758; continue 'dispatch;
            }
            0x832C974C => {
    //   block [0x832C974C..0x832C9758)
	// 832C974C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C9750: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9754: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C9758; continue 'dispatch;
            }
            0x832C9758 => {
    //   block [0x832C9758..0x832C9778)
	// 832C9758: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C975C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C9760: 419A0018  beq cr6, 0x832c9778
	if ctx.cr[6].eq {
	pc = 0x832C9778; continue 'dispatch;
	}
	// 832C9764: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C9768: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 832C976C: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 832C9770: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 832C9774: 4800005C  b 0x832c97d0
	pc = 0x832C97D0; continue 'dispatch;
            }
            0x832C9778 => {
    //   block [0x832C9778..0x832C97A0)
	// 832C9778: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C977C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 832C9780: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C9784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C9788: 409A0018  bne cr6, 0x832c97a0
	if !ctx.cr[6].eq {
	pc = 0x832C97A0; continue 'dispatch;
	}
	// 832C978C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9790: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C9794: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C9798: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C979C: 48000010  b 0x832c97ac
	pc = 0x832C97AC; continue 'dispatch;
            }
            0x832C97A0 => {
    //   block [0x832C97A0..0x832C97AC)
	// 832C97A0: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C97A4: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C97A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C97AC; continue 'dispatch;
            }
            0x832C97AC => {
    //   block [0x832C97AC..0x832C97C0)
	// 832C97AC: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C97B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C97B4: 7D5B00D0  neg r10, r27
	ctx.r[10].s64 = -ctx.r[27].s64;
	// 832C97B8: 409A0008  bne cr6, 0x832c97c0
	if !ctx.cr[6].eq {
	pc = 0x832C97C0; continue 'dispatch;
	}
	// 832C97BC: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	pc = 0x832C97C0; continue 'dispatch;
            }
            0x832C97C0 => {
    //   block [0x832C97C0..0x832C97D0)
	// 832C97C0: 7F1E2840  cmplw cr6, r30, r5
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[5].u32, &mut ctx.xer);
	// 832C97C4: 7D4919AE  stbx r10, r9, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u8) };
	// 832C97C8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832C97CC: 419A01C8  beq cr6, 0x832c9994
	if ctx.cr[6].eq {
	pc = 0x832C9994; continue 'dispatch;
	}
	pc = 0x832C97D0; continue 'dispatch;
            }
            0x832C97D0 => {
    //   block [0x832C97D0..0x832C97F0)
	// 832C97D0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832C97D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C97D8: 409A0018  bne cr6, 0x832c97f0
	if !ctx.cr[6].eq {
	pc = 0x832C97F0; continue 'dispatch;
	}
	// 832C97DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C97E0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C97E4: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C97E8: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C97EC: 48000010  b 0x832c97fc
	pc = 0x832C97FC; continue 'dispatch;
            }
            0x832C97F0 => {
    //   block [0x832C97F0..0x832C97FC)
	// 832C97F0: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C97F4: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C97F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C97FC; continue 'dispatch;
            }
            0x832C97FC => {
    //   block [0x832C97FC..0x832C981C)
	// 832C97FC: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C9800: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C9804: 419A0018  beq cr6, 0x832c981c
	if ctx.cr[6].eq {
	pc = 0x832C981C; continue 'dispatch;
	}
	// 832C9808: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C980C: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 832C9810: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 832C9814: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 832C9818: 4800005C  b 0x832c9874
	pc = 0x832C9874; continue 'dispatch;
            }
            0x832C981C => {
    //   block [0x832C981C..0x832C9844)
	// 832C981C: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C9820: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 832C9824: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C9828: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C982C: 409A0018  bne cr6, 0x832c9844
	if !ctx.cr[6].eq {
	pc = 0x832C9844; continue 'dispatch;
	}
	// 832C9830: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9834: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C9838: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C983C: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9840: 48000010  b 0x832c9850
	pc = 0x832C9850; continue 'dispatch;
            }
            0x832C9844 => {
    //   block [0x832C9844..0x832C9850)
	// 832C9844: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C9848: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C984C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C9850; continue 'dispatch;
            }
            0x832C9850 => {
    //   block [0x832C9850..0x832C9864)
	// 832C9850: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C9854: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C9858: 7D5B00D0  neg r10, r27
	ctx.r[10].s64 = -ctx.r[27].s64;
	// 832C985C: 409A0008  bne cr6, 0x832c9864
	if !ctx.cr[6].eq {
	pc = 0x832C9864; continue 'dispatch;
	}
	// 832C9860: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	pc = 0x832C9864; continue 'dispatch;
            }
            0x832C9864 => {
    //   block [0x832C9864..0x832C9874)
	// 832C9864: 7F1E2840  cmplw cr6, r30, r5
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[5].u32, &mut ctx.xer);
	// 832C9868: 7D4919AE  stbx r10, r9, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u8) };
	// 832C986C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832C9870: 419A0124  beq cr6, 0x832c9994
	if ctx.cr[6].eq {
	pc = 0x832C9994; continue 'dispatch;
	}
	pc = 0x832C9874; continue 'dispatch;
            }
            0x832C9874 => {
    //   block [0x832C9874..0x832C9894)
	// 832C9874: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832C9878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C987C: 409A0018  bne cr6, 0x832c9894
	if !ctx.cr[6].eq {
	pc = 0x832C9894; continue 'dispatch;
	}
	// 832C9880: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9884: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C9888: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C988C: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9890: 48000010  b 0x832c98a0
	pc = 0x832C98A0; continue 'dispatch;
            }
            0x832C9894 => {
    //   block [0x832C9894..0x832C98A0)
	// 832C9894: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C9898: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C989C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C98A0; continue 'dispatch;
            }
            0x832C98A0 => {
    //   block [0x832C98A0..0x832C98C0)
	// 832C98A0: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C98A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C98A8: 419A0018  beq cr6, 0x832c98c0
	if ctx.cr[6].eq {
	pc = 0x832C98C0; continue 'dispatch;
	}
	// 832C98AC: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832C98B0: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 832C98B4: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 832C98B8: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 832C98BC: 480000C4  b 0x832c9980
	pc = 0x832C9980; continue 'dispatch;
            }
            0x832C98C0 => {
    //   block [0x832C98C0..0x832C98E8)
	// 832C98C0: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C98C4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 832C98C8: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C98CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C98D0: 409A0018  bne cr6, 0x832c98e8
	if !ctx.cr[6].eq {
	pc = 0x832C98E8; continue 'dispatch;
	}
	// 832C98D4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C98D8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C98DC: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C98E0: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C98E4: 48000010  b 0x832c98f4
	pc = 0x832C98F4; continue 'dispatch;
            }
            0x832C98E8 => {
    //   block [0x832C98E8..0x832C98F4)
	// 832C98E8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C98EC: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C98F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C98F4; continue 'dispatch;
            }
            0x832C98F4 => {
    //   block [0x832C98F4..0x832C9908)
	// 832C98F4: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C98F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C98FC: 7D5B00D0  neg r10, r27
	ctx.r[10].s64 = -ctx.r[27].s64;
	// 832C9900: 409A0008  bne cr6, 0x832c9908
	if !ctx.cr[6].eq {
	pc = 0x832C9908; continue 'dispatch;
	}
	// 832C9904: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	pc = 0x832C9908; continue 'dispatch;
            }
            0x832C9908 => {
    //   block [0x832C9908..0x832C991C)
	// 832C9908: 7F1E2840  cmplw cr6, r30, r5
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[5].u32, &mut ctx.xer);
	// 832C990C: 7D4919AE  stbx r10, r9, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u8) };
	// 832C9910: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832C9914: 419A0080  beq cr6, 0x832c9994
	if ctx.cr[6].eq {
	pc = 0x832C9994; continue 'dispatch;
	}
	// 832C9918: 48000068  b 0x832c9980
	pc = 0x832C9980; continue 'dispatch;
            }
            0x832C991C => {
    //   block [0x832C991C..0x832C9948)
	// 832C991C: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832C9920: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 832C9924: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 832C9928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832C992C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 832C9930: 409A0018  bne cr6, 0x832c9948
	if !ctx.cr[6].eq {
	pc = 0x832C9948; continue 'dispatch;
	}
	// 832C9934: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832C9938: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 832C993C: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 832C9940: 5547F87E  srwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9944: 48000010  b 0x832c9954
	pc = 0x832C9954; continue 'dispatch;
            }
            0x832C9948 => {
    //   block [0x832C9948..0x832C9954)
	// 832C9948: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 832C994C: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832C9950: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x832C9954; continue 'dispatch;
            }
            0x832C9954 => {
    //   block [0x832C9954..0x832C9968)
	// 832C9954: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 832C9958: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 832C995C: 7D5B00D0  neg r10, r27
	ctx.r[10].s64 = -ctx.r[27].s64;
	// 832C9960: 409A0008  bne cr6, 0x832c9968
	if !ctx.cr[6].eq {
	pc = 0x832C9968; continue 'dispatch;
	}
	// 832C9964: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	pc = 0x832C9968; continue 'dispatch;
            }
            0x832C9968 => {
    //   block [0x832C9968..0x832C997C)
	// 832C9968: 7F1E2840  cmplw cr6, r30, r5
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[5].u32, &mut ctx.xer);
	// 832C996C: 7D4919AE  stbx r10, r9, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u8) };
	// 832C9970: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832C9974: 419A0020  beq cr6, 0x832c9994
	if ctx.cr[6].eq {
	pc = 0x832C9994; continue 'dispatch;
	}
	// 832C9978: 9B260000  stb r25, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	pc = 0x832C997C; continue 'dispatch;
            }
            0x832C997C => {
    //   block [0x832C997C..0x832C9980)
	// 832C997C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	pc = 0x832C9980; continue 'dispatch;
            }
            0x832C9980 => {
    //   block [0x832C9980..0x832C9988)
	// 832C9980: 7F06D040  cmplw cr6, r6, r26
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[26].u32, &mut ctx.xer);
	// 832C9984: 4198FC38  blt cr6, 0x832c95bc
	if ctx.cr[6].lt {
	pc = 0x832C95BC; continue 'dispatch;
	}
	pc = 0x832C9988; continue 'dispatch;
            }
            0x832C9988 => {
    //   block [0x832C9988..0x832C9994)
	// 832C9988: 7F7B0E70  srawi r27, r27, 1
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[27].s32 >> 1) as i64;
	// 832C998C: 3718FFFF  addic. r24, r24, -1
	ctx.xer.ca = (ctx.r[24].u32 > (!(-1 as u32)));
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 832C9990: 4082FB98  bne 0x832c9528
	if !ctx.cr[0].eq {
	pc = 0x832C9528; continue 'dispatch;
	}
	pc = 0x832C9994; continue 'dispatch;
            }
            0x832C9994 => {
    //   block [0x832C9994..0x832C99A4)
	// 832C9994: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 832C9998: 90E40000  stw r7, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 832C999C: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832C99A0: 4B9DFA9C  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832C99A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832C99A8 size=1684
    let mut pc: u32 = 0x832C99A8;
    'dispatch: loop {
        match pc {
            0x832C99A8 => {
    //   block [0x832C99A8..0x832CA03C)
	// 832C99A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832C99AC: 4B9DFA25  bl 0x82ca93d0
	ctx.lr = 0x832C99B0;
	sub_82CA93D0(ctx, base);
	// 832C99B0: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832C99B4: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 832C99B8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 832C99BC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 832C99C0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 832C99C4: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 832C99C8: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 832C99CC: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 832C99D0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 832C99D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


