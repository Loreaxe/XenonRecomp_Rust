pub fn sub_824110A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824110A8 size=32
    let mut pc: u32 = 0x824110A8;
    'dispatch: loop {
        match pc {
            0x824110A8 => {
    //   block [0x824110A8..0x824110C4)
	// 824110A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824110AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824110B0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824110B4: 41980044  blt cr6, 0x824110f8
	if ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x824110F8);
		return;
	}
	// 824110B8: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824110BC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824110C0: 40800008  bge 0x824110c8
	if !ctx.cr[0].lt {
		crate::recompiler::externs::call(ctx, base, 0x824110C8);
		return;
	}
	pc = 0x824110C4; continue 'dispatch;
            }
            0x824110C4 => {
    //   block [0x824110C4..0x824110C8)
	// 824110C4: 48000000  b 0x824110c4
	pc = 0x824110C4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411100 size=556
    let mut pc: u32 = 0x82411100;
    'dispatch: loop {
        match pc {
            0x82411100 => {
    //   block [0x82411100..0x8241111C)
	// 82411100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411104: 48123FA1  bl 0x825350a4
	ctx.lr = 0x82411108;
	sub_82535080(ctx, base);
	// 82411108: 9421FC80  stwu r1, -0x380(r1)
	ea = ctx.r[1].u32.wrapping_add(-896 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241110C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82411110: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82411114: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82411118: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	pc = 0x8241111C; continue 'dispatch;
            }
            0x8241111C => {
    //   block [0x8241111C..0x8241112C)
	// 8241111C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82411120: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82411124: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 82411128: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8241112C; continue 'dispatch;
            }
            0x8241112C => {
    //   block [0x8241112C..0x82411168)
	// 8241112C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82411130: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82411134: 4200FFF8  bdnz 0x8241112c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8241112C; continue 'dispatch;
	}
	// 82411138: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8241113C: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82411140: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82411144: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 82411148: 4080FFD4  bge 0x8241111c
	if !ctx.cr[0].lt {
	pc = 0x8241111C; continue 'dispatch;
	}
	// 8241114C: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411150: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82411154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82411158: 3B8B3920  addi r28, r11, 0x3920
	ctx.r[28].s64 = ctx.r[11].s64 + 14624;
	// 8241115C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411160: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82411164: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82411168; continue 'dispatch;
            }
            0x82411168 => {
    //   block [0x82411168..0x824111A4)
	// 82411168: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8241116C: 41980040  blt cr6, 0x824111ac
	if ctx.cr[6].lt {
	pc = 0x824111AC; continue 'dispatch;
	}
	// 82411170: 2F07000C  cmpwi cr6, r7, 0xc
	ctx.cr[6].compare_i32(ctx.r[7].s32, 12, &mut ctx.xer);
	// 82411174: 41990034  bgt cr6, 0x824111a8
	if ctx.cr[6].gt {
	pc = 0x824111A8; continue 'dispatch;
	}
	// 82411178: 38C70001  addi r6, r7, 1
	ctx.r[6].s64 = ctx.r[7].s64 + 1;
	// 8241117C: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411180: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82411184: 7CC6482E  lwzx r6, r6, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82411188: 7F062840  cmplw cr6, r6, r5
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8241118C: 419A0024  beq cr6, 0x824111b0
	if ctx.cr[6].eq {
	pc = 0x824111B0; continue 'dispatch;
	}
	// 82411190: 39080038  addi r8, r8, 0x38
	ctx.r[8].s64 = ctx.r[8].s64 + 56;
	// 82411194: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411198: 394A0038  addi r10, r10, 0x38
	ctx.r[10].s64 = ctx.r[10].s64 + 56;
	// 8241119C: 2B0802D8  cmplwi cr6, r8, 0x2d8
	ctx.cr[6].compare_u32(ctx.r[8].u32, 728 as u32, &mut ctx.xer);
	// 824111A0: 4198FFC8  blt cr6, 0x82411168
	if ctx.cr[6].lt {
	pc = 0x82411168; continue 'dispatch;
	}
	pc = 0x824111A4; continue 'dispatch;
            }
            0x824111A4 => {
    //   block [0x824111A4..0x824111A8)
	// 824111A4: 48000000  b 0x824111a4
	pc = 0x824111A4; continue 'dispatch;
            }
            0x824111A8 => {
    //   block [0x824111A8..0x824111AC)
	// 824111A8: 48000000  b 0x824111a8
	pc = 0x824111A8; continue 'dispatch;
            }
            0x824111AC => {
    //   block [0x824111AC..0x824111B0)
	// 824111AC: 48000000  b 0x824111ac
	pc = 0x824111AC; continue 'dispatch;
            }
            0x824111B0 => {
    //   block [0x824111B0..0x824111CC)
	// 824111B0: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 56 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824111B4: 7FABE214  add r29, r11, r28
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824111B8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824111BC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824111C0: 3B7D0004  addi r27, r29, 4
	ctx.r[27].s64 = ctx.r[29].s64 + 4;
	// 824111C4: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 824111C8: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	pc = 0x824111CC; continue 'dispatch;
            }
            0x824111CC => {
    //   block [0x824111CC..0x82411230)
	// 824111CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824111D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824111D4: 419A00A0  beq cr6, 0x82411274
	if ctx.cr[6].eq {
	pc = 0x82411274; continue 'dispatch;
	}
	// 824111D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824111DC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 824111E0: 419800B4  blt cr6, 0x82411294
	if ctx.cr[6].lt {
	pc = 0x82411294; continue 'dispatch;
	}
	// 824111E4: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 824111E8: 409800A8  bge cr6, 0x82411290
	if !ctx.cr[6].lt {
	pc = 0x82411290; continue 'dispatch;
	}
	// 824111EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824111F0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824111F4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824111F8: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 824111FC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82411200: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82411204: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411208: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8241120C: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82411210: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411214: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 56 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82411218: 7CABE214  add r5, r11, r28
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8241121C: 4BFFFD05  bl 0x82410f20
	ctx.lr = 0x82411220;
	sub_82410F20(ctx, base);
	// 82411220: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411224: 4182000C  beq 0x82411230
	if ctx.cr[0].eq {
	pc = 0x82411230; continue 'dispatch;
	}
	// 82411228: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8241122C: 48000034  b 0x82411260
	pc = 0x82411260; continue 'dispatch;
            }
            0x82411230 => {
    //   block [0x82411230..0x82411260)
	// 82411230: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411234: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82411238: 41800054  blt 0x8241128c
	if ctx.cr[0].lt {
	pc = 0x8241128C; continue 'dispatch;
	}
	// 8241123C: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 82411240: 40980048  bge cr6, 0x82411288
	if !ctx.cr[6].lt {
	pc = 0x82411288; continue 'dispatch;
	}
	// 82411244: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241124C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82411250: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82411254: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411258: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8241125C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82411260; continue 'dispatch;
            }
            0x82411260 => {
    //   block [0x82411260..0x82411274)
	// 82411260: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82411264: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82411268: 3BFF0038  addi r31, r31, 0x38
	ctx.r[31].s64 = ctx.r[31].s64 + 56;
	// 8241126C: 2B1A000D  cmplwi cr6, r26, 0xd
	ctx.cr[6].compare_u32(ctx.r[26].u32, 13 as u32, &mut ctx.xer);
	// 82411270: 4198FF5C  blt cr6, 0x824111cc
	if ctx.cr[6].lt {
	pc = 0x824111CC; continue 'dispatch;
	}
	pc = 0x82411274; continue 'dispatch;
            }
            0x82411274 => {
    //   block [0x82411274..0x82411280)
	// 82411274: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82411278: 409A0020  bne cr6, 0x82411298
	if !ctx.cr[6].eq {
	pc = 0x82411298; continue 'dispatch;
	}
	// 8241127C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82411280; continue 'dispatch;
            }
            0x82411280 => {
    //   block [0x82411280..0x82411288)
	// 82411280: 38210380  addi r1, r1, 0x380
	ctx.r[1].s64 = ctx.r[1].s64 + 896;
	// 82411284: 48123E70  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            0x82411288 => {
    //   block [0x82411288..0x8241128C)
	// 82411288: 48000000  b 0x82411288
	pc = 0x82411288; continue 'dispatch;
            }
            0x8241128C => {
    //   block [0x8241128C..0x82411290)
	// 8241128C: 48000000  b 0x8241128c
	pc = 0x8241128C; continue 'dispatch;
            }
            0x82411290 => {
    //   block [0x82411290..0x82411294)
	// 82411290: 48000000  b 0x82411290
	pc = 0x82411290; continue 'dispatch;
            }
            0x82411294 => {
    //   block [0x82411294..0x82411298)
	// 82411294: 48000000  b 0x82411294
	pc = 0x82411294; continue 'dispatch;
            }
            0x82411298 => {
    //   block [0x82411298..0x824112AC)
	// 82411298: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8241129C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 824112A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824112A4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 824112A8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	pc = 0x824112AC; continue 'dispatch;
            }
            0x824112AC => {
    //   block [0x824112AC..0x824112D4)
	// 824112AC: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 824112B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824112B4: 419A0034  beq cr6, 0x824112e8
	if ctx.cr[6].eq {
	pc = 0x824112E8; continue 'dispatch;
	}
	// 824112B8: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824112BC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824112C0: 41800014  blt 0x824112d4
	if ctx.cr[0].lt {
	pc = 0x824112D4; continue 'dispatch;
	}
	// 824112C4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824112C8: 4098000C  bge cr6, 0x824112d4
	if !ctx.cr[6].lt {
	pc = 0x824112D4; continue 'dispatch;
	}
	// 824112CC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824112D0: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	pc = 0x824112D4; continue 'dispatch;
            }
            0x824112D4 => {
    //   block [0x824112D4..0x824112E8)
	// 824112D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824112D8: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 824112DC: 39290038  addi r9, r9, 0x38
	ctx.r[9].s64 = ctx.r[9].s64 + 56;
	// 824112E0: 2B0B000D  cmplwi cr6, r11, 0xd
	ctx.cr[6].compare_u32(ctx.r[11].u32, 13 as u32, &mut ctx.xer);
	// 824112E4: 4198FFC8  blt cr6, 0x824112ac
	if ctx.cr[6].lt {
	pc = 0x824112AC; continue 'dispatch;
	}
	pc = 0x824112E8; continue 'dispatch;
            }
            0x824112E8 => {
    //   block [0x824112E8..0x824112F0)
	// 824112E8: 2F08000D  cmpwi cr6, r8, 0xd
	ctx.cr[6].compare_i32(ctx.r[8].s32, 13, &mut ctx.xer);
	// 824112EC: 409A0008  bne cr6, 0x824112f4
	if !ctx.cr[6].eq {
	pc = 0x824112F4; continue 'dispatch;
	}
	pc = 0x824112F0; continue 'dispatch;
            }
            0x824112F0 => {
    //   block [0x824112F0..0x824112F4)
	// 824112F0: 48000000  b 0x824112f0
	pc = 0x824112F0; continue 'dispatch;
            }
            0x824112F4 => {
    //   block [0x824112F4..0x824112FC)
	// 824112F4: 2B07000D  cmplwi cr6, r7, 0xd
	ctx.cr[6].compare_u32(ctx.r[7].u32, 13 as u32, &mut ctx.xer);
	// 824112F8: 409A0008  bne cr6, 0x82411300
	if !ctx.cr[6].eq {
	pc = 0x82411300; continue 'dispatch;
	}
	pc = 0x824112FC; continue 'dispatch;
            }
            0x824112FC => {
    //   block [0x824112FC..0x82411300)
	// 824112FC: 48000000  b 0x824112fc
	pc = 0x824112FC; continue 'dispatch;
            }
            0x82411300 => {
    //   block [0x82411300..0x8241132C)
	// 82411300: 1D670038  mulli r11, r7, 0x38
	ctx.r[11].s32 = ((ctx.r[7].s32 as i64 * 56 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82411304: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82411308: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241130C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411310: 4BFFFB39  bl 0x82410e48
	ctx.lr = 0x82411314;
	sub_82410E48(ctx, base);
	// 82411314: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 82411318: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241131C: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411320: 48123831  bl 0x82534b50
	ctx.lr = 0x82411324;
	sub_82534B50(ctx, base);
	// 82411324: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411328: 4BFFFF58  b 0x82411280
	pc = 0x82411280; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411330 size=4
    let mut pc: u32 = 0x82411330;
    'dispatch: loop {
        match pc {
            0x82411330 => {
    //   block [0x82411330..0x82411334)
	// 82411330: 4BFFFDD0  b 0x82411100
	sub_82411100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411338 size=8
    let mut pc: u32 = 0x82411338;
    'dispatch: loop {
        match pc {
            0x82411338 => {
    //   block [0x82411338..0x82411340)
	// 82411338: 48003DE0  b 0x82415118
	sub_82415118(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411340 size=220
    let mut pc: u32 = 0x82411340;
    'dispatch: loop {
        match pc {
            0x82411340 => {
    //   block [0x82411340..0x82411394)
	// 82411340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411344: 48123D75  bl 0x825350b8
	ctx.lr = 0x82411348;
	sub_82535080(ctx, base);
	// 82411348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241134C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411350: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82411354: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82411358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241135C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411360: 409A005C  bne cr6, 0x824113bc
	if !ctx.cr[6].eq {
	pc = 0x824113BC; continue 'dispatch;
	}
	// 82411364: 4BFFF45D  bl 0x824107c0
	ctx.lr = 0x82411368;
	sub_824107C0(ctx, base);
	// 82411368: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241136C: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82411370: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82411374: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411378: 4E800421  bctrl
	ctx.lr = 0x8241137C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241137C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411380: 4182002C  beq 0x824113ac
	if ctx.cr[0].eq {
	pc = 0x824113AC; continue 'dispatch;
	}
	// 82411384: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82411388: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8241138C: 3940000D  li r10, 0xd
	ctx.r[10].s64 = 13;
	// 82411390: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
            }
            0x82411394 => {
    //   block [0x82411394..0x824113AC)
	// 82411394: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82411398: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8241139C: 4200FFF8  bdnz 0x82411394
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82411394; continue 'dispatch;
	}
	// 824113A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824113A4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824113A8: 48000008  b 0x824113b0
	pc = 0x824113B0; continue 'dispatch;
            }
            0x824113AC => {
    //   block [0x824113AC..0x824113B0)
	// 824113AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x824113B0; continue 'dispatch;
            }
            0x824113B0 => {
    //   block [0x824113B0..0x824113BC)
	// 824113B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824113B4: 48003D65  bl 0x82415118
	ctx.lr = 0x824113B8;
	sub_82415118(ctx, base);
	// 824113B8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	pc = 0x824113BC; continue 'dispatch;
            }
            0x824113BC => {
    //   block [0x824113BC..0x824113D0)
	// 824113BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824113C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824113C4: 3940000D  li r10, 0xd
	ctx.r[10].s64 = 13;
	// 824113C8: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 824113CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x824113D0; continue 'dispatch;
            }
            0x824113D0 => {
    //   block [0x824113D0..0x824113F0)
	// 824113D0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824113D4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824113D8: 4200FFF8  bdnz 0x824113d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824113D0; continue 'dispatch;
	}
	// 824113DC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824113E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824113E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824113E8: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 824113EC: 40980008  bge cr6, 0x824113f4
	if !ctx.cr[6].lt {
	pc = 0x824113F4; continue 'dispatch;
	}
	pc = 0x824113F0; continue 'dispatch;
            }
            0x824113F0 => {
    //   block [0x824113F0..0x824113F4)
	// 824113F0: 48000000  b 0x824113f0
	pc = 0x824113F0; continue 'dispatch;
            }
            0x824113F4 => {
    //   block [0x824113F4..0x824113FC)
	// 824113F4: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 824113F8: 41980008  blt cr6, 0x82411400
	if ctx.cr[6].lt {
	pc = 0x82411400; continue 'dispatch;
	}
	pc = 0x824113FC; continue 'dispatch;
            }
            0x824113FC => {
    //   block [0x824113FC..0x82411400)
	// 824113FC: 48000000  b 0x824113fc
	pc = 0x824113FC; continue 'dispatch;
            }
            0x82411400 => {
    //   block [0x82411400..0x8241141C)
	// 82411400: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82411404: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82411408: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8241140C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82411410: 7F89592E  stwx r28, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 82411414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82411418: 48123CF0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411420 size=60
    let mut pc: u32 = 0x82411420;
    'dispatch: loop {
        match pc {
            0x82411420 => {
    //   block [0x82411420..0x8241145C)
	// 82411420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241142C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411430: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411434: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411438: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241143C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411440: 4BFFFF01  bl 0x82411340
	ctx.lr = 0x82411444;
	sub_82411340(ctx, base);
	// 82411444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411460 size=68
    let mut pc: u32 = 0x82411460;
    'dispatch: loop {
        match pc {
            0x82411460 => {
    //   block [0x82411460..0x8241148C)
	// 82411460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241146C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411470: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411478: 396BEB50  addi r11, r11, -0x14b0
	ctx.r[11].s64 = ctx.r[11].s64 + -5296;
	// 8241147C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82411480: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411484: 41820008  beq 0x8241148c
	if ctx.cr[0].eq {
	pc = 0x8241148C; continue 'dispatch;
	}
	// 82411488: 48121731  bl 0x82532bb8
	ctx.lr = 0x8241148C;
	sub_82532BB8(ctx, base);
	pc = 0x8241148C; continue 'dispatch;
            }
            0x8241148C => {
    //   block [0x8241148C..0x824114A4)
	// 8241148C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241149C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824114A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824114A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824114A8 size=68
    let mut pc: u32 = 0x824114A8;
    'dispatch: loop {
        match pc {
            0x824114A8 => {
    //   block [0x824114A8..0x824114D4)
	// 824114A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824114AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824114B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824114B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824114B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824114BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824114C0: 396BEB58  addi r11, r11, -0x14a8
	ctx.r[11].s64 = ctx.r[11].s64 + -5288;
	// 824114C4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824114C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824114CC: 41820008  beq 0x824114d4
	if ctx.cr[0].eq {
	pc = 0x824114D4; continue 'dispatch;
	}
	// 824114D0: 481216E9  bl 0x82532bb8
	ctx.lr = 0x824114D4;
	sub_82532BB8(ctx, base);
	pc = 0x824114D4; continue 'dispatch;
            }
            0x824114D4 => {
    //   block [0x824114D4..0x824114EC)
	// 824114D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824114D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824114DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824114E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824114E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824114E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824114F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824114F0 size=40
    let mut pc: u32 = 0x824114F0;
    'dispatch: loop {
        match pc {
            0x824114F0 => {
    //   block [0x824114F0..0x82411518)
	// 824114F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824114F4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824114F8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824114FC: 396BEB6C  addi r11, r11, -0x1494
	ctx.r[11].s64 = ctx.r[11].s64 + -5268;
	// 82411500: 394AEB58  addi r10, r10, -0x14a8
	ctx.r[10].s64 = ctx.r[10].s64 + -5288;
	// 82411504: 3929EB50  addi r9, r9, -0x14b0
	ctx.r[9].s64 = ctx.r[9].s64 + -5296;
	// 82411508: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8241150C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82411510: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82411514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411518 size=8
    let mut pc: u32 = 0x82411518;
    'dispatch: loop {
        match pc {
            0x82411518 => {
    //   block [0x82411518..0x82411520)
	// 82411518: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411520 size=136
    let mut pc: u32 = 0x82411520;
    'dispatch: loop {
        match pc {
            0x82411520 => {
    //   block [0x82411520..0x8241153C)
	// 82411520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411524: 48123B99  bl 0x825350bc
	ctx.lr = 0x82411528;
	sub_82535080(ctx, base);
	// 82411528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241152C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411534: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82411538: 409A0008  bne cr6, 0x82411540
	if !ctx.cr[6].eq {
	pc = 0x82411540; continue 'dispatch;
	}
	pc = 0x8241153C; continue 'dispatch;
            }
            0x8241153C => {
    //   block [0x8241153C..0x82411540)
	// 8241153C: 48000000  b 0x8241153c
	pc = 0x8241153C; continue 'dispatch;
            }
            0x82411540 => {
    //   block [0x82411540..0x8241159C)
	// 82411540: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82411544: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82411548: 419A0054  beq cr6, 0x8241159c
	if ctx.cr[6].eq {
	pc = 0x8241159C; continue 'dispatch;
	}
	// 8241154C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411550: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 82411554: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82411558: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8241155C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411560: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82411564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411568: 4E800421  bctrl
	ctx.lr = 0x8241156C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241156C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411570: 4182002C  beq 0x8241159c
	if ctx.cr[0].eq {
	pc = 0x8241159C; continue 'dispatch;
	}
	// 82411574: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411578: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241157C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82411580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411584: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241158C: 4E800421  bctrl
	ctx.lr = 0x82411590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411590: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411594: 4182000C  beq 0x824115a0
	if ctx.cr[0].eq {
	pc = 0x824115A0; continue 'dispatch;
	}
	// 82411598: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
            }
            0x8241159C => {
    //   block [0x8241159C..0x824115A0)
	// 8241159C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x824115A0; continue 'dispatch;
            }
            0x824115A0 => {
    //   block [0x824115A0..0x824115A8)
	// 824115A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824115A4: 48123B68  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824115C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824115C0 size=96
    let mut pc: u32 = 0x824115C0;
    'dispatch: loop {
        match pc {
            0x824115C0 => {
    //   block [0x824115C0..0x82411608)
	// 824115C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824115C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824115C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824115CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824115D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824115D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824115D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824115DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824115E0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824115E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824115E8: 4E800421  bctrl
	ctx.lr = 0x824115EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824115EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824115F0: 41820018  beq 0x82411608
	if ctx.cr[0].eq {
	pc = 0x82411608; continue 'dispatch;
	}
	// 824115F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824115F8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 824115FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411600: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82411604: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
            }
            0x82411608 => {
    //   block [0x82411608..0x82411620)
	// 82411608: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241160C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411614: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241161C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82411620 size=288
    let mut pc: u32 = 0x82411620;
    'dispatch: loop {
        match pc {
            0x82411620 => {
    //   block [0x82411620..0x82411668)
	// 82411620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241162C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411638: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241163C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82411640: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411644: 41820028  beq 0x8241166c
	if ctx.cr[0].eq {
	pc = 0x8241166C; continue 'dispatch;
	}
	// 82411648: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8241164C: 419A0020  beq cr6, 0x8241166c
	if ctx.cr[6].eq {
	pc = 0x8241166C; continue 'dispatch;
	}
	// 82411650: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82411654: 419A0048  beq cr6, 0x8241169c
	if ctx.cr[6].eq {
	pc = 0x8241169C; continue 'dispatch;
	}
	// 82411658: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 8241165C: 419A0080  beq cr6, 0x824116dc
	if ctx.cr[6].eq {
	pc = 0x824116DC; continue 'dispatch;
	}
	// 82411660: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82411664: 419A00BC  beq cr6, 0x82411720
	if ctx.cr[6].eq {
	pc = 0x82411720; continue 'dispatch;
	}
	pc = 0x82411668; continue 'dispatch;
            }
            0x82411668 => {
    //   block [0x82411668..0x8241166C)
	// 82411668: 48000000  b 0x82411668
	pc = 0x82411668; continue 'dispatch;
            }
            0x8241166C => {
    //   block [0x8241166C..0x8241169C)
	// 8241166C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82411670: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82411674: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82411678: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241167C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82411680: 409A001C  bne cr6, 0x8241169c
	if !ctx.cr[6].eq {
	pc = 0x8241169C; continue 'dispatch;
	}
	// 82411684: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241168C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82411690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411694: 4E800421  bctrl
	ctx.lr = 0x82411698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411698: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
            }
            0x8241169C => {
    //   block [0x8241169C..0x824116DC)
	// 8241169C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824116A0: C01F0024  lfs f0, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824116A4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 824116A8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824116AC: 80BF0028  lwz r5, 0x28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824116B0: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 824116B4: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 824116B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824116BC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824116C0: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824116C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824116C8: 4E800421  bctrl
	ctx.lr = 0x824116CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824116CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824116D0: 41820054  beq 0x82411724
	if ctx.cr[0].eq {
	pc = 0x82411724; continue 'dispatch;
	}
	// 824116D4: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 824116D8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
            }
            0x824116DC => {
    //   block [0x824116DC..0x82411720)
	// 824116DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824116E0: C01F0024  lfs f0, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824116E4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 824116E8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824116EC: 80BF0028  lwz r5, 0x28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824116F0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824116F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824116F8: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 824116FC: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411700: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82411704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411708: 4E800421  bctrl
	ctx.lr = 0x8241170C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241170C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411710: 41820014  beq 0x82411724
	if ctx.cr[0].eq {
	pc = 0x82411724; continue 'dispatch;
	}
	// 82411714: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82411718: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8241171C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
            }
            0x82411720 => {
    //   block [0x82411720..0x82411724)
	// 82411720: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	pc = 0x82411724; continue 'dispatch;
            }
            0x82411724 => {
    //   block [0x82411724..0x82411740)
	// 82411724: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82411728: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241172C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411734: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411738: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241173C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82411740 size=204
    let mut pc: u32 = 0x82411740;
    'dispatch: loop {
        match pc {
            0x82411740 => {
    //   block [0x82411740..0x8241178C)
	// 82411740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241174C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411750: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82411754: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241175C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82411760: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82411764: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82411768: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 8241176C: 419A0020  beq cr6, 0x8241178c
	if ctx.cr[6].eq {
	pc = 0x8241178C; continue 'dispatch;
	}
	// 82411770: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82411774: 419A0044  beq cr6, 0x824117b8
	if ctx.cr[6].eq {
	pc = 0x824117B8; continue 'dispatch;
	}
	// 82411778: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 8241177C: 419A006C  beq cr6, 0x824117e8
	if ctx.cr[6].eq {
	pc = 0x824117E8; continue 'dispatch;
	}
	// 82411780: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82411784: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82411788: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	pc = 0x8241178C; continue 'dispatch;
            }
            0x8241178C => {
    //   block [0x8241178C..0x824117B8)
	// 8241178C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411790: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82411794: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82411798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241179C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824117A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824117A4: 4E800421  bctrl
	ctx.lr = 0x824117A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824117A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824117AC: 41820040  beq 0x824117ec
	if ctx.cr[0].eq {
	pc = 0x824117EC; continue 'dispatch;
	}
	// 824117B0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 824117B4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
            }
            0x824117B8 => {
    //   block [0x824117B8..0x824117E8)
	// 824117B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824117BC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824117C0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824117C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824117C8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 824117CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824117D0: 4E800421  bctrl
	ctx.lr = 0x824117D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824117D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824117D8: 41820014  beq 0x824117ec
	if ctx.cr[0].eq {
	pc = 0x824117EC; continue 'dispatch;
	}
	// 824117DC: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 824117E0: D3FF002C  stfs f31, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824117E4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
            }
            0x824117E8 => {
    //   block [0x824117E8..0x824117EC)
	// 824117E8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	pc = 0x824117EC; continue 'dispatch;
            }
            0x824117EC => {
    //   block [0x824117EC..0x8241180C)
	// 824117EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824117F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824117F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824117F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824117FC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82411800: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82411810 size=132
    let mut pc: u32 = 0x82411810;
    'dispatch: loop {
        match pc {
            0x82411810 => {
    //   block [0x82411810..0x82411838)
	// 82411810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241181C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411824: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82411828: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 8241182C: 419A0010  beq cr6, 0x8241183c
	if ctx.cr[6].eq {
	pc = 0x8241183C; continue 'dispatch;
	}
	// 82411830: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82411834: 419A0018  beq cr6, 0x8241184c
	if ctx.cr[6].eq {
	pc = 0x8241184C; continue 'dispatch;
	}
	pc = 0x82411838; continue 'dispatch;
            }
            0x82411838 => {
    //   block [0x82411838..0x8241183C)
	// 82411838: 48000000  b 0x82411838
	pc = 0x82411838; continue 'dispatch;
            }
            0x8241183C => {
    //   block [0x8241183C..0x8241184C)
	// 8241183C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411840: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82411844: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82411848: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	pc = 0x8241184C; continue 'dispatch;
            }
            0x8241184C => {
    //   block [0x8241184C..0x82411894)
	// 8241184C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411850: C05F002C  lfs f2, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82411854: C03F0024  lfs f1, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82411858: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8241185C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411860: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82411864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411868: 4E800421  bctrl
	ctx.lr = 0x8241186C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241186C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411874: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82411878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241187C: 4E800421  bctrl
	ctx.lr = 0x82411880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241188C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824118C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824118C8 size=100
    let mut pc: u32 = 0x824118C8;
    'dispatch: loop {
        match pc {
            0x824118C8 => {
    //   block [0x824118C8..0x8241192C)
	// 824118C8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824118CC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824118D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824118D4: 396BEB58  addi r11, r11, -0x14a8
	ctx.r[11].s64 = ctx.r[11].s64 + -5288;
	// 824118D8: C00A1850  lfs f0, 0x1850(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824118DC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824118E0: C1A91FF8  lfs f13, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824118E4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824118E8: 394AEB80  addi r10, r10, -0x1480
	ctx.r[10].s64 = ctx.r[10].s64 + -5248;
	// 824118EC: 3929EB6C  addi r9, r9, -0x1494
	ctx.r[9].s64 = ctx.r[9].s64 + -5268;
	// 824118F0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824118F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824118F8: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824118FC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82411900: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82411904: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82411908: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 8241190C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82411910: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82411914: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82411918: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8241191C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82411920: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82411924: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82411928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411930 size=112
    let mut pc: u32 = 0x82411930;
    'dispatch: loop {
        match pc {
            0x82411930 => {
    //   block [0x82411930..0x82411988)
	// 82411930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241193C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411940: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411944: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82411948: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8241194C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411950: 396BEB6C  addi r11, r11, -0x1494
	ctx.r[11].s64 = ctx.r[11].s64 + -5268;
	// 82411954: 394AEB58  addi r10, r10, -0x14a8
	ctx.r[10].s64 = ctx.r[10].s64 + -5288;
	// 82411958: 3929EB50  addi r9, r9, -0x14b0
	ctx.r[9].s64 = ctx.r[9].s64 + -5296;
	// 8241195C: 548807FF  clrlwi. r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82411960: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82411964: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82411968: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8241196C: 4182001C  beq 0x82411988
	if ctx.cr[0].eq {
	pc = 0x82411988; continue 'dispatch;
	}
	// 82411970: 4BFFEE51  bl 0x824107c0
	ctx.lr = 0x82411974;
	sub_824107C0(ctx, base);
	// 82411974: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411978: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241197C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411984: 4E800421  bctrl
	ctx.lr = 0x82411988;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82411988 => {
    //   block [0x82411988..0x824119A0)
	// 82411988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241198C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241199C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824119A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824119A0 size=232
    let mut pc: u32 = 0x824119A0;
    'dispatch: loop {
        match pc {
            0x824119A0 => {
    //   block [0x824119A0..0x824119F4)
	// 824119A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824119A4: 48123719  bl 0x825350bc
	ctx.lr = 0x824119A8;
	sub_82535080(ctx, base);
	// 824119A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824119AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824119B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824119B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824119B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824119BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824119C0: 41820038  beq 0x824119f8
	if ctx.cr[0].eq {
	pc = 0x824119F8; continue 'dispatch;
	}
	// 824119C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824119C8: 419A003C  beq cr6, 0x82411a04
	if ctx.cr[6].eq {
	pc = 0x82411A04; continue 'dispatch;
	}
	// 824119CC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 824119D0: 419A005C  beq cr6, 0x82411a2c
	if ctx.cr[6].eq {
	pc = 0x82411A2C; continue 'dispatch;
	}
	// 824119D4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 824119D8: 419A00A0  beq cr6, 0x82411a78
	if ctx.cr[6].eq {
	pc = 0x82411A78; continue 'dispatch;
	}
	// 824119DC: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 824119E0: 419A0018  beq cr6, 0x824119f8
	if ctx.cr[6].eq {
	pc = 0x824119F8; continue 'dispatch;
	}
	// 824119E4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 824119E8: 4099000C  ble cr6, 0x824119f4
	if !ctx.cr[6].gt {
	pc = 0x824119F4; continue 'dispatch;
	}
	// 824119EC: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 824119F0: 40990008  ble cr6, 0x824119f8
	if !ctx.cr[6].gt {
	pc = 0x824119F8; continue 'dispatch;
	}
	pc = 0x824119F4; continue 'dispatch;
            }
            0x824119F4 => {
    //   block [0x824119F4..0x824119F8)
	// 824119F4: 48000000  b 0x824119f4
	pc = 0x824119F4; continue 'dispatch;
            }
            0x824119F8 => {
    //   block [0x824119F8..0x82411A04)
	// 824119F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824119FC: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82411A00: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	pc = 0x82411A04; continue 'dispatch;
            }
            0x82411A04 => {
    //   block [0x82411A04..0x82411A2C)
	// 82411A04: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411A08: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82411A0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411A10: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82411A14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411A18: 4E800421  bctrl
	ctx.lr = 0x82411A1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411A1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411A20: 4182005C  beq 0x82411a7c
	if ctx.cr[0].eq {
	pc = 0x82411A7C; continue 'dispatch;
	}
	// 82411A24: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82411A28: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
            }
            0x82411A2C => {
    //   block [0x82411A2C..0x82411A64)
	// 82411A2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411A30: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82411A34: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82411A38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411A3C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82411A40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411A44: 4E800421  bctrl
	ctx.lr = 0x82411A48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411A48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411A4C: 41820030  beq 0x82411a7c
	if ctx.cr[0].eq {
	pc = 0x82411A7C; continue 'dispatch;
	}
	// 82411A50: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82411A54: 419A0010  beq cr6, 0x82411a64
	if ctx.cr[6].eq {
	pc = 0x82411A64; continue 'dispatch;
	}
	// 82411A58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82411A5C: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82411A60: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
            }
            0x82411A64 => {
    //   block [0x82411A64..0x82411A78)
	// 82411A64: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82411A68: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82411A6C: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82411A70: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82411A74: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	pc = 0x82411A78; continue 'dispatch;
            }
            0x82411A78 => {
    //   block [0x82411A78..0x82411A7C)
	// 82411A78: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	pc = 0x82411A7C; continue 'dispatch;
            }
            0x82411A7C => {
    //   block [0x82411A7C..0x82411A88)
	// 82411A7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82411A80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411A84: 48123688  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411A88 size=92
    let mut pc: u32 = 0x82411A88;
    'dispatch: loop {
        match pc {
            0x82411A88 => {
    //   block [0x82411A88..0x82411AC8)
	// 82411A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411A98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411A9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411AA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411AA4: 48000CDD  bl 0x82412780
	ctx.lr = 0x82411AA8;
	sub_82412780(ctx, base);
	// 82411AA8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411AAC: 4182001C  beq 0x82411ac8
	if ctx.cr[0].eq {
	pc = 0x82411AC8; continue 'dispatch;
	}
	// 82411AB0: 4BFFED11  bl 0x824107c0
	ctx.lr = 0x82411AB4;
	sub_824107C0(ctx, base);
	// 82411AB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411AB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411ABC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411AC4: 4E800421  bctrl
	ctx.lr = 0x82411AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82411AC8 => {
    //   block [0x82411AC8..0x82411AE4)
	// 82411AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411ACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411AD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411AE8 size=8
    let mut pc: u32 = 0x82411AE8;
    'dispatch: loop {
        match pc {
            0x82411AE8 => {
    //   block [0x82411AE8..0x82411AF0)
	// 82411AE8: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411AEC: 48000584  b 0x82412070
	sub_82412070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411AF0 size=8
    let mut pc: u32 = 0x82411AF0;
    'dispatch: loop {
        match pc {
            0x82411AF0 => {
    //   block [0x82411AF0..0x82411AF8)
	// 82411AF0: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82411AF4: 48000AAC  b 0x824125a0
	sub_824125A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411AF8 size=8
    let mut pc: u32 = 0x82411AF8;
    'dispatch: loop {
        match pc {
            0x82411AF8 => {
    //   block [0x82411AF8..0x82411B00)
	// 82411AF8: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82411AFC: 48000AF4  b 0x824125f0
	sub_824125F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B00 size=8
    let mut pc: u32 = 0x82411B00;
    'dispatch: loop {
        match pc {
            0x82411B00 => {
    //   block [0x82411B00..0x82411B08)
	// 82411B00: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82411B04: 48000B3C  b 0x82412640
	sub_82412640(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B08 size=8
    let mut pc: u32 = 0x82411B08;
    'dispatch: loop {
        match pc {
            0x82411B08 => {
    //   block [0x82411B08..0x82411B10)
	// 82411B08: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82411B0C: 48000B84  b 0x82412690
	sub_82412690(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B10 size=8
    let mut pc: u32 = 0x82411B10;
    'dispatch: loop {
        match pc {
            0x82411B10 => {
    //   block [0x82411B10..0x82411B18)
	// 82411B10: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B14: 480006FC  b 0x82412210
	crate::recompiler::externs::call(ctx, base, 0x82412210);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B18 size=8
    let mut pc: u32 = 0x82411B18;
    'dispatch: loop {
        match pc {
            0x82411B18 => {
    //   block [0x82411B18..0x82411B20)
	// 82411B18: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B1C: 48001004  b 0x82412b20
	sub_82412B20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B20 size=8
    let mut pc: u32 = 0x82411B20;
    'dispatch: loop {
        match pc {
            0x82411B20 => {
    //   block [0x82411B20..0x82411B28)
	// 82411B20: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B24: 48001024  b 0x82412b48
	sub_82412B48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82411B28 size=16
    let mut pc: u32 = 0x82411B28;
    'dispatch: loop {
        match pc {
            0x82411B28 => {
    //   block [0x82411B28..0x82411B38)
	// 82411B28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82411B2C: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B30: C02B0024  lfs f1, 0x24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82411B34: 4800103C  b 0x82412b70
	sub_82412B70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411B38 size=44
    let mut pc: u32 = 0x82411B38;
    'dispatch: loop {
        match pc {
            0x82411B38 => {
    //   block [0x82411B38..0x82411B64)
	// 82411B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411B44: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82411B48: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B4C: 48000CCD  bl 0x82412818
	ctx.lr = 0x82411B50;
	sub_82412818(ctx, base);
	// 82411B50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411B54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82411B68 size=96
    let mut pc: u32 = 0x82411B68;
    'dispatch: loop {
        match pc {
            0x82411B68 => {
    //   block [0x82411B68..0x82411B9C)
	// 82411B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411B78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411B80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411B84: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B88: 48000F99  bl 0x82412b20
	ctx.lr = 0x82411B8C;
	sub_82412B20(ctx, base);
	// 82411B8C: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82411B90: 4098000C  bge cr6, 0x82411b9c
	if !ctx.cr[6].lt {
	pc = 0x82411B9C; continue 'dispatch;
	}
	// 82411B94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82411B98: 48000018  b 0x82411bb0
	pc = 0x82411BB0; continue 'dispatch;
            }
            0x82411B9C => {
    //   block [0x82411B9C..0x82411BB0)
	// 82411B9C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411BA0: 48000FA9  bl 0x82412b48
	ctx.lr = 0x82411BA4;
	sub_82412B48(ctx, base);
	// 82411BA4: 7D7E1810  subfc r11, r30, r3
	ctx.xer.ca = ctx.r[3].u32 >= ctx.r[30].u32;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 82411BA8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82411BAC: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	pc = 0x82411BB0; continue 'dispatch;
            }
            0x82411BB0 => {
    //   block [0x82411BB0..0x82411BC8)
	// 82411BB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411BBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411BC8 size=48
    let mut pc: u32 = 0x82411BC8;
    'dispatch: loop {
        match pc {
            0x82411BC8 => {
    //   block [0x82411BC8..0x82411BF8)
	// 82411BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411BD4: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411BD8: 48000641  bl 0x82412218
	ctx.lr = 0x82411BDC;
	sub_82412218(ctx, base);
	// 82411BDC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82411BE0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82411BE4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82411BE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411BF8 size=48
    let mut pc: u32 = 0x82411BF8;
    'dispatch: loop {
        match pc {
            0x82411BF8 => {
    //   block [0x82411BF8..0x82411C28)
	// 82411BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411C04: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411C08: 48000689  bl 0x82412290
	ctx.lr = 0x82411C0C;
	sub_82412290(ctx, base);
	// 82411C0C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82411C10: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82411C14: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82411C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411C28 size=48
    let mut pc: u32 = 0x82411C28;
    'dispatch: loop {
        match pc {
            0x82411C28 => {
    //   block [0x82411C28..0x82411C58)
	// 82411C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411C34: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411C38: 48000EE1  bl 0x82412b18
	ctx.lr = 0x82411C3C;
	sub_82412B18(ctx, base);
	// 82411C3C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82411C40: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82411C44: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82411C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411C58 size=164
    let mut pc: u32 = 0x82411C58;
    'dispatch: loop {
        match pc {
            0x82411C58 => {
    //   block [0x82411C58..0x82411C8C)
	// 82411C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411C5C: 48123461  bl 0x825350bc
	ctx.lr = 0x82411C60;
	sub_82535080(ctx, base);
	// 82411C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411C64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411C6C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82411C70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411C74: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82411C78: 41980014  blt cr6, 0x82411c8c
	if ctx.cr[6].lt {
	pc = 0x82411C8C; continue 'dispatch;
	}
	// 82411C7C: 419A0034  beq cr6, 0x82411cb0
	if ctx.cr[6].eq {
	pc = 0x82411CB0; continue 'dispatch;
	}
	// 82411C80: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82411C84: 41980048  blt cr6, 0x82411ccc
	if ctx.cr[6].lt {
	pc = 0x82411CCC; continue 'dispatch;
	}
	// 82411C88: 48000068  b 0x82411cf0
	pc = 0x82411CF0; continue 'dispatch;
            }
            0x82411C8C => {
    //   block [0x82411C8C..0x82411CA0)
	// 82411C8C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82411C90: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411C94: 48000BF5  bl 0x82412888
	ctx.lr = 0x82411C98;
	sub_82412888(ctx, base);
	// 82411C98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411C9C: 40820008  bne 0x82411ca4
	if !ctx.cr[0].eq {
	pc = 0x82411CA4; continue 'dispatch;
	}
	pc = 0x82411CA0; continue 'dispatch;
            }
            0x82411CA0 => {
    //   block [0x82411CA0..0x82411CA4)
	// 82411CA0: 48000000  b 0x82411ca0
	pc = 0x82411CA0; continue 'dispatch;
            }
            0x82411CA4 => {
    //   block [0x82411CA4..0x82411CB0)
	// 82411CA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411CA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411CAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82411CB0; continue 'dispatch;
            }
            0x82411CB0 => {
    //   block [0x82411CB0..0x82411CCC)
	// 82411CB0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411CB4: 48000C4D  bl 0x82412900
	ctx.lr = 0x82411CB8;
	sub_82412900(ctx, base);
	// 82411CB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411CBC: 41820038  beq 0x82411cf4
	if ctx.cr[0].eq {
	pc = 0x82411CF4; continue 'dispatch;
	}
	// 82411CC0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411CC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411CC8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82411CCC; continue 'dispatch;
            }
            0x82411CCC => {
    //   block [0x82411CCC..0x82411CE0)
	// 82411CCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82411CD0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411CD4: 48000C55  bl 0x82412928
	ctx.lr = 0x82411CD8;
	sub_82412928(ctx, base);
	// 82411CD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411CDC: 40820008  bne 0x82411ce4
	if !ctx.cr[0].eq {
	pc = 0x82411CE4; continue 'dispatch;
	}
	pc = 0x82411CE0; continue 'dispatch;
            }
            0x82411CE0 => {
    //   block [0x82411CE0..0x82411CE4)
	// 82411CE0: 48000000  b 0x82411ce0
	pc = 0x82411CE0; continue 'dispatch;
            }
            0x82411CE4 => {
    //   block [0x82411CE4..0x82411CF0)
	// 82411CE4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411CE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411CEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82411CF0; continue 'dispatch;
            }
            0x82411CF0 => {
    //   block [0x82411CF0..0x82411CF4)
	// 82411CF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82411CF4; continue 'dispatch;
            }
            0x82411CF4 => {
    //   block [0x82411CF4..0x82411CFC)
	// 82411CF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411CF8: 48123414  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411D00 size=40
    let mut pc: u32 = 0x82411D00;
    'dispatch: loop {
        match pc {
            0x82411D00 => {
    //   block [0x82411D00..0x82411D28)
	// 82411D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411D0C: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D10: 480004B9  bl 0x824121c8
	ctx.lr = 0x82411D14;
	sub_824121C8(ctx, base);
	// 82411D14: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D28 size=8
    let mut pc: u32 = 0x82411D28;
    'dispatch: loop {
        match pc {
            0x82411D28 => {
    //   block [0x82411D28..0x82411D30)
	// 82411D28: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D2C: 48000C34  b 0x82412960
	crate::recompiler::externs::call(ctx, base, 0x82412960);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D30 size=8
    let mut pc: u32 = 0x82411D30;
    'dispatch: loop {
        match pc {
            0x82411D30 => {
    //   block [0x82411D30..0x82411D38)
	// 82411D30: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D34: 48000DBC  b 0x82412af0
	sub_82412AF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D38 size=8
    let mut pc: u32 = 0x82411D38;
    'dispatch: loop {
        match pc {
            0x82411D38 => {
    //   block [0x82411D38..0x82411D40)
	// 82411D38: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D3C: 480005CC  b 0x82412308
	sub_82412308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D40 size=8
    let mut pc: u32 = 0x82411D40;
    'dispatch: loop {
        match pc {
            0x82411D40 => {
    //   block [0x82411D40..0x82411D48)
	// 82411D40: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D44: 48000644  b 0x82412388
	sub_82412388(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82411D48 size=28
    let mut pc: u32 = 0x82411D48;
    'dispatch: loop {
        match pc {
            0x82411D48 => {
    //   block [0x82411D48..0x82411D64)
	// 82411D48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82411D4C: 38AB002C  addi r5, r11, 0x2c
	ctx.r[5].s64 = ctx.r[11].s64 + 44;
	// 82411D50: 388B0024  addi r4, r11, 0x24
	ctx.r[4].s64 = ctx.r[11].s64 + 36;
	// 82411D54: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411D5C: C02BEBCC  lfs f1, -0x1434(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5172 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82411D60: 48000C08  b 0x82412968
	sub_82412968(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D68 size=8
    let mut pc: u32 = 0x82411D68;
    'dispatch: loop {
        match pc {
            0x82411D68 => {
    //   block [0x82411D68..0x82411D70)
	// 82411D68: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D6C: 4800030C  b 0x82412078
	sub_82412070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411D70 size=84
    let mut pc: u32 = 0x82411D70;
    'dispatch: loop {
        match pc {
            0x82411D70 => {
    //   block [0x82411D70..0x82411DA8)
	// 82411D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411D78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411D7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411D80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411D84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411D88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411D8C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411D90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411D94: 41820014  beq 0x82411da8
	if ctx.cr[0].eq {
	pc = 0x82411DA8; continue 'dispatch;
	}
	// 82411D98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82411D9C: 4BFFFCED  bl 0x82411a88
	ctx.lr = 0x82411DA0;
	sub_82411A88(ctx, base);
	// 82411DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411DA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82411DA8; continue 'dispatch;
            }
            0x82411DA8 => {
    //   block [0x82411DA8..0x82411DC4)
	// 82411DA8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82411DAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411DB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411DBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411DC8 size=168
    let mut pc: u32 = 0x82411DC8;
    'dispatch: loop {
        match pc {
            0x82411DC8 => {
    //   block [0x82411DC8..0x82411E50)
	// 82411DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411DCC: 481232ED  bl 0x825350b8
	ctx.lr = 0x82411DD0;
	sub_82535080(ctx, base);
	// 82411DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411DD4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82411DD8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82411DDC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82411DE0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82411DE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82411DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411DEC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82411DF0: 4BFFFAD9  bl 0x824118c8
	ctx.lr = 0x82411DF4;
	sub_824118C8(ctx, base);
	// 82411DF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411DF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82411DFC: 396BEBE8  addi r11, r11, -0x1418
	ctx.r[11].s64 = ctx.r[11].s64 + -5144;
	// 82411E00: 394AEBD0  addi r10, r10, -0x1430
	ctx.r[10].s64 = ctx.r[10].s64 + -5168;
	// 82411E04: 3B9F0030  addi r28, r31, 0x30
	ctx.r[28].s64 = ctx.r[31].s64 + 48;
	// 82411E08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411E0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411E10: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82411E14: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82411E18: 4BFFE9A9  bl 0x824107c0
	ctx.lr = 0x82411E1C;
	sub_824107C0(ctx, base);
	// 82411E1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411E20: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 82411E24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82411E28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411E2C: 4E800421  bctrl
	ctx.lr = 0x82411E30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411E30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411E34: 4182001C  beq 0x82411e50
	if ctx.cr[0].eq {
	pc = 0x82411E50; continue 'dispatch;
	}
	// 82411E38: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82411E3C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82411E40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411E44: 4800089D  bl 0x824126e0
	ctx.lr = 0x82411E48;
	sub_824126E0(ctx, base);
	// 82411E48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82411E4C: 48000008  b 0x82411e54
	pc = 0x82411E54; continue 'dispatch;
            }
            0x82411E50 => {
    //   block [0x82411E50..0x82411E54)
	// 82411E50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	pc = 0x82411E54; continue 'dispatch;
            }
            0x82411E54 => {
    //   block [0x82411E54..0x82411E70)
	// 82411E54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82411E58: 4BFFFF19  bl 0x82411d70
	ctx.lr = 0x82411E5C;
	sub_82411D70(ctx, base);
	// 82411E5C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411E60: 48001029  bl 0x82412e88
	ctx.lr = 0x82411E64;
	sub_82412E88(ctx, base);
	// 82411E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82411E6C: 4812329C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411E78 size=100
    let mut pc: u32 = 0x82411E78;
    'dispatch: loop {
        match pc {
            0x82411E78 => {
    //   block [0x82411E78..0x82411EC0)
	// 82411E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411E88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411E8C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82411E90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411E94: 396BEBE8  addi r11, r11, -0x1418
	ctx.r[11].s64 = ctx.r[11].s64 + -5144;
	// 82411E98: 394AEBD0  addi r10, r10, -0x1430
	ctx.r[10].s64 = ctx.r[10].s64 + -5168;
	// 82411E9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411EA0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82411EA4: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411EA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411EAC: 41820014  beq 0x82411ec0
	if ctx.cr[0].eq {
	pc = 0x82411EC0; continue 'dispatch;
	}
	// 82411EB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82411EB4: 4BFFFBD5  bl 0x82411a88
	ctx.lr = 0x82411EB8;
	sub_82411A88(ctx, base);
	// 82411EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411EBC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	pc = 0x82411EC0; continue 'dispatch;
            }
            0x82411EC0 => {
    //   block [0x82411EC0..0x82411EDC)
	// 82411EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411EC4: 4BFFF62D  bl 0x824114f0
	ctx.lr = 0x82411EC8;
	sub_824114F0(ctx, base);
	// 82411EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411EE0 size=92
    let mut pc: u32 = 0x82411EE0;
    'dispatch: loop {
        match pc {
            0x82411EE0 => {
    //   block [0x82411EE0..0x82411F20)
	// 82411EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411EF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411EF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411EFC: 4BFFFF7D  bl 0x82411e78
	ctx.lr = 0x82411F00;
	sub_82411E78(ctx, base);
	// 82411F00: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411F04: 4182001C  beq 0x82411f20
	if ctx.cr[0].eq {
	pc = 0x82411F20; continue 'dispatch;
	}
	// 82411F08: 4BFFE8B9  bl 0x824107c0
	ctx.lr = 0x82411F0C;
	sub_824107C0(ctx, base);
	// 82411F0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411F10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411F14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411F18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411F1C: 4E800421  bctrl
	ctx.lr = 0x82411F20;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82411F20 => {
    //   block [0x82411F20..0x82411F3C)
	// 82411F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411F24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411F30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411F34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411F40 size=92
    let mut pc: u32 = 0x82411F40;
    'dispatch: loop {
        match pc {
            0x82411F40 => {
    //   block [0x82411F40..0x82411F80)
	// 82411F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411F48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411F4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411F50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411F58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411F5C: 480018CD  bl 0x82413828
	ctx.lr = 0x82411F60;
	sub_82413828(ctx, base);
	// 82411F60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411F64: 4182001C  beq 0x82411f80
	if ctx.cr[0].eq {
	pc = 0x82411F80; continue 'dispatch;
	}
	// 82411F68: 4BFFE859  bl 0x824107c0
	ctx.lr = 0x82411F6C;
	sub_824107C0(ctx, base);
	// 82411F6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411F70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411F74: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411F78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411F7C: 4E800421  bctrl
	ctx.lr = 0x82411F80;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82411F80 => {
    //   block [0x82411F80..0x82411F9C)
	// 82411F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411F84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411F90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411FA0 size=92
    let mut pc: u32 = 0x82411FA0;
    'dispatch: loop {
        match pc {
            0x82411FA0 => {
    //   block [0x82411FA0..0x82411FE0)
	// 82411FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411FA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411FAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411FB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411FB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411FB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411FBC: 48002225  bl 0x824141e0
	ctx.lr = 0x82411FC0;
	sub_824141E0(ctx, base);
	// 82411FC0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411FC4: 4182001C  beq 0x82411fe0
	if ctx.cr[0].eq {
	pc = 0x82411FE0; continue 'dispatch;
	}
	// 82411FC8: 4BFFE7F9  bl 0x824107c0
	ctx.lr = 0x82411FCC;
	sub_824107C0(ctx, base);
	// 82411FCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411FD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411FD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411FD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411FDC: 4E800421  bctrl
	ctx.lr = 0x82411FE0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82411FE0 => {
    //   block [0x82411FE0..0x82411FFC)
	// 82411FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411FE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411FF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411FF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412000 size=108
    let mut pc: u32 = 0x82412000;
    'dispatch: loop {
        match pc {
            0x82412000 => {
    //   block [0x82412000..0x82412048)
	// 82412000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412004: 481230B9  bl 0x825350bc
	ctx.lr = 0x82412008;
	sub_82535080(ctx, base);
	// 82412008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241200C: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82412010: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82412014: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412018: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8241201C: 419A002C  beq cr6, 0x82412048
	if ctx.cr[6].eq {
	pc = 0x82412048; continue 'dispatch;
	}
	// 82412020: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82412024: 409A003C  bne cr6, 0x82412060
	if !ctx.cr[6].eq {
	pc = 0x82412060; continue 'dispatch;
	}
	// 82412028: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8241202C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412030: 48056A91  bl 0x82468ac0
	ctx.lr = 0x82412034;
	sub_82468AC0(ctx, base);
	// 82412034: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82412038: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241203C: 40980024  bge cr6, 0x82412060
	if !ctx.cr[6].lt {
	pc = 0x82412060; continue 'dispatch;
	}
	// 82412040: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82412044: 48000014  b 0x82412058
	pc = 0x82412058; continue 'dispatch;
            }
            0x82412048 => {
    //   block [0x82412048..0x82412058)
	// 82412048: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241204C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412050: 41820010  beq 0x82412060
	if ctx.cr[0].eq {
	pc = 0x82412060; continue 'dispatch;
	}
	// 82412054: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x82412058; continue 'dispatch;
            }
            0x82412058 => {
    //   block [0x82412058..0x82412060)
	// 82412058: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241205C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	pc = 0x82412060; continue 'dispatch;
            }
            0x82412060 => {
    //   block [0x82412060..0x8241206C)
	// 82412060: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82412064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82412068: 481230A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412070 size=16
    let mut pc: u32 = 0x82412070;
    'dispatch: loop {
        match pc {
            0x82412070 => {
    //   block [0x82412070..0x82412080)
	// 82412070: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82412074: 480024B4  b 0x82414528
	sub_82414528(ctx, base);
	return;
	// 82412078: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8241207C: 48056A44  b 0x82468ac0
	sub_82468AC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412080 size=104
    let mut pc: u32 = 0x82412080;
    'dispatch: loop {
        match pc {
            0x82412080 => {
    //   block [0x82412080..0x824120B0)
	// 82412080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412084: 48123031  bl 0x825350b4
	ctx.lr = 0x82412088;
	sub_82535080(ctx, base);
	// 82412088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241208C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412090: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82412094: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 82412098: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8241209C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824120A0: 48056A21  bl 0x82468ac0
	ctx.lr = 0x824120A4;
	sub_82468AC0(ctx, base);
	// 824120A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824120A8: 4182002C  beq 0x824120d4
	if ctx.cr[0].eq {
	pc = 0x824120D4; continue 'dispatch;
	}
	// 824120AC: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	pc = 0x824120B0; continue 'dispatch;
            }
            0x824120B0 => {
    //   block [0x824120B0..0x824120D4)
	// 824120B0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824120B4: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824120B8: 48001AD1  bl 0x82413b88
	ctx.lr = 0x824120BC;
	sub_82413B88(ctx, base);
	// 824120BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824120C0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824120C4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824120C8: 480569F9  bl 0x82468ac0
	ctx.lr = 0x824120CC;
	sub_82468AC0(ctx, base);
	// 824120CC: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824120D0: 4198FFE0  blt cr6, 0x824120b0
	if ctx.cr[6].lt {
	pc = 0x824120B0; continue 'dispatch;
	}
	pc = 0x824120D4; continue 'dispatch;
            }
            0x824120D4 => {
    //   block [0x824120D4..0x824120E8)
	// 824120D4: 937F004C  stw r27, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[27].u32 ) };
	// 824120D8: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 824120DC: 937F0054  stw r27, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824120E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824120E4: 48123020  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824120E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824120E8 size=108
    let mut pc: u32 = 0x824120E8;
    'dispatch: loop {
        match pc {
            0x824120E8 => {
    //   block [0x824120E8..0x82412114)
	// 824120E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824120EC: 48122FCD  bl 0x825350b8
	ctx.lr = 0x824120F0;
	sub_82535080(ctx, base);
	// 824120F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824120F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824120F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824120FC: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 82412100: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412104: 480569BD  bl 0x82468ac0
	ctx.lr = 0x82412108;
	sub_82468AC0(ctx, base);
	// 82412108: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241210C: 41820038  beq 0x82412144
	if ctx.cr[0].eq {
	pc = 0x82412144; continue 'dispatch;
	}
	// 82412110: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82412114; continue 'dispatch;
            }
            0x82412114 => {
    //   block [0x82412114..0x8241212C)
	// 82412114: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412118: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241211C: 419A0010  beq cr6, 0x8241212c
	if ctx.cr[6].eq {
	pc = 0x8241212C; continue 'dispatch;
	}
	// 82412120: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412124: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82412128: 48001A61  bl 0x82413b88
	ctx.lr = 0x8241212C;
	sub_82413B88(ctx, base);
	pc = 0x8241212C; continue 'dispatch;
            }
            0x8241212C => {
    //   block [0x8241212C..0x82412144)
	// 8241212C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412130: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82412134: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82412138: 48056989  bl 0x82468ac0
	ctx.lr = 0x8241213C;
	sub_82468AC0(ctx, base);
	// 8241213C: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412140: 4198FFD4  blt cr6, 0x82412114
	if ctx.cr[6].lt {
	pc = 0x82412114; continue 'dispatch;
	}
	pc = 0x82412144; continue 'dispatch;
            }
            0x82412144 => {
    //   block [0x82412144..0x82412154)
	// 82412144: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412148: 917E0050  stw r11, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8241214C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412150: 48122FB8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412158 size=108
    let mut pc: u32 = 0x82412158;
    'dispatch: loop {
        match pc {
            0x82412158 => {
    //   block [0x82412158..0x82412180)
	// 82412158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241215C: 48122F61  bl 0x825350bc
	ctx.lr = 0x82412160;
	sub_82535080(ctx, base);
	// 82412160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412168: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8241216C: 3BBF0050  addi r29, r31, 0x50
	ctx.r[29].s64 = ctx.r[31].s64 + 80;
	// 82412170: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82412174: 4805694D  bl 0x82468ac0
	ctx.lr = 0x82412178;
	sub_82468AC0(ctx, base);
	// 82412178: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8241217C: 41980008  blt cr6, 0x82412184
	if ctx.cr[6].lt {
	pc = 0x82412184; continue 'dispatch;
	}
	pc = 0x82412180; continue 'dispatch;
            }
            0x82412180 => {
    //   block [0x82412180..0x82412184)
	// 82412180: 48000000  b 0x82412180
	pc = 0x82412180; continue 'dispatch;
            }
            0x82412184 => {
    //   block [0x82412184..0x824121BC)
	// 82412184: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412188: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241218C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82412190: 48002191  bl 0x82414320
	ctx.lr = 0x82412194;
	sub_82414320(ctx, base);
	// 82412194: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412198: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241219C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824121A0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824121A4: 48001755  bl 0x824138f8
	ctx.lr = 0x824121A8;
	sub_824138F8(ctx, base);
	// 824121A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824121AC: 41820010  beq 0x824121bc
	if ctx.cr[0].eq {
	pc = 0x824121BC; continue 'dispatch;
	}
	// 824121B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824121B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824121B8: 4BFFFE49  bl 0x82412000
	ctx.lr = 0x824121BC;
	sub_82412000(ctx, base);
	pc = 0x824121BC; continue 'dispatch;
            }
            0x824121BC => {
    //   block [0x824121BC..0x824121C4)
	// 824121BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824121C0: 48122F4C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824121C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824121C8 size=64
    let mut pc: u32 = 0x824121C8;
    'dispatch: loop {
        match pc {
            0x824121C8 => {
    //   block [0x824121C8..0x824121E4)
	// 824121C8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824121CC: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 824121D0: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824121D4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824121D8: 409A000C  bne cr6, 0x824121e4
	if !ctx.cr[6].eq {
	pc = 0x824121E4; continue 'dispatch;
	}
	// 824121DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824121E0: 48000014  b 0x824121f4
	pc = 0x824121F4; continue 'dispatch;
            }
            0x824121E4 => {
    //   block [0x824121E4..0x824121F4)
	// 824121E4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824121E8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824121EC: 41980008  blt cr6, 0x824121f4
	if ctx.cr[6].lt {
	pc = 0x824121F4; continue 'dispatch;
	}
	// 824121F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x824121F4; continue 'dispatch;
            }
            0x824121F4 => {
    //   block [0x824121F4..0x82412208)
	// 824121F4: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824121F8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824121FC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82412200: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82412204: 4BFFFEE4  b 0x824120e8
	sub_824120E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412218 size=116
    let mut pc: u32 = 0x82412218;
    'dispatch: loop {
        match pc {
            0x82412218 => {
    //   block [0x82412218..0x82412248)
	// 82412218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241221C: 48122E99  bl 0x825350b4
	ctx.lr = 0x82412220;
	sub_82535080(ctx, base);
	// 82412220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412224: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82412228: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241222C: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 82412230: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412234: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412238: 48056889  bl 0x82468ac0
	ctx.lr = 0x8241223C;
	sub_82468AC0(ctx, base);
	// 8241223C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412240: 41820038  beq 0x82412278
	if ctx.cr[0].eq {
	pc = 0x82412278; continue 'dispatch;
	}
	// 82412244: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82412248; continue 'dispatch;
            }
            0x82412248 => {
    //   block [0x82412248..0x82412278)
	// 82412248: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241224C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82412250: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82412254: 4800224D  bl 0x824144a0
	ctx.lr = 0x82412258;
	sub_824144A0(ctx, base);
	// 82412258: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241225C: 40820028  bne 0x82412284
	if !ctx.cr[0].eq {
	pc = 0x82412284; continue 'dispatch;
	}
	// 82412260: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412264: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82412268: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8241226C: 48056855  bl 0x82468ac0
	ctx.lr = 0x82412270;
	sub_82468AC0(ctx, base);
	// 82412270: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412274: 4198FFD4  blt cr6, 0x82412248
	if ctx.cr[6].lt {
	pc = 0x82412248; continue 'dispatch;
	}
	pc = 0x82412278; continue 'dispatch;
            }
            0x82412278 => {
    //   block [0x82412278..0x8241227C)
	// 82412278: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8241227C; continue 'dispatch;
            }
            0x8241227C => {
    //   block [0x8241227C..0x82412284)
	// 8241227C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412280: 48122E84  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82412284 => {
    //   block [0x82412284..0x8241228C)
	// 82412284: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82412288: 4BFFFFF4  b 0x8241227c
	pc = 0x8241227C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412290 size=116
    let mut pc: u32 = 0x82412290;
    'dispatch: loop {
        match pc {
            0x82412290 => {
    //   block [0x82412290..0x824122C0)
	// 82412290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412294: 48122E21  bl 0x825350b4
	ctx.lr = 0x82412298;
	sub_82535080(ctx, base);
	// 82412298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241229C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824122A0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824122A4: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 824122A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824122AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824122B0: 48056811  bl 0x82468ac0
	ctx.lr = 0x824122B4;
	sub_82468AC0(ctx, base);
	// 824122B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824122B8: 41820038  beq 0x824122f0
	if ctx.cr[0].eq {
	pc = 0x824122F0; continue 'dispatch;
	}
	// 824122BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x824122C0; continue 'dispatch;
            }
            0x824122C0 => {
    //   block [0x824122C0..0x824122F0)
	// 824122C0: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824122C4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824122C8: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824122CC: 480021FD  bl 0x824144c8
	ctx.lr = 0x824122D0;
	sub_824144C8(ctx, base);
	// 824122D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824122D4: 40820028  bne 0x824122fc
	if !ctx.cr[0].eq {
	pc = 0x824122FC; continue 'dispatch;
	}
	// 824122D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824122DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824122E0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824122E4: 480567DD  bl 0x82468ac0
	ctx.lr = 0x824122E8;
	sub_82468AC0(ctx, base);
	// 824122E8: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824122EC: 4198FFD4  blt cr6, 0x824122c0
	if ctx.cr[6].lt {
	pc = 0x824122C0; continue 'dispatch;
	}
	pc = 0x824122F0; continue 'dispatch;
            }
            0x824122F0 => {
    //   block [0x824122F0..0x824122F4)
	// 824122F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x824122F4; continue 'dispatch;
            }
            0x824122F4 => {
    //   block [0x824122F4..0x824122FC)
	// 824122F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824122F8: 48122E0C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x824122FC => {
    //   block [0x824122FC..0x82412304)
	// 824122FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82412300: 4BFFFFF4  b 0x824122f4
	pc = 0x824122F4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412308 size=128
    let mut pc: u32 = 0x82412308;
    'dispatch: loop {
        match pc {
            0x82412308 => {
    //   block [0x82412308..0x82412338)
	// 82412308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241230C: 48122DA9  bl 0x825350b4
	ctx.lr = 0x82412310;
	sub_82535080(ctx, base);
	// 82412310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412318: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241231C: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 82412320: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82412324: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412328: 48056799  bl 0x82468ac0
	ctx.lr = 0x8241232C;
	sub_82468AC0(ctx, base);
	// 8241232C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412330: 41820038  beq 0x82412368
	if ctx.cr[0].eq {
	pc = 0x82412368; continue 'dispatch;
	}
	// 82412334: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82412338; continue 'dispatch;
            }
            0x82412338 => {
    //   block [0x82412338..0x82412368)
	// 82412338: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241233C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82412340: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82412344: 4800215D  bl 0x824144a0
	ctx.lr = 0x82412348;
	sub_824144A0(ctx, base);
	// 82412348: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241234C: 40820028  bne 0x82412374
	if !ctx.cr[0].eq {
	pc = 0x82412374; continue 'dispatch;
	}
	// 82412350: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412354: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82412358: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8241235C: 48056765  bl 0x82468ac0
	ctx.lr = 0x82412360;
	sub_82468AC0(ctx, base);
	// 82412360: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412364: 4198FFD4  blt cr6, 0x82412338
	if ctx.cr[6].lt {
	pc = 0x82412338; continue 'dispatch;
	}
	pc = 0x82412368; continue 'dispatch;
            }
            0x82412368 => {
    //   block [0x82412368..0x8241236C)
	// 82412368: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8241236C; continue 'dispatch;
            }
            0x8241236C => {
    //   block [0x8241236C..0x82412374)
	// 8241236C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412370: 48122D94  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82412374 => {
    //   block [0x82412374..0x82412388)
	// 82412374: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412378: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241237C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412380: 4BEE9E51  bl 0x822fc1d0
	ctx.lr = 0x82412384;
	sub_822FC1D0(ctx, base);
	// 82412384: 4BFFFFE8  b 0x8241236c
	pc = 0x8241236C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412388 size=128
    let mut pc: u32 = 0x82412388;
    'dispatch: loop {
        match pc {
            0x82412388 => {
    //   block [0x82412388..0x824123B8)
	// 82412388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241238C: 48122D29  bl 0x825350b4
	ctx.lr = 0x82412390;
	sub_82535080(ctx, base);
	// 82412390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412398: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241239C: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 824123A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824123A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824123A8: 48056719  bl 0x82468ac0
	ctx.lr = 0x824123AC;
	sub_82468AC0(ctx, base);
	// 824123AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824123B0: 41820038  beq 0x824123e8
	if ctx.cr[0].eq {
	pc = 0x824123E8; continue 'dispatch;
	}
	// 824123B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x824123B8; continue 'dispatch;
            }
            0x824123B8 => {
    //   block [0x824123B8..0x824123E8)
	// 824123B8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824123BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824123C0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824123C4: 48002105  bl 0x824144c8
	ctx.lr = 0x824123C8;
	sub_824144C8(ctx, base);
	// 824123C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824123CC: 40820028  bne 0x824123f4
	if !ctx.cr[0].eq {
	pc = 0x824123F4; continue 'dispatch;
	}
	// 824123D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824123D4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824123D8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824123DC: 480566E5  bl 0x82468ac0
	ctx.lr = 0x824123E0;
	sub_82468AC0(ctx, base);
	// 824123E0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824123E4: 4198FFD4  blt cr6, 0x824123b8
	if ctx.cr[6].lt {
	pc = 0x824123B8; continue 'dispatch;
	}
	pc = 0x824123E8; continue 'dispatch;
            }
            0x824123E8 => {
    //   block [0x824123E8..0x824123EC)
	// 824123E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x824123EC; continue 'dispatch;
            }
            0x824123EC => {
    //   block [0x824123EC..0x824123F4)
	// 824123EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824123F0: 48122D14  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x824123F4 => {
    //   block [0x824123F4..0x82412408)
	// 824123F4: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824123F8: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824123FC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412400: 4BEE9DD1  bl 0x822fc1d0
	ctx.lr = 0x82412404;
	sub_822FC1D0(ctx, base);
	// 82412404: 4BFFFFE8  b 0x824123ec
	pc = 0x824123EC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412408 size=128
    let mut pc: u32 = 0x82412408;
    'dispatch: loop {
        match pc {
            0x82412408 => {
    //   block [0x82412408..0x8241243C)
	// 82412408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241240C: 48122CA5  bl 0x825350b0
	ctx.lr = 0x82412410;
	sub_82535080(ctx, base);
	// 82412410: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412414: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82412418: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241241C: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 82412420: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82412424: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412428: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241242C: 48056695  bl 0x82468ac0
	ctx.lr = 0x82412430;
	sub_82468AC0(ctx, base);
	// 82412430: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412434: 41820034  beq 0x82412468
	if ctx.cr[0].eq {
	pc = 0x82412468; continue 'dispatch;
	}
	// 82412438: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x8241243C; continue 'dispatch;
            }
            0x8241243C => {
    //   block [0x8241243C..0x82412468)
	// 8241243C: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412440: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82412444: 4BEE9D8D  bl 0x822fc1d0
	ctx.lr = 0x82412448;
	sub_822FC1D0(ctx, base);
	// 82412448: 7F03D840  cmplw cr6, r3, r27
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8241244C: 419A0028  beq cr6, 0x82412474
	if ctx.cr[6].eq {
	pc = 0x82412474; continue 'dispatch;
	}
	// 82412450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412454: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82412458: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8241245C: 48056665  bl 0x82468ac0
	ctx.lr = 0x82412460;
	sub_82468AC0(ctx, base);
	// 82412460: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412464: 4198FFD8  blt cr6, 0x8241243c
	if ctx.cr[6].lt {
	pc = 0x8241243C; continue 'dispatch;
	}
	pc = 0x82412468; continue 'dispatch;
            }
            0x82412468 => {
    //   block [0x82412468..0x8241246C)
	// 82412468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8241246C; continue 'dispatch;
            }
            0x8241246C => {
    //   block [0x8241246C..0x82412474)
	// 8241246C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82412470: 48122C90  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            0x82412474 => {
    //   block [0x82412474..0x82412480)
	// 82412474: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82412478: 419A0008  beq cr6, 0x82412480
	if ctx.cr[6].eq {
	pc = 0x82412480; continue 'dispatch;
	}
	// 8241247C: 93BA0000  stw r29, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	pc = 0x82412480; continue 'dispatch;
            }
            0x82412480 => {
    //   block [0x82412480..0x82412488)
	// 82412480: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82412484: 4BFFFFE8  b 0x8241246c
	pc = 0x8241246C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412488 size=84
    let mut pc: u32 = 0x82412488;
    'dispatch: loop {
        match pc {
            0x82412488 => {
    //   block [0x82412488..0x824124B4)
	// 82412488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241248C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412490: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82412494: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241249C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824124A0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824124A4: 83DF004C  lwz r30, 0x4c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824124A8: 48056619  bl 0x82468ac0
	ctx.lr = 0x824124AC;
	sub_82468AC0(ctx, base);
	// 824124AC: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824124B0: 41980008  blt cr6, 0x824124b8
	if ctx.cr[6].lt {
	pc = 0x824124B8; continue 'dispatch;
	}
	pc = 0x824124B4; continue 'dispatch;
            }
            0x824124B4 => {
    //   block [0x824124B4..0x824124B8)
	// 824124B4: 48000000  b 0x824124b4
	pc = 0x824124B4; continue 'dispatch;
            }
            0x824124B8 => {
    //   block [0x824124B8..0x824124DC)
	// 824124B8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824124BC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824124C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824124C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824124C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824124CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824124D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824124D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824124D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824124E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824124E0 size=72
    let mut pc: u32 = 0x824124E0;
    'dispatch: loop {
        match pc {
            0x824124E0 => {
    //   block [0x824124E0..0x82412528)
	// 824124E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824124E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824124E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824124EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824124F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824124F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824124F8: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824124FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82412500: 4BFFFB01  bl 0x82412000
	ctx.lr = 0x82412504;
	sub_82412000(ctx, base);
	// 82412504: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82412508: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241250C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82412510: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412514: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82412518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241251C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412528 size=120
    let mut pc: u32 = 0x82412528;
    'dispatch: loop {
        match pc {
            0x82412528 => {
    //   block [0x82412528..0x82412564)
	// 82412528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82412534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412538: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 8241253C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82412540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412548: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8241254C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82412550: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82412554: 83DF004C  lwz r30, 0x4c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412558: 48056569  bl 0x82468ac0
	ctx.lr = 0x8241255C;
	sub_82468AC0(ctx, base);
	// 8241255C: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412560: 41980008  blt cr6, 0x82412568
	if ctx.cr[6].lt {
	pc = 0x82412568; continue 'dispatch;
	}
	pc = 0x82412564; continue 'dispatch;
            }
            0x82412564 => {
    //   block [0x82412564..0x82412568)
	// 82412564: 48000000  b 0x82412564
	pc = 0x82412564; continue 'dispatch;
            }
            0x82412568 => {
    //   block [0x82412568..0x824125A0)
	// 82412568: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241256C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82412570: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82412574: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82412578: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8241257C: 48001475  bl 0x824139f0
	ctx.lr = 0x82412580;
	sub_824139F0(ctx, base);
	// 82412580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241258C: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82412590: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82412594: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82412598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241259C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824125A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824125A0 size=76
    let mut pc: u32 = 0x824125A0;
    'dispatch: loop {
        match pc {
            0x824125A0 => {
    //   block [0x824125A0..0x824125E4)
	// 824125A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824125A4: 48122B19  bl 0x825350bc
	ctx.lr = 0x824125A8;
	sub_82535080(ctx, base);
	// 824125A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824125AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824125B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824125B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824125B8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 824125BC: 4BFFFE4D  bl 0x82412408
	ctx.lr = 0x824125C0;
	sub_82412408(ctx, base);
	// 824125C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824125C4: 41820020  beq 0x824125e4
	if ctx.cr[0].eq {
	pc = 0x824125E4; continue 'dispatch;
	}
	// 824125C8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824125CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824125D0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824125D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824125D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824125DC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824125E0: 48001A89  bl 0x82414068
	ctx.lr = 0x824125E4;
	sub_82414068(ctx, base);
	pc = 0x824125E4; continue 'dispatch;
            }
            0x824125E4 => {
    //   block [0x824125E4..0x824125EC)
	// 824125E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824125E8: 48122B24  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824125F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824125F0 size=76
    let mut pc: u32 = 0x824125F0;
    'dispatch: loop {
        match pc {
            0x824125F0 => {
    //   block [0x824125F0..0x82412634)
	// 824125F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824125F4: 48122AC9  bl 0x825350bc
	ctx.lr = 0x824125F8;
	sub_82535080(ctx, base);
	// 824125F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824125FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82412600: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82412604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412608: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8241260C: 4BFFFDFD  bl 0x82412408
	ctx.lr = 0x82412610;
	sub_82412408(ctx, base);
	// 82412610: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412614: 41820020  beq 0x82412634
	if ctx.cr[0].eq {
	pc = 0x82412634; continue 'dispatch;
	}
	// 82412618: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241261C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82412620: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412628: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241262C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412630: 48001A89  bl 0x824140b8
	ctx.lr = 0x82412634;
	sub_824140B8(ctx, base);
	pc = 0x82412634; continue 'dispatch;
            }
            0x82412634 => {
    //   block [0x82412634..0x8241263C)
	// 82412634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412638: 48122AD4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412640 size=76
    let mut pc: u32 = 0x82412640;
    'dispatch: loop {
        match pc {
            0x82412640 => {
    //   block [0x82412640..0x82412684)
	// 82412640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412644: 48122A79  bl 0x825350bc
	ctx.lr = 0x82412648;
	sub_82535080(ctx, base);
	// 82412648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241264C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82412650: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82412654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412658: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8241265C: 4BFFFDAD  bl 0x82412408
	ctx.lr = 0x82412660;
	sub_82412408(ctx, base);
	// 82412660: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412664: 41820020  beq 0x82412684
	if ctx.cr[0].eq {
	pc = 0x82412684; continue 'dispatch;
	}
	// 82412668: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241266C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82412670: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412674: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412678: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241267C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412680: 48001A89  bl 0x82414108
	ctx.lr = 0x82412684;
	sub_82414108(ctx, base);
	pc = 0x82412684; continue 'dispatch;
            }
            0x82412684 => {
    //   block [0x82412684..0x8241268C)
	// 82412684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412688: 48122A84  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412690 size=76
    let mut pc: u32 = 0x82412690;
    'dispatch: loop {
        match pc {
            0x82412690 => {
    //   block [0x82412690..0x824126D4)
	// 82412690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412694: 48122A29  bl 0x825350bc
	ctx.lr = 0x82412698;
	sub_82535080(ctx, base);
	// 82412698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241269C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824126A0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824126A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824126A8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 824126AC: 4BFFFD5D  bl 0x82412408
	ctx.lr = 0x824126B0;
	sub_82412408(ctx, base);
	// 824126B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824126B4: 41820020  beq 0x824126d4
	if ctx.cr[0].eq {
	pc = 0x824126D4; continue 'dispatch;
	}
	// 824126B8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824126BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824126C0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824126C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824126C8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824126CC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824126D0: 48001A89  bl 0x82414158
	ctx.lr = 0x824126D4;
	sub_82414158(ctx, base);
	pc = 0x824126D4; continue 'dispatch;
            }
            0x824126D4 => {
    //   block [0x824126D4..0x824126DC)
	// 824126D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824126D8: 48122A34  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824126E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824126E0 size=160
    let mut pc: u32 = 0x824126E0;
    'dispatch: loop {
        match pc {
            0x824126E0 => {
    //   block [0x824126E0..0x82412770)
	// 824126E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824126E4: 481229CD  bl 0x825350b0
	ctx.lr = 0x824126E8;
	sub_82535080(ctx, base);
	// 824126E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824126EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824126F0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824126F4: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 824126F8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824126FC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82412700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412704: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82412708: 48002849  bl 0x82414f50
	ctx.lr = 0x8241270C;
	sub_82414F50(ctx, base);
	// 8241270C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82412710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412714: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82412718: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241271C: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82412720: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82412724: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82412728: 83AA0008  lwz r29, 8(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241272C: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412730: 48001DC1  bl 0x824144f0
	ctx.lr = 0x82412734;
	sub_824144F0(ctx, base);
	// 82412734: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82412738: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241273C: 48001DE5  bl 0x82414520
	ctx.lr = 0x82412740;
	sub_82414520(ctx, base);
	// 82412740: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82412744: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412748: 48001DD1  bl 0x82414518
	ctx.lr = 0x8241274C;
	sub_82414518(ctx, base);
	// 8241274C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412750: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82412754: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82412758: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8241275C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82412760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412764: 4E800421  bctrl
	ctx.lr = 0x82412768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412768: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241276C: 40820008  bne 0x82412774
	if !ctx.cr[0].eq {
	pc = 0x82412774; continue 'dispatch;
	}
            }
            0x82412770 => {
    //   block [0x82412770..0x82412774)
	// 82412770: 48000000  b 0x82412770
	pc = 0x82412770; continue 'dispatch;
            }
            0x82412774 => {
    //   block [0x82412774..0x82412780)
	// 82412774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412778: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241277C: 48122984  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412780 size=152
    let mut pc: u32 = 0x82412780;
    'dispatch: loop {
        match pc {
            0x82412780 => {
    //   block [0x82412780..0x824127B0)
	// 82412780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412784: 48122931  bl 0x825350b4
	ctx.lr = 0x82412788;
	sub_82535080(ctx, base);
	// 82412788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241278C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82412790: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82412794: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 82412798: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8241279C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824127A0: 48056321  bl 0x82468ac0
	ctx.lr = 0x824127A4;
	sub_82468AC0(ctx, base);
	// 824127A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824127A8: 4182004C  beq 0x824127f4
	if ctx.cr[0].eq {
	pc = 0x824127F4; continue 'dispatch;
	}
	// 824127AC: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	pc = 0x824127B0; continue 'dispatch;
            }
            0x824127B0 => {
    //   block [0x824127B0..0x824127D4)
	// 824127B0: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824127B4: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824127B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824127BC: 419A0020  beq cr6, 0x824127dc
	if ctx.cr[6].eq {
	pc = 0x824127DC; continue 'dispatch;
	}
	// 824127C0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824127C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824127C8: 4182000C  beq 0x824127d4
	if ctx.cr[0].eq {
	pc = 0x824127D4; continue 'dispatch;
	}
	// 824127CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824127D0: 4BFFF7D1  bl 0x82411fa0
	ctx.lr = 0x824127D4;
	sub_82411FA0(ctx, base);
	pc = 0x824127D4; continue 'dispatch;
            }
            0x824127D4 => {
    //   block [0x824127D4..0x824127DC)
	// 824127D4: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824127D8: 7F6BF92E  stwx r27, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[27].u32) };
	pc = 0x824127DC; continue 'dispatch;
            }
            0x824127DC => {
    //   block [0x824127DC..0x824127F4)
	// 824127DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824127E0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824127E4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824127E8: 480562D9  bl 0x82468ac0
	ctx.lr = 0x824127EC;
	sub_82468AC0(ctx, base);
	// 824127EC: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824127F0: 4198FFC0  blt cr6, 0x824127b0
	if ctx.cr[6].lt {
	pc = 0x824127B0; continue 'dispatch;
	}
	pc = 0x824127F4; continue 'dispatch;
            }
            0x824127F4 => {
    //   block [0x824127F4..0x82412808)
	// 824127F4: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824127F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824127FC: 4182000C  beq 0x82412808
	if ctx.cr[0].eq {
	pc = 0x82412808; continue 'dispatch;
	}
	// 82412800: 481203B9  bl 0x82532bb8
	ctx.lr = 0x82412804;
	sub_82532BB8(ctx, base);
	// 82412804: 937E0048  stw r27, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[27].u32 ) };
	pc = 0x82412808; continue 'dispatch;
            }
            0x82412808 => {
    //   block [0x82412808..0x82412818)
	// 82412808: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8241280C: 480025A5  bl 0x82414db0
	ctx.lr = 0x82412810;
	sub_82414DB0(ctx, base);
	// 82412810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412814: 481228F0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412818 size=108
    let mut pc: u32 = 0x82412818;
    'dispatch: loop {
        match pc {
            0x82412818 => {
    //   block [0x82412818..0x82412848)
	// 82412818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241281C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82412824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241282C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412830: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82412834: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82412838: 48056289  bl 0x82468ac0
	ctx.lr = 0x8241283C;
	sub_82468AC0(ctx, base);
	// 8241283C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412840: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412844: 41980008  blt cr6, 0x8241284c
	if ctx.cr[6].lt {
	pc = 0x8241284C; continue 'dispatch;
	}
	pc = 0x82412848; continue 'dispatch;
            }
            0x82412848 => {
    //   block [0x82412848..0x8241284C)
	// 82412848: 48000000  b 0x82412848
	pc = 0x82412848; continue 'dispatch;
            }
            0x8241284C => {
    //   block [0x8241284C..0x82412860)
	// 8241284C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82412850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412854: 419A000C  beq cr6, 0x82412860
	if ctx.cr[6].eq {
	pc = 0x82412860; continue 'dispatch;
	}
	// 82412858: 4BFFF829  bl 0x82412080
	ctx.lr = 0x8241285C;
	sub_82412080(ctx, base);
	// 8241285C: 48000010  b 0x8241286c
	pc = 0x8241286C; continue 'dispatch;
            }
            0x82412860 => {
    //   block [0x82412860..0x8241286C)
	// 82412860: 4BFFF889  bl 0x824120e8
	ctx.lr = 0x82412864;
	sub_824120E8(ctx, base);
	// 82412864: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82412868: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	pc = 0x8241286C; continue 'dispatch;
            }
            0x8241286C => {
    //   block [0x8241286C..0x82412884)
	// 8241286C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82412870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412878: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241287C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412888 size=116
    let mut pc: u32 = 0x82412888;
    'dispatch: loop {
        match pc {
            0x82412888 => {
    //   block [0x82412888..0x824128D4)
	// 82412888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241288C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82412894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241289C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824128A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824128A4: 4BFFFB65  bl 0x82412408
	ctx.lr = 0x824128A8;
	sub_82412408(ctx, base);
	// 824128A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824128AC: 41820038  beq 0x824128e4
	if ctx.cr[0].eq {
	pc = 0x824128E4; continue 'dispatch;
	}
	// 824128B0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824128B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824128B8: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824128BC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824128C0: 409A0014  bne cr6, 0x824128d4
	if !ctx.cr[6].eq {
	pc = 0x824128D4; continue 'dispatch;
	}
	// 824128C4: 4BFFF825  bl 0x824120e8
	ctx.lr = 0x824128C8;
	sub_824120E8(ctx, base);
	// 824128C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824128CC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824128D0: 48000008  b 0x824128d8
	pc = 0x824128D8; continue 'dispatch;
            }
            0x824128D4 => {
    //   block [0x824128D4..0x824128D8)
	// 824128D4: 4BFFF7AD  bl 0x82412080
	ctx.lr = 0x824128D8;
	sub_82412080(ctx, base);
	pc = 0x824128D8; continue 'dispatch;
            }
            0x824128D8 => {
    //   block [0x824128D8..0x824128E4)
	// 824128D8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824128DC: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 824128E0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	pc = 0x824128E4; continue 'dispatch;
            }
            0x824128E4 => {
    //   block [0x824128E4..0x824128FC)
	// 824128E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824128E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824128EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824128F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824128F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824128F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412900 size=36
    let mut pc: u32 = 0x82412900;
    'dispatch: loop {
        match pc {
            0x82412900 => {
    //   block [0x82412900..0x82412924)
	// 82412900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241290C: 4BFFFB7D  bl 0x82412488
	ctx.lr = 0x82412910;
	sub_82412488(ctx, base);
	// 82412910: 48000FE9  bl 0x824138f8
	ctx.lr = 0x82412914;
	sub_824138F8(ctx, base);
	// 82412914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241291C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412928 size=52
    let mut pc: u32 = 0x82412928;
    'dispatch: loop {
        match pc {
            0x82412928 => {
    //   block [0x82412928..0x8241295C)
	// 82412928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241292C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412938: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8241293C: 4BFFFB4D  bl 0x82412488
	ctx.lr = 0x82412940;
	sub_82412488(ctx, base);
	// 82412940: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82412944: 480012DD  bl 0x82413c20
	ctx.lr = 0x82412948;
	sub_82413C20(ctx, base);
	// 82412948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241294C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82412968 size=388
    let mut pc: u32 = 0x82412968;
    'dispatch: loop {
        match pc {
            0x82412968 => {
    //   block [0x82412968..0x8241299C)
	// 82412968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241296C: 4812274D  bl 0x825350b8
	ctx.lr = 0x82412970;
	sub_82535080(ctx, base);
	// 82412970: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82412974: 48123671  bl 0x82535fe4
	ctx.lr = 0x82412978;
	sub_82535FB0(ctx, base);
	// 82412978: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241297C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412980: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 82412984: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82412988: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8241298C: 4BFFFAFD  bl 0x82412488
	ctx.lr = 0x82412990;
	sub_82412488(ctx, base);
	// 82412990: 48000F69  bl 0x824138f8
	ctx.lr = 0x82412994;
	sub_824138F8(ctx, base);
	// 82412994: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412998: 4082000C  bne 0x824129a4
	if !ctx.cr[0].eq {
	pc = 0x824129A4; continue 'dispatch;
	}
	pc = 0x8241299C; continue 'dispatch;
            }
            0x8241299C => {
    //   block [0x8241299C..0x824129A4)
	// 8241299C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824129A0: 4800013C  b 0x82412adc
	pc = 0x82412ADC; continue 'dispatch;
            }
            0x824129A4 => {
    //   block [0x824129A4..0x82412A14)
	// 824129A4: C01C0000  lfs f0, 0(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824129A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824129AC: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824129B0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824129B4: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824129B8: 4BFFFAD1  bl 0x82412488
	ctx.lr = 0x824129BC;
	sub_82412488(ctx, base);
	// 824129BC: 4800269D  bl 0x82415058
	ctx.lr = 0x824129C0;
	sub_82415058(ctx, base);
	// 824129C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824129C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824129C8: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 824129CC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824129D0: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824129D4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824129D8: FFC00018  frsp f30, f0
	ctx.f[30].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824129DC: 4BFFFAAD  bl 0x82412488
	ctx.lr = 0x824129E0;
	sub_82412488(ctx, base);
	// 824129E0: 48000F41  bl 0x82413920
	ctx.lr = 0x824129E4;
	sub_82413920(ctx, base);
	// 824129E4: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 824129E8: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 824129EC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824129F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824129F4: C38B1850  lfs f28, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 824129F8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824129FC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82412A00: FFA00018  frsp f29, f0
	ctx.f[29].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82412A04: 41980010  blt cr6, 0x82412a14
	if ctx.cr[6].lt {
	pc = 0x82412A14; continue 'dispatch;
	}
	// 82412A08: EC1DE02A  fadds f0, f29, f28
	ctx.f[0].f64 = ((ctx.f[29].f64 + ctx.f[28].f64) as f32) as f64;
	// 82412A0C: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82412A10: 419900C4  bgt cr6, 0x82412ad4
	if ctx.cr[6].gt {
	pc = 0x82412AD4; continue 'dispatch;
	}
	pc = 0x82412A14; continue 'dispatch;
            }
            0x82412A14 => {
    //   block [0x82412A14..0x82412A78)
	// 82412A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412A18: 4BFFFAC9  bl 0x824124e0
	ctx.lr = 0x82412A1C;
	sub_824124E0(ctx, base);
	// 82412A1C: 48000EDD  bl 0x824138f8
	ctx.lr = 0x82412A20;
	sub_824138F8(ctx, base);
	// 82412A20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412A24: 4182FF78  beq 0x8241299c
	if ctx.cr[0].eq {
	pc = 0x8241299C; continue 'dispatch;
	}
	// 82412A28: 389F004C  addi r4, r31, 0x4c
	ctx.r[4].s64 = ctx.r[31].s64 + 76;
	// 82412A2C: 83DF004C  lwz r30, 0x4c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412A34: 4BFFF5CD  bl 0x82412000
	ctx.lr = 0x82412A38;
	sub_82412000(ctx, base);
	// 82412A38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412A3C: 41820074  beq 0x82412ab0
	if ctx.cr[0].eq {
	pc = 0x82412AB0; continue 'dispatch;
	}
	// 82412A40: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412A44: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82412A48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82412A4C: 4800113D  bl 0x82413b88
	ctx.lr = 0x82412A50;
	sub_82413B88(ctx, base);
	// 82412A50: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82412A54: 40980024  bge cr6, 0x82412a78
	if !ctx.cr[6].lt {
	pc = 0x82412A78; continue 'dispatch;
	}
	// 82412A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412A5C: 4BFFFA2D  bl 0x82412488
	ctx.lr = 0x82412A60;
	sub_82412488(ctx, base);
	// 82412A60: 48000EC1  bl 0x82413920
	ctx.lr = 0x82412A64;
	sub_82413920(ctx, base);
	// 82412A64: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82412A68: EC1EF828  fsubs f0, f30, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[31].f64) as f32) as f64);
	// 82412A6C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82412A70: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82412A74: 4800002C  b 0x82412aa0
	pc = 0x82412AA0; continue 'dispatch;
            }
            0x82412A78 => {
    //   block [0x82412A78..0x82412AA0)
	// 82412A78: EFDDE02A  fadds f30, f29, f28
	ctx.f[30].f64 = ((ctx.f[29].f64 + ctx.f[28].f64) as f32) as f64;
	// 82412A7C: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82412A80: 41990054  bgt cr6, 0x82412ad4
	if ctx.cr[6].gt {
	pc = 0x82412AD4; continue 'dispatch;
	}
	// 82412A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412A88: 4BFFFA01  bl 0x82412488
	ctx.lr = 0x82412A8C;
	sub_82412488(ctx, base);
	// 82412A8C: 480025CD  bl 0x82415058
	ctx.lr = 0x82412A90;
	sub_82415058(ctx, base);
	// 82412A90: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82412A94: EC1FF028  fsubs f0, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 82412A98: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82412A9C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	pc = 0x82412AA0; continue 'dispatch;
            }
            0x82412AA0 => {
    //   block [0x82412AA0..0x82412AB0)
	// 82412AA0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82412AA4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82412AA8: EFED002A  fadds f31, f13, f0
	ctx.f[31].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82412AAC: 48000028  b 0x82412ad4
	pc = 0x82412AD4; continue 'dispatch;
            }
            0x82412AB0 => {
    //   block [0x82412AB0..0x82412AC0)
	// 82412AB0: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82412AB4: 4098000C  bge cr6, 0x82412ac0
	if !ctx.cr[6].lt {
	pc = 0x82412AC0; continue 'dispatch;
	}
	// 82412AB8: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 82412ABC: 48000014  b 0x82412ad0
	pc = 0x82412AD0; continue 'dispatch;
            }
            0x82412AC0 => {
    //   block [0x82412AC0..0x82412AD0)
	// 82412AC0: EC1DE02A  fadds f0, f29, f28
	ctx.f[0].f64 = ((ctx.f[29].f64 + ctx.f[28].f64) as f32) as f64;
	// 82412AC4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82412AC8: 4199000C  bgt cr6, 0x82412ad4
	if ctx.cr[6].gt {
	pc = 0x82412AD4; continue 'dispatch;
	}
	// 82412ACC: EFFDD82A  fadds f31, f29, f27
	ctx.f[31].f64 = ((ctx.f[29].f64 + ctx.f[27].f64) as f32) as f64;
	pc = 0x82412AD0; continue 'dispatch;
            }
            0x82412AD0 => {
    //   block [0x82412AD0..0x82412AD4)
	// 82412AD0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	pc = 0x82412AD4; continue 'dispatch;
            }
            0x82412AD4 => {
    //   block [0x82412AD4..0x82412ADC)
	// 82412AD4: D3FC0000  stfs f31, 0(r28)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82412AD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	pc = 0x82412ADC; continue 'dispatch;
            }
            0x82412ADC => {
    //   block [0x82412ADC..0x82412AEC)
	// 82412ADC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82412AE0: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82412AE4: 4812354D  bl 0x82536030
	ctx.lr = 0x82412AE8;
	sub_82535FFC(ctx, base);
	// 82412AE8: 48122620  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412AF0 size=36
    let mut pc: u32 = 0x82412AF0;
    'dispatch: loop {
        match pc {
            0x82412AF0 => {
    //   block [0x82412AF0..0x82412B14)
	// 82412AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412AFC: 4BFFF98D  bl 0x82412488
	ctx.lr = 0x82412B00;
	sub_82412488(ctx, base);
	// 82412B00: 4BEE96D1  bl 0x822fc1d0
	ctx.lr = 0x82412B04;
	sub_822FC1D0(ctx, base);
	// 82412B04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412B18 size=8
    let mut pc: u32 = 0x82412B18;
    'dispatch: loop {
        match pc {
            0x82412B18 => {
    //   block [0x82412B18..0x82412B20)
	// 82412B18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82412B1C: 4BFFF8EC  b 0x82412408
	sub_82412408(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412B20 size=36
    let mut pc: u32 = 0x82412B20;
    'dispatch: loop {
        match pc {
            0x82412B20 => {
    //   block [0x82412B20..0x82412B44)
	// 82412B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412B2C: 4BFFF95D  bl 0x82412488
	ctx.lr = 0x82412B30;
	sub_82412488(ctx, base);
	// 82412B30: 48002529  bl 0x82415058
	ctx.lr = 0x82412B34;
	sub_82415058(ctx, base);
	// 82412B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412B48 size=36
    let mut pc: u32 = 0x82412B48;
    'dispatch: loop {
        match pc {
            0x82412B48 => {
    //   block [0x82412B48..0x82412B6C)
	// 82412B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412B54: 4BFFF935  bl 0x82412488
	ctx.lr = 0x82412B58;
	sub_82412488(ctx, base);
	// 82412B58: 48000DC9  bl 0x82413920
	ctx.lr = 0x82412B5C;
	sub_82413920(ctx, base);
	// 82412B5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412B70 size=68
    let mut pc: u32 = 0x82412B70;
    'dispatch: loop {
        match pc {
            0x82412B70 => {
    //   block [0x82412B70..0x82412BB4)
	// 82412B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412B78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412B7C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82412B80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412B84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82412B88: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82412B8C: 4BFFF8FD  bl 0x82412488
	ctx.lr = 0x82412B90;
	sub_82412488(ctx, base);
	// 82412B90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82412B94: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82412B98: 48001101  bl 0x82413c98
	ctx.lr = 0x82412B9C;
	sub_82413C98(ctx, base);
	// 82412B9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82412BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412BA8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82412BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412BB8 size=720
    let mut pc: u32 = 0x82412BB8;
    'dispatch: loop {
        match pc {
            0x82412BB8 => {
    //   block [0x82412BB8..0x82412BEC)
	// 82412BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412BBC: 481224E9  bl 0x825350a4
	ctx.lr = 0x82412BC0;
	sub_82535080(ctx, base);
	// 82412BC0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412BC4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82412BC8: 3BF80008  addi r31, r24, 8
	ctx.r[31].s64 = ctx.r[24].s64 + 8;
	// 82412BCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412BD0: 48055EF1  bl 0x82468ac0
	ctx.lr = 0x82412BD4;
	sub_82468AC0(ctx, base);
	// 82412BD4: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82412BD8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82412BDC: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82412BE0: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82412BE4: 40990008  ble cr6, 0x82412bec
	if !ctx.cr[6].gt {
	pc = 0x82412BEC; continue 'dispatch;
	}
	// 82412BE8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x82412BEC; continue 'dispatch;
            }
            0x82412BEC => {
    //   block [0x82412BEC..0x82412C18)
	// 82412BEC: 4BFF21DD  bl 0x82404dc8
	ctx.lr = 0x82412BF0;
	sub_82404DC8(ctx, base);
	// 82412BF0: 3B580048  addi r26, r24, 0x48
	ctx.r[26].s64 = ctx.r[24].s64 + 72;
	// 82412BF4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412BF8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82412BFC: 4800141D  bl 0x82414018
	ctx.lr = 0x82412C00;
	sub_82414018(ctx, base);
	// 82412C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C04: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82412C08: 48055EB9  bl 0x82468ac0
	ctx.lr = 0x82412C0C;
	sub_82468AC0(ctx, base);
	// 82412C0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412C10: 418201B0  beq 0x82412dc0
	if ctx.cr[0].eq {
	pc = 0x82412DC0; continue 'dispatch;
	}
	// 82412C14: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	pc = 0x82412C18; continue 'dispatch;
            }
            0x82412C18 => {
    //   block [0x82412C18..0x82412C4C)
	// 82412C18: 4BFFDBA9  bl 0x824107c0
	ctx.lr = 0x82412C1C;
	sub_824107C0(ctx, base);
	// 82412C1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C20: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82412C24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82412C28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412C2C: 4E800421  bctrl
	ctx.lr = 0x82412C30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412C30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412C34: 41820018  beq 0x82412c4c
	if ctx.cr[0].eq {
	pc = 0x82412C4C; continue 'dispatch;
	}
	// 82412C38: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 82412C3C: 80980000  lwz r4, 0(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C40: 48001569  bl 0x824141a8
	ctx.lr = 0x82412C44;
	sub_824141A8(ctx, base);
	// 82412C44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82412C48: 48000008  b 0x82412c50
	pc = 0x82412C50; continue 'dispatch;
            }
            0x82412C4C => {
    //   block [0x82412C4C..0x82412C50)
	// 82412C4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82412C50; continue 'dispatch;
            }
            0x82412C50 => {
    //   block [0x82412C50..0x82412CBC)
	// 82412C50: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C58: 7D6AE12E  stwx r11, r10, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u32) };
	// 82412C5C: 83DA0000  lwz r30, 0(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C60: 480018E1  bl 0x82414540
	ctx.lr = 0x82412C64;
	sub_82414540(ctx, base);
	// 82412C64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82412C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C6C: 4BFFE8AD  bl 0x82411518
	ctx.lr = 0x82412C70;
	sub_82411518(ctx, base);
	// 82412C70: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82412C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C78: 480018C1  bl 0x82414538
	ctx.lr = 0x82412C7C;
	sub_82414538(ctx, base);
	// 82412C7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412C80: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82412C84: 7C7EE02E  lwzx r3, r30, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412C88: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82412C8C: 48000C3D  bl 0x824138c8
	ctx.lr = 0x82412C90;
	sub_824138C8(ctx, base);
	// 82412C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C94: 83DA0000  lwz r30, 0(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C98: 48001899  bl 0x82414530
	ctx.lr = 0x82412C9C;
	sub_82414530(ctx, base);
	// 82412C9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412CA0: 7C7EE02E  lwzx r3, r30, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412CA4: 480015C5  bl 0x82414268
	ctx.lr = 0x82412CA8;
	sub_82414268(ctx, base);
	// 82412CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412CAC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82412CB0: 48001881  bl 0x82414530
	ctx.lr = 0x82412CB4;
	sub_82414530(ctx, base);
	// 82412CB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412CB8: 418200F0  beq 0x82412da8
	if ctx.cr[0].eq {
	pc = 0x82412DA8; continue 'dispatch;
	}
	pc = 0x82412CBC; continue 'dispatch;
            }
            0x82412CBC => {
    //   block [0x82412CBC..0x82412D30)
	// 82412CBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412CC0: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412CC8: 48001901  bl 0x824145c8
	ctx.lr = 0x82412CCC;
	sub_824145C8(ctx, base);
	// 82412CCC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82412CD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412CD8: 480018C9  bl 0x824145a0
	ctx.lr = 0x82412CDC;
	sub_824145A0(ctx, base);
	// 82412CDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82412CE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412CE4: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412CE8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82412CEC: 48000C6D  bl 0x82413958
	ctx.lr = 0x82412CF0;
	sub_82413958(ctx, base);
	// 82412CF0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412CF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412CF8: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D00: 480018F1  bl 0x824145f0
	ctx.lr = 0x82412D04;
	sub_824145F0(ctx, base);
	// 82412D04: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82412D08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D0C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412D10: 48000C19  bl 0x82413928
	ctx.lr = 0x82412D14;
	sub_82413928(ctx, base);
	// 82412D14: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412D18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D20: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412D24: 480018CD  bl 0x824145f0
	ctx.lr = 0x82412D28;
	sub_824145F0(ctx, base);
	// 82412D28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412D2C: 41820068  beq 0x82412d94
	if ctx.cr[0].eq {
	pc = 0x82412D94; continue 'dispatch;
	}
	pc = 0x82412D30; continue 'dispatch;
            }
            0x82412D30 => {
    //   block [0x82412D30..0x82412D94)
	// 82412D30: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82412D34: 833A0000  lwz r25, 0(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412D38: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412D3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D44: 48001945  bl 0x82414688
	ctx.lr = 0x82412D48;
	sub_82414688(ctx, base);
	// 82412D48: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82412D4C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82412D50: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D5C: 480018D5  bl 0x82414630
	ctx.lr = 0x82412D60;
	sub_82414630(ctx, base);
	// 82412D60: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82412D64: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82412D68: 7C79E02E  lwzx r3, r25, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412D6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D70: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82412D74: 48000C45  bl 0x824139b8
	ctx.lr = 0x82412D78;
	sub_824139B8(ctx, base);
	// 82412D78: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412D7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D84: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82412D88: 48001869  bl 0x824145f0
	ctx.lr = 0x82412D8C;
	sub_824145F0(ctx, base);
	// 82412D8C: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412D90: 4198FFA0  blt cr6, 0x82412d30
	if ctx.cr[6].lt {
	pc = 0x82412D30; continue 'dispatch;
	}
	pc = 0x82412D94; continue 'dispatch;
            }
            0x82412D94 => {
    //   block [0x82412D94..0x82412DA8)
	// 82412D94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D98: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82412D9C: 48001795  bl 0x82414530
	ctx.lr = 0x82412DA0;
	sub_82414530(ctx, base);
	// 82412DA0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412DA4: 4198FF18  blt cr6, 0x82412cbc
	if ctx.cr[6].lt {
	pc = 0x82412CBC; continue 'dispatch;
	}
	pc = 0x82412DA8; continue 'dispatch;
            }
            0x82412DA8 => {
    //   block [0x82412DA8..0x82412DC0)
	// 82412DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412DAC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82412DB0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82412DB4: 48055D0D  bl 0x82468ac0
	ctx.lr = 0x82412DB8;
	sub_82468AC0(ctx, base);
	// 82412DB8: 7F1B1840  cmplw cr6, r27, r3
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412DBC: 4198FE5C  blt cr6, 0x82412c18
	if ctx.cr[6].lt {
	pc = 0x82412C18; continue 'dispatch;
	}
	pc = 0x82412DC0; continue 'dispatch;
            }
            0x82412DC0 => {
    //   block [0x82412DC0..0x82412DD4)
	// 82412DC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412DC4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412DC8: 48001769  bl 0x82414530
	ctx.lr = 0x82412DCC;
	sub_82414530(ctx, base);
	// 82412DCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412DD0: 418200B0  beq 0x82412e80
	if ctx.cr[0].eq {
	pc = 0x82412E80; continue 'dispatch;
	}
	pc = 0x82412DD4; continue 'dispatch;
            }
            0x82412DD4 => {
    //   block [0x82412DD4..0x82412E1C)
	// 82412DD4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412DD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82412DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412DE0: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82412DE4: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412DE8: 480017B9  bl 0x824145a0
	ctx.lr = 0x82412DEC;
	sub_824145A0(ctx, base);
	// 82412DEC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412DF0: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82412DF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412DFC: 4E800421  bctrl
	ctx.lr = 0x82412E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412E00: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82412E04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412E08: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82412E0C: 48055CB5  bl 0x82468ac0
	ctx.lr = 0x82412E10;
	sub_82468AC0(ctx, base);
	// 82412E10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412E14: 41820058  beq 0x82412e6c
	if ctx.cr[0].eq {
	pc = 0x82412E6C; continue 'dispatch;
	}
	// 82412E18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
            }
            0x82412E1C => {
    //   block [0x82412E1C..0x82412E44)
	// 82412E1C: 839A0000  lwz r28, 0(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412E20: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82412E24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82412E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412E2C: 419A0018  beq cr6, 0x82412e44
	if ctx.cr[6].eq {
	pc = 0x82412E44; continue 'dispatch;
	}
	// 82412E30: 48001771  bl 0x824145a0
	ctx.lr = 0x82412E34;
	sub_824145A0(ctx, base);
	// 82412E34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412E38: 7C7CF02E  lwzx r3, r28, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82412E3C: 48000ED5  bl 0x82413d10
	ctx.lr = 0x82412E40;
	sub_82413D10(ctx, base);
	// 82412E40: 48000014  b 0x82412e54
	pc = 0x82412E54; continue 'dispatch;
            }
            0x82412E44 => {
    //   block [0x82412E44..0x82412E54)
	// 82412E44: 4800175D  bl 0x824145a0
	ctx.lr = 0x82412E48;
	sub_824145A0(ctx, base);
	// 82412E48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412E4C: 7C7CF02E  lwzx r3, r28, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82412E50: 48000EF1  bl 0x82413d40
	ctx.lr = 0x82412E54;
	sub_82413D40(ctx, base);
	pc = 0x82412E54; continue 'dispatch;
            }
            0x82412E54 => {
    //   block [0x82412E54..0x82412E6C)
	// 82412E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412E58: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82412E5C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82412E60: 48055C61  bl 0x82468ac0
	ctx.lr = 0x82412E64;
	sub_82468AC0(ctx, base);
	// 82412E64: 7F1B1840  cmplw cr6, r27, r3
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412E68: 4198FFB4  blt cr6, 0x82412e1c
	if ctx.cr[6].lt {
	pc = 0x82412E1C; continue 'dispatch;
	}
	pc = 0x82412E6C; continue 'dispatch;
            }
            0x82412E6C => {
    //   block [0x82412E6C..0x82412E80)
	// 82412E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412E70: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82412E74: 480016BD  bl 0x82414530
	ctx.lr = 0x82412E78;
	sub_82414530(ctx, base);
	// 82412E78: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412E7C: 4198FF58  blt cr6, 0x82412dd4
	if ctx.cr[6].lt {
	pc = 0x82412DD4; continue 'dispatch;
	}
	pc = 0x82412E80; continue 'dispatch;
            }
            0x82412E80 => {
    //   block [0x82412E80..0x82412E88)
	// 82412E80: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82412E84: 48122270  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412E88 size=4
    let mut pc: u32 = 0x82412E88;
    'dispatch: loop {
        match pc {
            0x82412E88 => {
    //   block [0x82412E88..0x82412E8C)
	// 82412E88: 4BFFFD30  b 0x82412bb8
	sub_82412BB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412E90 size=88
    let mut pc: u32 = 0x82412E90;
    'dispatch: loop {
        match pc {
            0x82412E90 => {
    //   block [0x82412E90..0x82412ED0)
	// 82412E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412E98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412E9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412EA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82412EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412EA8: 396BEC38  addi r11, r11, -0x13c8
	ctx.r[11].s64 = ctx.r[11].s64 + -5064;
	// 82412EAC: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82412EB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82412EB4: 4182001C  beq 0x82412ed0
	if ctx.cr[0].eq {
	pc = 0x82412ED0; continue 'dispatch;
	}
	// 82412EB8: 4BFFD909  bl 0x824107c0
	ctx.lr = 0x82412EBC;
	sub_824107C0(ctx, base);
	// 82412EBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412EC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82412EC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82412EC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412ECC: 4E800421  bctrl
	ctx.lr = 0x82412ED0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82412ED0 => {
    //   block [0x82412ED0..0x82412EE8)
	// 82412ED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412ED4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412EE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412F08 size=504
    let mut pc: u32 = 0x82412F08;
    'dispatch: loop {
        match pc {
            0x82412F08 => {
    //   block [0x82412F08..0x82412F70)
	// 82412F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412F0C: 4812218D  bl 0x82535098
	ctx.lr = 0x82412F10;
	sub_82535080(ctx, base);
	// 82412F10: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412F14: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82412F18: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82412F1C: 3BFA0008  addi r31, r26, 8
	ctx.r[31].s64 = ctx.r[26].s64 + 8;
	// 82412F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412F24: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412F28: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82412F2C: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412F30: 480020E9  bl 0x82415018
	ctx.lr = 0x82412F34;
	sub_82415018(ctx, base);
	// 82412F34: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82412F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412F3C: 4800210D  bl 0x82415048
	ctx.lr = 0x82412F40;
	sub_82415048(ctx, base);
	// 82412F40: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82412F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412F48: 480020F9  bl 0x82415040
	ctx.lr = 0x82412F4C;
	sub_82415040(ctx, base);
	// 82412F4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412F50: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82412F54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412F58: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412F5C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82412F60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412F64: 4E800421  bctrl
	ctx.lr = 0x82412F68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412F68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412F6C: 40820008  bne 0x82412f74
	if !ctx.cr[0].eq {
	pc = 0x82412F74; continue 'dispatch;
	}
            }
            0x82412F70 => {
    //   block [0x82412F70..0x82412F74)
	// 82412F70: 48000000  b 0x82412f70
	pc = 0x82412F70; continue 'dispatch;
            }
            0x82412F74 => {
    //   block [0x82412F74..0x82412F94)
	// 82412F74: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412F78: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82412F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412F80: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82412F84: 832B0010  lwz r25, 0x10(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82412F88: 480015A9  bl 0x82414530
	ctx.lr = 0x82412F8C;
	sub_82414530(ctx, base);
	// 82412F8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412F90: 418200A4  beq 0x82413034
	if ctx.cr[0].eq {
	pc = 0x82413034; continue 'dispatch;
	}
	pc = 0x82412F94; continue 'dispatch;
            }
            0x82412F94 => {
    //   block [0x82412F94..0x82412FD0)
	// 82412F94: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82412F98: B3810052  sth r28, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[28].u16 ) };
	// 82412F9C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82412FA0: B3810050  sth r28, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u16 ) };
	// 82412FA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82412FA8: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82412FAC: 38810052  addi r4, r1, 0x52
	ctx.r[4].s64 = ctx.r[1].s64 + 82;
	// 82412FB0: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82412FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412FB8: 48002261  bl 0x82415218
	ctx.lr = 0x82412FBC;
	sub_82415218(ctx, base);
	// 82412FBC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412FC0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412FC4: 3B6B0004  addi r27, r11, 4
	ctx.r[27].s64 = ctx.r[11].s64 + 4;
	// 82412FC8: 40820008  bne 0x82412fd0
	if !ctx.cr[0].eq {
	pc = 0x82412FD0; continue 'dispatch;
	}
	// 82412FCC: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	pc = 0x82412FD0; continue 'dispatch;
            }
            0x82412FD0 => {
    //   block [0x82412FD0..0x82413034)
	// 82412FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412FD4: 83B90000  lwz r29, 0(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412FD8: 82E10054  lwz r23, 0x54(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82412FDC: 82C10058  lwz r22, 0x58(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82412FE0: A2A10050  lhz r21, 0x50(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82412FE4: A2810052  lhz r20, 0x52(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 82412FE8: 48001541  bl 0x82414528
	ctx.lr = 0x82412FEC;
	sub_82414528(ctx, base);
	// 82412FEC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82412FF0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82412FF4: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82412FF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82412FFC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82413000: 7E87A378  mr r7, r20
	ctx.r[7].u64 = ctx.r[20].u64;
	// 82413004: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 82413008: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 8241300C: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82413010: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82413014: 4E800421  bctrl
	ctx.lr = 0x82413018;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82413018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241301C: 48002255  bl 0x82415270
	ctx.lr = 0x82413020;
	sub_82415270(ctx, base);
	// 82413020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413024: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82413028: 48001509  bl 0x82414530
	ctx.lr = 0x8241302C;
	sub_82414530(ctx, base);
	// 8241302C: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82413030: 4198FF64  blt cr6, 0x82412f94
	if ctx.cr[6].lt {
	pc = 0x82412F94; continue 'dispatch;
	}
            }
            0x82413034 => {
    //   block [0x82413034..0x8241305C)
	// 82413034: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82413038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241303C: 48002285  bl 0x824152c0
	ctx.lr = 0x82413040;
	sub_824152C0(ctx, base);
	// 82413040: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413048: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8241304C: 832B0010  lwz r25, 0x10(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82413050: 48002001  bl 0x82415050
	ctx.lr = 0x82413054;
	sub_82415050(ctx, base);
	// 82413054: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413058: 41820094  beq 0x824130ec
	if ctx.cr[0].eq {
	pc = 0x824130EC; continue 'dispatch;
	}
	pc = 0x8241305C; continue 'dispatch;
            }
            0x8241305C => {
    //   block [0x8241305C..0x82413090)
	// 8241305C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82413060: B3810052  sth r28, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[28].u16 ) };
	// 82413064: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82413068: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8241306C: 38810052  addi r4, r1, 0x52
	ctx.r[4].s64 = ctx.r[1].s64 + 82;
	// 82413070: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82413074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413078: 48002329  bl 0x824153a0
	ctx.lr = 0x8241307C;
	sub_824153A0(ctx, base);
	// 8241307C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413080: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413084: 3B6B0004  addi r27, r11, 4
	ctx.r[27].s64 = ctx.r[11].s64 + 4;
	// 82413088: 40820008  bne 0x82413090
	if !ctx.cr[0].eq {
	pc = 0x82413090; continue 'dispatch;
	}
	// 8241308C: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	pc = 0x82413090; continue 'dispatch;
            }
            0x82413090 => {
    //   block [0x82413090..0x824130EC)
	// 82413090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413094: 83B90000  lwz r29, 0(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413098: 82E10058  lwz r23, 0x58(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8241309C: 82C10054  lwz r22, 0x54(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824130A0: A2A10052  lhz r21, 0x52(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 824130A4: 48001485  bl 0x82414528
	ctx.lr = 0x824130A8;
	sub_82414528(ctx, base);
	// 824130A8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824130AC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824130B0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 824130B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824130B8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824130BC: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 824130C0: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 824130C4: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 824130C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824130CC: 4E800421  bctrl
	ctx.lr = 0x824130D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824130D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824130D4: 48002315  bl 0x824153e8
	ctx.lr = 0x824130D8;
	sub_824153E8(ctx, base);
	// 824130D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824130DC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824130E0: 48001F71  bl 0x82415050
	ctx.lr = 0x824130E4;
	sub_82415050(ctx, base);
	// 824130E4: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824130E8: 4198FF74  blt cr6, 0x8241305c
	if ctx.cr[6].lt {
	pc = 0x8241305C; continue 'dispatch;
	}
            }
            0x824130EC => {
    //   block [0x824130EC..0x82413100)
	// 824130EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824130F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824130F4: 48002345  bl 0x82415438
	ctx.lr = 0x824130F8;
	sub_82415438(ctx, base);
	// 824130F8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824130FC: 48121FEC  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413100 size=8
    let mut pc: u32 = 0x82413100;
    'dispatch: loop {
        match pc {
            0x82413100 => {
    //   block [0x82413100..0x82413108)
	// 82413100: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82413104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413108 size=8
    let mut pc: u32 = 0x82413108;
    'dispatch: loop {
        match pc {
            0x82413108 => {
    //   block [0x82413108..0x82413110)
	// 82413108: 80630094  lwz r3, 0x94(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8241310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413110 size=20
    let mut pc: u32 = 0x82413110;
    'dispatch: loop {
        match pc {
            0x82413110 => {
    //   block [0x82413110..0x82413124)
	// 82413110: 80630098  lwz r3, 0x98(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82413114: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413118: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241311C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82413120: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413128 size=172
    let mut pc: u32 = 0x82413128;
    'dispatch: loop {
        match pc {
            0x82413128 => {
    //   block [0x82413128..0x82413194)
	// 82413128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241312C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82413130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413134: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82413138: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8241313C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82413140: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82413144: 81630088  lwz r11, 0x88(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413148: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 8241314C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413150: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82413154: 2B0A8002  cmplwi cr6, r10, 0x8002
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32770 as u32, &mut ctx.xer);
	// 82413158: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 8241315C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413160: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82413164: 419A005C  beq cr6, 0x824131c0
	if ctx.cr[6].eq {
	pc = 0x824131C0; continue 'dispatch;
	}
	// 82413168: 2B0A8003  cmplwi cr6, r10, 0x8003
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32771 as u32, &mut ctx.xer);
	// 8241316C: 419A0054  beq cr6, 0x824131c0
	if ctx.cr[6].eq {
	pc = 0x824131C0; continue 'dispatch;
	}
	// 82413170: 2B0A8001  cmplwi cr6, r10, 0x8001
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32769 as u32, &mut ctx.xer);
	// 82413174: 409A0044  bne cr6, 0x824131b8
	if !ctx.cr[6].eq {
	pc = 0x824131B8; continue 'dispatch;
	}
	// 82413178: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8241317C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82413180: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82413184: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82413188: 48002341  bl 0x824154c8
	ctx.lr = 0x8241318C;
	sub_824154C8(ctx, base);
	// 8241318C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413190: 40820008  bne 0x82413198
	if !ctx.cr[0].eq {
	pc = 0x82413198; continue 'dispatch;
	}
	pc = 0x82413194; continue 'dispatch;
            }
            0x82413194 => {
    //   block [0x82413194..0x82413198)
	// 82413194: 48000000  b 0x82413194
	pc = 0x82413194; continue 'dispatch;
            }
            0x82413198 => {
    //   block [0x82413198..0x824131AC)
	// 82413198: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241319C: 2B0B8002  cmplwi cr6, r11, 0x8002
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32770 as u32, &mut ctx.xer);
	// 824131A0: 419A000C  beq cr6, 0x824131ac
	if ctx.cr[6].eq {
	pc = 0x824131AC; continue 'dispatch;
	}
	// 824131A4: 2B0B8003  cmplwi cr6, r11, 0x8003
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32771 as u32, &mut ctx.xer);
	// 824131A8: 409A0010  bne cr6, 0x824131b8
	if !ctx.cr[6].eq {
	pc = 0x824131B8; continue 'dispatch;
	}
	pc = 0x824131AC; continue 'dispatch;
            }
            0x824131AC => {
    //   block [0x824131AC..0x824131B8)
	// 824131AC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824131B0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824131B4: 48000010  b 0x824131c4
	pc = 0x824131C4; continue 'dispatch;
            }
            0x824131B8 => {
    //   block [0x824131B8..0x824131C0)
	// 824131B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824131BC: 48000008  b 0x824131c4
	pc = 0x824131C4; continue 'dispatch;
            }
            0x824131C0 => {
    //   block [0x824131C0..0x824131C4)
	// 824131C0: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x824131C4; continue 'dispatch;
            }
            0x824131C4 => {
    //   block [0x824131C4..0x824131D4)
	// 824131C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824131C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824131CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824131D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824131D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824131D8 size=256
    let mut pc: u32 = 0x824131D8;
    'dispatch: loop {
        match pc {
            0x824131D8 => {
    //   block [0x824131D8..0x82413204)
	// 824131D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824131DC: 48121EE1  bl 0x825350bc
	ctx.lr = 0x824131E0;
	sub_82535080(ctx, base);
	// 824131E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824131E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824131E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824131EC: 817D0090  lwz r11, 0x90(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 824131F0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824131F4: 41980010  blt cr6, 0x82413204
	if ctx.cr[6].lt {
	pc = 0x82413204; continue 'dispatch;
	}
	// 824131F8: 817D0094  lwz r11, 0x94(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 824131FC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82413200: 4098000C  bge cr6, 0x8241320c
	if !ctx.cr[6].lt {
	pc = 0x8241320C; continue 'dispatch;
	}
	pc = 0x82413204; continue 'dispatch;
            }
            0x82413204 => {
    //   block [0x82413204..0x8241320C)
	// 82413204: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413208: 480000C8  b 0x824132d0
	pc = 0x824132D0; continue 'dispatch;
            }
            0x8241320C => {
    //   block [0x8241320C..0x82413240)
	// 8241320C: 3BFD0008  addi r31, r29, 8
	ctx.r[31].s64 = ctx.r[29].s64 + 8;
	// 82413210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413214: 48001E7D  bl 0x82415090
	ctx.lr = 0x82413218;
	sub_82415090(ctx, base);
	// 82413218: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241321C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413220: 48002779  bl 0x82415998
	ctx.lr = 0x82413224;
	sub_82415998(ctx, base);
	// 82413224: 817D0090  lwz r11, 0x90(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82413228: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241322C: 409A0018  bne cr6, 0x82413244
	if !ctx.cr[6].eq {
	pc = 0x82413244; continue 'dispatch;
	}
	// 82413230: 817D0088  lwz r11, 0x88(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413234: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413238: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 8241323C: 419A0090  beq cr6, 0x824132cc
	if ctx.cr[6].eq {
	pc = 0x824132CC; continue 'dispatch;
	}
	pc = 0x82413240; continue 'dispatch;
            }
            0x82413240 => {
    //   block [0x82413240..0x82413244)
	// 82413240: 48000000  b 0x82413240
	pc = 0x82413240; continue 'dispatch;
            }
            0x82413244 => {
    //   block [0x82413244..0x8241326C)
	// 82413244: 815D009C  lwz r10, 0x9c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(156 as u32) ) } as u64;
	// 82413248: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241324C: 41820020  beq 0x8241326c
	if ctx.cr[0].eq {
	pc = 0x8241326C; continue 'dispatch;
	}
	// 82413250: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 82413254: 41980018  blt cr6, 0x8241326c
	if ctx.cr[6].lt {
	pc = 0x8241326C; continue 'dispatch;
	}
	// 82413258: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241325C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413260: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82413264: 388BFFF8  addi r4, r11, -8
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	// 82413268: 48001E11  bl 0x82415078
	ctx.lr = 0x8241326C;
	sub_82415078(ctx, base);
	pc = 0x8241326C; continue 'dispatch;
            }
            0x8241326C => {
    //   block [0x8241326C..0x8241328C)
	// 8241326C: 817D0088  lwz r11, 0x88(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413270: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413274: 2B0B8002  cmplwi cr6, r11, 0x8002
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32770 as u32, &mut ctx.xer);
	// 82413278: 409A0014  bne cr6, 0x8241328c
	if !ctx.cr[6].eq {
	pc = 0x8241328C; continue 'dispatch;
	}
	// 8241327C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82413280: 4BFFFEA9  bl 0x82413128
	ctx.lr = 0x82413284;
	sub_82413128(ctx, base);
	// 82413284: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82413288: 419A0044  beq cr6, 0x824132cc
	if ctx.cr[6].eq {
	pc = 0x824132CC; continue 'dispatch;
	}
	pc = 0x8241328C; continue 'dispatch;
            }
            0x8241328C => {
    //   block [0x8241328C..0x824132C8)
	// 8241328C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413290: 80BF0084  lwz r5, 0x84(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413294: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82413298: 48001E51  bl 0x824150e8
	ctx.lr = 0x8241329C;
	sub_824150E8(ctx, base);
	// 8241329C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824132A0: 40820028  bne 0x824132c8
	if !ctx.cr[0].eq {
	pc = 0x824132C8; continue 'dispatch;
	}
	// 824132A4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824132A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824132AC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824132B0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824132B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824132B8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824132BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824132C0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824132C4: 4BFFFFA8  b 0x8241326c
	pc = 0x8241326C; continue 'dispatch;
            }
            0x824132C8 => {
    //   block [0x824132C8..0x824132CC)
	// 824132C8: 48000000  b 0x824132c8
	pc = 0x824132C8; continue 'dispatch;
            }
            0x824132CC => {
    //   block [0x824132CC..0x824132D0)
	// 824132CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x824132D0; continue 'dispatch;
            }
            0x824132D0 => {
    //   block [0x824132D0..0x824132D8)
	// 824132D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824132D4: 48121E38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824132D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824132D8 size=608
    let mut pc: u32 = 0x824132D8;
    'dispatch: loop {
        match pc {
            0x824132D8 => {
    //   block [0x824132D8..0x82413324)
	// 824132D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824132DC: 48121DD1  bl 0x825350ac
	ctx.lr = 0x824132E0;
	sub_82535080(ctx, base);
	// 824132E0: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 824132E4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824132E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824132EC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824132F0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 824132F4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 824132F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824132FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413300: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413304: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82413308: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241330C: 83AA0010  lwz r29, 0x10(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82413310: 419A0014  beq cr6, 0x82413324
	if ctx.cr[6].eq {
	pc = 0x82413324; continue 'dispatch;
	}
	// 82413314: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413318: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241331C: 3B6B0004  addi r27, r11, 4
	ctx.r[27].s64 = ctx.r[11].s64 + 4;
	// 82413320: 48000008  b 0x82413328
	pc = 0x82413328; continue 'dispatch;
            }
            0x82413324 => {
    //   block [0x82413324..0x82413328)
	// 82413324: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	pc = 0x82413328; continue 'dispatch;
            }
            0x82413328 => {
    //   block [0x82413328..0x82413360)
	// 82413328: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241332C: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82413330: 480011F9  bl 0x82414528
	ctx.lr = 0x82413334;
	sub_82414528(ctx, base);
	// 82413334: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413338: B3E10060  sth r31, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u16 ) };
	// 8241333C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82413340: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82413344: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82413348: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241334C: A14A0000  lhz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413350: 2B0A8002  cmplwi cr6, r10, 0x8002
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32770 as u32, &mut ctx.xer);
	// 82413354: 419A0010  beq cr6, 0x82413364
	if ctx.cr[6].eq {
	pc = 0x82413364; continue 'dispatch;
	}
	// 82413358: 2B0A8001  cmplwi cr6, r10, 0x8001
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32769 as u32, &mut ctx.xer);
	// 8241335C: 419A0008  beq cr6, 0x82413364
	if ctx.cr[6].eq {
	pc = 0x82413364; continue 'dispatch;
	}
	pc = 0x82413360; continue 'dispatch;
            }
            0x82413360 => {
    //   block [0x82413360..0x82413364)
	// 82413360: 48000000  b 0x82413360
	pc = 0x82413360; continue 'dispatch;
            }
            0x82413364 => {
    //   block [0x82413364..0x824133B8)
	// 82413364: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413368: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8241336C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82413370: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82413374: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82413378: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8241337C: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413380: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82413384: B1210060  sth r9, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u16 ) };
	// 82413388: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241338C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82413390: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82413394: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413398: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241339C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824133A0: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824133A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824133A8: 81680020  lwz r11, 0x20(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 824133AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824133B0: 4E800421  bctrl
	ctx.lr = 0x824133B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824133B4: 48000094  b 0x82413448
	pc = 0x82413448; continue 'dispatch;
            }
            0x824133B8 => {
    //   block [0x824133B8..0x824133F4)
	// 824133B8: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 824133BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824133C0: 80BF0084  lwz r5, 0x84(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824133C4: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824133C8: 48001D21  bl 0x824150e8
	ctx.lr = 0x824133CC;
	sub_824150E8(ctx, base);
	// 824133CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824133D0: 40820024  bne 0x824133f4
	if !ctx.cr[0].eq {
	pc = 0x824133F4; continue 'dispatch;
	}
	// 824133D4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824133D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824133DC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824133E0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824133E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824133E8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824133EC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824133F0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	pc = 0x824133F4; continue 'dispatch;
            }
            0x824133F4 => {
    //   block [0x824133F4..0x82413448)
	// 824133F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824133F8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824133FC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82413400: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82413404: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82413408: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8241340C: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413410: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82413414: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413418: B1210060  sth r9, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u16 ) };
	// 8241341C: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413420: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82413424: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82413428: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241342C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413430: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82413434: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413438: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8241343C: 81680020  lwz r11, 0x20(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 82413440: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82413444: 4E800421  bctrl
	ctx.lr = 0x82413448;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82413448 => {
    //   block [0x82413448..0x82413474)
	// 82413448: A1610060  lhz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241344C: 2B0B8003  cmplwi cr6, r11, 0x8003
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32771 as u32, &mut ctx.xer);
	// 82413450: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413454: 409AFF64  bne cr6, 0x824133b8
	if !ctx.cr[6].eq {
	pc = 0x824133B8; continue 'dispatch;
	}
	// 82413458: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 8241345C: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82413460: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82413464: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82413468: 48002061  bl 0x824154c8
	ctx.lr = 0x8241346C;
	sub_824154C8(ctx, base);
	// 8241346C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413470: 40820008  bne 0x82413478
	if !ctx.cr[0].eq {
	pc = 0x82413478; continue 'dispatch;
	}
	pc = 0x82413474; continue 'dispatch;
            }
            0x82413474 => {
    //   block [0x82413474..0x82413478)
	// 82413474: 48000000  b 0x82413474
	pc = 0x82413474; continue 'dispatch;
            }
            0x82413478 => {
    //   block [0x82413478..0x824134C4)
	// 82413478: A1610060  lhz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241347C: 2B0B8004  cmplwi cr6, r11, 0x8004
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32772 as u32, &mut ctx.xer);
	// 82413480: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413484: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82413488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241348C: 80BF0084  lwz r5, 0x84(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413490: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82413494: 409A006C  bne cr6, 0x82413500
	if !ctx.cr[6].eq {
	pc = 0x82413500; continue 'dispatch;
	}
	// 82413498: 48001C51  bl 0x824150e8
	ctx.lr = 0x8241349C;
	sub_824150E8(ctx, base);
	// 8241349C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824134A0: 40820024  bne 0x824134c4
	if !ctx.cr[0].eq {
	pc = 0x824134C4; continue 'dispatch;
	}
	// 824134A4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824134A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824134AC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824134B0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824134B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824134B8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824134BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824134C0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	pc = 0x824134C4; continue 'dispatch;
            }
            0x824134C4 => {
    //   block [0x824134C4..0x82413500)
	// 824134C4: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824134C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824134CC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824134D0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 824134D4: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824134D8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 824134DC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824134E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824134E4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 824134E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824134EC: A1210060  lhz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824134F0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824134F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824134F8: 4E800421  bctrl
	ctx.lr = 0x824134FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824134FC: 48000030  b 0x8241352c
	pc = 0x8241352C; continue 'dispatch;
            }
            0x82413500 => {
    //   block [0x82413500..0x8241352C)
	// 82413500: 48001BE9  bl 0x824150e8
	ctx.lr = 0x82413504;
	sub_824150E8(ctx, base);
	// 82413504: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413508: 40820024  bne 0x8241352c
	if !ctx.cr[0].eq {
	pc = 0x8241352C; continue 'dispatch;
	}
	// 8241350C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413510: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82413514: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82413518: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8241351C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413520: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82413524: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82413528: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	pc = 0x8241352C; continue 'dispatch;
            }
            0x8241352C => {
    //   block [0x8241352C..0x82413538)
	// 8241352C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82413530: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82413534: 48121BC8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82413538 size=96
    let mut pc: u32 = 0x82413538;
    'dispatch: loop {
        match pc {
            0x82413538 => {
    //   block [0x82413538..0x8241358C)
	// 82413538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241353C: 48121B81  bl 0x825350bc
	ctx.lr = 0x82413540;
	sub_82535080(ctx, base);
	// 82413540: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82413544: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413548: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8241354C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82413550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413554: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82413558: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241355C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413560: FC00FE5E  fctidz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[31].f64.trunc() as i64 };
	// 82413564: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82413568: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241356C: 4BFFFC6D  bl 0x824131d8
	ctx.lr = 0x82413570;
	sub_824131D8(ctx, base);
	// 82413570: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413574: 41820018  beq 0x8241358c
	if ctx.cr[0].eq {
	pc = 0x8241358C; continue 'dispatch;
	}
	// 82413578: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241357C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82413580: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82413584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413588: 4BFFFD51  bl 0x824132d8
	ctx.lr = 0x8241358C;
	sub_824132D8(ctx, base);
	pc = 0x8241358C; continue 'dispatch;
            }
            0x8241358C => {
    //   block [0x8241358C..0x82413598)
	// 8241358C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413590: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82413594: 48121B78  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413598 size=652
    let mut pc: u32 = 0x82413598;
    'dispatch: loop {
        match pc {
            0x82413598 => {
    //   block [0x82413598..0x82413618)
	// 82413598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241359C: 48121B11  bl 0x825350ac
	ctx.lr = 0x824135A0;
	sub_82535080(ctx, base);
	// 824135A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824135A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824135A8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824135AC: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 824135B0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824135B4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 824135B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824135BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824135C0: 480023E9  bl 0x824159a8
	ctx.lr = 0x824135C4;
	sub_824159A8(ctx, base);
	// 824135C4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824135C8: 3B9F0098  addi r28, r31, 0x98
	ctx.r[28].s64 = ctx.r[31].s64 + 152;
	// 824135CC: 3B3F009C  addi r25, r31, 0x9c
	ctx.r[25].s64 = ctx.r[31].s64 + 156;
	// 824135D0: 935F0090  stw r26, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[26].u32 ) };
	// 824135D4: 935F0094  stw r26, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[26].u32 ) };
	// 824135D8: 935F0098  stw r26, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[26].u32 ) };
	// 824135DC: 935F009C  stw r26, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[26].u32 ) };
	// 824135E0: 4BFFD1E1  bl 0x824107c0
	ctx.lr = 0x824135E4;
	sub_824107C0(ctx, base);
	// 824135E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824135E8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824135EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824135F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824135F4: 4E800421  bctrl
	ctx.lr = 0x824135F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824135F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824135FC: 4182001C  beq 0x82413618
	if ctx.cr[0].eq {
	pc = 0x82413618; continue 'dispatch;
	}
	// 82413600: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82413604: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82413608: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8241360C: 396BEC40  addi r11, r11, -0x13c0
	ctx.r[11].s64 = ctx.r[11].s64 + -5056;
	// 82413610: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82413614: 48000008  b 0x8241361c
	pc = 0x8241361C; continue 'dispatch;
            }
            0x82413618 => {
    //   block [0x82413618..0x8241361C)
	// 82413618: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	pc = 0x8241361C; continue 'dispatch;
            }
            0x8241361C => {
    //   block [0x8241361C..0x82413644)
	// 8241361C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82413620: 4BFFD1B9  bl 0x824107d8
	ctx.lr = 0x82413624;
	sub_824107D8(ctx, base);
	// 82413624: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82413628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241362C: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82413630: 48002369  bl 0x82415998
	ctx.lr = 0x82413634;
	sub_82415998(ctx, base);
	// 82413634: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413638: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241363C: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 82413640: 419A0008  beq cr6, 0x82413648
	if ctx.cr[6].eq {
	pc = 0x82413648; continue 'dispatch;
	}
	pc = 0x82413644; continue 'dispatch;
            }
            0x82413644 => {
    //   block [0x82413644..0x82413648)
	// 82413644: 48000000  b 0x82413644
	pc = 0x82413644; continue 'dispatch;
            }
            0x82413648 => {
    //   block [0x82413648..0x82413680)
	// 82413648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241364C: 80BE0084  lwz r5, 0x84(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413650: 809E0080  lwz r4, 0x80(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82413654: 48001A95  bl 0x824150e8
	ctx.lr = 0x82413658;
	sub_824150E8(ctx, base);
	// 82413658: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241365C: 40820024  bne 0x82413680
	if !ctx.cr[0].eq {
	pc = 0x82413680; continue 'dispatch;
	}
	// 82413660: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413664: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82413668: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8241366C: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82413670: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413674: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82413678: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8241367C: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	pc = 0x82413680; continue 'dispatch;
            }
            0x82413680 => {
    //   block [0x82413680..0x82413690)
	// 82413680: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413684: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413688: 2B0B8002  cmplwi cr6, r11, 0x8002
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32770 as u32, &mut ctx.xer);
	// 8241368C: 419A0008  beq cr6, 0x82413694
	if ctx.cr[6].eq {
	pc = 0x82413694; continue 'dispatch;
	}
	pc = 0x82413690; continue 'dispatch;
            }
            0x82413690 => {
    //   block [0x82413690..0x82413694)
	// 82413690: 48000000  b 0x82413690
	pc = 0x82413690; continue 'dispatch;
            }
            0x82413694 => {
    //   block [0x82413694..0x824136A8)
	// 82413694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413698: 4BFFFA91  bl 0x82413128
	ctx.lr = 0x8241369C;
	sub_82413128(ctx, base);
	// 8241369C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824136A0: 907F0090  stw r3, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[3].u32 ) };
	// 824136A4: 40820008  bne 0x824136ac
	if !ctx.cr[0].eq {
	pc = 0x824136AC; continue 'dispatch;
	}
	pc = 0x824136A8; continue 'dispatch;
            }
            0x824136A8 => {
    //   block [0x824136A8..0x824136AC)
	// 824136A8: 48000000  b 0x824136a8
	pc = 0x824136A8; continue 'dispatch;
            }
            0x824136AC => {
    //   block [0x824136AC..0x824136BC)
	// 824136AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824136B0: 480019A9  bl 0x82415058
	ctx.lr = 0x824136B4;
	sub_82415058(ctx, base);
	// 824136B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824136B8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	pc = 0x824136BC; continue 'dispatch;
            }
            0x824136BC => {
    //   block [0x824136BC..0x824136E4)
	// 824136BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824136C0: 419A0024  beq cr6, 0x824136e4
	if ctx.cr[6].eq {
	pc = 0x824136E4; continue 'dispatch;
	}
	// 824136C4: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 824136C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824136CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824136D0: 480022C9  bl 0x82415998
	ctx.lr = 0x824136D4;
	sub_82415998(ctx, base);
	// 824136D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824136D8: 4BFFFA51  bl 0x82413128
	ctx.lr = 0x824136DC;
	sub_82413128(ctx, base);
	// 824136DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824136E0: 4182FFDC  beq 0x824136bc
	if ctx.cr[0].eq {
	pc = 0x824136BC; continue 'dispatch;
	}
	pc = 0x824136E4; continue 'dispatch;
            }
            0x824136E4 => {
    //   block [0x824136E4..0x824136F0)
	// 824136E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824136E8: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 824136EC: 409A0008  bne cr6, 0x824136f4
	if !ctx.cr[6].eq {
	pc = 0x824136F4; continue 'dispatch;
	}
	pc = 0x824136F0; continue 'dispatch;
            }
            0x824136F0 => {
    //   block [0x824136F0..0x824136F4)
	// 824136F0: 48000000  b 0x824136f0
	pc = 0x824136F0; continue 'dispatch;
            }
            0x824136F4 => {
    //   block [0x824136F4..0x82413700)
	// 824136F4: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 824136F8: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824136FC: 40990008  ble cr6, 0x82413704
	if !ctx.cr[6].gt {
	pc = 0x82413704; continue 'dispatch;
	}
	pc = 0x82413700; continue 'dispatch;
            }
            0x82413700 => {
    //   block [0x82413700..0x82413704)
	// 82413700: 48000000  b 0x82413700
	pc = 0x82413700; continue 'dispatch;
            }
            0x82413704 => {
    //   block [0x82413704..0x82413738)
	// 82413704: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82413708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241370C: 4800228D  bl 0x82415998
	ctx.lr = 0x82413710;
	sub_82415998(ctx, base);
	// 82413710: 83BF0094  lwz r29, 0x94(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82413714: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413718: 93BF00A0  stw r29, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[29].u32 ) };
	// 8241371C: 418200FC  beq 0x82413818
	if ctx.cr[0].eq {
	pc = 0x82413818; continue 'dispatch;
	}
	// 82413720: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82413724: 57BC1838  slwi r28, r29, 3
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82413728: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8241372C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413730: 40990008  ble cr6, 0x82413738
	if !ctx.cr[6].gt {
	pc = 0x82413738; continue 'dispatch;
	}
	// 82413734: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	pc = 0x82413738; continue 'dispatch;
            }
            0x82413738 => {
    //   block [0x82413738..0x82413764)
	// 82413738: 4BFFD089  bl 0x824107c0
	ctx.lr = 0x8241373C;
	sub_824107C0(ctx, base);
	// 8241373C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413740: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82413744: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413748: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241374C: 4E800421  bctrl
	ctx.lr = 0x82413750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82413750: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413754: 4182002C  beq 0x82413780
	if ctx.cr[0].eq {
	pc = 0x82413780; continue 'dispatch;
	}
	// 82413758: 357DFFFF  addic. r11, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241375C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82413760: 41800018  blt 0x82413778
	if ctx.cr[0].lt {
	pc = 0x82413778; continue 'dispatch;
	}
            }
            0x82413764 => {
    //   block [0x82413764..0x82413778)
	// 82413764: 934A0000  stw r26, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82413768: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241376C: 934A0004  stw r26, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82413770: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82413774: 4080FFF0  bge 0x82413764
	if !ctx.cr[0].lt {
	pc = 0x82413764; continue 'dispatch;
	}
	pc = 0x82413778; continue 'dispatch;
            }
            0x82413778 => {
    //   block [0x82413778..0x82413780)
	// 82413778: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8241377C: 48000008  b 0x82413784
	pc = 0x82413784; continue 'dispatch;
            }
            0x82413780 => {
    //   block [0x82413780..0x82413784)
	// 82413780: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	pc = 0x82413784; continue 'dispatch;
            }
            0x82413784 => {
    //   block [0x82413784..0x82413798)
	// 82413784: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82413788: 48001991  bl 0x82415118
	ctx.lr = 0x8241378C;
	sub_82415118(ctx, base);
	// 8241378C: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82413790: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82413794: 93790000  stw r27, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	pc = 0x82413798; continue 'dispatch;
            }
            0x82413798 => {
    //   block [0x82413798..0x824137C0)
	// 82413798: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241379C: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824137A0: 2B0B8002  cmplwi cr6, r11, 0x8002
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32770 as u32, &mut ctx.xer);
	// 824137A4: 409A001C  bne cr6, 0x824137c0
	if !ctx.cr[6].eq {
	pc = 0x824137C0; continue 'dispatch;
	}
	// 824137A8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824137AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824137B0: 7C9D5A14  add r4, r29, r11
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 824137B4: 480018AD  bl 0x82415060
	ctx.lr = 0x824137B8;
	sub_82415060(ctx, base);
	// 824137B8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824137BC: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	pc = 0x824137C0; continue 'dispatch;
            }
            0x824137C0 => {
    //   block [0x824137C0..0x824137FC)
	// 824137C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824137C4: 80BE0084  lwz r5, 0x84(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 824137C8: 809E0080  lwz r4, 0x80(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 824137CC: 4800191D  bl 0x824150e8
	ctx.lr = 0x824137D0;
	sub_824150E8(ctx, base);
	// 824137D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824137D4: 40820028  bne 0x824137fc
	if !ctx.cr[0].eq {
	pc = 0x824137FC; continue 'dispatch;
	}
	// 824137D8: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 824137DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824137E0: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824137E4: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 824137E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824137EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824137F0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824137F4: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824137F8: 4BFFFFA0  b 0x82413798
	pc = 0x82413798; continue 'dispatch;
            }
            0x824137FC => {
    //   block [0x824137FC..0x82413808)
	// 824137FC: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82413800: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413804: 419A0008  beq cr6, 0x8241380c
	if ctx.cr[6].eq {
	pc = 0x8241380C; continue 'dispatch;
	}
	pc = 0x82413808; continue 'dispatch;
            }
            0x82413808 => {
    //   block [0x82413808..0x8241380C)
	// 82413808: 48000000  b 0x82413808
	pc = 0x82413808; continue 'dispatch;
            }
            0x8241380C => {
    //   block [0x8241380C..0x82413818)
	// 8241380C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82413810: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82413814: 48002185  bl 0x82415998
	ctx.lr = 0x82413818;
	sub_82415998(ctx, base);
	pc = 0x82413818; continue 'dispatch;
            }
            0x82413818 => {
    //   block [0x82413818..0x82413824)
	// 82413818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241381C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82413820: 481218DC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413828 size=64
    let mut pc: u32 = 0x82413828;
    'dispatch: loop {
        match pc {
            0x82413828 => {
    //   block [0x82413828..0x82413868)
	// 82413828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82413830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82413834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241383C: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 82413840: 480018D9  bl 0x82415118
	ctx.lr = 0x82413844;
	sub_82415118(ctx, base);
	// 82413844: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 82413848: 4BFFCF91  bl 0x824107d8
	ctx.lr = 0x8241384C;
	sub_824107D8(ctx, base);
	// 8241384C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82413850: 480020F9  bl 0x82415948
	ctx.lr = 0x82413854;
	sub_82415948(ctx, base);
	// 82413854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82413858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241385C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82413860: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82413864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413868 size=92
    let mut pc: u32 = 0x82413868;
    'dispatch: loop {
        match pc {
            0x82413868 => {
    //   block [0x82413868..0x824138A8)
	// 82413868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241386C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82413870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82413874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82413878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241387C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413880: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82413884: 48002825  bl 0x824160a8
	ctx.lr = 0x82413888;
	sub_824160A8(ctx, base);
	// 82413888: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241388C: 4182001C  beq 0x824138a8
	if ctx.cr[0].eq {
	pc = 0x824138A8; continue 'dispatch;
	}
	// 82413890: 4BFFCF31  bl 0x824107c0
	ctx.lr = 0x82413894;
	sub_824107C0(ctx, base);
	// 82413894: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413898: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241389C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824138A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824138A4: 4E800421  bctrl
	ctx.lr = 0x824138A8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824138A8 => {
    //   block [0x824138A8..0x824138C4)
	// 824138A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824138AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824138B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824138B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824138B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824138BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824138C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824138C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824138C8 size=16
    let mut pc: u32 = 0x824138C8;
    'dispatch: loop {
        match pc {
            0x824138C8 => {
    //   block [0x824138C8..0x824138D4)
	// 824138C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824138CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824138D0: 419A0008  beq cr6, 0x824138d8
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x824138D8);
		return;
	}
	pc = 0x824138D4; continue 'dispatch;
            }
            0x824138D4 => {
    //   block [0x824138D4..0x824138D8)
	// 824138D4: 48000000  b 0x824138d4
	pc = 0x824138D4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824138F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824138F8 size=36
    let mut pc: u32 = 0x824138F8;
    'dispatch: loop {
        match pc {
            0x824138F8 => {
    //   block [0x824138F8..0x82413914)
	// 824138F8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824138FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413900: 419A0014  beq cr6, 0x82413914
	if ctx.cr[6].eq {
	pc = 0x82413914; continue 'dispatch;
	}
	// 82413904: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82413908: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241390C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413910: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	pc = 0x82413914; continue 'dispatch;
            }
            0x82413914 => {
    //   block [0x82413914..0x8241391C)
	// 82413914: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413920 size=8
    let mut pc: u32 = 0x82413920;
    'dispatch: loop {
        match pc {
            0x82413920 => {
    //   block [0x82413920..0x82413928)
	// 82413920: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413928 size=28
    let mut pc: u32 = 0x82413928;
    'dispatch: loop {
        match pc {
            0x82413928 => {
    //   block [0x82413928..0x82413940)
	// 82413928: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241392C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82413930: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413934: 4182000C  beq 0x82413940
	if ctx.cr[0].eq {
	pc = 0x82413940; continue 'dispatch;
	}
	// 82413938: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241393C: 41980008  blt cr6, 0x82413944
	if ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x82413944);
		return;
	}
	pc = 0x82413940; continue 'dispatch;
            }
            0x82413940 => {
    //   block [0x82413940..0x82413944)
	// 82413940: 48000000  b 0x82413940
	pc = 0x82413940; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413958 size=92
    let mut pc: u32 = 0x82413958;
    'dispatch: loop {
        match pc {
            0x82413958 => {
    //   block [0x82413958..0x82413984)
	// 82413958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241395C: 48121761  bl 0x825350bc
	ctx.lr = 0x82413960;
	sub_82535080(ctx, base);
	// 82413960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413968: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8241396C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82413970: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413974: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413978: 4182000C  beq 0x82413984
	if ctx.cr[0].eq {
	pc = 0x82413984; continue 'dispatch;
	}
	// 8241397C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82413980: 41980008  blt cr6, 0x82413988
	if ctx.cr[6].lt {
	pc = 0x82413988; continue 'dispatch;
	}
	pc = 0x82413984; continue 'dispatch;
            }
            0x82413984 => {
    //   block [0x82413984..0x82413988)
	// 82413984: 48000000  b 0x82413984
	pc = 0x82413984; continue 'dispatch;
            }
            0x82413988 => {
    //   block [0x82413988..0x824139B4)
	// 82413988: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8241398C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413990: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82413994: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82413998: 480020E1  bl 0x82415a78
	ctx.lr = 0x8241399C;
	sub_82415A78(ctx, base);
	// 8241399C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824139A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824139A4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824139A8: 480020E9  bl 0x82415a90
	ctx.lr = 0x824139AC;
	sub_82415A90(ctx, base);
	// 824139AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824139B0: 4812175C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824139B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824139B8 size=36
    let mut pc: u32 = 0x824139B8;
    'dispatch: loop {
        match pc {
            0x824139B8 => {
    //   block [0x824139B8..0x824139D8)
	// 824139B8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824139BC: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 824139C0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824139C4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 824139C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824139CC: 4182000C  beq 0x824139d8
	if ctx.cr[0].eq {
	pc = 0x824139D8; continue 'dispatch;
	}
	// 824139D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824139D4: 41980008  blt cr6, 0x824139dc
	if ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x824139DC);
		return;
	}
	pc = 0x824139D8; continue 'dispatch;
            }
            0x824139D8 => {
    //   block [0x824139D8..0x824139DC)
	// 824139D8: 48000000  b 0x824139d8
	pc = 0x824139D8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824139F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824139F0 size=408
    let mut pc: u32 = 0x824139F0;
    'dispatch: loop {
        match pc {
            0x824139F0 => {
    //   block [0x824139F0..0x82413A8C)
	// 824139F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824139F4: 481216C9  bl 0x825350bc
	ctx.lr = 0x824139F8;
	sub_82535080(ctx, base);
	// 824139F8: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 824139FC: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82413A00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413A08: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82413A0C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82413A10: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413A14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82413A18: 419A0118  beq cr6, 0x82413b30
	if ctx.cr[6].eq {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A1C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82413A20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413A24: 419A010C  beq cr6, 0x82413b30
	if ctx.cr[6].eq {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A28: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82413A2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413A30: 419A0100  beq cr6, 0x82413b30
	if ctx.cr[6].eq {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A34: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82413A38: FC00FE5E  fctidz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[31].f64.trunc() as i64 };
	// 82413A3C: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82413A40: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82413A44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82413A48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82413A4C: 419800E4  blt cr6, 0x82413b30
	if ctx.cr[6].lt {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A50: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413A54: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413A58: 419800D8  blt cr6, 0x82413b30
	if ctx.cr[6].lt {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A5C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82413A60: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82413A64: 40810120  ble 0x82413b84
	if !ctx.cr[0].gt {
	pc = 0x82413B84; continue 'dispatch;
	}
	// 82413A68: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82413A6C: 409900D4  ble cr6, 0x82413b40
	if !ctx.cr[6].gt {
	pc = 0x82413B40; continue 'dispatch;
	}
	// 82413A70: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82413A74: 409A0110  bne cr6, 0x82413b84
	if !ctx.cr[6].eq {
	pc = 0x82413B84; continue 'dispatch;
	}
	// 82413A78: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413A7C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413A80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413A84: 40990054  ble cr6, 0x82413ad8
	if !ctx.cr[6].gt {
	pc = 0x82413AD8; continue 'dispatch;
	}
	// 82413A88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82413A8C; continue 'dispatch;
            }
            0x82413A8C => {
    //   block [0x82413A8C..0x82413ABC)
	// 82413A8C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413A90: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413A94: 48000A9D  bl 0x82414530
	ctx.lr = 0x82413A98;
	sub_82414530(ctx, base);
	// 82413A98: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82413A9C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413AA0: 419A001C  beq cr6, 0x82413abc
	if ctx.cr[6].eq {
	pc = 0x82413ABC; continue 'dispatch;
	}
	// 82413AA4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413AA8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413AAC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413AB0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413AB4: 4198FFD8  blt cr6, 0x82413a8c
	if ctx.cr[6].lt {
	pc = 0x82413A8C; continue 'dispatch;
	}
	// 82413AB8: 48000020  b 0x82413ad8
	pc = 0x82413AD8; continue 'dispatch;
            }
            0x82413ABC => {
    //   block [0x82413ABC..0x82413AD8)
	// 82413ABC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413AC0: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82413AC4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413AC8: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82413ACC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82413AD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413AD4: 48002765  bl 0x82416238
	ctx.lr = 0x82413AD8;
	sub_82416238(ctx, base);
	pc = 0x82413AD8; continue 'dispatch;
            }
            0x82413AD8 => {
    //   block [0x82413AD8..0x82413AEC)
	// 82413AD8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413ADC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413AE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413AE4: 4099004C  ble cr6, 0x82413b30
	if !ctx.cr[6].gt {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413AE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82413AEC; continue 'dispatch;
            }
            0x82413AEC => {
    //   block [0x82413AEC..0x82413B1C)
	// 82413AEC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413AF0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413AF4: 48000A3D  bl 0x82414530
	ctx.lr = 0x82413AF8;
	sub_82414530(ctx, base);
	// 82413AF8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82413AFC: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413B00: 419A001C  beq cr6, 0x82413b1c
	if ctx.cr[6].eq {
	pc = 0x82413B1C; continue 'dispatch;
	}
	// 82413B04: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413B08: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82413B0C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413B10: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82413B14: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413B18: 48002721  bl 0x82416238
	ctx.lr = 0x82413B1C;
	sub_82416238(ctx, base);
	pc = 0x82413B1C; continue 'dispatch;
            }
            0x82413B1C => {
    //   block [0x82413B1C..0x82413B30)
	// 82413B1C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413B20: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413B24: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413B28: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413B2C: 4198FFC0  blt cr6, 0x82413aec
	if ctx.cr[6].lt {
	pc = 0x82413AEC; continue 'dispatch;
	}
	pc = 0x82413B30; continue 'dispatch;
            }
            0x82413B30 => {
    //   block [0x82413B30..0x82413B40)
	// 82413B30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82413B34: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82413B38: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82413B3C: 481215D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82413B40 => {
    //   block [0x82413B40..0x82413B54)
	// 82413B40: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413B44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413B48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413B4C: 4099FFE4  ble cr6, 0x82413b30
	if !ctx.cr[6].gt {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413B50: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82413B54; continue 'dispatch;
            }
            0x82413B54 => {
    //   block [0x82413B54..0x82413B84)
	// 82413B54: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413B58: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82413B5C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413B60: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82413B64: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413B68: 480026D1  bl 0x82416238
	ctx.lr = 0x82413B6C;
	sub_82416238(ctx, base);
	// 82413B6C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413B70: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413B74: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413B78: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413B7C: 4198FFD8  blt cr6, 0x82413b54
	if ctx.cr[6].lt {
	pc = 0x82413B54; continue 'dispatch;
	}
	// 82413B80: 4BFFFFB0  b 0x82413b30
	pc = 0x82413B30; continue 'dispatch;
            }
            0x82413B84 => {
    //   block [0x82413B84..0x82413B88)
	// 82413B84: 48000000  b 0x82413b84
	pc = 0x82413B84; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413B88 size=152
    let mut pc: u32 = 0x82413B88;
    'dispatch: loop {
        match pc {
            0x82413B88 => {
    //   block [0x82413B88..0x82413BBC)
	// 82413B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413B8C: 4812152D  bl 0x825350b8
	ctx.lr = 0x82413B90;
	sub_82535080(ctx, base);
	// 82413B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413B94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413B98: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82413B9C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82413BA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413BA4: 419A0038  beq cr6, 0x82413bdc
	if ctx.cr[6].eq {
	pc = 0x82413BDC; continue 'dispatch;
	}
	// 82413BA8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413BAC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82413BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413BB4: 40990028  ble cr6, 0x82413bdc
	if !ctx.cr[6].gt {
	pc = 0x82413BDC; continue 'dispatch;
	}
	// 82413BB8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	pc = 0x82413BBC; continue 'dispatch;
            }
            0x82413BBC => {
    //   block [0x82413BBC..0x82413BDC)
	// 82413BBC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413BC0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413BC4: 4800213D  bl 0x82415d00
	ctx.lr = 0x82413BC8;
	sub_82415D00(ctx, base);
	// 82413BC8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413BCC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413BD0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413BD4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413BD8: 4198FFE4  blt cr6, 0x82413bbc
	if ctx.cr[6].lt {
	pc = 0x82413BBC; continue 'dispatch;
	}
	pc = 0x82413BDC; continue 'dispatch;
            }
            0x82413BDC => {
    //   block [0x82413BDC..0x82413C08)
	// 82413BDC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82413BE0: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 82413BE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413BE8: 419A0020  beq cr6, 0x82413c08
	if ctx.cr[6].eq {
	pc = 0x82413C08; continue 'dispatch;
	}
	// 82413BEC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413BF0: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413BF4: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82413BF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413BFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82413C00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82413C04: 4E800421  bctrl
	ctx.lr = 0x82413C08;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82413C08 => {
    //   block [0x82413C08..0x82413C20)
	// 82413C08: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82413C0C: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82413C10: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82413C14: 939F0020  stw r28, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82413C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413C1C: 481214EC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413C20 size=120
    let mut pc: u32 = 0x82413C20;
    'dispatch: loop {
        match pc {
            0x82413C20 => {
    //   block [0x82413C20..0x82413C4C)
	// 82413C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413C24: 48121495  bl 0x825350b8
	ctx.lr = 0x82413C28;
	sub_82535080(ctx, base);
	// 82413C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413C30: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82413C34: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82413C38: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413C3C: 41980010  blt cr6, 0x82413c4c
	if ctx.cr[6].lt {
	pc = 0x82413C4C; continue 'dispatch;
	}
	// 82413C40: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413C44: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82413C48: 4098000C  bge cr6, 0x82413c54
	if !ctx.cr[6].lt {
	pc = 0x82413C54; continue 'dispatch;
	}
	pc = 0x82413C4C; continue 'dispatch;
            }
            0x82413C4C => {
    //   block [0x82413C4C..0x82413C54)
	// 82413C4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413C50: 48000040  b 0x82413c90
	pc = 0x82413C90; continue 'dispatch;
            }
            0x82413C54 => {
    //   block [0x82413C54..0x82413C68)
	// 82413C54: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413C58: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413C5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413C60: 4099002C  ble cr6, 0x82413c8c
	if !ctx.cr[6].gt {
	pc = 0x82413C8C; continue 'dispatch;
	}
	// 82413C64: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82413C68; continue 'dispatch;
            }
            0x82413C68 => {
    //   block [0x82413C68..0x82413C8C)
	// 82413C68: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413C6C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82413C70: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82413C74: 48002115  bl 0x82415d88
	ctx.lr = 0x82413C78;
	sub_82415D88(ctx, base);
	// 82413C78: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413C7C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413C80: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413C84: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413C88: 4198FFE0  blt cr6, 0x82413c68
	if ctx.cr[6].lt {
	pc = 0x82413C68; continue 'dispatch;
	}
	pc = 0x82413C8C; continue 'dispatch;
            }
            0x82413C8C => {
    //   block [0x82413C8C..0x82413C90)
	// 82413C8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82413C90; continue 'dispatch;
            }
            0x82413C90 => {
    //   block [0x82413C90..0x82413C98)
	// 82413C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413C94: 48121474  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82413C98 size=116
    let mut pc: u32 = 0x82413C98;
    'dispatch: loop {
        match pc {
            0x82413C98 => {
    //   block [0x82413C98..0x82413CBC)
	// 82413C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82413CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413CA4: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413CA8: 3924FFFF  addi r9, r4, -1
	ctx.r[9].s64 = ctx.r[4].s64 + -1;
	// 82413CAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413CB0: 4182000C  beq 0x82413cbc
	if ctx.cr[0].eq {
	pc = 0x82413CBC; continue 'dispatch;
	}
	// 82413CB4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413CB8: 4198000C  blt cr6, 0x82413cc4
	if ctx.cr[6].lt {
	pc = 0x82413CC4; continue 'dispatch;
	}
	pc = 0x82413CBC; continue 'dispatch;
            }
            0x82413CBC => {
    //   block [0x82413CBC..0x82413CC4)
	// 82413CBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413CC0: 4800003C  b 0x82413cfc
	pc = 0x82413CFC; continue 'dispatch;
            }
            0x82413CC4 => {
    //   block [0x82413CC4..0x82413CFC)
	// 82413CC4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82413CC8: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82413CCC: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82413CD0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82413CD4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82413CD8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82413CDC: 4198FFE0  blt cr6, 0x82413cbc
	if ctx.cr[6].lt {
	pc = 0x82413CBC; continue 'dispatch;
	}
	// 82413CE0: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413CE4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413CE8: 4198FFD4  blt cr6, 0x82413cbc
	if ctx.cr[6].lt {
	pc = 0x82413CBC; continue 'dispatch;
	}
	// 82413CEC: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413CF0: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82413CF4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82413CF8: 48002669  bl 0x82416360
	ctx.lr = 0x82413CFC;
	sub_82416360(ctx, base);
	pc = 0x82413CFC; continue 'dispatch;
            }
            0x82413CFC => {
    //   block [0x82413CFC..0x82413D0C)
	// 82413CFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82413D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82413D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82413D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413D10 size=28
    let mut pc: u32 = 0x82413D10;
    'dispatch: loop {
        match pc {
            0x82413D10 => {
    //   block [0x82413D10..0x82413D28)
	// 82413D10: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413D14: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82413D18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413D1C: 4182000C  beq 0x82413d28
	if ctx.cr[0].eq {
	pc = 0x82413D28; continue 'dispatch;
	}
	// 82413D20: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413D24: 41980008  blt cr6, 0x82413d2c
	if ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x82413D2C);
		return;
	}
	pc = 0x82413D28; continue 'dispatch;
            }
            0x82413D28 => {
    //   block [0x82413D28..0x82413D2C)
	// 82413D28: 48000000  b 0x82413d28
	pc = 0x82413D28; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413D40 size=28
    let mut pc: u32 = 0x82413D40;
    'dispatch: loop {
        match pc {
            0x82413D40 => {
    //   block [0x82413D40..0x82413D58)
	// 82413D40: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413D44: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82413D48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413D4C: 4182000C  beq 0x82413d58
	if ctx.cr[0].eq {
	pc = 0x82413D58; continue 'dispatch;
	}
	// 82413D50: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413D54: 41980008  blt cr6, 0x82413d5c
	if ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x82413D5C);
		return;
	}
	pc = 0x82413D58; continue 'dispatch;
            }
            0x82413D58 => {
    //   block [0x82413D58..0x82413D5C)
	// 82413D58: 48000000  b 0x82413d58
	pc = 0x82413D58; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413D70 size=112
    let mut pc: u32 = 0x82413D70;
    'dispatch: loop {
        match pc {
            0x82413D70 => {
    //   block [0x82413D70..0x82413D9C)
	// 82413D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413D74: 48121341  bl 0x825350b4
	ctx.lr = 0x82413D78;
	sub_82535080(ctx, base);
	// 82413D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413D7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413D80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82413D84: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82413D88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413D8C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413D90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413D94: 40990034  ble cr6, 0x82413dc8
	if !ctx.cr[6].gt {
	pc = 0x82413DC8; continue 'dispatch;
	}
	// 82413D98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82413D9C; continue 'dispatch;
            }
            0x82413D9C => {
    //   block [0x82413D9C..0x82413DC8)
	// 82413D9C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413DA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82413DA4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82413DA8: 48002551  bl 0x824162f8
	ctx.lr = 0x82413DAC;
	sub_824162F8(ctx, base);
	// 82413DAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413DB0: 40820024  bne 0x82413dd4
	if !ctx.cr[0].eq {
	pc = 0x82413DD4; continue 'dispatch;
	}
	// 82413DB4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413DB8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413DBC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413DC0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413DC4: 4198FFD8  blt cr6, 0x82413d9c
	if ctx.cr[6].lt {
	pc = 0x82413D9C; continue 'dispatch;
	}
	pc = 0x82413DC8; continue 'dispatch;
            }
            0x82413DC8 => {
    //   block [0x82413DC8..0x82413DCC)
	// 82413DC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82413DCC; continue 'dispatch;
            }
            0x82413DCC => {
    //   block [0x82413DCC..0x82413DD4)
	// 82413DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413DD0: 48121334  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82413DD4 => {
    //   block [0x82413DD4..0x82413DE0)
	// 82413DD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82413DD8: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82413DDC: 4BFFFFF0  b 0x82413dcc
	pc = 0x82413DCC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413DE0 size=112
    let mut pc: u32 = 0x82413DE0;
    'dispatch: loop {
        match pc {
            0x82413DE0 => {
    //   block [0x82413DE0..0x82413E0C)
	// 82413DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413DE4: 481212D1  bl 0x825350b4
	ctx.lr = 0x82413DE8;
	sub_82535080(ctx, base);
	// 82413DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413DF0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82413DF4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82413DF8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413DFC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413E00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413E04: 40990034  ble cr6, 0x82413e38
	if !ctx.cr[6].gt {
	pc = 0x82413E38; continue 'dispatch;
	}
	// 82413E08: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82413E0C; continue 'dispatch;
            }
            0x82413E0C => {
    //   block [0x82413E0C..0x82413E38)
	// 82413E0C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413E10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82413E14: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82413E18: 48002521  bl 0x82416338
	ctx.lr = 0x82413E1C;
	sub_82416338(ctx, base);
	// 82413E1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413E20: 40820024  bne 0x82413e44
	if !ctx.cr[0].eq {
	pc = 0x82413E44; continue 'dispatch;
	}
	// 82413E24: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413E28: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413E2C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413E30: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413E34: 4198FFD8  blt cr6, 0x82413e0c
	if ctx.cr[6].lt {
	pc = 0x82413E0C; continue 'dispatch;
	}
	pc = 0x82413E38; continue 'dispatch;
            }
            0x82413E38 => {
    //   block [0x82413E38..0x82413E3C)
	// 82413E38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82413E3C; continue 'dispatch;
            }
            0x82413E3C => {
    //   block [0x82413E3C..0x82413E44)
	// 82413E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413E40: 481212C4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82413E44 => {
    //   block [0x82413E44..0x82413E50)
	// 82413E44: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82413E48: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82413E4C: 4BFFFFF0  b 0x82413e3c
	pc = 0x82413E3C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413E50 size=124
    let mut pc: u32 = 0x82413E50;
    'dispatch: loop {
        match pc {
            0x82413E50 => {
    //   block [0x82413E50..0x82413E88)
	// 82413E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413E54: 48121269  bl 0x825350bc
	ctx.lr = 0x82413E58;
	sub_82535080(ctx, base);
	// 82413E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413E60: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413E64: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413E68: 48001C41  bl 0x82415aa8
	ctx.lr = 0x82413E6C;
	sub_82415AA8(ctx, base);
	// 82413E6C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413E70: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413E74: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82413E78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413E7C: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82413E80: 40990044  ble cr6, 0x82413ec4
	if !ctx.cr[6].gt {
	pc = 0x82413EC4; continue 'dispatch;
	}
	// 82413E84: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	pc = 0x82413E88; continue 'dispatch;
            }
            0x82413E88 => {
    //   block [0x82413E88..0x82413EB0)
	// 82413E88: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413E8C: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413E90: 48001C39  bl 0x82415ac8
	ctx.lr = 0x82413E94;
	sub_82415AC8(ctx, base);
	// 82413E94: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413E98: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82413E9C: 41990014  bgt cr6, 0x82413eb0
	if ctx.cr[6].gt {
	pc = 0x82413EB0; continue 'dispatch;
	}
	// 82413EA0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413EA4: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413EA8: 48001C21  bl 0x82415ac8
	ctx.lr = 0x82413EAC;
	sub_82415AC8(ctx, base);
	// 82413EAC: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	pc = 0x82413EB0; continue 'dispatch;
            }
            0x82413EB0 => {
    //   block [0x82413EB0..0x82413EC4)
	// 82413EB0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413EB4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413EB8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413EBC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413EC0: 4198FFC8  blt cr6, 0x82413e88
	if ctx.cr[6].lt {
	pc = 0x82413E88; continue 'dispatch;
	}
	pc = 0x82413EC4; continue 'dispatch;
            }
            0x82413EC4 => {
    //   block [0x82413EC4..0x82413ECC)
	// 82413EC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82413EC8: 48121244  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413ED0 size=136
    let mut pc: u32 = 0x82413ED0;
    'dispatch: loop {
        match pc {
            0x82413ED0 => {
    //   block [0x82413ED0..0x82413F08)
	// 82413ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413ED4: 481211E9  bl 0x825350bc
	ctx.lr = 0x82413ED8;
	sub_82535080(ctx, base);
	// 82413ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413EDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413EE0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413EE4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413EE8: 48001BC1  bl 0x82415aa8
	ctx.lr = 0x82413EEC;
	sub_82415AA8(ctx, base);
	// 82413EEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413EF0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413EF4: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82413EF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413EFC: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82413F00: 40990050  ble cr6, 0x82413f50
	if !ctx.cr[6].gt {
	pc = 0x82413F50; continue 'dispatch;
	}
	// 82413F04: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	pc = 0x82413F08; continue 'dispatch;
            }
            0x82413F08 => {
    //   block [0x82413F08..0x82413F2C)
	// 82413F08: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413F0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413F10: 419A001C  beq cr6, 0x82413f2c
	if ctx.cr[6].eq {
	pc = 0x82413F2C; continue 'dispatch;
	}
	// 82413F14: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413F18: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413F1C: 48001BAD  bl 0x82415ac8
	ctx.lr = 0x82413F20;
	sub_82415AC8(ctx, base);
	// 82413F20: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413F24: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413F28: 40980014  bge cr6, 0x82413f3c
	if !ctx.cr[6].lt {
	pc = 0x82413F3C; continue 'dispatch;
	}
	pc = 0x82413F2C; continue 'dispatch;
            }
            0x82413F2C => {
    //   block [0x82413F2C..0x82413F3C)
	// 82413F2C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413F30: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413F34: 48001B95  bl 0x82415ac8
	ctx.lr = 0x82413F38;
	sub_82415AC8(ctx, base);
	// 82413F38: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	pc = 0x82413F3C; continue 'dispatch;
            }
            0x82413F3C => {
    //   block [0x82413F3C..0x82413F50)
	// 82413F3C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413F40: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413F44: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413F48: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413F4C: 4198FFBC  blt cr6, 0x82413f08
	if ctx.cr[6].lt {
	pc = 0x82413F08; continue 'dispatch;
	}
	pc = 0x82413F50; continue 'dispatch;
            }
            0x82413F50 => {
    //   block [0x82413F50..0x82413F58)
	// 82413F50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82413F54: 481211B8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413F58 size=188
    let mut pc: u32 = 0x82413F58;
    'dispatch: loop {
        match pc {
            0x82413F58 => {
    //   block [0x82413F58..0x82413F7C)
	// 82413F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413F5C: 4812115D  bl 0x825350b8
	ctx.lr = 0x82413F60;
	sub_82535080(ctx, base);
	// 82413F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413F68: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82413F6C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413F70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413F74: 40990070  ble cr6, 0x82413fe4
	if !ctx.cr[6].gt {
	pc = 0x82413FE4; continue 'dispatch;
	}
	// 82413F78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82413F7C; continue 'dispatch;
            }
            0x82413F7C => {
    //   block [0x82413F7C..0x82413FA4)
	// 82413F7C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413F80: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413F84: 480005AD  bl 0x82414530
	ctx.lr = 0x82413F88;
	sub_82414530(ctx, base);
	// 82413F88: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82413F8C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413F90: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413F94: 419A0058  beq cr6, 0x82413fec
	if ctx.cr[6].eq {
	pc = 0x82413FEC; continue 'dispatch;
	}
	// 82413F98: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82413F9C: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413FA0: 48054B21  bl 0x82468ac0
	ctx.lr = 0x82413FA4;
	sub_82468AC0(ctx, base);
	pc = 0x82413FA4; continue 'dispatch;
            }
            0x82413FA4 => {
    //   block [0x82413FA4..0x82413FC8)
	// 82413FA4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413FA8: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413FAC: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82413FB0: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82413FB4: 41820014  beq 0x82413fc8
	if ctx.cr[0].eq {
	pc = 0x82413FC8; continue 'dispatch;
	}
	// 82413FB8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82413FBC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82413FC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82413FC4: 419AFFE0  beq cr6, 0x82413fa4
	if ctx.cr[6].eq {
	pc = 0x82413FA4; continue 'dispatch;
	}
	pc = 0x82413FC8; continue 'dispatch;
            }
            0x82413FC8 => {
    //   block [0x82413FC8..0x82413FE4)
	// 82413FC8: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82413FCC: 4182001C  beq 0x82413fe8
	if ctx.cr[0].eq {
	pc = 0x82413FE8; continue 'dispatch;
	}
	// 82413FD0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413FD4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82413FD8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82413FDC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413FE0: 4198FF9C  blt cr6, 0x82413f7c
	if ctx.cr[6].lt {
	pc = 0x82413F7C; continue 'dispatch;
	}
	pc = 0x82413FE4; continue 'dispatch;
            }
            0x82413FE4 => {
    //   block [0x82413FE4..0x82413FE8)
	// 82413FE4: 48000000  b 0x82413fe4
	pc = 0x82413FE4; continue 'dispatch;
            }
            0x82413FE8 => {
    //   block [0x82413FE8..0x82413FEC)
	// 82413FE8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	pc = 0x82413FEC; continue 'dispatch;
            }
            0x82413FEC => {
    //   block [0x82413FEC..0x82414014)
	// 82413FEC: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82413FF0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413FF4: 48001AB5  bl 0x82415aa8
	ctx.lr = 0x82413FF8;
	sub_82415AA8(ctx, base);
	// 82413FF8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413FFC: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82414000: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82414004: 48001AC5  bl 0x82415ac8
	ctx.lr = 0x82414008;
	sub_82415AC8(ctx, base);
	// 82414008: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 8241400C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414010: 481210F8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414018 size=80
    let mut pc: u32 = 0x82414018;
    'dispatch: loop {
        match pc {
            0x82414018 => {
    //   block [0x82414018..0x8241404C)
	// 82414018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241401C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82414024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241402C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414034: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414038: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241403C: 41820010  beq 0x8241404c
	if ctx.cr[0].eq {
	pc = 0x8241404C; continue 'dispatch;
	}
	// 82414040: 4811EB79  bl 0x82532bb8
	ctx.lr = 0x82414044;
	sub_82532BB8(ctx, base);
	// 82414044: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414048: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8241404C; continue 'dispatch;
            }
            0x8241404C => {
    //   block [0x8241404C..0x82414068)
	// 8241404C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82414050: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82414054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241405C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82414060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82414064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414068 size=76
    let mut pc: u32 = 0x82414068;
    'dispatch: loop {
        match pc {
            0x82414068 => {
    //   block [0x82414068..0x824140AC)
	// 82414068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241406C: 48121051  bl 0x825350bc
	ctx.lr = 0x82414070;
	sub_82535080(ctx, base);
	// 82414070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414074: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82414078: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241407C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414080: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414084: 4BFFFCED  bl 0x82413d70
	ctx.lr = 0x82414088;
	sub_82413D70(ctx, base);
	// 82414088: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241408C: 41820020  beq 0x824140ac
	if ctx.cr[0].eq {
	pc = 0x824140AC; continue 'dispatch;
	}
	// 82414090: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82414094: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82414098: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241409C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824140A0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824140A4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824140A8: 48001E51  bl 0x82415ef8
	ctx.lr = 0x824140AC;
	sub_82415EF8(ctx, base);
	pc = 0x824140AC; continue 'dispatch;
            }
            0x824140AC => {
    //   block [0x824140AC..0x824140B4)
	// 824140AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824140B0: 4812105C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824140B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824140B8 size=76
    let mut pc: u32 = 0x824140B8;
    'dispatch: loop {
        match pc {
            0x824140B8 => {
    //   block [0x824140B8..0x824140FC)
	// 824140B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824140BC: 48121001  bl 0x825350bc
	ctx.lr = 0x824140C0;
	sub_82535080(ctx, base);
	// 824140C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824140C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824140C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824140CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824140D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824140D4: 4BFFFC9D  bl 0x82413d70
	ctx.lr = 0x824140D8;
	sub_82413D70(ctx, base);
	// 824140D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824140DC: 41820020  beq 0x824140fc
	if ctx.cr[0].eq {
	pc = 0x824140FC; continue 'dispatch;
	}
	// 824140E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824140E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824140E8: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824140EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824140F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824140F4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824140F8: 48001E61  bl 0x82415f58
	ctx.lr = 0x824140FC;
	sub_82415F58(ctx, base);
	pc = 0x824140FC; continue 'dispatch;
            }
            0x824140FC => {
    //   block [0x824140FC..0x82414104)
	// 824140FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414100: 4812100C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414108 size=76
    let mut pc: u32 = 0x82414108;
    'dispatch: loop {
        match pc {
            0x82414108 => {
    //   block [0x82414108..0x8241414C)
	// 82414108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241410C: 48120FB1  bl 0x825350bc
	ctx.lr = 0x82414110;
	sub_82535080(ctx, base);
	// 82414110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414114: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82414118: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241411C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414120: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414124: 4BFFFC4D  bl 0x82413d70
	ctx.lr = 0x82414128;
	sub_82413D70(ctx, base);
	// 82414128: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241412C: 41820020  beq 0x8241414c
	if ctx.cr[0].eq {
	pc = 0x8241414C; continue 'dispatch;
	}
	// 82414130: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82414134: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82414138: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241413C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414140: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82414144: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82414148: 48001E71  bl 0x82415fb8
	ctx.lr = 0x8241414C;
	sub_82415FB8(ctx, base);
	pc = 0x8241414C; continue 'dispatch;
            }
            0x8241414C => {
    //   block [0x8241414C..0x82414154)
	// 8241414C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414150: 48120FBC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414158 size=76
    let mut pc: u32 = 0x82414158;
    'dispatch: loop {
        match pc {
            0x82414158 => {
    //   block [0x82414158..0x8241419C)
	// 82414158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241415C: 48120F61  bl 0x825350bc
	ctx.lr = 0x82414160;
	sub_82535080(ctx, base);
	// 82414160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414164: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82414168: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241416C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414170: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414174: 4BFFFBFD  bl 0x82413d70
	ctx.lr = 0x82414178;
	sub_82413D70(ctx, base);
	// 82414178: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241417C: 41820020  beq 0x8241419c
	if ctx.cr[0].eq {
	pc = 0x8241419C; continue 'dispatch;
	}
	// 82414180: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82414184: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82414188: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241418C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414190: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82414194: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82414198: 48001E81  bl 0x82416018
	ctx.lr = 0x8241419C;
	sub_82416018(ctx, base);
	pc = 0x8241419C; continue 'dispatch;
            }
            0x8241419C => {
    //   block [0x8241419C..0x824141A4)
	// 8241419C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824141A0: 48120F6C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824141A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824141A8 size=56
    let mut pc: u32 = 0x824141A8;
    'dispatch: loop {
        match pc {
            0x824141A8 => {
    //   block [0x824141A8..0x824141E0)
	// 824141A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824141AC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824141B0: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824141B4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824141B8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824141BC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824141C0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824141C4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824141C8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824141CC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824141D0: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824141D4: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824141D8: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824141DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824141E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824141E0 size=136
    let mut pc: u32 = 0x824141E0;
    'dispatch: loop {
        match pc {
            0x824141E0 => {
    //   block [0x824141E0..0x8241420C)
	// 824141E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824141E4: 48120ED5  bl 0x825350b8
	ctx.lr = 0x824141E8;
	sub_82535080(ctx, base);
	// 824141E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824141EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824141F0: 4BFFF999  bl 0x82413b88
	ctx.lr = 0x824141F4;
	sub_82413B88(ctx, base);
	// 824141F4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824141F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824141FC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82414200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82414204: 40990048  ble cr6, 0x8241424c
	if !ctx.cr[6].gt {
	pc = 0x8241424C; continue 'dispatch;
	}
	// 82414208: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	pc = 0x8241420C; continue 'dispatch;
            }
            0x8241420C => {
    //   block [0x8241420C..0x82414230)
	// 8241420C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414210: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82414214: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82414218: 419A0020  beq cr6, 0x82414238
	if ctx.cr[6].eq {
	pc = 0x82414238; continue 'dispatch;
	}
	// 8241421C: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82414220: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414224: 4182000C  beq 0x82414230
	if ctx.cr[0].eq {
	pc = 0x82414230; continue 'dispatch;
	}
	// 82414228: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241422C: 4BFFF63D  bl 0x82413868
	ctx.lr = 0x82414230;
	sub_82413868(ctx, base);
	pc = 0x82414230; continue 'dispatch;
            }
            0x82414230 => {
    //   block [0x82414230..0x82414238)
	// 82414230: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414234: 7F9E592E  stwx r28, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	pc = 0x82414238; continue 'dispatch;
            }
            0x82414238 => {
    //   block [0x82414238..0x8241424C)
	// 82414238: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241423C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82414240: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82414244: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414248: 4198FFC4  blt cr6, 0x8241420c
	if ctx.cr[6].lt {
	pc = 0x8241420C; continue 'dispatch;
	}
	pc = 0x8241424C; continue 'dispatch;
            }
            0x8241424C => {
    //   block [0x8241424C..0x82414260)
	// 8241424C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414250: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414254: 4182000C  beq 0x82414260
	if ctx.cr[0].eq {
	pc = 0x82414260; continue 'dispatch;
	}
	// 82414258: 4811E961  bl 0x82532bb8
	ctx.lr = 0x8241425C;
	sub_82532BB8(ctx, base);
	// 8241425C: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	pc = 0x82414260; continue 'dispatch;
            }
            0x82414260 => {
    //   block [0x82414260..0x82414268)
	// 82414260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414264: 48120EA4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414268 size=184
    let mut pc: u32 = 0x82414268;
    'dispatch: loop {
        match pc {
            0x82414268 => {
    //   block [0x82414268..0x82414288)
	// 82414268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241426C: 48120E4D  bl 0x825350b8
	ctx.lr = 0x82414270;
	sub_82535080(ctx, base);
	// 82414270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414278: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 8241427C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414280: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414284: 419A0008  beq cr6, 0x8241428c
	if ctx.cr[6].eq {
	pc = 0x8241428C; continue 'dispatch;
	}
	pc = 0x82414288; continue 'dispatch;
            }
            0x82414288 => {
    //   block [0x82414288..0x8241428C)
	// 82414288: 48000000  b 0x82414288
	pc = 0x82414288; continue 'dispatch;
            }
            0x8241428C => {
    //   block [0x8241428C..0x824142A8)
	// 8241428C: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82414290: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82414294: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82414298: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8241429C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824142A0: 40990008  ble cr6, 0x824142a8
	if !ctx.cr[6].gt {
	pc = 0x824142A8; continue 'dispatch;
	}
	// 824142A4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x824142A8; continue 'dispatch;
            }
            0x824142A8 => {
    //   block [0x824142A8..0x824142CC)
	// 824142A8: 4BFF0B21  bl 0x82404dc8
	ctx.lr = 0x824142AC;
	sub_82404DC8(ctx, base);
	// 824142AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824142B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824142B4: 4BFFFD65  bl 0x82414018
	ctx.lr = 0x824142B8;
	sub_82414018(ctx, base);
	// 824142B8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824142BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824142C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824142C4: 40990054  ble cr6, 0x82414318
	if !ctx.cr[6].gt {
	pc = 0x82414318; continue 'dispatch;
	}
	// 824142C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x824142CC; continue 'dispatch;
            }
            0x824142CC => {
    //   block [0x824142CC..0x824142F8)
	// 824142CC: 4BFFC4F5  bl 0x824107c0
	ctx.lr = 0x824142D0;
	sub_824107C0(ctx, base);
	// 824142D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824142D4: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 824142D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824142DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824142E0: 4E800421  bctrl
	ctx.lr = 0x824142E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824142E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824142E8: 41820010  beq 0x824142f8
	if ctx.cr[0].eq {
	pc = 0x824142F8; continue 'dispatch;
	}
	// 824142EC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824142F0: 48001D89  bl 0x82416078
	ctx.lr = 0x824142F4;
	sub_82416078(ctx, base);
	// 824142F4: 48000008  b 0x824142fc
	pc = 0x824142FC; continue 'dispatch;
            }
            0x824142F8 => {
    //   block [0x824142F8..0x824142FC)
	// 824142F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x824142FC; continue 'dispatch;
            }
            0x824142FC => {
    //   block [0x824142FC..0x82414318)
	// 824142FC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414300: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82414304: 7C7D592E  stwx r3, r29, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82414308: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8241430C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82414310: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414314: 4198FFB8  blt cr6, 0x824142cc
	if ctx.cr[6].lt {
	pc = 0x824142CC; continue 'dispatch;
	}
	pc = 0x82414318; continue 'dispatch;
            }
            0x82414318 => {
    //   block [0x82414318..0x82414320)
	// 82414318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241431C: 48120DEC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414320 size=380
    let mut pc: u32 = 0x82414320;
    'dispatch: loop {
        match pc {
            0x82414320 => {
    //   block [0x82414320..0x82414368)
	// 82414320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414324: 48120D91  bl 0x825350b4
	ctx.lr = 0x82414328;
	sub_82535080(ctx, base);
	// 82414328: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241432C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414330: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82414334: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82414338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241433C: 409A0038  bne cr6, 0x82414374
	if !ctx.cr[6].eq {
	pc = 0x82414374; continue 'dispatch;
	}
	// 82414340: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414344: 937F001C  stw r27, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[27].u32 ) };
	// 82414348: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241434C: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414350: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414354: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241435C: 4E800421  bctrl
	ctx.lr = 0x82414360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414360: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414364: 4082000C  bne 0x82414370
	if !ctx.cr[0].eq {
	pc = 0x82414370; continue 'dispatch;
	}
            }
            0x82414368 => {
    //   block [0x82414368..0x82414370)
	// 82414368: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241436C: 48000128  b 0x82414494
	pc = 0x82414494; continue 'dispatch;
            }
            0x82414370 => {
    //   block [0x82414370..0x82414374)
	// 82414370: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	pc = 0x82414374; continue 'dispatch;
            }
            0x82414374 => {
    //   block [0x82414374..0x824143A4)
	// 82414374: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82414378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241437C: 409A0114  bne cr6, 0x82414490
	if !ctx.cr[6].eq {
	pc = 0x82414490; continue 'dispatch;
	}
	// 82414380: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 82414384: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82414388: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8241438C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82414390: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82414394: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82414398: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8241439C: 40990008  ble cr6, 0x824143a4
	if !ctx.cr[6].gt {
	pc = 0x824143A4; continue 'dispatch;
	}
	// 824143A0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x824143A4; continue 'dispatch;
            }
            0x824143A4 => {
    //   block [0x824143A4..0x824143CC)
	// 824143A4: 4BFF0A25  bl 0x82404dc8
	ctx.lr = 0x824143A8;
	sub_82404DC8(ctx, base);
	// 824143A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824143AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824143B0: 4BFFFC69  bl 0x82414018
	ctx.lr = 0x824143B4;
	sub_82414018(ctx, base);
	// 824143B4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824143B8: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824143BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824143C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824143C4: 40990030  ble cr6, 0x824143f4
	if !ctx.cr[6].gt {
	pc = 0x824143F4; continue 'dispatch;
	}
	// 824143C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x824143CC; continue 'dispatch;
            }
            0x824143CC => {
    //   block [0x824143CC..0x824143F4)
	// 824143CC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824143D0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824143D4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824143D8: 48001771  bl 0x82415b48
	ctx.lr = 0x824143DC;
	sub_82415B48(ctx, base);
	// 824143DC: 7C7EE12E  stwx r3, r30, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32), ctx.r[3].u32) };
	// 824143E0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824143E4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824143E8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824143EC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824143F0: 4198FFDC  blt cr6, 0x824143cc
	if ctx.cr[6].lt {
	pc = 0x824143CC; continue 'dispatch;
	}
	pc = 0x824143F4; continue 'dispatch;
            }
            0x824143F4 => {
    //   block [0x824143F4..0x82414408)
	// 824143F4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824143F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824143FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82414400: 40990028  ble cr6, 0x82414428
	if !ctx.cr[6].gt {
	pc = 0x82414428; continue 'dispatch;
	}
	// 82414404: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	pc = 0x82414408; continue 'dispatch;
            }
            0x82414408 => {
    //   block [0x82414408..0x82414428)
	// 82414408: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241440C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82414410: 419A0038  beq cr6, 0x82414448
	if ctx.cr[6].eq {
	pc = 0x82414448; continue 'dispatch;
	}
	// 82414414: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82414418: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241441C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82414420: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82414424: 4198FFE4  blt cr6, 0x82414408
	if ctx.cr[6].lt {
	pc = 0x82414408; continue 'dispatch;
	}
	pc = 0x82414428; continue 'dispatch;
            }
            0x82414428 => {
    //   block [0x82414428..0x82414444)
	// 82414428: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241442C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82414430: 419A0044  beq cr6, 0x82414474
	if ctx.cr[6].eq {
	pc = 0x82414474; continue 'dispatch;
	}
	// 82414434: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82414438: 419A0030  beq cr6, 0x82414468
	if ctx.cr[6].eq {
	pc = 0x82414468; continue 'dispatch;
	}
	// 8241443C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82414440: 419A001C  beq cr6, 0x8241445c
	if ctx.cr[6].eq {
	pc = 0x8241445C; continue 'dispatch;
	}
	pc = 0x82414444; continue 'dispatch;
            }
            0x82414444 => {
    //   block [0x82414444..0x82414448)
	// 82414444: 48000000  b 0x82414444
	pc = 0x82414444; continue 'dispatch;
            }
            0x82414448 => {
    //   block [0x82414448..0x8241445C)
	// 82414448: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8241444C: 419AFF1C  beq cr6, 0x82414368
	if ctx.cr[6].eq {
	pc = 0x82414368; continue 'dispatch;
	}
	// 82414450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82414454: 4811E765  bl 0x82532bb8
	ctx.lr = 0x82414458;
	sub_82532BB8(ctx, base);
	// 82414458: 4BFFFF10  b 0x82414368
	pc = 0x82414368; continue 'dispatch;
            }
            0x8241445C => {
    //   block [0x8241445C..0x82414468)
	// 8241445C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414460: 4BFFFAF9  bl 0x82413f58
	ctx.lr = 0x82414464;
	sub_82413F58(ctx, base);
	// 82414464: 48000018  b 0x8241447c
	pc = 0x8241447C; continue 'dispatch;
            }
            0x82414468 => {
    //   block [0x82414468..0x82414474)
	// 82414468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241446C: 4BFFFA65  bl 0x82413ed0
	ctx.lr = 0x82414470;
	sub_82413ED0(ctx, base);
	// 82414470: 4800000C  b 0x8241447c
	pc = 0x8241447C; continue 'dispatch;
            }
            0x82414474 => {
    //   block [0x82414474..0x8241447C)
	// 82414474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414478: 4BFFF9D9  bl 0x82413e50
	ctx.lr = 0x8241447C;
	sub_82413E50(ctx, base);
	pc = 0x8241447C; continue 'dispatch;
            }
            0x8241447C => {
    //   block [0x8241447C..0x82414490)
	// 8241447C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82414480: 937F0024  stw r27, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 82414484: 419A000C  beq cr6, 0x82414490
	if ctx.cr[6].eq {
	pc = 0x82414490; continue 'dispatch;
	}
	// 82414488: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8241448C: 4811E72D  bl 0x82532bb8
	ctx.lr = 0x82414490;
	sub_82532BB8(ctx, base);
	pc = 0x82414490; continue 'dispatch;
            }
            0x82414490 => {
    //   block [0x82414490..0x82414494)
	// 82414490: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82414494; continue 'dispatch;
            }
            0x82414494 => {
    //   block [0x82414494..0x8241449C)
	// 82414494: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82414498: 48120C6C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824144A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824144A0 size=36
    let mut pc: u32 = 0x824144A0;
    'dispatch: loop {
        match pc {
            0x824144A0 => {
    //   block [0x824144A0..0x824144C4)
	// 824144A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824144A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824144A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824144AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824144B0: 4BFFF8C1  bl 0x82413d70
	ctx.lr = 0x824144B4;
	sub_82413D70(ctx, base);
	// 824144B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824144B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824144BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824144C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824144C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824144C8 size=36
    let mut pc: u32 = 0x824144C8;
    'dispatch: loop {
        match pc {
            0x824144C8 => {
    //   block [0x824144C8..0x824144EC)
	// 824144C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824144CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824144D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824144D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824144D8: 4BFFF909  bl 0x82413de0
	ctx.lr = 0x824144DC;
	sub_82413DE0(ctx, base);
	// 824144DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824144E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824144E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824144E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824144F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824144F0 size=40
    let mut pc: u32 = 0x824144F0;
    'dispatch: loop {
        match pc {
            0x824144F0 => {
    //   block [0x824144F0..0x82414518)
	// 824144F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824144F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824144F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824144FC: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82414500: 48001F21  bl 0x82416420
	ctx.lr = 0x82414504;
	sub_82416420(ctx, base);
	// 82414504: 5463063E  clrlwi r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82414508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241450C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414518 size=8
    let mut pc: u32 = 0x82414518;
    'dispatch: loop {
        match pc {
            0x82414518 => {
    //   block [0x82414518..0x82414520)
	// 82414518: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 8241451C: 48001F3C  b 0x82416458
	sub_82416458(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414520 size=8
    let mut pc: u32 = 0x82414520;
    'dispatch: loop {
        match pc {
            0x82414520 => {
    //   block [0x82414520..0x82414528)
	// 82414520: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82414524: 48001F54  b 0x82416478
	sub_82416478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414528 size=8
    let mut pc: u32 = 0x82414528;
    'dispatch: loop {
        match pc {
            0x82414528 => {
    //   block [0x82414528..0x82414530)
	// 82414528: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241452C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414530 size=8
    let mut pc: u32 = 0x82414530;
    'dispatch: loop {
        match pc {
            0x82414530 => {
    //   block [0x82414530..0x82414538)
	// 82414530: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414538 size=8
    let mut pc: u32 = 0x82414538;
    'dispatch: loop {
        match pc {
            0x82414538 => {
    //   block [0x82414538..0x82414540)
	// 82414538: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241453C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414540 size=8
    let mut pc: u32 = 0x82414540;
    'dispatch: loop {
        match pc {
            0x82414540 => {
    //   block [0x82414540..0x82414548)
	// 82414540: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414548 size=88
    let mut pc: u32 = 0x82414548;
    'dispatch: loop {
        match pc {
            0x82414548 => {
    //   block [0x82414548..0x82414580)
	// 82414548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241454C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241455C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82414560: 38801001  li r4, 0x1001
	ctx.r[4].s64 = 4097;
	// 82414564: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82414568: 48001FA9  bl 0x82416510
	ctx.lr = 0x8241456C;
	sub_82416510(ctx, base);
	// 8241456C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414570: 4182001C  beq 0x8241458c
	if ctx.cr[0].eq {
	pc = 0x8241458C; continue 'dispatch;
	}
	// 82414574: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414578: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241457C: 40820008  bne 0x82414584
	if !ctx.cr[0].eq {
	pc = 0x82414584; continue 'dispatch;
	}
	pc = 0x82414580; continue 'dispatch;
            }
            0x82414580 => {
    //   block [0x82414580..0x82414584)
	// 82414580: 48000000  b 0x82414580
	pc = 0x82414580; continue 'dispatch;
            }
            0x82414584 => {
    //   block [0x82414584..0x8241458C)
	// 82414584: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82414588: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x8241458C; continue 'dispatch;
            }
            0x8241458C => {
    //   block [0x8241458C..0x824145A0)
	// 8241458C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82414590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241459C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824145A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824145A0 size=20
    let mut pc: u32 = 0x824145A0;
    'dispatch: loop {
        match pc {
            0x824145A0 => {
    //   block [0x824145A0..0x824145B4)
	// 824145A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824145A4: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824145A8: 4199000C  bgt cr6, 0x824145b4
	if ctx.cr[6].gt {
		crate::recompiler::externs::call(ctx, base, 0x824145B4);
		return;
	}
	// 824145AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824145B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824145C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824145C8 size=20
    let mut pc: u32 = 0x824145C8;
    'dispatch: loop {
        match pc {
            0x824145C8 => {
    //   block [0x824145C8..0x824145DC)
	// 824145C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824145CC: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824145D0: 4199000C  bgt cr6, 0x824145dc
	if ctx.cr[6].gt {
		crate::recompiler::externs::call(ctx, base, 0x824145DC);
		return;
	}
	// 824145D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824145D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824145F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824145F0 size=20
    let mut pc: u32 = 0x824145F0;
    'dispatch: loop {
        match pc {
            0x824145F0 => {
    //   block [0x824145F0..0x82414604)
	// 824145F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824145F4: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824145F8: 4199000C  bgt cr6, 0x82414604
	if ctx.cr[6].gt {
		crate::recompiler::externs::call(ctx, base, 0x82414604);
		return;
	}
	// 824145FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82414600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414630 size=20
    let mut pc: u32 = 0x82414630;
    'dispatch: loop {
        match pc {
            0x82414630 => {
    //   block [0x82414630..0x82414644)
	// 82414630: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414634: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82414638: 4199000C  bgt cr6, 0x82414644
	if ctx.cr[6].gt {
		sub_82414644(ctx, base);
		return;
	}
	// 8241463C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82414640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414644(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414644 size=64
    let mut pc: u32 = 0x82414644;
    'dispatch: loop {
        match pc {
            0x82414644 => {
    //   block [0x82414644..0x82414684)
	// 82414644: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414648: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8241464C: 4099FFF0  ble cr6, 0x8241463c
	if !ctx.cr[6].gt {
		sub_82414630(ctx, base);
		return;
	}
	// 82414650: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414654: 1D64000C  mulli r11, r4, 0xc
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 12 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82414658: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241465C: 54AA1838  slwi r10, r5, 3
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82414660: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414664: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82414668: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241466C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82414670: 4099FFCC  ble cr6, 0x8241463c
	if !ctx.cr[6].gt {
		sub_82414630(ctx, base);
		return;
	}
	// 82414674: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414678: 54CA1838  slwi r10, r6, 3
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241467C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82414680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414688 size=20
    let mut pc: u32 = 0x82414688;
    'dispatch: loop {
        match pc {
            0x82414688 => {
    //   block [0x82414688..0x8241469C)
	// 82414688: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241468C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82414690: 4199000C  bgt cr6, 0x8241469c
	if ctx.cr[6].gt {
		sub_8241469C(ctx, base);
		return;
	}
	// 82414694: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82414698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241469C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241469C size=68
    let mut pc: u32 = 0x8241469C;
    'dispatch: loop {
        match pc {
            0x8241469C => {
    //   block [0x8241469C..0x824146E0)
	// 8241469C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824146A0: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 824146A4: 4099FFF0  ble cr6, 0x82414694
	if !ctx.cr[6].gt {
		sub_82414688(ctx, base);
		return;
	}
	// 824146A8: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824146AC: 1D64000C  mulli r11, r4, 0xc
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 12 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824146B0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824146B4: 54AA1838  slwi r10, r5, 3
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824146B8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824146BC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824146C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824146C4: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 824146C8: 4099FFCC  ble cr6, 0x82414694
	if !ctx.cr[6].gt {
		sub_82414688(ctx, base);
		return;
	}
	// 824146CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824146D0: 54CA1838  slwi r10, r6, 3
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824146D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824146D8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824146DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824146E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824146E0 size=112
    let mut pc: u32 = 0x824146E0;
    'dispatch: loop {
        match pc {
            0x824146E0 => {
    //   block [0x824146E0..0x82414720)
	// 824146E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824146E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824146E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824146EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824146F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824146F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824146F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824146FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82414700: 38802001  li r4, 0x2001
	ctx.r[4].s64 = 8193;
	// 82414704: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82414708: 48001E09  bl 0x82416510
	ctx.lr = 0x8241470C;
	sub_82416510(ctx, base);
	// 8241470C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414710: 41820028  beq 0x82414738
	if ctx.cr[0].eq {
	pc = 0x82414738; continue 'dispatch;
	}
	// 82414714: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414718: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241471C: 40820008  bne 0x82414724
	if !ctx.cr[0].eq {
	pc = 0x82414724; continue 'dispatch;
	}
	pc = 0x82414720; continue 'dispatch;
            }
            0x82414720 => {
    //   block [0x82414720..0x82414724)
	// 82414720: 48000000  b 0x82414720
	pc = 0x82414720; continue 'dispatch;
            }
            0x82414724 => {
    //   block [0x82414724..0x82414738)
	// 82414724: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414728: 1D7E000C  mulli r11, r30, 0xc
	ctx.r[11].s32 = ((ctx.r[30].s32 as i64 * 12 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8241472C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82414730: 39430002  addi r10, r3, 2
	ctx.r[10].s64 = ctx.r[3].s64 + 2;
	// 82414734: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x82414738; continue 'dispatch;
            }
            0x82414738 => {
    //   block [0x82414738..0x82414750)
	// 82414738: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241473C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414744: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82414748: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241474C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414750 size=380
    let mut pc: u32 = 0x82414750;
    'dispatch: loop {
        match pc {
            0x82414750 => {
    //   block [0x82414750..0x82414798)
	// 82414750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414754: 48120955  bl 0x825350a8
	ctx.lr = 0x82414758;
	sub_82535080(ctx, base);
	// 82414758: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241475C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82414760: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82414764: 3BBC0020  addi r29, r28, 0x20
	ctx.r[29].s64 = ctx.r[28].s64 + 32;
	// 82414768: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8241476C: 38802002  li r4, 0x2002
	ctx.r[4].s64 = 8194;
	// 82414770: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414774: 48001D9D  bl 0x82416510
	ctx.lr = 0x82414778;
	sub_82416510(ctx, base);
	// 82414778: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241477C: 40820020  bne 0x8241479c
	if !ctx.cr[0].eq {
	pc = 0x8241479C; continue 'dispatch;
	}
	// 82414780: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82414784: 38802003  li r4, 0x2003
	ctx.r[4].s64 = 8195;
	// 82414788: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241478C: 48001D85  bl 0x82416510
	ctx.lr = 0x82414790;
	sub_82416510(ctx, base);
	// 82414790: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414794: 4082012C  bne 0x824148c0
	if !ctx.cr[0].eq {
	pc = 0x824148C0; continue 'dispatch;
	}
	pc = 0x82414798; continue 'dispatch;
            }
            0x82414798 => {
    //   block [0x82414798..0x8241479C)
	// 82414798: 48000000  b 0x82414798
	pc = 0x82414798; continue 'dispatch;
            }
            0x8241479C => {
    //   block [0x8241479C..0x824147B8)
	// 8241479C: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 824147A0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824147A4: 7F38CB78  mr r24, r25
	ctx.r[24].u64 = ctx.r[25].u64;
	// 824147A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824147AC: 40990114  ble cr6, 0x824148c0
	if !ctx.cr[6].gt {
	pc = 0x824148C0; continue 'dispatch;
	}
	// 824147B0: 1F5F000C  mulli r26, r31, 0xc
	ctx.r[26].s32 = ((ctx.r[31].s32 as i64 * 12 as i64) as i32);
	ctx.r[26].s64 = ctx.r[26].s32 as i64;
	// 824147B4: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	pc = 0x824147B8; continue 'dispatch;
            }
            0x824147B8 => {
    //   block [0x824147B8..0x824147E8)
	// 824147B8: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 824147BC: 3D401FFF  lis r10, 0x1fff
	ctx.r[10].s64 = 536805376;
	// 824147C0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824147C4: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 824147C8: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 824147CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824147D0: 7D2BE92E  stwx r9, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[9].u32) };
	// 824147D4: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824147D8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824147DC: 57FB1838  slwi r27, r31, 3
	ctx.r[27].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 824147E0: 40990008  ble cr6, 0x824147e8
	if !ctx.cr[6].gt {
	pc = 0x824147E8; continue 'dispatch;
	}
	// 824147E4: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	pc = 0x824147E8; continue 'dispatch;
            }
            0x824147E8 => {
    //   block [0x824147E8..0x82414814)
	// 824147E8: 4BFFBFD9  bl 0x824107c0
	ctx.lr = 0x824147EC;
	sub_824107C0(ctx, base);
	// 824147EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824147F0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824147F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824147F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824147FC: 4E800421  bctrl
	ctx.lr = 0x82414800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414800: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414804: 4182002C  beq 0x82414830
	if ctx.cr[0].eq {
	pc = 0x82414830; continue 'dispatch;
	}
	// 82414808: 357FFFFF  addic. r11, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241480C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82414810: 41800018  blt 0x82414828
	if ctx.cr[0].lt {
	pc = 0x82414828; continue 'dispatch;
	}
            }
            0x82414814 => {
    //   block [0x82414814..0x82414828)
	// 82414814: 932A0000  stw r25, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82414818: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241481C: 932A0004  stw r25, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82414820: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82414824: 4080FFF0  bge 0x82414814
	if !ctx.cr[0].lt {
	pc = 0x82414814; continue 'dispatch;
	}
	pc = 0x82414828; continue 'dispatch;
            }
            0x82414828 => {
    //   block [0x82414828..0x82414830)
	// 82414828: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8241482C: 48000008  b 0x82414834
	pc = 0x82414834; continue 'dispatch;
            }
            0x82414830 => {
    //   block [0x82414830..0x82414834)
	// 82414830: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	pc = 0x82414834; continue 'dispatch;
            }
            0x82414834 => {
    //   block [0x82414834..0x8241486C)
	// 82414834: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414838: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 8241483C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414840: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82414844: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 82414848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241484C: 480008CD  bl 0x82415118
	ctx.lr = 0x82414850;
	sub_82415118(ctx, base);
	// 82414850: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82414854: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414858: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 8241485C: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82414860: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82414864: 40990044  ble cr6, 0x824148a8
	if !ctx.cr[6].gt {
	pc = 0x824148A8; continue 'dispatch;
	}
	// 82414868: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	pc = 0x8241486C; continue 'dispatch;
            }
            0x8241486C => {
    //   block [0x8241486C..0x824148A8)
	// 8241486C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414870: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414874: 41820054  beq 0x824148c8
	if ctx.cr[0].eq {
	pc = 0x824148C8; continue 'dispatch;
	}
	// 82414878: 811C001C  lwz r8, 0x1c(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241487C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82414880: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82414884: 7D1A4214  add r8, r26, r8
	ctx.r[8].u64 = ctx.r[26].u64 + ctx.r[8].u64;
	// 82414888: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241488C: 7D08EA14  add r8, r8, r29
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[29].u64;
	// 82414890: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414894: 7CE8492E  stwx r7, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[7].u32) };
	// 82414898: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8241489C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824148A0: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824148A4: 4198FFC8  blt cr6, 0x8241486c
	if ctx.cr[6].lt {
	pc = 0x8241486C; continue 'dispatch;
	}
	pc = 0x824148A8; continue 'dispatch;
            }
            0x824148A8 => {
    //   block [0x824148A8..0x824148C0)
	// 824148A8: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 824148AC: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 824148B0: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 824148B4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 824148B8: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824148BC: 4198FEFC  blt cr6, 0x824147b8
	if ctx.cr[6].lt {
	pc = 0x824147B8; continue 'dispatch;
	}
	pc = 0x824148C0; continue 'dispatch;
            }
            0x824148C0 => {
    //   block [0x824148C0..0x824148C8)
	// 824148C0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824148C4: 48120834  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x824148C8 => {
    //   block [0x824148C8..0x824148CC)
	// 824148C8: 48000000  b 0x824148c8
	pc = 0x824148C8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824148D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824148D0 size=464
    let mut pc: u32 = 0x824148D0;
    'dispatch: loop {
        match pc {
            0x824148D0 => {
    //   block [0x824148D0..0x82414918)
	// 824148D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824148D4: 481207D5  bl 0x825350a8
	ctx.lr = 0x824148D8;
	sub_82535080(ctx, base);
	// 824148D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824148DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824148E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824148E4: 3BDD0020  addi r30, r29, 0x20
	ctx.r[30].s64 = ctx.r[29].s64 + 32;
	// 824148E8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824148EC: 38802003  li r4, 0x2003
	ctx.r[4].s64 = 8195;
	// 824148F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824148F4: 48001C1D  bl 0x82416510
	ctx.lr = 0x824148F8;
	sub_82416510(ctx, base);
	// 824148F8: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824148FC: 40820020  bne 0x8241491c
	if !ctx.cr[0].eq {
	pc = 0x8241491C; continue 'dispatch;
	}
	// 82414900: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82414904: 38802002  li r4, 0x2002
	ctx.r[4].s64 = 8194;
	// 82414908: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241490C: 48001C05  bl 0x82416510
	ctx.lr = 0x82414910;
	sub_82416510(ctx, base);
	// 82414910: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414914: 4082017C  bne 0x82414a90
	if !ctx.cr[0].eq {
	pc = 0x82414A90; continue 'dispatch;
	}
	pc = 0x82414918; continue 'dispatch;
            }
            0x82414918 => {
    //   block [0x82414918..0x8241491C)
	// 82414918: 48000000  b 0x82414918
	pc = 0x82414918; continue 'dispatch;
            }
            0x8241491C => {
    //   block [0x8241491C..0x82414938)
	// 8241491C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414920: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82414924: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82414928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241492C: 40990164  ble cr6, 0x82414a90
	if !ctx.cr[6].gt {
	pc = 0x82414A90; continue 'dispatch;
	}
	// 82414930: 1F5F000C  mulli r26, r31, 0xc
	ctx.r[26].s32 = ((ctx.r[31].s32 as i64 * 12 as i64) as i32);
	ctx.r[26].s64 = ctx.r[26].s32 as i64;
	// 82414934: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	pc = 0x82414938; continue 'dispatch;
            }
            0x82414938 => {
    //   block [0x82414938..0x82414970)
	// 82414938: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241493C: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82414940: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414944: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82414948: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241494C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82414950: 409A00A4  bne cr6, 0x824149f4
	if !ctx.cr[6].eq {
	pc = 0x824149F4; continue 'dispatch;
	}
	// 82414954: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82414958: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241495C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82414960: 577F1838  slwi r31, r27, 3
	ctx.r[31].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82414964: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414968: 40990008  ble cr6, 0x82414970
	if !ctx.cr[6].gt {
	pc = 0x82414970; continue 'dispatch;
	}
	// 8241496C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	pc = 0x82414970; continue 'dispatch;
            }
            0x82414970 => {
    //   block [0x82414970..0x8241499C)
	// 82414970: 4BFFBE51  bl 0x824107c0
	ctx.lr = 0x82414974;
	sub_824107C0(ctx, base);
	// 82414974: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414978: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241497C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414984: 4E800421  bctrl
	ctx.lr = 0x82414988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414988: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241498C: 4182002C  beq 0x824149b8
	if ctx.cr[0].eq {
	pc = 0x824149B8; continue 'dispatch;
	}
	// 82414990: 357BFFFF  addic. r11, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414994: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82414998: 41800018  blt 0x824149b0
	if ctx.cr[0].lt {
	pc = 0x824149B0; continue 'dispatch;
	}
            }
            0x8241499C => {
    //   block [0x8241499C..0x824149B0)
	// 8241499C: 930A0000  stw r24, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 824149A0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824149A4: 930A0004  stw r24, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 824149A8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824149AC: 4080FFF0  bge 0x8241499c
	if !ctx.cr[0].lt {
	pc = 0x8241499C; continue 'dispatch;
	}
	pc = 0x824149B0; continue 'dispatch;
            }
            0x824149B0 => {
    //   block [0x824149B0..0x824149B8)
	// 824149B0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824149B4: 48000008  b 0x824149bc
	pc = 0x824149BC; continue 'dispatch;
            }
            0x824149B8 => {
    //   block [0x824149B8..0x824149BC)
	// 824149B8: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	pc = 0x824149BC; continue 'dispatch;
            }
            0x824149BC => {
    //   block [0x824149BC..0x824149F4)
	// 824149BC: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 824149C0: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 824149C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824149C8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824149CC: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 824149D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824149D4: 48000745  bl 0x82415118
	ctx.lr = 0x824149D8;
	sub_82415118(ctx, base);
	// 824149D8: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 824149DC: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 824149E0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824149E4: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 824149E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824149EC: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 824149F0: 48000014  b 0x82414a04
	pc = 0x82414A04; continue 'dispatch;
            }
            0x824149F4 => {
    //   block [0x824149F4..0x82414A04)
	// 824149F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824149F8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824149FC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82414A00: 409A009C  bne cr6, 0x82414a9c
	if !ctx.cr[6].eq {
	pc = 0x82414A9C; continue 'dispatch;
	}
	pc = 0x82414A04; continue 'dispatch;
            }
            0x82414A04 => {
    //   block [0x82414A04..0x82414A1C)
	// 82414A04: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414A08: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82414A0C: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82414A10: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82414A14: 40990054  ble cr6, 0x82414a68
	if !ctx.cr[6].gt {
	pc = 0x82414A68; continue 'dispatch;
	}
	// 82414A18: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	pc = 0x82414A1C; continue 'dispatch;
            }
            0x82414A1C => {
    //   block [0x82414A1C..0x82414A68)
	// 82414A1C: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414A20: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414A24: 41820074  beq 0x82414a98
	if ctx.cr[0].eq {
	pc = 0x82414A98; continue 'dispatch;
	}
	// 82414A28: 811D001C  lwz r8, 0x1c(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414A2C: 38EB0002  addi r7, r11, 2
	ctx.r[7].s64 = ctx.r[11].s64 + 2;
	// 82414A30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82414A34: 7D08D214  add r8, r8, r26
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[26].u64;
	// 82414A38: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414A3C: 7D08F214  add r8, r8, r30
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 82414A40: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414A44: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82414A48: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82414A4C: 90E80004  stw r7, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82414A50: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414A54: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414A58: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82414A5C: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82414A60: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82414A64: 4198FFB8  blt cr6, 0x82414a1c
	if ctx.cr[6].lt {
	pc = 0x82414A1C; continue 'dispatch;
	}
	pc = 0x82414A68; continue 'dispatch;
            }
            0x82414A68 => {
    //   block [0x82414A68..0x82414A7C)
	// 82414A68: 556A07BF  clrlwi. r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82414A6C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82414A70: 4182000C  beq 0x82414a7c
	if ctx.cr[0].eq {
	pc = 0x82414A7C; continue 'dispatch;
	}
	// 82414A74: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82414A78: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	pc = 0x82414A7C; continue 'dispatch;
            }
            0x82414A7C => {
    //   block [0x82414A7C..0x82414A90)
	// 82414A7C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414A80: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82414A84: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82414A88: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414A8C: 4198FEAC  blt cr6, 0x82414938
	if ctx.cr[6].lt {
	pc = 0x82414938; continue 'dispatch;
	}
	pc = 0x82414A90; continue 'dispatch;
            }
            0x82414A90 => {
    //   block [0x82414A90..0x82414A98)
	// 82414A90: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82414A94: 48120664  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x82414A98 => {
    //   block [0x82414A98..0x82414A9C)
	// 82414A98: 48000000  b 0x82414a98
	pc = 0x82414A98; continue 'dispatch;
            }
            0x82414A9C => {
    //   block [0x82414A9C..0x82414AA0)
	// 82414A9C: 48000000  b 0x82414a9c
	pc = 0x82414A9C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414AA0 size=168
    let mut pc: u32 = 0x82414AA0;
    'dispatch: loop {
        match pc {
            0x82414AA0 => {
    //   block [0x82414AA0..0x82414AD8)
	// 82414AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414AA4: 48120615  bl 0x825350b8
	ctx.lr = 0x82414AA8;
	sub_82535080(ctx, base);
	// 82414AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414AAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82414AB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414AB4: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414AB8: 4182005C  beq 0x82414b14
	if ctx.cr[0].eq {
	pc = 0x82414B14; continue 'dispatch;
	}
	// 82414ABC: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82414AC0: 3BBFFFFC  addi r29, r31, -4
	ctx.r[29].s64 = ctx.r[31].s64 + -4;
	// 82414AC4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82414AC8: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82414ACC: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82414AD0: 4180001C  blt 0x82414aec
	if ctx.cr[0].lt {
	pc = 0x82414AEC; continue 'dispatch;
	}
	// 82414AD4: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	pc = 0x82414AD8; continue 'dispatch;
            }
            0x82414AD8 => {
    //   block [0x82414AD8..0x82414AEC)
	// 82414AD8: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82414ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414AE0: 48000639  bl 0x82415118
	ctx.lr = 0x82414AE4;
	sub_82415118(ctx, base);
	// 82414AE4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82414AE8: 4080FFF0  bge 0x82414ad8
	if !ctx.cr[0].lt {
	pc = 0x82414AD8; continue 'dispatch;
	}
	pc = 0x82414AEC; continue 'dispatch;
            }
            0x82414AEC => {
    //   block [0x82414AEC..0x82414B0C)
	// 82414AEC: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414AF0: 4182001C  beq 0x82414b0c
	if ctx.cr[0].eq {
	pc = 0x82414B0C; continue 'dispatch;
	}
	// 82414AF4: 4BFFBCCD  bl 0x824107c0
	ctx.lr = 0x82414AF8;
	sub_824107C0(ctx, base);
	// 82414AF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414AFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82414B00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414B04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414B08: 4E800421  bctrl
	ctx.lr = 0x82414B0C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82414B0C => {
    //   block [0x82414B0C..0x82414B14)
	// 82414B0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414B10: 48000030  b 0x82414b40
	pc = 0x82414B40; continue 'dispatch;
            }
            0x82414B14 => {
    //   block [0x82414B14..0x82414B3C)
	// 82414B14: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82414B18: 48000601  bl 0x82415118
	ctx.lr = 0x82414B1C;
	sub_82415118(ctx, base);
	// 82414B1C: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414B20: 4182001C  beq 0x82414b3c
	if ctx.cr[0].eq {
	pc = 0x82414B3C; continue 'dispatch;
	}
	// 82414B24: 4BFFBC9D  bl 0x824107c0
	ctx.lr = 0x82414B28;
	sub_824107C0(ctx, base);
	// 82414B28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414B2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82414B30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414B38: 4E800421  bctrl
	ctx.lr = 0x82414B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82414B3C => {
    //   block [0x82414B3C..0x82414B40)
	// 82414B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82414B40; continue 'dispatch;
            }
            0x82414B40 => {
    //   block [0x82414B40..0x82414B48)
	// 82414B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414B44: 481205C4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414B48 size=84
    let mut pc: u32 = 0x82414B48;
    'dispatch: loop {
        match pc {
            0x82414B48 => {
    //   block [0x82414B48..0x82414B80)
	// 82414B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414B50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82414B54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414B58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414B5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414B60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414B64: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414B68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414B6C: 41820014  beq 0x82414b80
	if ctx.cr[0].eq {
	pc = 0x82414B80; continue 'dispatch;
	}
	// 82414B70: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414B74: 4BFFFF2D  bl 0x82414aa0
	ctx.lr = 0x82414B78;
	sub_82414AA0(ctx, base);
	// 82414B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414B7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82414B80; continue 'dispatch;
            }
            0x82414B80 => {
    //   block [0x82414B80..0x82414B9C)
	// 82414B80: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82414B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82414B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82414B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82414B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414BA0 size=232
    let mut pc: u32 = 0x82414BA0;
    'dispatch: loop {
        match pc {
            0x82414BA0 => {
    //   block [0x82414BA0..0x82414BCC)
	// 82414BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414BA4: 48120515  bl 0x825350b8
	ctx.lr = 0x82414BA8;
	sub_82535080(ctx, base);
	// 82414BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414BAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82414BB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82414BB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82414BB8: 38802000  li r4, 0x2000
	ctx.r[4].s64 = 8192;
	// 82414BBC: 387D0020  addi r3, r29, 0x20
	ctx.r[3].s64 = ctx.r[29].s64 + 32;
	// 82414BC0: 48001951  bl 0x82416510
	ctx.lr = 0x82414BC4;
	sub_82416510(ctx, base);
	// 82414BC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414BC8: 40820008  bne 0x82414bd0
	if !ctx.cr[0].eq {
	pc = 0x82414BD0; continue 'dispatch;
	}
	pc = 0x82414BCC; continue 'dispatch;
            }
            0x82414BCC => {
    //   block [0x82414BCC..0x82414BD0)
	// 82414BCC: 48000000  b 0x82414bcc
	pc = 0x82414BCC; continue 'dispatch;
            }
            0x82414BD0 => {
    //   block [0x82414BD0..0x82414BDC)
	// 82414BD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414BD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414BD8: 40820008  bne 0x82414be0
	if !ctx.cr[0].eq {
	pc = 0x82414BE0; continue 'dispatch;
	}
	pc = 0x82414BDC; continue 'dispatch;
            }
            0x82414BDC => {
    //   block [0x82414BDC..0x82414BE0)
	// 82414BDC: 48000000  b 0x82414bdc
	pc = 0x82414BDC; continue 'dispatch;
            }
            0x82414BE0 => {
    //   block [0x82414BE0..0x82414C0C)
	// 82414BE0: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414BE4: 1F9F000C  mulli r28, r31, 0xc
	ctx.r[28].s32 = ((ctx.r[31].s32 as i64 * 12 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 82414BE8: 3D201FFF  lis r9, 0x1fff
	ctx.r[9].s64 = 536805376;
	// 82414BEC: 7D6AE12E  stwx r11, r10, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u32) };
	// 82414BF0: 83FD000C  lwz r31, 0xc(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414BF4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82414BF8: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82414BFC: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82414C00: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82414C04: 40990008  ble cr6, 0x82414c0c
	if !ctx.cr[6].gt {
	pc = 0x82414C0C; continue 'dispatch;
	}
	// 82414C08: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x82414C0C; continue 'dispatch;
            }
            0x82414C0C => {
    //   block [0x82414C0C..0x82414C1C)
	// 82414C0C: 3940FFFB  li r10, -5
	ctx.r[10].s64 = -5;
	// 82414C10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82414C14: 41990008  bgt cr6, 0x82414c1c
	if ctx.cr[6].gt {
	pc = 0x82414C1C; continue 'dispatch;
	}
	// 82414C18: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	pc = 0x82414C1C; continue 'dispatch;
            }
            0x82414C1C => {
    //   block [0x82414C1C..0x82414C54)
	// 82414C1C: 4BFFBBA5  bl 0x824107c0
	ctx.lr = 0x82414C20;
	sub_824107C0(ctx, base);
	// 82414C20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414C24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414C28: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414C2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414C30: 4E800421  bctrl
	ctx.lr = 0x82414C34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414C34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414C38: 41820034  beq 0x82414c6c
	if ctx.cr[0].eq {
	pc = 0x82414C6C; continue 'dispatch;
	}
	// 82414C3C: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82414C40: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82414C44: 357FFFFF  addic. r11, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414C48: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82414C4C: 41800024  blt 0x82414c70
	if ctx.cr[0].lt {
	pc = 0x82414C70; continue 'dispatch;
	}
	// 82414C50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
            }
            0x82414C54 => {
    //   block [0x82414C54..0x82414C6C)
	// 82414C54: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82414C58: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414C5C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82414C60: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82414C64: 4080FFF0  bge 0x82414c54
	if !ctx.cr[0].lt {
	pc = 0x82414C54; continue 'dispatch;
	}
	// 82414C68: 48000008  b 0x82414c70
	pc = 0x82414C70; continue 'dispatch;
            }
            0x82414C6C => {
    //   block [0x82414C6C..0x82414C70)
	// 82414C6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	pc = 0x82414C70; continue 'dispatch;
            }
            0x82414C70 => {
    //   block [0x82414C70..0x82414C88)
	// 82414C70: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414C74: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82414C78: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82414C7C: 4BFFFECD  bl 0x82414b48
	ctx.lr = 0x82414C80;
	sub_82414B48(ctx, base);
	// 82414C80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414C84: 48120484  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414C88 size=208
    let mut pc: u32 = 0x82414C88;
    'dispatch: loop {
        match pc {
            0x82414C88 => {
    //   block [0x82414C88..0x82414CC0)
	// 82414C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414C8C: 4812042D  bl 0x825350b8
	ctx.lr = 0x82414C90;
	sub_82535080(ctx, base);
	// 82414C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414C94: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82414C98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414C9C: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414CA0: 41820070  beq 0x82414d10
	if ctx.cr[0].eq {
	pc = 0x82414D10; continue 'dispatch;
	}
	// 82414CA4: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82414CA8: 3BBFFFFC  addi r29, r31, -4
	ctx.r[29].s64 = ctx.r[31].s64 + -4;
	// 82414CAC: 1D4B000C  mulli r10, r11, 0xc
	ctx.r[10].s32 = ((ctx.r[11].s32 as i64 * 12 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82414CB0: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82414CB4: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82414CB8: 41800030  blt 0x82414ce8
	if ctx.cr[0].lt {
	pc = 0x82414CE8; continue 'dispatch;
	}
	// 82414CBC: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	pc = 0x82414CC0; continue 'dispatch;
            }
            0x82414CC0 => {
    //   block [0x82414CC0..0x82414CE0)
	// 82414CC0: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 82414CC4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414CC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414CCC: 41820014  beq 0x82414ce0
	if ctx.cr[0].eq {
	pc = 0x82414CE0; continue 'dispatch;
	}
	// 82414CD0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414CD4: 4BFFFDCD  bl 0x82414aa0
	ctx.lr = 0x82414CD8;
	sub_82414AA0(ctx, base);
	// 82414CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414CDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82414CE0; continue 'dispatch;
            }
            0x82414CE0 => {
    //   block [0x82414CE0..0x82414CE8)
	// 82414CE0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82414CE4: 4080FFDC  bge 0x82414cc0
	if !ctx.cr[0].lt {
	pc = 0x82414CC0; continue 'dispatch;
	}
	pc = 0x82414CE8; continue 'dispatch;
            }
            0x82414CE8 => {
    //   block [0x82414CE8..0x82414D08)
	// 82414CE8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414CEC: 4182001C  beq 0x82414d08
	if ctx.cr[0].eq {
	pc = 0x82414D08; continue 'dispatch;
	}
	// 82414CF0: 4BFFBAD1  bl 0x824107c0
	ctx.lr = 0x82414CF4;
	sub_824107C0(ctx, base);
	// 82414CF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414CF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82414CFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414D00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414D04: 4E800421  bctrl
	ctx.lr = 0x82414D08;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82414D08 => {
    //   block [0x82414D08..0x82414D10)
	// 82414D08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414D0C: 48000044  b 0x82414d50
	pc = 0x82414D50; continue 'dispatch;
            }
            0x82414D10 => {
    //   block [0x82414D10..0x82414D2C)
	// 82414D10: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414D14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414D18: 41820014  beq 0x82414d2c
	if ctx.cr[0].eq {
	pc = 0x82414D2C; continue 'dispatch;
	}
	// 82414D1C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414D20: 4BFFFD81  bl 0x82414aa0
	ctx.lr = 0x82414D24;
	sub_82414AA0(ctx, base);
	// 82414D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414D28: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82414D2C; continue 'dispatch;
            }
            0x82414D2C => {
    //   block [0x82414D2C..0x82414D4C)
	// 82414D2C: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414D30: 4182001C  beq 0x82414d4c
	if ctx.cr[0].eq {
	pc = 0x82414D4C; continue 'dispatch;
	}
	// 82414D34: 4BFFBA8D  bl 0x824107c0
	ctx.lr = 0x82414D38;
	sub_824107C0(ctx, base);
	// 82414D38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414D3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82414D40: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414D48: 4E800421  bctrl
	ctx.lr = 0x82414D4C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82414D4C => {
    //   block [0x82414D4C..0x82414D50)
	// 82414D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82414D50; continue 'dispatch;
            }
            0x82414D50 => {
    //   block [0x82414D50..0x82414D58)
	// 82414D50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414D54: 481203B4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414D58 size=84
    let mut pc: u32 = 0x82414D58;
    'dispatch: loop {
        match pc {
            0x82414D58 => {
    //   block [0x82414D58..0x82414D90)
	// 82414D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82414D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414D70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414D74: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414D78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414D7C: 41820014  beq 0x82414d90
	if ctx.cr[0].eq {
	pc = 0x82414D90; continue 'dispatch;
	}
	// 82414D80: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414D84: 4BFFFF05  bl 0x82414c88
	ctx.lr = 0x82414D88;
	sub_82414C88(ctx, base);
	// 82414D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414D8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82414D90; continue 'dispatch;
            }
            0x82414D90 => {
    //   block [0x82414D90..0x82414DAC)
	// 82414D90: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82414D94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82414D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414DA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82414DA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82414DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414DB0 size=76
    let mut pc: u32 = 0x82414DB0;
    'dispatch: loop {
        match pc {
            0x82414DB0 => {
    //   block [0x82414DB0..0x82414DE8)
	// 82414DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414DB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414DBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414DC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414DC4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82414DC8: 48001941  bl 0x82416708
	ctx.lr = 0x82414DCC;
	sub_82416708(ctx, base);
	// 82414DCC: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414DD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414DD4: 41820014  beq 0x82414de8
	if ctx.cr[0].eq {
	pc = 0x82414DE8; continue 'dispatch;
	}
	// 82414DD8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414DDC: 4BFFFEAD  bl 0x82414c88
	ctx.lr = 0x82414DE0;
	sub_82414C88(ctx, base);
	// 82414DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414DE4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	pc = 0x82414DE8; continue 'dispatch;
            }
            0x82414DE8 => {
    //   block [0x82414DE8..0x82414DFC)
	// 82414DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82414DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414DF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82414DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414E00 size=336
    let mut pc: u32 = 0x82414E00;
    'dispatch: loop {
        match pc {
            0x82414E00 => {
    //   block [0x82414E00..0x82414E28)
	// 82414E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414E04: 481202B9  bl 0x825350bc
	ctx.lr = 0x82414E08;
	sub_82535080(ctx, base);
	// 82414E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414E0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82414E10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82414E14: 38801000  li r4, 0x1000
	ctx.r[4].s64 = 4096;
	// 82414E18: 387D0020  addi r3, r29, 0x20
	ctx.r[3].s64 = ctx.r[29].s64 + 32;
	// 82414E1C: 480016F5  bl 0x82416510
	ctx.lr = 0x82414E20;
	sub_82416510(ctx, base);
	// 82414E20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414E24: 40820008  bne 0x82414e2c
	if !ctx.cr[0].eq {
	pc = 0x82414E2C; continue 'dispatch;
	}
	pc = 0x82414E28; continue 'dispatch;
            }
            0x82414E28 => {
    //   block [0x82414E28..0x82414E2C)
	// 82414E28: 48000000  b 0x82414e28
	pc = 0x82414E28; continue 'dispatch;
            }
            0x82414E2C => {
    //   block [0x82414E2C..0x82414E38)
	// 82414E2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414E30: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414E34: 40820008  bne 0x82414e3c
	if !ctx.cr[0].eq {
	pc = 0x82414E3C; continue 'dispatch;
	}
	pc = 0x82414E38; continue 'dispatch;
            }
            0x82414E38 => {
    //   block [0x82414E38..0x82414E3C)
	// 82414E38: 48000000  b 0x82414e38
	pc = 0x82414E38; continue 'dispatch;
            }
            0x82414E3C => {
    //   block [0x82414E3C..0x82414E4C)
	// 82414E3C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82414E40: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414E44: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414E48: 40820008  bne 0x82414e50
	if !ctx.cr[0].eq {
	pc = 0x82414E50; continue 'dispatch;
	}
	pc = 0x82414E4C; continue 'dispatch;
            }
            0x82414E4C => {
    //   block [0x82414E4C..0x82414E50)
	// 82414E4C: 48000000  b 0x82414e4c
	pc = 0x82414E4C; continue 'dispatch;
            }
            0x82414E50 => {
    //   block [0x82414E50..0x82414E60)
	// 82414E50: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82414E54: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414E58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414E5C: 40820008  bne 0x82414e64
	if !ctx.cr[0].eq {
	pc = 0x82414E64; continue 'dispatch;
	}
	pc = 0x82414E60; continue 'dispatch;
            }
            0x82414E60 => {
    //   block [0x82414E60..0x82414E64)
	// 82414E60: 48000000  b 0x82414e60
	pc = 0x82414E60; continue 'dispatch;
            }
            0x82414E64 => {
    //   block [0x82414E64..0x82414E84)
	// 82414E64: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82414E68: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414E6C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82414E70: 419A003C  beq cr6, 0x82414eac
	if ctx.cr[6].eq {
	pc = 0x82414EAC; continue 'dispatch;
	}
	// 82414E74: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82414E78: 419A002C  beq cr6, 0x82414ea4
	if ctx.cr[6].eq {
	pc = 0x82414EA4; continue 'dispatch;
	}
	// 82414E7C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82414E80: 419A0008  beq cr6, 0x82414e88
	if ctx.cr[6].eq {
	pc = 0x82414E88; continue 'dispatch;
	}
	pc = 0x82414E84; continue 'dispatch;
            }
            0x82414E84 => {
    //   block [0x82414E84..0x82414E88)
	// 82414E84: 48000000  b 0x82414e84
	pc = 0x82414E84; continue 'dispatch;
            }
            0x82414E88 => {
    //   block [0x82414E88..0x82414EA4)
	// 82414E88: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82414E8C: 39430016  addi r10, r3, 0x16
	ctx.r[10].s64 = ctx.r[3].s64 + 22;
	// 82414E90: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82414E94: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82414E98: 915D0018  stw r10, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82414E9C: 917D0014  stw r11, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82414EA0: 48000014  b 0x82414eb4
	pc = 0x82414EB4; continue 'dispatch;
            }
            0x82414EA4 => {
    //   block [0x82414EA4..0x82414EAC)
	// 82414EA4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82414EA8: 48000008  b 0x82414eb0
	pc = 0x82414EB0; continue 'dispatch;
            }
            0x82414EAC => {
    //   block [0x82414EAC..0x82414EB0)
	// 82414EAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82414EB0; continue 'dispatch;
            }
            0x82414EB0 => {
    //   block [0x82414EB0..0x82414EB4)
	// 82414EB0: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	pc = 0x82414EB4; continue 'dispatch;
            }
            0x82414EB4 => {
    //   block [0x82414EB4..0x82414ED4)
	// 82414EB4: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 82414EB8: 83FD0008  lwz r31, 8(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414EBC: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82414EC0: 616B5555  ori r11, r11, 0x5555
	ctx.r[11].u64 = ctx.r[11].u64 | 21845;
	// 82414EC4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414EC8: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s32 = ((ctx.r[31].s32 as i64 * 12 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82414ECC: 40990008  ble cr6, 0x82414ed4
	if !ctx.cr[6].gt {
	pc = 0x82414ED4; continue 'dispatch;
	}
	// 82414ED0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x82414ED4; continue 'dispatch;
            }
            0x82414ED4 => {
    //   block [0x82414ED4..0x82414EE4)
	// 82414ED4: 3940FFFB  li r10, -5
	ctx.r[10].s64 = -5;
	// 82414ED8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82414EDC: 41990008  bgt cr6, 0x82414ee4
	if ctx.cr[6].gt {
	pc = 0x82414EE4; continue 'dispatch;
	}
	// 82414EE0: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	pc = 0x82414EE4; continue 'dispatch;
            }
            0x82414EE4 => {
    //   block [0x82414EE4..0x82414F1C)
	// 82414EE4: 4BFFB8DD  bl 0x824107c0
	ctx.lr = 0x82414EE8;
	sub_824107C0(ctx, base);
	// 82414EE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414EEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414EF0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414EF8: 4E800421  bctrl
	ctx.lr = 0x82414EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414EFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414F00: 41820038  beq 0x82414f38
	if ctx.cr[0].eq {
	pc = 0x82414F38; continue 'dispatch;
	}
	// 82414F04: 39030004  addi r8, r3, 4
	ctx.r[8].s64 = ctx.r[3].s64 + 4;
	// 82414F08: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82414F0C: 355FFFFF  addic. r10, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82414F10: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82414F14: 41800028  blt 0x82414f3c
	if ctx.cr[0].lt {
	pc = 0x82414F3C; continue 'dispatch;
	}
	// 82414F18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
            }
            0x82414F1C => {
    //   block [0x82414F1C..0x82414F38)
	// 82414F1C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82414F20: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82414F24: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82414F28: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82414F2C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82414F30: 4080FFEC  bge 0x82414f1c
	if !ctx.cr[0].lt {
	pc = 0x82414F1C; continue 'dispatch;
	}
	// 82414F34: 48000008  b 0x82414f3c
	pc = 0x82414F3C; continue 'dispatch;
            }
            0x82414F38 => {
    //   block [0x82414F38..0x82414F3C)
	// 82414F38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	pc = 0x82414F3C; continue 'dispatch;
            }
            0x82414F3C => {
    //   block [0x82414F3C..0x82414F50)
	// 82414F3C: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82414F40: 387D001C  addi r3, r29, 0x1c
	ctx.r[3].s64 = ctx.r[29].s64 + 28;
	// 82414F44: 4BFFFE15  bl 0x82414d58
	ctx.lr = 0x82414F48;
	sub_82414D58(ctx, base);
	// 82414F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82414F4C: 481201C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414F50 size=196
    let mut pc: u32 = 0x82414F50;
    'dispatch: loop {
        match pc {
            0x82414F50 => {
    //   block [0x82414F50..0x82414FA8)
	// 82414F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414F54: 48120169  bl 0x825350bc
	ctx.lr = 0x82414F58;
	sub_82535080(ctx, base);
	// 82414F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414F60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82414F64: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82414F68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414F6C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82414F70: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82414F74: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82414F78: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82414F7C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82414F80: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82414F84: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82414F88: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82414F8C: 48001605  bl 0x82416590
	ctx.lr = 0x82414F90;
	sub_82416590(ctx, base);
	// 82414F90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82414F94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414F98: 388BEC48  addi r4, r11, -0x13b8
	ctx.r[4].s64 = ctx.r[11].s64 + -5048;
	// 82414F9C: 48001435  bl 0x824163d0
	ctx.lr = 0x82414FA0;
	sub_824163D0(ctx, base);
	// 82414FA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414FA4: 40820008  bne 0x82414fac
	if !ctx.cr[0].eq {
	pc = 0x82414FAC; continue 'dispatch;
	}
	pc = 0x82414FA8; continue 'dispatch;
            }
            0x82414FA8 => {
    //   block [0x82414FA8..0x82414FAC)
	// 82414FA8: 48000000  b 0x82414fa8
	pc = 0x82414FA8; continue 'dispatch;
            }
            0x82414FAC => {
    //   block [0x82414FAC..0x82414FC8)
	// 82414FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FB0: 4BFFFE51  bl 0x82414e00
	ctx.lr = 0x82414FB4;
	sub_82414E00(ctx, base);
	// 82414FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FB8: 4BFFF591  bl 0x82414548
	ctx.lr = 0x82414FBC;
	sub_82414548(ctx, base);
	// 82414FBC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414FC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82414FC4: 40990044  ble cr6, 0x82415008
	if !ctx.cr[6].gt {
	pc = 0x82415008; continue 'dispatch;
	}
	pc = 0x82414FC8; continue 'dispatch;
            }
            0x82414FC8 => {
    //   block [0x82414FC8..0x82415008)
	// 82414FC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FD0: 4BFFFBD1  bl 0x82414ba0
	ctx.lr = 0x82414FD4;
	sub_82414BA0(ctx, base);
	// 82414FD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FDC: 4BFFF705  bl 0x824146e0
	ctx.lr = 0x82414FE0;
	sub_824146E0(ctx, base);
	// 82414FE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414FE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FE8: 4BFFF769  bl 0x82414750
	ctx.lr = 0x82414FEC;
	sub_82414750(ctx, base);
	// 82414FEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FF4: 4BFFF8DD  bl 0x824148d0
	ctx.lr = 0x82414FF8;
	sub_824148D0(ctx, base);
	// 82414FF8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414FFC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82415000: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415004: 4198FFC4  blt cr6, 0x82414fc8
	if ctx.cr[6].lt {
	pc = 0x82414FC8; continue 'dispatch;
	}
	pc = 0x82415008; continue 'dispatch;
            }
            0x82415008 => {
    //   block [0x82415008..0x82415014)
	// 82415008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241500C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415010: 481200FC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415018 size=40
    let mut pc: u32 = 0x82415018;
    'dispatch: loop {
        match pc {
            0x82415018 => {
    //   block [0x82415018..0x82415040)
	// 82415018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241501C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82415020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415024: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 82415028: 480013F9  bl 0x82416420
	ctx.lr = 0x8241502C;
	sub_82416420(ctx, base);
	// 8241502C: 5463063E  clrlwi r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82415030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82415034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82415038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241503C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415040 size=8
    let mut pc: u32 = 0x82415040;
    'dispatch: loop {
        match pc {
            0x82415040 => {
    //   block [0x82415040..0x82415048)
	// 82415040: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 82415044: 48001414  b 0x82416458
	sub_82416458(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415048 size=8
    let mut pc: u32 = 0x82415048;
    'dispatch: loop {
        match pc {
            0x82415048 => {
    //   block [0x82415048..0x82415050)
	// 82415048: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 8241504C: 4800142C  b 0x82416478
	sub_82416478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415050 size=8
    let mut pc: u32 = 0x82415050;
    'dispatch: loop {
        match pc {
            0x82415050 => {
    //   block [0x82415050..0x82415058)
	// 82415050: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415058 size=8
    let mut pc: u32 = 0x82415058;
    'dispatch: loop {
        match pc {
            0x82415058 => {
    //   block [0x82415058..0x82415060)
	// 82415058: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8241505C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415060 size=20
    let mut pc: u32 = 0x82415060;
    'dispatch: loop {
        match pc {
            0x82415060 => {
    //   block [0x82415060..0x82415074)
	// 82415060: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82415064: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415068: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 8241506C: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82415070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415078 size=20
    let mut pc: u32 = 0x82415078;
    'dispatch: loop {
        match pc {
            0x82415078 => {
    //   block [0x82415078..0x8241508C)
	// 82415078: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241507C: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82415080: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415084: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82415088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415090 size=8
    let mut pc: u32 = 0x82415090;
    'dispatch: loop {
        match pc {
            0x82415090 => {
    //   block [0x82415090..0x82415098)
	// 82415090: 80630084  lwz r3, 0x84(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82415094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415098 size=76
    let mut pc: u32 = 0x82415098;
    'dispatch: loop {
        match pc {
            0x82415098 => {
    //   block [0x82415098..0x824150C4)
	// 82415098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241509C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824150A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824150A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824150A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824150AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824150B0: 38803000  li r4, 0x3000
	ctx.r[4].s64 = 12288;
	// 824150B4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 824150B8: 48001459  bl 0x82416510
	ctx.lr = 0x824150BC;
	sub_82416510(ctx, base);
	// 824150BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824150C0: 40820008  bne 0x824150c8
	if !ctx.cr[0].eq {
	pc = 0x824150C8; continue 'dispatch;
	}
	pc = 0x824150C4; continue 'dispatch;
            }
            0x824150C4 => {
    //   block [0x824150C4..0x824150C8)
	// 824150C4: 48000000  b 0x824150c4
	pc = 0x824150C4; continue 'dispatch;
            }
            0x824150C8 => {
    //   block [0x824150C8..0x824150E4)
	// 824150C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824150CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824150D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824150D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824150D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824150DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824150E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824150E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824150E8 size=20
    let mut pc: u32 = 0x824150E8;
    'dispatch: loop {
        match pc {
            0x824150E8 => {
    //   block [0x824150E8..0x824150FC)
	// 824150E8: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824150EC: 2B0B8004  cmplwi cr6, r11, 0x8004
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32772 as u32, &mut ctx.xer);
	// 824150F0: 409A000C  bne cr6, 0x824150fc
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x824150FC);
		return;
	}
	// 824150F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824150F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415118 size=92
    let mut pc: u32 = 0x82415118;
    'dispatch: loop {
        match pc {
            0x82415118 => {
    //   block [0x82415118..0x8241515C)
	// 82415118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241511C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82415120: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82415124: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82415128: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241512C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415130: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415134: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415138: 41820024  beq 0x8241515c
	if ctx.cr[0].eq {
	pc = 0x8241515C; continue 'dispatch;
	}
	// 8241513C: 4BFFB685  bl 0x824107c0
	ctx.lr = 0x82415140;
	sub_824107C0(ctx, base);
	// 82415140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415144: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82415148: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241514C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82415150: 4E800421  bctrl
	ctx.lr = 0x82415154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82415154: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415158: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
            }
            0x8241515C => {
    //   block [0x8241515C..0x82415174)
	// 8241515C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82415164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82415168: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241516C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82415170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415178 size=64
    let mut pc: u32 = 0x82415178;
    'dispatch: loop {
        match pc {
            0x82415178 => {
    //   block [0x82415178..0x82415194)
	// 82415178: 80E3000C  lwz r7, 0xc(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241517C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415180: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415184: 4182002C  beq 0x824151b0
	if ctx.cr[0].eq {
	pc = 0x824151B0; continue 'dispatch;
	}
	// 82415188: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241518C: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82415190: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x82415194; continue 'dispatch;
            }
            0x82415194 => {
    //   block [0x82415194..0x824151B0)
	// 82415194: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415198: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8241519C: 419A001C  beq cr6, 0x824151b8
	if ctx.cr[6].eq {
		sub_824151B8(ctx, base);
		return;
	}
	// 824151A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824151A4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824151A8: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824151AC: 4198FFE8  blt cr6, 0x82415194
	if ctx.cr[6].lt {
	pc = 0x82415194; continue 'dispatch;
	}
	pc = 0x824151B0; continue 'dispatch;
            }
            0x824151B0 => {
    //   block [0x824151B0..0x824151B8)
	// 824151B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824151B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824151B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824151B8 size=16
    let mut pc: u32 = 0x824151B8;
    'dispatch: loop {
        match pc {
            0x824151B8 => {
    //   block [0x824151B8..0x824151C8)
	// 824151B8: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824151BC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824151C0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824151C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824151C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824151C8 size=64
    let mut pc: u32 = 0x824151C8;
    'dispatch: loop {
        match pc {
            0x824151C8 => {
    //   block [0x824151C8..0x824151E4)
	// 824151C8: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824151CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824151D0: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824151D4: 4182002C  beq 0x82415200
	if ctx.cr[0].eq {
	pc = 0x82415200; continue 'dispatch;
	}
	// 824151D8: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 824151DC: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 824151E0: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x824151E4; continue 'dispatch;
            }
            0x824151E4 => {
    //   block [0x824151E4..0x82415200)
	// 824151E4: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824151E8: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824151EC: 419A001C  beq cr6, 0x82415208
	if ctx.cr[6].eq {
		sub_82415208(ctx, base);
		return;
	}
	// 824151F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824151F4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824151F8: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824151FC: 4198FFE8  blt cr6, 0x824151e4
	if ctx.cr[6].lt {
	pc = 0x824151E4; continue 'dispatch;
	}
	pc = 0x82415200; continue 'dispatch;
            }
            0x82415200 => {
    //   block [0x82415200..0x82415208)
	// 82415200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415208 size=16
    let mut pc: u32 = 0x82415208;
    'dispatch: loop {
        match pc {
            0x82415208 => {
    //   block [0x82415208..0x82415218)
	// 82415208: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241520C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82415210: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415218 size=84
    let mut pc: u32 = 0x82415218;
    'dispatch: loop {
        match pc {
            0x82415218 => {
    //   block [0x82415218..0x82415254)
	// 82415218: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241521C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415220: 419A0034  beq cr6, 0x82415254
	if ctx.cr[6].eq {
	pc = 0x82415254; continue 'dispatch;
	}
	// 82415224: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415228: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241522C: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82415230: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82415234: A16B0002  lhz r11, 2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82415238: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8241523C: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82415240: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415244: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415248: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241524C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415250: 48000014  b 0x82415264
	pc = 0x82415264; continue 'dispatch;
            }
            0x82415254 => {
    //   block [0x82415254..0x82415264)
	// 82415254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415258: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8241525C: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82415260: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82415264; continue 'dispatch;
            }
            0x82415264 => {
    //   block [0x82415264..0x8241526C)
	// 82415264: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415270 size=24
    let mut pc: u32 = 0x82415270;
    'dispatch: loop {
        match pc {
            0x82415270 => {
    //   block [0x82415270..0x82415288)
	// 82415270: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82415274: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82415278: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241527C: 4082000C  bne 0x82415288
	if !ctx.cr[0].eq {
		crate::recompiler::externs::call(ctx, base, 0x82415288);
		return;
	}
	// 82415280: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824152C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824152C0 size=144
    let mut pc: u32 = 0x824152C0;
    'dispatch: loop {
        match pc {
            0x824152C0 => {
    //   block [0x824152C0..0x824152DC)
	// 824152C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824152C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824152C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824152CC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 824152D0: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 824152D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824152D8: 4082000C  bne 0x824152e4
	if !ctx.cr[0].eq {
	pc = 0x824152E4; continue 'dispatch;
	}
	pc = 0x824152DC; continue 'dispatch;
            }
            0x824152DC => {
    //   block [0x824152DC..0x824152E4)
	// 824152DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824152E0: 48000060  b 0x82415340
	pc = 0x82415340; continue 'dispatch;
            }
            0x824152E4 => {
    //   block [0x824152E4..0x82415314)
	// 824152E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824152E8: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824152EC: 4198FFF0  blt cr6, 0x824152dc
	if ctx.cr[6].lt {
	pc = 0x824152DC; continue 'dispatch;
	}
	// 824152F0: 81680064  lwz r11, 0x64(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(100 as u32) ) } as u64;
	// 824152F4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824152F8: 419A0044  beq cr6, 0x8241533c
	if ctx.cr[6].eq {
	pc = 0x8241533C; continue 'dispatch;
	}
	// 824152FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415300: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415304: 409A0010  bne cr6, 0x82415314
	if !ctx.cr[6].eq {
	pc = 0x82415314; continue 'dispatch;
	}
	// 82415308: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8241530C: 4BFFFF65  bl 0x82415270
	ctx.lr = 0x82415310;
	sub_82415270(ctx, base);
	// 82415310: 48000030  b 0x82415340
	pc = 0x82415340; continue 'dispatch;
            }
            0x82415314 => {
    //   block [0x82415314..0x8241532C)
	// 82415314: 81680058  lwz r11, 0x58(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(88 as u32) ) } as u64;
	// 82415318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241531C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82415320: 91480064  stw r10, 0x64(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82415324: 91680060  stw r11, 0x60(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82415328: 419A0014  beq cr6, 0x8241533c
	if ctx.cr[6].eq {
	pc = 0x8241533C; continue 'dispatch;
	}
	pc = 0x8241532C; continue 'dispatch;
            }
            0x8241532C => {
    //   block [0x8241532C..0x8241533C)
	// 8241532C: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82415330: 4BFFFF41  bl 0x82415270
	ctx.lr = 0x82415334;
	sub_82415270(ctx, base);
	// 82415334: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82415338: 4082FFF4  bne 0x8241532c
	if !ctx.cr[0].eq {
	pc = 0x8241532C; continue 'dispatch;
	}
	pc = 0x8241533C; continue 'dispatch;
            }
            0x8241533C => {
    //   block [0x8241533C..0x82415340)
	// 8241533C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82415340; continue 'dispatch;
            }
            0x82415340 => {
    //   block [0x82415340..0x82415350)
	// 82415340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82415344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82415348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415350 size=64
    let mut pc: u32 = 0x82415350;
    'dispatch: loop {
        match pc {
            0x82415350 => {
    //   block [0x82415350..0x8241536C)
	// 82415350: 80E30020  lwz r7, 0x20(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415358: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241535C: 4182002C  beq 0x82415388
	if ctx.cr[0].eq {
	pc = 0x82415388; continue 'dispatch;
	}
	// 82415360: 81230024  lwz r9, 0x24(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82415364: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82415368: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x8241536C; continue 'dispatch;
            }
            0x8241536C => {
    //   block [0x8241536C..0x82415388)
	// 8241536C: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415370: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82415374: 419A001C  beq cr6, 0x82415390
	if ctx.cr[6].eq {
		sub_82415390(ctx, base);
		return;
	}
	// 82415378: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241537C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415380: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82415384: 4198FFE8  blt cr6, 0x8241536c
	if ctx.cr[6].lt {
	pc = 0x8241536C; continue 'dispatch;
	}
	pc = 0x82415388; continue 'dispatch;
            }
            0x82415388 => {
    //   block [0x82415388..0x82415390)
	// 82415388: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241538C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415390 size=16
    let mut pc: u32 = 0x82415390;
    'dispatch: loop {
        match pc {
            0x82415390 => {
    //   block [0x82415390..0x824153A0)
	// 82415390: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415394: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82415398: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241539C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824153A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824153A0 size=68
    let mut pc: u32 = 0x824153A0;
    'dispatch: loop {
        match pc {
            0x824153A0 => {
    //   block [0x824153A0..0x824153D0)
	// 824153A0: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 824153A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824153A8: 419A0028  beq cr6, 0x824153d0
	if ctx.cr[6].eq {
	pc = 0x824153D0; continue 'dispatch;
	}
	// 824153AC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824153B0: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824153B4: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824153B8: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 824153BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824153C0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824153C4: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 824153C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824153CC: 48000010  b 0x824153dc
	pc = 0x824153DC; continue 'dispatch;
            }
            0x824153D0 => {
    //   block [0x824153D0..0x824153DC)
	// 824153D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824153D4: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824153D8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x824153DC; continue 'dispatch;
            }
            0x824153DC => {
    //   block [0x824153DC..0x824153E4)
	// 824153DC: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824153E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824153E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824153E8 size=24
    let mut pc: u32 = 0x824153E8;
    'dispatch: loop {
        match pc {
            0x824153E8 => {
    //   block [0x824153E8..0x82415400)
	// 824153E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824153EC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824153F0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824153F4: 4082000C  bne 0x82415400
	if !ctx.cr[0].eq {
		crate::recompiler::externs::call(ctx, base, 0x82415400);
		return;
	}
	// 824153F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824153FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415438 size=144
    let mut pc: u32 = 0x82415438;
    'dispatch: loop {
        match pc {
            0x82415438 => {
    //   block [0x82415438..0x82415454)
	// 82415438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241543C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82415440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415444: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82415448: 8168001C  lwz r11, 0x1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241544C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415450: 4082000C  bne 0x8241545c
	if !ctx.cr[0].eq {
	pc = 0x8241545C; continue 'dispatch;
	}
	pc = 0x82415454; continue 'dispatch;
            }
            0x82415454 => {
    //   block [0x82415454..0x8241545C)
	// 82415454: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415458: 48000060  b 0x824154b8
	pc = 0x824154B8; continue 'dispatch;
            }
            0x8241545C => {
    //   block [0x8241545C..0x8241548C)
	// 8241545C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82415460: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82415464: 4198FFF0  blt cr6, 0x82415454
	if ctx.cr[6].lt {
	pc = 0x82415454; continue 'dispatch;
	}
	// 82415468: 81680074  lwz r11, 0x74(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(116 as u32) ) } as u64;
	// 8241546C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415470: 419A0044  beq cr6, 0x824154b4
	if ctx.cr[6].eq {
	pc = 0x824154B4; continue 'dispatch;
	}
	// 82415474: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415478: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241547C: 409A0010  bne cr6, 0x8241548c
	if !ctx.cr[6].eq {
	pc = 0x8241548C; continue 'dispatch;
	}
	// 82415480: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82415484: 4BFFFF65  bl 0x824153e8
	ctx.lr = 0x82415488;
	sub_824153E8(ctx, base);
	// 82415488: 48000030  b 0x824154b8
	pc = 0x824154B8; continue 'dispatch;
            }
            0x8241548C => {
    //   block [0x8241548C..0x824154A4)
	// 8241548C: 81680068  lwz r11, 0x68(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(104 as u32) ) } as u64;
	// 82415490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415494: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82415498: 91480074  stw r10, 0x74(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8241549C: 91680070  stw r11, 0x70(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 824154A0: 419A0014  beq cr6, 0x824154b4
	if ctx.cr[6].eq {
	pc = 0x824154B4; continue 'dispatch;
	}
	pc = 0x824154A4; continue 'dispatch;
            }
            0x824154A4 => {
    //   block [0x824154A4..0x824154B4)
	// 824154A4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824154A8: 4BFFFF41  bl 0x824153e8
	ctx.lr = 0x824154AC;
	sub_824153E8(ctx, base);
	// 824154AC: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824154B0: 4082FFF4  bne 0x824154a4
	if !ctx.cr[0].eq {
	pc = 0x824154A4; continue 'dispatch;
	}
	pc = 0x824154B4; continue 'dispatch;
            }
            0x824154B4 => {
    //   block [0x824154B4..0x824154B8)
	// 824154B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x824154B8; continue 'dispatch;
            }
            0x824154B8 => {
    //   block [0x824154B8..0x824154C8)
	// 824154B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824154BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824154C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824154C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824154C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824154C8 size=80
    let mut pc: u32 = 0x824154C8;
    'dispatch: loop {
        match pc {
            0x824154C8 => {
    //   block [0x824154C8..0x82415518)
	// 824154C8: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 824154CC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824154D0: 2B0A8004  cmplwi cr6, r10, 0x8004
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32772 as u32, &mut ctx.xer);
	// 824154D4: 419A0044  beq cr6, 0x82415518
	if ctx.cr[6].eq {
		sub_82415518(ctx, base);
		return;
	}
	// 824154D8: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824154DC: 81230084  lwz r9, 0x84(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824154E0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824154E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824154E8: 40990030  ble cr6, 0x82415518
	if !ctx.cr[6].gt {
		sub_82415518(ctx, base);
		return;
	}
	// 824154EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824154F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824154F4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824154F8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824154FC: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 82415500: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415504: B1440000  sth r10, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82415508: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8241550C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415510: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415518 size=8
    let mut pc: u32 = 0x82415518;
    'dispatch: loop {
        match pc {
            0x82415518 => {
    //   block [0x82415518..0x82415520)
	// 82415518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241551C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415520 size=64
    let mut pc: u32 = 0x82415520;
    'dispatch: loop {
        match pc {
            0x82415520 => {
    //   block [0x82415520..0x8241553C)
	// 82415520: 80E3002C  lwz r7, 0x2c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82415524: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415528: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241552C: 4182002C  beq 0x82415558
	if ctx.cr[0].eq {
	pc = 0x82415558; continue 'dispatch;
	}
	// 82415530: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82415534: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82415538: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x8241553C; continue 'dispatch;
            }
            0x8241553C => {
    //   block [0x8241553C..0x82415558)
	// 8241553C: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415540: 7F083040  cmplw cr6, r8, r6
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82415544: 419A001C  beq cr6, 0x82415560
	if ctx.cr[6].eq {
		sub_82415560(ctx, base);
		return;
	}
	// 82415548: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241554C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415550: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82415554: 4198FFE8  blt cr6, 0x8241553c
	if ctx.cr[6].lt {
	pc = 0x8241553C; continue 'dispatch;
	}
	pc = 0x82415558; continue 'dispatch;
            }
            0x82415558 => {
    //   block [0x82415558..0x82415560)
	// 82415558: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241555C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415560 size=16
    let mut pc: u32 = 0x82415560;
    'dispatch: loop {
        match pc {
            0x82415560 => {
    //   block [0x82415560..0x82415570)
	// 82415560: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415564: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82415568: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241556C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415570 size=276
    let mut pc: u32 = 0x82415570;
    'dispatch: loop {
        match pc {
            0x82415570 => {
    //   block [0x82415570..0x824155C4)
	// 82415570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415574: 4811FB49  bl 0x825350bc
	ctx.lr = 0x82415578;
	sub_82535080(ctx, base);
	// 82415578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241557C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415580: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82415584: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 82415588: 38803020  li r4, 0x3020
	ctx.r[4].s64 = 12320;
	// 8241558C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82415590: 48000F81  bl 0x82416510
	ctx.lr = 0x82415594;
	sub_82416510(ctx, base);
	// 82415594: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82415598: 418200D4  beq 0x8241566c
	if ctx.cr[0].eq {
	pc = 0x8241566C; continue 'dispatch;
	}
	// 8241559C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824155A0: 38803020  li r4, 0x3020
	ctx.r[4].s64 = 12320;
	// 824155A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824155A8: 48000EF1  bl 0x82416498
	ctx.lr = 0x824155AC;
	sub_82416498(ctx, base);
	// 824155AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824155B0: 418200BC  beq 0x8241566c
	if ctx.cr[0].eq {
	pc = 0x8241566C; continue 'dispatch;
	}
	// 824155B4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824155B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824155BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824155C0: 4099004C  ble cr6, 0x8241560c
	if !ctx.cr[6].gt {
	pc = 0x8241560C; continue 'dispatch;
	}
	pc = 0x824155C4; continue 'dispatch;
            }
            0x824155C4 => {
    //   block [0x824155C4..0x8241560C)
	// 824155C4: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824155C8: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824155CC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824155D0: 4182003C  beq 0x8241560c
	if ctx.cr[0].eq {
	pc = 0x8241560C; continue 'dispatch;
	}
	// 824155D4: A12A0002  lhz r9, 2(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 824155D8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824155DC: 41820030  beq 0x8241560c
	if ctx.cr[0].eq {
	pc = 0x8241560C; continue 'dispatch;
	}
	// 824155E0: 554907BF  clrlwi. r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824155E4: 40820090  bne 0x82415674
	if !ctx.cr[0].eq {
	pc = 0x82415674; continue 'dispatch;
	}
	// 824155E8: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824155EC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824155F0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824155F4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824155F8: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824155FC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82415600: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415604: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415608: 4198FFBC  blt cr6, 0x824155c4
	if ctx.cr[6].lt {
	pc = 0x824155C4; continue 'dispatch;
	}
	pc = 0x8241560C; continue 'dispatch;
            }
            0x8241560C => {
    //   block [0x8241560C..0x8241562C)
	// 8241560C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82415610: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415614: 41820058  beq 0x8241566c
	if ctx.cr[0].eq {
	pc = 0x8241566C; continue 'dispatch;
	}
	// 82415618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241561C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82415620: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82415624: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82415628: 419A0044  beq cr6, 0x8241566c
	if ctx.cr[6].eq {
	pc = 0x8241566C; continue 'dispatch;
	}
	pc = 0x8241562C; continue 'dispatch;
            }
            0x8241562C => {
    //   block [0x8241562C..0x8241566C)
	// 8241562C: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82415630: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415634: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415638: 41820048  beq 0x82415680
	if ctx.cr[0].eq {
	pc = 0x82415680; continue 'dispatch;
	}
	// 8241563C: A15E0002  lhz r10, 2(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82415640: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415644: 41820038  beq 0x8241567c
	if ctx.cr[0].eq {
	pc = 0x8241567C; continue 'dispatch;
	}
	// 82415648: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241564C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415650: 41820028  beq 0x82415678
	if ctx.cr[0].eq {
	pc = 0x82415678; continue 'dispatch;
	}
	// 82415654: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82415658: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241565C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82415660: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415664: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 82415668: 4198FFC4  blt cr6, 0x8241562c
	if ctx.cr[6].lt {
	pc = 0x8241562C; continue 'dispatch;
	}
	pc = 0x8241566C; continue 'dispatch;
            }
            0x8241566C => {
    //   block [0x8241566C..0x82415674)
	// 8241566C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415670: 4811FA9C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82415674 => {
    //   block [0x82415674..0x82415678)
	// 82415674: 48000000  b 0x82415674
	pc = 0x82415674; continue 'dispatch;
            }
            0x82415678 => {
    //   block [0x82415678..0x8241567C)
	// 82415678: 48000000  b 0x82415678
	pc = 0x82415678; continue 'dispatch;
            }
            0x8241567C => {
    //   block [0x8241567C..0x82415680)
	// 8241567C: 48000000  b 0x8241567c
	pc = 0x8241567C; continue 'dispatch;
            }
            0x82415680 => {
    //   block [0x82415680..0x82415684)
	// 82415680: 48000000  b 0x82415680
	pc = 0x82415680; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415688 size=248
    let mut pc: u32 = 0x82415688;
    'dispatch: loop {
        match pc {
            0x82415688 => {
    //   block [0x82415688..0x824156DC)
	// 82415688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241568C: 4811FA31  bl 0x825350bc
	ctx.lr = 0x82415690;
	sub_82535080(ctx, base);
	// 82415690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415698: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241569C: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 824156A0: 38803030  li r4, 0x3030
	ctx.r[4].s64 = 12336;
	// 824156A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824156A8: 48000E69  bl 0x82416510
	ctx.lr = 0x824156AC;
	sub_82416510(ctx, base);
	// 824156AC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824156B0: 418200BC  beq 0x8241576c
	if ctx.cr[0].eq {
	pc = 0x8241576C; continue 'dispatch;
	}
	// 824156B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824156B8: 38803030  li r4, 0x3030
	ctx.r[4].s64 = 12336;
	// 824156BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824156C0: 48000DD9  bl 0x82416498
	ctx.lr = 0x824156C4;
	sub_82416498(ctx, base);
	// 824156C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824156C8: 418200A4  beq 0x8241576c
	if ctx.cr[0].eq {
	pc = 0x8241576C; continue 'dispatch;
	}
	// 824156CC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824156D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824156D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824156D8: 40990040  ble cr6, 0x82415718
	if !ctx.cr[6].gt {
	pc = 0x82415718; continue 'dispatch;
	}
	pc = 0x824156DC; continue 'dispatch;
            }
            0x824156DC => {
    //   block [0x824156DC..0x82415718)
	// 824156DC: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824156E0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824156E4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824156E8: 41820030  beq 0x82415718
	if ctx.cr[0].eq {
	pc = 0x82415718; continue 'dispatch;
	}
	// 824156EC: 554907BF  clrlwi. r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824156F0: 40820084  bne 0x82415774
	if !ctx.cr[0].eq {
	pc = 0x82415774; continue 'dispatch;
	}
	// 824156F4: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824156F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824156FC: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82415700: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415704: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415708: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8241570C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415710: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415714: 4198FFC8  blt cr6, 0x824156dc
	if ctx.cr[6].lt {
	pc = 0x824156DC; continue 'dispatch;
	}
	pc = 0x82415718; continue 'dispatch;
            }
            0x82415718 => {
    //   block [0x82415718..0x82415738)
	// 82415718: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241571C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415720: 4182004C  beq 0x8241576c
	if ctx.cr[0].eq {
	pc = 0x8241576C; continue 'dispatch;
	}
	// 82415724: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415728: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8241572C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82415730: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82415734: 419A0038  beq cr6, 0x8241576c
	if ctx.cr[6].eq {
	pc = 0x8241576C; continue 'dispatch;
	}
	pc = 0x82415738; continue 'dispatch;
            }
            0x82415738 => {
    //   block [0x82415738..0x8241576C)
	// 82415738: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8241573C: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415740: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415744: 41820038  beq 0x8241577c
	if ctx.cr[0].eq {
	pc = 0x8241577C; continue 'dispatch;
	}
	// 82415748: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241574C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415750: 41820028  beq 0x82415778
	if ctx.cr[0].eq {
	pc = 0x82415778; continue 'dispatch;
	}
	// 82415754: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415758: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241575C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82415760: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415764: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 82415768: 4198FFD0  blt cr6, 0x82415738
	if ctx.cr[6].lt {
	pc = 0x82415738; continue 'dispatch;
	}
	pc = 0x8241576C; continue 'dispatch;
            }
            0x8241576C => {
    //   block [0x8241576C..0x82415774)
	// 8241576C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415770: 4811F99C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82415774 => {
    //   block [0x82415774..0x82415778)
	// 82415774: 48000000  b 0x82415774
	pc = 0x82415774; continue 'dispatch;
            }
            0x82415778 => {
    //   block [0x82415778..0x8241577C)
	// 82415778: 48000000  b 0x82415778
	pc = 0x82415778; continue 'dispatch;
            }
            0x8241577C => {
    //   block [0x8241577C..0x82415780)
	// 8241577C: 48000000  b 0x8241577c
	pc = 0x8241577C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415780 size=280
    let mut pc: u32 = 0x82415780;
    'dispatch: loop {
        match pc {
            0x82415780 => {
    //   block [0x82415780..0x824157AC)
	// 82415780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415784: 4811F939  bl 0x825350bc
	ctx.lr = 0x82415788;
	sub_82535080(ctx, base);
	// 82415788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241578C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415790: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82415794: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 82415798: 38803040  li r4, 0x3040
	ctx.r[4].s64 = 12352;
	// 8241579C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824157A0: 48000D71  bl 0x82416510
	ctx.lr = 0x824157A4;
	sub_82416510(ctx, base);
	// 824157A4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824157A8: 40820008  bne 0x824157b0
	if !ctx.cr[0].eq {
	pc = 0x824157B0; continue 'dispatch;
	}
	pc = 0x824157AC; continue 'dispatch;
            }
            0x824157AC => {
    //   block [0x824157AC..0x824157B0)
	// 824157AC: 48000000  b 0x824157ac
	pc = 0x824157AC; continue 'dispatch;
            }
            0x824157B0 => {
    //   block [0x824157B0..0x824157C8)
	// 824157B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824157B4: 38803040  li r4, 0x3040
	ctx.r[4].s64 = 12352;
	// 824157B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824157BC: 48000CDD  bl 0x82416498
	ctx.lr = 0x824157C0;
	sub_82416498(ctx, base);
	// 824157C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824157C4: 40820008  bne 0x824157cc
	if !ctx.cr[0].eq {
	pc = 0x824157CC; continue 'dispatch;
	}
	pc = 0x824157C8; continue 'dispatch;
            }
            0x824157C8 => {
    //   block [0x824157C8..0x824157CC)
	// 824157C8: 48000000  b 0x824157c8
	pc = 0x824157C8; continue 'dispatch;
            }
            0x824157CC => {
    //   block [0x824157CC..0x824157DC)
	// 824157CC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824157D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824157D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824157D8: 40990040  ble cr6, 0x82415818
	if !ctx.cr[6].gt {
	pc = 0x82415818; continue 'dispatch;
	}
	pc = 0x824157DC; continue 'dispatch;
            }
            0x824157DC => {
    //   block [0x824157DC..0x82415818)
	// 824157DC: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824157E0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824157E4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824157E8: 41820030  beq 0x82415818
	if ctx.cr[0].eq {
	pc = 0x82415818; continue 'dispatch;
	}
	// 824157EC: 554907BF  clrlwi. r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824157F0: 40820038  bne 0x82415828
	if !ctx.cr[0].eq {
	pc = 0x82415828; continue 'dispatch;
	}
	// 824157F4: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824157F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824157FC: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82415800: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415804: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415808: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8241580C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415810: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415814: 4198FFC8  blt cr6, 0x824157dc
	if ctx.cr[6].lt {
	pc = 0x824157DC; continue 'dispatch;
	}
	pc = 0x82415818; continue 'dispatch;
            }
            0x82415818 => {
    //   block [0x82415818..0x82415824)
	// 82415818: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8241581C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415820: 4082000C  bne 0x8241582c
	if !ctx.cr[0].eq {
	pc = 0x8241582C; continue 'dispatch;
	}
	pc = 0x82415824; continue 'dispatch;
            }
            0x82415824 => {
    //   block [0x82415824..0x82415828)
	// 82415824: 48000000  b 0x82415824
	pc = 0x82415824; continue 'dispatch;
            }
            0x82415828 => {
    //   block [0x82415828..0x8241582C)
	// 82415828: 48000000  b 0x82415828
	pc = 0x82415828; continue 'dispatch;
            }
            0x8241582C => {
    //   block [0x8241582C..0x82415838)
	// 8241582C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415830: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 82415834: 419A0008  beq cr6, 0x8241583c
	if ctx.cr[6].eq {
	pc = 0x8241583C; continue 'dispatch;
	}
	pc = 0x82415838; continue 'dispatch;
            }
            0x82415838 => {
    //   block [0x82415838..0x8241583C)
	// 82415838: 48000000  b 0x82415838
	pc = 0x82415838; continue 'dispatch;
            }
            0x8241583C => {
    //   block [0x8241583C..0x82415850)
	// 8241583C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415840: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82415844: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82415848: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 8241584C: 419A0034  beq cr6, 0x82415880
	if ctx.cr[6].eq {
	pc = 0x82415880; continue 'dispatch;
	}
	pc = 0x82415850; continue 'dispatch;
            }
            0x82415850 => {
    //   block [0x82415850..0x8241587C)
	// 82415850: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415854: 2B0A8004  cmplwi cr6, r10, 0x8004
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32772 as u32, &mut ctx.xer);
	// 82415858: 419A0024  beq cr6, 0x8241587c
	if ctx.cr[6].eq {
	pc = 0x8241587C; continue 'dispatch;
	}
	// 8241585C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415860: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415864: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82415868: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8241586C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415870: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 82415874: 4198FFDC  blt cr6, 0x82415850
	if ctx.cr[6].lt {
	pc = 0x82415850; continue 'dispatch;
	}
	// 82415878: 48000008  b 0x82415880
	pc = 0x82415880; continue 'dispatch;
            }
            0x8241587C => {
    //   block [0x8241587C..0x82415880)
	// 8241587C: 93DF007C  stw r30, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	pc = 0x82415880; continue 'dispatch;
            }
            0x82415880 => {
    //   block [0x82415880..0x8241588C)
	// 82415880: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82415884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415888: 409A0008  bne cr6, 0x82415890
	if !ctx.cr[6].eq {
	pc = 0x82415890; continue 'dispatch;
	}
	pc = 0x8241588C; continue 'dispatch;
            }
            0x8241588C => {
    //   block [0x8241588C..0x82415890)
	// 8241588C: 48000000  b 0x8241588c
	pc = 0x8241588C; continue 'dispatch;
            }
            0x82415890 => {
    //   block [0x82415890..0x82415898)
	// 82415890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415894: 4811F878  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415898 size=24
    let mut pc: u32 = 0x82415898;
    'dispatch: loop {
        match pc {
            0x82415898 => {
    //   block [0x82415898..0x824158B0)
	// 82415898: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8241589C: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 824158A0: 7F083040  cmplw cr6, r8, r6
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[6].u32, &mut ctx.xer);
	// 824158A4: 4098000C  bge cr6, 0x824158b0
	if !ctx.cr[6].lt {
		sub_824158B0(ctx, base);
		return;
	}
	// 824158A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824158AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824158B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824158B0 size=148
    let mut pc: u32 = 0x824158B0;
    'dispatch: loop {
        match pc {
            0x824158B0 => {
    //   block [0x824158B0..0x824158F8)
	// 824158B0: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824158B4: 7F065040  cmplw cr6, r6, r10
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824158B8: 419A0084  beq cr6, 0x8241593c
	if ctx.cr[6].eq {
	pc = 0x8241593C; continue 'dispatch;
	}
	// 824158BC: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 824158C0: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824158C4: 409A0034  bne cr6, 0x824158f8
	if !ctx.cr[6].eq {
	pc = 0x824158F8; continue 'dispatch;
	}
	// 824158C8: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824158CC: A1290000  lhz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824158D0: 2B098004  cmplwi cr6, r9, 0x8004
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32772 as u32, &mut ctx.xer);
	// 824158D4: 419AFFD4  beq cr6, 0x824158a8
	if ctx.cr[6].eq {
		sub_82415898(ctx, base);
		return;
	}
	// 824158D8: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824158DC: 4099FFCC  ble cr6, 0x824158a8
	if !ctx.cr[6].gt {
		sub_82415898(ctx, base);
		return;
	}
	// 824158E0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824158E4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824158E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824158EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824158F0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824158F4: 48000044  b 0x82415938
	pc = 0x82415938; continue 'dispatch;
            }
            0x824158F8 => {
    //   block [0x824158F8..0x82415908)
	// 824158F8: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 824158FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415900: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82415904: 419A0030  beq cr6, 0x82415934
	if ctx.cr[6].eq {
	pc = 0x82415934; continue 'dispatch;
	}
	pc = 0x82415908; continue 'dispatch;
            }
            0x82415908 => {
    //   block [0x82415908..0x8241592C)
	// 82415908: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241590C: 2B098004  cmplwi cr6, r9, 0x8004
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32772 as u32, &mut ctx.xer);
	// 82415910: 419A001C  beq cr6, 0x8241592c
	if ctx.cr[6].eq {
	pc = 0x8241592C; continue 'dispatch;
	}
	// 82415914: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82415918: 40990014  ble cr6, 0x8241592c
	if !ctx.cr[6].gt {
	pc = 0x8241592C; continue 'dispatch;
	}
	// 8241591C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415920: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82415924: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82415928: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	pc = 0x8241592C; continue 'dispatch;
            }
            0x8241592C => {
    //   block [0x8241592C..0x82415934)
	// 8241592C: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82415930: 4082FFD8  bne 0x82415908
	if !ctx.cr[0].eq {
	pc = 0x82415908; continue 'dispatch;
	}
	pc = 0x82415934; continue 'dispatch;
            }
            0x82415934 => {
    //   block [0x82415934..0x82415938)
	// 82415934: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82415938; continue 'dispatch;
            }
            0x82415938 => {
    //   block [0x82415938..0x8241593C)
	// 82415938: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8241593C; continue 'dispatch;
            }
            0x8241593C => {
    //   block [0x8241593C..0x82415944)
	// 8241593C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415948 size=80
    let mut pc: u32 = 0x82415948;
    'dispatch: loop {
        match pc {
            0x82415948 => {
    //   block [0x82415948..0x82415998)
	// 82415948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241594C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82415950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82415954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415958: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241595C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82415960: 48000DA9  bl 0x82416708
	ctx.lr = 0x82415964;
	sub_82416708(ctx, base);
	// 82415964: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82415968: 4BFFF7B1  bl 0x82415118
	ctx.lr = 0x8241596C;
	sub_82415118(ctx, base);
	// 8241596C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82415970: 4BFFF7A9  bl 0x82415118
	ctx.lr = 0x82415974;
	sub_82415118(ctx, base);
	// 82415974: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82415978: 4BFFF7A1  bl 0x82415118
	ctx.lr = 0x8241597C;
	sub_82415118(ctx, base);
	// 8241597C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82415980: 4BFFF799  bl 0x82415118
	ctx.lr = 0x82415984;
	sub_82415118(ctx, base);
	// 82415984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82415988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241598C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82415990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82415994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415998 size=16
    let mut pc: u32 = 0x82415998;
    'dispatch: loop {
        match pc {
            0x82415998 => {
    //   block [0x82415998..0x824159A8)
	// 82415998: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8241599C: 38A30084  addi r5, r3, 0x84
	ctx.r[5].s64 = ctx.r[3].s64 + 132;
	// 824159A0: 38830080  addi r4, r3, 0x80
	ctx.r[4].s64 = ctx.r[3].s64 + 128;
	// 824159A4: 4BFFFEF4  b 0x82415898
	sub_82415898(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824159A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824159A8 size=204
    let mut pc: u32 = 0x824159A8;
    'dispatch: loop {
        match pc {
            0x824159A8 => {
    //   block [0x824159A8..0x82415A44)
	// 824159A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824159AC: 4811F711  bl 0x825350bc
	ctx.lr = 0x824159B0;
	sub_82535080(ctx, base);
	// 824159B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824159B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824159B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824159BC: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 824159C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824159C4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824159C8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 824159CC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 824159D0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 824159D4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 824159D8: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 824159DC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 824159E0: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 824159E4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 824159E8: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 824159EC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 824159F0: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 824159F4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 824159F8: 48000B99  bl 0x82416590
	ctx.lr = 0x824159FC;
	sub_82416590(ctx, base);
	// 824159FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82415A00: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82415A04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82415A08: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82415A0C: 388BEC54  addi r4, r11, -0x13ac
	ctx.r[4].s64 = ctx.r[11].s64 + -5036;
	// 82415A10: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82415A14: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82415A18: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82415A1C: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82415A20: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82415A24: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82415A28: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82415A2C: 93DF007C  stw r30, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82415A30: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 82415A34: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82415A38: 48000999  bl 0x824163d0
	ctx.lr = 0x82415A3C;
	sub_824163D0(ctx, base);
	// 82415A3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415A40: 40820008  bne 0x82415a48
	if !ctx.cr[0].eq {
	pc = 0x82415A48; continue 'dispatch;
	}
	pc = 0x82415A44; continue 'dispatch;
            }
            0x82415A44 => {
    //   block [0x82415A44..0x82415A48)
	// 82415A44: 48000000  b 0x82415a44
	pc = 0x82415A44; continue 'dispatch;
            }
            0x82415A48 => {
    //   block [0x82415A48..0x82415A74)
	// 82415A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A4C: 4BFFF64D  bl 0x82415098
	ctx.lr = 0x82415A50;
	sub_82415098(ctx, base);
	// 82415A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A54: 4BFFFB1D  bl 0x82415570
	ctx.lr = 0x82415A58;
	sub_82415570(ctx, base);
	// 82415A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A5C: 4BFFFC2D  bl 0x82415688
	ctx.lr = 0x82415A60;
	sub_82415688(ctx, base);
	// 82415A60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A64: 4BFFFD1D  bl 0x82415780
	ctx.lr = 0x82415A68;
	sub_82415780(ctx, base);
	// 82415A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415A70: 4811F69C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415A78 size=16
    let mut pc: u32 = 0x82415A78;
    'dispatch: loop {
        match pc {
            0x82415A78 => {
    //   block [0x82415A78..0x82415A84)
	// 82415A78: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82415A7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415A80: 419A0008  beq cr6, 0x82415a88
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82415A88);
		return;
	}
	pc = 0x82415A84; continue 'dispatch;
            }
            0x82415A84 => {
    //   block [0x82415A84..0x82415A88)
	// 82415A84: 48000000  b 0x82415a84
	pc = 0x82415A84; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415A90 size=16
    let mut pc: u32 = 0x82415A90;
    'dispatch: loop {
        match pc {
            0x82415A90 => {
    //   block [0x82415A90..0x82415A9C)
	// 82415A90: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82415A94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415A98: 419A0008  beq cr6, 0x82415aa0
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82415AA0);
		return;
	}
	pc = 0x82415A9C; continue 'dispatch;
            }
            0x82415A9C => {
    //   block [0x82415A9C..0x82415AA0)
	// 82415A9C: 48000000  b 0x82415a9c
	pc = 0x82415A9C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AA8 size=20
    let mut pc: u32 = 0x82415AA8;
    'dispatch: loop {
        match pc {
            0x82415AA8 => {
    //   block [0x82415AA8..0x82415ABC)
	// 82415AA8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415AAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415AB0: 419A000C  beq cr6, 0x82415abc
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82415ABC);
		return;
	}
	// 82415AB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AC8 size=20
    let mut pc: u32 = 0x82415AC8;
    'dispatch: loop {
        match pc {
            0x82415AC8 => {
    //   block [0x82415AC8..0x82415ADC)
	// 82415AC8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415ACC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415AD0: 419A000C  beq cr6, 0x82415adc
	if ctx.cr[6].eq {
		sub_82415ADC(ctx, base);
		return;
	}
	// 82415AD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415ADC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415ADC size=8
    let mut pc: u32 = 0x82415ADC;
    'dispatch: loop {
        match pc {
            0x82415ADC => {
    //   block [0x82415ADC..0x82415AE4)
	// 82415ADC: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AE8 size=12
    let mut pc: u32 = 0x82415AE8;
    'dispatch: loop {
        match pc {
            0x82415AE8 => {
    //   block [0x82415AE8..0x82415AF4)
	// 82415AE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82415AEC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82415AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AF8 size=12
    let mut pc: u32 = 0x82415AF8;
    'dispatch: loop {
        match pc {
            0x82415AF8 => {
    //   block [0x82415AF8..0x82415B04)
	// 82415AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415AFC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82415B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415B08 size=16
    let mut pc: u32 = 0x82415B08;
    'dispatch: loop {
        match pc {
            0x82415B08 => {
    //   block [0x82415B08..0x82415B14)
	// 82415B08: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415B0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82415B10: 409A0008  bne cr6, 0x82415b18
	if !ctx.cr[6].eq {
		sub_82415B18(ctx, base);
		return;
	}
	pc = 0x82415B14; continue 'dispatch;
            }
            0x82415B14 => {
    //   block [0x82415B14..0x82415B18)
	// 82415B14: 48000000  b 0x82415b14
	pc = 0x82415B14; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415B18 size=16
    let mut pc: u32 = 0x82415B18;
    'dispatch: loop {
        match pc {
            0x82415B18 => {
    //   block [0x82415B18..0x82415B24)
	// 82415B18: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415B1C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415B20: 41980008  blt cr6, 0x82415b28
	if ctx.cr[6].lt {
		sub_82415B28(ctx, base);
		return;
	}
	pc = 0x82415B24; continue 'dispatch;
            }
            0x82415B24 => {
    //   block [0x82415B24..0x82415B28)
	// 82415B24: 48000000  b 0x82415b24
	pc = 0x82415B24; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415B28 size=28
    let mut pc: u32 = 0x82415B28;
    'dispatch: loop {
        match pc {
            0x82415B28 => {
    //   block [0x82415B28..0x82415B44)
	// 82415B28: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415B2C: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415B30: 7CAB512E  stwx r5, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	// 82415B34: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415B38: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82415B3C: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82415B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415B48 size=440
    let mut pc: u32 = 0x82415B48;
    'dispatch: loop {
        match pc {
            0x82415B48 => {
    //   block [0x82415B48..0x82415B74)
	// 82415B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415B4C: 4811F565  bl 0x825350b0
	ctx.lr = 0x82415B50;
	sub_82535080(ctx, base);
	// 82415B50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415B54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415B58: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82415B5C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415B60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415B64: 409A0188  bne cr6, 0x82415cec
	if !ctx.cr[6].eq {
	pc = 0x82415CEC; continue 'dispatch;
	}
	// 82415B68: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415B6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415B70: 40820008  bne 0x82415b78
	if !ctx.cr[0].eq {
	pc = 0x82415B78; continue 'dispatch;
	}
	pc = 0x82415B74; continue 'dispatch;
            }
            0x82415B74 => {
    //   block [0x82415B74..0x82415B78)
	// 82415B74: 48000000  b 0x82415b74
	pc = 0x82415B74; continue 'dispatch;
            }
            0x82415B78 => {
    //   block [0x82415B78..0x82415B90)
	// 82415B78: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82415B7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415B80: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82415B84: 419A00B0  beq cr6, 0x82415c34
	if ctx.cr[6].eq {
	pc = 0x82415C34; continue 'dispatch;
	}
	// 82415B88: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82415B8C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	pc = 0x82415B90; continue 'dispatch;
            }
            0x82415B90 => {
    //   block [0x82415B90..0x82415C10)
	// 82415B90: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415B94: 7D6BE02E  lwzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82415B98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415B9C: 409A0080  bne cr6, 0x82415c1c
	if !ctx.cr[6].eq {
	pc = 0x82415C1C; continue 'dispatch;
	}
	// 82415BA0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415BA4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82415BA8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415BAC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82415BB0: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82415BB4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82415BB8: FBC10058  std r30, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u64 ) };
	// 82415BBC: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82415BC0: 7D0BE82E  lwzx r8, r11, r29
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82415BC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415BC8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82415BCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82415BD0: 4E800421  bctrl
	ctx.lr = 0x82415BD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82415BD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415BD8: 41820044  beq 0x82415c1c
	if ctx.cr[0].eq {
	pc = 0x82415C1C; continue 'dispatch;
	}
	// 82415BDC: 4BFFABE5  bl 0x824107c0
	ctx.lr = 0x82415BE0;
	sub_824107C0(ctx, base);
	// 82415BE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415BE4: 388000A8  li r4, 0xa8
	ctx.r[4].s64 = 168;
	// 82415BE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82415BF0: 4E800421  bctrl
	ctx.lr = 0x82415BF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82415BF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415BF8: 41820018  beq 0x82415c10
	if ctx.cr[0].eq {
	pc = 0x82415C10; continue 'dispatch;
	}
	// 82415BFC: E8C10058  ld r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82415C00: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82415C04: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415C08: 4BFFD991  bl 0x82413598
	ctx.lr = 0x82415C0C;
	sub_82413598(ctx, base);
	// 82415C0C: 48000008  b 0x82415c14
	pc = 0x82415C14; continue 'dispatch;
            }
            0x82415C10 => {
    //   block [0x82415C10..0x82415C14)
	// 82415C10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82415C14; continue 'dispatch;
            }
            0x82415C14 => {
    //   block [0x82415C14..0x82415C1C)
	// 82415C14: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415C18: 7C6BE12E  stwx r3, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[3].u32) };
	pc = 0x82415C1C; continue 'dispatch;
            }
            0x82415C1C => {
    //   block [0x82415C1C..0x82415C34)
	// 82415C1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415C20: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82415C24: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82415C28: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82415C2C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415C30: 4198FF60  blt cr6, 0x82415b90
	if ctx.cr[6].lt {
	pc = 0x82415B90; continue 'dispatch;
	}
	pc = 0x82415C34; continue 'dispatch;
            }
            0x82415C34 => {
    //   block [0x82415C34..0x82415C48)
	// 82415C34: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415C38: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82415C3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82415C40: 40990028  ble cr6, 0x82415c68
	if !ctx.cr[6].gt {
	pc = 0x82415C68; continue 'dispatch;
	}
	// 82415C44: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	pc = 0x82415C48; continue 'dispatch;
            }
            0x82415C48 => {
    //   block [0x82415C48..0x82415C68)
	// 82415C48: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415C4C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82415C50: 419A00A8  beq cr6, 0x82415cf8
	if ctx.cr[6].eq {
	pc = 0x82415CF8; continue 'dispatch;
	}
	// 82415C54: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415C58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415C5C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82415C60: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415C64: 4198FFE4  blt cr6, 0x82415c48
	if ctx.cr[6].lt {
	pc = 0x82415C48; continue 'dispatch;
	}
	pc = 0x82415C68; continue 'dispatch;
            }
            0x82415C68 => {
    //   block [0x82415C68..0x82415C8C)
	// 82415C68: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415C6C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415C70: 4BFFD491  bl 0x82413100
	ctx.lr = 0x82415C74;
	sub_82413100(ctx, base);
	// 82415C74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415C78: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82415C7C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82415C80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415C84: 40990034  ble cr6, 0x82415cb8
	if !ctx.cr[6].gt {
	pc = 0x82415CB8; continue 'dispatch;
	}
	// 82415C88: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	pc = 0x82415C8C; continue 'dispatch;
            }
            0x82415C8C => {
    //   block [0x82415C8C..0x82415CB8)
	// 82415C8C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415C90: 7C6BE02E  lwzx r3, r11, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82415C94: 4BFFD475  bl 0x82413108
	ctx.lr = 0x82415C98;
	sub_82413108(ctx, base);
	// 82415C98: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415C9C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82415CA0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82415CA4: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82415CA8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82415CAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415CB0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415CB4: 4198FFD8  blt cr6, 0x82415c8c
	if ctx.cr[6].lt {
	pc = 0x82415C8C; continue 'dispatch;
	}
	pc = 0x82415CB8; continue 'dispatch;
            }
            0x82415CB8 => {
    //   block [0x82415CB8..0x82415CC8)
	// 82415CB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415CBC: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82415CC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415CC4: 40990028  ble cr6, 0x82415cec
	if !ctx.cr[6].gt {
	pc = 0x82415CEC; continue 'dispatch;
	}
	pc = 0x82415CC8; continue 'dispatch;
            }
            0x82415CC8 => {
    //   block [0x82415CC8..0x82415CEC)
	// 82415CC8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415CCC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82415CD0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82415CD4: 4BFFD235  bl 0x82412f08
	ctx.lr = 0x82415CD8;
	sub_82412F08(ctx, base);
	// 82415CD8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415CDC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82415CE0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82415CE4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415CE8: 4198FFE0  blt cr6, 0x82415cc8
	if ctx.cr[6].lt {
	pc = 0x82415CC8; continue 'dispatch;
	}
	pc = 0x82415CEC; continue 'dispatch;
            }
            0x82415CEC => {
    //   block [0x82415CEC..0x82415CF0)
	// 82415CEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82415CF0; continue 'dispatch;
            }
            0x82415CF0 => {
    //   block [0x82415CF0..0x82415CF8)
	// 82415CF0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82415CF4: 4811F40C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            0x82415CF8 => {
    //   block [0x82415CF8..0x82415D00)
	// 82415CF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415CFC: 4BFFFFF4  b 0x82415cf0
	pc = 0x82415CF0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415D00 size=132
    let mut pc: u32 = 0x82415D00;
    'dispatch: loop {
        match pc {
            0x82415D00 => {
    //   block [0x82415D00..0x82415D28)
	// 82415D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415D04: 4811F3B5  bl 0x825350b8
	ctx.lr = 0x82415D08;
	sub_82535080(ctx, base);
	// 82415D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415D0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82415D10: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415D14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415D18: 409A0064  bne cr6, 0x82415d7c
	if !ctx.cr[6].eq {
	pc = 0x82415D7C; continue 'dispatch;
	}
	// 82415D1C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415D20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415D24: 40820008  bne 0x82415d2c
	if !ctx.cr[0].eq {
	pc = 0x82415D2C; continue 'dispatch;
	}
	pc = 0x82415D28; continue 'dispatch;
            }
            0x82415D28 => {
    //   block [0x82415D28..0x82415D2C)
	// 82415D28: 48000000  b 0x82415d28
	pc = 0x82415D28; continue 'dispatch;
            }
            0x82415D2C => {
    //   block [0x82415D2C..0x82415D40)
	// 82415D2C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82415D30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415D34: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82415D38: 419A003C  beq cr6, 0x82415d74
	if ctx.cr[6].eq {
	pc = 0x82415D74; continue 'dispatch;
	}
	// 82415D3C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	pc = 0x82415D40; continue 'dispatch;
            }
            0x82415D40 => {
    //   block [0x82415D40..0x82415D58)
	// 82415D40: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415D44: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82415D48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415D4C: 4182000C  beq 0x82415d58
	if ctx.cr[0].eq {
	pc = 0x82415D58; continue 'dispatch;
	}
	// 82415D50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82415D54: 4BFFC1ED  bl 0x82411f40
	ctx.lr = 0x82415D58;
	sub_82411F40(ctx, base);
	pc = 0x82415D58; continue 'dispatch;
            }
            0x82415D58 => {
    //   block [0x82415D58..0x82415D74)
	// 82415D58: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415D5C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82415D60: 7F8BF92E  stwx r28, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u32) };
	// 82415D64: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82415D68: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415D6C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415D70: 4198FFD0  blt cr6, 0x82415d40
	if ctx.cr[6].lt {
	pc = 0x82415D40; continue 'dispatch;
	}
	pc = 0x82415D74; continue 'dispatch;
            }
            0x82415D74 => {
    //   block [0x82415D74..0x82415D7C)
	// 82415D74: 939E0018  stw r28, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82415D78: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	pc = 0x82415D7C; continue 'dispatch;
            }
            0x82415D7C => {
    //   block [0x82415D7C..0x82415D84)
	// 82415D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82415D80: 4811F388  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415D88 size=20
    let mut pc: u32 = 0x82415D88;
    'dispatch: loop {
        match pc {
            0x82415D88 => {
    //   block [0x82415D88..0x82415D9C)
	// 82415D88: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415D8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415D90: 419A000C  beq cr6, 0x82415d9c
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82415D9C);
		return;
	}
	// 82415D94: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415DC8 size=180
    let mut pc: u32 = 0x82415DC8;
    'dispatch: loop {
        match pc {
            0x82415DC8 => {
    //   block [0x82415DC8..0x82415E08)
	// 82415DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415DCC: 4811F2E1  bl 0x825350ac
	ctx.lr = 0x82415DD0;
	sub_82535080(ctx, base);
	// 82415DD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415DD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415DD8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82415DDC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82415DE0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82415DE4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415DE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415DEC: 409A001C  bne cr6, 0x82415e08
	if !ctx.cr[6].eq {
	pc = 0x82415E08; continue 'dispatch;
	}
	// 82415DF0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82415DF4: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415DF8: 41980010  blt cr6, 0x82415e08
	if ctx.cr[6].lt {
	pc = 0x82415E08; continue 'dispatch;
	}
	// 82415DFC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415E00: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82415E04: 40980010  bge cr6, 0x82415e14
	if !ctx.cr[6].lt {
	pc = 0x82415E14; continue 'dispatch;
	}
	pc = 0x82415E08; continue 'dispatch;
            }
            0x82415E08 => {
    //   block [0x82415E08..0x82415E0C)
	// 82415E08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82415E0C; continue 'dispatch;
            }
            0x82415E0C => {
    //   block [0x82415E0C..0x82415E14)
	// 82415E0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82415E10: 4811F2EC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            0x82415E14 => {
    //   block [0x82415E14..0x82415E2C)
	// 82415E14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415E18: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82415E1C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82415E20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415E24: 40990044  ble cr6, 0x82415e68
	if !ctx.cr[6].gt {
	pc = 0x82415E68; continue 'dispatch;
	}
	// 82415E28: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82415E2C; continue 'dispatch;
            }
            0x82415E2C => {
    //   block [0x82415E2C..0x82415E68)
	// 82415E2C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415E30: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82415E34: 4BFFD2D5  bl 0x82413108
	ctx.lr = 0x82415E38;
	sub_82413108(ctx, base);
	// 82415E38: 7D63EA14  add r11, r3, r29
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82415E3C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415E40: 4099002C  ble cr6, 0x82415e6c
	if !ctx.cr[6].gt {
	pc = 0x82415E6C; continue 'dispatch;
	}
	// 82415E44: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415E48: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82415E4C: 4BFFD2BD  bl 0x82413108
	ctx.lr = 0x82415E50;
	sub_82413108(ctx, base);
	// 82415E50: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415E54: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82415E58: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82415E5C: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82415E60: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415E64: 4198FFC8  blt cr6, 0x82415e2c
	if ctx.cr[6].lt {
	pc = 0x82415E2C; continue 'dispatch;
	}
	pc = 0x82415E68; continue 'dispatch;
            }
            0x82415E68 => {
    //   block [0x82415E68..0x82415E6C)
	// 82415E68: 48000000  b 0x82415e68
	pc = 0x82415E68; continue 'dispatch;
            }
            0x82415E6C => {
    //   block [0x82415E6C..0x82415E7C)
	// 82415E6C: 93B90000  stw r29, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82415E70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415E74: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82415E78: 4BFFFF94  b 0x82415e0c
	pc = 0x82415E0C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415E80 size=120
    let mut pc: u32 = 0x82415E80;
    'dispatch: loop {
        match pc {
            0x82415E80 => {
    //   block [0x82415E80..0x82415E9C)
	// 82415E80: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82415E84: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415E88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82415E8C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415E90: 41820050  beq 0x82415ee0
	if ctx.cr[0].eq {
	pc = 0x82415EE0; continue 'dispatch;
	}
	// 82415E94: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415E98: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	pc = 0x82415E9C; continue 'dispatch;
            }
            0x82415E9C => {
    //   block [0x82415E9C..0x82415EA4)
	// 82415E9C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415EA0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	pc = 0x82415EA4; continue 'dispatch;
            }
            0x82415EA4 => {
    //   block [0x82415EA4..0x82415EC8)
	// 82415EA4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415EA8: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415EAC: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82415EB0: 7D084850  subf r8, r8, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82415EB4: 41820014  beq 0x82415ec8
	if ctx.cr[0].eq {
	pc = 0x82415EC8; continue 'dispatch;
	}
	// 82415EB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415EBC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82415EC0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82415EC4: 419AFFE0  beq cr6, 0x82415ea4
	if ctx.cr[6].eq {
	pc = 0x82415EA4; continue 'dispatch;
	}
	pc = 0x82415EC8; continue 'dispatch;
            }
            0x82415EC8 => {
    //   block [0x82415EC8..0x82415EE0)
	// 82415EC8: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82415ECC: 41820020  beq 0x82415eec
	if ctx.cr[0].eq {
	pc = 0x82415EEC; continue 'dispatch;
	}
	// 82415ED0: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82415ED4: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 82415ED8: 7F06F840  cmplw cr6, r6, r31
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82415EDC: 4198FFC0  blt cr6, 0x82415e9c
	if ctx.cr[6].lt {
	pc = 0x82415E9C; continue 'dispatch;
	}
	pc = 0x82415EE0; continue 'dispatch;
            }
            0x82415EE0 => {
    //   block [0x82415EE0..0x82415EE4)
	// 82415EE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82415EE4; continue 'dispatch;
            }
            0x82415EE4 => {
    //   block [0x82415EE4..0x82415EEC)
	// 82415EE4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82415EE8: 4E800020  blr
	return;
            }
            0x82415EEC => {
    //   block [0x82415EEC..0x82415EF8)
	// 82415EEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415EF0: 90C50000  stw r6, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82415EF4: 4BFFFFF0  b 0x82415ee4
	pc = 0x82415EE4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415EF8 size=20
    let mut pc: u32 = 0x82415EF8;
    'dispatch: loop {
        match pc {
            0x82415EF8 => {
    //   block [0x82415EF8..0x82415F0C)
	// 82415EF8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415EFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415F00: 419A000C  beq cr6, 0x82415f0c
	if ctx.cr[6].eq {
		sub_82415F0C(ctx, base);
		return;
	}
	// 82415F04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415F0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415F0C size=52
    let mut pc: u32 = 0x82415F0C;
    'dispatch: loop {
        match pc {
            0x82415F0C => {
    //   block [0x82415F0C..0x82415F20)
	// 82415F0C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415F14: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415F18: 4182FFEC  beq 0x82415f04
	if ctx.cr[0].eq {
		sub_82415EF8(ctx, base);
		return;
	}
	// 82415F1C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	pc = 0x82415F20; continue 'dispatch;
            }
            0x82415F20 => {
    //   block [0x82415F20..0x82415F40)
	// 82415F20: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415F24: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82415F28: 419A0018  beq cr6, 0x82415f40
	if ctx.cr[6].eq {
		sub_82415F40(ctx, base);
		return;
	}
	// 82415F2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415F30: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82415F34: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415F38: 4198FFE8  blt cr6, 0x82415f20
	if ctx.cr[6].lt {
	pc = 0x82415F20; continue 'dispatch;
	}
	// 82415F3C: 4BFFFFC8  b 0x82415f04
	sub_82415EF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415F40 size=20
    let mut pc: u32 = 0x82415F40;
    'dispatch: loop {
        match pc {
            0x82415F40 => {
    //   block [0x82415F40..0x82415F54)
	// 82415F40: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415F44: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415F48: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82415F4C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82415F50: 4BFFCF98  b 0x82412ee8
	crate::recompiler::externs::call(ctx, base, 0x82412EE8);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415F58 size=20
    let mut pc: u32 = 0x82415F58;
    'dispatch: loop {
        match pc {
            0x82415F58 => {
    //   block [0x82415F58..0x82415F6C)
	// 82415F58: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415F5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415F60: 419A000C  beq cr6, 0x82415f6c
	if ctx.cr[6].eq {
		sub_82415F6C(ctx, base);
		return;
	}
	// 82415F64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415F6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415F6C size=52
    let mut pc: u32 = 0x82415F6C;
    'dispatch: loop {
        match pc {
            0x82415F6C => {
    //   block [0x82415F6C..0x82415F80)
	// 82415F6C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415F74: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415F78: 4182FFEC  beq 0x82415f64
	if ctx.cr[0].eq {
		sub_82415F58(ctx, base);
		return;
	}
	// 82415F7C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	pc = 0x82415F80; continue 'dispatch;
            }
            0x82415F80 => {
    //   block [0x82415F80..0x82415FA0)
	// 82415F80: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415F84: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82415F88: 419A0018  beq cr6, 0x82415fa0
	if ctx.cr[6].eq {
		sub_82415FA0(ctx, base);
		return;
	}
	// 82415F8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415F90: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82415F94: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415F98: 4198FFE8  blt cr6, 0x82415f80
	if ctx.cr[6].lt {
	pc = 0x82415F80; continue 'dispatch;
	}
	// 82415F9C: 4BFFFFC8  b 0x82415f64
	sub_82415F58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415FA0 size=20
    let mut pc: u32 = 0x82415FA0;
    'dispatch: loop {
        match pc {
            0x82415FA0 => {
    //   block [0x82415FA0..0x82415FB4)
	// 82415FA0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415FA4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415FA8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82415FAC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82415FB0: 4BFFCF40  b 0x82412ef0
	crate::recompiler::externs::call(ctx, base, 0x82412EF0);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415FB8 size=20
    let mut pc: u32 = 0x82415FB8;
    'dispatch: loop {
        match pc {
            0x82415FB8 => {
    //   block [0x82415FB8..0x82415FCC)
	// 82415FB8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415FC0: 419A000C  beq cr6, 0x82415fcc
	if ctx.cr[6].eq {
		sub_82415FCC(ctx, base);
		return;
	}
	// 82415FC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415FCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415FCC size=52
    let mut pc: u32 = 0x82415FCC;
    'dispatch: loop {
        match pc {
            0x82415FCC => {
    //   block [0x82415FCC..0x82415FE0)
	// 82415FCC: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415FD4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415FD8: 4182FFEC  beq 0x82415fc4
	if ctx.cr[0].eq {
		sub_82415FB8(ctx, base);
		return;
	}
	// 82415FDC: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	pc = 0x82415FE0; continue 'dispatch;
            }
            0x82415FE0 => {
    //   block [0x82415FE0..0x82416000)
	// 82415FE0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415FE4: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82415FE8: 419A0018  beq cr6, 0x82416000
	if ctx.cr[6].eq {
		sub_82416000(ctx, base);
		return;
	}
	// 82415FEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415FF0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82415FF4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415FF8: 4198FFE8  blt cr6, 0x82415fe0
	if ctx.cr[6].lt {
	pc = 0x82415FE0; continue 'dispatch;
	}
	// 82415FFC: 4BFFFFC8  b 0x82415fc4
	sub_82415FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416000 size=20
    let mut pc: u32 = 0x82416000;
    'dispatch: loop {
        match pc {
            0x82416000 => {
    //   block [0x82416000..0x82416014)
	// 82416000: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416004: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416008: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8241600C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82416010: 4BFFCEE8  b 0x82412ef8
	crate::recompiler::externs::call(ctx, base, 0x82412EF8);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416018 size=20
    let mut pc: u32 = 0x82416018;
    'dispatch: loop {
        match pc {
            0x82416018 => {
    //   block [0x82416018..0x8241602C)
	// 82416018: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8241601C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416020: 419A000C  beq cr6, 0x8241602c
	if ctx.cr[6].eq {
		sub_8241602C(ctx, base);
		return;
	}
	// 82416024: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241602C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241602C size=52
    let mut pc: u32 = 0x8241602C;
    'dispatch: loop {
        match pc {
            0x8241602C => {
    //   block [0x8241602C..0x82416040)
	// 8241602C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416034: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416038: 4182FFEC  beq 0x82416024
	if ctx.cr[0].eq {
		sub_82416018(ctx, base);
		return;
	}
	// 8241603C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	pc = 0x82416040; continue 'dispatch;
            }
            0x82416040 => {
    //   block [0x82416040..0x82416060)
	// 82416040: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416044: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82416048: 419A0018  beq cr6, 0x82416060
	if ctx.cr[6].eq {
		sub_82416060(ctx, base);
		return;
	}
	// 8241604C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82416050: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82416054: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82416058: 4198FFE8  blt cr6, 0x82416040
	if ctx.cr[6].lt {
	pc = 0x82416040; continue 'dispatch;
	}
	// 8241605C: 4BFFFFC8  b 0x82416024
	sub_82416018(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416060 size=20
    let mut pc: u32 = 0x82416060;
    'dispatch: loop {
        match pc {
            0x82416060 => {
    //   block [0x82416060..0x82416074)
	// 82416060: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416064: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416068: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8241606C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82416070: 4BFFCE90  b 0x82412f00
	crate::recompiler::externs::call(ctx, base, 0x82412F00);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416078 size=44
    let mut pc: u32 = 0x82416078;
    'dispatch: loop {
        match pc {
            0x82416078 => {
    //   block [0x82416078..0x824160A4)
	// 82416078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241607C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82416080: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82416084: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82416088: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8241608C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82416090: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82416094: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82416098: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8241609C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824160A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824160A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824160A8 size=140
    let mut pc: u32 = 0x824160A8;
    'dispatch: loop {
        match pc {
            0x824160A8 => {
    //   block [0x824160A8..0x824160D0)
	// 824160A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824160AC: 4811F00D  bl 0x825350b8
	ctx.lr = 0x824160B0;
	sub_82535080(ctx, base);
	// 824160B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824160B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824160B8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824160BC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 824160C0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824160C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824160C8: 40990048  ble cr6, 0x82416110
	if !ctx.cr[6].gt {
	pc = 0x82416110; continue 'dispatch;
	}
	// 824160CC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	pc = 0x824160D0; continue 'dispatch;
            }
            0x824160D0 => {
    //   block [0x824160D0..0x824160F4)
	// 824160D0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824160D4: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824160D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824160DC: 419A0020  beq cr6, 0x824160fc
	if ctx.cr[6].eq {
	pc = 0x824160FC; continue 'dispatch;
	}
	// 824160E0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824160E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824160E8: 4182000C  beq 0x824160f4
	if ctx.cr[0].eq {
	pc = 0x824160F4; continue 'dispatch;
	}
	// 824160EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824160F0: 4BFFBE51  bl 0x82411f40
	ctx.lr = 0x824160F4;
	sub_82411F40(ctx, base);
	pc = 0x824160F4; continue 'dispatch;
            }
            0x824160F4 => {
    //   block [0x824160F4..0x824160FC)
	// 824160F4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824160F8: 7F8BF92E  stwx r28, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u32) };
	pc = 0x824160FC; continue 'dispatch;
            }
            0x824160FC => {
    //   block [0x824160FC..0x82416110)
	// 824160FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416100: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82416104: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82416108: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241610C: 4198FFC4  blt cr6, 0x824160d0
	if ctx.cr[6].lt {
	pc = 0x824160D0; continue 'dispatch;
	}
	pc = 0x82416110; continue 'dispatch;
            }
            0x82416110 => {
    //   block [0x82416110..0x8241612C)
	// 82416110: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82416114: 4BFFF005  bl 0x82415118
	ctx.lr = 0x82416118;
	sub_82415118(ctx, base);
	// 82416118: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241611C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416120: 4182000C  beq 0x8241612c
	if ctx.cr[0].eq {
	pc = 0x8241612C; continue 'dispatch;
	}
	// 82416124: 4811CA95  bl 0x82532bb8
	ctx.lr = 0x82416128;
	sub_82532BB8(ctx, base);
	// 82416128: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	pc = 0x8241612C; continue 'dispatch;
            }
            0x8241612C => {
    //   block [0x8241612C..0x82416134)
	// 8241612C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82416130: 4811EFD8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416138 size=256
    let mut pc: u32 = 0x82416138;
    'dispatch: loop {
        match pc {
            0x82416138 => {
    //   block [0x82416138..0x82416154)
	// 82416138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241613C: 4811EF79  bl 0x825350b4
	ctx.lr = 0x82416140;
	sub_82535080(ctx, base);
	// 82416140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416144: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82416148: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241614C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416150: 419A0008  beq cr6, 0x82416158
	if ctx.cr[6].eq {
	pc = 0x82416158; continue 'dispatch;
	}
	pc = 0x82416154; continue 'dispatch;
            }
            0x82416154 => {
    //   block [0x82416154..0x82416158)
	// 82416154: 48000000  b 0x82416154
	pc = 0x82416154; continue 'dispatch;
            }
            0x82416158 => {
    //   block [0x82416158..0x82416178)
	// 82416158: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 8241615C: 909C0004  stw r4, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82416160: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82416164: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82416168: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8241616C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82416170: 40990008  ble cr6, 0x82416178
	if !ctx.cr[6].gt {
	pc = 0x82416178; continue 'dispatch;
	}
	// 82416174: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	pc = 0x82416178; continue 'dispatch;
            }
            0x82416178 => {
    //   block [0x82416178..0x824161A4)
	// 82416178: 4BFEEC51  bl 0x82404dc8
	ctx.lr = 0x8241617C;
	sub_82404DC8(ctx, base);
	// 8241617C: 3BFC0010  addi r31, r28, 0x10
	ctx.r[31].s64 = ctx.r[28].s64 + 16;
	// 82416180: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82416184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416188: 4BFFDE91  bl 0x82414018
	ctx.lr = 0x8241618C;
	sub_82414018(ctx, base);
	// 8241618C: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416190: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82416194: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82416198: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8241619C: 40990024  ble cr6, 0x824161c0
	if !ctx.cr[6].gt {
	pc = 0x824161C0; continue 'dispatch;
	}
	// 824161A0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	pc = 0x824161A4; continue 'dispatch;
            }
            0x824161A4 => {
    //   block [0x824161A4..0x824161C0)
	// 824161A4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824161A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824161AC: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824161B0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824161B4: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824161B8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824161BC: 4198FFE8  blt cr6, 0x824161a4
	if ctx.cr[6].lt {
	pc = 0x824161A4; continue 'dispatch;
	}
	pc = 0x824161C0; continue 'dispatch;
            }
            0x824161C0 => {
    //   block [0x824161C0..0x824161DC)
	// 824161C0: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 824161C4: 83BC0004  lwz r29, 4(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824161C8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 824161CC: 57BF1838  slwi r31, r29, 3
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824161D0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824161D4: 40990008  ble cr6, 0x824161dc
	if !ctx.cr[6].gt {
	pc = 0x824161DC; continue 'dispatch;
	}
	// 824161D8: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	pc = 0x824161DC; continue 'dispatch;
            }
            0x824161DC => {
    //   block [0x824161DC..0x82416208)
	// 824161DC: 4BFFA5E5  bl 0x824107c0
	ctx.lr = 0x824161E0;
	sub_824107C0(ctx, base);
	// 824161E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824161E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824161E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824161EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824161F0: 4E800421  bctrl
	ctx.lr = 0x824161F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824161F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824161F8: 41820028  beq 0x82416220
	if ctx.cr[0].eq {
	pc = 0x82416220; continue 'dispatch;
	}
	// 824161FC: 357DFFFF  addic. r11, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82416200: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82416204: 41800018  blt 0x8241621c
	if ctx.cr[0].lt {
	pc = 0x8241621C; continue 'dispatch;
	}
            }
            0x82416208 => {
    //   block [0x82416208..0x8241621C)
	// 82416208: 93CA0000  stw r30, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8241620C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82416210: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82416214: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82416218: 4080FFF0  bge 0x82416208
	if !ctx.cr[0].lt {
	pc = 0x82416208; continue 'dispatch;
	}
	pc = 0x8241621C; continue 'dispatch;
            }
            0x8241621C => {
    //   block [0x8241621C..0x82416220)
	// 8241621C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	pc = 0x82416220; continue 'dispatch;
            }
            0x82416220 => {
    //   block [0x82416220..0x82416238)
	// 82416220: 3BFC0014  addi r31, r28, 0x14
	ctx.r[31].s64 = ctx.r[28].s64 + 20;
	// 82416224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416228: 4BFFEEF1  bl 0x82415118
	ctx.lr = 0x8241622C;
	sub_82415118(ctx, base);
	// 8241622C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82416230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82416234: 4811EED0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82416238 size=188
    let mut pc: u32 = 0x82416238;
    'dispatch: loop {
        match pc {
            0x82416238 => {
    //   block [0x82416238..0x824162D4)
	// 82416238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82416244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416248: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 8241624C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82416250: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416258: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8241625C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82416260: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82416264: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82416268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241626C: 409A0068  bne cr6, 0x824162d4
	if !ctx.cr[6].eq {
	pc = 0x824162D4; continue 'dispatch;
	}
	// 82416270: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82416274: FC00FE5E  fctidz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[31].f64.trunc() as i64 };
	// 82416278: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8241627C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82416280: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82416284: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82416288: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8241628C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82416290: 4BFFFB39  bl 0x82415dc8
	ctx.lr = 0x82416294;
	sub_82415DC8(ctx, base);
	// 82416294: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416298: 4182003C  beq 0x824162d4
	if ctx.cr[0].eq {
	pc = 0x824162D4; continue 'dispatch;
	}
	// 8241629C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824162A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824162A4: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824162A8: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 824162AC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824162B0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824162B4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824162B8: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 824162BC: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 824162C0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824162C4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824162C8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824162CC: EC3F0028  fsubs f1, f31, f0
	ctx.f[1].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 824162D0: 4BFFCE41  bl 0x82413110
	ctx.lr = 0x824162D4;
	sub_82413110(ctx, base);
	pc = 0x824162D4; continue 'dispatch;
            }
            0x824162D4 => {
    //   block [0x824162D4..0x824162F4)
	// 824162D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824162D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824162DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824162E0: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824162E4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824162E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824162EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824162F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824162F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824162F8 size=56
    let mut pc: u32 = 0x824162F8;
    'dispatch: loop {
        match pc {
            0x824162F8 => {
    //   block [0x824162F8..0x8241630C)
	// 824162F8: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824162FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416300: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416304: 41820024  beq 0x82416328
	if ctx.cr[0].eq {
	pc = 0x82416328; continue 'dispatch;
	}
	// 82416308: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	pc = 0x8241630C; continue 'dispatch;
            }
            0x8241630C => {
    //   block [0x8241630C..0x82416328)
	// 8241630C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416310: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82416314: 419A001C  beq cr6, 0x82416330
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82416330);
		return;
	}
	// 82416318: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241631C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82416320: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82416324: 4198FFE8  blt cr6, 0x8241630c
	if ctx.cr[6].lt {
	pc = 0x8241630C; continue 'dispatch;
	}
	pc = 0x82416328; continue 'dispatch;
            }
            0x82416328 => {
    //   block [0x82416328..0x82416330)
	// 82416328: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241632C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416338 size=36
    let mut pc: u32 = 0x82416338;
    'dispatch: loop {
        match pc {
            0x82416338 => {
    //   block [0x82416338..0x8241635C)
	// 82416338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416344: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82416348: 4BFFFB39  bl 0x82415e80
	ctx.lr = 0x8241634C;
	sub_82415E80(ctx, base);
	// 8241634C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82416360 size=108
    let mut pc: u32 = 0x82416360;
    'dispatch: loop {
        match pc {
            0x82416360 => {
    //   block [0x82416360..0x824163B8)
	// 82416360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241636C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416370: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82416374: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82416378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241637C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82416380: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82416384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416388: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8241638C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82416390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82416394: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82416398: 4BFFFA31  bl 0x82415dc8
	ctx.lr = 0x8241639C;
	sub_82415DC8(ctx, base);
	// 8241639C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824163A0: 41820018  beq 0x824163b8
	if ctx.cr[0].eq {
	pc = 0x824163B8; continue 'dispatch;
	}
	// 824163A4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824163A8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824163AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824163B0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824163B4: 4BFFBCBD  bl 0x82412070
	ctx.lr = 0x824163B8;
	sub_82412070(ctx, base);
	pc = 0x824163B8; continue 'dispatch;
            }
            0x824163B8 => {
    //   block [0x824163B8..0x824163CC)
	// 824163B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824163BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824163C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824163C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824163C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824163D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824163D0 size=16
    let mut pc: u32 = 0x824163D0;
    'dispatch: loop {
        match pc {
            0x824163D0 => {
    //   block [0x824163D0..0x824163DC)
	// 824163D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824163D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824163D8: 409A0008  bne cr6, 0x824163e0
	if !ctx.cr[6].eq {
		sub_824163E0(ctx, base);
		return;
	}
	pc = 0x824163DC; continue 'dispatch;
            }
            0x824163DC => {
    //   block [0x824163DC..0x824163E0)
	// 824163DC: 48000000  b 0x824163dc
	pc = 0x824163DC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824163E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824163E0 size=56
    let mut pc: u32 = 0x824163E0;
    'dispatch: loop {
        match pc {
            0x824163E0 => {
    //   block [0x824163E0..0x824163E8)
	// 824163E0: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824163E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x824163E8; continue 'dispatch;
            }
            0x824163E8 => {
    //   block [0x824163E8..0x82416410)
	// 824163E8: 7D4958AE  lbzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824163EC: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824163F0: 41820020  beq 0x82416410
	if ctx.cr[0].eq {
	pc = 0x82416410; continue 'dispatch;
	}
	// 824163F4: 7D0B20AE  lbzx r8, r11, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 824163F8: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 824163FC: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82416400: 409A0018  bne cr6, 0x82416418
	if !ctx.cr[6].eq {
		sub_82416418(ctx, base);
		return;
	}
	// 82416404: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82416408: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 8241640C: 4198FFDC  blt cr6, 0x824163e8
	if ctx.cr[6].lt {
	pc = 0x824163E8; continue 'dispatch;
	}
	pc = 0x82416410; continue 'dispatch;
            }
            0x82416410 => {
    //   block [0x82416410..0x82416418)
	// 82416410: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82416414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416418 size=8
    let mut pc: u32 = 0x82416418;
    'dispatch: loop {
        match pc {
            0x82416418 => {
    //   block [0x82416418..0x82416420)
	// 82416418: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241641C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416420 size=16
    let mut pc: u32 = 0x82416420;
    'dispatch: loop {
        match pc {
            0x82416420 => {
    //   block [0x82416420..0x8241642C)
	// 82416420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416424: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416428: 409A0008  bne cr6, 0x82416430
	if !ctx.cr[6].eq {
		sub_82416430(ctx, base);
		return;
	}
	pc = 0x8241642C; continue 'dispatch;
            }
            0x8241642C => {
    //   block [0x8241642C..0x82416430)
	// 8241642C: 48000000  b 0x8241642c
	pc = 0x8241642C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416430 size=32
    let mut pc: u32 = 0x82416430;
    'dispatch: loop {
        match pc {
            0x82416430 => {
    //   block [0x82416430..0x82416450)
	// 82416430: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416434: 896B0008  lbz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416438: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241643C: 419A0014  beq cr6, 0x82416450
	if ctx.cr[6].eq {
		sub_82416450(ctx, base);
		return;
	}
	// 82416440: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82416444: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82416448: 5563E7BC  rlwinm r3, r11, 0x1c, 0x1e, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8241644C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416450 size=8
    let mut pc: u32 = 0x82416450;
    'dispatch: loop {
        match pc {
            0x82416450 => {
    //   block [0x82416450..0x82416458)
	// 82416450: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82416454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416458 size=16
    let mut pc: u32 = 0x82416458;
    'dispatch: loop {
        match pc {
            0x82416458 => {
    //   block [0x82416458..0x82416464)
	// 82416458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241645C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416460: 409A0008  bne cr6, 0x82416468
	if !ctx.cr[6].eq {
		sub_82416468(ctx, base);
		return;
	}
	pc = 0x82416464; continue 'dispatch;
            }
            0x82416464 => {
    //   block [0x82416464..0x82416468)
	// 82416464: 48000000  b 0x82416464
	pc = 0x82416464; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416468 size=12
    let mut pc: u32 = 0x82416468;
    'dispatch: loop {
        match pc {
            0x82416468 => {
    //   block [0x82416468..0x82416474)
	// 82416468: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241646C: 886B000A  lbz r3, 0xa(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82416470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416478 size=16
    let mut pc: u32 = 0x82416478;
    'dispatch: loop {
        match pc {
            0x82416478 => {
    //   block [0x82416478..0x82416484)
	// 82416478: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241647C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416480: 409A0008  bne cr6, 0x82416488
	if !ctx.cr[6].eq {
		sub_82416488(ctx, base);
		return;
	}
	pc = 0x82416484; continue 'dispatch;
            }
            0x82416484 => {
    //   block [0x82416484..0x82416488)
	// 82416484: 48000000  b 0x82416484
	pc = 0x82416484; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416488 size=12
    let mut pc: u32 = 0x82416488;
    'dispatch: loop {
        match pc {
            0x82416488 => {
    //   block [0x82416488..0x82416494)
	// 82416488: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241648C: 886B000B  lbz r3, 0xb(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82416490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416498 size=16
    let mut pc: u32 = 0x82416498;
    'dispatch: loop {
        match pc {
            0x82416498 => {
    //   block [0x82416498..0x824164A4)
	// 82416498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241649C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824164A0: 409A0008  bne cr6, 0x824164a8
	if !ctx.cr[6].eq {
		sub_824164A8(ctx, base);
		return;
	}
	pc = 0x824164A4; continue 'dispatch;
            }
            0x824164A4 => {
    //   block [0x824164A4..0x824164A8)
	// 824164A4: 48000000  b 0x824164a4
	pc = 0x824164A4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824164A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824164A8 size=12
    let mut pc: u32 = 0x824164A8;
    'dispatch: loop {
        match pc {
            0x824164A8 => {
    //   block [0x824164A8..0x824164B0)
	// 824164A8: 5486043F  clrlwi. r6, r4, 0x10
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 824164AC: 40820008  bne 0x824164b4
	if !ctx.cr[0].eq {
		sub_824164B4(ctx, base);
		return;
	}
	pc = 0x824164B0; continue 'dispatch;
            }
            0x824164B0 => {
    //   block [0x824164B0..0x824164B4)
	// 824164B0: 48000000  b 0x824164b0
	pc = 0x824164B0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824164B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824164B4 size=80
    let mut pc: u32 = 0x824164B4;
    'dispatch: loop {
        match pc {
            0x824164B4 => {
    //   block [0x824164B4..0x824164D0)
	// 824164B4: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824164B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824164BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824164C0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824164C4: 41820038  beq 0x824164fc
	if ctx.cr[0].eq {
	pc = 0x824164FC; continue 'dispatch;
	}
	// 824164C8: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 824164CC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	pc = 0x824164D0; continue 'dispatch;
            }
            0x824164D0 => {
    //   block [0x824164D0..0x824164EC)
	// 824164D0: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824164D4: A0840000  lhz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824164D8: 7F043040  cmplw cr6, r4, r6
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[6].u32, &mut ctx.xer);
	// 824164DC: 409A0010  bne cr6, 0x824164ec
	if !ctx.cr[6].eq {
	pc = 0x824164EC; continue 'dispatch;
	}
	// 824164E0: 7F054040  cmplw cr6, r5, r8
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824164E4: 419A0020  beq cr6, 0x82416504
	if ctx.cr[6].eq {
		sub_82416504(ctx, base);
		return;
	}
	// 824164E8: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	pc = 0x824164EC; continue 'dispatch;
            }
            0x824164EC => {
    //   block [0x824164EC..0x824164FC)
	// 824164EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824164F0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824164F4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824164F8: 4198FFD8  blt cr6, 0x824164d0
	if ctx.cr[6].lt {
	pc = 0x824164D0; continue 'dispatch;
	}
	pc = 0x824164FC; continue 'dispatch;
            }
            0x824164FC => {
    //   block [0x824164FC..0x82416504)
	// 824164FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416504 size=12
    let mut pc: u32 = 0x82416504;
    'dispatch: loop {
        match pc {
            0x82416504 => {
    //   block [0x82416504..0x82416510)
	// 82416504: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416508: 7C6B382E  lwzx r3, r11, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8241650C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416510 size=16
    let mut pc: u32 = 0x82416510;
    'dispatch: loop {
        match pc {
            0x82416510 => {
    //   block [0x82416510..0x8241651C)
	// 82416510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416514: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416518: 409A0008  bne cr6, 0x82416520
	if !ctx.cr[6].eq {
		sub_82416520(ctx, base);
		return;
	}
	pc = 0x8241651C; continue 'dispatch;
            }
            0x8241651C => {
    //   block [0x8241651C..0x82416520)
	// 8241651C: 48000000  b 0x8241651c
	pc = 0x8241651C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416520 size=12
    let mut pc: u32 = 0x82416520;
    'dispatch: loop {
        match pc {
            0x82416520 => {
    //   block [0x82416520..0x82416528)
	// 82416520: 5486043F  clrlwi. r6, r4, 0x10
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82416524: 40820008  bne 0x8241652c
	if !ctx.cr[0].eq {
		sub_8241652C(ctx, base);
		return;
	}
	pc = 0x82416528; continue 'dispatch;
            }
            0x82416528 => {
    //   block [0x82416528..0x8241652C)
	// 82416528: 48000000  b 0x82416528
	pc = 0x82416528; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241652C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241652C size=80
    let mut pc: u32 = 0x8241652C;
    'dispatch: loop {
        match pc {
            0x8241652C => {
    //   block [0x8241652C..0x82416548)
	// 8241652C: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82416530: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82416534: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416538: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241653C: 41820038  beq 0x82416574
	if ctx.cr[0].eq {
	pc = 0x82416574; continue 'dispatch;
	}
	// 82416540: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82416544: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	pc = 0x82416548; continue 'dispatch;
            }
            0x82416548 => {
    //   block [0x82416548..0x82416564)
	// 82416548: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241654C: A0840000  lhz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416550: 7F043040  cmplw cr6, r4, r6
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82416554: 409A0010  bne cr6, 0x82416564
	if !ctx.cr[6].eq {
	pc = 0x82416564; continue 'dispatch;
	}
	// 82416558: 7F054040  cmplw cr6, r5, r8
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8241655C: 419A0020  beq cr6, 0x8241657c
	if ctx.cr[6].eq {
		sub_8241657C(ctx, base);
		return;
	}
	// 82416560: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	pc = 0x82416564; continue 'dispatch;
            }
            0x82416564 => {
    //   block [0x82416564..0x82416574)
	// 82416564: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82416568: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8241656C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82416570: 4198FFD8  blt cr6, 0x82416548
	if ctx.cr[6].lt {
	pc = 0x82416548; continue 'dispatch;
	}
	pc = 0x82416574; continue 'dispatch;
            }
            0x82416574 => {
    //   block [0x82416574..0x8241657C)
	// 82416574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241657C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241657C size=16
    let mut pc: u32 = 0x8241657C;
    'dispatch: loop {
        match pc {
            0x8241657C => {
    //   block [0x8241657C..0x8241658C)
	// 8241657C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416580: 7D6B382E  lwzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82416584: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82416588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416590 size=376
    let mut pc: u32 = 0x82416590;
    'dispatch: loop {
        match pc {
            0x82416590 => {
    //   block [0x82416590..0x824165C8)
	// 82416590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416594: 4811EB29  bl 0x825350bc
	ctx.lr = 0x82416598;
	sub_82535080(ctx, base);
	// 82416598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241659C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824165A0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824165A4: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 824165A8: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824165AC: F8BF0008  std r5, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 824165B0: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 824165B4: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 824165B8: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 824165BC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824165C0: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824165C4: 40820008  bne 0x824165cc
	if !ctx.cr[0].eq {
	pc = 0x824165CC; continue 'dispatch;
	}
	pc = 0x824165C8; continue 'dispatch;
            }
            0x824165C8 => {
    //   block [0x824165C8..0x824165CC)
	// 824165C8: 48000000  b 0x824165c8
	pc = 0x824165C8; continue 'dispatch;
            }
            0x824165CC => {
    //   block [0x824165CC..0x824165D8)
	// 824165CC: E95F0008  ld r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 824165D0: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 824165D4: 409A0008  bne cr6, 0x824165dc
	if !ctx.cr[6].eq {
	pc = 0x824165DC; continue 'dispatch;
	}
	pc = 0x824165D8; continue 'dispatch;
            }
            0x824165D8 => {
    //   block [0x824165D8..0x824165DC)
	// 824165D8: 48000000  b 0x824165d8
	pc = 0x824165D8; continue 'dispatch;
            }
            0x824165DC => {
    //   block [0x824165DC..0x824165EC)
	// 824165DC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824165E0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824165E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824165E8: 409A0008  bne cr6, 0x824165f0
	if !ctx.cr[6].eq {
	pc = 0x824165F0; continue 'dispatch;
	}
	pc = 0x824165EC; continue 'dispatch;
            }
            0x824165EC => {
    //   block [0x824165EC..0x824165F0)
	// 824165EC: 48000000  b 0x824165ec
	pc = 0x824165EC; continue 'dispatch;
            }
            0x824165F0 => {
    //   block [0x824165F0..0x824165FC)
	// 824165F0: 892B0008  lbz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824165F4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824165F8: 40820008  bne 0x82416600
	if !ctx.cr[0].eq {
	pc = 0x82416600; continue 'dispatch;
	}
	pc = 0x824165FC; continue 'dispatch;
            }
            0x824165FC => {
    //   block [0x824165FC..0x82416600)
	// 824165FC: 48000000  b 0x824165fc
	pc = 0x824165FC; continue 'dispatch;
            }
            0x82416600 => {
    //   block [0x82416600..0x8241660C)
	// 82416600: 892B000A  lbz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82416604: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416608: 40820008  bne 0x82416610
	if !ctx.cr[0].eq {
	pc = 0x82416610; continue 'dispatch;
	}
	pc = 0x8241660C; continue 'dispatch;
            }
            0x8241660C => {
    //   block [0x8241660C..0x82416610)
	// 8241660C: 48000000  b 0x8241660c
	pc = 0x8241660C; continue 'dispatch;
            }
            0x82416610 => {
    //   block [0x82416610..0x8241661C)
	// 82416610: 896B000B  lbz r11, 0xb(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82416614: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416618: 40820008  bne 0x82416620
	if !ctx.cr[0].eq {
	pc = 0x82416620; continue 'dispatch;
	}
	pc = 0x8241661C; continue 'dispatch;
            }
            0x8241661C => {
    //   block [0x8241661C..0x82416620)
	// 8241661C: 48000000  b 0x8241661c
	pc = 0x8241661C; continue 'dispatch;
            }
            0x82416620 => {
    //   block [0x82416620..0x8241662C)
	// 82416620: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82416624: 2F2A0010  cmpdi cr6, r10, 0x10
	ctx.cr[6].compare_i64(ctx.r[10].s64, 16, &mut ctx.xer);
	// 82416628: 40990050  ble cr6, 0x82416678
	if !ctx.cr[6].gt {
	pc = 0x82416678; continue 'dispatch;
	}
	pc = 0x8241662C; continue 'dispatch;
            }
            0x8241662C => {
    //   block [0x8241662C..0x82416678)
	// 8241662C: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82416630: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416634: 2B09FFFF  cmplwi cr6, r9, 0xffff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 65535 as u32, &mut ctx.xer);
	// 82416638: 419A0040  beq cr6, 0x82416678
	if ctx.cr[6].eq {
	pc = 0x82416678; continue 'dispatch;
	}
	// 8241663C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82416640: 419A00C4  beq cr6, 0x82416704
	if ctx.cr[6].eq {
	pc = 0x82416704; continue 'dispatch;
	}
	// 82416644: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416648: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8241664C: 419A00B4  beq cr6, 0x82416700
	if ctx.cr[6].eq {
	pc = 0x82416700; continue 'dispatch;
	}
	// 82416650: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82416654: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82416658: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8241665C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416660: E93F0008  ld r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82416664: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82416668: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8241666C: 796A0020  clrldi r10, r11, 0x20
	ctx.r[10].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82416670: 7F2A4800  cmpd cr6, r10, r9
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[9].s64, &mut ctx.xer);
	// 82416674: 4198FFB8  blt cr6, 0x8241662c
	if ctx.cr[6].lt {
	pc = 0x8241662C; continue 'dispatch;
	}
	pc = 0x82416678; continue 'dispatch;
            }
            0x82416678 => {
    //   block [0x82416678..0x8241669C)
	// 82416678: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241667C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416680: 41820074  beq 0x824166f4
	if ctx.cr[0].eq {
	pc = 0x824166F4; continue 'dispatch;
	}
	// 82416684: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 82416688: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8241668C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82416690: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82416694: 40990008  ble cr6, 0x8241669c
	if !ctx.cr[6].gt {
	pc = 0x8241669C; continue 'dispatch;
	}
	// 82416698: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8241669C; continue 'dispatch;
            }
            0x8241669C => {
    //   block [0x8241669C..0x824166C4)
	// 8241669C: 4BFEE72D  bl 0x82404dc8
	ctx.lr = 0x824166A0;
	sub_82404DC8(ctx, base);
	// 824166A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824166A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824166A8: 4BFFD971  bl 0x82414018
	ctx.lr = 0x824166AC;
	sub_82414018(ctx, base);
	// 824166AC: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824166B0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824166B4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 824166B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824166BC: 40990038  ble cr6, 0x824166f4
	if !ctx.cr[6].gt {
	pc = 0x824166F4; continue 'dispatch;
	}
	// 824166C0: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x824166C4; continue 'dispatch;
            }
            0x824166C4 => {
    //   block [0x824166C4..0x824166F4)
	// 824166C4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824166C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824166CC: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824166D0: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824166D4: 7D07492E  stwx r8, r7, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 824166D8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824166DC: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824166E0: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824166E4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824166E8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824166EC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824166F0: 4198FFD4  blt cr6, 0x824166c4
	if ctx.cr[6].lt {
	pc = 0x824166C4; continue 'dispatch;
	}
	pc = 0x824166F4; continue 'dispatch;
            }
            0x824166F4 => {
    //   block [0x824166F4..0x82416700)
	// 824166F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824166F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824166FC: 4811EA10  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82416700 => {
    //   block [0x82416700..0x82416704)
	// 82416700: 48000000  b 0x82416700
	pc = 0x82416700; continue 'dispatch;
            }
            0x82416704 => {
    //   block [0x82416704..0x82416708)
	// 82416704: 48000000  b 0x82416704
	pc = 0x82416704; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416708 size=64
    let mut pc: u32 = 0x82416708;
    'dispatch: loop {
        match pc {
            0x82416708 => {
    //   block [0x82416708..0x82416734)
	// 82416708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241670C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416710: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416714: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241671C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82416720: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416724: 41820010  beq 0x82416734
	if ctx.cr[0].eq {
	pc = 0x82416734; continue 'dispatch;
	}
	// 82416728: 4811C491  bl 0x82532bb8
	ctx.lr = 0x8241672C;
	sub_82532BB8(ctx, base);
	// 8241672C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416730: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	pc = 0x82416734; continue 'dispatch;
            }
            0x82416734 => {
    //   block [0x82416734..0x82416748)
	// 82416734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241673C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416750 size=228
    let mut pc: u32 = 0x82416750;
    'dispatch: loop {
        match pc {
            0x82416750 => {
    //   block [0x82416750..0x82416774)
	// 82416750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241675C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416764: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82416768: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241676C: 38EBF210  addi r7, r11, -0xdf0
	ctx.r[7].s64 = ctx.r[11].s64 + -3568;
	// 82416770: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	pc = 0x82416774; continue 'dispatch;
            }
            0x82416774 => {
    //   block [0x82416774..0x824167A8)
	// 82416774: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82416778: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241677C: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82416780: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82416784: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82416788: 4082FFEC  bne 0x82416774
	if !ctx.cr[0].eq {
	pc = 0x82416774; continue 'dispatch;
	}
	// 8241678C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82416790: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82416794: 419A0014  beq cr6, 0x824167a8
	if ctx.cr[6].eq {
	pc = 0x824167A8; continue 'dispatch;
	}
	// 82416798: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241679C: 386BEC98  addi r3, r11, -0x1368
	ctx.r[3].s64 = ctx.r[11].s64 + -4968;
	// 824167A0: 48006469  bl 0x8241cc08
	ctx.lr = 0x824167A4;
	sub_8241CC08(ctx, base);
	// 824167A4: 48000078  b 0x8241681c
	pc = 0x8241681C; continue 'dispatch;
            }
            0x824167A8 => {
    //   block [0x824167A8..0x82416814)
	// 824167A8: 48008C79  bl 0x8241f420
	ctx.lr = 0x824167AC;
	sub_8241F420(ctx, base);
	// 824167AC: 3D608241  lis r11, -0x7dbf
	ctx.r[11].s64 = -2109669376;
	// 824167B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824167B4: 386B6748  addi r3, r11, 0x6748
	ctx.r[3].s64 = ctx.r[11].s64 + 26440;
	// 824167B8: 480094E1  bl 0x8241fc98
	ctx.lr = 0x824167BC;
	sub_8241FC98(ctx, base);
	// 824167BC: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 824167C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824167C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824167C8: 386AEC90  addi r3, r10, -0x1370
	ctx.r[3].s64 = ctx.r[10].s64 + -4976;
	// 824167CC: 388B05E8  addi r4, r11, 0x5e8
	ctx.r[4].s64 = ctx.r[11].s64 + 1512;
	// 824167D0: 48009D49  bl 0x82420518
	ctx.lr = 0x824167D4;
	sub_82420518(ctx, base);
	// 824167D4: 48007595  bl 0x8241dd68
	ctx.lr = 0x824167D8;
	sub_8241DD68(ctx, base);
	// 824167D8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824167DC: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 824167E0: 3BEAE0F4  addi r31, r10, -0x1f0c
	ctx.r[31].s64 = ctx.r[10].s64 + -7948;
	// 824167E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824167E8: 388BCC80  addi r4, r11, -0x3380
	ctx.r[4].s64 = ctx.r[11].s64 + -13184;
	// 824167EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824167F0: 48009D29  bl 0x82420518
	ctx.lr = 0x824167F4;
	sub_82420518(ctx, base);
	// 824167F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824167F8: 480097F9  bl 0x8241fff0
	ctx.lr = 0x824167FC;
	sub_8241FFF0(ctx, base);
	// 824167FC: 48006325  bl 0x8241cb20
	ctx.lr = 0x82416800;
	sub_8241CB20(ctx, base);
	// 82416800: 48008B19  bl 0x8241f318
	ctx.lr = 0x82416804;
	sub_8241F318(ctx, base);
	// 82416804: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82416808: 419A000C  beq cr6, 0x82416814
	if ctx.cr[6].eq {
	pc = 0x82416814; continue 'dispatch;
	}
	// 8241680C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416810: 48000008  b 0x82416818
	pc = 0x82416818; continue 'dispatch;
            }
            0x82416814 => {
    //   block [0x82416814..0x82416818)
	// 82416814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82416818; continue 'dispatch;
            }
            0x82416818 => {
    //   block [0x82416818..0x8241681C)
	// 82416818: 48007461  bl 0x8241dc78
	ctx.lr = 0x8241681C;
	sub_8241DC78(ctx, base);
	pc = 0x8241681C; continue 'dispatch;
            }
            0x8241681C => {
    //   block [0x8241681C..0x82416834)
	// 8241681C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82416820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416828: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241682C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416838 size=132
    let mut pc: u32 = 0x82416838;
    'dispatch: loop {
        match pc {
            0x82416838 => {
    //   block [0x82416838..0x82416858)
	// 82416838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241683C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416844: 48005715  bl 0x8241bf58
	ctx.lr = 0x82416848;
	sub_8241BF58(ctx, base);
	// 82416848: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241684C: 41820014  beq 0x82416860
	if ctx.cr[0].eq {
	pc = 0x82416860; continue 'dispatch;
	}
	// 82416850: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416854: 386BED48  addi r3, r11, -0x12b8
	ctx.r[3].s64 = ctx.r[11].s64 + -4792;
	pc = 0x82416858; continue 'dispatch;
            }
            0x82416858 => {
    //   block [0x82416858..0x82416860)
	// 82416858: 480063B1  bl 0x8241cc08
	ctx.lr = 0x8241685C;
	sub_8241CC08(ctx, base);
	// 8241685C: 48000050  b 0x824168ac
	pc = 0x824168AC; continue 'dispatch;
            }
            0x82416860 => {
    //   block [0x82416860..0x8241686C)
	// 82416860: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82416864: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82416868: 38EBF210  addi r7, r11, -0xdf0
	ctx.r[7].s64 = ctx.r[11].s64 + -3568;
	pc = 0x8241686C; continue 'dispatch;
            }
            0x8241686C => {
    //   block [0x8241686C..0x8241689C)
	// 8241686C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82416870: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82416874: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82416878: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241687C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82416880: 4082FFEC  bne 0x8241686c
	if !ctx.cr[0].eq {
	pc = 0x8241686C; continue 'dispatch;
	}
	// 82416884: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82416888: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241688C: 409A0010  bne cr6, 0x8241689c
	if !ctx.cr[6].eq {
	pc = 0x8241689C; continue 'dispatch;
	}
	// 82416890: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416894: 386BECE8  addi r3, r11, -0x1318
	ctx.r[3].s64 = ctx.r[11].s64 + -4888;
	// 82416898: 4BFFFFC0  b 0x82416858
	pc = 0x82416858; continue 'dispatch;
            }
            0x8241689C => {
    //   block [0x8241689C..0x824168AC)
	// 8241689C: 48008AD5  bl 0x8241f370
	ctx.lr = 0x824168A0;
	sub_8241F370(ctx, base);
	// 824168A0: 480062B9  bl 0x8241cb58
	ctx.lr = 0x824168A4;
	sub_8241CB58(ctx, base);
	// 824168A4: 48008BED  bl 0x8241f490
	ctx.lr = 0x824168A8;
	sub_8241F490(ctx, base);
	// 824168A8: 48008671  bl 0x8241ef18
	ctx.lr = 0x824168AC;
	sub_8241EF18(ctx, base);
	pc = 0x824168AC; continue 'dispatch;
            }
            0x824168AC => {
    //   block [0x824168AC..0x824168BC)
	// 824168AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824168B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824168B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824168B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824168C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824168C0 size=8
    let mut pc: u32 = 0x824168C0;
    'dispatch: loop {
        match pc {
            0x824168C0 => {
    //   block [0x824168C0..0x824168C8)
	// 824168C0: 480068E0  b 0x8241d1a0
	crate::recompiler::externs::call(ctx, base, 0x8241D1A0);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824168C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824168C8 size=56
    let mut pc: u32 = 0x824168C8;
    'dispatch: loop {
        match pc {
            0x824168C8 => {
    //   block [0x824168C8..0x82416900)
	// 824168C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824168CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824168D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824168D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824168D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824168DC: 4800A755  bl 0x82421030
	ctx.lr = 0x824168E0;
	sub_82421030(ctx, base);
	// 824168E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824168E4: 4800856D  bl 0x8241ee50
	ctx.lr = 0x824168E8;
	sub_8241EE50(ctx, base);
	// 824168E8: 4800A789  bl 0x82421070
	ctx.lr = 0x824168EC;
	sub_82421070(ctx, base);
	// 824168EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824168F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824168F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824168F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824168FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82416900 size=132
    let mut pc: u32 = 0x82416900;
    'dispatch: loop {
        match pc {
            0x82416900 => {
    //   block [0x82416900..0x82416950)
	// 82416900: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82416904: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82416908: 3D208313  lis r9, -0x7ced
	ctx.r[9].s64 = -2095906816;
	// 8241690C: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82416910: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82416914: 390AC420  addi r8, r10, -0x3be0
	ctx.r[8].s64 = ctx.r[10].s64 + -15328;
	// 82416918: 8169C520  lwz r11, -0x3ae0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15072 as u32) ) } as u64;
	// 8241691C: 7D7F2670  srawi r31, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82416920: 7D5F0194  addze r10, r31
	tmp.s64 = ctx.r[31].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[31].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82416924: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82416928: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8241692C: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416930: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82416934: 3D008313  lis r8, -0x7ced
	ctx.r[8].s64 = -2095906816;
	// 82416938: 3908C540  addi r8, r8, -0x3ac0
	ctx.r[8].s64 = ctx.r[8].s64 + -15040;
	// 8241693C: 409A0014  bne cr6, 0x82416950
	if !ctx.cr[6].eq {
	pc = 0x82416950; continue 'dispatch;
	}
	// 82416940: 547F083C  slwi r31, r3, 1
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82416944: 7FDF422E  lhzx r30, r31, r8
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82416948: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8241694C: 7FDF432E  sthx r30, r31, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[30].u16) };
	pc = 0x82416950; continue 'dispatch;
            }
            0x82416950 => {
    //   block [0x82416950..0x82416984)
	// 82416950: 547F083C  slwi r31, r3, 1
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82416954: 986B0000  stb r3, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 82416958: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241695C: 988B0001  stb r4, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 82416960: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82416964: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82416968: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8241696C: 7D1F422E  lhzx r8, r31, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82416970: 9149C520  stw r10, -0x3ae0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-15072 as u32), ctx.r[10].u32 ) };
	// 82416974: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82416978: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241697C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82416980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416988 size=80
    let mut pc: u32 = 0x82416988;
    'dispatch: loop {
        match pc {
            0x82416988 => {
    //   block [0x82416988..0x824169B0)
	// 82416988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241698C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416994: 2B0300FF  cmplwi cr6, r3, 0xff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 255 as u32, &mut ctx.xer);
	// 82416998: 41990020  bgt cr6, 0x824169b8
	if ctx.cr[6].gt {
	pc = 0x824169B8; continue 'dispatch;
	}
	// 8241699C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824169A0: 409A0010  bne cr6, 0x824169b0
	if !ctx.cr[6].eq {
	pc = 0x824169B0; continue 'dispatch;
	}
	// 824169A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824169A8: 386BEDE4  addi r3, r11, -0x121c
	ctx.r[3].s64 = ctx.r[11].s64 + -4636;
	// 824169AC: 48000014  b 0x824169c0
	pc = 0x824169C0; continue 'dispatch;
            }
            0x824169B0 => {
    //   block [0x824169B0..0x824169B8)
	// 824169B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824169B4: 48000014  b 0x824169c8
	pc = 0x824169C8; continue 'dispatch;
            }
            0x824169B8 => {
    //   block [0x824169B8..0x824169C0)
	// 824169B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824169BC: 386BEDAC  addi r3, r11, -0x1254
	ctx.r[3].s64 = ctx.r[11].s64 + -4692;
	pc = 0x824169C0; continue 'dispatch;
            }
            0x824169C0 => {
    //   block [0x824169C0..0x824169C8)
	// 824169C0: 4800A8D9  bl 0x82421298
	ctx.lr = 0x824169C4;
	sub_82421298(ctx, base);
	// 824169C4: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	pc = 0x824169C8; continue 'dispatch;
            }
            0x824169C8 => {
    //   block [0x824169C8..0x824169D8)
	// 824169C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824169CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824169D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824169D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824169D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824169D8 size=56
    let mut pc: u32 = 0x824169D8;
    'dispatch: loop {
        match pc {
            0x824169D8 => {
    //   block [0x824169D8..0x824169EC)
	// 824169D8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824169DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824169E0: 396BC980  addi r11, r11, -0x3680
	ctx.r[11].s64 = ctx.r[11].s64 + -13952;
	// 824169E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824169E8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x824169EC; continue 'dispatch;
            }
            0x824169EC => {
    //   block [0x824169EC..0x82416A10)
	// 824169EC: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824169F0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824169F4: 419A001C  beq cr6, 0x82416a10
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82416A10);
		return;
	}
	// 824169F8: 394A0034  addi r10, r10, 0x34
	ctx.r[10].s64 = ctx.r[10].s64 + 52;
	// 824169FC: 390B0340  addi r8, r11, 0x340
	ctx.r[8].s64 = ctx.r[11].s64 + 832;
	// 82416A00: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82416A04: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82416A08: 4198FFE4  blt cr6, 0x824169ec
	if ctx.cr[6].lt {
	pc = 0x824169EC; continue 'dispatch;
	}
	// 82416A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416A20 size=156
    let mut pc: u32 = 0x82416A20;
    'dispatch: loop {
        match pc {
            0x82416A20 => {
    //   block [0x82416A20..0x82416A44)
	// 82416A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416A28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416A2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416A30: 4BFFFFA9  bl 0x824169d8
	ctx.lr = 0x82416A34;
	sub_824169D8(ctx, base);
	// 82416A34: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82416A38: 40820018  bne 0x82416a50
	if !ctx.cr[0].eq {
	pc = 0x82416A50; continue 'dispatch;
	}
	// 82416A3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416A40: 386BEE48  addi r3, r11, -0x11b8
	ctx.r[3].s64 = ctx.r[11].s64 + -4536;
	pc = 0x82416A44; continue 'dispatch;
            }
            0x82416A44 => {
    //   block [0x82416A44..0x82416A50)
	// 82416A44: 4800A855  bl 0x82421298
	ctx.lr = 0x82416A48;
	sub_82421298(ctx, base);
	// 82416A48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416A4C: 4800005C  b 0x82416aa8
	pc = 0x82416AA8; continue 'dispatch;
            }
            0x82416A50 => {
    //   block [0x82416A50..0x82416A74)
	// 82416A50: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82416A54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416A58: 4800B759  bl 0x824221b0
	ctx.lr = 0x82416A5C;
	sub_824221B0(ctx, base);
	// 82416A5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416A60: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82416A64: 40820010  bne 0x82416a74
	if !ctx.cr[0].eq {
	pc = 0x82416A74; continue 'dispatch;
	}
	// 82416A68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416A6C: 386BEE10  addi r3, r11, -0x11f0
	ctx.r[3].s64 = ctx.r[11].s64 + -4592;
	// 82416A70: 4BFFFFD4  b 0x82416a44
	pc = 0x82416A44; continue 'dispatch;
            }
            0x82416A74 => {
    //   block [0x82416A74..0x82416AA8)
	// 82416A74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416A78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82416A7C: 39200200  li r9, 0x200
	ctx.r[9].s64 = 512;
	// 82416A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416A84: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82416A88: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82416A8C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82416A90: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82416A94: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82416A98: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82416A9C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82416AA0: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82416AA4: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	pc = 0x82416AA8; continue 'dispatch;
            }
            0x82416AA8 => {
    //   block [0x82416AA8..0x82416ABC)
	// 82416AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416AC0 size=108
    let mut pc: u32 = 0x82416AC0;
    'dispatch: loop {
        match pc {
            0x82416AC0 => {
    //   block [0x82416AC0..0x82416AEC)
	// 82416AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416ACC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82416AD0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82416AD4: 409A0018  bne cr6, 0x82416aec
	if !ctx.cr[6].eq {
	pc = 0x82416AEC; continue 'dispatch;
	}
	// 82416AD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416ADC: 386BEE7C  addi r3, r11, -0x1184
	ctx.r[3].s64 = ctx.r[11].s64 + -4484;
	// 82416AE0: 4800A7B9  bl 0x82421298
	ctx.lr = 0x82416AE4;
	sub_82421298(ctx, base);
	// 82416AE4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82416AE8: 48000034  b 0x82416b1c
	pc = 0x82416B1C; continue 'dispatch;
            }
            0x82416AEC => {
    //   block [0x82416AEC..0x82416B1C)
	// 82416AEC: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82416AF0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416AF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82416AF8: 90AB0030  stw r5, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 82416AFC: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82416B00: 38E0F800  li r7, -0x800
	ctx.r[7].s64 = -2048;
	// 82416B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82416B08: 78E70580  clrldi r7, r7, 0x16
	ctx.r[7].u64 = ctx.r[7].u64 & 0x000003FFFFFFFFFFu64;
	// 82416B0C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82416B10: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82416B14: 4800B6ED  bl 0x82422200
	ctx.lr = 0x82416B18;
	sub_82422200(ctx, base);
	// 82416B18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82416B1C; continue 'dispatch;
            }
            0x82416B1C => {
    //   block [0x82416B1C..0x82416B2C)
	// 82416B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416B30 size=120
    let mut pc: u32 = 0x82416B30;
    'dispatch: loop {
        match pc {
            0x82416B30 => {
    //   block [0x82416B30..0x82416B78)
	// 82416B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416B44: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416B48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416B4C: 419A0048  beq cr6, 0x82416b94
	if ctx.cr[6].eq {
	pc = 0x82416B94; continue 'dispatch;
	}
	// 82416B50: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82416B54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416B58: 409A003C  bne cr6, 0x82416b94
	if !ctx.cr[6].eq {
	pc = 0x82416B94; continue 'dispatch;
	}
	// 82416B5C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82416B60: 816BC524  lwz r11, -0x3adc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15068 as u32) ) } as u64;
	// 82416B64: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82416B68: 409A0010  bne cr6, 0x82416b78
	if !ctx.cr[6].eq {
	pc = 0x82416B78; continue 'dispatch;
	}
	// 82416B6C: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82416B70: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82416B74: 480183DD  bl 0x8242ef50
	ctx.lr = 0x82416B78;
	sub_8242EF50(ctx, base);
	pc = 0x82416B78; continue 'dispatch;
            }
            0x82416B78 => {
    //   block [0x82416B78..0x82416B94)
	// 82416B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416B7C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416B80: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82416B84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416B88: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82416B8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82416B90: 4E800421  bctrl
	ctx.lr = 0x82416B94;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82416B94 => {
    //   block [0x82416B94..0x82416BA8)
	// 82416B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416BA8 size=228
    let mut pc: u32 = 0x82416BA8;
    'dispatch: loop {
        match pc {
            0x82416BA8 => {
    //   block [0x82416BA8..0x82416BD8)
	// 82416BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416BAC: 4811E511  bl 0x825350bc
	ctx.lr = 0x82416BB0;
	sub_82535080(ctx, base);
	// 82416BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416BB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82416BBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82416BC0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416BC4: 4800ACBD  bl 0x82421880
	ctx.lr = 0x82416BC8;
	sub_82421880(ctx, base);
	// 82416BC8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82416BCC: 419A000C  beq cr6, 0x82416bd8
	if ctx.cr[6].eq {
	pc = 0x82416BD8; continue 'dispatch;
	}
	// 82416BD0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416BD4: 4800B96D  bl 0x82422540
	ctx.lr = 0x82416BD8;
	sub_82422540(ctx, base);
	pc = 0x82416BD8; continue 'dispatch;
            }
            0x82416BD8 => {
    //   block [0x82416BD8..0x82416C00)
	// 82416BD8: 48018379  bl 0x8242ef50
	ctx.lr = 0x82416BDC;
	sub_8242EF50(ctx, base);
	// 82416BDC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416BE0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82416BE4: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82416BE8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82416BEC: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82416BF0: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82416BF4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82416BF8: 40980008  bge cr6, 0x82416c00
	if !ctx.cr[6].lt {
	pc = 0x82416C00; continue 'dispatch;
	}
	// 82416BFC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	pc = 0x82416C00; continue 'dispatch;
            }
            0x82416C00 => {
    //   block [0x82416C00..0x82416C20)
	// 82416C00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82416C04: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82416C08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82416C0C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82416C10: 409A0010  bne cr6, 0x82416c20
	if !ctx.cr[6].eq {
	pc = 0x82416C20; continue 'dispatch;
	}
	// 82416C14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82416C18: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416C1C: 4800005C  b 0x82416c78
	pc = 0x82416C78; continue 'dispatch;
            }
            0x82416C20 => {
    //   block [0x82416C20..0x82416C78)
	// 82416C20: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82416C24: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C28: 4800AD71  bl 0x82421998
	ctx.lr = 0x82416C2C;
	sub_82421998(ctx, base);
	// 82416C2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82416C30: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C34: 4800B82D  bl 0x82422460
	ctx.lr = 0x82416C38;
	sub_82422460(ctx, base);
	// 82416C38: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82416C3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C40: 4800B479  bl 0x824220b8
	ctx.lr = 0x82416C44;
	sub_824220B8(ctx, base);
	// 82416C44: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82416C48: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82416C50: 9BDF0003  stb r30, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[30].u8 ) };
	// 82416C54: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416C58: 4800B519  bl 0x82422170
	ctx.lr = 0x82416C5C;
	sub_82422170(ctx, base);
	// 82416C5C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416C60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C64: 4800AC5D  bl 0x824218c0
	ctx.lr = 0x82416C68;
	sub_824218C0(ctx, base);
	// 82416C68: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82416C6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C70: 4800B679  bl 0x824222e8
	ctx.lr = 0x82416C74;
	sub_824222E8(ctx, base);
	// 82416C74: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	pc = 0x82416C78; continue 'dispatch;
            }
            0x82416C78 => {
    //   block [0x82416C78..0x82416C8C)
	// 82416C78: 480182D9  bl 0x8242ef50
	ctx.lr = 0x82416C7C;
	sub_8242EF50(ctx, base);
	// 82416C7C: 4800BA55  bl 0x824226d0
	ctx.lr = 0x82416C80;
	sub_824226D0(ctx, base);
	// 82416C80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82416C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82416C88: 4811E484  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416C90 size=460
    let mut pc: u32 = 0x82416C90;
    'dispatch: loop {
        match pc {
            0x82416C90 => {
    //   block [0x82416C90..0x82416CD0)
	// 82416C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416C94: 4811E421  bl 0x825350b4
	ctx.lr = 0x82416C98;
	sub_82535080(ctx, base);
	// 82416C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416C9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416CA0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82416CA4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82416CA8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82416CAC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82416CB0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82416CB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82416CB8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82416CBC: 4BFFFC45  bl 0x82416900
	ctx.lr = 0x82416CC0;
	sub_82416900(ctx, base);
	// 82416CC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82416CC4: 409A0018  bne cr6, 0x82416cdc
	if !ctx.cr[6].eq {
	pc = 0x82416CDC; continue 'dispatch;
	}
	// 82416CC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416CCC: 386BEFA4  addi r3, r11, -0x105c
	ctx.r[3].s64 = ctx.r[11].s64 + -4188;
	pc = 0x82416CD0; continue 'dispatch;
            }
            0x82416CD0 => {
    //   block [0x82416CD0..0x82416CDC)
	// 82416CD0: 4800A5C9  bl 0x82421298
	ctx.lr = 0x82416CD4;
	sub_82421298(ctx, base);
	// 82416CD4: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82416CD8: 4800017C  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
            }
            0x82416CDC => {
    //   block [0x82416CDC..0x82416CF0)
	// 82416CDC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82416CE0: 40980010  bge cr6, 0x82416cf0
	if !ctx.cr[6].lt {
	pc = 0x82416CF0; continue 'dispatch;
	}
	// 82416CE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416CE8: 386BEF78  addi r3, r11, -0x1088
	ctx.r[3].s64 = ctx.r[11].s64 + -4232;
	// 82416CEC: 4BFFFFE4  b 0x82416cd0
	pc = 0x82416CD0; continue 'dispatch;
            }
            0x82416CF0 => {
    //   block [0x82416CF0..0x82416D08)
	// 82416CF0: 3D600010  lis r11, 0x10
	ctx.r[11].s64 = 1048576;
	// 82416CF4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82416CF8: 41980010  blt cr6, 0x82416d08
	if ctx.cr[6].lt {
	pc = 0x82416D08; continue 'dispatch;
	}
	// 82416CFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416D00: 386BEF48  addi r3, r11, -0x10b8
	ctx.r[3].s64 = ctx.r[11].s64 + -4280;
	// 82416D04: 4BFFFFCC  b 0x82416cd0
	pc = 0x82416CD0; continue 'dispatch;
            }
            0x82416D08 => {
    //   block [0x82416D08..0x82416D1C)
	// 82416D08: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82416D0C: 409A0010  bne cr6, 0x82416d1c
	if !ctx.cr[6].eq {
	pc = 0x82416D1C; continue 'dispatch;
	}
	// 82416D10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416D14: 386BEF20  addi r3, r11, -0x10e0
	ctx.r[3].s64 = ctx.r[11].s64 + -4320;
	// 82416D18: 4BFFFFB8  b 0x82416cd0
	pc = 0x82416CD0; continue 'dispatch;
            }
            0x82416D1C => {
    //   block [0x82416D1C..0x82416D30)
	// 82416D1C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82416D20: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82416D24: 409A000C  bne cr6, 0x82416d30
	if !ctx.cr[6].eq {
	pc = 0x82416D30; continue 'dispatch;
	}
	// 82416D28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416D2C: 48000128  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
            }
            0x82416D30 => {
    //   block [0x82416D30..0x82416D70)
	// 82416D30: 48018221  bl 0x8242ef50
	ctx.lr = 0x82416D34;
	sub_8242EF50(ctx, base);
	// 82416D34: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416D38: 4800A8D9  bl 0x82421610
	ctx.lr = 0x82416D3C;
	sub_82421610(ctx, base);
	// 82416D3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82416D40: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416D44: 4800A88D  bl 0x824215d0
	ctx.lr = 0x82416D48;
	sub_824215D0(ctx, base);
	// 82416D48: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82416D4C: 48018205  bl 0x8242ef50
	ctx.lr = 0x82416D50;
	sub_8242EF50(ctx, base);
	// 82416D50: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82416D54: 409A001C  bne cr6, 0x82416d70
	if !ctx.cr[6].eq {
	pc = 0x82416D70; continue 'dispatch;
	}
	// 82416D58: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82416D5C: 409A0014  bne cr6, 0x82416d70
	if !ctx.cr[6].eq {
	pc = 0x82416D70; continue 'dispatch;
	}
	// 82416D60: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82416D64: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82416D68: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416D6C: 480000E8  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
            }
            0x82416D70 => {
    //   block [0x82416D70..0x82416D98)
	// 82416D70: 480181E1  bl 0x8242ef50
	ctx.lr = 0x82416D74;
	sub_8242EF50(ctx, base);
	// 82416D74: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416D78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416D7C: 419A001C  beq cr6, 0x82416d98
	if ctx.cr[6].eq {
	pc = 0x82416D98; continue 'dispatch;
	}
	// 82416D80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416D84: 386BEEF4  addi r3, r11, -0x110c
	ctx.r[3].s64 = ctx.r[11].s64 + -4364;
	// 82416D88: 4800A511  bl 0x82421298
	ctx.lr = 0x82416D8C;
	sub_82421298(ctx, base);
	// 82416D8C: 480181C5  bl 0x8242ef50
	ctx.lr = 0x82416D90;
	sub_8242EF50(ctx, base);
	// 82416D90: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82416D94: 480000C0  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
            }
            0x82416D98 => {
    //   block [0x82416D98..0x82416DC0)
	// 82416D98: 480181B9  bl 0x8242ef50
	ctx.lr = 0x82416D9C;
	sub_8242EF50(ctx, base);
	// 82416D9C: 577E5828  slwi r30, r27, 0xb
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(11);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82416DA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82416DA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82416DA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82416DAC: 4800C69D  bl 0x82423448
	ctx.lr = 0x82416DB0;
	sub_82423448(ctx, base);
	// 82416DB0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82416DB4: 4082000C  bne 0x82416dc0
	if !ctx.cr[0].eq {
	pc = 0x82416DC0; continue 'dispatch;
	}
	// 82416DB8: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82416DBC: 48000098  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
            }
            0x82416DC0 => {
    //   block [0x82416DC0..0x82416DEC)
	// 82416DC0: 48018191  bl 0x8242ef50
	ctx.lr = 0x82416DC4;
	sub_8242EF50(ctx, base);
	// 82416DC4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82416DC8: 939F0020  stw r28, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82416DCC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82416DD0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82416DD4: 816BC524  lwz r11, -0x3adc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15068 as u32) ) } as u64;
	// 82416DD8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82416DDC: 409A0010  bne cr6, 0x82416dec
	if !ctx.cr[6].eq {
	pc = 0x82416DEC; continue 'dispatch;
	}
	// 82416DE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82416DE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82416DE8: 48018169  bl 0x8242ef50
	ctx.lr = 0x82416DEC;
	sub_8242EF50(ctx, base);
	pc = 0x82416DEC; continue 'dispatch;
            }
            0x82416DEC => {
    //   block [0x82416DEC..0x82416E30)
	// 82416DEC: 48018165  bl 0x8242ef50
	ctx.lr = 0x82416DF0;
	sub_8242EF50(ctx, base);
	// 82416DF0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82416DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416DF8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416DFC: 4BFFFDAD  bl 0x82416ba8
	ctx.lr = 0x82416E00;
	sub_82416BA8(ctx, base);
	// 82416E00: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82416E04: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82416E08: 4181002C  bgt 0x82416e34
	if ctx.cr[0].gt {
	pc = 0x82416E34; continue 'dispatch;
	}
	// 82416E0C: 48018145  bl 0x8242ef50
	ctx.lr = 0x82416E10;
	sub_8242EF50(ctx, base);
	// 82416E10: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416E14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416E18: 41820018  beq 0x82416e30
	if ctx.cr[0].eq {
	pc = 0x82416E30; continue 'dispatch;
	}
	// 82416E1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416E20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82416E24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82416E28: 4E800421  bctrl
	ctx.lr = 0x82416E2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82416E2C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
            }
            0x82416E30 => {
    //   block [0x82416E30..0x82416E34)
	// 82416E30: 48018121  bl 0x8242ef50
	ctx.lr = 0x82416E34;
	sub_8242EF50(ctx, base);
	pc = 0x82416E34; continue 'dispatch;
            }
            0x82416E34 => {
    //   block [0x82416E34..0x82416E54)
	// 82416E34: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82416E38: 9BBF0002  stb r29, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82416E3C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82416E40: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82416E44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82416E48: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82416E4C: 4BFFFAB5  bl 0x82416900
	ctx.lr = 0x82416E50;
	sub_82416900(ctx, base);
	// 82416E50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82416E54; continue 'dispatch;
            }
            0x82416E54 => {
    //   block [0x82416E54..0x82416E5C)
	// 82416E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82416E58: 4811E2AC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416E60 size=240
    let mut pc: u32 = 0x82416E60;
    'dispatch: loop {
        match pc {
            0x82416E60 => {
    //   block [0x82416E60..0x82416EA8)
	// 82416E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416E74: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82416E78: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82416E7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82416E80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82416E84: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82416E88: 4BFFFA79  bl 0x82416900
	ctx.lr = 0x82416E8C;
	sub_82416900(ctx, base);
	// 82416E8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82416E90: 409A0018  bne cr6, 0x82416ea8
	if !ctx.cr[6].eq {
	pc = 0x82416EA8; continue 'dispatch;
	}
	// 82416E94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416E98: 386BF02C  addi r3, r11, -0xfd4
	ctx.r[3].s64 = ctx.r[11].s64 + -4052;
	// 82416E9C: 4800A3FD  bl 0x82421298
	ctx.lr = 0x82416EA0;
	sub_82421298(ctx, base);
	// 82416EA0: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82416EA4: 48000098  b 0x82416f3c
	pc = 0x82416F3C; continue 'dispatch;
            }
            0x82416EA8 => {
    //   block [0x82416EA8..0x82416ECC)
	// 82416EA8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82416EAC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82416EB0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82416EB4: 419A0084  beq cr6, 0x82416f38
	if ctx.cr[6].eq {
	pc = 0x82416F38; continue 'dispatch;
	}
	// 82416EB8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82416EBC: 409A0010  bne cr6, 0x82416ecc
	if !ctx.cr[6].eq {
	pc = 0x82416ECC; continue 'dispatch;
	}
	// 82416EC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82416EC4: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416EC8: 48000070  b 0x82416f38
	pc = 0x82416F38; continue 'dispatch;
            }
            0x82416ECC => {
    //   block [0x82416ECC..0x82416EEC)
	// 82416ECC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416ED0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416ED4: 40820018  bne 0x82416eec
	if !ctx.cr[0].eq {
	pc = 0x82416EEC; continue 'dispatch;
	}
	// 82416ED8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416EDC: 386BF000  addi r3, r11, -0x1000
	ctx.r[3].s64 = ctx.r[11].s64 + -4096;
	// 82416EE0: 4800A3B9  bl 0x82421298
	ctx.lr = 0x82416EE4;
	sub_82421298(ctx, base);
	// 82416EE4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82416EE8: 48000054  b 0x82416f3c
	pc = 0x82416F3C; continue 'dispatch;
            }
            0x82416EEC => {
    //   block [0x82416EEC..0x82416F38)
	// 82416EEC: 4800B655  bl 0x82422540
	ctx.lr = 0x82416EF0;
	sub_82422540(ctx, base);
	// 82416EF0: 48018061  bl 0x8242ef50
	ctx.lr = 0x82416EF4;
	sub_8242EF50(ctx, base);
	// 82416EF4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416EF8: 4800AA21  bl 0x82421918
	ctx.lr = 0x82416EFC;
	sub_82421918(ctx, base);
	// 82416EFC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416F00: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82416F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416F08: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82416F0C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82416F10: 4BFFFC21  bl 0x82416b30
	ctx.lr = 0x82416F14;
	sub_82416B30(ctx, base);
	// 82416F14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82416F18: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416F1C: 48018035  bl 0x8242ef50
	ctx.lr = 0x82416F20;
	sub_8242EF50(ctx, base);
	// 82416F20: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82416F24: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82416F28: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82416F2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82416F30: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82416F34: 4BFFF9CD  bl 0x82416900
	ctx.lr = 0x82416F38;
	sub_82416900(ctx, base);
	pc = 0x82416F38; continue 'dispatch;
            }
            0x82416F38 => {
    //   block [0x82416F38..0x82416F3C)
	// 82416F38: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	pc = 0x82416F3C; continue 'dispatch;
            }
            0x82416F3C => {
    //   block [0x82416F3C..0x82416F50)
	// 82416F3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416F48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416F50 size=320
    let mut pc: u32 = 0x82416F50;
    'dispatch: loop {
        match pc {
            0x82416F50 => {
    //   block [0x82416F50..0x82416F88)
	// 82416F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416F58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82416F5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416F68: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416F6C: 4800A915  bl 0x82421880
	ctx.lr = 0x82416F70;
	sub_82421880(ctx, base);
	// 82416F70: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82416F74: 409A0014  bne cr6, 0x82416f88
	if !ctx.cr[6].eq {
	pc = 0x82416F88; continue 'dispatch;
	}
	// 82416F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416F7C: 4BFFFBB5  bl 0x82416b30
	ctx.lr = 0x82416F80;
	sub_82416B30(ctx, base);
	// 82416F80: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82416F84: 480000F0  b 0x82417074
	pc = 0x82417074; continue 'dispatch;
            }
            0x82416F88 => {
    //   block [0x82416F88..0x82416FE0)
	// 82416F88: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82416F8C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82416F90: 409A009C  bne cr6, 0x8241702c
	if !ctx.cr[6].eq {
	pc = 0x8241702C; continue 'dispatch;
	}
	// 82416F94: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82416F98: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82416F9C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82416FA0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82416FA4: 409A003C  bne cr6, 0x82416fe0
	if !ctx.cr[6].eq {
	pc = 0x82416FE0; continue 'dispatch;
	}
	// 82416FA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416FAC: 4800A625  bl 0x824215d0
	ctx.lr = 0x82416FB0;
	sub_824215D0(ctx, base);
	// 82416FB0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82416FB4: 4182002C  beq 0x82416fe0
	if ctx.cr[0].eq {
	pc = 0x82416FE0; continue 'dispatch;
	}
	// 82416FB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416FBC: 4800B17D  bl 0x82422138
	ctx.lr = 0x82416FC0;
	sub_82422138(ctx, base);
	// 82416FC0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82416FC4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82416FC8: 41990018  bgt cr6, 0x82416fe0
	if ctx.cr[6].gt {
	pc = 0x82416FE0; continue 'dispatch;
	}
	// 82416FCC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82416FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416FD4: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416FD8: 4BFFFB59  bl 0x82416b30
	ctx.lr = 0x82416FDC;
	sub_82416B30(ctx, base);
	// 82416FDC: 4800009C  b 0x82417078
	pc = 0x82417078; continue 'dispatch;
            }
            0x82416FE0 => {
    //   block [0x82416FE0..0x8241701C)
	// 82416FE0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416FE4: 4800A89D  bl 0x82421880
	ctx.lr = 0x82416FE8;
	sub_82421880(ctx, base);
	// 82416FE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82416FEC: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416FF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416FF4: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416FF8: 4800A921  bl 0x82421918
	ctx.lr = 0x82416FFC;
	sub_82421918(ctx, base);
	// 82416FFC: 895F0001  lbz r10, 1(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82417000: 7D7E1850  subf r11, r30, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 82417004: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82417008: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8241700C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82417010: 419A000C  beq cr6, 0x8241701c
	if ctx.cr[6].eq {
	pc = 0x8241701C; continue 'dispatch;
	}
	// 82417014: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82417018: 409A0014  bne cr6, 0x8241702c
	if !ctx.cr[6].eq {
	pc = 0x8241702C; continue 'dispatch;
	}
	pc = 0x8241701C; continue 'dispatch;
            }
            0x8241701C => {
    //   block [0x8241701C..0x8241702C)
	// 8241701C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82417020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417024: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82417028: 4BFFFB09  bl 0x82416b30
	ctx.lr = 0x8241702C;
	sub_82416B30(ctx, base);
	pc = 0x8241702C; continue 'dispatch;
            }
            0x8241702C => {
    //   block [0x8241702C..0x82417074)
	// 8241702C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82417030: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82417034: 409A0044  bne cr6, 0x82417078
	if !ctx.cr[6].eq {
	pc = 0x82417078; continue 'dispatch;
	}
	// 82417038: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241703C: 4800A845  bl 0x82421880
	ctx.lr = 0x82417040;
	sub_82421880(ctx, base);
	// 82417040: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82417044: 409A0034  bne cr6, 0x82417078
	if !ctx.cr[6].eq {
	pc = 0x82417078; continue 'dispatch;
	}
	// 82417048: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241704C: 4800A8CD  bl 0x82421918
	ctx.lr = 0x82417050;
	sub_82421918(ctx, base);
	// 82417050: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82417054: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82417058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241705C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82417060: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82417064: 4BFFFACD  bl 0x82416b30
	ctx.lr = 0x82417068;
	sub_82416B30(ctx, base);
	// 82417068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241706C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82417070: 995F0003  stb r10, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	pc = 0x82417074; continue 'dispatch;
            }
            0x82417074 => {
    //   block [0x82417074..0x82417078)
	// 82417074: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	pc = 0x82417078; continue 'dispatch;
            }
            0x82417078 => {
    //   block [0x82417078..0x82417090)
	// 82417078: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241707C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417084: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82417088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241708C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417090 size=120
    let mut pc: u32 = 0x82417090;
    'dispatch: loop {
        match pc {
            0x82417090 => {
    //   block [0x82417090..0x824170B0)
	// 82417090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241709C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824170A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824170A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824170A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824170AC: 3BCBEECC  addi r30, r11, -0x1134
	ctx.r[30].s64 = ctx.r[11].s64 + -4404;
	pc = 0x824170B0; continue 'dispatch;
            }
            0x824170B0 => {
    //   block [0x824170B0..0x824170C4)
	// 824170B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824170B4: 409A0010  bne cr6, 0x824170c4
	if !ctx.cr[6].eq {
	pc = 0x824170C4; continue 'dispatch;
	}
	// 824170B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824170BC: 4800A1DD  bl 0x82421298
	ctx.lr = 0x824170C0;
	sub_82421298(ctx, base);
	// 824170C0: 48000014  b 0x824170d4
	pc = 0x824170D4; continue 'dispatch;
            }
            0x824170C4 => {
    //   block [0x824170C4..0x824170D4)
	// 824170C4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824170C8: 4800A509  bl 0x824215d0
	ctx.lr = 0x824170CC;
	sub_824215D0(ctx, base);
	// 824170CC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824170D0: 419A0020  beq cr6, 0x824170f0
	if ctx.cr[6].eq {
	pc = 0x824170F0; continue 'dispatch;
	}
	pc = 0x824170D4; continue 'dispatch;
            }
            0x824170D4 => {
    //   block [0x824170D4..0x824170F0)
	// 824170D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824170D8: 4800A539  bl 0x82421610
	ctx.lr = 0x824170DC;
	sub_82421610(ctx, base);
	// 824170DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824170E0: 41820010  beq 0x824170f0
	if ctx.cr[0].eq {
	pc = 0x824170F0; continue 'dispatch;
	}
	// 824170E4: 4800B5ED  bl 0x824226d0
	ctx.lr = 0x824170E8;
	sub_824226D0(ctx, base);
	// 824170E8: 48003BF1  bl 0x8241acd8
	ctx.lr = 0x824170EC;
	sub_8241ACD8(ctx, base);
	// 824170EC: 4BFFFFC4  b 0x824170b0
	pc = 0x824170B0; continue 'dispatch;
            }
            0x824170F0 => {
    //   block [0x824170F0..0x82417108)
	// 824170F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824170F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824170F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824170FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82417100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417108 size=116
    let mut pc: u32 = 0x82417108;
    'dispatch: loop {
        match pc {
            0x82417108 => {
    //   block [0x82417108..0x82417150)
	// 82417108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241710C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417114: 2B0300FF  cmplwi cr6, r3, 0xff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 255 as u32, &mut ctx.xer);
	// 82417118: 41990044  bgt cr6, 0x8241715c
	if ctx.cr[6].gt {
	pc = 0x8241715C; continue 'dispatch;
	}
	// 8241711C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417120: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82417124: 396BC580  addi r11, r11, -0x3a80
	ctx.r[11].s64 = ctx.r[11].s64 + -14976;
	// 82417128: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8241712C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82417130: 4182002C  beq 0x8241715c
	if ctx.cr[0].eq {
	pc = 0x8241715C; continue 'dispatch;
	}
	// 82417134: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82417138: 41980018  blt cr6, 0x82417150
	if ctx.cr[6].lt {
	pc = 0x82417150; continue 'dispatch;
	}
	// 8241713C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82417140: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82417144: 4098000C  bge cr6, 0x82417150
	if !ctx.cr[6].lt {
	pc = 0x82417150; continue 'dispatch;
	}
	// 82417148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241714C: 48000020  b 0x8241716c
	pc = 0x8241716C; continue 'dispatch;
            }
            0x82417150 => {
    //   block [0x82417150..0x8241715C)
	// 82417150: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417154: 386BF0F0  addi r3, r11, -0xf10
	ctx.r[3].s64 = ctx.r[11].s64 + -3856;
	// 82417158: 4800000C  b 0x82417164
	pc = 0x82417164; continue 'dispatch;
            }
            0x8241715C => {
    //   block [0x8241715C..0x82417164)
	// 8241715C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417160: 386BF0CC  addi r3, r11, -0xf34
	ctx.r[3].s64 = ctx.r[11].s64 + -3892;
	pc = 0x82417164; continue 'dispatch;
            }
            0x82417164 => {
    //   block [0x82417164..0x8241716C)
	// 82417164: 4800A135  bl 0x82421298
	ctx.lr = 0x82417168;
	sub_82421298(ctx, base);
	// 82417168: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	pc = 0x8241716C; continue 'dispatch;
            }
            0x8241716C => {
    //   block [0x8241716C..0x8241717C)
	// 8241716C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82417170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


