pub fn sub_82ED0FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED0FE0 size=2192
    let mut pc: u32 = 0x82ED0FE0;
    'dispatch: loop {
        match pc {
            0x82ED0FE0 => {
    //   block [0x82ED0FE0..0x82ED1870)
	// 82ED0FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED0FE4: 482D7189  bl 0x831a816c
	ctx.lr = 0x82ED0FE8;
	sub_831A8130(ctx, base);
	// 82ED0FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED0FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED0FF0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED0FF4: 394B45A8  addi r10, r11, 0x45a8
	ctx.r[10].s64 = ctx.r[11].s64 + 17832;
	// 82ED0FF8: 80BF0068  lwz r5, 0x68(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED0FFC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED1000: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82ED1004: 419A004C  beq cr6, 0x82ed1050
	if ctx.cr[6].eq {
	pc = 0x82ED1050; continue 'dispatch;
	}
	// 82ED1008: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED100C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED1010: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED1014: 81230054  lwz r9, 0x54(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED1018: 81030034  lwz r8, 0x34(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED101C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED1020: 41980014  blt cr6, 0x82ed1034
	if ctx.cr[6].lt {
	pc = 0x82ED1034; continue 'dispatch;
	}
	// 82ED1024: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82ED1028: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82ED102C: 4BFCF035  bl 0x82ea0060
	ctx.lr = 0x82ED1030;
	sub_82EA0060(ctx, base);
	// 82ED1030: 48000020  b 0x82ed1050
	pc = 0x82ED1050; continue 'dispatch;
	// 82ED1034: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED1038: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82ED103C: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED1040: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED1044: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82ED1048: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82ED104C: 90A30050  stw r5, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82ED1050: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED1054: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1058: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED105C: 419A0030  beq cr6, 0x82ed108c
	if ctx.cr[6].eq {
	pc = 0x82ED108C; continue 'dispatch;
	}
	// 82ED1060: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED1064: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED1068: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED106C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED1070: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED1074: 409A0018  bne cr6, 0x82ed108c
	if !ctx.cr[6].eq {
	pc = 0x82ED108C; continue 'dispatch;
	}
	// 82ED1078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED107C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED1080: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1084: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1088: 4E800421  bctrl
	ctx.lr = 0x82ED108C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED108C: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82ED1090: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED1094: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82ED1098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED109C: 419A0010  beq cr6, 0x82ed10ac
	if ctx.cr[6].eq {
	pc = 0x82ED10AC; continue 'dispatch;
	}
	// 82ED10A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED10A4: 4BFF6B0D  bl 0x82ec7bb0
	ctx.lr = 0x82ED10A8;
	sub_82EC7BB0(ctx, base);
	// 82ED10A8: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82ED10AC: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82ED10B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED10B4: 419A0010  beq cr6, 0x82ed10c4
	if ctx.cr[6].eq {
	pc = 0x82ED10C4; continue 'dispatch;
	}
	// 82ED10B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED10BC: 4BFF6AF5  bl 0x82ec7bb0
	ctx.lr = 0x82ED10C0;
	sub_82EC7BB0(ctx, base);
	// 82ED10C0: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82ED10C4: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82ED10C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED10CC: 419A0010  beq cr6, 0x82ed10dc
	if ctx.cr[6].eq {
	pc = 0x82ED10DC; continue 'dispatch;
	}
	// 82ED10D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED10D4: 4BFF6ADD  bl 0x82ec7bb0
	ctx.lr = 0x82ED10D8;
	sub_82EC7BB0(ctx, base);
	// 82ED10D8: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82ED10DC: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82ED10E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED10E4: 419A0010  beq cr6, 0x82ed10f4
	if ctx.cr[6].eq {
	pc = 0x82ED10F4; continue 'dispatch;
	}
	// 82ED10E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED10EC: 4BFF6AC5  bl 0x82ec7bb0
	ctx.lr = 0x82ED10F0;
	sub_82EC7BB0(ctx, base);
	// 82ED10F0: 93DF00B0  stw r30, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 82ED10F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED10F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED10FC: 419A0040  beq cr6, 0x82ed113c
	if ctx.cr[6].eq {
	pc = 0x82ED113C; continue 'dispatch;
	}
	// 82ED1100: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1104: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED1108: 419A0030  beq cr6, 0x82ed1138
	if ctx.cr[6].eq {
	pc = 0x82ED1138; continue 'dispatch;
	}
	// 82ED110C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED1110: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED1114: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED1118: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED111C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED1120: 409A0018  bne cr6, 0x82ed1138
	if !ctx.cr[6].eq {
	pc = 0x82ED1138; continue 'dispatch;
	}
	// 82ED1124: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1128: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED112C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1130: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1134: 4E800421  bctrl
	ctx.lr = 0x82ED1138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1138: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82ED113C: 817F00E8  lwz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ED1140: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 82ED1144: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1148: 40990020  ble cr6, 0x82ed1168
	if !ctx.cr[6].gt {
	pc = 0x82ED1168; continue 'dispatch;
	}
	// 82ED114C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ED1150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED1154: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1158: 4BFFC419  bl 0x82ecd570
	ctx.lr = 0x82ED115C;
	sub_82ECD570(ctx, base);
	// 82ED115C: 815F00E8  lwz r10, 0xe8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ED1160: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1164: 4199FFE8  bgt cr6, 0x82ed114c
	if ctx.cr[6].gt {
	pc = 0x82ED114C; continue 'dispatch;
	}
	// 82ED1168: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED116C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1170: 40990038  ble cr6, 0x82ed11a8
	if !ctx.cr[6].gt {
	pc = 0x82ED11A8; continue 'dispatch;
	}
	// 82ED1174: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1178: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED117C: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED1180: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1184: 419A0024  beq cr6, 0x82ed11a8
	if ctx.cr[6].eq {
	pc = 0x82ED11A8; continue 'dispatch;
	}
	// 82ED1188: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED118C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED1190: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED1194: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1198: 4BFFC2A9  bl 0x82ecd440
	ctx.lr = 0x82ED119C;
	sub_82ECD440(ctx, base);
	// 82ED119C: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED11A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED11A4: 4199FFD0  bgt cr6, 0x82ed1174
	if ctx.cr[6].gt {
	pc = 0x82ED1174; continue 'dispatch;
	}
	// 82ED11A8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ED11AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED11B0: 4099002C  ble cr6, 0x82ed11dc
	if !ctx.cr[6].gt {
	pc = 0x82ED11DC; continue 'dispatch;
	}
	// 82ED11B4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED11B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED11BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED11C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED11C4: 812A0048  lwz r9, 0x48(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED11C8: 80A90000  lwz r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED11CC: 4BFFC275  bl 0x82ecd440
	ctx.lr = 0x82ED11D0;
	sub_82ECD440(ctx, base);
	// 82ED11D0: 811F0038  lwz r8, 0x38(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ED11D4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED11D8: 4199FFDC  bgt cr6, 0x82ed11b4
	if ctx.cr[6].gt {
	pc = 0x82ED11B4; continue 'dispatch;
	}
	// 82ED11DC: 897F00B8  lbz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED11E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED11E4: 409A005C  bne cr6, 0x82ed1240
	if !ctx.cr[6].eq {
	pc = 0x82ED1240; continue 'dispatch;
	}
	// 82ED11E8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED11EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED11F0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED11F4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED11F8: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED11FC: 8089FFFC  lwz r4, -4(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82ED1200: 4800DCA9  bl 0x82edeea8
	ctx.lr = 0x82ED1204;
	sub_82EDEEA8(ctx, base);
	// 82ED1204: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1208: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED120C: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED1210: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED1214: 8067FFFC  lwz r3, -4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82ED1218: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED121C: 419A0018  beq cr6, 0x82ed1234
	if ctx.cr[6].eq {
	pc = 0x82ED1234; continue 'dispatch;
	}
	// 82ED1220: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1224: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED1228: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED122C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1230: 4E800421  bctrl
	ctx.lr = 0x82ED1234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1234: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1238: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82ED123C: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82ED1240: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED1244: 80BF0024  lwz r5, 0x24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ED1248: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED124C: 4BFFC1F5  bl 0x82ecd440
	ctx.lr = 0x82ED1250;
	sub_82ECD440(ctx, base);
	// 82ED1250: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED1254: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82ED1258: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED125C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1260: 4099002C  ble cr6, 0x82ed128c
	if !ctx.cr[6].gt {
	pc = 0x82ED128C; continue 'dispatch;
	}
	// 82ED1264: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED1268: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED126C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED1270: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED1274: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1278: 4BFFC1C9  bl 0x82ecd440
	ctx.lr = 0x82ED127C;
	sub_82ECD440(ctx, base);
	// 82ED127C: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED1280: 8109004C  lwz r8, 0x4c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED1284: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED1288: 4199FFDC  bgt cr6, 0x82ed1264
	if ctx.cr[6].gt {
	pc = 0x82ED1264; continue 'dispatch;
	}
	// 82ED128C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED1290: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED1294: 4800DC15  bl 0x82edeea8
	ctx.lr = 0x82ED1298;
	sub_82EDEEA8(ctx, base);
	// 82ED1298: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED129C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED12A0: 419A0018  beq cr6, 0x82ed12b8
	if ctx.cr[6].eq {
	pc = 0x82ED12B8; continue 'dispatch;
	}
	// 82ED12A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED12A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED12AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED12B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED12B4: 4E800421  bctrl
	ctx.lr = 0x82ED12B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED12B8: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82ED12BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED12C0: 48013201  bl 0x82ee44c0
	ctx.lr = 0x82ED12C4;
	sub_82EE44C0(ctx, base);
	// 82ED12C4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED12C8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED12CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED12D0: 419A0030  beq cr6, 0x82ed1300
	if ctx.cr[6].eq {
	pc = 0x82ED1300; continue 'dispatch;
	}
	// 82ED12D4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED12D8: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED12DC: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED12E0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED12E4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED12E8: 409A0018  bne cr6, 0x82ed1300
	if !ctx.cr[6].eq {
	pc = 0x82ED1300; continue 'dispatch;
	}
	// 82ED12EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED12F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED12F4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED12F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED12FC: 4E800421  bctrl
	ctx.lr = 0x82ED1300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1300: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED1304: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ED1308: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED130C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED1310: 419A0030  beq cr6, 0x82ed1340
	if ctx.cr[6].eq {
	pc = 0x82ED1340; continue 'dispatch;
	}
	// 82ED1314: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED1318: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED131C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED1320: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED1324: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED1328: 409A0018  bne cr6, 0x82ed1340
	if !ctx.cr[6].eq {
	pc = 0x82ED1340; continue 'dispatch;
	}
	// 82ED132C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1330: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED1334: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1338: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED133C: 4E800421  bctrl
	ctx.lr = 0x82ED1340;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1340: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED1344: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82ED1348: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED134C: 419A000C  beq cr6, 0x82ed1358
	if ctx.cr[6].eq {
	pc = 0x82ED1358; continue 'dispatch;
	}
	// 82ED1350: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED1354: 48003505  bl 0x82ed4858
	ctx.lr = 0x82ED1358;
	sub_82ED4858(ctx, base);
	// 82ED1358: 807F018C  lwz r3, 0x18c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(396 as u32) ) } as u64;
	// 82ED135C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED1360: 419A003C  beq cr6, 0x82ed139c
	if ctx.cr[6].eq {
	pc = 0x82ED139C; continue 'dispatch;
	}
	// 82ED1364: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1368: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED136C: 419A0030  beq cr6, 0x82ed139c
	if ctx.cr[6].eq {
	pc = 0x82ED139C; continue 'dispatch;
	}
	// 82ED1370: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED1374: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED1378: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED137C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED1380: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED1384: 409A0018  bne cr6, 0x82ed139c
	if !ctx.cr[6].eq {
	pc = 0x82ED139C; continue 'dispatch;
	}
	// 82ED1388: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED138C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED1390: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1394: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1398: 4E800421  bctrl
	ctx.lr = 0x82ED139C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED139C: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ED13A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED13A4: 419A0018  beq cr6, 0x82ed13bc
	if ctx.cr[6].eq {
	pc = 0x82ED13BC; continue 'dispatch;
	}
	// 82ED13A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED13AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED13B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED13B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED13B8: 4E800421  bctrl
	ctx.lr = 0x82ED13BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED13BC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED13C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED13C4: 419A0018  beq cr6, 0x82ed13dc
	if ctx.cr[6].eq {
	pc = 0x82ED13DC; continue 'dispatch;
	}
	// 82ED13C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED13CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED13D0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED13D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED13D8: 4E800421  bctrl
	ctx.lr = 0x82ED13DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED13DC: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED13E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED13E4: 419A0018  beq cr6, 0x82ed13fc
	if ctx.cr[6].eq {
	pc = 0x82ED13FC; continue 'dispatch;
	}
	// 82ED13E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED13EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED13F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED13F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED13F8: 4E800421  bctrl
	ctx.lr = 0x82ED13FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED13FC: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1400: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1404: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED1408: 419A0030  beq cr6, 0x82ed1438
	if ctx.cr[6].eq {
	pc = 0x82ED1438; continue 'dispatch;
	}
	// 82ED140C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED1410: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED1414: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED1418: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED141C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED1420: 409A0018  bne cr6, 0x82ed1438
	if !ctx.cr[6].eq {
	pc = 0x82ED1438; continue 'dispatch;
	}
	// 82ED1424: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1428: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED142C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1430: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1434: 4E800421  bctrl
	ctx.lr = 0x82ED1438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1438: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED143C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED1444: 419A0030  beq cr6, 0x82ed1474
	if ctx.cr[6].eq {
	pc = 0x82ED1474; continue 'dispatch;
	}
	// 82ED1448: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED144C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED1450: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED1454: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED1458: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED145C: 409A0018  bne cr6, 0x82ed1474
	if !ctx.cr[6].eq {
	pc = 0x82ED1474; continue 'dispatch;
	}
	// 82ED1460: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1464: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED1468: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED146C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1470: 4E800421  bctrl
	ctx.lr = 0x82ED1474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1474: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED1478: 80AB006C  lwz r5, 0x6c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED147C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82ED1480: 419A004C  beq cr6, 0x82ed14cc
	if ctx.cr[6].eq {
	pc = 0x82ED14CC; continue 'dispatch;
	}
	// 82ED1484: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1488: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED148C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED1490: 8123004C  lwz r9, 0x4c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED1494: 81030034  lwz r8, 0x34(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED1498: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED149C: 41980014  blt cr6, 0x82ed14b0
	if ctx.cr[6].lt {
	pc = 0x82ED14B0; continue 'dispatch;
	}
	// 82ED14A0: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82ED14A4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82ED14A8: 4BFCEBB9  bl 0x82ea0060
	ctx.lr = 0x82ED14AC;
	sub_82EA0060(ctx, base);
	// 82ED14AC: 48000020  b 0x82ed14cc
	pc = 0x82ED14CC; continue 'dispatch;
	// 82ED14B0: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED14B4: 39630048  addi r11, r3, 0x48
	ctx.r[11].s64 = ctx.r[3].s64 + 72;
	// 82ED14B8: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED14BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED14C0: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82ED14C4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82ED14C8: 90A30048  stw r5, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[5].u32 ) };
	// 82ED14CC: 80BF006C  lwz r5, 0x6c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED14D0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82ED14D4: 419A004C  beq cr6, 0x82ed1520
	if ctx.cr[6].eq {
	pc = 0x82ED1520; continue 'dispatch;
	}
	// 82ED14D8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED14DC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED14E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED14E4: 81230074  lwz r9, 0x74(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED14E8: 81030034  lwz r8, 0x34(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED14EC: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED14F0: 41980014  blt cr6, 0x82ed1504
	if ctx.cr[6].lt {
	pc = 0x82ED1504; continue 'dispatch;
	}
	// 82ED14F4: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82ED14F8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82ED14FC: 4BFCEB65  bl 0x82ea0060
	ctx.lr = 0x82ED1500;
	sub_82EA0060(ctx, base);
	// 82ED1500: 48000020  b 0x82ed1520
	pc = 0x82ED1520; continue 'dispatch;
	// 82ED1504: 81430074  lwz r10, 0x74(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED1508: 39630070  addi r11, r3, 0x70
	ctx.r[11].s64 = ctx.r[3].s64 + 112;
	// 82ED150C: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1510: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED1514: 91430074  stw r10, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82ED1518: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82ED151C: 90A30070  stw r5, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[5].u32 ) };
	// 82ED1520: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED1524: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1528: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED152C: 419A0030  beq cr6, 0x82ed155c
	if ctx.cr[6].eq {
	pc = 0x82ED155C; continue 'dispatch;
	}
	// 82ED1530: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED1534: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED1538: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED153C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED1540: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED1544: 409A0018  bne cr6, 0x82ed155c
	if !ctx.cr[6].eq {
	pc = 0x82ED155C; continue 'dispatch;
	}
	// 82ED1548: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED154C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED1550: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1554: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1558: 4E800421  bctrl
	ctx.lr = 0x82ED155C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED155C: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED1560: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED1564: 419A000C  beq cr6, 0x82ed1570
	if ctx.cr[6].eq {
	pc = 0x82ED1570; continue 'dispatch;
	}
	// 82ED1568: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED156C: 4800338D  bl 0x82ed48f8
	ctx.lr = 0x82ED1570;
	sub_82ED48F8(ctx, base);
	// 82ED1570: 817F0188  lwz r11, 0x188(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) } as u64;
	// 82ED1574: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1578: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED157C: 409A0020  bne cr6, 0x82ed159c
	if !ctx.cr[6].eq {
	pc = 0x82ED159C; continue 'dispatch;
	}
	// 82ED1580: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1584: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED1588: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED158C: 809F0180  lwz r4, 0x180(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(384 as u32) ) } as u64;
	// 82ED1590: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1594: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1598: 4BFCF219  bl 0x82ea07b0
	ctx.lr = 0x82ED159C;
	sub_82EA07B0(ctx, base);
	// 82ED159C: 817F017C  lwz r11, 0x17c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(380 as u32) ) } as u64;
	// 82ED15A0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED15A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED15A8: 409A0020  bne cr6, 0x82ed15c8
	if !ctx.cr[6].eq {
	pc = 0x82ED15C8; continue 'dispatch;
	}
	// 82ED15AC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED15B0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED15B4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED15B8: 809F0174  lwz r4, 0x174(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 82ED15BC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED15C0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED15C4: 4BFCF1ED  bl 0x82ea07b0
	ctx.lr = 0x82ED15C8;
	sub_82EA07B0(ctx, base);
	// 82ED15C8: 817F0170  lwz r11, 0x170(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 82ED15CC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED15D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED15D4: 409A0020  bne cr6, 0x82ed15f4
	if !ctx.cr[6].eq {
	pc = 0x82ED15F4; continue 'dispatch;
	}
	// 82ED15D8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED15DC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED15E0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED15E4: 809F0168  lwz r4, 0x168(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 82ED15E8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED15EC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED15F0: 4BFCF1C1  bl 0x82ea07b0
	ctx.lr = 0x82ED15F4;
	sub_82EA07B0(ctx, base);
	// 82ED15F4: 817F0164  lwz r11, 0x164(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 82ED15F8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED15FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1600: 409A0020  bne cr6, 0x82ed1620
	if !ctx.cr[6].eq {
	pc = 0x82ED1620; continue 'dispatch;
	}
	// 82ED1604: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1608: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED160C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1610: 809F015C  lwz r4, 0x15c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 82ED1614: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1618: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED161C: 4BFCF195  bl 0x82ea07b0
	ctx.lr = 0x82ED1620;
	sub_82EA07B0(ctx, base);
	// 82ED1620: 817F0158  lwz r11, 0x158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 82ED1624: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1628: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED162C: 409A0020  bne cr6, 0x82ed164c
	if !ctx.cr[6].eq {
	pc = 0x82ED164C; continue 'dispatch;
	}
	// 82ED1630: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1634: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED1638: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED163C: 809F0150  lwz r4, 0x150(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED1640: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1644: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1648: 4BFCF169  bl 0x82ea07b0
	ctx.lr = 0x82ED164C;
	sub_82EA07B0(ctx, base);
	// 82ED164C: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82ED1650: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1654: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1658: 409A0020  bne cr6, 0x82ed1678
	if !ctx.cr[6].eq {
	pc = 0x82ED1678; continue 'dispatch;
	}
	// 82ED165C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1660: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED1664: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1668: 809F0144  lwz r4, 0x144(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 82ED166C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1670: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1674: 4BFCF13D  bl 0x82ea07b0
	ctx.lr = 0x82ED1678;
	sub_82EA07B0(ctx, base);
	// 82ED1678: 817F0140  lwz r11, 0x140(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 82ED167C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1680: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1684: 409A0020  bne cr6, 0x82ed16a4
	if !ctx.cr[6].eq {
	pc = 0x82ED16A4; continue 'dispatch;
	}
	// 82ED1688: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED168C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED1690: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1694: 809F0138  lwz r4, 0x138(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 82ED1698: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED169C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED16A0: 4BFCF111  bl 0x82ea07b0
	ctx.lr = 0x82ED16A4;
	sub_82EA07B0(ctx, base);
	// 82ED16A4: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82ED16A8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED16AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED16B0: 409A0020  bne cr6, 0x82ed16d0
	if !ctx.cr[6].eq {
	pc = 0x82ED16D0; continue 'dispatch;
	}
	// 82ED16B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED16B8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED16BC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED16C0: 809F012C  lwz r4, 0x12c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 82ED16C4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED16C8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED16CC: 4BFCF0E5  bl 0x82ea07b0
	ctx.lr = 0x82ED16D0;
	sub_82EA07B0(ctx, base);
	// 82ED16D0: 817F0128  lwz r11, 0x128(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 82ED16D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED16D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED16DC: 409A0020  bne cr6, 0x82ed16fc
	if !ctx.cr[6].eq {
	pc = 0x82ED16FC; continue 'dispatch;
	}
	// 82ED16E0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED16E4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED16E8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED16EC: 809F0120  lwz r4, 0x120(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 82ED16F0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED16F4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED16F8: 4BFCF0B9  bl 0x82ea07b0
	ctx.lr = 0x82ED16FC;
	sub_82EA07B0(ctx, base);
	// 82ED16FC: 817F011C  lwz r11, 0x11c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 82ED1700: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1704: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1708: 409A0020  bne cr6, 0x82ed1728
	if !ctx.cr[6].eq {
	pc = 0x82ED1728; continue 'dispatch;
	}
	// 82ED170C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1710: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED1714: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1718: 809F0114  lwz r4, 0x114(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 82ED171C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1720: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1724: 4BFCF08D  bl 0x82ea07b0
	ctx.lr = 0x82ED1728;
	sub_82EA07B0(ctx, base);
	// 82ED1728: 817F0110  lwz r11, 0x110(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 82ED172C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1730: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1734: 409A0020  bne cr6, 0x82ed1754
	if !ctx.cr[6].eq {
	pc = 0x82ED1754; continue 'dispatch;
	}
	// 82ED1738: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED173C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED1740: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1744: 809F0108  lwz r4, 0x108(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 82ED1748: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED174C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1750: 4BFCF061  bl 0x82ea07b0
	ctx.lr = 0x82ED1754;
	sub_82EA07B0(ctx, base);
	// 82ED1754: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 82ED1758: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED175C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1760: 409A0020  bne cr6, 0x82ed1780
	if !ctx.cr[6].eq {
	pc = 0x82ED1780; continue 'dispatch;
	}
	// 82ED1764: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1768: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED176C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1770: 809F00FC  lwz r4, 0xfc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82ED1774: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1778: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED177C: 4BFCF035  bl 0x82ea07b0
	ctx.lr = 0x82ED1780;
	sub_82EA07B0(ctx, base);
	// 82ED1780: 817F00F8  lwz r11, 0xf8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 82ED1784: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1788: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED178C: 409A0020  bne cr6, 0x82ed17ac
	if !ctx.cr[6].eq {
	pc = 0x82ED17AC; continue 'dispatch;
	}
	// 82ED1790: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1794: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED1798: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED179C: 809F00F0  lwz r4, 0xf0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 82ED17A0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED17A4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED17A8: 4BFCF009  bl 0x82ea07b0
	ctx.lr = 0x82ED17AC;
	sub_82EA07B0(ctx, base);
	// 82ED17AC: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82ED17B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED17B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED17B8: 409A0020  bne cr6, 0x82ed17d8
	if !ctx.cr[6].eq {
	pc = 0x82ED17D8; continue 'dispatch;
	}
	// 82ED17BC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED17C0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED17C4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED17C8: 809F00E4  lwz r4, 0xe4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ED17CC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED17D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED17D4: 4BFCEFDD  bl 0x82ea07b0
	ctx.lr = 0x82ED17D8;
	sub_82EA07B0(ctx, base);
	// 82ED17D8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED17DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED17E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED17E4: 409A0020  bne cr6, 0x82ed1804
	if !ctx.cr[6].eq {
	pc = 0x82ED1804; continue 'dispatch;
	}
	// 82ED17E8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED17EC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED17F0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED17F4: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ED17F8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED17FC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1800: 4BFCEFB1  bl 0x82ea07b0
	ctx.lr = 0x82ED1804;
	sub_82EA07B0(ctx, base);
	// 82ED1804: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82ED1808: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED180C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1810: 409A0020  bne cr6, 0x82ed1830
	if !ctx.cr[6].eq {
	pc = 0x82ED1830; continue 'dispatch;
	}
	// 82ED1814: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1818: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED181C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1820: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED1824: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1828: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED182C: 4BFCEF85  bl 0x82ea07b0
	ctx.lr = 0x82ED1830;
	sub_82EA07B0(ctx, base);
	// 82ED1830: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED1834: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1838: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED183C: 409A0020  bne cr6, 0x82ed185c
	if !ctx.cr[6].eq {
	pc = 0x82ED185C; continue 'dispatch;
	}
	// 82ED1840: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1844: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED1848: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED184C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1850: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1854: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1858: 4BFCEF59  bl 0x82ea07b0
	ctx.lr = 0x82ED185C;
	sub_82EA07B0(ctx, base);
	// 82ED185C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED1860: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82ED1864: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED1868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED186C: 482D6950  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED1870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED1870 size=680
    let mut pc: u32 = 0x82ED1870;
    'dispatch: loop {
        match pc {
            0x82ED1870 => {
    //   block [0x82ED1870..0x82ED1B18)
	// 82ED1870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED1874: 482D68DD  bl 0x831a8150
	ctx.lr = 0x82ED1878;
	sub_831A8130(ctx, base);
	// 82ED1878: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED187C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED1880: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82ED1884: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82ED1888: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED188C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1890: 419A0028  beq cr6, 0x82ed18b8
	if ctx.cr[6].eq {
	pc = 0x82ED18B8; continue 'dispatch;
	}
	// 82ED1894: 39600013  li r11, 0x13
	ctx.r[11].s64 = 19;
	// 82ED1898: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 82ED189C: 9AC10058  stb r22, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[22].u8 ) };
	// 82ED18A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED18A4: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED18A8: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED18AC: 48015F5D  bl 0x82ee7808
	ctx.lr = 0x82ED18B0;
	sub_82EE7808(ctx, base);
	// 82ED18B0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82ED18B4: 482D68EC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 82ED18B8: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED18BC: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82ED18C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ED18C4: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED18C8: 7D5BC02E  lwzx r10, r27, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED18CC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED18D0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED18D4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED18D8: 4098002C  bge cr6, 0x82ed1904
	if !ctx.cr[6].lt {
	pc = 0x82ED1904; continue 'dispatch;
	}
	// 82ED18DC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED18E0: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED18E4: 38E945EC  addi r7, r9, 0x45ec
	ctx.r[7].s64 = ctx.r[9].s64 + 17900;
	// 82ED18E8: 38C845DC  addi r6, r8, 0x45dc
	ctx.r[6].s64 = ctx.r[8].s64 + 17884;
	// 82ED18EC: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED18F0: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82ED18F4: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82ED18F8: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82ED18FC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED1900: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82ED1904: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 82ED1908: 83FE02F8  lwz r31, 0x2f8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(760 as u32) ) } as u64;
	// 82ED190C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82ED1910: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 82ED1914: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82ED1918: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82ED191C: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82ED1920: 3BBA0010  addi r29, r26, 0x10
	ctx.r[29].s64 = ctx.r[26].s64 + 16;
	// 82ED1924: 7C7BB82E  lwzx r3, r27, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED1928: 55641836  rlwinm r4, r11, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82ED192C: 93810068  stw r28, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 82ED1930: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED1934: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED1938: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED193C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1940: 4199000C  bgt cr6, 0x82ed194c
	if ctx.cr[6].gt {
	pc = 0x82ED194C; continue 'dispatch;
	}
	// 82ED1944: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED1948: 48000018  b 0x82ed1960
	pc = 0x82ED1960; continue 'dispatch;
	// 82ED194C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1950: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED1954: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1958: 4E800421  bctrl
	ctx.lr = 0x82ED195C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED195C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED1960: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82ED1964: 7FEAE378  or r10, r31, r28
	ctx.r[10].u64 = ctx.r[31].u64 | ctx.r[28].u64;
	// 82ED1968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82ED196C: 3BBD0014  addi r29, r29, 0x14
	ctx.r[29].s64 = ctx.r[29].s64 + 20;
	// 82ED1970: 807E0054  lwz r3, 0x54(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED1974: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82ED1978: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82ED197C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED1980: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1984: 81090044  lwz r8, 0x44(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(68 as u32) ) } as u64;
	// 82ED1988: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ED198C: 4E800421  bctrl
	ctx.lr = 0x82ED1990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1990: 7D7BC02E  lwzx r11, r27, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED1994: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1998: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED199C: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82ED19A0: 40980020  bge cr6, 0x82ed19c0
	if !ctx.cr[6].lt {
	pc = 0x82ED19C0; continue 'dispatch;
	}
	// 82ED19A4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED19A8: 390945C8  addi r8, r9, 0x45c8
	ctx.r[8].s64 = ctx.r[9].s64 + 17864;
	// 82ED19AC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED19B0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED19B4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82ED19B8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED19BC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED19C0: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED19C4: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82ED19C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED19CC: 40990048  ble cr6, 0x82ed1a14
	if !ctx.cr[6].gt {
	pc = 0x82ED1A14; continue 'dispatch;
	}
	// 82ED19D0: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82ED19D4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED19D8: 7D5F5A14  add r10, r31, r11
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82ED19DC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED19E0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82ED19E4: 419A001C  beq cr6, 0x82ed1a00
	if ctx.cr[6].eq {
	pc = 0x82ED1A00; continue 'dispatch;
	}
	// 82ED19E8: 894B0005  lbz r10, 5(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82ED19EC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ED19F0: 80BE0070  lwz r5, 0x70(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED19F4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82ED19F8: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED19FC: 4BFFB185  bl 0x82eccb80
	ctx.lr = 0x82ED1A00;
	sub_82ECCB80(ctx, base);
	// 82ED1A00: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED1A04: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82ED1A08: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82ED1A0C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED1A10: 4198FFC4  blt cr6, 0x82ed19d4
	if ctx.cr[6].lt {
	pc = 0x82ED19D4; continue 'dispatch;
	}
	// 82ED1A14: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82ED1A18: 419A0048  beq cr6, 0x82ed1a60
	if ctx.cr[6].eq {
	pc = 0x82ED1A60; continue 'dispatch;
	}
	// 82ED1A1C: 7D5BC02E  lwzx r10, r27, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED1A20: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED1A24: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1A28: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1A2C: 40980020  bge cr6, 0x82ed1a4c
	if !ctx.cr[6].lt {
	pc = 0x82ED1A4C; continue 'dispatch;
	}
	// 82ED1A30: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED1A34: 390945B4  addi r8, r9, 0x45b4
	ctx.r[8].s64 = ctx.r[9].s64 + 17844;
	// 82ED1A38: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED1A3C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED1A40: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED1A44: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED1A48: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED1A4C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1A50: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ED1A54: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED1A58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1A5C: 4E800421  bctrl
	ctx.lr = 0x82ED1A60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1A60: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED1A64: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1A68: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED1A6C: 40820028  bne 0x82ed1a94
	if !ctx.cr[0].eq {
	pc = 0x82ED1A94; continue 'dispatch;
	}
	// 82ED1A70: 897E008C  lbz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED1A74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED1A78: 409A001C  bne cr6, 0x82ed1a94
	if !ctx.cr[6].eq {
	pc = 0x82ED1A94; continue 'dispatch;
	}
	// 82ED1A7C: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED1A80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1A84: 419A0010  beq cr6, 0x82ed1a94
	if ctx.cr[6].eq {
	pc = 0x82ED1A94; continue 'dispatch;
	}
	// 82ED1A88: 933E0080  stw r25, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82ED1A8C: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED1A90: 48016369  bl 0x82ee7df8
	ctx.lr = 0x82ED1A94;
	sub_82EE7DF8(ctx, base);
	// 82ED1A94: 7D5BC02E  lwzx r10, r27, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED1A98: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED1A9C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1AA0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1AA4: 40980020  bge cr6, 0x82ed1ac4
	if !ctx.cr[6].lt {
	pc = 0x82ED1AC4; continue 'dispatch;
	}
	// 82ED1AA8: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82ED1AAC: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82ED1AB0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED1AB4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED1AB8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED1ABC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED1AC0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED1AC4: 7C7BB82E  lwzx r3, r27, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED1AC8: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED1ACC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED1AD0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED1AD4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED1AD8: 409A0014  bne cr6, 0x82ed1aec
	if !ctx.cr[6].eq {
	pc = 0x82ED1AEC; continue 'dispatch;
	}
	// 82ED1ADC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1AE0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED1AE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1AE8: 4E800421  bctrl
	ctx.lr = 0x82ED1AEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1AEC: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED1AF0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1AF4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1AF8: 409A0018  bne cr6, 0x82ed1b10
	if !ctx.cr[6].eq {
	pc = 0x82ED1B10; continue 'dispatch;
	}
	// 82ED1AFC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1B00: 7C7BB82E  lwzx r3, r27, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED1B04: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1B08: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED1B0C: 4BFCECA5  bl 0x82ea07b0
	ctx.lr = 0x82ED1B10;
	sub_82EA07B0(ctx, base);
	// 82ED1B10: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82ED1B14: 482D668C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED1B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED1B18 size=1264
    let mut pc: u32 = 0x82ED1B18;
    'dispatch: loop {
        match pc {
            0x82ED1B18 => {
    //   block [0x82ED1B18..0x82ED2008)
	// 82ED1B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED1B1C: 482D6629  bl 0x831a8144
	ctx.lr = 0x82ED1B20;
	sub_831A8130(ctx, base);
	// 82ED1B20: 9421F700  stwu r1, -0x900(r1)
	ea = ctx.r[1].u32.wrapping_add(-2304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED1B24: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82ED1B28: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82ED1B2C: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82ED1B30: 81780084  lwz r11, 0x84(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED1B34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1B38: 419A002C  beq cr6, 0x82ed1b64
	if ctx.cr[6].eq {
	pc = 0x82ED1B64; continue 'dispatch;
	}
	// 82ED1B3C: 39400012  li r10, 0x12
	ctx.r[10].s64 = 18;
	// 82ED1B40: 92E1005C  stw r23, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[23].u32 ) };
	// 82ED1B44: 98A10060  stb r5, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u8 ) };
	// 82ED1B48: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82ED1B4C: 99410058  stb r10, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u8 ) };
	// 82ED1B50: 9AA10061  stb r21, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[21].u8 ) };
	// 82ED1B54: 8078007C  lwz r3, 0x7c(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED1B58: 48015CB1  bl 0x82ee7808
	ctx.lr = 0x82ED1B5C;
	sub_82EE7808(ctx, base);
	// 82ED1B5C: 38210900  addi r1, r1, 0x900
	ctx.r[1].s64 = ctx.r[1].s64 + 2304;
	// 82ED1B60: 482D6634  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
	// 82ED1B64: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1B68: 3A800018  li r20, 0x18
	ctx.r[20].s64 = 24;
	// 82ED1B6C: 7D56A02E  lwzx r10, r22, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82ED1B70: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED1B74: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1B78: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1B7C: 4098002C  bge cr6, 0x82ed1ba8
	if !ctx.cr[6].lt {
	pc = 0x82ED1BA8; continue 'dispatch;
	}
	// 82ED1B80: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED1B84: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED1B88: 38E94630  addi r7, r9, 0x4630
	ctx.r[7].s64 = ctx.r[9].s64 + 17968;
	// 82ED1B8C: 38C84628  addi r6, r8, 0x4628
	ctx.r[6].s64 = ctx.r[8].s64 + 17960;
	// 82ED1B90: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED1B94: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82ED1B98: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82ED1B9C: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82ED1BA0: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82ED1BA4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82ED1BA8: 81780084  lwz r11, 0x84(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED1BAC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82ED1BB0: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82ED1BB4: 615C0080  ori r28, r10, 0x80
	ctx.r[28].u64 = ctx.r[10].u64 | 128;
	// 82ED1BB8: 3921007C  addi r9, r1, 0x7c
	ctx.r[9].s64 = ctx.r[1].s64 + 124;
	// 82ED1BBC: 92610074  stw r19, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[19].u32 ) };
	// 82ED1BC0: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED1BC4: 93810078  stw r28, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[28].u32 ) };
	// 82ED1BC8: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82ED1BCC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82ED1BD0: 91180084  stw r8, 0x84(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(132 as u32), ctx.r[8].u32 ) };
	// 82ED1BD4: 409A0218  bne cr6, 0x82ed1dec
	if !ctx.cr[6].eq {
	pc = 0x82ED1DEC; continue 'dispatch;
	}
	// 82ED1BD8: 7D56A02E  lwzx r10, r22, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82ED1BDC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED1BE0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1BE4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1BE8: 40980020  bge cr6, 0x82ed1c08
	if !ctx.cr[6].lt {
	pc = 0x82ED1C08; continue 'dispatch;
	}
	// 82ED1BEC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED1BF0: 390945DC  addi r8, r9, 0x45dc
	ctx.r[8].s64 = ctx.r[9].s64 + 17884;
	// 82ED1BF4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED1BF8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED1BFC: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED1C00: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED1C04: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED1C08: 80780054  lwz r3, 0x54(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED1C0C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82ED1C10: 38970024  addi r4, r23, 0x24
	ctx.r[4].s64 = ctx.r[23].s64 + 36;
	// 82ED1C14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1C18: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82ED1C1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1C20: 4E800421  bctrl
	ctx.lr = 0x82ED1C24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1C24: 7D56A02E  lwzx r10, r22, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82ED1C28: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1C2C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED1C30: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1C34: 40980020  bge cr6, 0x82ed1c54
	if !ctx.cr[6].lt {
	pc = 0x82ED1C54; continue 'dispatch;
	}
	// 82ED1C38: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED1C3C: 3909461C  addi r8, r9, 0x461c
	ctx.r[8].s64 = ctx.r[9].s64 + 17948;
	// 82ED1C40: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED1C44: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED1C48: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED1C4C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED1C50: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED1C54: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED1C58: 7E7E9B78  mr r30, r19
	ctx.r[30].u64 = ctx.r[19].u64;
	// 82ED1C5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1C60: 409900B4  ble cr6, 0x82ed1d14
	if !ctx.cr[6].gt {
	pc = 0x82ED1D14; continue 'dispatch;
	}
	// 82ED1C64: 7E7F9B78  mr r31, r19
	ctx.r[31].u64 = ctx.r[19].u64;
	// 82ED1C68: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1C6C: 7D5F5A14  add r10, r31, r11
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82ED1C70: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1C74: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82ED1C78: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82ED1C7C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED1C80: 890B0018  lbz r8, 0x18(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED1C84: 2B080002  cmplwi cr6, r8, 2
	ctx.cr[6].compare_u32(ctx.r[8].u32, 2 as u32, &mut ctx.xer);
	// 82ED1C88: 409A0078  bne cr6, 0x82ed1d00
	if !ctx.cr[6].eq {
	pc = 0x82ED1D00; continue 'dispatch;
	}
	// 82ED1C8C: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED1C90: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82ED1C94: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED1C98: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82ED1C9C: 419A0064  beq cr6, 0x82ed1d00
	if ctx.cr[6].eq {
	pc = 0x82ED1D00; continue 'dispatch;
	}
	// 82ED1CA0: 38970010  addi r4, r23, 0x10
	ctx.r[4].s64 = ctx.r[23].s64 + 16;
	// 82ED1CA4: 80B80070  lwz r5, 0x70(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1CA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED1CAC: 4BFFAED5  bl 0x82eccb80
	ctx.lr = 0x82ED1CB0;
	sub_82ECCB80(ctx, base);
	// 82ED1CB0: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82ED1CB4: 419A0018  beq cr6, 0x82ed1ccc
	if ctx.cr[6].eq {
	pc = 0x82ED1CCC; continue 'dispatch;
	}
	// 82ED1CB8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1CBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED1CC0: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED1CC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1CC8: 4E800421  bctrl
	ctx.lr = 0x82ED1CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1CCC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED1CD0: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82ED1CD4: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1CD8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82ED1CDC: 7D3F5214  add r9, r31, r10
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82ED1CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82ED1CE4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED1CE8: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82ED1CEC: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ED1CF0: 7CEB502E  lwzx r7, r11, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1CF4: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED1CF8: 80C80004  lwz r6, 4(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1CFC: 90C90004  stw r6, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82ED1D00: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED1D04: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82ED1D08: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82ED1D0C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED1D10: 4198FF58  blt cr6, 0x82ed1c68
	if ctx.cr[6].lt {
	pc = 0x82ED1C68; continue 'dispatch;
	}
	// 82ED1D14: 81570060  lwz r10, 0x60(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED1D18: 3921048C  addi r9, r1, 0x48c
	ctx.r[9].s64 = ctx.r[1].s64 + 1164;
	// 82ED1D1C: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 82ED1D20: 93810488  stw r28, 0x488(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1160 as u32), ctx.r[28].u32 ) };
	// 82ED1D24: 3BD7005C  addi r30, r23, 0x5c
	ctx.r[30].s64 = ctx.r[23].s64 + 92;
	// 82ED1D28: 91210480  stw r9, 0x480(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1152 as u32), ctx.r[9].u32 ) };
	// 82ED1D2C: 91610484  stw r11, 0x484(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1156 as u32), ctx.r[11].u32 ) };
	// 82ED1D30: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 82ED1D34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1D38: 40990080  ble cr6, 0x82ed1db8
	if !ctx.cr[6].gt {
	pc = 0x82ED1DB8; continue 'dispatch;
	}
	// 82ED1D3C: 7E7F9B78  mr r31, r19
	ctx.r[31].u64 = ctx.r[19].u64;
	// 82ED1D40: 48000008  b 0x82ed1d48
	pc = 0x82ED1D48; continue 'dispatch;
	// 82ED1D44: 81610484  lwz r11, 0x484(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1156 as u32) ) } as u64;
	// 82ED1D48: 81410488  lwz r10, 0x488(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1160 as u32) ) } as u64;
	// 82ED1D4C: 554900BE  clrlwi r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED1D50: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED1D54: 409A0014  bne cr6, 0x82ed1d68
	if !ctx.cr[6].eq {
	pc = 0x82ED1D68; continue 'dispatch;
	}
	// 82ED1D58: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82ED1D5C: 38610480  addi r3, r1, 0x480
	ctx.r[3].s64 = ctx.r[1].s64 + 1152;
	// 82ED1D60: 4BFD4B21  bl 0x82ea6880
	ctx.lr = 0x82ED1D64;
	sub_82EA6880(ctx, base);
	// 82ED1D64: 81610484  lwz r11, 0x484(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1156 as u32) ) } as u64;
	// 82ED1D68: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ED1D6C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1D70: 81010480  lwz r8, 0x480(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1152 as u32) ) } as u64;
	// 82ED1D74: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED1D78: 91410484  stw r10, 0x484(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1156 as u32), ctx.r[10].u32 ) };
	// 82ED1D7C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED1D80: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82ED1D84: 7CE9F82E  lwzx r7, r9, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED1D88: 81470010  lwz r10, 0x10(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED1D8C: 38CA0014  addi r6, r10, 0x14
	ctx.r[6].s64 = ctx.r[10].s64 + 20;
	// 82ED1D90: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82ED1D94: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1D98: 7C85F82E  lwzx r4, r5, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED1D9C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82ED1DA0: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED1DA4: 386A0014  addi r3, r10, 0x14
	ctx.r[3].s64 = ctx.r[10].s64 + 20;
	// 82ED1DA8: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82ED1DAC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1DB0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED1DB4: 4198FF90  blt cr6, 0x82ed1d44
	if ctx.cr[6].lt {
	pc = 0x82ED1D44; continue 'dispatch;
	}
	// 82ED1DB8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82ED1DBC: 38610480  addi r3, r1, 0x480
	ctx.r[3].s64 = ctx.r[1].s64 + 1152;
	// 82ED1DC0: 48051CC9  bl 0x82f23a88
	ctx.lr = 0x82ED1DC4;
	sub_82F23A88(ctx, base);
	// 82ED1DC4: 81610488  lwz r11, 0x488(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1160 as u32) ) } as u64;
	// 82ED1DC8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1DCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1DD0: 409A001C  bne cr6, 0x82ed1dec
	if !ctx.cr[6].eq {
	pc = 0x82ED1DEC; continue 'dispatch;
	}
	// 82ED1DD4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED1DD8: 80810480  lwz r4, 0x480(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1152 as u32) ) } as u64;
	// 82ED1DDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1DE0: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1DE4: 7C76502E  lwzx r3, r22, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1DE8: 4BFCE9C9  bl 0x82ea07b0
	ctx.lr = 0x82ED1DEC;
	sub_82EA07B0(ctx, base);
	// 82ED1DEC: 7D56A02E  lwzx r10, r22, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82ED1DF0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED1DF4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1DF8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1DFC: 40980020  bge cr6, 0x82ed1e1c
	if !ctx.cr[6].lt {
	pc = 0x82ED1E1C; continue 'dispatch;
	}
	// 82ED1E00: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED1E04: 39094610  addi r8, r9, 0x4610
	ctx.r[8].s64 = ctx.r[9].s64 + 17936;
	// 82ED1E08: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED1E0C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED1E10: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED1E14: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED1E18: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED1E1C: 81770060  lwz r11, 0x60(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED1E20: 3B77005C  addi r27, r23, 0x5c
	ctx.r[27].s64 = ctx.r[23].s64 + 92;
	// 82ED1E24: A397002A  lhz r28, 0x2a(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(42 as u32) ) } as u64;
	// 82ED1E28: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 82ED1E2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1E30: 409900E4  ble cr6, 0x82ed1f14
	if !ctx.cr[6].gt {
	pc = 0x82ED1F14; continue 'dispatch;
	}
	// 82ED1E34: 3B570010  addi r26, r23, 0x10
	ctx.r[26].s64 = ctx.r[23].s64 + 16;
	// 82ED1E38: 7E7E9B78  mr r30, r19
	ctx.r[30].u64 = ctx.r[19].u64;
	// 82ED1E3C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82ED1E40: 81780070  lwz r11, 0x70(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1E44: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82ED1E48: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1E4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED1E50: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82ED1E54: 7FEAF214  add r31, r10, r30
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82ED1E58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED1E5C: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1E60: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1E64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED1E68: 4E800421  bctrl
	ctx.lr = 0x82ED1E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED1E6C: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1E70: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED1E74: 419A0030  beq cr6, 0x82ed1ea4
	if ctx.cr[6].eq {
	pc = 0x82ED1EA4; continue 'dispatch;
	}
	// 82ED1E78: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1E7C: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED1E80: 81380074  lwz r9, 0x74(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED1E84: 7D5C5214  add r10, r28, r10
	ctx.r[10].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82ED1E88: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED1E8C: A10B001A  lhz r8, 0x1a(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 82ED1E90: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82ED1E94: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82ED1E98: 89091BB0  lbz r8, 0x1bb0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7088 as u32) ) } as u64;
	// 82ED1E9C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED1EA0: 409A0028  bne cr6, 0x82ed1ec8
	if !ctx.cr[6].eq {
	pc = 0x82ED1EC8; continue 'dispatch;
	}
	// 82ED1EA4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1EA8: 4801B4C1  bl 0x82eed368
	ctx.lr = 0x82ED1EAC;
	sub_82EED368(ctx, base);
	// 82ED1EAC: 817700B8  lwz r11, 0xb8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED1EB0: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82ED1EB4: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 82ED1EB8: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82ED1EBC: 532A3032  rlwimi r10, r25, 6, 0, 0x19
	ctx.r[10].u64 = (((ctx.r[25].u32).rotate_left(6) as u64) & 0x00000000FFFFFFC0) | (ctx.r[10].u64 & 0xFFFFFFFF0000003F);
	// 82ED1EC0: 994B0025  stb r10, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[10].u8 ) };
	// 82ED1EC4: 4800003C  b 0x82ed1f00
	pc = 0x82ED1F00; continue 'dispatch;
	// 82ED1EC8: 2F150001  cmpwi cr6, r21, 1
	ctx.cr[6].compare_i32(ctx.r[21].s32, 1, &mut ctx.xer);
	// 82ED1ECC: 409A0034  bne cr6, 0x82ed1f00
	if !ctx.cr[6].eq {
	pc = 0x82ED1F00; continue 'dispatch;
	}
	// 82ED1ED0: 895700D8  lbz r10, 0xd8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ED1ED4: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 82ED1ED8: 409A0018  bne cr6, 0x82ed1ef0
	if !ctx.cr[6].eq {
	pc = 0x82ED1EF0; continue 'dispatch;
	}
	// 82ED1EDC: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED1EE0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82ED1EE4: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED1EE8: 80A900B8  lwz r5, 0xb8(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED1EEC: 48000008  b 0x82ed1ef4
	pc = 0x82ED1EF4; continue 'dispatch;
	// 82ED1EF0: 80B700B8  lwz r5, 0xb8(r23)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED1EF4: 8098006C  lwz r4, 0x6c(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED1EF8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED1EFC: 4809BCBD  bl 0x82f6dbb8
	ctx.lr = 0x82ED1F00;
	sub_82F6DBB8(ctx, base);
	// 82ED1F00: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1F04: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED1F08: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82ED1F0C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED1F10: 4198FF30  blt cr6, 0x82ed1e40
	if ctx.cr[6].lt {
	pc = 0x82ED1E40; continue 'dispatch;
	}
	// 82ED1F14: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED1F18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1F1C: 40990058  ble cr6, 0x82ed1f74
	if !ctx.cr[6].gt {
	pc = 0x82ED1F74; continue 'dispatch;
	}
	// 82ED1F20: 7D56A02E  lwzx r10, r22, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82ED1F24: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED1F28: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1F2C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1F30: 40980020  bge cr6, 0x82ed1f50
	if !ctx.cr[6].lt {
	pc = 0x82ED1F50; continue 'dispatch;
	}
	// 82ED1F34: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED1F38: 39094604  addi r8, r9, 0x4604
	ctx.r[8].s64 = ctx.r[9].s64 + 17924;
	// 82ED1F3C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED1F40: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED1F44: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED1F48: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED1F4C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED1F50: 81780070  lwz r11, 0x70(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1F54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED1F58: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82ED1F5C: 409A0008  bne cr6, 0x82ed1f64
	if !ctx.cr[6].eq {
	pc = 0x82ED1F64; continue 'dispatch;
	}
	// 82ED1F60: 7E669B78  mr r6, r19
	ctx.r[6].u64 = ctx.r[19].u64;
	// 82ED1F64: 80780058  lwz r3, 0x58(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED1F68: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED1F6C: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1F70: 480519F1  bl 0x82f23960
	ctx.lr = 0x82ED1F74;
	sub_82F23960(ctx, base);
	// 82ED1F74: 81780084  lwz r11, 0x84(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED1F78: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1F7C: 91780084  stw r11, 0x84(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED1F80: 40820028  bne 0x82ed1fa8
	if !ctx.cr[0].eq {
	pc = 0x82ED1FA8; continue 'dispatch;
	}
	// 82ED1F84: 8978008C  lbz r11, 0x8c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED1F88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED1F8C: 409A001C  bne cr6, 0x82ed1fa8
	if !ctx.cr[6].eq {
	pc = 0x82ED1FA8; continue 'dispatch;
	}
	// 82ED1F90: 81780080  lwz r11, 0x80(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED1F94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED1F98: 419A0010  beq cr6, 0x82ed1fa8
	if ctx.cr[6].eq {
	pc = 0x82ED1FA8; continue 'dispatch;
	}
	// 82ED1F9C: 92780080  stw r19, 0x80(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(128 as u32), ctx.r[19].u32 ) };
	// 82ED1FA0: 8078007C  lwz r3, 0x7c(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED1FA4: 48015E55  bl 0x82ee7df8
	ctx.lr = 0x82ED1FA8;
	sub_82EE7DF8(ctx, base);
	// 82ED1FA8: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED1FAC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED1FB0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED1FB4: 409A001C  bne cr6, 0x82ed1fd0
	if !ctx.cr[6].eq {
	pc = 0x82ED1FD0; continue 'dispatch;
	}
	// 82ED1FB8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED1FBC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED1FC0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED1FC4: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED1FC8: 7C76502E  lwzx r3, r22, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED1FCC: 4BFCE7E5  bl 0x82ea07b0
	ctx.lr = 0x82ED1FD0;
	sub_82EA07B0(ctx, base);
	// 82ED1FD0: 7D56A02E  lwzx r10, r22, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82ED1FD4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED1FD8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED1FDC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED1FE0: 40980020  bge cr6, 0x82ed2000
	if !ctx.cr[6].lt {
	pc = 0x82ED2000; continue 'dispatch;
	}
	// 82ED1FE4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82ED1FE8: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82ED1FEC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED1FF0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED1FF4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED1FF8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED1FFC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED2000: 38210900  addi r1, r1, 0x900
	ctx.r[1].s64 = ctx.r[1].s64 + 2304;
	// 82ED2004: 482D6190  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED2008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED2008 size=496
    let mut pc: u32 = 0x82ED2008;
    'dispatch: loop {
        match pc {
            0x82ED2008 => {
    //   block [0x82ED2008..0x82ED21F8)
	// 82ED2008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED200C: 482D6151  bl 0x831a815c
	ctx.lr = 0x82ED2010;
	sub_831A8130(ctx, base);
	// 82ED2010: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED2014: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ED2018: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED201C: 817B0084  lwz r11, 0x84(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2020: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2024: 419A0028  beq cr6, 0x82ed204c
	if ctx.cr[6].eq {
	pc = 0x82ED204C; continue 'dispatch;
	}
	// 82ED2028: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82ED202C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ED2030: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED2034: 807B007C  lwz r3, 0x7c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED2038: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED203C: 480157CD  bl 0x82ee7808
	ctx.lr = 0x82ED2040;
	sub_82EE7808(ctx, base);
	// 82ED2040: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ED2044: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82ED2048: 482D6164  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82ED204C: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2050: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED2054: 419A0010  beq cr6, 0x82ed2064
	if ctx.cr[6].eq {
	pc = 0x82ED2064; continue 'dispatch;
	}
	// 82ED2058: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED205C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ED2060: B15E0006  sth r10, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED2064: 817B0084  lwz r11, 0x84(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2068: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82ED206C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82ED2070: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82ED2074: 3901006C  addi r8, r1, 0x6c
	ctx.r[8].s64 = ctx.r[1].s64 + 108;
	// 82ED2078: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82ED207C: 913B0084  stw r9, 0x84(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82ED2080: 61470004  ori r7, r10, 4
	ctx.r[7].u64 = ctx.r[10].u64 | 4;
	// 82ED2084: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2088: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82ED208C: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 82ED2090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED2094: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82ED2098: 80A60010  lwz r5, 0x10(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED209C: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82ED20A0: 4E800421  bctrl
	ctx.lr = 0x82ED20A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED20A4: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED20A8: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82ED20AC: 937E0008  stw r27, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82ED20B0: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 82ED20B4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82ED20B8: 409900B4  ble cr6, 0x82ed216c
	if !ctx.cr[6].gt {
	pc = 0x82ED216C; continue 'dispatch;
	}
	// 82ED20BC: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82ED20C0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED20C4: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED20C8: 3BEB01F8  addi r31, r11, 0x1f8
	ctx.r[31].s64 = ctx.r[11].s64 + 504;
	// 82ED20CC: A14B01FE  lhz r10, 0x1fe(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(510 as u32) ) } as u64;
	// 82ED20D0: A12B01FC  lhz r9, 0x1fc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(508 as u32) ) } as u64;
	// 82ED20D4: 554804BE  clrlwi r8, r10, 0x12
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 82ED20D8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82ED20DC: 409A0010  bne cr6, 0x82ed20ec
	if !ctx.cr[6].eq {
	pc = 0x82ED20EC; continue 'dispatch;
	}
	// 82ED20E0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED20E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED20E8: 4832D271  bl 0x831ff358
	ctx.lr = 0x82ED20EC;
	sub_831FF358(ctx, base);
	// 82ED20EC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED20F0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED20F4: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82ED20F8: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED20FC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2100: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED2104: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82ED2108: 80C10060  lwz r6, 0x60(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED210C: 7CBD302E  lwzx r5, r29, r6
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82ED2110: 806500B8  lwz r3, 0xb8(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED2114: A0830020  lhz r4, 0x20(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED2118: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82ED211C: 419A0034  beq cr6, 0x82ed2150
	if ctx.cr[6].eq {
	pc = 0x82ED2150; continue 'dispatch;
	}
	// 82ED2120: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82ED2124: 409A0014  bne cr6, 0x82ed2138
	if !ctx.cr[6].eq {
	pc = 0x82ED2138; continue 'dispatch;
	}
	// 82ED2128: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED212C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82ED2130: 4801A349  bl 0x82eec478
	ctx.lr = 0x82ED2134;
	sub_82EEC478(ctx, base);
	// 82ED2134: 4800001C  b 0x82ed2150
	pc = 0x82ED2150; continue 'dispatch;
	// 82ED2138: 817C00B8  lwz r11, 0xb8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED213C: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82ED2140: 419A0010  beq cr6, 0x82ed2150
	if ctx.cr[6].eq {
	pc = 0x82ED2150; continue 'dispatch;
	}
	// 82ED2144: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ED2148: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED214C: 480105D5  bl 0x82ee2720
	ctx.lr = 0x82ED2150;
	sub_82EE2720(ctx, base);
	// 82ED2150: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED2154: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82ED2158: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82ED215C: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED2160: 4198FF60  blt cr6, 0x82ed20c0
	if ctx.cr[6].lt {
	pc = 0x82ED20C0; continue 'dispatch;
	}
	// 82ED2164: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82ED2168: 409A0018  bne cr6, 0x82ed2180
	if !ctx.cr[6].eq {
	pc = 0x82ED2180; continue 'dispatch;
	}
	// 82ED216C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED2170: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED2174: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2178: 806A00B8  lwz r3, 0xb8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED217C: 4801A2FD  bl 0x82eec478
	ctx.lr = 0x82ED2180;
	sub_82EEC478(ctx, base);
	// 82ED2180: 817B0084  lwz r11, 0x84(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2184: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2188: 917B0084  stw r11, 0x84(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED218C: 40820028  bne 0x82ed21b4
	if !ctx.cr[0].eq {
	pc = 0x82ED21B4; continue 'dispatch;
	}
	// 82ED2190: 897B008C  lbz r11, 0x8c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED2194: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED2198: 409A001C  bne cr6, 0x82ed21b4
	if !ctx.cr[6].eq {
	pc = 0x82ED21B4; continue 'dispatch;
	}
	// 82ED219C: 817B0080  lwz r11, 0x80(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED21A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED21A4: 419A0010  beq cr6, 0x82ed21b4
	if ctx.cr[6].eq {
	pc = 0x82ED21B4; continue 'dispatch;
	}
	// 82ED21A8: 933B0080  stw r25, 0x80(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82ED21AC: 807B007C  lwz r3, 0x7c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED21B0: 48015C49  bl 0x82ee7df8
	ctx.lr = 0x82ED21B4;
	sub_82EE7DF8(ctx, base);
	// 82ED21B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED21B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED21BC: 48011715  bl 0x82ee38d0
	ctx.lr = 0x82ED21C0;
	sub_82EE38D0(ctx, base);
	// 82ED21C0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED21C4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED21C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED21CC: 409A0020  bne cr6, 0x82ed21ec
	if !ctx.cr[6].eq {
	pc = 0x82ED21EC; continue 'dispatch;
	}
	// 82ED21D0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED21D4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED21D8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED21DC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED21E0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED21E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED21E8: 4BFCE5C9  bl 0x82ea07b0
	ctx.lr = 0x82ED21EC;
	sub_82EA07B0(ctx, base);
	// 82ED21EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED21F0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82ED21F4: 482D5FB8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED21F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED21F8 size=596
    let mut pc: u32 = 0x82ED21F8;
    'dispatch: loop {
        match pc {
            0x82ED21F8 => {
    //   block [0x82ED21F8..0x82ED244C)
	// 82ED21F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED21FC: 482D5F69  bl 0x831a8164
	ctx.lr = 0x82ED2200;
	sub_831A8130(ctx, base);
	// 82ED2200: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED2204: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED2208: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ED220C: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED2214: 419A0010  beq cr6, 0x82ed2224
	if ctx.cr[6].eq {
	pc = 0x82ED2224; continue 'dispatch;
	}
	// 82ED2218: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED221C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ED2220: B15D0006  sth r10, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED2224: 817C0084  lwz r11, 0x84(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2228: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED222C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED2230: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED2234: 917C0084  stw r11, 0x84(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED2238: 48011771  bl 0x82ee39a8
	ctx.lr = 0x82ED223C;
	sub_82EE39A8(ctx, base);
	// 82ED223C: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2240: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82ED2244: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82ED2248: 3921005C  addi r9, r1, 0x5c
	ctx.r[9].s64 = ctx.r[1].s64 + 92;
	// 82ED224C: 61480004  ori r8, r10, 4
	ctx.r[8].u64 = ctx.r[10].u64 | 4;
	// 82ED2250: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82ED2254: 80C70010  lwz r6, 0x10(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED2258: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED225C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82ED2260: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED2264: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82ED2268: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82ED226C: 4E800421  bctrl
	ctx.lr = 0x82ED2270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2270: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED2274: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82ED2278: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82ED227C: 40990084  ble cr6, 0x82ed2300
	if !ctx.cr[6].gt {
	pc = 0x82ED2300; continue 'dispatch;
	}
	// 82ED2280: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82ED2284: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82ED2288: 6165FFFF  ori r5, r11, 0xffff
	ctx.r[5].u64 = ctx.r[11].u64 | 65535;
	// 82ED228C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED2290: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82ED2294: 7D27502E  lwzx r9, r7, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED2298: A10901FC  lhz r8, 0x1fc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(508 as u32) ) } as u64;
	// 82ED229C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED22A0: 40990024  ble cr6, 0x82ed22c4
	if !ctx.cr[6].gt {
	pc = 0x82ED22C4; continue 'dispatch;
	}
	// 82ED22A4: 814901F8  lwz r10, 0x1f8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(504 as u32) ) } as u64;
	// 82ED22A8: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED22AC: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82ED22B0: 419A0018  beq cr6, 0x82ed22c8
	if ctx.cr[6].eq {
	pc = 0x82ED22C8; continue 'dispatch;
	}
	// 82ED22B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED22B8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED22BC: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED22C0: 4198FFE8  blt cr6, 0x82ed22a8
	if ctx.cr[6].lt {
	pc = 0x82ED22A8; continue 'dispatch;
	}
	// 82ED22C4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED22C8: A14901FC  lhz r10, 0x1fc(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(508 as u32) ) } as u64;
	// 82ED22CC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED22D0: 808901F8  lwz r4, 0x1f8(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(504 as u32) ) } as u64;
	// 82ED22D4: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82ED22D8: 7C6A2A14  add r3, r10, r5
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82ED22DC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82ED22E0: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82ED22E4: 556A13BA  rlwinm r10, r11, 2, 0xe, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED22E8: B16901FC  sth r11, 0x1fc(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(508 as u32), ctx.r[11].u16 ) };
	// 82ED22EC: 7D2A202E  lwzx r9, r10, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82ED22F0: 7D28212E  stwx r9, r8, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32), ctx.r[9].u32) };
	// 82ED22F4: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED22F8: 7F064000  cmpw cr6, r6, r8
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED22FC: 4198FF90  blt cr6, 0x82ed228c
	if ctx.cr[6].lt {
	pc = 0x82ED228C; continue 'dispatch;
	}
	// 82ED2300: 83DD000C  lwz r30, 0xc(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2304: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED2308: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED230C: 48019CB5  bl 0x82eebfc0
	ctx.lr = 0x82ED2310;
	sub_82EEBFC0(ctx, base);
	// 82ED2310: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2314: 937D0008  stw r27, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82ED2318: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED231C: 419A0034  beq cr6, 0x82ed2350
	if ctx.cr[6].eq {
	pc = 0x82ED2350; continue 'dispatch;
	}
	// 82ED2320: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED2324: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED2328: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED232C: B13D0006  sth r9, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED2330: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED2334: 409A001C  bne cr6, 0x82ed2350
	if !ctx.cr[6].eq {
	pc = 0x82ED2350; continue 'dispatch;
	}
	// 82ED2338: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED233C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED2340: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED2344: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2348: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED234C: 4E800421  bctrl
	ctx.lr = 0x82ED2350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2350: A15E0022  lhz r10, 0x22(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(34 as u32) ) } as u64;
	// 82ED2354: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED2358: 2B0AFFFF  cmplwi cr6, r10, 0xffff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 65535 as u32, &mut ctx.xer);
	// 82ED235C: 409A004C  bne cr6, 0x82ed23a8
	if !ctx.cr[6].eq {
	pc = 0x82ED23A8; continue 'dispatch;
	}
	// 82ED2360: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82ED2364: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 82ED2368: B15E0022  sth r10, 0x22(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(34 as u32), ctx.r[10].u16 ) };
	// 82ED236C: 810B0044  lwz r8, 0x44(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82ED2370: 80EB0048  lwz r7, 0x48(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED2374: 54E600BE  clrlwi r6, r7, 2
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED2378: 7F083000  cmpw cr6, r8, r6
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82ED237C: 409A0010  bne cr6, 0x82ed238c
	if !ctx.cr[6].eq {
	pc = 0x82ED238C; continue 'dispatch;
	}
	// 82ED2380: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED2384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED2388: 4BFD44F9  bl 0x82ea6880
	ctx.lr = 0x82ED238C;
	sub_82EA6880(ctx, base);
	// 82ED238C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2390: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2394: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED2398: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED239C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED23A0: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED23A4: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED23A8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED23AC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED23B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED23B4: 409A0020  bne cr6, 0x82ed23d4
	if !ctx.cr[6].eq {
	pc = 0x82ED23D4; continue 'dispatch;
	}
	// 82ED23B8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED23BC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED23C0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED23C4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED23C8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED23CC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED23D0: 4BFCE3E1  bl 0x82ea07b0
	ctx.lr = 0x82ED23D4;
	sub_82EA07B0(ctx, base);
	// 82ED23D4: 817C0084  lwz r11, 0x84(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED23D8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED23DC: 917C0084  stw r11, 0x84(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED23E0: 40820028  bne 0x82ed2408
	if !ctx.cr[0].eq {
	pc = 0x82ED2408; continue 'dispatch;
	}
	// 82ED23E4: 897C008C  lbz r11, 0x8c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED23E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED23EC: 409A001C  bne cr6, 0x82ed2408
	if !ctx.cr[6].eq {
	pc = 0x82ED2408; continue 'dispatch;
	}
	// 82ED23F0: 817C0080  lwz r11, 0x80(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED23F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED23F8: 419A0010  beq cr6, 0x82ed2408
	if ctx.cr[6].eq {
	pc = 0x82ED2408; continue 'dispatch;
	}
	// 82ED23FC: 937C0080  stw r27, 0x80(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(128 as u32), ctx.r[27].u32 ) };
	// 82ED2400: 807C007C  lwz r3, 0x7c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED2404: 480159F5  bl 0x82ee7df8
	ctx.lr = 0x82ED2408;
	sub_82EE7DF8(ctx, base);
	// 82ED2408: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED240C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED2410: 419A0034  beq cr6, 0x82ed2444
	if ctx.cr[6].eq {
	pc = 0x82ED2444; continue 'dispatch;
	}
	// 82ED2414: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED2418: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED241C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED2420: B13D0006  sth r9, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED2424: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED2428: 409A001C  bne cr6, 0x82ed2444
	if !ctx.cr[6].eq {
	pc = 0x82ED2444; continue 'dispatch;
	}
	// 82ED242C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2430: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED2434: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED2438: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED243C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED2440: 4E800421  bctrl
	ctx.lr = 0x82ED2444;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2444: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82ED2448: 482D5D6C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED2450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED2450 size=336
    let mut pc: u32 = 0x82ED2450;
    'dispatch: loop {
        match pc {
            0x82ED2450 => {
    //   block [0x82ED2450..0x82ED25A0)
	// 82ED2450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED2454: 482D5D19  bl 0x831a816c
	ctx.lr = 0x82ED2458;
	sub_831A8130(ctx, base);
	// 82ED2458: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED245C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82ED2460: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED2464: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED2468: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED246C: A13F01FC  lhz r9, 0x1fc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82ED2470: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED2474: 40990024  ble cr6, 0x82ed2498
	if !ctx.cr[6].gt {
	pc = 0x82ED2498; continue 'dispatch;
	}
	// 82ED2478: 815F01F8  lwz r10, 0x1f8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 82ED247C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2480: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82ED2484: 419A0018  beq cr6, 0x82ed249c
	if ctx.cr[6].eq {
	pc = 0x82ED249C; continue 'dispatch;
	}
	// 82ED2488: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED248C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED2490: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED2494: 4198FFE8  blt cr6, 0x82ed247c
	if ctx.cr[6].lt {
	pc = 0x82ED247C; continue 'dispatch;
	}
	// 82ED2498: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED249C: A15F01FC  lhz r10, 0x1fc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82ED24A0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED24A4: 811F01F8  lwz r8, 0x1f8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 82ED24A8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82ED24AC: 3CCA0001  addis r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 65536;
	// 82ED24B0: 3CA08000  lis r5, -0x8000
	ctx.r[5].s64 = -2147483648;
	// 82ED24B4: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 82ED24B8: 3941005C  addi r10, r1, 0x5c
	ctx.r[10].s64 = ctx.r[1].s64 + 92;
	// 82ED24BC: 54C4043E  clrlwi r4, r6, 0x10
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	// 82ED24C0: 60A50004  ori r5, r5, 4
	ctx.r[5].u64 = ctx.r[5].u64 | 4;
	// 82ED24C4: 548613BA  rlwinm r6, r4, 2, 0xe, 0x1d
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED24C8: B09F01FC  sth r4, 0x1fc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[4].u16 ) };
	// 82ED24CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED24D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED24D4: 7D66402E  lwzx r11, r6, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82ED24D8: 7D69412E  stwx r11, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u32) };
	// 82ED24DC: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED24E0: 892B0025  lbz r9, 0x25(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82ED24E4: 50E93032  rlwimi r9, r7, 6, 0, 0x19
	ctx.r[9].u64 = (((ctx.r[7].u32).rotate_left(6) as u64) & 0x00000000FFFFFFC0) | (ctx.r[9].u64 & 0xFFFFFFFF0000003F);
	// 82ED24E8: 992B0025  stb r9, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[9].u8 ) };
	// 82ED24EC: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED24F0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82ED24F4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ED24F8: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82ED24FC: 80E80010  lwz r7, 0x10(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED2500: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82ED2504: 4E800421  bctrl
	ctx.lr = 0x82ED2508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2508: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED250C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82ED2510: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED2514: 40990038  ble cr6, 0x82ed254c
	if !ctx.cr[6].gt {
	pc = 0x82ED254C; continue 'dispatch;
	}
	// 82ED2518: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED251C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2520: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82ED2524: 419A0018  beq cr6, 0x82ed253c
	if ctx.cr[6].eq {
	pc = 0x82ED253C; continue 'dispatch;
	}
	// 82ED2528: 5508003E  slwi r8, r8, 0
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED252C: 83C800B8  lwz r30, 0xb8(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED2530: A0FE0020  lhz r7, 0x20(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED2534: 2B07FFFF  cmplwi cr6, r7, 0xffff
	ctx.cr[6].compare_u32(ctx.r[7].u32, 65535 as u32, &mut ctx.xer);
	// 82ED2538: 409A0014  bne cr6, 0x82ed254c
	if !ctx.cr[6].eq {
	pc = 0x82ED254C; continue 'dispatch;
	}
	// 82ED253C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED2540: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82ED2544: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED2548: 4198FFD4  blt cr6, 0x82ed251c
	if ctx.cr[6].lt {
	pc = 0x82ED251C; continue 'dispatch;
	}
	// 82ED254C: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2550: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82ED2554: 419A0018  beq cr6, 0x82ed256c
	if ctx.cr[6].eq {
	pc = 0x82ED256C; continue 'dispatch;
	}
	// 82ED2558: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED255C: 48019A65  bl 0x82eebfc0
	ctx.lr = 0x82ED2560;
	sub_82EEBFC0(ctx, base);
	// 82ED2560: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED2564: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED2568: 48019F11  bl 0x82eec478
	ctx.lr = 0x82ED256C;
	sub_82EEC478(ctx, base);
	// 82ED256C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED2570: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED2574: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED2578: 409A0020  bne cr6, 0x82ed2598
	if !ctx.cr[6].eq {
	pc = 0x82ED2598; continue 'dispatch;
	}
	// 82ED257C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2580: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED2584: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED2588: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED258C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED2590: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED2594: 4BFCE21D  bl 0x82ea07b0
	ctx.lr = 0x82ED2598;
	sub_82EA07B0(ctx, base);
	// 82ED2598: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED259C: 482D5C20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED25A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ED25A0 size=540
    let mut pc: u32 = 0x82ED25A0;
    'dispatch: loop {
        match pc {
            0x82ED25A0 => {
    //   block [0x82ED25A0..0x82ED27BC)
	// 82ED25A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED25A4: 482D5BAD  bl 0x831a8150
	ctx.lr = 0x82ED25A8;
	sub_831A8130(ctx, base);
	// 82ED25A8: 9421FB10  stwu r1, -0x4f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1264 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED25AC: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED25B0: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82ED25B4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ED25B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED25BC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ED25C0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82ED25C4: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED25C8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED25CC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED25D0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED25D4: 4098002C  bge cr6, 0x82ed2600
	if !ctx.cr[6].lt {
	pc = 0x82ED2600; continue 'dispatch;
	}
	// 82ED25D8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED25DC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED25E0: 38E94658  addi r7, r9, 0x4658
	ctx.r[7].s64 = ctx.r[9].s64 + 18008;
	// 82ED25E4: 38C845DC  addi r6, r8, 0x45dc
	ctx.r[6].s64 = ctx.r[8].s64 + 17884;
	// 82ED25E8: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED25EC: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82ED25F0: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82ED25F4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82ED25F8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED25FC: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82ED2600: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2604: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED2608: 815B006C  lwz r10, 0x6c(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED260C: C1BD0004  lfs f13, 4(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED2610: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED2614: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82ED2618: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED261C: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED2620: C18A0004  lfs f12, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ED2624: EC2C683C  fnmsubs f1, f12, f0, f13
	ctx.f[1].f64 = -(((ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82ED2628: 8109001C  lwz r8, 0x1c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ED262C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ED2630: 4E800421  bctrl
	ctx.lr = 0x82ED2634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2634: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82ED2638: 807B0054  lwz r3, 0x54(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED263C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82ED2640: 60E40080  ori r4, r7, 0x80
	ctx.r[4].u64 = ctx.r[7].u64 | 128;
	// 82ED2644: 38C1008C  addi r6, r1, 0x8c
	ctx.r[6].s64 = ctx.r[1].s64 + 140;
	// 82ED2648: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82ED264C: 90810088  stw r4, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[4].u32 ) };
	// 82ED2650: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82ED2654: 90C10080  stw r6, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[6].u32 ) };
	// 82ED2658: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82ED265C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2660: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ED2664: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED2668: 4E800421  bctrl
	ctx.lr = 0x82ED266C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED266C: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED2670: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2674: 81210080  lwz r9, 0x80(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED2678: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED267C: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2680: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82ED2684: 8388000C  lwz r28, 0xc(r8)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2688: 40980020  bge cr6, 0x82ed26a8
	if !ctx.cr[6].lt {
	pc = 0x82ED26A8; continue 'dispatch;
	}
	// 82ED268C: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED2690: 38E84648  addi r7, r8, 0x4648
	ctx.r[7].s64 = ctx.r[8].s64 + 17992;
	// 82ED2694: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED2698: 7CCC42E6  mftb r6, 0x10c
	ctx.r[6].u64 = crate::rt::rdtsc_u64();
	// 82ED269C: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82ED26A0: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82ED26A4: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82ED26A8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED26AC: 36CBFFFF  addic. r22, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82ED26B0: 418000AC  blt 0x82ed275c
	if ctx.cr[0].lt {
	pc = 0x82ED275C; continue 'dispatch;
	}
	// 82ED26B4: 3B290004  addi r25, r9, 4
	ctx.r[25].s64 = ctx.r[9].s64 + 4;
	// 82ED26B8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED26BC: 894B0005  lbz r10, 5(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82ED26C0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82ED26C4: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED26C8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82ED26CC: 419A0084  beq cr6, 0x82ed2750
	if ctx.cr[6].eq {
	pc = 0x82ED2750; continue 'dispatch;
	}
	// 82ED26D0: 817B0070  lwz r11, 0x70(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED26D4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82ED26D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ED26DC: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82ED26E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED26E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED26E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED26EC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED26F0: 4E800421  bctrl
	ctx.lr = 0x82ED26F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED26F4: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED26F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED26FC: 419A0054  beq cr6, 0x82ed2750
	if ctx.cr[6].eq {
	pc = 0x82ED2750; continue 'dispatch;
	}
	// 82ED2700: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2704: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED2708: 419A0048  beq cr6, 0x82ed2750
	if ctx.cr[6].eq {
	pc = 0x82ED2750; continue 'dispatch;
	}
	// 82ED270C: 393C000D  addi r9, r28, 0xd
	ctx.r[9].s64 = ctx.r[28].s64 + 13;
	// 82ED2710: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2714: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2718: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82ED271C: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED2720: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82ED2724: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82ED2728: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED272C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED2730: 7D6850AE  lbzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED2734: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82ED2738: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82ED273C: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED2740: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ED2744: 814B09A8  lwz r10, 0x9a8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82ED2748: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED274C: 4E800421  bctrl
	ctx.lr = 0x82ED2750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2750: 36D6FFFF  addic. r22, r22, -1
	ctx.xer.ca = (ctx.r[22].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82ED2754: 3B390008  addi r25, r25, 8
	ctx.r[25].s64 = ctx.r[25].s64 + 8;
	// 82ED2758: 4080FF60  bge 0x82ed26b8
	if !ctx.cr[0].lt {
	pc = 0x82ED26B8; continue 'dispatch;
	}
	// 82ED275C: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED2760: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2764: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2768: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED276C: 40980020  bge cr6, 0x82ed278c
	if !ctx.cr[6].lt {
	pc = 0x82ED278C; continue 'dispatch;
	}
	// 82ED2770: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82ED2774: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82ED2778: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED277C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED2780: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED2784: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED2788: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED278C: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED2790: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED2794: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED2798: 409A001C  bne cr6, 0x82ed27b4
	if !ctx.cr[6].eq {
	pc = 0x82ED27B4; continue 'dispatch;
	}
	// 82ED279C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED27A0: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED27A4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED27A8: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED27AC: 7C77502E  lwzx r3, r23, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED27B0: 4BFCE001  bl 0x82ea07b0
	ctx.lr = 0x82ED27B4;
	sub_82EA07B0(ctx, base);
	// 82ED27B4: 382104F0  addi r1, r1, 0x4f0
	ctx.r[1].s64 = ctx.r[1].s64 + 1264;
	// 82ED27B8: 482D59E8  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED27C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ED27C0 size=552
    let mut pc: u32 = 0x82ED27C0;
    'dispatch: loop {
        match pc {
            0x82ED27C0 => {
    //   block [0x82ED27C0..0x82ED29E8)
	// 82ED27C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED27C4: 482D598D  bl 0x831a8150
	ctx.lr = 0x82ED27C8;
	sub_831A8130(ctx, base);
	// 82ED27C8: 9421FB10  stwu r1, -0x4f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1264 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED27CC: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED27D0: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82ED27D4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82ED27D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED27DC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ED27E0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82ED27E4: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED27E8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED27EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED27F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED27F4: 4098002C  bge cr6, 0x82ed2820
	if !ctx.cr[6].lt {
	pc = 0x82ED2820; continue 'dispatch;
	}
	// 82ED27F8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED27FC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED2800: 38E94678  addi r7, r9, 0x4678
	ctx.r[7].s64 = ctx.r[9].s64 + 18040;
	// 82ED2804: 38C845DC  addi r6, r8, 0x45dc
	ctx.r[6].s64 = ctx.r[8].s64 + 17884;
	// 82ED2808: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED280C: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82ED2810: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82ED2814: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82ED2818: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED281C: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82ED2820: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2824: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED2828: 815A006C  lwz r10, 0x6c(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED282C: C1BD0004  lfs f13, 4(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED2830: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED2834: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82ED2838: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED283C: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED2840: C18A0004  lfs f12, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ED2844: EC2C683C  fnmsubs f1, f12, f0, f13
	ctx.f[1].f64 = -(((ctx.f[12].f64 * ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82ED2848: 8109001C  lwz r8, 0x1c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ED284C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ED2850: 4E800421  bctrl
	ctx.lr = 0x82ED2854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2854: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82ED2858: 807A0054  lwz r3, 0x54(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED285C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82ED2860: 60E40080  ori r4, r7, 0x80
	ctx.r[4].u64 = ctx.r[7].u64 | 128;
	// 82ED2864: 38C1008C  addi r6, r1, 0x8c
	ctx.r[6].s64 = ctx.r[1].s64 + 140;
	// 82ED2868: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82ED286C: 90810088  stw r4, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[4].u32 ) };
	// 82ED2870: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82ED2874: 90C10080  stw r6, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[6].u32 ) };
	// 82ED2878: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82ED287C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2880: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ED2884: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED2888: 4E800421  bctrl
	ctx.lr = 0x82ED288C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED288C: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED2890: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2894: 81210080  lwz r9, 0x80(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED2898: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED289C: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED28A0: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82ED28A4: 8368000C  lwz r27, 0xc(r8)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED28A8: 40980020  bge cr6, 0x82ed28c8
	if !ctx.cr[6].lt {
	pc = 0x82ED28C8; continue 'dispatch;
	}
	// 82ED28AC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED28B0: 38E84648  addi r7, r8, 0x4648
	ctx.r[7].s64 = ctx.r[8].s64 + 17992;
	// 82ED28B4: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED28B8: 7CCC42E6  mftb r6, 0x10c
	ctx.r[6].u64 = crate::rt::rdtsc_u64();
	// 82ED28BC: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82ED28C0: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82ED28C4: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82ED28C8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED28CC: 36CBFFFF  addic. r22, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82ED28D0: 418000B8  blt 0x82ed2988
	if ctx.cr[0].lt {
	pc = 0x82ED2988; continue 'dispatch;
	}
	// 82ED28D4: 3B290004  addi r25, r9, 4
	ctx.r[25].s64 = ctx.r[9].s64 + 4;
	// 82ED28D8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED28DC: 894B0005  lbz r10, 5(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82ED28E0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82ED28E4: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED28E8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82ED28EC: 419A0090  beq cr6, 0x82ed297c
	if ctx.cr[6].eq {
	pc = 0x82ED297C; continue 'dispatch;
	}
	// 82ED28F0: 817A0070  lwz r11, 0x70(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED28F4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82ED28F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ED28FC: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82ED2900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED2904: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED2908: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED290C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED2910: 4E800421  bctrl
	ctx.lr = 0x82ED2914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2914: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2918: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED291C: 419A0060  beq cr6, 0x82ed297c
	if ctx.cr[6].eq {
	pc = 0x82ED297C; continue 'dispatch;
	}
	// 82ED2920: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2924: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED2928: 419A0054  beq cr6, 0x82ed297c
	if ctx.cr[6].eq {
	pc = 0x82ED297C; continue 'dispatch;
	}
	// 82ED292C: 393B000D  addi r9, r27, 0xd
	ctx.r[9].s64 = ctx.r[27].s64 + 13;
	// 82ED2930: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2934: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2938: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82ED293C: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED2940: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82ED2944: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82ED2948: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED294C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED2950: 7D6850AE  lbzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED2954: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82ED2958: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82ED295C: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED2960: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ED2964: 814B09A4  lwz r10, 0x9a4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82ED2968: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED296C: 4E800421  bctrl
	ctx.lr = 0x82ED2970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2970: 893C0004  lbz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2974: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED2978: 409A0010  bne cr6, 0x82ed2988
	if !ctx.cr[6].eq {
	pc = 0x82ED2988; continue 'dispatch;
	}
	// 82ED297C: 36D6FFFF  addic. r22, r22, -1
	ctx.xer.ca = (ctx.r[22].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82ED2980: 3B390008  addi r25, r25, 8
	ctx.r[25].s64 = ctx.r[25].s64 + 8;
	// 82ED2984: 4080FF54  bge 0x82ed28d8
	if !ctx.cr[0].lt {
	pc = 0x82ED28D8; continue 'dispatch;
	}
	// 82ED2988: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82ED298C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2990: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2994: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED2998: 40980020  bge cr6, 0x82ed29b8
	if !ctx.cr[6].lt {
	pc = 0x82ED29B8; continue 'dispatch;
	}
	// 82ED299C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82ED29A0: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82ED29A4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED29A8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED29AC: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED29B0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED29B4: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED29B8: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED29BC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED29C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED29C4: 409A001C  bne cr6, 0x82ed29e0
	if !ctx.cr[6].eq {
	pc = 0x82ED29E0; continue 'dispatch;
	}
	// 82ED29C8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED29CC: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED29D0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED29D4: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED29D8: 7C77502E  lwzx r3, r23, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED29DC: 4BFCDDD5  bl 0x82ea07b0
	ctx.lr = 0x82ED29E0;
	sub_82EA07B0(ctx, base);
	// 82ED29E0: 382104F0  addi r1, r1, 0x4f0
	ctx.r[1].s64 = ctx.r[1].s64 + 1264;
	// 82ED29E4: 482D57BC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED29E8 size=1080
    let mut pc: u32 = 0x82ED29E8;
    'dispatch: loop {
        match pc {
            0x82ED29E8 => {
    //   block [0x82ED29E8..0x82ED2E20)
	// 82ED29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED29EC: 482D5749  bl 0x831a8134
	ctx.lr = 0x82ED29F0;
	sub_831A8130(ctx, base);
	// 82ED29F0: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED29F4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82ED29F8: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 82ED29FC: 81790084  lwz r11, 0x84(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2A00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2A04: 419A0028  beq cr6, 0x82ed2a2c
	if ctx.cr[6].eq {
	pc = 0x82ED2A2C; continue 'dispatch;
	}
	// 82ED2A08: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED2A0C: 98810051  stb r4, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[4].u8 ) };
	// 82ED2A10: 9A410052  stb r18, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[18].u8 ) };
	// 82ED2A14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED2A18: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82ED2A1C: 8079007C  lwz r3, 0x7c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED2A20: 48014DE9  bl 0x82ee7808
	ctx.lr = 0x82ED2A24;
	sub_82EE7808(ctx, base);
	// 82ED2A24: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82ED2A28: 482D575C  b 0x831a8184
	sub_831A8180(ctx, base);
	return;
	// 82ED2A2C: 820D0000  lwz r16, 0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2A30: 39E00018  li r15, 0x18
	ctx.r[15].s64 = 24;
	// 82ED2A34: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 82ED2A38: 9B19008C  stb r24, 0x8c(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(140 as u32), ctx.r[24].u8 ) };
	// 82ED2A3C: 7D50782E  lwzx r10, r16, r15
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82ED2A40: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2A44: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2A48: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED2A4C: 40980020  bge cr6, 0x82ed2a6c
	if !ctx.cr[6].lt {
	pc = 0x82ED2A6C; continue 'dispatch;
	}
	// 82ED2A50: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED2A54: 39094694  addi r8, r9, 0x4694
	ctx.r[8].s64 = ctx.r[9].s64 + 18068;
	// 82ED2A58: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED2A5C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED2A60: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED2A64: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED2A68: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED2A6C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82ED2A70: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 82ED2A74: 409A018C  bne cr6, 0x82ed2c00
	if !ctx.cr[6].eq {
	pc = 0x82ED2C00; continue 'dispatch;
	}
	// 82ED2A78: 8179002C  lwz r11, 0x2c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED2A7C: 3B790028  addi r27, r25, 0x28
	ctx.r[27].s64 = ctx.r[25].s64 + 40;
	// 82ED2A80: 7E3A8B78  mr r26, r17
	ctx.r[26].u64 = ctx.r[17].u64;
	// 82ED2A84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2A88: 40990068  ble cr6, 0x82ed2af0
	if !ctx.cr[6].gt {
	pc = 0x82ED2AF0; continue 'dispatch;
	}
	// 82ED2A8C: 7E3C8B78  mr r28, r17
	ctx.r[28].u64 = ctx.r[17].u64;
	// 82ED2A90: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2A94: 7E3F8B78  mr r31, r17
	ctx.r[31].u64 = ctx.r[17].u64;
	// 82ED2A98: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED2A9C: 3BAB0048  addi r29, r11, 0x48
	ctx.r[29].s64 = ctx.r[11].s64 + 72;
	// 82ED2AA0: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED2AA4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED2AA8: 40990034  ble cr6, 0x82ed2adc
	if !ctx.cr[6].gt {
	pc = 0x82ED2ADC; continue 'dispatch;
	}
	// 82ED2AAC: 7E3E8B78  mr r30, r17
	ctx.r[30].u64 = ctx.r[17].u64;
	// 82ED2AB0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2AB4: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 82ED2AB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82ED2ABC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82ED2AC0: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED2AC4: 4BFFF055  bl 0x82ed1b18
	ctx.lr = 0x82ED2AC8;
	sub_82ED1B18(ctx, base);
	// 82ED2AC8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2ACC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82ED2AD0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ED2AD4: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED2AD8: 4198FFD8  blt cr6, 0x82ed2ab0
	if ctx.cr[6].lt {
	pc = 0x82ED2AB0; continue 'dispatch;
	}
	// 82ED2ADC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2AE0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82ED2AE4: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82ED2AE8: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED2AEC: 4198FFA4  blt cr6, 0x82ed2a90
	if ctx.cr[6].lt {
	pc = 0x82ED2A90; continue 'dispatch;
	}
	// 82ED2AF0: 81790038  lwz r11, 0x38(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ED2AF4: 7E3B8B78  mr r27, r17
	ctx.r[27].u64 = ctx.r[17].u64;
	// 82ED2AF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2AFC: 40990068  ble cr6, 0x82ed2b64
	if !ctx.cr[6].gt {
	pc = 0x82ED2B64; continue 'dispatch;
	}
	// 82ED2B00: 7E3C8B78  mr r28, r17
	ctx.r[28].u64 = ctx.r[17].u64;
	// 82ED2B04: 81790034  lwz r11, 0x34(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED2B08: 7E3F8B78  mr r31, r17
	ctx.r[31].u64 = ctx.r[17].u64;
	// 82ED2B0C: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED2B10: 3BAB0048  addi r29, r11, 0x48
	ctx.r[29].s64 = ctx.r[11].s64 + 72;
	// 82ED2B14: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED2B18: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED2B1C: 40990034  ble cr6, 0x82ed2b50
	if !ctx.cr[6].gt {
	pc = 0x82ED2B50; continue 'dispatch;
	}
	// 82ED2B20: 7E3E8B78  mr r30, r17
	ctx.r[30].u64 = ctx.r[17].u64;
	// 82ED2B24: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2B28: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 82ED2B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82ED2B30: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82ED2B34: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED2B38: 4BFFEFE1  bl 0x82ed1b18
	ctx.lr = 0x82ED2B3C;
	sub_82ED1B18(ctx, base);
	// 82ED2B3C: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2B40: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82ED2B44: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ED2B48: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED2B4C: 4198FFD8  blt cr6, 0x82ed2b24
	if ctx.cr[6].lt {
	pc = 0x82ED2B24; continue 'dispatch;
	}
	// 82ED2B50: 81790038  lwz r11, 0x38(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ED2B54: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82ED2B58: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82ED2B5C: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED2B60: 4198FFA4  blt cr6, 0x82ed2b04
	if ctx.cr[6].lt {
	pc = 0x82ED2B04; continue 'dispatch;
	}
	// 82ED2B64: 817900E8  lwz r11, 0xe8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ED2B68: 7E3E8B78  mr r30, r17
	ctx.r[30].u64 = ctx.r[17].u64;
	// 82ED2B6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2B70: 40990030  ble cr6, 0x82ed2ba0
	if !ctx.cr[6].gt {
	pc = 0x82ED2BA0; continue 'dispatch;
	}
	// 82ED2B74: 7E3F8B78  mr r31, r17
	ctx.r[31].u64 = ctx.r[17].u64;
	// 82ED2B78: 817900E4  lwz r11, 0xe4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ED2B7C: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 82ED2B80: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82ED2B84: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED2B88: 4BFFECE9  bl 0x82ed1870
	ctx.lr = 0x82ED2B8C;
	sub_82ED1870(ctx, base);
	// 82ED2B8C: 815900E8  lwz r10, 0xe8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ED2B90: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82ED2B94: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ED2B98: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED2B9C: 4198FFDC  blt cr6, 0x82ed2b78
	if ctx.cr[6].lt {
	pc = 0x82ED2B78; continue 'dispatch;
	}
	// 82ED2BA0: 81790084  lwz r11, 0x84(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2BA4: 9A39008C  stb r17, 0x8c(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(140 as u32), ctx.r[17].u8 ) };
	// 82ED2BA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2BAC: 409A001C  bne cr6, 0x82ed2bc8
	if !ctx.cr[6].eq {
	pc = 0x82ED2BC8; continue 'dispatch;
	}
	// 82ED2BB0: 81790080  lwz r11, 0x80(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED2BB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2BB8: 419A0010  beq cr6, 0x82ed2bc8
	if ctx.cr[6].eq {
	pc = 0x82ED2BC8; continue 'dispatch;
	}
	// 82ED2BBC: 92390080  stw r17, 0x80(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(128 as u32), ctx.r[17].u32 ) };
	// 82ED2BC0: 8079007C  lwz r3, 0x7c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED2BC4: 48015235  bl 0x82ee7df8
	ctx.lr = 0x82ED2BC8;
	sub_82EE7DF8(ctx, base);
	// 82ED2BC8: 7D50782E  lwzx r10, r16, r15
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82ED2BCC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2BD0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2BD4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED2BD8: 40980020  bge cr6, 0x82ed2bf8
	if !ctx.cr[6].lt {
	pc = 0x82ED2BF8; continue 'dispatch;
	}
	// 82ED2BDC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ED2BE0: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82ED2BE4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED2BE8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED2BEC: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED2BF0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED2BF4: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED2BF8: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82ED2BFC: 482D5588  b 0x831a8184
	sub_831A8180(ctx, base);
	return;
	// 82ED2C00: 81790084  lwz r11, 0x84(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2C04: 39590028  addi r10, r25, 0x28
	ctx.r[10].s64 = ctx.r[25].s64 + 40;
	// 82ED2C08: 39390034  addi r9, r25, 0x34
	ctx.r[9].s64 = ctx.r[25].s64 + 52;
	// 82ED2C0C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED2C10: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82ED2C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82ED2C18: 3A810058  addi r20, r1, 0x58
	ctx.r[20].s64 = ctx.r[1].s64 + 88;
	// 82ED2C1C: 91190084  stw r8, 0x84(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(132 as u32), ctx.r[8].u32 ) };
	// 82ED2C20: 3A600002  li r19, 2
	ctx.r[19].s64 = 2;
	// 82ED2C24: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82ED2C28: 82F40000  lwz r23, 0(r20)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2C2C: 3941006C  addi r10, r1, 0x6c
	ctx.r[10].s64 = ctx.r[1].s64 + 108;
	// 82ED2C30: 92210064  stw r17, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[17].u32 ) };
	// 82ED2C34: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82ED2C38: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82ED2C3C: 7E358B78  mr r21, r17
	ctx.r[21].u64 = ctx.r[17].u64;
	// 82ED2C40: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82ED2C44: 81370004  lwz r9, 4(r23)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2C48: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED2C4C: 40990194  ble cr6, 0x82ed2de0
	if !ctx.cr[6].gt {
	pc = 0x82ED2DE0; continue 'dispatch;
	}
	// 82ED2C50: 7E368B78  mr r22, r17
	ctx.r[22].u64 = ctx.r[17].u64;
	// 82ED2C54: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2C58: 7E2A8B78  mr r10, r17
	ctx.r[10].u64 = ctx.r[17].u64;
	// 82ED2C5C: 7E3B8B78  mr r27, r17
	ctx.r[27].u64 = ctx.r[17].u64;
	// 82ED2C60: 7FD6582E  lwzx r30, r22, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED2C64: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82ED2C68: 3B9E0058  addi r28, r30, 0x58
	ctx.r[28].s64 = ctx.r[30].s64 + 88;
	// 82ED2C6C: 813E0060  lwz r9, 0x60(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED2C70: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED2C74: 40990154  ble cr6, 0x82ed2dc8
	if !ctx.cr[6].gt {
	pc = 0x82ED2DC8; continue 'dispatch;
	}
	// 82ED2C78: 7E3A8B78  mr r26, r17
	ctx.r[26].u64 = ctx.r[17].u64;
	// 82ED2C7C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2C80: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82ED2C84: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED2C88: 7F1B4800  cmpw cr6, r27, r9
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED2C8C: 7FFA582E  lwzx r31, r26, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED2C90: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82ED2C94: 409A0010  bne cr6, 0x82ed2ca4
	if !ctx.cr[6].eq {
	pc = 0x82ED2CA4; continue 'dispatch;
	}
	// 82ED2C98: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2C9C: 7FABFA14  add r29, r11, r31
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82ED2CA0: 48000008  b 0x82ed2ca8
	pc = 0x82ED2CA8; continue 'dispatch;
	// 82ED2CA4: 3BBF0200  addi r29, r31, 0x200
	ctx.r[29].s64 = ctx.r[31].s64 + 512;
	// 82ED2CA8: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82ED2CAC: 409800E0  bge cr6, 0x82ed2d8c
	if !ctx.cr[6].lt {
	pc = 0x82ED2D8C; continue 'dispatch;
	}
	// 82ED2CB0: 81790070  lwz r11, 0x70(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED2CB4: 38610053  addi r3, r1, 0x53
	ctx.r[3].s64 = ctx.r[1].s64 + 83;
	// 82ED2CB8: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED2CBC: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82ED2CC0: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED2CC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED2CC8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2CCC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED2CD0: 4E800421  bctrl
	ctx.lr = 0x82ED2CD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2CD4: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2CD8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED2CDC: 419A0038  beq cr6, 0x82ed2d14
	if ctx.cr[6].eq {
	pc = 0x82ED2D14; continue 'dispatch;
	}
	// 82ED2CE0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED2CE4: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED2CE8: 81590074  lwz r10, 0x74(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED2CEC: A16B001A  lhz r11, 0x1a(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 82ED2CF0: A109001A  lhz r8, 0x1a(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(26 as u32) ) } as u64;
	// 82ED2CF4: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82ED2CF8: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82ED2CFC: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED2D00: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82ED2D04: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ED2D08: 88A61BB0  lbz r5, 0x1bb0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(7088 as u32) ) } as u64;
	// 82ED2D0C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82ED2D10: 409A0050  bne cr6, 0x82ed2d60
	if !ctx.cr[6].eq {
	pc = 0x82ED2D60; continue 'dispatch;
	}
	// 82ED2D14: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED2D18: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED2D1C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED2D20: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED2D24: 409A0010  bne cr6, 0x82ed2d34
	if !ctx.cr[6].eq {
	pc = 0x82ED2D34; continue 'dispatch;
	}
	// 82ED2D28: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED2D2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82ED2D30: 4BFD3B51  bl 0x82ea6880
	ctx.lr = 0x82ED2D34;
	sub_82EA6880(ctx, base);
	// 82ED2D34: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED2D38: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED2D3C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED2D40: 7FE9512E  stwx r31, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82ED2D44: 81010064  lwz r8, 0x64(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED2D48: 38E80001  addi r7, r8, 1
	ctx.r[7].s64 = ctx.r[8].s64 + 1;
	// 82ED2D4C: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82ED2D50: 88DE0025  lbz r6, 0x25(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(37 as u32) ) } as u64;
	// 82ED2D54: 53063032  rlwimi r6, r24, 6, 0, 0x19
	ctx.r[6].u64 = (((ctx.r[24].u32).rotate_left(6) as u64) & 0x00000000FFFFFFC0) | (ctx.r[6].u64 & 0xFFFFFFFF0000003F);
	// 82ED2D58: 98DE0025  stb r6, 0x25(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(37 as u32), ctx.r[6].u8 ) };
	// 82ED2D5C: 4800001C  b 0x82ed2d78
	pc = 0x82ED2D78; continue 'dispatch;
	// 82ED2D60: 2F120001  cmpwi cr6, r18, 1
	ctx.cr[6].compare_i32(ctx.r[18].s32, 1, &mut ctx.xer);
	// 82ED2D64: 409A0014  bne cr6, 0x82ed2d78
	if !ctx.cr[6].eq {
	pc = 0x82ED2D78; continue 'dispatch;
	}
	// 82ED2D68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82ED2D6C: 8099006C  lwz r4, 0x6c(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED2D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED2D74: 4809AE45  bl 0x82f6dbb8
	ctx.lr = 0x82ED2D78;
	sub_82F6DBB8(ctx, base);
	// 82ED2D78: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82ED2D7C: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82ED2D80: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82ED2D84: 4198FF2C  blt cr6, 0x82ed2cb0
	if ctx.cr[6].lt {
	pc = 0x82ED2CB0; continue 'dispatch;
	}
	// 82ED2D88: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED2D8C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED2D90: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED2D94: 4198FEE8  blt cr6, 0x82ed2c7c
	if ctx.cr[6].lt {
	pc = 0x82ED2C7C; continue 'dispatch;
	}
	// 82ED2D98: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED2D9C: 419A002C  beq cr6, 0x82ed2dc8
	if ctx.cr[6].eq {
	pc = 0x82ED2DC8; continue 'dispatch;
	}
	// 82ED2DA0: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED2DA4: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED2DA8: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 82ED2DAC: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82ED2DB0: 8067FFFC  lwz r3, -4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82ED2DB4: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82ED2DB8: 4801A5B1  bl 0x82eed368
	ctx.lr = 0x82ED2DBC;
	sub_82EED368(ctx, base);
	// 82ED2DBC: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED2DC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED2DC4: 409AFFDC  bne cr6, 0x82ed2da0
	if !ctx.cr[6].eq {
	pc = 0x82ED2DA0; continue 'dispatch;
	}
	// 82ED2DC8: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2DCC: 3AB50001  addi r21, r21, 1
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	// 82ED2DD0: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 82ED2DD4: 7F155800  cmpw cr6, r21, r11
	ctx.cr[6].compare_i32(ctx.r[21].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED2DD8: 4198FE7C  blt cr6, 0x82ed2c54
	if ctx.cr[6].lt {
	pc = 0x82ED2C54; continue 'dispatch;
	}
	// 82ED2DDC: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED2DE0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED2DE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED2DE8: 409A001C  bne cr6, 0x82ed2e04
	if !ctx.cr[6].eq {
	pc = 0x82ED2E04; continue 'dispatch;
	}
	// 82ED2DEC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED2DF0: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED2DF4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED2DF8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED2DFC: 7C70502E  lwzx r3, r16, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED2E00: 4BFCD9B1  bl 0x82ea07b0
	ctx.lr = 0x82ED2E04;
	sub_82EA07B0(ctx, base);
	// 82ED2E04: 3673FFFF  addic. r19, r19, -1
	ctx.xer.ca = (ctx.r[19].u32 > (!(-1 as u32)));
	ctx.r[19].s64 = ctx.r[19].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 82ED2E08: 3A940004  addi r20, r20, 4
	ctx.r[20].s64 = ctx.r[20].s64 + 4;
	// 82ED2E0C: 4082FE18  bne 0x82ed2c24
	if !ctx.cr[0].eq {
	pc = 0x82ED2C24; continue 'dispatch;
	}
	// 82ED2E10: 81790084  lwz r11, 0x84(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2E14: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82ED2E18: 91790084  stw r11, 0x84(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED2E1C: 4BFFFD84  b 0x82ed2ba0
	pc = 0x82ED2BA0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED2E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ED2E20 size=1816
    let mut pc: u32 = 0x82ED2E20;
    'dispatch: loop {
        match pc {
            0x82ED2E20 => {
    //   block [0x82ED2E20..0x82ED3538)
	// 82ED2E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED2E24: 482D530D  bl 0x831a8130
	ctx.lr = 0x82ED2E28;
	sub_831A8130(ctx, base);
	// 82ED2E28: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82ED2E2C: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82ED2E30: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED2E34: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 82ED2E38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED2E3C: 7C8E2378  mr r14, r4
	ctx.r[14].u64 = ctx.r[4].u64;
	// 82ED2E40: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 82ED2E44: 2F120000  cmpwi cr6, r18, 0
	ctx.cr[6].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 82ED2E48: 419A06E0  beq cr6, 0x82ed3528
	if ctx.cr[6].eq {
	pc = 0x82ED3528; continue 'dispatch;
	}
	// 82ED2E4C: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2E50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED2E54: 419A0034  beq cr6, 0x82ed2e88
	if ctx.cr[6].eq {
	pc = 0x82ED2E88; continue 'dispatch;
	}
	// 82ED2E58: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82ED2E5C: 91C10054  stw r14, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[14].u32 ) };
	// 82ED2E60: B2410058  sth r18, 0x58(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[18].u16 ) };
	// 82ED2E64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED2E68: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED2E6C: 9A01005A  stb r16, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[16].u8 ) };
	// 82ED2E70: 807D007C  lwz r3, 0x7c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED2E74: 48014995  bl 0x82ee7808
	ctx.lr = 0x82ED2E78;
	sub_82EE7808(ctx, base);
	// 82ED2E78: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82ED2E7C: CBC1FF58  lfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-168 as u32) ) };
	// 82ED2E80: CBE1FF60  lfd f31, -0xa0(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) };
	// 82ED2E84: 482D52FC  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 82ED2E88: 828D0000  lwz r20, 0(r13)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2E8C: 39E00018  li r15, 0x18
	ctx.r[15].s64 = 24;
	// 82ED2E90: 7D54782E  lwzx r10, r20, r15
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82ED2E94: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED2E98: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED2E9C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED2EA0: 4098002C  bge cr6, 0x82ed2ecc
	if !ctx.cr[6].lt {
	pc = 0x82ED2ECC; continue 'dispatch;
	}
	// 82ED2EA4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED2EA8: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED2EAC: 38E946D8  addi r7, r9, 0x46d8
	ctx.r[7].s64 = ctx.r[9].s64 + 18136;
	// 82ED2EB0: 38C84628  addi r6, r8, 0x4628
	ctx.r[6].s64 = ctx.r[8].s64 + 17960;
	// 82ED2EB4: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED2EB8: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82ED2EBC: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82ED2EC0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82ED2EC4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED2EC8: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82ED2ECC: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED2ED0: 3A200014  li r17, 0x14
	ctx.r[17].s64 = 20;
	// 82ED2ED4: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82ED2ED8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED2EDC: 3E608000  lis r19, -0x8000
	ctx.r[19].s64 = -2147483648;
	// 82ED2EE0: 92E10060  stw r23, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 82ED2EE4: 917D0084  stw r11, 0x84(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED2EE8: 39520004  addi r10, r18, 4
	ctx.r[10].s64 = ctx.r[18].s64 + 4;
	// 82ED2EEC: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED2EF0: 92E10064  stw r23, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[23].u32 ) };
	// 82ED2EF4: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED2EF8: 92610068  stw r19, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[19].u32 ) };
	// 82ED2EFC: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED2F00: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED2F04: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED2F08: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED2F0C: 41990010  bgt cr6, 0x82ed2f1c
	if ctx.cr[6].gt {
	pc = 0x82ED2F1C; continue 'dispatch;
	}
	// 82ED2F10: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED2F14: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED2F18: 48000014  b 0x82ed2f2c
	pc = 0x82ED2F2C; continue 'dispatch;
	// 82ED2F1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2F20: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED2F24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED2F28: 4E800421  bctrl
	ctx.lr = 0x82ED2F2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2F2C: 7E5F9B78  or r31, r18, r19
	ctx.r[31].u64 = ctx.r[18].u64 | ctx.r[19].u64;
	// 82ED2F30: 92E10070  stw r23, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[23].u32 ) };
	// 82ED2F34: 92610078  stw r19, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[19].u32 ) };
	// 82ED2F38: 564B2834  slwi r11, r18, 5
	ctx.r[11].u32 = ctx.r[18].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED2F3C: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82ED2F40: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82ED2F44: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82ED2F48: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82ED2F4C: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED2F50: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED2F54: 92E10074  stw r23, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[23].u32 ) };
	// 82ED2F58: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED2F5C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED2F60: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED2F64: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED2F68: 4199000C  bgt cr6, 0x82ed2f74
	if ctx.cr[6].gt {
	pc = 0x82ED2F74; continue 'dispatch;
	}
	// 82ED2F6C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED2F70: 48000018  b 0x82ed2f88
	pc = 0x82ED2F88; continue 'dispatch;
	// 82ED2F74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED2F78: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED2F7C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED2F80: 4E800421  bctrl
	ctx.lr = 0x82ED2F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED2F84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED2F88: 895D00B8  lbz r10, 0xb8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED2F8C: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 82ED2F90: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82ED2F94: 7EF5BB78  mr r21, r23
	ctx.r[21].u64 = ctx.r[23].u64;
	// 82ED2F98: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 82ED2F9C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED2FA0: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82ED2FA4: 419A0070  beq cr6, 0x82ed3014
	if ctx.cr[6].eq {
	pc = 0x82ED3014; continue 'dispatch;
	}
	// 82ED2FA8: 38A00032  li r5, 0x32
	ctx.r[5].s64 = 50;
	// 82ED2FAC: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED2FB0: 3880006C  li r4, 0x6c
	ctx.r[4].s64 = 108;
	// 82ED2FB4: 4BFCD77D  bl 0x82ea0730
	ctx.lr = 0x82ED2FB8;
	sub_82EA0730(ctx, base);
	// 82ED2FB8: 3960006C  li r11, 0x6c
	ctx.r[11].s64 = 108;
	// 82ED2FBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED2FC0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82ED2FC4: 48019915  bl 0x82eec8d8
	ctx.lr = 0x82ED2FC8;
	sub_82EEC8D8(ctx, base);
	// 82ED2FC8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ED2FCC: 2F100001  cmpwi cr6, r16, 1
	ctx.cr[6].compare_i32(ctx.r[16].s32, 1, &mut ctx.xer);
	// 82ED2FD0: 895B0025  lbz r10, 0x25(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(37 as u32) ) } as u64;
	// 82ED2FD4: 52CA3032  rlwimi r10, r22, 6, 0, 0x19
	ctx.r[10].u64 = (((ctx.r[22].u32).rotate_left(6) as u64) & 0x00000000FFFFFFC0) | (ctx.r[10].u64 & 0xFFFFFFFF0000003F);
	// 82ED2FD8: 995B0025  stb r10, 0x25(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(37 as u32), ctx.r[10].u8 ) };
	// 82ED2FDC: 409A0020  bne cr6, 0x82ed2ffc
	if !ctx.cr[6].eq {
	pc = 0x82ED2FFC; continue 'dispatch;
	}
	// 82ED2FE0: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED2FE4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82ED2FE8: 893B0026  lbz r9, 0x26(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(38 as u32) ) } as u64;
	// 82ED2FEC: 51492036  rlwimi r9, r10, 4, 0, 0x1b
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(4) as u64) & 0x00000000FFFFFFF0) | (ctx.r[9].u64 & 0xFFFFFFFF0000000F);
	// 82ED2FF0: B17B0020  sth r11, 0x20(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u16 ) };
	// 82ED2FF4: 993B0026  stb r9, 0x26(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(38 as u32), ctx.r[9].u8 ) };
	// 82ED2FF8: 4800005C  b 0x82ed3054
	pc = 0x82ED3054; continue 'dispatch;
	// 82ED2FFC: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ED3000: 895B0026  lbz r10, 0x26(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(38 as u32) ) } as u64;
	// 82ED3004: 5548073E  clrlwi r8, r10, 0x1c
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82ED3008: B17B0020  sth r11, 0x20(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u16 ) };
	// 82ED300C: 991B0026  stb r8, 0x26(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(38 as u32), ctx.r[8].u8 ) };
	// 82ED3010: 48000044  b 0x82ed3054
	pc = 0x82ED3054; continue 'dispatch;
	// 82ED3014: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED3018: 836B0000  lwz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED301C: 387B0048  addi r3, r27, 0x48
	ctx.r[3].s64 = ctx.r[27].s64 + 72;
	// 82ED3020: 817B004C  lwz r11, 0x4c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED3024: 815B0050  lwz r10, 0x50(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED3028: 7D6B9214  add r11, r11, r18
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 82ED302C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED3030: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED3034: 40980020  bge cr6, 0x82ed3054
	if !ctx.cr[6].lt {
	pc = 0x82ED3054; continue 'dispatch;
	}
	// 82ED3038: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED303C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED3040: 40980008  bge cr6, 0x82ed3048
	if !ctx.cr[6].lt {
	pc = 0x82ED3048; continue 'dispatch;
	}
	// 82ED3044: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82ED3048: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED304C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82ED3050: 4BFD37A9  bl 0x82ea67f8
	ctx.lr = 0x82ED3054;
	sub_82EA67F8(ctx, base);
	// 82ED3054: 817D006C  lwz r11, 0x6c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED3058: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82ED305C: 2F120000  cmpwi cr6, r18, 0
	ctx.cr[6].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 82ED3060: C1AB0004  lfs f13, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED3064: C00A9450  lfs f0, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED3068: EFCD0032  fmuls f30, f13, f0
	ctx.f[30].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82ED306C: 40990180  ble cr6, 0x82ed31ec
	if !ctx.cr[6].gt {
	pc = 0x82ED31EC; continue 'dispatch;
	}
	// 82ED3070: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82ED3074: 3B3D02C5  addi r25, r29, 0x2c5
	ctx.r[25].s64 = ctx.r[29].s64 + 709;
	// 82ED3078: 7DDA7378  mr r26, r14
	ctx.r[26].u64 = ctx.r[14].u64;
	// 82ED307C: 7E589378  mr r24, r18
	ctx.r[24].u64 = ctx.r[18].u64;
	// 82ED3080: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82ED3084: 817D00E0  lwz r11, 0xe0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(224 as u32) ) } as u64;
	// 82ED3088: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED308C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED3090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3094: 917D00E0  stw r11, 0xe0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82ED3098: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82ED309C: 4800204D  bl 0x82ed50e8
	ctx.lr = 0x82ED30A0;
	sub_82ED50E8(ctx, base);
	// 82ED30A0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED30A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED30A8: 409A001C  bne cr6, 0x82ed30c4
	if !ctx.cr[6].eq {
	pc = 0x82ED30C4; continue 'dispatch;
	}
	// 82ED30AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED30B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED30B4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED30B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED30BC: 4E800421  bctrl
	ctx.lr = 0x82ED30C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED30C0: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82ED30C4: 38BF00E0  addi r5, r31, 0xe0
	ctx.r[5].s64 = ctx.r[31].s64 + 224;
	// 82ED30C8: FC40F890  fmr f2, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82ED30CC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82ED30D0: 4832B429  bl 0x831fe4f8
	ctx.lr = 0x82ED30D4;
	sub_831FE4F8(ctx, base);
	// 82ED30D4: B2FF00DA  sth r23, 0xda(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(218 as u32), ctx.r[23].u16 ) };
	// 82ED30D8: B2FF00DC  sth r23, 0xdc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[23].u16 ) };
	// 82ED30DC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82ED30E0: 897D02C7  lbz r11, 0x2c7(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(711 as u32) ) } as u64;
	// 82ED30E4: 895F00D9  lbz r10, 0xd9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(217 as u32) ) } as u64;
	// 82ED30E8: 554607BE  clrlwi r6, r10, 0x1e
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82ED30EC: 556707BE  clrlwi r7, r11, 0x1e
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82ED30F0: 89190001  lbz r8, 1(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(1 as u32) ) } as u64;
	// 82ED30F4: 89390000  lbz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED30F8: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82ED30FC: 40980008  bge cr6, 0x82ed3104
	if !ctx.cr[6].lt {
	pc = 0x82ED3104; continue 'dispatch;
	}
	// 82ED3100: 7D2948F8  nor r9, r9, r9
	ctx.r[9].u64 = !(ctx.r[9].u64 | ctx.r[9].u64);
	// 82ED3104: 55297022  slwi r9, r9, 0xe
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(14);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED3108: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED310C: 40980010  bge cr6, 0x82ed311c
	if !ctx.cr[6].lt {
	pc = 0x82ED311C; continue 'dispatch;
	}
	// 82ED3110: 7D0B40F8  nor r11, r8, r8
	ctx.r[11].u64 = !(ctx.r[8].u64 | ctx.r[8].u64);
	// 82ED3114: 556B7022  slwi r11, r11, 0xe
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED3118: 48000008  b 0x82ed3120
	pc = 0x82ED3120; continue 'dispatch;
	// 82ED311C: 550B7022  slwi r11, r8, 0xe
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED3120: A15F00DC  lhz r10, 0xdc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 82ED3124: A11F00DA  lhz r8, 0xda(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(218 as u32) ) } as u64;
	// 82ED3128: 554704BE  clrlwi r7, r10, 0x12
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 82ED312C: 550604BE  clrlwi r6, r8, 0x12
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00003FFFu64;
	// 82ED3130: 7CE55B78  or r5, r7, r11
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 82ED3134: 7CC44B78  or r4, r6, r9
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[9].u64;
	// 82ED3138: B0BF00DC  sth r5, 0xdc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[5].u16 ) };
	// 82ED313C: B09F00DA  sth r4, 0xda(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(218 as u32), ctx.r[4].u16 ) };
	// 82ED3140: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED3144: 895F00D8  lbz r10, 0xd8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ED3148: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 82ED314C: 409A000C  bne cr6, 0x82ed3158
	if !ctx.cr[6].eq {
	pc = 0x82ED3158; continue 'dispatch;
	}
	// 82ED3150: 807D0020  lwz r3, 0x20(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED3154: 4800000C  b 0x82ed3160
	pc = 0x82ED3160; continue 'dispatch;
	// 82ED3158: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED315C: 7ED5B378  mr r21, r22
	ctx.r[21].u64 = ctx.r[22].u64;
	// 82ED3160: 48018F11  bl 0x82eec070
	ctx.lr = 0x82ED3164;
	sub_82EEC070(ctx, base);
	// 82ED3164: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82ED3168: 839F0010  lwz r28, 0x10(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED316C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82ED3170: 419A0068  beq cr6, 0x82ed31d8
	if ctx.cr[6].eq {
	pc = 0x82ED31D8; continue 'dispatch;
	}
	// 82ED3174: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED3178: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82ED317C: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED3180: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82ED3184: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED3188: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82ED318C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3190: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ED3194: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED3198: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED319C: 80E8001C  lwz r7, 0x1c(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ED31A0: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82ED31A4: 4E800421  bctrl
	ctx.lr = 0x82ED31A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED31A8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED31AC: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED31B0: 38DE0014  addi r6, r30, 0x14
	ctx.r[6].s64 = ctx.r[30].s64 + 20;
	// 82ED31B4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED31B8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82ED31BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED31C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED31C4: 7CC9512E  stwx r6, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[6].u32) };
	// 82ED31C8: 81010064  lwz r8, 0x64(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED31CC: 38E80001  addi r7, r8, 1
	ctx.r[7].s64 = ctx.r[8].s64 + 1;
	// 82ED31D0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82ED31D4: 48005875  bl 0x82ed8a48
	ctx.lr = 0x82ED31D8;
	sub_82ED8A48(ctx, base);
	// 82ED31D8: 3718FFFF  addic. r24, r24, -1
	ctx.xer.ca = (ctx.r[24].u32 > (!(-1 as u32)));
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82ED31DC: 92DE0020  stw r22, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[22].u32 ) };
	// 82ED31E0: 92FE0030  stw r23, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[23].u32 ) };
	// 82ED31E4: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82ED31E8: 4082FE9C  bne 0x82ed3084
	if !ctx.cr[0].eq {
	pc = 0x82ED3084; continue 'dispatch;
	}
	// 82ED31EC: 897D00B8  lbz r11, 0xb8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED31F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED31F4: 419A0088  beq cr6, 0x82ed327c
	if ctx.cr[6].eq {
	pc = 0x82ED327C; continue 'dispatch;
	}
	// 82ED31F8: 56AB063E  clrlwi r11, r21, 0x18
	ctx.r[11].u64 = ctx.r[21].u32 as u64 & 0x000000FFu64;
	// 82ED31FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED3200: 419A0064  beq cr6, 0x82ed3264
	if ctx.cr[6].eq {
	pc = 0x82ED3264; continue 'dispatch;
	}
	// 82ED3204: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED3208: 2F100001  cmpwi cr6, r16, 1
	ctx.cr[6].compare_i32(ctx.r[16].s32, 1, &mut ctx.xer);
	// 82ED320C: 3BFD0028  addi r31, r29, 0x28
	ctx.r[31].s64 = ctx.r[29].s64 + 40;
	// 82ED3210: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3214: 812A00C0  lwz r9, 0xc0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(192 as u32) ) } as u64;
	// 82ED3218: 993B0024  stb r9, 0x24(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[9].u8 ) };
	// 82ED321C: 419A0008  beq cr6, 0x82ed3224
	if ctx.cr[6].eq {
	pc = 0x82ED3224; continue 'dispatch;
	}
	// 82ED3220: 3BFD0034  addi r31, r29, 0x34
	ctx.r[31].s64 = ctx.r[29].s64 + 52;
	// 82ED3224: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED3228: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED322C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED3230: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED3234: 409A0010  bne cr6, 0x82ed3244
	if !ctx.cr[6].eq {
	pc = 0x82ED3244; continue 'dispatch;
	}
	// 82ED3238: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED323C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3240: 4BFD3641  bl 0x82ea6880
	ctx.lr = 0x82ED3244;
	sub_82EA6880(ctx, base);
	// 82ED3244: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3248: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED324C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED3250: 7F69512E  stwx r27, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u32) };
	// 82ED3254: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3258: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED325C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED3260: 4800001C  b 0x82ed327c
	pc = 0x82ED327C; continue 'dispatch;
	// 82ED3264: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3268: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED326C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED3270: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3274: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3278: 4E800421  bctrl
	ctx.lr = 0x82ED327C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED327C: 92E10080  stw r23, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[23].u32 ) };
	// 82ED3280: 92E10084  stw r23, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[23].u32 ) };
	// 82ED3284: 92610088  stw r19, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[19].u32 ) };
	// 82ED3288: 83FD02F8  lwz r31, 0x2f8(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(760 as u32) ) } as u64;
	// 82ED328C: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED3290: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82ED3294: 55641836  rlwinm r4, r11, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82ED3298: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED329C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED32A0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED32A4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED32A8: 4199000C  bgt cr6, 0x82ed32b4
	if ctx.cr[6].gt {
	pc = 0x82ED32B4; continue 'dispatch;
	}
	// 82ED32AC: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED32B0: 48000018  b 0x82ed32c8
	pc = 0x82ED32C8; continue 'dispatch;
	// 82ED32B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED32B8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED32BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED32C0: 4E800421  bctrl
	ctx.lr = 0x82ED32C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED32C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED32C8: 7FEA9B78  or r10, r31, r19
	ctx.r[10].u64 = ctx.r[31].u64 | ctx.r[19].u64;
	// 82ED32CC: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ED32D0: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82ED32D4: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82ED32D8: 7D74782E  lwzx r11, r20, r15
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82ED32DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED32E0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED32E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED32E8: 40980020  bge cr6, 0x82ed3308
	if !ctx.cr[6].lt {
	pc = 0x82ED3308; continue 'dispatch;
	}
	// 82ED32EC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED32F0: 390946C8  addi r8, r9, 0x46c8
	ctx.r[8].s64 = ctx.r[9].s64 + 18120;
	// 82ED32F4: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED32F8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED32FC: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82ED3300: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED3304: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED3308: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED330C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82ED3310: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82ED3314: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82ED3318: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED331C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3320: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3324: 4E800421  bctrl
	ctx.lr = 0x82ED3328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3328: 7D54782E  lwzx r10, r20, r15
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82ED332C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3330: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED3334: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED3338: 40980020  bge cr6, 0x82ed3358
	if !ctx.cr[6].lt {
	pc = 0x82ED3358; continue 'dispatch;
	}
	// 82ED333C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED3340: 390946B8  addi r8, r9, 0x46b8
	ctx.r[8].s64 = ctx.r[9].s64 + 18104;
	// 82ED3344: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED3348: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED334C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED3350: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED3354: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED3358: 817D0070  lwz r11, 0x70(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED335C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED3360: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82ED3364: 409A0008  bne cr6, 0x82ed336c
	if !ctx.cr[6].eq {
	pc = 0x82ED336C; continue 'dispatch;
	}
	// 82ED3368: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82ED336C: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED3370: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3374: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED3378: 480505E9  bl 0x82f23960
	ctx.lr = 0x82ED337C;
	sub_82F23960(ctx, base);
	// 82ED337C: 7D54782E  lwzx r10, r20, r15
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82ED3380: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3384: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED3388: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED338C: 40980020  bge cr6, 0x82ed33ac
	if !ctx.cr[6].lt {
	pc = 0x82ED33AC; continue 'dispatch;
	}
	// 82ED3390: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED3394: 390946AC  addi r8, r9, 0x46ac
	ctx.r[8].s64 = ctx.r[9].s64 + 18092;
	// 82ED3398: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED339C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED33A0: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED33A4: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED33A8: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED33AC: 2F120000  cmpwi cr6, r18, 0
	ctx.cr[6].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 82ED33B0: 40990030  ble cr6, 0x82ed33e0
	if !ctx.cr[6].gt {
	pc = 0x82ED33E0; continue 'dispatch;
	}
	// 82ED33B4: 7DDE7378  mr r30, r14
	ctx.r[30].u64 = ctx.r[14].u64;
	// 82ED33B8: 7E5F9378  mr r31, r18
	ctx.r[31].u64 = ctx.r[18].u64;
	// 82ED33BC: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED33C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED33C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ED33C8: 480106B9  bl 0x82ee3a80
	ctx.lr = 0x82ED33CC;
	sub_82EE3A80(ctx, base);
	// 82ED33CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED33D0: 4800FBA1  bl 0x82ee2f70
	ctx.lr = 0x82ED33D4;
	sub_82EE2F70(ctx, base);
	// 82ED33D4: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82ED33D8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ED33DC: 4082FFE0  bne 0x82ed33bc
	if !ctx.cr[0].eq {
	pc = 0x82ED33BC; continue 'dispatch;
	}
	// 82ED33E0: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED33E4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED33E8: 917D0084  stw r11, 0x84(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED33EC: 40820028  bne 0x82ed3414
	if !ctx.cr[0].eq {
	pc = 0x82ED3414; continue 'dispatch;
	}
	// 82ED33F0: 897D008C  lbz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED33F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED33F8: 409A001C  bne cr6, 0x82ed3414
	if !ctx.cr[6].eq {
	pc = 0x82ED3414; continue 'dispatch;
	}
	// 82ED33FC: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED3400: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3404: 419A0010  beq cr6, 0x82ed3414
	if ctx.cr[6].eq {
	pc = 0x82ED3414; continue 'dispatch;
	}
	// 82ED3408: 92FD0080  stw r23, 0x80(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(128 as u32), ctx.r[23].u32 ) };
	// 82ED340C: 807D007C  lwz r3, 0x7c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3410: 480149E9  bl 0x82ee7df8
	ctx.lr = 0x82ED3414;
	sub_82EE7DF8(ctx, base);
	// 82ED3414: 7D54782E  lwzx r10, r20, r15
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82ED3418: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED341C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3420: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED3424: 40980020  bge cr6, 0x82ed3444
	if !ctx.cr[6].lt {
	pc = 0x82ED3444; continue 'dispatch;
	}
	// 82ED3428: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82ED342C: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82ED3430: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED3434: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED3438: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED343C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED3440: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED3444: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED3448: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED344C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED3450: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED3454: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED3458: 409A0014  bne cr6, 0x82ed346c
	if !ctx.cr[6].eq {
	pc = 0x82ED346C; continue 'dispatch;
	}
	// 82ED345C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3460: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3464: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3468: 4E800421  bctrl
	ctx.lr = 0x82ED346C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED346C: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED3470: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED3474: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED3478: 409A0018  bne cr6, 0x82ed3490
	if !ctx.cr[6].eq {
	pc = 0x82ED3490; continue 'dispatch;
	}
	// 82ED347C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED3480: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED3484: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED3488: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED348C: 4BFCD325  bl 0x82ea07b0
	ctx.lr = 0x82ED3490;
	sub_82EA07B0(ctx, base);
	// 82ED3490: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED3494: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3498: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED349C: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED34A0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED34A4: 409A0014  bne cr6, 0x82ed34b8
	if !ctx.cr[6].eq {
	pc = 0x82ED34B8; continue 'dispatch;
	}
	// 82ED34A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED34AC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED34B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED34B4: 4E800421  bctrl
	ctx.lr = 0x82ED34B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED34B8: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED34BC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED34C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED34C4: 409A0018  bne cr6, 0x82ed34dc
	if !ctx.cr[6].eq {
	pc = 0x82ED34DC; continue 'dispatch;
	}
	// 82ED34C8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED34CC: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED34D0: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED34D4: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED34D8: 4BFCD2D9  bl 0x82ea07b0
	ctx.lr = 0x82ED34DC;
	sub_82EA07B0(ctx, base);
	// 82ED34DC: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED34E0: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED34E4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED34E8: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED34EC: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED34F0: 409A0014  bne cr6, 0x82ed3504
	if !ctx.cr[6].eq {
	pc = 0x82ED3504; continue 'dispatch;
	}
	// 82ED34F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED34F8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED34FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3500: 4E800421  bctrl
	ctx.lr = 0x82ED3504;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3504: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED3508: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED350C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED3510: 409A0018  bne cr6, 0x82ed3528
	if !ctx.cr[6].eq {
	pc = 0x82ED3528; continue 'dispatch;
	}
	// 82ED3514: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED3518: 7C74882E  lwzx r3, r20, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82ED351C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED3520: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED3524: 4BFCD28D  bl 0x82ea07b0
	ctx.lr = 0x82ED3528;
	sub_82EA07B0(ctx, base);
	// 82ED3528: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82ED352C: CBC1FF58  lfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-168 as u32) ) };
	// 82ED3530: CBE1FF60  lfd f31, -0xa0(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) };
	// 82ED3534: 482D4C4C  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED3538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED3538 size=1068
    let mut pc: u32 = 0x82ED3538;
    'dispatch: loop {
        match pc {
            0x82ED3538 => {
    //   block [0x82ED3538..0x82ED3964)
	// 82ED3538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED353C: 482D4C19  bl 0x831a8154
	ctx.lr = 0x82ED3540;
	sub_831A8130(ctx, base);
	// 82ED3540: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED3544: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82ED3548: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ED354C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED3550: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82ED3554: 41980408  blt cr6, 0x82ed395c
	if ctx.cr[6].lt {
	pc = 0x82ED395C; continue 'dispatch;
	}
	// 82ED3558: 817B0084  lwz r11, 0x84(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED355C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3560: 419A0028  beq cr6, 0x82ed3588
	if ctx.cr[6].eq {
	pc = 0x82ED3588; continue 'dispatch;
	}
	// 82ED3564: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82ED3568: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82ED356C: B3C10060  sth r30, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u16 ) };
	// 82ED3570: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82ED3574: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82ED3578: 807B007C  lwz r3, 0x7c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED357C: 4801428D  bl 0x82ee7808
	ctx.lr = 0x82ED3580;
	sub_82EE7808(ctx, base);
	// 82ED3580: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82ED3584: 482D4C20  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 82ED3588: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED358C: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82ED3590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ED3594: 913B0084  stw r9, 0x84(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82ED3598: 7D5DB82E  lwzx r10, r29, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED359C: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED35A0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED35A4: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82ED35A8: 4098002C  bge cr6, 0x82ed35d4
	if !ctx.cr[6].lt {
	pc = 0x82ED35D4; continue 'dispatch;
	}
	// 82ED35AC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED35B0: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82ED35B4: 38C84720  addi r6, r8, 0x4720
	ctx.r[6].s64 = ctx.r[8].s64 + 18208;
	// 82ED35B8: 38A74710  addi r5, r7, 0x4710
	ctx.r[5].s64 = ctx.r[7].s64 + 18192;
	// 82ED35BC: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82ED35C0: 90AB000C  stw r5, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82ED35C4: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82ED35C8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82ED35CC: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82ED35D0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED35D4: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82ED35D8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82ED35DC: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED35E0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82ED35E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82ED35E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED35EC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED35F0: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED35F4: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED35F8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED35FC: 4E800421  bctrl
	ctx.lr = 0x82ED3600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3600: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82ED3604: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82ED3608: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 82ED360C: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82ED3610: 391E0004  addi r8, r30, 4
	ctx.r[8].s64 = ctx.r[30].s64 + 4;
	// 82ED3614: 93210074  stw r25, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82ED3618: 7C7DD02E  lwzx r3, r29, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED361C: 55041036  rlwinm r4, r8, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED3620: 93810078  stw r28, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[28].u32 ) };
	// 82ED3624: 80E3002C  lwz r7, 0x2c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED3628: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED362C: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED3630: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82ED3634: 4199000C  bgt cr6, 0x82ed3640
	if ctx.cr[6].gt {
	pc = 0x82ED3640; continue 'dispatch;
	}
	// 82ED3638: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED363C: 48000018  b 0x82ed3654
	pc = 0x82ED3654; continue 'dispatch;
	// 82ED3640: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3644: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED3648: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED364C: 4E800421  bctrl
	ctx.lr = 0x82ED3650;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3650: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED3654: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED3658: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82ED365C: 7FC9E378  or r9, r30, r28
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[28].u64;
	// 82ED3660: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82ED3664: 7F0AFA14  add r24, r10, r31
	ctx.r[24].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82ED3668: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 82ED366C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82ED3670: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82ED3674: 419A0044  beq cr6, 0x82ed36b8
	if ctx.cr[6].eq {
	pc = 0x82ED36B8; continue 'dispatch;
	}
	// 82ED3678: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED367C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3680: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82ED3684: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3688: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED368C: 419A0020  beq cr6, 0x82ed36ac
	if ctx.cr[6].eq {
	pc = 0x82ED36AC; continue 'dispatch;
	}
	// 82ED3690: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED3694: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED3698: 38CB0014  addi r6, r11, 0x14
	ctx.r[6].s64 = ctx.r[11].s64 + 20;
	// 82ED369C: 7CC7412E  stwx r6, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u32) };
	// 82ED36A0: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED36A4: 39250001  addi r9, r5, 1
	ctx.r[9].s64 = ctx.r[5].s64 + 1;
	// 82ED36A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82ED36AC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED36B0: 7F0AC040  cmplw cr6, r10, r24
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82ED36B4: 409AFFC8  bne cr6, 0x82ed367c
	if !ctx.cr[6].eq {
	pc = 0x82ED367C; continue 'dispatch;
	}
	// 82ED36B8: 7D5DB82E  lwzx r10, r29, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED36BC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED36C0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED36C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED36C8: 40980020  bge cr6, 0x82ed36e8
	if !ctx.cr[6].lt {
	pc = 0x82ED36E8; continue 'dispatch;
	}
	// 82ED36CC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED36D0: 39094700  addi r8, r9, 0x4700
	ctx.r[8].s64 = ctx.r[9].s64 + 18176;
	// 82ED36D4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED36D8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED36DC: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED36E0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED36E4: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED36E8: 93210084  stw r25, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[25].u32 ) };
	// 82ED36EC: 83DB02F8  lwz r30, 0x2f8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(760 as u32) ) } as u64;
	// 82ED36F0: 7C7DD02E  lwzx r3, r29, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED36F4: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82ED36F8: 93810088  stw r28, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[28].u32 ) };
	// 82ED36FC: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82ED3700: 55641836  rlwinm r4, r11, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82ED3704: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED3708: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED370C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED3710: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED3714: 4199000C  bgt cr6, 0x82ed3720
	if ctx.cr[6].gt {
	pc = 0x82ED3720; continue 'dispatch;
	}
	// 82ED3718: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED371C: 48000018  b 0x82ed3734
	pc = 0x82ED3734; continue 'dispatch;
	// 82ED3720: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3724: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED3728: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED372C: 4E800421  bctrl
	ctx.lr = 0x82ED3730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3730: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED3734: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ED3738: 7FCAE378  or r10, r30, r28
	ctx.r[10].u64 = ctx.r[30].u64 | ctx.r[28].u64;
	// 82ED373C: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82ED3740: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82ED3744: 807B0054  lwz r3, 0x54(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED3748: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82ED374C: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82ED3750: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3754: 81090020  lwz r8, 0x20(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED3758: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ED375C: 4E800421  bctrl
	ctx.lr = 0x82ED3760;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3760: 7D7DB82E  lwzx r11, r29, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED3764: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3768: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED376C: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82ED3770: 40980020  bge cr6, 0x82ed3790
	if !ctx.cr[6].lt {
	pc = 0x82ED3790; continue 'dispatch;
	}
	// 82ED3774: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED3778: 390946F4  addi r8, r9, 0x46f4
	ctx.r[8].s64 = ctx.r[9].s64 + 18164;
	// 82ED377C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED3780: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED3784: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82ED3788: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED378C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED3790: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3794: 807B0058  lwz r3, 0x58(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED3798: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED379C: 4805027D  bl 0x82f23a18
	ctx.lr = 0x82ED37A0;
	sub_82F23A18(ctx, base);
	// 82ED37A0: 7C7DD02E  lwzx r3, r29, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED37A4: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED37A8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED37AC: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED37B0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED37B4: 409A0014  bne cr6, 0x82ed37c8
	if !ctx.cr[6].eq {
	pc = 0x82ED37C8; continue 'dispatch;
	}
	// 82ED37B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED37BC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED37C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED37C4: 4E800421  bctrl
	ctx.lr = 0x82ED37C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED37C8: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED37CC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED37D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED37D4: 409A0018  bne cr6, 0x82ed37ec
	if !ctx.cr[6].eq {
	pc = 0x82ED37EC; continue 'dispatch;
	}
	// 82ED37D8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED37DC: 7C7DD02E  lwzx r3, r29, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED37E0: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED37E4: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED37E8: 4BFCCFC9  bl 0x82ea07b0
	ctx.lr = 0x82ED37EC;
	sub_82EA07B0(ctx, base);
	// 82ED37EC: 7C7DD02E  lwzx r3, r29, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED37F0: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED37F4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED37F8: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED37FC: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED3800: 409A0014  bne cr6, 0x82ed3814
	if !ctx.cr[6].eq {
	pc = 0x82ED3814; continue 'dispatch;
	}
	// 82ED3804: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3808: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED380C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3810: 4E800421  bctrl
	ctx.lr = 0x82ED3814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3814: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED3818: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED381C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED3820: 409A0018  bne cr6, 0x82ed3838
	if !ctx.cr[6].eq {
	pc = 0x82ED3838; continue 'dispatch;
	}
	// 82ED3824: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED3828: 7C7DD02E  lwzx r3, r29, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED382C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED3830: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED3834: 4BFCCF7D  bl 0x82ea07b0
	ctx.lr = 0x82ED3838;
	sub_82EA07B0(ctx, base);
	// 82ED3838: 7D5DB82E  lwzx r10, r29, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED383C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED3840: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3844: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED3848: 40980020  bge cr6, 0x82ed3868
	if !ctx.cr[6].lt {
	pc = 0x82ED3868; continue 'dispatch;
	}
	// 82ED384C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED3850: 390946E8  addi r8, r9, 0x46e8
	ctx.r[8].s64 = ctx.r[9].s64 + 18152;
	// 82ED3854: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED3858: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED385C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED3860: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED3864: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED3868: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82ED386C: 419A008C  beq cr6, 0x82ed38f8
	if ctx.cr[6].eq {
	pc = 0x82ED38F8; continue 'dispatch;
	}
	// 82ED3870: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED3874: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3878: 480102E1  bl 0x82ee3b58
	ctx.lr = 0x82ED387C;
	sub_82EE3B58(ctx, base);
	// 82ED387C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3880: 4800F7C9  bl 0x82ee3048
	ctx.lr = 0x82ED3884;
	sub_82EE3048(ctx, base);
	// 82ED3884: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED3888: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED388C: 4800C7FD  bl 0x82ee0088
	ctx.lr = 0x82ED3890;
	sub_82EE0088(ctx, base);
	// 82ED3890: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3894: 808B0054  lwz r4, 0x54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED3898: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82ED389C: 419A0028  beq cr6, 0x82ed38c4
	if ctx.cr[6].eq {
	pc = 0x82ED38C4; continue 'dispatch;
	}
	// 82ED38A0: A16B0050  lhz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED38A4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82ED38A8: 7C7DD02E  lwzx r3, r29, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED38AC: 5565283E  rotlwi r5, r11, 5
	ctx.r[5].u64 = ((ctx.r[11].u32).rotate_left(5)) as u64;
	// 82ED38B0: 4BFCCF01  bl 0x82ea07b0
	ctx.lr = 0x82ED38B4;
	sub_82EA07B0(ctx, base);
	// 82ED38B4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED38B8: 932A0054  stw r25, 0x54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82ED38BC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED38C0: B3290050  sth r25, 0x50(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(80 as u32), ctx.r[25].u16 ) };
	// 82ED38C4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED38C8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED38CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED38D0: 409A0014  bne cr6, 0x82ed38e4
	if !ctx.cr[6].eq {
	pc = 0x82ED38E4; continue 'dispatch;
	}
	// 82ED38D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED38D8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED38DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED38E0: 4E800421  bctrl
	ctx.lr = 0x82ED38E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED38E4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED38E8: 48001821  bl 0x82ed5108
	ctx.lr = 0x82ED38EC;
	sub_82ED5108(ctx, base);
	// 82ED38EC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ED38F0: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82ED38F4: 409AFF7C  bne cr6, 0x82ed3870
	if !ctx.cr[6].eq {
	pc = 0x82ED3870; continue 'dispatch;
	}
	// 82ED38F8: 7D5DB82E  lwzx r10, r29, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED38FC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED3900: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3904: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED3908: 40980020  bge cr6, 0x82ed3928
	if !ctx.cr[6].lt {
	pc = 0x82ED3928; continue 'dispatch;
	}
	// 82ED390C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82ED3910: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82ED3914: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED3918: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ED391C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ED3920: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED3924: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED3928: 817B0084  lwz r11, 0x84(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED392C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3930: 917B0084  stw r11, 0x84(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED3934: 40820028  bne 0x82ed395c
	if !ctx.cr[0].eq {
	pc = 0x82ED395C; continue 'dispatch;
	}
	// 82ED3938: 897B008C  lbz r11, 0x8c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED393C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED3940: 409A001C  bne cr6, 0x82ed395c
	if !ctx.cr[6].eq {
	pc = 0x82ED395C; continue 'dispatch;
	}
	// 82ED3944: 817B0080  lwz r11, 0x80(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED3948: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED394C: 419A0010  beq cr6, 0x82ed395c
	if ctx.cr[6].eq {
	pc = 0x82ED395C; continue 'dispatch;
	}
	// 82ED3950: 933B0080  stw r25, 0x80(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82ED3954: 807B007C  lwz r3, 0x7c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3958: 480144A1  bl 0x82ee7df8
	ctx.lr = 0x82ED395C;
	sub_82EE7DF8(ctx, base);
	// 82ED395C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82ED3960: 482D4844  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED3968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED3968 size=328
    let mut pc: u32 = 0x82ED3968;
    'dispatch: loop {
        match pc {
            0x82ED3968 => {
    //   block [0x82ED3968..0x82ED3AB0)
	// 82ED3968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED396C: 482D47FD  bl 0x831a8168
	ctx.lr = 0x82ED3970;
	sub_831A8130(ctx, base);
	// 82ED3970: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED3974: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED3978: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED397C: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3980: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3984: 419A0028  beq cr6, 0x82ed39ac
	if ctx.cr[6].eq {
	pc = 0x82ED39AC; continue 'dispatch;
	}
	// 82ED3988: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82ED398C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82ED3990: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82ED3994: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3998: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82ED399C: 48013E6D  bl 0x82ee7808
	ctx.lr = 0x82ED39A0;
	sub_82EE7808(ctx, base);
	// 82ED39A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ED39A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED39A8: 482D4810  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82ED39AC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED39B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED39B4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED39B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED39BC: 4E800421  bctrl
	ctx.lr = 0x82ED39C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED39C0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82ED39C4: 2F03000C  cmpwi cr6, r3, 0xc
	ctx.cr[6].compare_i32(ctx.r[3].s32, 12, &mut ctx.xer);
	// 82ED39C8: 409A0048  bne cr6, 0x82ed3a10
	if !ctx.cr[6].eq {
	pc = 0x82ED3A10; continue 'dispatch;
	}
	// 82ED39CC: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED39D0: A17D001C  lhz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ED39D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED39D8: 409A0038  bne cr6, 0x82ed3a10
	if !ctx.cr[6].eq {
	pc = 0x82ED3A10; continue 'dispatch;
	}
	// 82ED39DC: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED39E0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82ED39E4: 9B810050  stb r28, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u8 ) };
	// 82ED39E8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82ED39EC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED39F0: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED39F4: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED39F8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED39FC: 4E800421  bctrl
	ctx.lr = 0x82ED3A00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3A00: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED3A04: 80E10064  lwz r7, 0x64(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED3A08: B11D001C  sth r8, 0x1c(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[8].u16 ) };
	// 82ED3A0C: B0FD001E  sth r7, 0x1e(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(30 as u32), ctx.r[7].u16 ) };
	// 82ED3A10: 9B9E008C  stb r28, 0x8c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[28].u8 ) };
	// 82ED3A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3A18: 48009D91  bl 0x82edd7a8
	ctx.lr = 0x82ED3A1C;
	sub_82EDD7A8(ctx, base);
	// 82ED3A1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ED3A20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED3A24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED3A28: 4800B169  bl 0x82edeb90
	ctx.lr = 0x82ED3A2C;
	sub_82EDEB90(ctx, base);
	// 82ED3A2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3A30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED3A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3A38: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3A3C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3A40: 4E800421  bctrl
	ctx.lr = 0x82ED3A44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3A44: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82ED3A48: 409A0030  bne cr6, 0x82ed3a78
	if !ctx.cr[6].eq {
	pc = 0x82ED3A78; continue 'dispatch;
	}
	// 82ED3A4C: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ED3A50: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED3A54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3A58: 409A000C  bne cr6, 0x82ed3a64
	if !ctx.cr[6].eq {
	pc = 0x82ED3A64; continue 'dispatch;
	}
	// 82ED3A5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED3A60: 4BFFE5A9  bl 0x82ed2008
	ctx.lr = 0x82ED3A64;
	sub_82ED2008(ctx, base);
	// 82ED3A64: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED3A68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3A6C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED3A70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3A74: 4E800421  bctrl
	ctx.lr = 0x82ED3A78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3A78: 815E0084  lwz r10, 0x84(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3A7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED3A80: 997E008C  stb r11, 0x8c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[11].u8 ) };
	// 82ED3A84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED3A88: 409A001C  bne cr6, 0x82ed3aa4
	if !ctx.cr[6].eq {
	pc = 0x82ED3AA4; continue 'dispatch;
	}
	// 82ED3A8C: 815E0080  lwz r10, 0x80(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED3A90: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED3A94: 419A0010  beq cr6, 0x82ed3aa4
	if ctx.cr[6].eq {
	pc = 0x82ED3AA4; continue 'dispatch;
	}
	// 82ED3A98: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ED3A9C: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3AA0: 48014359  bl 0x82ee7df8
	ctx.lr = 0x82ED3AA4;
	sub_82EE7DF8(ctx, base);
	// 82ED3AA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED3AA8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED3AAC: 482D470C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED3AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED3AB0 size=116
    let mut pc: u32 = 0x82ED3AB0;
    'dispatch: loop {
        match pc {
            0x82ED3AB0 => {
    //   block [0x82ED3AB0..0x82ED3B24)
	// 82ED3AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED3AB4: 482D46B5  bl 0x831a8168
	ctx.lr = 0x82ED3AB8;
	sub_831A8130(ctx, base);
	// 82ED3AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED3ABC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3AC0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED3AC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED3AC8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ED3ACC: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82ED3AD0: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82ED3AD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED3AD8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED3ADC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82ED3AE0: 4BFCCC51  bl 0x82ea0730
	ctx.lr = 0x82ED3AE4;
	sub_82EA0730(ctx, base);
	// 82ED3AE4: 3920002C  li r9, 0x2c
	ctx.r[9].s64 = 44;
	// 82ED3AE8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82ED3AEC: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82ED3AF0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82ED3AF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82ED3AF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED3AFC: 4800996D  bl 0x82edd468
	ctx.lr = 0x82ED3B00;
	sub_82EDD468(ctx, base);
	// 82ED3B00: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED3B04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED3B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3B0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED3B10: 911E0024  stw r8, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 82ED3B14: 4BFFFE55  bl 0x82ed3968
	ctx.lr = 0x82ED3B18;
	sub_82ED3968(ctx, base);
	// 82ED3B18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED3B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED3B20: 482D4698  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED3B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED3B28 size=240
    let mut pc: u32 = 0x82ED3B28;
    'dispatch: loop {
        match pc {
            0x82ED3B28 => {
    //   block [0x82ED3B28..0x82ED3C18)
	// 82ED3B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED3B2C: 482D463D  bl 0x831a8168
	ctx.lr = 0x82ED3B30;
	sub_831A8130(ctx, base);
	// 82ED3B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED3B34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED3B38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED3B3C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82ED3B40: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3B44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3B48: 419A0030  beq cr6, 0x82ed3b78
	if ctx.cr[6].eq {
	pc = 0x82ED3B78; continue 'dispatch;
	}
	// 82ED3B4C: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82ED3B50: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ED3B54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED3B58: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3B5C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED3B60: 48013CA9  bl 0x82ee7808
	ctx.lr = 0x82ED3B64;
	sub_82EE7808(ctx, base);
	// 82ED3B64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED3B68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED3B6C: 995D0000  stb r10, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82ED3B70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED3B74: 482D4644  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82ED3B78: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82ED3B7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED3B80: 939F0084  stw r28, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 82ED3B84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3B88: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3B8C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3B90: 4E800421  bctrl
	ctx.lr = 0x82ED3B94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3B94: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82ED3B98: 409A001C  bne cr6, 0x82ed3bb4
	if !ctx.cr[6].eq {
	pc = 0x82ED3BB4; continue 'dispatch;
	}
	// 82ED3B9C: 809E0038  lwz r4, 0x38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ED3BA0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED3BA4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82ED3BA8: 409A000C  bne cr6, 0x82ed3bb4
	if !ctx.cr[6].eq {
	pc = 0x82ED3BB4; continue 'dispatch;
	}
	// 82ED3BAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3BB0: 4BFFE649  bl 0x82ed21f8
	ctx.lr = 0x82ED3BB4;
	sub_82ED21F8(ctx, base);
	// 82ED3BB4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ED3BB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED3BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3BC0: 4800B089  bl 0x82edec48
	ctx.lr = 0x82ED3BC4;
	sub_82EDEC48(ctx, base);
	// 82ED3BC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED3BC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED3BCC: 48009855  bl 0x82edd420
	ctx.lr = 0x82ED3BD0;
	sub_82EDD420(ctx, base);
	// 82ED3BD0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3BD4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3BD8: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED3BDC: 4082002C  bne 0x82ed3c08
	if !ctx.cr[0].eq {
	pc = 0x82ED3C08; continue 'dispatch;
	}
	// 82ED3BE0: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED3BE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED3BE8: 409A0020  bne cr6, 0x82ed3c08
	if !ctx.cr[6].eq {
	pc = 0x82ED3C08; continue 'dispatch;
	}
	// 82ED3BEC: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED3BF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3BF4: 419A0014  beq cr6, 0x82ed3c08
	if ctx.cr[6].eq {
	pc = 0x82ED3C08; continue 'dispatch;
	}
	// 82ED3BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED3BFC: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3C00: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ED3C04: 480141F5  bl 0x82ee7df8
	ctx.lr = 0x82ED3C08;
	sub_82EE7DF8(ctx, base);
	// 82ED3C08: 9B9D0000  stb r28, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82ED3C0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED3C10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED3C14: 482D45A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED3C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED3C18 size=84
    let mut pc: u32 = 0x82ED3C18;
    'dispatch: loop {
        match pc {
            0x82ED3C18 => {
    //   block [0x82ED3C18..0x82ED3C6C)
	// 82ED3C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED3C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED3C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED3C24: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3C28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3C2C: 419A002C  beq cr6, 0x82ed3c58
	if ctx.cr[6].eq {
	pc = 0x82ED3C58; continue 'dispatch;
	}
	// 82ED3C30: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82ED3C34: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82ED3C38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED3C3C: 8063007C  lwz r3, 0x7c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3C40: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED3C44: 48013BC5  bl 0x82ee7808
	ctx.lr = 0x82ED3C48;
	sub_82EE7808(ctx, base);
	// 82ED3C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED3C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED3C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED3C54: 4E800020  blr
	return;
	// 82ED3C58: 4BFFE5A1  bl 0x82ed21f8
	ctx.lr = 0x82ED3C5C;
	sub_82ED21F8(ctx, base);
	// 82ED3C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED3C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED3C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED3C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED3C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED3C70 size=944
    let mut pc: u32 = 0x82ED3C70;
    'dispatch: loop {
        match pc {
            0x82ED3C70 => {
    //   block [0x82ED3C70..0x82ED4020)
	// 82ED3C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED3C74: 482D44D9  bl 0x831a814c
	ctx.lr = 0x82ED3C78;
	sub_831A8130(ctx, base);
	// 82ED3C78: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED3C7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ED3C80: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82ED3C84: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82ED3C88: 815C0088  lwz r10, 0x88(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED3C8C: 817C0084  lwz r11, 0x84(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3C90: 7D4A5A15  add. r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED3C94: 41820028  beq 0x82ed3cbc
	if ctx.cr[0].eq {
	pc = 0x82ED3CBC; continue 'dispatch;
	}
	// 82ED3C98: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82ED3C9C: 92A10054  stw r21, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[21].u32 ) };
	// 82ED3CA0: B3E10058  sth r31, 0x58(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u16 ) };
	// 82ED3CA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED3CA8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED3CAC: 807C007C  lwz r3, 0x7c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3CB0: 48013B59  bl 0x82ee7808
	ctx.lr = 0x82ED3CB4;
	sub_82EE7808(ctx, base);
	// 82ED3CB4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82ED3CB8: 482D44E4  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 82ED3CBC: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3CC0: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 82ED3CC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED3CC8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82ED3CCC: 917C0084  stw r11, 0x84(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED3CD0: 3F008000  lis r24, -0x8000
	ctx.r[24].s64 = -2147483648;
	// 82ED3CD4: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82ED3CD8: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82ED3CDC: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED3CE0: 93210084  stw r25, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[25].u32 ) };
	// 82ED3CE4: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED3CE8: 93010088  stw r24, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[24].u32 ) };
	// 82ED3CEC: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED3CF0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED3CF4: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED3CF8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED3CFC: 41990010  bgt cr6, 0x82ed3d0c
	if ctx.cr[6].gt {
	pc = 0x82ED3D0C; continue 'dispatch;
	}
	// 82ED3D00: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED3D04: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED3D08: 48000014  b 0x82ed3d1c
	pc = 0x82ED3D1C; continue 'dispatch;
	// 82ED3D0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3D10: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED3D14: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3D18: 4E800421  bctrl
	ctx.lr = 0x82ED3D1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3D1C: 7FFEC378  or r30, r31, r24
	ctx.r[30].u64 = ctx.r[31].u64 | ctx.r[24].u64;
	// 82ED3D20: 90610080  stw r3, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[3].u32 ) };
	// 82ED3D24: 93010078  stw r24, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[24].u32 ) };
	// 82ED3D28: 57EB2834  slwi r11, r31, 5
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED3D2C: 93C10088  stw r30, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 82ED3D30: 9061008C  stw r3, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[3].u32 ) };
	// 82ED3D34: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82ED3D38: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED3D3C: 93210074  stw r25, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82ED3D40: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED3D44: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82ED3D48: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED3D4C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED3D50: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED3D54: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED3D58: 4199000C  bgt cr6, 0x82ed3d64
	if ctx.cr[6].gt {
	pc = 0x82ED3D64; continue 'dispatch;
	}
	// 82ED3D5C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED3D60: 48000018  b 0x82ed3d78
	pc = 0x82ED3D78; continue 'dispatch;
	// 82ED3D64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3D68: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED3D6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3D70: 4E800421  bctrl
	ctx.lr = 0x82ED3D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3D74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED3D78: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82ED3D7C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82ED3D80: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82ED3D84: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82ED3D88: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82ED3D8C: 93E10084  stw r31, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 82ED3D90: 409900D0  ble cr6, 0x82ed3e60
	if !ctx.cr[6].gt {
	pc = 0x82ED3E60; continue 'dispatch;
	}
	// 82ED3D94: 3BDC00E4  addi r30, r28, 0xe4
	ctx.r[30].s64 = ctx.r[28].s64 + 228;
	// 82ED3D98: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 82ED3D9C: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82ED3DA0: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82ED3DA4: 7FFDA82E  lwzx r31, r29, r21
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82ED3DA8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3DAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3DB0: 409A001C  bne cr6, 0x82ed3dcc
	if !ctx.cr[6].eq {
	pc = 0x82ED3DCC; continue 'dispatch;
	}
	// 82ED3DB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3DBC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED3DC0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3DC4: 4E800421  bctrl
	ctx.lr = 0x82ED3DC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3DC8: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82ED3DCC: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82ED3DD0: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82ED3DD4: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED3DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3DDC: 7D7D512E  stwx r11, r29, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82ED3DE0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3DE4: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED3DE8: 7C9B5A14  add r4, r27, r11
	ctx.r[4].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82ED3DEC: 81090018  lwz r8, 0x18(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3DF0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ED3DF4: 4E800421  bctrl
	ctx.lr = 0x82ED3DF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3DF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3DFC: 480012ED  bl 0x82ed50e8
	ctx.lr = 0x82ED3E00;
	sub_82ED50E8(ctx, base);
	// 82ED3E00: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED3E04: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3E08: 54E500BE  clrlwi r5, r7, 2
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED3E0C: 7F062800  cmpw cr6, r6, r5
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82ED3E10: 409A0010  bne cr6, 0x82ed3e20
	if !ctx.cr[6].eq {
	pc = 0x82ED3E20; continue 'dispatch;
	}
	// 82ED3E14: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED3E18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED3E1C: 4BFD2A65  bl 0x82ea6880
	ctx.lr = 0x82ED3E20;
	sub_82EA6880(ctx, base);
	// 82ED3E20: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3E24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED3E28: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3E2C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED3E30: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED3E34: 7FE9512E  stwx r31, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82ED3E38: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED3E3C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED3E40: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED3E44: 4800FEC5  bl 0x82ee3d08
	ctx.lr = 0x82ED3E48;
	sub_82EE3D08(ctx, base);
	// 82ED3E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED3E4C: 48001A3D  bl 0x82ed5888
	ctx.lr = 0x82ED3E50;
	sub_82ED5888(ctx, base);
	// 82ED3E50: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82ED3E54: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82ED3E58: 3B7B0020  addi r27, r27, 0x20
	ctx.r[27].s64 = ctx.r[27].s64 + 32;
	// 82ED3E5C: 4082FF48  bne 0x82ed3da4
	if !ctx.cr[0].eq {
	pc = 0x82ED3DA4; continue 'dispatch;
	}
	// 82ED3E60: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82ED3E64: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED3E68: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82ED3E6C: 93010068  stw r24, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[24].u32 ) };
	// 82ED3E70: 83FC02F8  lwz r31, 0x2f8(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(760 as u32) ) } as u64;
	// 82ED3E74: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82ED3E78: 55641836  rlwinm r4, r11, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82ED3E7C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED3E80: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED3E84: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED3E88: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED3E8C: 4199000C  bgt cr6, 0x82ed3e98
	if ctx.cr[6].gt {
	pc = 0x82ED3E98; continue 'dispatch;
	}
	// 82ED3E90: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED3E94: 48000018  b 0x82ed3eac
	pc = 0x82ED3EAC; continue 'dispatch;
	// 82ED3E98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3E9C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED3EA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3EA4: 4E800421  bctrl
	ctx.lr = 0x82ED3EA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3EA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED3EAC: 7FEAC378  or r10, r31, r24
	ctx.r[10].u64 = ctx.r[31].u64 | ctx.r[24].u64;
	// 82ED3EB0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82ED3EB4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82ED3EB8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82ED3EBC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82ED3EC0: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82ED3EC4: 807C0054  lwz r3, 0x54(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED3EC8: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82ED3ECC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3ED0: 81090018  lwz r8, 0x18(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3ED4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ED3ED8: 4E800421  bctrl
	ctx.lr = 0x82ED3EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3EDC: 817C0070  lwz r11, 0x70(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED3EE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED3EE4: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82ED3EE8: 409A0008  bne cr6, 0x82ed3ef0
	if !ctx.cr[6].eq {
	pc = 0x82ED3EF0; continue 'dispatch;
	}
	// 82ED3EEC: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82ED3EF0: 807C0058  lwz r3, 0x58(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED3EF4: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED3EF8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED3EFC: 4804FA65  bl 0x82f23960
	ctx.lr = 0x82ED3F00;
	sub_82F23960(ctx, base);
	// 82ED3F00: 817C0084  lwz r11, 0x84(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED3F04: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3F08: 917C0084  stw r11, 0x84(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED3F0C: 40820028  bne 0x82ed3f34
	if !ctx.cr[0].eq {
	pc = 0x82ED3F34; continue 'dispatch;
	}
	// 82ED3F10: 897C008C  lbz r11, 0x8c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED3F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED3F18: 409A001C  bne cr6, 0x82ed3f34
	if !ctx.cr[6].eq {
	pc = 0x82ED3F34; continue 'dispatch;
	}
	// 82ED3F1C: 817C0080  lwz r11, 0x80(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED3F20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED3F24: 419A0010  beq cr6, 0x82ed3f34
	if ctx.cr[6].eq {
	pc = 0x82ED3F34; continue 'dispatch;
	}
	// 82ED3F28: 933C0080  stw r25, 0x80(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82ED3F2C: 807C007C  lwz r3, 0x7c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3F30: 48013EC9  bl 0x82ee7df8
	ctx.lr = 0x82ED3F34;
	sub_82EE7DF8(ctx, base);
	// 82ED3F34: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED3F38: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED3F3C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED3F40: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED3F44: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED3F48: 409A0014  bne cr6, 0x82ed3f5c
	if !ctx.cr[6].eq {
	pc = 0x82ED3F5C; continue 'dispatch;
	}
	// 82ED3F4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3F50: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3F54: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3F58: 4E800421  bctrl
	ctx.lr = 0x82ED3F5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3F5C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED3F60: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED3F64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED3F68: 409A0018  bne cr6, 0x82ed3f80
	if !ctx.cr[6].eq {
	pc = 0x82ED3F80; continue 'dispatch;
	}
	// 82ED3F6C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED3F70: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED3F74: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED3F78: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED3F7C: 4BFCC835  bl 0x82ea07b0
	ctx.lr = 0x82ED3F80;
	sub_82EA07B0(ctx, base);
	// 82ED3F80: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED3F84: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED3F88: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED3F8C: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED3F90: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED3F94: 409A0014  bne cr6, 0x82ed3fa8
	if !ctx.cr[6].eq {
	pc = 0x82ED3FA8; continue 'dispatch;
	}
	// 82ED3F98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3F9C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3FA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3FA4: 4E800421  bctrl
	ctx.lr = 0x82ED3FA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3FA8: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED3FAC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED3FB0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED3FB4: 409A0018  bne cr6, 0x82ed3fcc
	if !ctx.cr[6].eq {
	pc = 0x82ED3FCC; continue 'dispatch;
	}
	// 82ED3FB8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED3FBC: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED3FC0: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED3FC4: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED3FC8: 4BFCC7E9  bl 0x82ea07b0
	ctx.lr = 0x82ED3FCC;
	sub_82EA07B0(ctx, base);
	// 82ED3FCC: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED3FD0: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED3FD4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED3FD8: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED3FDC: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED3FE0: 409A0014  bne cr6, 0x82ed3ff4
	if !ctx.cr[6].eq {
	pc = 0x82ED3FF4; continue 'dispatch;
	}
	// 82ED3FE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED3FE8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED3FEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED3FF0: 4E800421  bctrl
	ctx.lr = 0x82ED3FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED3FF4: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED3FF8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED3FFC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED4000: 409A0018  bne cr6, 0x82ed4018
	if !ctx.cr[6].eq {
	pc = 0x82ED4018; continue 'dispatch;
	}
	// 82ED4004: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED4008: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82ED400C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED4010: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED4014: 4BFCC79D  bl 0x82ea07b0
	ctx.lr = 0x82ED4018;
	sub_82EA07B0(ctx, base);
	// 82ED4018: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82ED401C: 482D4180  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4020 size=776
    let mut pc: u32 = 0x82ED4020;
    'dispatch: loop {
        match pc {
            0x82ED4020 => {
    //   block [0x82ED4020..0x82ED4328)
	// 82ED4020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4024: 482D412D  bl 0x831a8150
	ctx.lr = 0x82ED4028;
	sub_831A8130(ctx, base);
	// 82ED4028: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED402C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4030: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82ED4034: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82ED4038: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED403C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED4040: 7D4A5A15  add. r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED4044: 41820028  beq 0x82ed406c
	if ctx.cr[0].eq {
	pc = 0x82ED406C; continue 'dispatch;
	}
	// 82ED4048: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82ED404C: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82ED4050: B3410058  sth r26, 0x58(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u16 ) };
	// 82ED4054: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED4058: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED405C: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED4060: 480137A9  bl 0x82ee7808
	ctx.lr = 0x82ED4064;
	sub_82EE7808(ctx, base);
	// 82ED4064: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82ED4068: 482D4138  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 82ED406C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4070: 3AC00014  li r22, 0x14
	ctx.r[22].s64 = 20;
	// 82ED4074: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED4078: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82ED407C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED4080: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82ED4084: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82ED4088: 395A0004  addi r10, r26, 4
	ctx.r[10].s64 = ctx.r[26].s64 + 4;
	// 82ED408C: 7C76B82E  lwzx r3, r22, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED4090: 93010064  stw r24, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[24].u32 ) };
	// 82ED4094: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED4098: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 82ED409C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED40A0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED40A4: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED40A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED40AC: 4199000C  bgt cr6, 0x82ed40b8
	if ctx.cr[6].gt {
	pc = 0x82ED40B8; continue 'dispatch;
	}
	// 82ED40B0: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED40B4: 48000018  b 0x82ed40cc
	pc = 0x82ED40CC; continue 'dispatch;
	// 82ED40B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED40BC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED40C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED40C4: 4E800421  bctrl
	ctx.lr = 0x82ED40C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED40C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED40CC: 7F4ADB78  or r10, r26, r27
	ctx.r[10].u64 = ctx.r[26].u64 | ctx.r[27].u64;
	// 82ED40D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82ED40D4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82ED40D8: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82ED40DC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82ED40E0: 40990050  ble cr6, 0x82ed4130
	if !ctx.cr[6].gt {
	pc = 0x82ED4130; continue 'dispatch;
	}
	// 82ED40E4: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82ED40E8: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82ED40EC: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED40F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED40F4: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED40F8: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED40FC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED4100: 391E0024  addi r8, r30, 0x24
	ctx.r[8].s64 = ctx.r[30].s64 + 36;
	// 82ED4104: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED4108: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ED410C: 80E10064  lwz r7, 0x64(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED4110: 38C70001  addi r6, r7, 1
	ctx.r[6].s64 = ctx.r[7].s64 + 1;
	// 82ED4114: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 82ED4118: 4800FCC9  bl 0x82ee3de0
	ctx.lr = 0x82ED411C;
	sub_82EE3DE0(ctx, base);
	// 82ED411C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED4120: 48001701  bl 0x82ed5820
	ctx.lr = 0x82ED4124;
	sub_82ED5820(ctx, base);
	// 82ED4124: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED4128: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82ED412C: 4082FFC0  bne 0x82ed40ec
	if !ctx.cr[0].eq {
	pc = 0x82ED40EC; continue 'dispatch;
	}
	// 82ED4130: 93010070  stw r24, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[24].u32 ) };
	// 82ED4134: 7C76B82E  lwzx r3, r22, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED4138: 93010074  stw r24, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[24].u32 ) };
	// 82ED413C: 93610078  stw r27, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[27].u32 ) };
	// 82ED4140: 83DF02F8  lwz r30, 0x2f8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) } as u64;
	// 82ED4144: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82ED4148: 55641836  rlwinm r4, r11, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 82ED414C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED4150: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED4154: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED4158: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED415C: 41990010  bgt cr6, 0x82ed416c
	if ctx.cr[6].gt {
	pc = 0x82ED416C; continue 'dispatch;
	}
	// 82ED4160: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED4164: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED4168: 48000014  b 0x82ed417c
	pc = 0x82ED417C; continue 'dispatch;
	// 82ED416C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4170: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED4174: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED4178: 4E800421  bctrl
	ctx.lr = 0x82ED417C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED417C: 7FCBDB78  or r11, r30, r27
	ctx.r[11].u64 = ctx.r[30].u64 | ctx.r[27].u64;
	// 82ED4180: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82ED4184: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82ED4188: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82ED418C: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82ED4190: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82ED4194: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED4198: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED419C: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED41A0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED41A4: 4E800421  bctrl
	ctx.lr = 0x82ED41A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED41A8: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED41AC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED41B0: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED41B4: 4804F865  bl 0x82f23a18
	ctx.lr = 0x82ED41B8;
	sub_82F23A18(ctx, base);
	// 82ED41B8: 7C76B82E  lwzx r3, r22, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED41BC: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED41C0: 81030028  lwz r8, 0x28(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED41C4: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED41C8: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82ED41CC: 409A0014  bne cr6, 0x82ed41e0
	if !ctx.cr[6].eq {
	pc = 0x82ED41E0; continue 'dispatch;
	}
	// 82ED41D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED41D4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED41D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED41DC: 4E800421  bctrl
	ctx.lr = 0x82ED41E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED41E0: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED41E4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED41E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED41EC: 409A0018  bne cr6, 0x82ed4204
	if !ctx.cr[6].eq {
	pc = 0x82ED4204; continue 'dispatch;
	}
	// 82ED41F0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED41F4: 7C76B82E  lwzx r3, r22, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED41F8: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED41FC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED4200: 4BFCC5B1  bl 0x82ea07b0
	ctx.lr = 0x82ED4204;
	sub_82EA07B0(ctx, base);
	// 82ED4204: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82ED4208: 40990098  ble cr6, 0x82ed42a0
	if !ctx.cr[6].gt {
	pc = 0x82ED42A0; continue 'dispatch;
	}
	// 82ED420C: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82ED4210: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4214: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82ED4218: 931E0008  stw r24, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82ED421C: 813F00E8  lwz r9, 0xe8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ED4220: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED4224: 40990024  ble cr6, 0x82ed4248
	if !ctx.cr[6].gt {
	pc = 0x82ED4248; continue 'dispatch;
	}
	// 82ED4228: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ED422C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4230: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82ED4234: 419A0018  beq cr6, 0x82ed424c
	if ctx.cr[6].eq {
	pc = 0x82ED424C; continue 'dispatch;
	}
	// 82ED4238: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED423C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED4240: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED4244: 4198FFE8  blt cr6, 0x82ed422c
	if ctx.cr[6].lt {
	pc = 0x82ED422C; continue 'dispatch;
	}
	// 82ED4248: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED424C: 815F00E8  lwz r10, 0xe8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ED4250: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED4254: 811F00E4  lwz r8, 0xe4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ED4258: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82ED425C: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 82ED4260: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED4264: 7CC7402E  lwzx r6, r7, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82ED4268: 7CC9412E  stwx r6, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u32) };
	// 82ED426C: A0BE0004  lhz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED4270: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82ED4274: 409A0018  bne cr6, 0x82ed428c
	if !ctx.cr[6].eq {
	pc = 0x82ED428C; continue 'dispatch;
	}
	// 82ED4278: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED427C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED4280: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED4284: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED4288: 4E800421  bctrl
	ctx.lr = 0x82ED428C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED428C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED4290: 48000E79  bl 0x82ed5108
	ctx.lr = 0x82ED4294;
	sub_82ED5108(ctx, base);
	// 82ED4294: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82ED4298: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82ED429C: 4082FF74  bne 0x82ed4210
	if !ctx.cr[0].eq {
	pc = 0x82ED4210; continue 'dispatch;
	}
	// 82ED42A0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED42A4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED42A8: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED42AC: 40820028  bne 0x82ed42d4
	if !ctx.cr[0].eq {
	pc = 0x82ED42D4; continue 'dispatch;
	}
	// 82ED42B0: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED42B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED42B8: 409A001C  bne cr6, 0x82ed42d4
	if !ctx.cr[6].eq {
	pc = 0x82ED42D4; continue 'dispatch;
	}
	// 82ED42BC: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED42C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED42C4: 419A0010  beq cr6, 0x82ed42d4
	if ctx.cr[6].eq {
	pc = 0x82ED42D4; continue 'dispatch;
	}
	// 82ED42C8: 931F0080  stw r24, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 82ED42CC: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED42D0: 48013B29  bl 0x82ee7df8
	ctx.lr = 0x82ED42D4;
	sub_82EE7DF8(ctx, base);
	// 82ED42D4: 7C76B82E  lwzx r3, r22, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED42D8: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED42DC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED42E0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED42E4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED42E8: 409A0014  bne cr6, 0x82ed42fc
	if !ctx.cr[6].eq {
	pc = 0x82ED42FC; continue 'dispatch;
	}
	// 82ED42EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED42F0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED42F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED42F8: 4E800421  bctrl
	ctx.lr = 0x82ED42FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED42FC: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED4300: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED4304: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED4308: 409A0018  bne cr6, 0x82ed4320
	if !ctx.cr[6].eq {
	pc = 0x82ED4320; continue 'dispatch;
	}
	// 82ED430C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED4310: 7C76B82E  lwzx r3, r22, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82ED4314: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED4318: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED431C: 4BFCC495  bl 0x82ea07b0
	ctx.lr = 0x82ED4320;
	sub_82EA07B0(ctx, base);
	// 82ED4320: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82ED4324: 482D3E7C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4328 size=544
    let mut pc: u32 = 0x82ED4328;
    'dispatch: loop {
        match pc {
            0x82ED4328 => {
    //   block [0x82ED4328..0x82ED4548)
	// 82ED4328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED432C: 482D3E3D  bl 0x831a8168
	ctx.lr = 0x82ED4330;
	sub_831A8130(ctx, base);
	// 82ED4330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4334: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED4338: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ED433C: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED4340: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82ED4344: 409900B8  ble cr6, 0x82ed43fc
	if !ctx.cr[6].gt {
	pc = 0x82ED43FC; continue 'dispatch;
	}
	// 82ED4348: 54A9003E  slwi r9, r5, 0
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED434C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED4350: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED4354: 40990030  ble cr6, 0x82ed4384
	if !ctx.cr[6].gt {
	pc = 0x82ED4384; continue 'dispatch;
	}
	// 82ED4358: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED435C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4360: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED4364: 419A0018  beq cr6, 0x82ed437c
	if ctx.cr[6].eq {
	pc = 0x82ED437C; continue 'dispatch;
	}
	// 82ED4368: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED436C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED4370: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED4374: 4198FFE8  blt cr6, 0x82ed435c
	if ctx.cr[6].lt {
	pc = 0x82ED435C; continue 'dispatch;
	}
	// 82ED4378: 4800000C  b 0x82ed4384
	pc = 0x82ED4384; continue 'dispatch;
	// 82ED437C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82ED4380: 409A0028  bne cr6, 0x82ed43a8
	if !ctx.cr[6].eq {
	pc = 0x82ED43A8; continue 'dispatch;
	}
	// 82ED4384: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ED4388: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED438C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4390: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ED4394: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82ED4398: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82ED439C: 69060001  xori r6, r8, 1
	ctx.r[6].u64 = ctx.r[8].u64 ^ 1;
	// 82ED43A0: 4BFFEA81  bl 0x82ed2e20
	ctx.lr = 0x82ED43A4;
	sub_82ED2E20(ctx, base);
	// 82ED43A4: 48000058  b 0x82ed43fc
	pc = 0x82ED43FC; continue 'dispatch;
	// 82ED43A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED43AC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82ED43B0: 4099004C  ble cr6, 0x82ed43fc
	if !ctx.cr[6].gt {
	pc = 0x82ED43FC; continue 'dispatch;
	}
	// 82ED43B4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED43B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED43BC: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED43C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED43C4: 419A0024  beq cr6, 0x82ed43e8
	if ctx.cr[6].eq {
	pc = 0x82ED43E8; continue 'dispatch;
	}
	// 82ED43C8: 895F0040  lbz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ED43CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED43D0: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED43D4: 7D490774  extsb r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	// 82ED43D8: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82ED43DC: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82ED43E0: 68E50001  xori r5, r7, 1
	ctx.r[5].u64 = ctx.r[7].u64 ^ 1;
	// 82ED43E4: 4BFF8EBD  bl 0x82ecd2a0
	ctx.lr = 0x82ED43E8;
	sub_82ECD2A0(ctx, base);
	// 82ED43E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED43EC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED43F0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ED43F4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED43F8: 4198FFC0  blt cr6, 0x82ed43b8
	if ctx.cr[6].lt {
	pc = 0x82ED43B8; continue 'dispatch;
	}
	// 82ED43FC: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED4400: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82ED4404: 4099009C  ble cr6, 0x82ed44a0
	if !ctx.cr[6].gt {
	pc = 0x82ED44A0; continue 'dispatch;
	}
	// 82ED4408: 54A9003E  slwi r9, r5, 0
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED440C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED4410: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED4414: 4099003C  ble cr6, 0x82ed4450
	if !ctx.cr[6].gt {
	pc = 0x82ED4450; continue 'dispatch;
	}
	// 82ED4418: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED441C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4420: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED4424: 419A0024  beq cr6, 0x82ed4448
	if ctx.cr[6].eq {
	pc = 0x82ED4448; continue 'dispatch;
	}
	// 82ED4428: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED442C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED4430: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED4434: 4198FFE8  blt cr6, 0x82ed441c
	if ctx.cr[6].lt {
	pc = 0x82ED441C; continue 'dispatch;
	}
	// 82ED4438: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED443C: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED4440: 4BFFF831  bl 0x82ed3c70
	ctx.lr = 0x82ED4444;
	sub_82ED3C70(ctx, base);
	// 82ED4444: 4800005C  b 0x82ed44a0
	pc = 0x82ED44A0; continue 'dispatch;
	// 82ED4448: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82ED444C: 409A0014  bne cr6, 0x82ed4460
	if !ctx.cr[6].eq {
	pc = 0x82ED4460; continue 'dispatch;
	}
	// 82ED4450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED4454: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED4458: 4BFFF819  bl 0x82ed3c70
	ctx.lr = 0x82ED445C;
	sub_82ED3C70(ctx, base);
	// 82ED445C: 48000044  b 0x82ed44a0
	pc = 0x82ED44A0; continue 'dispatch;
	// 82ED4460: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED4464: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82ED4468: 40990038  ble cr6, 0x82ed44a0
	if !ctx.cr[6].gt {
	pc = 0x82ED44A0; continue 'dispatch;
	}
	// 82ED446C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED4470: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED4474: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED4478: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED447C: 419A0010  beq cr6, 0x82ed448c
	if ctx.cr[6].eq {
	pc = 0x82ED448C; continue 'dispatch;
	}
	// 82ED4480: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED4484: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82ED4488: 4BFFA5B9  bl 0x82ecea40
	ctx.lr = 0x82ED448C;
	sub_82ECEA40(ctx, base);
	// 82ED448C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED4490: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED4494: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ED4498: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED449C: 4198FFD4  blt cr6, 0x82ed4470
	if ctx.cr[6].lt {
	pc = 0x82ED4470; continue 'dispatch;
	}
	// 82ED44A0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ED44A4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED44A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED44AC: 40990038  ble cr6, 0x82ed44e4
	if !ctx.cr[6].gt {
	pc = 0x82ED44E4; continue 'dispatch;
	}
	// 82ED44B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED44B4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED44B8: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED44BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED44C0: 419A0010  beq cr6, 0x82ed44d0
	if ctx.cr[6].eq {
	pc = 0x82ED44D0; continue 'dispatch;
	}
	// 82ED44C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED44C8: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82ED44CC: 4BFFDB3D  bl 0x82ed2008
	ctx.lr = 0x82ED44D0;
	sub_82ED2008(ctx, base);
	// 82ED44D0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ED44D4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED44D8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ED44DC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED44E0: 4198FFD4  blt cr6, 0x82ed44b4
	if ctx.cr[6].lt {
	pc = 0x82ED44B4; continue 'dispatch;
	}
	// 82ED44E4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED44E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED44EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED44F0: 40990050  ble cr6, 0x82ed4540
	if !ctx.cr[6].gt {
	pc = 0x82ED4540; continue 'dispatch;
	}
	// 82ED44F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED44F8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED44FC: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED4500: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED4504: 419A0028  beq cr6, 0x82ed452c
	if ctx.cr[6].eq {
	pc = 0x82ED452C; continue 'dispatch;
	}
	// 82ED4508: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82ED450C: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED4510: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED4514: 409A0010  bne cr6, 0x82ed4524
	if !ctx.cr[6].eq {
	pc = 0x82ED4524; continue 'dispatch;
	}
	// 82ED4518: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED451C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED4520: 419A000C  beq cr6, 0x82ed452c
	if ctx.cr[6].eq {
	pc = 0x82ED452C; continue 'dispatch;
	}
	// 82ED4524: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED4528: 4BFFF441  bl 0x82ed3968
	ctx.lr = 0x82ED452C;
	sub_82ED3968(ctx, base);
	// 82ED452C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED4530: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED4534: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ED4538: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED453C: 4198FFBC  blt cr6, 0x82ed44f8
	if ctx.cr[6].lt {
	pc = 0x82ED44F8; continue 'dispatch;
	}
	// 82ED4540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED4544: 482D3C74  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4548 size=548
    let mut pc: u32 = 0x82ED4548;
    'dispatch: loop {
        match pc {
            0x82ED4548 => {
    //   block [0x82ED4548..0x82ED476C)
	// 82ED4548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED454C: 482D3C19  bl 0x831a8164
	ctx.lr = 0x82ED4550;
	sub_831A8130(ctx, base);
	// 82ED4550: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4554: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED4558: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ED455C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED4560: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED4564: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED4568: 4099003C  ble cr6, 0x82ed45a4
	if !ctx.cr[6].gt {
	pc = 0x82ED45A4; continue 'dispatch;
	}
	// 82ED456C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ED4570: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED4574: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED4578: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED457C: 419A0014  beq cr6, 0x82ed4590
	if ctx.cr[6].eq {
	pc = 0x82ED4590; continue 'dispatch;
	}
	// 82ED4580: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82ED4584: 5545003E  slwi r5, r10, 0
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED4588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED458C: 4BFFF59D  bl 0x82ed3b28
	ctx.lr = 0x82ED4590;
	sub_82ED3B28(ctx, base);
	// 82ED4590: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED4594: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED4598: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ED459C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED45A0: 4198FFD0  blt cr6, 0x82ed4570
	if ctx.cr[6].lt {
	pc = 0x82ED4570; continue 'dispatch;
	}
	// 82ED45A4: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ED45A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED45AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED45B0: 40990060  ble cr6, 0x82ed4610
	if !ctx.cr[6].gt {
	pc = 0x82ED4610; continue 'dispatch;
	}
	// 82ED45B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ED45B8: 3B80000B  li r28, 0xb
	ctx.r[28].s64 = 11;
	// 82ED45BC: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED45C0: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED45C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED45C8: 419A0034  beq cr6, 0x82ed45fc
	if ctx.cr[6].eq {
	pc = 0x82ED45FC; continue 'dispatch;
	}
	// 82ED45CC: 815B0084  lwz r10, 0x84(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED45D0: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED45D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED45D8: 419A001C  beq cr6, 0x82ed45f4
	if ctx.cr[6].eq {
	pc = 0x82ED45F4; continue 'dispatch;
	}
	// 82ED45DC: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82ED45E0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82ED45E4: 9B810058  stb r28, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u8 ) };
	// 82ED45E8: 807B007C  lwz r3, 0x7c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED45EC: 4801321D  bl 0x82ee7808
	ctx.lr = 0x82ED45F0;
	sub_82EE7808(ctx, base);
	// 82ED45F0: 4800000C  b 0x82ed45fc
	pc = 0x82ED45FC; continue 'dispatch;
	// 82ED45F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED45F8: 4BFFDC01  bl 0x82ed21f8
	ctx.lr = 0x82ED45FC;
	sub_82ED21F8(ctx, base);
	// 82ED45FC: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ED4600: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED4604: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ED4608: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED460C: 4198FFB0  blt cr6, 0x82ed45bc
	if ctx.cr[6].lt {
	pc = 0x82ED45BC; continue 'dispatch;
	}
	// 82ED4610: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED4614: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED4618: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED461C: 40990040  ble cr6, 0x82ed465c
	if !ctx.cr[6].gt {
	pc = 0x82ED465C; continue 'dispatch;
	}
	// 82ED4620: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4624: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4628: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED462C: 419A0028  beq cr6, 0x82ed4654
	if ctx.cr[6].eq {
	pc = 0x82ED4654; continue 'dispatch;
	}
	// 82ED4630: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED4634: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED4638: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED463C: 4198FFE8  blt cr6, 0x82ed4624
	if ctx.cr[6].lt {
	pc = 0x82ED4624; continue 'dispatch;
	}
	// 82ED4640: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED4644: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED4648: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED464C: 4BFFEEED  bl 0x82ed3538
	ctx.lr = 0x82ED4650;
	sub_82ED3538(ctx, base);
	// 82ED4650: 48000068  b 0x82ed46b8
	pc = 0x82ED46B8; continue 'dispatch;
	// 82ED4654: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82ED4658: 409A0018  bne cr6, 0x82ed4670
	if !ctx.cr[6].eq {
	pc = 0x82ED4670; continue 'dispatch;
	}
	// 82ED465C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED4660: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED4664: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4668: 4BFFEED1  bl 0x82ed3538
	ctx.lr = 0x82ED466C;
	sub_82ED3538(ctx, base);
	// 82ED466C: 4800004C  b 0x82ed46b8
	pc = 0x82ED46B8; continue 'dispatch;
	// 82ED4670: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED4674: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED4678: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED467C: 4099003C  ble cr6, 0x82ed46b8
	if !ctx.cr[6].gt {
	pc = 0x82ED46B8; continue 'dispatch;
	}
	// 82ED4680: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ED4684: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4688: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED468C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED4690: 419A0014  beq cr6, 0x82ed46a4
	if ctx.cr[6].eq {
	pc = 0x82ED46A4; continue 'dispatch;
	}
	// 82ED4694: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82ED4698: 5545003E  slwi r5, r10, 0
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED469C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED46A0: 4BFF8DA1  bl 0x82ecd440
	ctx.lr = 0x82ED46A4;
	sub_82ECD440(ctx, base);
	// 82ED46A4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED46A8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED46AC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ED46B0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED46B4: 4198FFD0  blt cr6, 0x82ed4684
	if ctx.cr[6].lt {
	pc = 0x82ED4684; continue 'dispatch;
	}
	// 82ED46B8: 813E0030  lwz r9, 0x30(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED46BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED46C0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED46C4: 40990044  ble cr6, 0x82ed4708
	if !ctx.cr[6].gt {
	pc = 0x82ED4708; continue 'dispatch;
	}
	// 82ED46C8: 815E002C  lwz r10, 0x2c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED46CC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED46D0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED46D4: 419A002C  beq cr6, 0x82ed4700
	if ctx.cr[6].eq {
	pc = 0x82ED4700; continue 'dispatch;
	}
	// 82ED46D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED46DC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED46E0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED46E4: 4198FFE8  blt cr6, 0x82ed46cc
	if ctx.cr[6].lt {
	pc = 0x82ED46CC; continue 'dispatch;
	}
	// 82ED46E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED46EC: 80BE0030  lwz r5, 0x30(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED46F0: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED46F4: 4BFFF92D  bl 0x82ed4020
	ctx.lr = 0x82ED46F8;
	sub_82ED4020(ctx, base);
	// 82ED46F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED46FC: 482D3AB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82ED4700: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82ED4704: 409A001C  bne cr6, 0x82ed4720
	if !ctx.cr[6].eq {
	pc = 0x82ED4720; continue 'dispatch;
	}
	// 82ED4708: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED470C: 80BE0030  lwz r5, 0x30(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED4710: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED4714: 4BFFF90D  bl 0x82ed4020
	ctx.lr = 0x82ED4718;
	sub_82ED4020(ctx, base);
	// 82ED4718: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED471C: 482D3A98  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82ED4720: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED4724: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED4728: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED472C: 40990038  ble cr6, 0x82ed4764
	if !ctx.cr[6].gt {
	pc = 0x82ED4764; continue 'dispatch;
	}
	// 82ED4730: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ED4734: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED4738: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED473C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED4740: 419A0010  beq cr6, 0x82ed4750
	if ctx.cr[6].eq {
	pc = 0x82ED4750; continue 'dispatch;
	}
	// 82ED4744: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ED4748: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82ED474C: 4BFF8E25  bl 0x82ecd570
	ctx.lr = 0x82ED4750;
	sub_82ECD570(ctx, base);
	// 82ED4750: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED4754: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ED4758: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ED475C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED4760: 4198FFD4  blt cr6, 0x82ed4734
	if ctx.cr[6].lt {
	pc = 0x82ED4734; continue 'dispatch;
	}
	// 82ED4764: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED4768: 482D3A4C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4770 size=232
    let mut pc: u32 = 0x82ED4770;
    'dispatch: loop {
        match pc {
            0x82ED4770 => {
    //   block [0x82ED4770..0x82ED4858)
	// 82ED4770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4774: 482D39F1  bl 0x831a8164
	ctx.lr = 0x82ED4778;
	sub_831A8130(ctx, base);
	// 82ED4778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED477C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED4780: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED4784: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ED4788: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82ED478C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82ED4790: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED4794: 409A0030  bne cr6, 0x82ed47c4
	if !ctx.cr[6].eq {
	pc = 0x82ED47C4; continue 'dispatch;
	}
	// 82ED4798: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED479C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED47A0: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82ED47A4: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82ED47A8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED47AC: 4BFCBF85  bl 0x82ea0730
	ctx.lr = 0x82ED47B0;
	sub_82EA0730(ctx, base);
	// 82ED47B0: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82ED47B4: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82ED47B8: 48048339  bl 0x82f1caf0
	ctx.lr = 0x82ED47BC;
	sub_82F1CAF0(ctx, base);
	// 82ED47BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED47C0: 4800001C  b 0x82ed47dc
	pc = 0x82ED47DC; continue 'dispatch;
	// 82ED47C4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED47C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED47CC: 419A0010  beq cr6, 0x82ed47dc
	if ctx.cr[6].eq {
	pc = 0x82ED47DC; continue 'dispatch;
	}
	// 82ED47D0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED47D4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ED47D8: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED47DC: 807E0070  lwz r3, 0x70(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED47E0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED47E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED47E8: 419A0030  beq cr6, 0x82ed4818
	if ctx.cr[6].eq {
	pc = 0x82ED4818; continue 'dispatch;
	}
	// 82ED47EC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED47F0: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED47F4: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED47F8: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED47FC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED4800: 409A0018  bne cr6, 0x82ed4818
	if !ctx.cr[6].eq {
	pc = 0x82ED4818; continue 'dispatch;
	}
	// 82ED4804: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4808: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED480C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4810: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED4814: 4E800421  bctrl
	ctx.lr = 0x82ED4818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED4818: 93FE0070  stw r31, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82ED481C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED4820: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82ED4824: 409A0008  bne cr6, 0x82ed482c
	if !ctx.cr[6].eq {
	pc = 0x82ED482C; continue 'dispatch;
	}
	// 82ED4828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED482C: 815E006C  lwz r10, 0x6c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED4830: 7FA90774  extsb r9, r29
	ctx.r[9].s64 = ctx.r[29].s8 as i64;
	// 82ED4834: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED4838: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82ED483C: 419A0014  beq cr6, 0x82ed4850
	if ctx.cr[6].eq {
	pc = 0x82ED4850; continue 'dispatch;
	}
	// 82ED4840: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82ED4844: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ED4848: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED484C: 4BFFE19D  bl 0x82ed29e8
	ctx.lr = 0x82ED4850;
	sub_82ED29E8(ctx, base);
	// 82ED4850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED4854: 482D3960  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4858 size=160
    let mut pc: u32 = 0x82ED4858;
    'dispatch: loop {
        match pc {
            0x82ED4858 => {
    //   block [0x82ED4858..0x82ED48F8)
	// 82ED4858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED485C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4860: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED4864: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4868: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED486C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4870: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED4874: 4804F0BD  bl 0x82f23930
	ctx.lr = 0x82ED4878;
	sub_82F23930(ctx, base);
	// 82ED4878: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED487C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED4880: 419A005C  beq cr6, 0x82ed48dc
	if ctx.cr[6].eq {
	pc = 0x82ED48DC; continue 'dispatch;
	}
	// 82ED4884: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED4888: 419A0054  beq cr6, 0x82ed48dc
	if ctx.cr[6].eq {
	pc = 0x82ED48DC; continue 'dispatch;
	}
	// 82ED488C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4890: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED4894: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED4898: 812B0094  lwz r9, 0x94(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 82ED489C: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED48A0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED48A4: 4198001C  blt cr6, 0x82ed48c0
	if ctx.cr[6].lt {
	pc = 0x82ED48C0; continue 'dispatch;
	}
	// 82ED48A8: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82ED48AC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ED48B0: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82ED48B4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED48B8: 4BFCB7A9  bl 0x82ea0060
	ctx.lr = 0x82ED48BC;
	sub_82EA0060(ctx, base);
	// 82ED48BC: 48000020  b 0x82ed48dc
	pc = 0x82ED48DC; continue 'dispatch;
	// 82ED48C0: 812B0094  lwz r9, 0x94(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 82ED48C4: 394B0090  addi r10, r11, 0x90
	ctx.r[10].s64 = ctx.r[11].s64 + 144;
	// 82ED48C8: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED48CC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82ED48D0: 912B0094  stw r9, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82ED48D4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED48D8: 93EB0090  stw r31, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82ED48DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED48E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED48E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED48E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED48EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED48F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED48F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED48F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED48F8 size=160
    let mut pc: u32 = 0x82ED48F8;
    'dispatch: loop {
        match pc {
            0x82ED48F8 => {
    //   block [0x82ED48F8..0x82ED4998)
	// 82ED48F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED48FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED4904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED490C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED4914: 48012E55  bl 0x82ee7768
	ctx.lr = 0x82ED4918;
	sub_82EE7768(ctx, base);
	// 82ED4918: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED491C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED4920: 419A005C  beq cr6, 0x82ed497c
	if ctx.cr[6].eq {
	pc = 0x82ED497C; continue 'dispatch;
	}
	// 82ED4924: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED4928: 419A0054  beq cr6, 0x82ed497c
	if ctx.cr[6].eq {
	pc = 0x82ED497C; continue 'dispatch;
	}
	// 82ED492C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4930: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED4934: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED4938: 812B006C  lwz r9, 0x6c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED493C: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED4940: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED4944: 4198001C  blt cr6, 0x82ed4960
	if ctx.cr[6].lt {
	pc = 0x82ED4960; continue 'dispatch;
	}
	// 82ED4948: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82ED494C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ED4950: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82ED4954: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED4958: 4BFCB709  bl 0x82ea0060
	ctx.lr = 0x82ED495C;
	sub_82EA0060(ctx, base);
	// 82ED495C: 48000020  b 0x82ed497c
	pc = 0x82ED497C; continue 'dispatch;
	// 82ED4960: 812B006C  lwz r9, 0x6c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED4964: 394B0068  addi r10, r11, 0x68
	ctx.r[10].s64 = ctx.r[11].s64 + 104;
	// 82ED4968: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED496C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82ED4970: 912B006C  stw r9, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82ED4974: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED4978: 93EB0068  stw r31, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82ED497C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED4980: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED4984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED4988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED498C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED4990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED4994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED4998 size=8
    let mut pc: u32 = 0x82ED4998;
    'dispatch: loop {
        match pc {
            0x82ED4998 => {
    //   block [0x82ED4998..0x82ED49A0)
	// 82ED4998: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82ED499C: 48000144  b 0x82ed4ae0
	sub_82ED4AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED49A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED49A0 size=8
    let mut pc: u32 = 0x82ED49A0;
    'dispatch: loop {
        match pc {
            0x82ED49A0 => {
    //   block [0x82ED49A0..0x82ED49A8)
	// 82ED49A0: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82ED49A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED49A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED49A8 size=100
    let mut pc: u32 = 0x82ED49A8;
    'dispatch: loop {
        match pc {
            0x82ED49A8 => {
    //   block [0x82ED49A8..0x82ED4A0C)
	// 82ED49A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED49AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED49B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED49B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED49B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED49BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED49C0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82ED49C4: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82ED49C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED49CC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82ED49D0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED49D4: 419A0020  beq cr6, 0x82ed49f4
	if ctx.cr[6].eq {
	pc = 0x82ED49F4; continue 'dispatch;
	}
	// 82ED49D8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED49DC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED49E0: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82ED49E4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED49E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED49EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED49F0: 4BFCBDC1  bl 0x82ea07b0
	ctx.lr = 0x82ED49F4;
	sub_82EA07B0(ctx, base);
	// 82ED49F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED49F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED49FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED4A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED4A04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED4A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4A10 size=100
    let mut pc: u32 = 0x82ED4A10;
    'dispatch: loop {
        match pc {
            0x82ED4A10 => {
    //   block [0x82ED4A10..0x82ED4A74)
	// 82ED4A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4A18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED4A1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4A20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4A24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4A28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED4A2C: 4804F575  bl 0x82f23fa0
	ctx.lr = 0x82ED4A30;
	sub_82F23FA0(ctx, base);
	// 82ED4A30: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED4A34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED4A38: 419A0020  beq cr6, 0x82ed4a58
	if ctx.cr[6].eq {
	pc = 0x82ED4A58; continue 'dispatch;
	}
	// 82ED4A3C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4A40: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED4A44: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82ED4A48: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED4A4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED4A50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED4A54: 4BFCBD5D  bl 0x82ea07b0
	ctx.lr = 0x82ED4A58;
	sub_82EA07B0(ctx, base);
	// 82ED4A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED4A5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED4A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED4A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED4A68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED4A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED4A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED4A78 size=8
    let mut pc: u32 = 0x82ED4A78;
    'dispatch: loop {
        match pc {
            0x82ED4A78 => {
    //   block [0x82ED4A78..0x82ED4A80)
	// 82ED4A78: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82ED4A7C: 4BFFFF94  b 0x82ed4a10
	sub_82ED4A10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED4A80 size=8
    let mut pc: u32 = 0x82ED4A80;
    'dispatch: loop {
        match pc {
            0x82ED4A80 => {
    //   block [0x82ED4A80..0x82ED4A88)
	// 82ED4A80: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82ED4A84: 4BFFFF8C  b 0x82ed4a10
	sub_82ED4A10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED4A88 size=8
    let mut pc: u32 = 0x82ED4A88;
    'dispatch: loop {
        match pc {
            0x82ED4A88 => {
    //   block [0x82ED4A88..0x82ED4A90)
	// 82ED4A88: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82ED4A8C: 4BFFFF84  b 0x82ed4a10
	sub_82ED4A10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED4A90 size=8
    let mut pc: u32 = 0x82ED4A90;
    'dispatch: loop {
        match pc {
            0x82ED4A90 => {
    //   block [0x82ED4A90..0x82ED4A98)
	// 82ED4A90: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82ED4A94: 4BFFFF7C  b 0x82ed4a10
	sub_82ED4A10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4A98 size=72
    let mut pc: u32 = 0x82ED4A98;
    'dispatch: loop {
        match pc {
            0x82ED4A98 => {
    //   block [0x82ED4A98..0x82ED4AE0)
	// 82ED4A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4AA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4AAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82ED4AB0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82ED4AB4: 392BBEC8  addi r9, r11, -0x4138
	ctx.r[9].s64 = ctx.r[11].s64 + -16696;
	// 82ED4AB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED4ABC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED4AC0: 419A000C  beq cr6, 0x82ed4acc
	if ctx.cr[6].eq {
	pc = 0x82ED4ACC; continue 'dispatch;
	}
	// 82ED4AC4: 4B3EB7A5  bl 0x822c0268
	ctx.lr = 0x82ED4AC8;
	sub_822C0268(ctx, base);
	// 82ED4AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED4ACC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED4AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED4AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED4AD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED4ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4AE0 size=124
    let mut pc: u32 = 0x82ED4AE0;
    'dispatch: loop {
        match pc {
            0x82ED4AE0 => {
    //   block [0x82ED4AE0..0x82ED4B5C)
	// 82ED4AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4AE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4AEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4AF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4AF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED4AF8: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82ED4AFC: 409A0008  bne cr6, 0x82ed4b04
	if !ctx.cr[6].eq {
	pc = 0x82ED4B04; continue 'dispatch;
	}
	// 82ED4B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED4B04: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED4B08: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ED4B0C: 390A41F8  addi r8, r10, 0x41f8
	ctx.r[8].s64 = ctx.r[10].s64 + 16888;
	// 82ED4B10: 38E99EAC  addi r7, r9, -0x6154
	ctx.r[7].s64 = ctx.r[9].s64 + -24916;
	// 82ED4B14: 548607FE  clrlwi r6, r4, 0x1f
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82ED4B18: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED4B1C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED4B20: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82ED4B24: 419A0020  beq cr6, 0x82ed4b44
	if ctx.cr[6].eq {
	pc = 0x82ED4B44; continue 'dispatch;
	}
	// 82ED4B28: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4B2C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED4B30: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82ED4B34: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED4B38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED4B3C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED4B40: 4BFCBC71  bl 0x82ea07b0
	ctx.lr = 0x82ED4B44;
	sub_82EA07B0(ctx, base);
	// 82ED4B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED4B48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED4B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED4B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED4B54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED4B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4B60 size=100
    let mut pc: u32 = 0x82ED4B60;
    'dispatch: loop {
        match pc {
            0x82ED4B60 => {
    //   block [0x82ED4B60..0x82ED4BC4)
	// 82ED4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4B68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED4B6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4B74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4B78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED4B7C: 4BFFC465  bl 0x82ed0fe0
	ctx.lr = 0x82ED4B80;
	sub_82ED0FE0(ctx, base);
	// 82ED4B80: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED4B84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED4B88: 419A0020  beq cr6, 0x82ed4ba8
	if ctx.cr[6].eq {
	pc = 0x82ED4BA8; continue 'dispatch;
	}
	// 82ED4B8C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4B90: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED4B94: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82ED4B98: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED4B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED4BA0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED4BA4: 4BFCBC0D  bl 0x82ea07b0
	ctx.lr = 0x82ED4BA8;
	sub_82EA07B0(ctx, base);
	// 82ED4BA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED4BAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED4BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED4BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED4BB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED4BBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED4BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED4BC8 size=8
    let mut pc: u32 = 0x82ED4BC8;
    'dispatch: loop {
        match pc {
            0x82ED4BC8 => {
    //   block [0x82ED4BC8..0x82ED4BD0)
	// 82ED4BC8: 386300A0  addi r3, r3, 0xa0
	ctx.r[3].s64 = ctx.r[3].s64 + 160;
	// 82ED4BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED4BD0 size=4
    let mut pc: u32 = 0x82ED4BD0;
    'dispatch: loop {
        match pc {
            0x82ED4BD0 => {
    //   block [0x82ED4BD0..0x82ED4BD4)
	// 82ED4BD0: 48001010  b 0x82ed5be0
	sub_82ED5BE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4BD8 size=336
    let mut pc: u32 = 0x82ED4BD8;
    'dispatch: loop {
        match pc {
            0x82ED4BD8 => {
    //   block [0x82ED4BD8..0x82ED4D28)
	// 82ED4BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4BE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED4BE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4BEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4BF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED4BF4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4BF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED4BFC: 419A0050  beq cr6, 0x82ed4c4c
	if ctx.cr[6].eq {
	pc = 0x82ED4C4C; continue 'dispatch;
	}
	// 82ED4C00: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED4C04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED4C08: 419A0024  beq cr6, 0x82ed4c2c
	if ctx.cr[6].eq {
	pc = 0x82ED4C2C; continue 'dispatch;
	}
	// 82ED4C0C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82ED4C10: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ED4C14: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82ED4C18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED4C1C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED4C20: 4BFF83E1  bl 0x82ecd000
	ctx.lr = 0x82ED4C24;
	sub_82ECD000(ctx, base);
	// 82ED4C24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ED4C28: 480000E8  b 0x82ed4d10
	pc = 0x82ED4D10; continue 'dispatch;
	// 82ED4C2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED4C30: 419A001C  beq cr6, 0x82ed4c4c
	if ctx.cr[6].eq {
	pc = 0x82ED4C4C; continue 'dispatch;
	}
	// 82ED4C34: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED4C38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED4C3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED4C40: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED4C44: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4C48: 4800CA01  bl 0x82ee1648
	ctx.lr = 0x82ED4C4C;
	sub_82EE1648(ctx, base);
	// 82ED4C4C: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED4C50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED4C54: 419A0010  beq cr6, 0x82ed4c64
	if ctx.cr[6].eq {
	pc = 0x82ED4C64; continue 'dispatch;
	}
	// 82ED4C58: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED4C5C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ED4C60: B15E0006  sth r10, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED4C64: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED4C68: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82ED4C6C: 419A003C  beq cr6, 0x82ed4ca8
	if ctx.cr[6].eq {
	pc = 0x82ED4CA8; continue 'dispatch;
	}
	// 82ED4C70: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED4C74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED4C78: 419A0030  beq cr6, 0x82ed4ca8
	if ctx.cr[6].eq {
	pc = 0x82ED4CA8; continue 'dispatch;
	}
	// 82ED4C7C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED4C80: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED4C84: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED4C88: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED4C8C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED4C90: 409A0018  bne cr6, 0x82ed4ca8
	if !ctx.cr[6].eq {
	pc = 0x82ED4CA8; continue 'dispatch;
	}
	// 82ED4C94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4C98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED4C9C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4CA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED4CA4: 4E800421  bctrl
	ctx.lr = 0x82ED4CA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED4CA8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4CAC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82ED4CB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED4CB4: 419A000C  beq cr6, 0x82ed4cc0
	if ctx.cr[6].eq {
	pc = 0x82ED4CC0; continue 'dispatch;
	}
	// 82ED4CB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED4CBC: 4800F1FD  bl 0x82ee3eb8
	ctx.lr = 0x82ED4CC0;
	sub_82EE3EB8(ctx, base);
	// 82ED4CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED4CC4: 48000C2D  bl 0x82ed58f0
	ctx.lr = 0x82ED4CC8;
	sub_82ED58F0(ctx, base);
	// 82ED4CC8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4CCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED4CD0: 419A003C  beq cr6, 0x82ed4d0c
	if ctx.cr[6].eq {
	pc = 0x82ED4D0C; continue 'dispatch;
	}
	// 82ED4CD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED4CD8: 4800C6F1  bl 0x82ee13c8
	ctx.lr = 0x82ED4CDC;
	sub_82EE13C8(ctx, base);
	// 82ED4CDC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4CE0: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED4CE4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED4CE8: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED4CEC: 40820020  bne 0x82ed4d0c
	if !ctx.cr[0].eq {
	pc = 0x82ED4D0C; continue 'dispatch;
	}
	// 82ED4CF0: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED4CF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED4CF8: 409A0014  bne cr6, 0x82ed4d0c
	if !ctx.cr[6].eq {
	pc = 0x82ED4D0C; continue 'dispatch;
	}
	// 82ED4CFC: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED4D00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED4D04: 419A0008  beq cr6, 0x82ed4d0c
	if ctx.cr[6].eq {
	pc = 0x82ED4D0C; continue 'dispatch;
	}
	// 82ED4D08: 4BFF82E1  bl 0x82eccfe8
	ctx.lr = 0x82ED4D0C;
	sub_82ECCFE8(ctx, base);
	// 82ED4D0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82ED4D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED4D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED4D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED4D1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED4D20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED4D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4D28 size=156
    let mut pc: u32 = 0x82ED4D28;
    'dispatch: loop {
        match pc {
            0x82ED4D28 => {
    //   block [0x82ED4D28..0x82ED4DC4)
	// 82ED4D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4D30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4D34: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4D38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4DC8 size=124
    let mut pc: u32 = 0x82ED4DC8;
    'dispatch: loop {
        match pc {
            0x82ED4DC8 => {
    //   block [0x82ED4DC8..0x82ED4E44)
	// 82ED4DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4DD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4DD4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4DD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ED4E48 size=56
    let mut pc: u32 = 0x82ED4E48;
    'dispatch: loop {
        match pc {
            0x82ED4E48 => {
    //   block [0x82ED4E48..0x82ED4E80)
	// 82ED4E48: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED4E4C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82ED4E50: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED4E54: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82ED4E58: 388300A0  addi r4, r3, 0xa0
	ctx.r[4].s64 = ctx.r[3].s64 + 160;
	// 82ED4E5C: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82ED4E60: 810B006C  lwz r8, 0x6c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED4E64: C00A9450  lfs f0, -0x6bb0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED4E68: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4E6C: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED4E70: 80A7001C  lwz r5, 0x1c(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ED4E74: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82ED4E78: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82ED4E7C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4E80 size=132
    let mut pc: u32 = 0x82ED4E80;
    'dispatch: loop {
        match pc {
            0x82ED4E80 => {
    //   block [0x82ED4E80..0x82ED4F04)
	// 82ED4E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4E84: 482D32E9  bl 0x831a816c
	ctx.lr = 0x82ED4E88;
	sub_831A8130(ctx, base);
	// 82ED4E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4E8C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ED4E90: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ED4E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4E98: 48000721  bl 0x82ed55b8
	ctx.lr = 0x82ED4E9C;
	sub_82ED55B8(ctx, base);
	// 82ED4E9C: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82ED4EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED4EA4: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82ED4EA8: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED4EAC: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ED4EB0: 7D4AF850  subf r10, r10, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 82ED4EB4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED4EB8: 913F0088  stw r9, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[9].u32 ) };
	// 82ED4EBC: 38C84784  addi r6, r8, 0x4784
	ctx.r[6].s64 = ctx.r[8].s64 + 18308;
	// 82ED4EC0: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82ED4EC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED4EC8: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82ED4ECC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED4ED0: 913F0094  stw r9, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82ED4ED4: 995F0020  stb r10, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 82ED4ED8: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82ED4EDC: 4BFD2575  bl 0x82ea7450
	ctx.lr = 0x82ED4EE0;
	sub_82EA7450(ctx, base);
	// 82ED4EE0: 3BDF00A0  addi r30, r31, 0xa0
	ctx.r[30].s64 = ctx.r[31].s64 + 160;
	// 82ED4EE4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82ED4EE8: 389D0030  addi r4, r29, 0x30
	ctx.r[4].s64 = ctx.r[29].s64 + 48;
	// 82ED4EEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED4EF0: 4832AA21  bl 0x831ff910
	ctx.lr = 0x82ED4EF4;
	sub_831FF910(ctx, base);
	// 82ED4EF4: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82ED4EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED4EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED4F00: 482D32BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4F08 size=220
    let mut pc: u32 = 0x82ED4F08;
    'dispatch: loop {
        match pc {
            0x82ED4F08 => {
    //   block [0x82ED4F08..0x82ED4FE4)
	// 82ED4F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4F10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED4F1C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED4F20: 394B4734  addi r10, r11, 0x4734
	ctx.r[10].s64 = ctx.r[11].s64 + 18228;
	// 82ED4F24: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED4F28: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED4F2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82ED4F30: 419A003C  beq cr6, 0x82ed4f6c
	if ctx.cr[6].eq {
	pc = 0x82ED4F6C; continue 'dispatch;
	}
	// 82ED4F34: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED4F38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED4F3C: 419A0030  beq cr6, 0x82ed4f6c
	if ctx.cr[6].eq {
	pc = 0x82ED4F6C; continue 'dispatch;
	}
	// 82ED4F40: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED4F44: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED4F48: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED4F4C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED4F50: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED4F54: 409A0018  bne cr6, 0x82ed4f6c
	if !ctx.cr[6].eq {
	pc = 0x82ED4F6C; continue 'dispatch;
	}
	// 82ED4F58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4F5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED4F60: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4F64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED4F68: 4E800421  bctrl
	ctx.lr = 0x82ED4F6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED4F6C: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED4F70: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED4F74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED4F78: 409A0020  bne cr6, 0x82ed4f98
	if !ctx.cr[6].eq {
	pc = 0x82ED4F98; continue 'dispatch;
	}
	// 82ED4F7C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4F80: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED4F84: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED4F88: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED4F8C: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED4F90: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED4F94: 4BFCB81D  bl 0x82ea07b0
	ctx.lr = 0x82ED4F98;
	sub_82EA07B0(ctx, base);
	// 82ED4F98: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED4F9C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED4FA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED4FA4: 409A0020  bne cr6, 0x82ed4fc4
	if !ctx.cr[6].eq {
	pc = 0x82ED4FC4; continue 'dispatch;
	}
	// 82ED4FA8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED4FAC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED4FB0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED4FB4: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ED4FB8: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED4FBC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED4FC0: 4BFCB7F1  bl 0x82ea07b0
	ctx.lr = 0x82ED4FC4;
	sub_82EA07B0(ctx, base);
	// 82ED4FC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED4FC8: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82ED4FCC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED4FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED4FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED4FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED4FDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED4FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED4FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED4FE8 size=100
    let mut pc: u32 = 0x82ED4FE8;
    'dispatch: loop {
        match pc {
            0x82ED4FE8 => {
    //   block [0x82ED4FE8..0x82ED504C)
	// 82ED4FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED4FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED4FF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED4FF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED4FF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED4FFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED5000: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED5004: 4BFFFF05  bl 0x82ed4f08
	ctx.lr = 0x82ED5008;
	sub_82ED4F08(ctx, base);
	// 82ED5008: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED500C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED5010: 419A0020  beq cr6, 0x82ed5030
	if ctx.cr[6].eq {
	pc = 0x82ED5030; continue 'dispatch;
	}
	// 82ED5014: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5018: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED501C: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82ED5020: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5024: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED5028: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED502C: 4BFCB785  bl 0x82ea07b0
	ctx.lr = 0x82ED5030;
	sub_82EA07B0(ctx, base);
	// 82ED5030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED5034: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED5038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED503C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5040: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED5044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED5048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5050 size=100
    let mut pc: u32 = 0x82ED5050;
    'dispatch: loop {
        match pc {
            0x82ED5050 => {
    //   block [0x82ED5050..0x82ED50B4)
	// 82ED5050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED5058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED505C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED5060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5064: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED5068: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED506C: 48000C35  bl 0x82ed5ca0
	ctx.lr = 0x82ED5070;
	sub_82ED5CA0(ctx, base);
	// 82ED5070: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED5074: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED5078: 419A0020  beq cr6, 0x82ed5098
	if ctx.cr[6].eq {
	pc = 0x82ED5098; continue 'dispatch;
	}
	// 82ED507C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5080: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED5084: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82ED5088: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED508C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED5090: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED5094: 4BFCB71D  bl 0x82ea07b0
	ctx.lr = 0x82ED5098;
	sub_82EA07B0(ctx, base);
	// 82ED5098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED509C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED50A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED50A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED50A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED50AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED50B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED50B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED50B8 size=8
    let mut pc: u32 = 0x82ED50B8;
    'dispatch: loop {
        match pc {
            0x82ED50B8 => {
    //   block [0x82ED50B8..0x82ED50C0)
	// 82ED50B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82ED50BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED50C0 size=12
    let mut pc: u32 = 0x82ED50C0;
    'dispatch: loop {
        match pc {
            0x82ED50C0 => {
    //   block [0x82ED50C0..0x82ED50CC)
	// 82ED50C0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED50C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED50C8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED50CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED50CC size=12
    let mut pc: u32 = 0x82ED50CC;
    'dispatch: loop {
        match pc {
            0x82ED50CC => {
    //   block [0x82ED50CC..0x82ED50D8)
	// 82ED50CC: 806B00B4  lwz r3, 0xb4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 82ED50D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED50D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED50D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED50D8 size=12
    let mut pc: u32 = 0x82ED50D8;
    'dispatch: loop {
        match pc {
            0x82ED50D8 => {
    //   block [0x82ED50D8..0x82ED50E4)
	// 82ED50D8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED50DC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82ED50E0: 4836D87C  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED50E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED50E4 size=4
    let mut pc: u32 = 0x82ED50E4;
    'dispatch: loop {
        match pc {
            0x82ED50E4 => {
    //   block [0x82ED50E4..0x82ED50E8)
	// 82ED50E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED50E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED50E8 size=12
    let mut pc: u32 = 0x82ED50E8;
    'dispatch: loop {
        match pc {
            0x82ED50E8 => {
    //   block [0x82ED50E8..0x82ED50F4)
	// 82ED50E8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED50EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED50F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED50F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED50F4 size=16
    let mut pc: u32 = 0x82ED50F4;
    'dispatch: loop {
        match pc {
            0x82ED50F4 => {
    //   block [0x82ED50F4..0x82ED5104)
	// 82ED50F4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED50F8: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ED50FC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED5100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5108 size=12
    let mut pc: u32 = 0x82ED5108;
    'dispatch: loop {
        match pc {
            0x82ED5108 => {
    //   block [0x82ED5108..0x82ED5114)
	// 82ED5108: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED510C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED5110: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5114 size=24
    let mut pc: u32 = 0x82ED5114;
    'dispatch: loop {
        match pc {
            0x82ED5114 => {
    //   block [0x82ED5114..0x82ED512C)
	// 82ED5114: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED5118: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED511C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED5120: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED5124: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED5128: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED512C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED512C size=20
    let mut pc: u32 = 0x82ED512C;
    'dispatch: loop {
        match pc {
            0x82ED512C => {
    //   block [0x82ED512C..0x82ED5140)
	// 82ED512C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5130: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED5134: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5138: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED513C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5140 size=4
    let mut pc: u32 = 0x82ED5140;
    'dispatch: loop {
        match pc {
            0x82ED5140 => {
    //   block [0x82ED5140..0x82ED5144)
	// 82ED5140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5148 size=4
    let mut pc: u32 = 0x82ED5148;
    'dispatch: loop {
        match pc {
            0x82ED5148 => {
    //   block [0x82ED5148..0x82ED514C)
	// 82ED5148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5150 size=4
    let mut pc: u32 = 0x82ED5150;
    'dispatch: loop {
        match pc {
            0x82ED5150 => {
    //   block [0x82ED5150..0x82ED5154)
	// 82ED5150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5158 size=4
    let mut pc: u32 = 0x82ED5158;
    'dispatch: loop {
        match pc {
            0x82ED5158 => {
    //   block [0x82ED5158..0x82ED515C)
	// 82ED5158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5160 size=144
    let mut pc: u32 = 0x82ED5160;
    'dispatch: loop {
        match pc {
            0x82ED5160 => {
    //   block [0x82ED5160..0x82ED51F0)
	// 82ED5160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5164: 482D3009  bl 0x831a816c
	ctx.lr = 0x82ED5168;
	sub_831A8130(ctx, base);
	// 82ED5168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED516C: 81230078  lwz r9, 0x78(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED5170: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED5174: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ED5178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED517C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED5180: 40990024  ble cr6, 0x82ed51a4
	if !ctx.cr[6].gt {
	pc = 0x82ED51A4; continue 'dispatch;
	}
	// 82ED5184: 81630074  lwz r11, 0x74(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED5188: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED518C: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82ED5190: 419A0058  beq cr6, 0x82ed51e8
	if ctx.cr[6].eq {
	pc = 0x82ED51E8; continue 'dispatch;
	}
	// 82ED5194: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED5198: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82ED519C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED51A0: 4198FFE8  blt cr6, 0x82ed5188
	if ctx.cr[6].lt {
	pc = 0x82ED5188; continue 'dispatch;
	}
	// 82ED51A4: 8163007C  lwz r11, 0x7c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED51A8: 3BE30074  addi r31, r3, 0x74
	ctx.r[31].s64 = ctx.r[3].s64 + 116;
	// 82ED51AC: 81430078  lwz r10, 0x78(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED51B0: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED51B4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED51B8: 409A0010  bne cr6, 0x82ed51c8
	if !ctx.cr[6].eq {
	pc = 0x82ED51C8; continue 'dispatch;
	}
	// 82ED51BC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82ED51C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED51C4: 4BFD16BD  bl 0x82ea6880
	ctx.lr = 0x82ED51C8;
	sub_82EA6880(ctx, base);
	// 82ED51C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED51CC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED51D0: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED51D4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED51D8: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82ED51DC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED51E0: 7FCA492E  stwx r30, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 82ED51E4: FBAB0008  std r29, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 82ED51E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED51EC: 482D2FD0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED51F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED51F0 size=68
    let mut pc: u32 = 0x82ED51F0;
    'dispatch: loop {
        match pc {
            0x82ED51F0 => {
    //   block [0x82ED51F0..0x82ED5234)
	// 82ED51F0: 81240078  lwz r9, 0x78(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED51F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED51F8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED51FC: 4099002C  ble cr6, 0x82ed5228
	if !ctx.cr[6].gt {
	pc = 0x82ED5228; continue 'dispatch;
	}
	// 82ED5200: 80E40074  lwz r7, 0x74(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED5204: 39040074  addi r8, r4, 0x74
	ctx.r[8].s64 = ctx.r[4].s64 + 116;
	// 82ED5208: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82ED520C: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5210: 7F062840  cmplw cr6, r6, r5
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82ED5214: 419A0020  beq cr6, 0x82ed5234
	if ctx.cr[6].eq {
		sub_82ED5234(ctx, base);
		return;
	}
	// 82ED5218: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED521C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82ED5220: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED5224: 4198FFE8  blt cr6, 0x82ed520c
	if ctx.cr[6].lt {
	pc = 0x82ED520C; continue 'dispatch;
	}
	// 82ED5228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED522C: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82ED5230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5234(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5234 size=112
    let mut pc: u32 = 0x82ED5234;
    'dispatch: loop {
        match pc {
            0x82ED5234 => {
    //   block [0x82ED5234..0x82ED52A4)
	// 82ED5234: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED5238: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 82ED523C: 7CA74A14  add r5, r7, r9
	ctx.r[5].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82ED5240: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82ED5244: 7C87482A  ldx r4, r7, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) };
	// 82ED5248: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED524C: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82ED5250: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82ED5254: 90E80004  stw r7, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED5258: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82ED525C: F8860000  std r4, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 82ED5260: F8A60008  std r5, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82ED5264: 40980034  bge cr6, 0x82ed5298
	if !ctx.cr[6].lt {
	pc = 0x82ED5298; continue 'dispatch;
	}
	// 82ED5268: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED526C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED5270: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82ED5274: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82ED5278: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 82ED527C: E8CB0010  ld r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82ED5280: F8CB0000  std r6, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 82ED5284: E8AB0018  ld r5, 0x18(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82ED5288: F8AB0008  std r5, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82ED528C: 80880004  lwz r4, 4(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5290: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82ED5294: 4198FFD4  blt cr6, 0x82ed5268
	if ctx.cr[6].lt {
	pc = 0x82ED5268; continue 'dispatch;
	}
	// 82ED5298: E961FFF8  ld r11, -8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82ED529C: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82ED52A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED52A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED52A8 size=16
    let mut pc: u32 = 0x82ED52A8;
    'dispatch: loop {
        match pc {
            0x82ED52A8 => {
    //   block [0x82ED52A8..0x82ED52B8)
	// 82ED52A8: 81230078  lwz r9, 0x78(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED52AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED52B0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED52B4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED52B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED52B8 size=40
    let mut pc: u32 = 0x82ED52B8;
    'dispatch: loop {
        match pc {
            0x82ED52B8 => {
    //   block [0x82ED52B8..0x82ED52E0)
	// 82ED52B8: 81030074  lwz r8, 0x74(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED52BC: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82ED52C0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED52C4: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ED52C8: 419A0018  beq cr6, 0x82ed52e0
	if ctx.cr[6].eq {
		sub_82ED52E0(ctx, base);
		return;
	}
	// 82ED52CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED52D0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82ED52D4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED52D8: 4198FFE8  blt cr6, 0x82ed52c0
	if ctx.cr[6].lt {
	pc = 0x82ED52C0; continue 'dispatch;
	}
	// 82ED52DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED52E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED52E0 size=16
    let mut pc: u32 = 0x82ED52E0;
    'dispatch: loop {
        match pc {
            0x82ED52E0 => {
    //   block [0x82ED52E0..0x82ED52F0)
	// 82ED52E0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED52E4: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82ED52E8: F8AB0008  std r5, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82ED52EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED52F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED52F0 size=12
    let mut pc: u32 = 0x82ED52F0;
    'dispatch: loop {
        match pc {
            0x82ED52F0 => {
    //   block [0x82ED52F0..0x82ED52FC)
	// 82ED52F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED52F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED52F8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED52FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED52FC size=12
    let mut pc: u32 = 0x82ED52FC;
    'dispatch: loop {
        match pc {
            0x82ED52FC => {
    //   block [0x82ED52FC..0x82ED5308)
	// 82ED52FC: 806B00B4  lwz r3, 0xb4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 82ED5300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED5304: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5308 size=8
    let mut pc: u32 = 0x82ED5308;
    'dispatch: loop {
        match pc {
            0x82ED5308 => {
    //   block [0x82ED5308..0x82ED5310)
	// 82ED5308: 4BFCCDA8  b 0x82ea20b0
	sub_82EA20B0(ctx, base);
	return;
	// 82ED530C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5310 size=116
    let mut pc: u32 = 0x82ED5310;
    'dispatch: loop {
        match pc {
            0x82ED5310 => {
    //   block [0x82ED5310..0x82ED5384)
	// 82ED5310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED5318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED531C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED5320: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5324: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82ED5328: 419A0034  beq cr6, 0x82ed535c
	if ctx.cr[6].eq {
	pc = 0x82ED535C; continue 'dispatch;
	}
	// 82ED532C: 81430084  lwz r10, 0x84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5330: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED5334: 419A0028  beq cr6, 0x82ed535c
	if ctx.cr[6].eq {
	pc = 0x82ED535C; continue 'dispatch;
	}
	// 82ED5338: 3940001D  li r10, 0x1d
	ctx.r[10].s64 = 29;
	// 82ED533C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82ED5340: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED5344: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82ED5348: 4BFF7CB9  bl 0x82ecd000
	ctx.lr = 0x82ED534C;
	sub_82ECD000(ctx, base);
	// 82ED534C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED5350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED5354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5358: 4E800020  blr
	return;
	// 82ED535C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5360: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED5364: 419A0010  beq cr6, 0x82ed5374
	if ctx.cr[6].eq {
	pc = 0x82ED5374; continue 'dispatch;
	}
	// 82ED5368: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED536C: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82ED5370: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED5374: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED5378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED537C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5388 size=148
    let mut pc: u32 = 0x82ED5388;
    'dispatch: loop {
        match pc {
            0x82ED5388 => {
    //   block [0x82ED5388..0x82ED541C)
	// 82ED5388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED5390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5394: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5398: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED539C: 419A0038  beq cr6, 0x82ed53d4
	if ctx.cr[6].eq {
	pc = 0x82ED53D4; continue 'dispatch;
	}
	// 82ED53A0: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED53A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED53A8: 419A002C  beq cr6, 0x82ed53d4
	if ctx.cr[6].eq {
	pc = 0x82ED53D4; continue 'dispatch;
	}
	// 82ED53AC: 3940001E  li r10, 0x1e
	ctx.r[10].s64 = 30;
	// 82ED53B0: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82ED53B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED53B8: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82ED53BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED53C0: 4BFF7C41  bl 0x82ecd000
	ctx.lr = 0x82ED53C4;
	sub_82ECD000(ctx, base);
	// 82ED53C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED53C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED53CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED53D0: 4E800020  blr
	return;
	// 82ED53D4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED53D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED53DC: 419A0030  beq cr6, 0x82ed540c
	if ctx.cr[6].eq {
	pc = 0x82ED540C; continue 'dispatch;
	}
	// 82ED53E0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED53E4: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED53E8: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED53EC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED53F0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED53F4: 409A0018  bne cr6, 0x82ed540c
	if !ctx.cr[6].eq {
	pc = 0x82ED540C; continue 'dispatch;
	}
	// 82ED53F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED53FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED5400: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5404: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED5408: 4E800421  bctrl
	ctx.lr = 0x82ED540C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED540C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED5410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED5414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5420 size=272
    let mut pc: u32 = 0x82ED5420;
    'dispatch: loop {
        match pc {
            0x82ED5420 => {
    //   block [0x82ED5420..0x82ED5530)
	// 82ED5420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED5428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED542C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED5430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5434: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED5438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED543C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED5440: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ED5444: 388A47EC  addi r4, r10, 0x47ec
	ctx.r[4].s64 = ctx.r[10].s64 + 18412;
	// 82ED5448: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED544C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED5450: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED5454: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED5458: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED545C: 4E800421  bctrl
	ctx.lr = 0x82ED5460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5460: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED5464: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5468: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED546C: 409A0034  bne cr6, 0x82ed54a0
	if !ctx.cr[6].eq {
	pc = 0x82ED54A0; continue 'dispatch;
	}
	// 82ED5470: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5474: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED5478: 80FF0060  lwz r7, 0x60(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED547C: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED5480: 80DF005C  lwz r6, 0x5c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ED5484: 388947E0  addi r4, r9, 0x47e0
	ctx.r[4].s64 = ctx.r[9].s64 + 18400;
	// 82ED5488: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED548C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ED5490: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED5498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED549C: 4E800421  bctrl
	ctx.lr = 0x82ED54A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED54A0: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED54A4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED54A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED54AC: 409A0034  bne cr6, 0x82ed54e0
	if !ctx.cr[6].eq {
	pc = 0x82ED54E0; continue 'dispatch;
	}
	// 82ED54B0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED54B4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED54B8: 80FF0078  lwz r7, 0x78(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED54BC: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED54C0: 80DF0074  lwz r6, 0x74(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED54C4: 388947D4  addi r4, r9, 0x47d4
	ctx.r[4].s64 = ctx.r[9].s64 + 18388;
	// 82ED54C8: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED54CC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED54D0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED54D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED54D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED54DC: 4E800421  bctrl
	ctx.lr = 0x82ED54E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED54E0: 80DF0054  lwz r6, 0x54(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED54E4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82ED54E8: 419A0030  beq cr6, 0x82ed5518
	if ctx.cr[6].eq {
	pc = 0x82ED5518; continue 'dispatch;
	}
	// 82ED54EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED54F0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED54F4: A13F0050  lhz r9, 0x50(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED54F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ED54FC: 388A47C4  addi r4, r10, 0x47c4
	ctx.r[4].s64 = ctx.r[10].s64 + 18372;
	// 82ED5500: 5527283E  rotlwi r7, r9, 5
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(5)) as u64;
	// 82ED5504: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ED5508: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED550C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED5510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED5514: 4E800421  bctrl
	ctx.lr = 0x82ED5518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED551C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED5520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5524: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED5528: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED552C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5530 size=132
    let mut pc: u32 = 0x82ED5530;
    'dispatch: loop {
        match pc {
            0x82ED5530 => {
    //   block [0x82ED5530..0x82ED55B4)
	// 82ED5530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5534: 482D2C39  bl 0x831a816c
	ctx.lr = 0x82ED5538;
	sub_831A8130(ctx, base);
	// 82ED5538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED553C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED5540: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED5544: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED5548: 392B4734  addi r9, r11, 0x4734
	ctx.r[9].s64 = ctx.r[11].s64 + 18228;
	// 82ED554C: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 82ED5550: B15E0006  sth r10, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED5554: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED5558: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED555C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82ED5560: 4804DBA9  bl 0x82f23108
	ctx.lr = 0x82ED5564;
	sub_82F23108(ctx, base);
	// 82ED5564: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED5568: 419A0010  beq cr6, 0x82ed5578
	if ctx.cr[6].eq {
	pc = 0x82ED5578; continue 'dispatch;
	}
	// 82ED556C: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82ED5570: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82ED5574: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82ED5578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED557C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82ED5580: 3920FFD1  li r9, -0x2f
	ctx.r[9].s64 = -47;
	// 82ED5584: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82ED5588: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82ED558C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED5590: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82ED5594: 913E0068  stw r9, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82ED5598: B17E006C  sth r11, 0x6c(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[11].u16 ) };
	// 82ED559C: 419A000C  beq cr6, 0x82ed55a8
	if ctx.cr[6].eq {
	pc = 0x82ED55A8; continue 'dispatch;
	}
	// 82ED55A0: 7D7FF050  subf r11, r31, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82ED55A4: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82ED55A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED55AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED55B0: 482D2C0C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED55B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED55B8 size=156
    let mut pc: u32 = 0x82ED55B8;
    'dispatch: loop {
        match pc {
            0x82ED55B8 => {
    //   block [0x82ED55B8..0x82ED5654)
	// 82ED55B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED55BC: 482D2BAD  bl 0x831a8168
	ctx.lr = 0x82ED55C0;
	sub_831A8130(ctx, base);
	// 82ED55C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED55C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED55C8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED55CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED55D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED55D4: 392B4734  addi r9, r11, 0x4734
	ctx.r[9].s64 = ctx.r[11].s64 + 18228;
	// 82ED55D8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82ED55DC: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 82ED55E0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED55E4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82ED55E8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED55EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82ED55F0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82ED55F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ED55F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED55FC: 4800005D  bl 0x82ed5658
	ctx.lr = 0x82ED5600;
	sub_82ED5658(ctx, base);
	// 82ED5600: 3900FFD1  li r8, -0x2f
	ctx.r[8].s64 = -47;
	// 82ED5604: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82ED5608: B3DF006C  sth r30, 0x6c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u16 ) };
	// 82ED560C: 911F0068  stw r8, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82ED5610: 7CFCF850  subf r7, r28, r31
	ctx.r[7].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 82ED5614: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82ED5618: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82ED561C: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82ED5620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED5624: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82ED5628: 90DF007C  stw r6, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[6].u32 ) };
	// 82ED562C: 98FF0020  stb r7, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u8 ) };
	// 82ED5630: 419A001C  beq cr6, 0x82ed564c
	if ctx.cr[6].eq {
	pc = 0x82ED564C; continue 'dispatch;
	}
	// 82ED5634: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED563C: 419A0010  beq cr6, 0x82ed564c
	if ctx.cr[6].eq {
	pc = 0x82ED564C; continue 'dispatch;
	}
	// 82ED5640: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED5644: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ED5648: B15D0006  sth r10, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED564C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED5650: 482D2B68  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ED5658 size=136
    let mut pc: u32 = 0x82ED5658;
    'dispatch: loop {
        match pc {
            0x82ED5658 => {
    //   block [0x82ED5658..0x82ED56E0)
	// 82ED5658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED565C: 482D2B11  bl 0x831a816c
	ctx.lr = 0x82ED5660;
	sub_831A8130(ctx, base);
	// 82ED5660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5664: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED5668: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED566C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED5670: 3920007F  li r9, 0x7f
	ctx.r[9].s64 = 127;
	// 82ED5674: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82ED5678: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82ED567C: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 82ED5680: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82ED5684: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82ED5688: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82ED568C: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82ED5690: 98DF0018  stb r6, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u8 ) };
	// 82ED5694: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82ED5698: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82ED569C: B3DF001A  sth r30, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[30].u16 ) };
	// 82ED56A0: 993F0019  stb r9, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[9].u8 ) };
	// 82ED56A4: 4804DA65  bl 0x82f23108
	ctx.lr = 0x82ED56A8;
	sub_82F23108(ctx, base);
	// 82ED56A8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82ED56AC: 7CFDF850  subf r7, r29, r31
	ctx.r[7].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 82ED56B0: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82ED56B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED56B8: C0089534  lfs f0, -0x6acc(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED56BC: D01F0048  stfs f0, 0x48(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82ED56C0: 98FF0019  stb r7, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[7].u8 ) };
	// 82ED56C4: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 82ED56C8: B3DF0012  sth r30, 0x12(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[30].u16 ) };
	// 82ED56CC: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82ED56D0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82ED56D4: 90DF0054  stw r6, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82ED56D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED56DC: 482D2AE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED56E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED56E0 size=12
    let mut pc: u32 = 0x82ED56E0;
    'dispatch: loop {
        match pc {
            0x82ED56E0 => {
    //   block [0x82ED56E0..0x82ED56EC)
	// 82ED56E0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED56E4: 350BFFFF  addic. r8, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED56E8: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED56EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED56EC size=100
    let mut pc: u32 = 0x82ED56EC;
    'dispatch: loop {
        match pc {
            0x82ED56EC => {
    //   block [0x82ED56EC..0x82ED5750)
	// 82ED56EC: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED56F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED56F4: 7D47582E  lwzx r10, r7, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED56F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED56FC: 409A0044  bne cr6, 0x82ed5740
	if !ctx.cr[6].eq {
	pc = 0x82ED5740; continue 'dispatch;
	}
	// 82ED5700: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5704: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82ED5708: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82ED570C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82ED5710: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED5714: 4098002C  bge cr6, 0x82ed5740
	if !ctx.cr[6].lt {
	pc = 0x82ED5740; continue 'dispatch;
	}
	// 82ED5718: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82ED571C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5720: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED5724: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82ED5728: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED572C: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5730: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82ED5734: 80A30004  lwz r5, 4(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5738: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82ED573C: 4198FFE0  blt cr6, 0x82ed571c
	if ctx.cr[6].lt {
	pc = 0x82ED571C; continue 'dispatch;
	}
	// 82ED5740: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED5744: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 82ED5748: 4080FFA8  bge 0x82ed56f0
	if !ctx.cr[0].lt {
	pc = 0x82ED56F0; continue 'dispatch;
	}
	// 82ED574C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5750 size=12
    let mut pc: u32 = 0x82ED5750;
    'dispatch: loop {
        match pc {
            0x82ED5750 => {
    //   block [0x82ED5750..0x82ED575C)
	// 82ED5750: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5754: 350BFFFF  addic. r8, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED5758: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED575C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED575C size=100
    let mut pc: u32 = 0x82ED575C;
    'dispatch: loop {
        match pc {
            0x82ED575C => {
    //   block [0x82ED575C..0x82ED57C0)
	// 82ED575C: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED5760: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5764: 7D47582E  lwzx r10, r7, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED5768: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED576C: 409A0044  bne cr6, 0x82ed57b0
	if !ctx.cr[6].eq {
	pc = 0x82ED57B0; continue 'dispatch;
	}
	// 82ED5770: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5774: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82ED5778: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82ED577C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82ED5780: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED5784: 4098002C  bge cr6, 0x82ed57b0
	if !ctx.cr[6].lt {
	pc = 0x82ED57B0; continue 'dispatch;
	}
	// 82ED5788: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82ED578C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5790: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED5794: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82ED5798: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED579C: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED57A0: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82ED57A4: 80A30004  lwz r5, 4(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED57A8: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82ED57AC: 4198FFE0  blt cr6, 0x82ed578c
	if ctx.cr[6].lt {
	pc = 0x82ED578C; continue 'dispatch;
	}
	// 82ED57B0: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED57B4: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 82ED57B8: 4080FFA8  bge 0x82ed5760
	if !ctx.cr[0].lt {
	pc = 0x82ED5760; continue 'dispatch;
	}
	// 82ED57BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED57C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED57C0 size=92
    let mut pc: u32 = 0x82ED57C0;
    'dispatch: loop {
        match pc {
            0x82ED57C0 => {
    //   block [0x82ED57C0..0x82ED581C)
	// 82ED57C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED57C4: 482D29A9  bl 0x831a816c
	ctx.lr = 0x82ED57C8;
	sub_831A8130(ctx, base);
	// 82ED57C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED57CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED57D0: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED57D4: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED57D8: 4180003C  blt 0x82ed5814
	if ctx.cr[0].lt {
	pc = 0x82ED5814; continue 'dispatch;
	}
	// 82ED57DC: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82ED57E0: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED57E4: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED57E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED57EC: 419A001C  beq cr6, 0x82ed5808
	if ctx.cr[6].eq {
	pc = 0x82ED5808; continue 'dispatch;
	}
	// 82ED57F0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82ED57F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED57F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED57FC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED5800: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED5804: 4E800421  bctrl
	ctx.lr = 0x82ED5808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5808: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED580C: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82ED5810: 4080FFD0  bge 0x82ed57e0
	if !ctx.cr[0].lt {
	pc = 0x82ED57E0; continue 'dispatch;
	}
	// 82ED5814: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED5818: 482D29A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5820 size=100
    let mut pc: u32 = 0x82ED5820;
    'dispatch: loop {
        match pc {
            0x82ED5820 => {
    //   block [0x82ED5820..0x82ED5884)
	// 82ED5820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5824: 482D2949  bl 0x831a816c
	ctx.lr = 0x82ED5828;
	sub_831A8130(ctx, base);
	// 82ED5828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED582C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED5830: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED5834: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED5838: 4180003C  blt 0x82ed5874
	if ctx.cr[0].lt {
	pc = 0x82ED5874; continue 'dispatch;
	}
	// 82ED583C: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82ED5840: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED5844: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED5848: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED584C: 419A001C  beq cr6, 0x82ed5868
	if ctx.cr[6].eq {
	pc = 0x82ED5868; continue 'dispatch;
	}
	// 82ED5850: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82ED5854: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED5858: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED585C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5860: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED5864: 4E800421  bctrl
	ctx.lr = 0x82ED5868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5868: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED586C: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82ED5870: 4080FFD0  bge 0x82ed5840
	if !ctx.cr[0].lt {
	pc = 0x82ED5840; continue 'dispatch;
	}
	// 82ED5874: 387E008C  addi r3, r30, 0x8c
	ctx.r[3].s64 = ctx.r[30].s64 + 140;
	// 82ED5878: 4BFFFE69  bl 0x82ed56e0
	ctx.lr = 0x82ED587C;
	sub_82ED56E0(ctx, base);
	// 82ED587C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED5880: 482D293C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5888 size=100
    let mut pc: u32 = 0x82ED5888;
    'dispatch: loop {
        match pc {
            0x82ED5888 => {
    //   block [0x82ED5888..0x82ED58EC)
	// 82ED5888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED588C: 482D28E1  bl 0x831a816c
	ctx.lr = 0x82ED5890;
	sub_831A8130(ctx, base);
	// 82ED5890: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5894: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED5898: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED589C: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED58A0: 4180003C  blt 0x82ed58dc
	if ctx.cr[0].lt {
	pc = 0x82ED58DC; continue 'dispatch;
	}
	// 82ED58A4: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82ED58A8: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED58AC: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED58B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED58B4: 419A001C  beq cr6, 0x82ed58d0
	if ctx.cr[6].eq {
	pc = 0x82ED58D0; continue 'dispatch;
	}
	// 82ED58B8: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82ED58BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED58C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED58C4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED58C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED58CC: 4E800421  bctrl
	ctx.lr = 0x82ED58D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED58D0: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED58D4: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82ED58D8: 4080FFD0  bge 0x82ed58a8
	if !ctx.cr[0].lt {
	pc = 0x82ED58A8; continue 'dispatch;
	}
	// 82ED58DC: 387E008C  addi r3, r30, 0x8c
	ctx.r[3].s64 = ctx.r[30].s64 + 140;
	// 82ED58E0: 4BFFFE01  bl 0x82ed56e0
	ctx.lr = 0x82ED58E4;
	sub_82ED56E0(ctx, base);
	// 82ED58E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED58E8: 482D28D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED58F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED58F0 size=100
    let mut pc: u32 = 0x82ED58F0;
    'dispatch: loop {
        match pc {
            0x82ED58F0 => {
    //   block [0x82ED58F0..0x82ED5954)
	// 82ED58F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED58F4: 482D2879  bl 0x831a816c
	ctx.lr = 0x82ED58F8;
	sub_831A8130(ctx, base);
	// 82ED58F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED58FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED5900: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED5904: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED5908: 4180003C  blt 0x82ed5944
	if ctx.cr[0].lt {
	pc = 0x82ED5944; continue 'dispatch;
	}
	// 82ED590C: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82ED5910: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED5914: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED5918: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED591C: 419A001C  beq cr6, 0x82ed5938
	if ctx.cr[6].eq {
	pc = 0x82ED5938; continue 'dispatch;
	}
	// 82ED5920: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82ED5924: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED5928: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED592C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED5930: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED5934: 4E800421  bctrl
	ctx.lr = 0x82ED5938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5938: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ED593C: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82ED5940: 4080FFD0  bge 0x82ed5910
	if !ctx.cr[0].lt {
	pc = 0x82ED5910; continue 'dispatch;
	}
	// 82ED5944: 387E008C  addi r3, r30, 0x8c
	ctx.r[3].s64 = ctx.r[30].s64 + 140;
	// 82ED5948: 4BFFFD99  bl 0x82ed56e0
	ctx.lr = 0x82ED594C;
	sub_82ED56E0(ctx, base);
	// 82ED594C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED5950: 482D286C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5958 size=76
    let mut pc: u32 = 0x82ED5958;
    'dispatch: loop {
        match pc {
            0x82ED5958 => {
    //   block [0x82ED5958..0x82ED59A4)
	// 82ED5958: 81230090  lwz r9, 0x90(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED595C: 3903008C  addi r8, r3, 0x8c
	ctx.r[8].s64 = ctx.r[3].s64 + 140;
	// 82ED5960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED5964: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED5968: 40990024  ble cr6, 0x82ed598c
	if !ctx.cr[6].gt {
	pc = 0x82ED598C; continue 'dispatch;
	}
	// 82ED596C: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5970: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5974: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ED5978: 419A0018  beq cr6, 0x82ed5990
	if ctx.cr[6].eq {
	pc = 0x82ED5990; continue 'dispatch;
	}
	// 82ED597C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED5980: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED5984: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED5988: 4198FFE8  blt cr6, 0x82ed5970
	if ctx.cr[6].lt {
	pc = 0x82ED5970; continue 'dispatch;
	}
	// 82ED598C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED5990: 81480004  lwz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5994: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82ED5998: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82ED599C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED59A0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED59A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED59A4 size=44
    let mut pc: u32 = 0x82ED59A4;
    'dispatch: loop {
        match pc {
            0x82ED59A4 => {
    //   block [0x82ED59A4..0x82ED59D0)
	// 82ED59A4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED59A8: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED59AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED59B0: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82ED59B4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED59B8: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED59BC: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED59C0: 80C80004  lwz r6, 4(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED59C4: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82ED59C8: 4198FFE0  blt cr6, 0x82ed59a8
	if ctx.cr[6].lt {
	pc = 0x82ED59A8; continue 'dispatch;
	}
	// 82ED59CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED59D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED59D0 size=76
    let mut pc: u32 = 0x82ED59D0;
    'dispatch: loop {
        match pc {
            0x82ED59D0 => {
    //   block [0x82ED59D0..0x82ED5A1C)
	// 82ED59D0: 81230084  lwz r9, 0x84(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED59D4: 39030080  addi r8, r3, 0x80
	ctx.r[8].s64 = ctx.r[3].s64 + 128;
	// 82ED59D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED59DC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED59E0: 40990024  ble cr6, 0x82ed5a04
	if !ctx.cr[6].gt {
	pc = 0x82ED5A04; continue 'dispatch;
	}
	// 82ED59E4: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED59E8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED59EC: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ED59F0: 419A0018  beq cr6, 0x82ed5a08
	if ctx.cr[6].eq {
	pc = 0x82ED5A08; continue 'dispatch;
	}
	// 82ED59F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED59F8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED59FC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED5A00: 4198FFE8  blt cr6, 0x82ed59e8
	if ctx.cr[6].lt {
	pc = 0x82ED59E8; continue 'dispatch;
	}
	// 82ED5A04: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED5A08: 81480004  lwz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5A0C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82ED5A10: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82ED5A14: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ED5A18: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5A1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED5A1C size=44
    let mut pc: u32 = 0x82ED5A1C;
    'dispatch: loop {
        match pc {
            0x82ED5A1C => {
    //   block [0x82ED5A1C..0x82ED5A48)
	// 82ED5A1C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED5A20: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5A24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED5A28: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82ED5A2C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED5A30: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5A34: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ED5A38: 80C80004  lwz r6, 4(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5A3C: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82ED5A40: 4198FFE0  blt cr6, 0x82ed5a20
	if ctx.cr[6].lt {
	pc = 0x82ED5A20; continue 'dispatch;
	}
	// 82ED5A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5A48 size=112
    let mut pc: u32 = 0x82ED5A48;
    'dispatch: loop {
        match pc {
            0x82ED5A48 => {
    //   block [0x82ED5A48..0x82ED5AB8)
	// 82ED5A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED5A50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED5A54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED5A58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5A5C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82ED5A60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED5A64: 81430090  lwz r10, 0x90(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED5A68: 3BE3008C  addi r31, r3, 0x8c
	ctx.r[31].s64 = ctx.r[3].s64 + 140;
	// 82ED5A6C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED5A70: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED5A74: 409A0010  bne cr6, 0x82ed5a84
	if !ctx.cr[6].eq {
	pc = 0x82ED5A84; continue 'dispatch;
	}
	// 82ED5A78: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED5A7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED5A80: 4BFD0E01  bl 0x82ea6880
	ctx.lr = 0x82ED5A84;
	sub_82EA6880(ctx, base);
	// 82ED5A84: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5A88: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5A8C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED5A90: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED5A94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5A98: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED5A9C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED5AA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED5AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED5AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5AAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED5AB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED5AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5AB8 size=112
    let mut pc: u32 = 0x82ED5AB8;
    'dispatch: loop {
        match pc {
            0x82ED5AB8 => {
    //   block [0x82ED5AB8..0x82ED5B28)
	// 82ED5AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED5AC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED5AC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED5AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5ACC: 81630088  lwz r11, 0x88(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED5AD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED5AD4: 81430084  lwz r10, 0x84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5AD8: 3BE30080  addi r31, r3, 0x80
	ctx.r[31].s64 = ctx.r[3].s64 + 128;
	// 82ED5ADC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED5AE0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED5AE4: 409A0010  bne cr6, 0x82ed5af4
	if !ctx.cr[6].eq {
	pc = 0x82ED5AF4; continue 'dispatch;
	}
	// 82ED5AE8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED5AEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED5AF0: 4BFD0D91  bl 0x82ea6880
	ctx.lr = 0x82ED5AF4;
	sub_82EA6880(ctx, base);
	// 82ED5AF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5AF8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5AFC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED5B00: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED5B04: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED5B08: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED5B0C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED5B10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED5B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED5B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5B1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED5B20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED5B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5B28 size=184
    let mut pc: u32 = 0x82ED5B28;
    'dispatch: loop {
        match pc {
            0x82ED5B28 => {
    //   block [0x82ED5B28..0x82ED5BE0)
	// 82ED5B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED5B30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED5B34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED5B38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5B3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED5B40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED5B44: 4BFFF8DD  bl 0x82ed5420
	ctx.lr = 0x82ED5B48;
	sub_82ED5420(ctx, base);
	// 82ED5B48: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED5B4C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5B50: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED5B54: 409A0034  bne cr6, 0x82ed5b88
	if !ctx.cr[6].eq {
	pc = 0x82ED5B88; continue 'dispatch;
	}
	// 82ED5B58: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5B5C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED5B60: 80FF0084  lwz r7, 0x84(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5B64: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED5B68: 80DF0080  lwz r6, 0x80(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED5B6C: 38894804  addi r4, r9, 0x4804
	ctx.r[4].s64 = ctx.r[9].s64 + 18436;
	// 82ED5B70: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED5B74: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED5B78: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5B7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED5B80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED5B84: 4E800421  bctrl
	ctx.lr = 0x82ED5B88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5B88: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82ED5B8C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5B90: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED5B94: 409A0034  bne cr6, 0x82ed5bc8
	if !ctx.cr[6].eq {
	pc = 0x82ED5BC8; continue 'dispatch;
	}
	// 82ED5B98: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5B9C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED5BA0: 80FF0090  lwz r7, 0x90(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED5BA4: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED5BA8: 80DF008C  lwz r6, 0x8c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED5BAC: 388947F4  addi r4, r9, 0x47f4
	ctx.r[4].s64 = ctx.r[9].s64 + 18420;
	// 82ED5BB0: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED5BB4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED5BB8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5BBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED5BC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED5BC4: 4E800421  bctrl
	ctx.lr = 0x82ED5BC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5BC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED5BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED5BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5BD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED5BD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED5BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5BE0 size=192
    let mut pc: u32 = 0x82ED5BE0;
    'dispatch: loop {
        match pc {
            0x82ED5BE0 => {
    //   block [0x82ED5BE0..0x82ED5CA0)
	// 82ED5BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5BE4: 482D2585  bl 0x831a8168
	ctx.lr = 0x82ED5BE8;
	sub_831A8130(ctx, base);
	// 82ED5BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5BEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED5BF0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82ED5BF4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED5BF8: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5BFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED5C00: 409A0048  bne cr6, 0x82ed5c48
	if !ctx.cr[6].eq {
	pc = 0x82ED5C48; continue 'dispatch;
	}
	// 82ED5C04: 817E0088  lwz r11, 0x88(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED5C08: 3BFE0080  addi r31, r30, 0x80
	ctx.r[31].s64 = ctx.r[30].s64 + 128;
	// 82ED5C0C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5C10: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED5C14: 409A0020  bne cr6, 0x82ed5c34
	if !ctx.cr[6].eq {
	pc = 0x82ED5C34; continue 'dispatch;
	}
	// 82ED5C18: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5C1C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED5C20: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED5C24: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5C28: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED5C2C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED5C30: 4BFCAB81  bl 0x82ea07b0
	ctx.lr = 0x82ED5C34;
	sub_82EA07B0(ctx, base);
	// 82ED5C34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5C38: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82ED5C3C: 538BF880  rlwimi r11, r28, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[28].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 82ED5C40: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82ED5C44: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82ED5C48: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED5C4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED5C50: 409A0048  bne cr6, 0x82ed5c98
	if !ctx.cr[6].eq {
	pc = 0x82ED5C98; continue 'dispatch;
	}
	// 82ED5C54: 817E0094  lwz r11, 0x94(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 82ED5C58: 3BFE008C  addi r31, r30, 0x8c
	ctx.r[31].s64 = ctx.r[30].s64 + 140;
	// 82ED5C5C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5C60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED5C64: 409A0020  bne cr6, 0x82ed5c84
	if !ctx.cr[6].eq {
	pc = 0x82ED5C84; continue 'dispatch;
	}
	// 82ED5C68: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5C6C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED5C70: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED5C74: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5C78: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED5C7C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED5C80: 4BFCAB31  bl 0x82ea07b0
	ctx.lr = 0x82ED5C84;
	sub_82EA07B0(ctx, base);
	// 82ED5C84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5C88: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82ED5C8C: 538BF880  rlwimi r11, r28, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[28].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 82ED5C90: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82ED5C94: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82ED5C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED5C9C: 482D251C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5CA0 size=152
    let mut pc: u32 = 0x82ED5CA0;
    'dispatch: loop {
        match pc {
            0x82ED5CA0 => {
    //   block [0x82ED5CA0..0x82ED5D38)
	// 82ED5CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED5CA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED5CAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5CB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED5CB4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED5CB8: 394B474C  addi r10, r11, 0x474c
	ctx.r[10].s64 = ctx.r[11].s64 + 18252;
	// 82ED5CBC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED5CC0: 4BFFFB01  bl 0x82ed57c0
	ctx.lr = 0x82ED5CC4;
	sub_82ED57C0(ctx, base);
	// 82ED5CC4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82ED5CC8: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5CCC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED5CD0: 409A0020  bne cr6, 0x82ed5cf0
	if !ctx.cr[6].eq {
	pc = 0x82ED5CF0; continue 'dispatch;
	}
	// 82ED5CD4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5CD8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED5CDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED5CE0: 809F008C  lwz r4, 0x8c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED5CE4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED5CE8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED5CEC: 4BFCAAC5  bl 0x82ea07b0
	ctx.lr = 0x82ED5CF0;
	sub_82EA07B0(ctx, base);
	// 82ED5CF0: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED5CF4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5CF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED5CFC: 409A0020  bne cr6, 0x82ed5d1c
	if !ctx.cr[6].eq {
	pc = 0x82ED5D1C; continue 'dispatch;
	}
	// 82ED5D00: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5D04: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED5D08: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED5D0C: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED5D10: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED5D14: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED5D18: 4BFCAA99  bl 0x82ea07b0
	ctx.lr = 0x82ED5D1C;
	sub_82EA07B0(ctx, base);
	// 82ED5D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED5D20: 4BFFF1E9  bl 0x82ed4f08
	ctx.lr = 0x82ED5D24;
	sub_82ED4F08(ctx, base);
	// 82ED5D24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED5D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED5D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED5D30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED5D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED5D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED5D38 size=720
    let mut pc: u32 = 0x82ED5D38;
    'dispatch: loop {
        match pc {
            0x82ED5D38 => {
    //   block [0x82ED5D38..0x82ED6008)
	// 82ED5D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED5D3C: 482D2421  bl 0x831a815c
	ctx.lr = 0x82ED5D40;
	sub_831A8130(ctx, base);
	// 82ED5D40: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED5D44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED5D48: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82ED5D4C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5D50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED5D54: 419A02AC  beq cr6, 0x82ed6000
	if ctx.cr[6].eq {
	pc = 0x82ED6000; continue 'dispatch;
	}
	// 82ED5D58: 81430088  lwz r10, 0x88(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED5D5C: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5D60: 7D6A5A15  add. r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED5D64: 41820024  beq 0x82ed5d88
	if ctx.cr[0].eq {
	pc = 0x82ED5D88; continue 'dispatch;
	}
	// 82ED5D68: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 82ED5D6C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82ED5D70: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82ED5D74: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82ED5D78: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82ED5D7C: 4BFF7285  bl 0x82ecd000
	ctx.lr = 0x82ED5D80;
	sub_82ECD000(ctx, base);
	// 82ED5D80: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82ED5D84: 482D2428  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82ED5D88: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5D8C: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 82ED5D90: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5D94: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82ED5D98: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 82ED5D9C: 816A0084  lwz r11, 0x84(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5DA0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82ED5DA4: 912A0084  stw r9, 0x84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82ED5DA8: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5DAC: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED5DB0: 83C802FC  lwz r30, 0x2fc(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(764 as u32) ) } as u64;
	// 82ED5DB4: 38FE0002  addi r7, r30, 2
	ctx.r[7].s64 = ctx.r[30].s64 + 2;
	// 82ED5DB8: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 82ED5DBC: 54E41836  rlwinm r4, r7, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[7].u32 as u64 & 0x1FFFFFFFu64;
	// 82ED5DC0: 93810084  stw r28, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 82ED5DC4: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82ED5DC8: 80C3002C  lwz r6, 0x2c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED5DCC: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED5DD0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ED5DD4: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82ED5DD8: 41990010  bgt cr6, 0x82ed5de8
	if ctx.cr[6].gt {
	pc = 0x82ED5DE8; continue 'dispatch;
	}
	// 82ED5DDC: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82ED5DE0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED5DE4: 48000014  b 0x82ed5df8
	pc = 0x82ED5DF8; continue 'dispatch;
	// 82ED5DE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5DEC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED5DF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED5DF4: 4E800421  bctrl
	ctx.lr = 0x82ED5DF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5DF8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5DFC: 7FC9EB78  or r9, r30, r29
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[29].u64;
	// 82ED5E00: 7D7BD02E  lwzx r11, r27, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED5E04: 91210088  stw r9, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[9].u32 ) };
	// 82ED5E08: 90610080  stw r3, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[3].u32 ) };
	// 82ED5E0C: 9061008C  stw r3, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[3].u32 ) };
	// 82ED5E10: 83CA02FC  lwz r30, 0x2fc(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(764 as u32) ) } as u64;
	// 82ED5E14: 391E0002  addi r8, r30, 2
	ctx.r[8].s64 = ctx.r[30].s64 + 2;
	// 82ED5E18: 93810070  stw r28, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[28].u32 ) };
	// 82ED5E1C: 55041836  rlwinm r4, r8, 3, 0, 0x1b
	ctx.r[4].u64 = ctx.r[8].u32 as u64 & 0x1FFFFFFFu64;
	// 82ED5E20: 93810074  stw r28, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[28].u32 ) };
	// 82ED5E24: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82ED5E28: 80EB002C  lwz r7, 0x2c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED5E2C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED5E30: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82ED5E34: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82ED5E38: 4199000C  bgt cr6, 0x82ed5e44
	if ctx.cr[6].gt {
	pc = 0x82ED5E44; continue 'dispatch;
	}
	// 82ED5E3C: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82ED5E40: 4800001C  b 0x82ed5e5c
	pc = 0x82ED5E5C; continue 'dispatch;
	// 82ED5E44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5E48: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED5E4C: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED5E50: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED5E54: 4E800421  bctrl
	ctx.lr = 0x82ED5E58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5E58: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82ED5E5C: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82ED5E60: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82ED5E64: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82ED5E68: 7FCAEB78  or r10, r30, r29
	ctx.r[10].u64 = ctx.r[30].u64 | ctx.r[29].u64;
	// 82ED5E6C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82ED5E70: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82ED5E74: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5E78: 80690054  lwz r3, 0x54(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED5E7C: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED5E80: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED5E84: 419A0008  beq cr6, 0x82ed5e8c
	if ctx.cr[6].eq {
	pc = 0x82ED5E8C; continue 'dispatch;
	}
	// 82ED5E88: 48099A09  bl 0x82f6f890
	ctx.lr = 0x82ED5E8C;
	sub_82F6F890(ctx, base);
	// 82ED5E8C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5E90: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82ED5E94: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82ED5E98: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82ED5E9C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82ED5EA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED5EA4: 806B0054  lwz r3, 0x54(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED5EA8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5EAC: 812A0028  lwz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED5EB0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED5EB4: 4E800421  bctrl
	ctx.lr = 0x82ED5EB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5EB8: 81010084  lwz r8, 0x84(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5EBC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED5EC0: 409A0010  bne cr6, 0x82ed5ed0
	if !ctx.cr[6].eq {
	pc = 0x82ED5ED0; continue 'dispatch;
	}
	// 82ED5EC4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED5EC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED5ECC: 419A0054  beq cr6, 0x82ed5f20
	if ctx.cr[6].eq {
	pc = 0x82ED5F20; continue 'dispatch;
	}
	// 82ED5ED0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82ED5ED4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82ED5ED8: 4804DBB1  bl 0x82f23a88
	ctx.lr = 0x82ED5EDC;
	sub_82F23A88(ctx, base);
	// 82ED5EDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5EE0: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED5EE4: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED5EE8: 806B0058  lwz r3, 0x58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED5EEC: 4804DB2D  bl 0x82f23a18
	ctx.lr = 0x82ED5EF0;
	sub_82F23A18(ctx, base);
	// 82ED5EF0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5EF4: 816A0070  lwz r11, 0x70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED5EF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED5EFC: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82ED5F00: 409A0008  bne cr6, 0x82ed5f08
	if !ctx.cr[6].eq {
	pc = 0x82ED5F08; continue 'dispatch;
	}
	// 82ED5F04: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82ED5F08: 806A0058  lwz r3, 0x58(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED5F0C: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5F10: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED5F14: 4804DA4D  bl 0x82f23960
	ctx.lr = 0x82ED5F18;
	sub_82F23960(ctx, base);
	// 82ED5F18: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82ED5F1C: 4BFFF835  bl 0x82ed5750
	ctx.lr = 0x82ED5F20;
	sub_82ED5750(ctx, base);
	// 82ED5F20: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5F24: 806B0054  lwz r3, 0x54(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED5F28: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED5F2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED5F30: 419A0008  beq cr6, 0x82ed5f38
	if ctx.cr[6].eq {
	pc = 0x82ED5F38; continue 'dispatch;
	}
	// 82ED5F34: 4809994D  bl 0x82f6f880
	ctx.lr = 0x82ED5F38;
	sub_82F6F880(ctx, base);
	// 82ED5F38: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED5F3C: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED5F40: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED5F44: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED5F48: 40820020  bne 0x82ed5f68
	if !ctx.cr[0].eq {
	pc = 0x82ED5F68; continue 'dispatch;
	}
	// 82ED5F4C: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED5F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED5F54: 409A0014  bne cr6, 0x82ed5f68
	if !ctx.cr[6].eq {
	pc = 0x82ED5F68; continue 'dispatch;
	}
	// 82ED5F58: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED5F5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED5F60: 419A0008  beq cr6, 0x82ed5f68
	if ctx.cr[6].eq {
	pc = 0x82ED5F68; continue 'dispatch;
	}
	// 82ED5F64: 4BFF7085  bl 0x82eccfe8
	ctx.lr = 0x82ED5F68;
	sub_82ECCFE8(ctx, base);
	// 82ED5F68: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED5F6C: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED5F70: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED5F74: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED5F78: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED5F7C: 409A0014  bne cr6, 0x82ed5f90
	if !ctx.cr[6].eq {
	pc = 0x82ED5F90; continue 'dispatch;
	}
	// 82ED5F80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5F84: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED5F88: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED5F8C: 4E800421  bctrl
	ctx.lr = 0x82ED5F90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5F90: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED5F94: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5F98: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED5F9C: 409A0018  bne cr6, 0x82ed5fb4
	if !ctx.cr[6].eq {
	pc = 0x82ED5FB4; continue 'dispatch;
	}
	// 82ED5FA0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED5FA4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED5FA8: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED5FAC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED5FB0: 4BFCA801  bl 0x82ea07b0
	ctx.lr = 0x82ED5FB4;
	sub_82EA07B0(ctx, base);
	// 82ED5FB4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED5FB8: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ED5FBC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED5FC0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82ED5FC4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ED5FC8: 409A0014  bne cr6, 0x82ed5fdc
	if !ctx.cr[6].eq {
	pc = 0x82ED5FDC; continue 'dispatch;
	}
	// 82ED5FCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED5FD0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED5FD4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED5FD8: 4E800421  bctrl
	ctx.lr = 0x82ED5FDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED5FDC: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED5FE0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED5FE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED5FE8: 409A0018  bne cr6, 0x82ed6000
	if !ctx.cr[6].eq {
	pc = 0x82ED6000; continue 'dispatch;
	}
	// 82ED5FEC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED5FF0: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82ED5FF4: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED5FF8: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED5FFC: 4BFCA7B5  bl 0x82ea07b0
	ctx.lr = 0x82ED6000;
	sub_82EA07B0(ctx, base);
	// 82ED6000: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82ED6004: 482D21A8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6008 size=8
    let mut pc: u32 = 0x82ED6008;
    'dispatch: loop {
        match pc {
            0x82ED6008 => {
    //   block [0x82ED6008..0x82ED6010)
	// 82ED6008: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82ED600C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6010 size=124
    let mut pc: u32 = 0x82ED6010;
    'dispatch: loop {
        match pc {
            0x82ED6010 => {
    //   block [0x82ED6010..0x82ED608C)
	// 82ED6010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6014: 482D214D  bl 0x831a8160
	ctx.lr = 0x82ED6018;
	sub_831A8130(ctx, base);
	// 82ED6018: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED601C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82ED6020: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED6024: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82ED6028: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82ED602C: 409A000C  bne cr6, 0x82ed6038
	if !ctx.cr[6].eq {
	pc = 0x82ED6038; continue 'dispatch;
	}
	// 82ED6030: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED6034: 836B006C  lwz r27, 0x6c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED6038: 817D0154  lwz r11, 0x154(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED603C: 37EBFFFF  addic. r31, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82ED6040: 41800044  blt 0x82ed6084
	if ctx.cr[0].lt {
	pc = 0x82ED6084; continue 'dispatch;
	}
	// 82ED6044: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 82ED6048: 57FE1838  slwi r30, r31, 3
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82ED604C: 817D0150  lwz r11, 0x150(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED6050: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82ED6054: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82ED6058: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82ED605C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ED6060: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6064: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED6068: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED606C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED6070: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED6074: 4E800421  bctrl
	ctx.lr = 0x82ED6078;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED6078: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82ED607C: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 82ED6080: 4080FFCC  bge 0x82ed604c
	if !ctx.cr[0].lt {
	pc = 0x82ED604C; continue 'dispatch;
	}
	// 82ED6084: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED6088: 482D2128  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6090 size=136
    let mut pc: u32 = 0x82ED6090;
    'dispatch: loop {
        match pc {
            0x82ED6090 => {
    //   block [0x82ED6090..0x82ED6118)
	// 82ED6090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6094: 482D20CD  bl 0x831a8160
	ctx.lr = 0x82ED6098;
	sub_831A8130(ctx, base);
	// 82ED6098: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED609C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82ED60A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED60A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED60A8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82ED60AC: 409A000C  bne cr6, 0x82ed60b8
	if !ctx.cr[6].eq {
	pc = 0x82ED60B8; continue 'dispatch;
	}
	// 82ED60B0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED60B4: 836B006C  lwz r27, 0x6c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED60B8: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED60BC: 374BFFFF  addic. r26, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82ED60C0: 41800050  blt 0x82ed6110
	if ctx.cr[0].lt {
	pc = 0x82ED6110; continue 'dispatch;
	}
	// 82ED60C4: 3B9E0010  addi r28, r30, 0x10
	ctx.r[28].s64 = ctx.r[30].s64 + 16;
	// 82ED60C8: 575F1838  slwi r31, r26, 3
	ctx.r[31].u32 = ctx.r[26].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82ED60CC: 817E0150  lwz r11, 0x150(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED60D0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82ED60D4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82ED60D8: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82ED60DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ED60E0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED60E4: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED60E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED60EC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED60F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED60F4: 4E800421  bctrl
	ctx.lr = 0x82ED60F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED60F8: 893D0004  lbz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED60FC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED6100: 409A0010  bne cr6, 0x82ed6110
	if !ctx.cr[6].eq {
	pc = 0x82ED6110; continue 'dispatch;
	}
	// 82ED6104: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82ED6108: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82ED610C: 4080FFC0  bge 0x82ed60cc
	if !ctx.cr[0].lt {
	pc = 0x82ED60CC; continue 'dispatch;
	}
	// 82ED6110: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED6114: 482D209C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6118 size=64
    let mut pc: u32 = 0x82ED6118;
    'dispatch: loop {
        match pc {
            0x82ED6118 => {
    //   block [0x82ED6118..0x82ED6158)
	// 82ED6118: 81240154  lwz r9, 0x154(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED611C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED6120: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED6124: 40990028  ble cr6, 0x82ed614c
	if !ctx.cr[6].gt {
	pc = 0x82ED614C; continue 'dispatch;
	}
	// 82ED6128: 81640150  lwz r11, 0x150(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED612C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82ED6130: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6134: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82ED6138: 419A0020  beq cr6, 0x82ed6158
	if ctx.cr[6].eq {
		sub_82ED6158(ctx, base);
		return;
	}
	// 82ED613C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED6140: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED6144: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED6148: 4198FFE8  blt cr6, 0x82ed6130
	if ctx.cr[6].lt {
	pc = 0x82ED6130; continue 'dispatch;
	}
	// 82ED614C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED6150: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ED6154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6158 size=12
    let mut pc: u32 = 0x82ED6158;
    'dispatch: loop {
        match pc {
            0x82ED6158 => {
    //   block [0x82ED6158..0x82ED6164)
	// 82ED6158: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ED615C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ED6160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6168 size=264
    let mut pc: u32 = 0x82ED6168;
    'dispatch: loop {
        match pc {
            0x82ED6168 => {
    //   block [0x82ED6168..0x82ED6270)
	// 82ED6168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED616C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED6170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED6174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED6178: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED617C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED6180: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6184: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ED6188: 390B4818  addi r8, r11, 0x4818
	ctx.r[8].s64 = ctx.r[11].s64 + 18456;
	// 82ED618C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED6190: B1210066  sth r9, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[9].u16 ) };
	// 82ED6194: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED6198: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 82ED619C: 419A0050  beq cr6, 0x82ed61ec
	if ctx.cr[6].eq {
	pc = 0x82ED61EC; continue 'dispatch;
	}
	// 82ED61A0: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED61A4: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED61A8: 4180002C  blt 0x82ed61d4
	if ctx.cr[0].lt {
	pc = 0x82ED61D4; continue 'dispatch;
	}
	// 82ED61AC: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED61B0: 57CA1838  slwi r10, r30, 3
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED61B4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED61B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82ED61BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED61C0: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ED61C4: 419A0040  beq cr6, 0x82ed6204
	if ctx.cr[6].eq {
	pc = 0x82ED6204; continue 'dispatch;
	}
	// 82ED61C8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED61CC: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82ED61D0: 4080FFEC  bge 0x82ed61bc
	if !ctx.cr[0].lt {
	pc = 0x82ED61BC; continue 'dispatch;
	}
	// 82ED61D4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82ED61D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED61DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED61E0: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82ED61E4: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED61E8: 480008E1  bl 0x82ed6ac8
	ctx.lr = 0x82ED61EC;
	sub_82ED6AC8(ctx, base);
	// 82ED61EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED61F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED61F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED61F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED61FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED6200: 4E800020  blr
	return;
	// 82ED6204: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82ED6208: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82ED620C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED6210: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6214: 480008B5  bl 0x82ed6ac8
	ctx.lr = 0x82ED6218;
	sub_82ED6AC8(ctx, base);
	// 82ED6218: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED621C: 57DE1838  slwi r30, r30, 3
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82ED6220: 7C7E502E  lwzx r3, r30, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED6224: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6228: 419A0018  beq cr6, 0x82ed6240
	if ctx.cr[6].eq {
	pc = 0x82ED6240; continue 'dispatch;
	}
	// 82ED622C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6230: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82ED6234: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ED6238: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED623C: 4E800421  bctrl
	ctx.lr = 0x82ED6240;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED6240: 815F0154  lwz r10, 0x154(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED6244: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED6248: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82ED624C: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82ED6250: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED6254: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 82ED6258: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82ED625C: 7CC9582E  lwzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED6260: 7CCBF12E  stwx r6, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[6].u32) };
	// 82ED6264: 80A70004  lwz r5, 4(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED6268: 90A80004  stw r5, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ED626C: 4BFFFF80  b 0x82ed61ec
	pc = 0x82ED61EC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6270 size=172
    let mut pc: u32 = 0x82ED6270;
    'dispatch: loop {
        match pc {
            0x82ED6270 => {
    //   block [0x82ED6270..0x82ED631C)
	// 82ED6270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6274: 482D1EF5  bl 0x831a8168
	ctx.lr = 0x82ED6278;
	sub_831A8130(ctx, base);
	// 82ED6278: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED627C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED6280: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82ED6284: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82ED6288: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED628C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED6290: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED6294: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82ED6298: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED629C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82ED62A0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82ED62A4: 4200FFF0  bdnz 0x82ed6294
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82ED6294; continue 'dispatch;
	}
	// 82ED62A8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED62AC: 815D0154  lwz r10, 0x154(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED62B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ED62B4: 390B4818  addi r8, r11, 0x4818
	ctx.r[8].s64 = ctx.r[11].s64 + 18456;
	// 82ED62B8: B1210056  sth r9, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[9].u16 ) };
	// 82ED62BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED62C0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82ED62C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED62C8: 4099004C  ble cr6, 0x82ed6314
	if !ctx.cr[6].gt {
	pc = 0x82ED6314; continue 'dispatch;
	}
	// 82ED62CC: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 82ED62D0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ED62D4: 817D0150  lwz r11, 0x150(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED62D8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82ED62DC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82ED62E0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82ED62E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ED62E8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED62EC: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED62F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED62F4: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED62F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED62FC: 4E800421  bctrl
	ctx.lr = 0x82ED6300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED6300: 813D0154  lwz r9, 0x154(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED6304: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82ED6308: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82ED630C: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED6310: 4198FFC4  blt cr6, 0x82ed62d4
	if ctx.cr[6].lt {
	pc = 0x82ED62D4; continue 'dispatch;
	}
	// 82ED6314: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82ED6318: 482D1EA0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6320 size=96
    let mut pc: u32 = 0x82ED6320;
    'dispatch: loop {
        match pc {
            0x82ED6320 => {
    //   block [0x82ED6320..0x82ED6380)
	// 82ED6320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED6328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED632C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED6330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED6334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED6338: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82ED633C: 4BFFEB45  bl 0x82ed4e80
	ctx.lr = 0x82ED6340;
	sub_82ED4E80(ctx, base);
	// 82ED6340: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED6344: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED6348: 392A483C  addi r9, r10, 0x483c
	ctx.r[9].s64 = ctx.r[10].s64 + 18492;
	// 82ED634C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82ED6350: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED6354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED6358: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82ED635C: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82ED6360: 911F0158  stw r8, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[8].u32 ) };
	// 82ED6364: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82ED6368: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED636C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED6370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED6374: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED6378: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED637C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6380 size=184
    let mut pc: u32 = 0x82ED6380;
    'dispatch: loop {
        match pc {
            0x82ED6380 => {
    //   block [0x82ED6380..0x82ED6438)
	// 82ED6380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6384: 482D1DE9  bl 0x831a816c
	ctx.lr = 0x82ED6388;
	sub_831A8130(ctx, base);
	// 82ED6388: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED638C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED6390: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED6394: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED6398: 390A483C  addi r8, r10, 0x483c
	ctx.r[8].s64 = ctx.r[10].s64 + 18492;
	// 82ED639C: 38E94818  addi r7, r9, 0x4818
	ctx.r[7].s64 = ctx.r[9].s64 + 18456;
	// 82ED63A0: 817D0154  lwz r11, 0x154(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED63A4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82ED63A8: 911D0000  stw r8, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED63AC: 37EBFFFF  addic. r31, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82ED63B0: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82ED63B4: B0C10056  sth r6, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[6].u16 ) };
	// 82ED63B8: 41800030  blt 0x82ed63e8
	if ctx.cr[0].lt {
	pc = 0x82ED63E8; continue 'dispatch;
	}
	// 82ED63BC: 57FE1838  slwi r30, r31, 3
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82ED63C0: 817D0150  lwz r11, 0x150(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED63C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED63C8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ED63CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED63D0: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ED63D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED63D8: 4E800421  bctrl
	ctx.lr = 0x82ED63DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED63DC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82ED63E0: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 82ED63E4: 4080FFDC  bge 0x82ed63c0
	if !ctx.cr[0].lt {
	pc = 0x82ED63C0; continue 'dispatch;
	}
	// 82ED63E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED63EC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82ED63F0: 917D0154  stw r11, 0x154(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82ED63F4: 817D0158  lwz r11, 0x158(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(344 as u32) ) } as u64;
	// 82ED63F8: 392A9EAC  addi r9, r10, -0x6154
	ctx.r[9].s64 = ctx.r[10].s64 + -24916;
	// 82ED63FC: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED6400: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82ED6404: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED6408: 409A0020  bne cr6, 0x82ed6428
	if !ctx.cr[6].eq {
	pc = 0x82ED6428; continue 'dispatch;
	}
	// 82ED640C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6410: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED6414: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED6418: 809D0150  lwz r4, 0x150(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED641C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED6420: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED6424: 4BFCA38D  bl 0x82ea07b0
	ctx.lr = 0x82ED6428;
	sub_82EA07B0(ctx, base);
	// 82ED6428: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED642C: 4BFFF875  bl 0x82ed5ca0
	ctx.lr = 0x82ED6430;
	sub_82ED5CA0(ctx, base);
	// 82ED6430: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED6434: 482D1D88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6438 size=432
    let mut pc: u32 = 0x82ED6438;
    'dispatch: loop {
        match pc {
            0x82ED6438 => {
    //   block [0x82ED6438..0x82ED65E8)
	// 82ED6438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED643C: 482D1D21  bl 0x831a815c
	ctx.lr = 0x82ED6440;
	sub_831A8130(ctx, base);
	// 82ED6440: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED6444: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6448: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82ED644C: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 82ED6450: 38800160  li r4, 0x160
	ctx.r[4].s64 = 352;
	// 82ED6454: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ED6458: 7C7AD82E  lwzx r3, r26, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82ED645C: 4BFCA2D5  bl 0x82ea0730
	ctx.lr = 0x82ED6460;
	sub_82EA0730(ctx, base);
	// 82ED6460: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED6464: 39600160  li r11, 0x160
	ctx.r[11].s64 = 352;
	// 82ED6468: 38BC00A0  addi r5, r28, 0xa0
	ctx.r[5].s64 = ctx.r[28].s64 + 160;
	// 82ED646C: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82ED6470: 833C002C  lwz r25, 0x2c(r28)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED6474: 809C0010  lwz r4, 0x10(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED6478: 4BFFEA09  bl 0x82ed4e80
	ctx.lr = 0x82ED647C;
	sub_82ED4E80(ctx, base);
	// 82ED647C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED6480: 392A483C  addi r9, r10, 0x483c
	ctx.r[9].s64 = ctx.r[10].s64 + 18492;
	// 82ED6484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED6488: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82ED648C: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED6490: 917E0150  stw r11, 0x150(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82ED6494: 3BFE0080  addi r31, r30, 0x80
	ctx.r[31].s64 = ctx.r[30].s64 + 128;
	// 82ED6498: 917E0154  stw r11, 0x154(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82ED649C: 3BBC0080  addi r29, r28, 0x80
	ctx.r[29].s64 = ctx.r[28].s64 + 128;
	// 82ED64A0: 911E0158  stw r8, 0x158(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(344 as u32), ctx.r[8].u32 ) };
	// 82ED64A4: 933E002C  stw r25, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[25].u32 ) };
	// 82ED64A8: 80FC0084  lwz r7, 0x84(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED64AC: 815E0088  lwz r10, 0x88(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ED64B0: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED64B4: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82ED64B8: 40980050  bge cr6, 0x82ed6508
	if !ctx.cr[6].lt {
	pc = 0x82ED6508; continue 'dispatch;
	}
	// 82ED64BC: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED64C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED64C4: 409A0018  bne cr6, 0x82ed64dc
	if !ctx.cr[6].eq {
	pc = 0x82ED64DC; continue 'dispatch;
	}
	// 82ED64C8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED64CC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED64D0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED64D4: 7C7AD82E  lwzx r3, r26, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82ED64D8: 4BFCA2D9  bl 0x82ea07b0
	ctx.lr = 0x82ED64DC;
	sub_82EA07B0(ctx, base);
	// 82ED64DC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED64E0: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82ED64E4: 7C7AD82E  lwzx r3, r26, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82ED64E8: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82ED64EC: 4BFCA245  bl 0x82ea0730
	ctx.lr = 0x82ED64F0;
	sub_82EA0730(ctx, base);
	// 82ED64F0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED64F4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82ED64F8: 55490042  rlwinm r9, r10, 0, 1, 1
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED64FC: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED6500: 7D274378  or r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82ED6504: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82ED6508: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED650C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6510: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED6514: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82ED6518: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED651C: 4099001C  ble cr6, 0x82ed6538
	if !ctx.cr[6].gt {
	pc = 0x82ED6538; continue 'dispatch;
	}
	// 82ED6520: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82ED6524: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED6528: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED652C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED6530: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82ED6534: 4082FFF0  bne 0x82ed6524
	if !ctx.cr[0].eq {
	pc = 0x82ED6524; continue 'dispatch;
	}
	// 82ED6538: 815E0094  lwz r10, 0x94(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 82ED653C: 3BFE008C  addi r31, r30, 0x8c
	ctx.r[31].s64 = ctx.r[30].s64 + 140;
	// 82ED6540: 813C0090  lwz r9, 0x90(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED6544: 3BBC008C  addi r29, r28, 0x8c
	ctx.r[29].s64 = ctx.r[28].s64 + 140;
	// 82ED6548: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED654C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED6550: 40980050  bge cr6, 0x82ed65a0
	if !ctx.cr[6].lt {
	pc = 0x82ED65A0; continue 'dispatch;
	}
	// 82ED6554: 554A0000  rlwinm r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED6558: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED655C: 409A0018  bne cr6, 0x82ed6574
	if !ctx.cr[6].eq {
	pc = 0x82ED6574; continue 'dispatch;
	}
	// 82ED6560: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED6564: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6568: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED656C: 7C7AD82E  lwzx r3, r26, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82ED6570: 4BFCA241  bl 0x82ea07b0
	ctx.lr = 0x82ED6574;
	sub_82EA07B0(ctx, base);
	// 82ED6574: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED6578: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82ED657C: 7C7AD82E  lwzx r3, r26, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82ED6580: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82ED6584: 4BFCA1AD  bl 0x82ea0730
	ctx.lr = 0x82ED6588;
	sub_82EA0730(ctx, base);
	// 82ED6588: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED658C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82ED6590: 55490042  rlwinm r9, r10, 0, 1, 1
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED6594: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED6598: 7D274378  or r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82ED659C: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82ED65A0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED65A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED65A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED65AC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82ED65B0: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED65B4: 4099001C  ble cr6, 0x82ed65d0
	if !ctx.cr[6].gt {
	pc = 0x82ED65D0; continue 'dispatch;
	}
	// 82ED65B8: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82ED65BC: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED65C0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED65C4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED65C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82ED65CC: 4082FFF0  bne 0x82ed65bc
	if !ctx.cr[0].eq {
	pc = 0x82ED65BC; continue 'dispatch;
	}
	// 82ED65D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ED65D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED65D8: 4BFF6331  bl 0x82ecc908
	ctx.lr = 0x82ED65DC;
	sub_82ECC908(ctx, base);
	// 82ED65DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED65E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED65E4: 482D1BC8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED65E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED65E8 size=256
    let mut pc: u32 = 0x82ED65E8;
    'dispatch: loop {
        match pc {
            0x82ED65E8 => {
    //   block [0x82ED65E8..0x82ED66E8)
	// 82ED65E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED65EC: 482D1B81  bl 0x831a816c
	ctx.lr = 0x82ED65F0;
	sub_831A8130(ctx, base);
	// 82ED65F0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED65F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED65F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED65FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6600: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED6604: 419A00DC  beq cr6, 0x82ed66e0
	if ctx.cr[6].eq {
	pc = 0x82ED66E0; continue 'dispatch;
	}
	// 82ED6608: 48000451  bl 0x82ed6a58
	ctx.lr = 0x82ED660C;
	sub_82ED6A58(ctx, base);
	// 82ED660C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82ED6610: 409A00D0  bne cr6, 0x82ed66e0
	if !ctx.cr[6].eq {
	pc = 0x82ED66E0; continue 'dispatch;
	}
	// 82ED6614: 817E0158  lwz r11, 0x158(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(344 as u32) ) } as u64;
	// 82ED6618: 3BFE0150  addi r31, r30, 0x150
	ctx.r[31].s64 = ctx.r[30].s64 + 336;
	// 82ED661C: 815E0154  lwz r10, 0x154(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED6620: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED6624: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED6628: 409A0010  bne cr6, 0x82ed6638
	if !ctx.cr[6].eq {
	pc = 0x82ED6638; continue 'dispatch;
	}
	// 82ED662C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82ED6630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED6634: 4BFD024D  bl 0x82ea6880
	ctx.lr = 0x82ED6638;
	sub_82EA6880(ctx, base);
	// 82ED6638: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED663C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82ED6640: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6644: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82ED6648: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82ED664C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED6650: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ED6654: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED6658: 7FEB4214  add r31, r11, r8
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82ED665C: 8166006C  lwz r11, 0x6c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED6660: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED6664: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82ED6668: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED666C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82ED6670: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82ED6674: 4200FFF0  bdnz 0x82ed6664
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82ED6664; continue 'dispatch;
	}
	// 82ED6678: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED667C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED6680: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6684: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82ED6688: 390B1C60  addi r8, r11, 0x1c60
	ctx.r[8].s64 = ctx.r[11].s64 + 7264;
	// 82ED668C: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED6690: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82ED6694: 394B01A0  addi r10, r11, 0x1a0
	ctx.r[10].s64 = ctx.r[11].s64 + 416;
	// 82ED6698: 910100B0  stw r8, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[8].u32 ) };
	// 82ED669C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ED66A0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82ED66A4: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED66A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED66AC: 80E7000C  lwz r7, 0xc(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED66B0: 54E92834  slwi r9, r7, 5
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED66B4: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82ED66B8: 7D4A40AE  lbzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82ED66BC: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82ED66C0: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82ED66C4: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ED66C8: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED66CC: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82ED66D0: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82ED66D4: 4E800421  bctrl
	ctx.lr = 0x82ED66D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED66D8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82ED66DC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82ED66E0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82ED66E4: 482D1AD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED66E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED66E8 size=244
    let mut pc: u32 = 0x82ED66E8;
    'dispatch: loop {
        match pc {
            0x82ED66E8 => {
    //   block [0x82ED66E8..0x82ED67DC)
	// 82ED66E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED66EC: 482D1A79  bl 0x831a8164
	ctx.lr = 0x82ED66F0;
	sub_831A8130(ctx, base);
	// 82ED66F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED66F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED66F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED66FC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED6700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED6704: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82ED6708: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED670C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82ED6710: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ED6714: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED6718: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED671C: 4E800421  bctrl
	ctx.lr = 0x82ED6720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED6720: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED6724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED6728: 4BFFF401  bl 0x82ed5b28
	ctx.lr = 0x82ED672C;
	sub_82ED5B28(ctx, base);
	// 82ED672C: 817F0158  lwz r11, 0x158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 82ED6730: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED6734: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED6738: 409A0034  bne cr6, 0x82ed676c
	if !ctx.cr[6].eq {
	pc = 0x82ED676C; continue 'dispatch;
	}
	// 82ED673C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6740: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED6744: 80FF0154  lwz r7, 0x154(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED6748: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED674C: 80DF0150  lwz r6, 0x150(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED6750: 38894884  addi r4, r9, 0x4884
	ctx.r[4].s64 = ctx.r[9].s64 + 18564;
	// 82ED6754: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ED6758: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ED675C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED6760: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED6764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED6768: 4E800421  bctrl
	ctx.lr = 0x82ED676C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED676C: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED6770: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82ED6774: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED6778: 40990048  ble cr6, 0x82ed67c0
	if !ctx.cr[6].gt {
	pc = 0x82ED67C0; continue 'dispatch;
	}
	// 82ED677C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED6780: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ED6784: 3B6B487C  addi r27, r11, 0x487c
	ctx.r[27].s64 = ctx.r[11].s64 + 18556;
	// 82ED6788: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED678C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82ED6790: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ED6794: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ED6798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED679C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED67A0: 7CCAE82E  lwzx r6, r10, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82ED67A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED67A8: 4E800421  bctrl
	ctx.lr = 0x82ED67AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED67AC: 811F0154  lwz r8, 0x154(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED67B0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82ED67B4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82ED67B8: 7F1C4000  cmpw cr6, r28, r8
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED67BC: 4198FFCC  blt cr6, 0x82ed6788
	if ctx.cr[6].lt {
	pc = 0x82ED6788; continue 'dispatch;
	}
	// 82ED67C0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED67C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED67C8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED67CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED67D0: 4E800421  bctrl
	ctx.lr = 0x82ED67D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED67D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED67D8: 482D19DC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED67E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED67E0 size=144
    let mut pc: u32 = 0x82ED67E0;
    'dispatch: loop {
        match pc {
            0x82ED67E0 => {
    //   block [0x82ED67E0..0x82ED6870)
	// 82ED67E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED67E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED67E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED67EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED67F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED67F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED67F8: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ED67FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED6800: 409A0050  bne cr6, 0x82ed6850
	if !ctx.cr[6].eq {
	pc = 0x82ED6850; continue 'dispatch;
	}
	// 82ED6804: 817E0158  lwz r11, 0x158(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(344 as u32) ) } as u64;
	// 82ED6808: 3BFE0150  addi r31, r30, 0x150
	ctx.r[31].s64 = ctx.r[30].s64 + 336;
	// 82ED680C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED6810: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED6814: 409A0020  bne cr6, 0x82ed6834
	if !ctx.cr[6].eq {
	pc = 0x82ED6834; continue 'dispatch;
	}
	// 82ED6818: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED681C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED6820: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED6824: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6828: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED682C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED6830: 4BFC9F81  bl 0x82ea07b0
	ctx.lr = 0x82ED6834;
	sub_82EA07B0(ctx, base);
	// 82ED6834: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED6838: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ED683C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED6840: 512AF880  rlwimi r10, r9, 0x1f, 2, 0
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[10].u64 & 0x0000000040000000);
	// 82ED6844: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82ED6848: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82ED684C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82ED6850: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED6854: 4BFFE37D  bl 0x82ed4bd0
	ctx.lr = 0x82ED6858;
	sub_82ED4BD0(ctx, base);
	// 82ED6858: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED685C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED6860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED6864: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED6868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED686C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6870 size=484
    let mut pc: u32 = 0x82ED6870;
    'dispatch: loop {
        match pc {
            0x82ED6870 => {
    //   block [0x82ED6870..0x82ED6A54)
	// 82ED6870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6874: 482D18E5  bl 0x831a8158
	ctx.lr = 0x82ED6878;
	sub_831A8130(ctx, base);
	// 82ED6878: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6A58 size=112
    let mut pc: u32 = 0x82ED6A58;
    'dispatch: loop {
        match pc {
            0x82ED6A58 => {
    //   block [0x82ED6A58..0x82ED6AC8)
	// 82ED6A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6A5C: 482D1711  bl 0x831a816c
	ctx.lr = 0x82ED6A60;
	sub_831A8130(ctx, base);
	// 82ED6A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED6A64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED6A68: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82ED6A6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ED6A70: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82ED6A74: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82ED6A78: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED6A7C: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED6A80: 41800040  blt 0x82ed6ac0
	if ctx.cr[0].lt {
	pc = 0x82ED6AC0; continue 'dispatch;
	}
	// 82ED6A84: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82ED6A88: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED6A8C: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED6A90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED6A94: 419A001C  beq cr6, 0x82ed6ab0
	if ctx.cr[6].eq {
	pc = 0x82ED6AB0; continue 'dispatch;
	}
	// 82ED6A98: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82ED6A9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED6AA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6AA4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6AA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED6AAC: 4E800421  bctrl
	ctx.lr = 0x82ED6AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED6AB0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED6AB4: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82ED6AB8: 4080FFD0  bge 0x82ed6a88
	if !ctx.cr[0].lt {
	pc = 0x82ED6A88; continue 'dispatch;
	}
	// 82ED6ABC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED6AC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED6AC4: 482D16F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6AC8 size=104
    let mut pc: u32 = 0x82ED6AC8;
    'dispatch: loop {
        match pc {
            0x82ED6AC8 => {
    //   block [0x82ED6AC8..0x82ED6B30)
	// 82ED6AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6ACC: 482D16A1  bl 0x831a816c
	ctx.lr = 0x82ED6AD0;
	sub_831A8130(ctx, base);
	// 82ED6AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED6AD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED6AD8: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82ED6ADC: 98A10058  stb r5, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u8 ) };
	// 82ED6AE0: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82ED6AE4: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED6AE8: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED6AEC: 4180003C  blt 0x82ed6b28
	if ctx.cr[0].lt {
	pc = 0x82ED6B28; continue 'dispatch;
	}
	// 82ED6AF0: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82ED6AF4: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ED6AF8: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ED6AFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED6B00: 419A001C  beq cr6, 0x82ed6b1c
	if ctx.cr[6].eq {
	pc = 0x82ED6B1C; continue 'dispatch;
	}
	// 82ED6B04: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82ED6B08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED6B0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6B10: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED6B14: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED6B18: 4E800421  bctrl
	ctx.lr = 0x82ED6B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED6B1C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED6B20: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82ED6B24: 4080FFD0  bge 0x82ed6af4
	if !ctx.cr[0].lt {
	pc = 0x82ED6AF4; continue 'dispatch;
	}
	// 82ED6B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED6B2C: 482D1690  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6B30 size=100
    let mut pc: u32 = 0x82ED6B30;
    'dispatch: loop {
        match pc {
            0x82ED6B30 => {
    //   block [0x82ED6B30..0x82ED6B94)
	// 82ED6B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED6B38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED6B3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED6B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED6B44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED6B48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED6B4C: 4BFFF835  bl 0x82ed6380
	ctx.lr = 0x82ED6B50;
	sub_82ED6380(ctx, base);
	// 82ED6B50: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED6B54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED6B58: 419A0020  beq cr6, 0x82ed6b78
	if ctx.cr[6].eq {
	pc = 0x82ED6B78; continue 'dispatch;
	}
	// 82ED6B5C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6B60: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED6B64: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82ED6B68: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED6B6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED6B70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED6B74: 4BFC9C3D  bl 0x82ea07b0
	ctx.lr = 0x82ED6B78;
	sub_82EA07B0(ctx, base);
	// 82ED6B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED6B7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED6B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED6B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED6B88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED6B8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED6B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6B98 size=4
    let mut pc: u32 = 0x82ED6B98;
    'dispatch: loop {
        match pc {
            0x82ED6B98 => {
    //   block [0x82ED6B98..0x82ED6B9C)
	// 82ED6B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BA0 size=4
    let mut pc: u32 = 0x82ED6BA0;
    'dispatch: loop {
        match pc {
            0x82ED6BA0 => {
    //   block [0x82ED6BA0..0x82ED6BA4)
	// 82ED6BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BA8 size=4
    let mut pc: u32 = 0x82ED6BA8;
    'dispatch: loop {
        match pc {
            0x82ED6BA8 => {
    //   block [0x82ED6BA8..0x82ED6BAC)
	// 82ED6BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BB0 size=12
    let mut pc: u32 = 0x82ED6BB0;
    'dispatch: loop {
        match pc {
            0x82ED6BB0 => {
    //   block [0x82ED6BB0..0x82ED6BBC)
	// 82ED6BB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED6BB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6BB8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BBC size=8
    let mut pc: u32 = 0x82ED6BBC;
    'dispatch: loop {
        match pc {
            0x82ED6BBC => {
    //   block [0x82ED6BBC..0x82ED6BC4)
	// 82ED6BBC: 480169BC  b 0x82eed578
	sub_82EED578(ctx, base);
	return;
	// 82ED6BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BC8 size=4
    let mut pc: u32 = 0x82ED6BC8;
    'dispatch: loop {
        match pc {
            0x82ED6BC8 => {
    //   block [0x82ED6BC8..0x82ED6BCC)
	// 82ED6BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BD0 size=4
    let mut pc: u32 = 0x82ED6BD0;
    'dispatch: loop {
        match pc {
            0x82ED6BD0 => {
    //   block [0x82ED6BD0..0x82ED6BD4)
	// 82ED6BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BD8 size=4
    let mut pc: u32 = 0x82ED6BD8;
    'dispatch: loop {
        match pc {
            0x82ED6BD8 => {
    //   block [0x82ED6BD8..0x82ED6BDC)
	// 82ED6BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BE0 size=4
    let mut pc: u32 = 0x82ED6BE0;
    'dispatch: loop {
        match pc {
            0x82ED6BE0 => {
    //   block [0x82ED6BE0..0x82ED6BE4)
	// 82ED6BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BE8 size=4
    let mut pc: u32 = 0x82ED6BE8;
    'dispatch: loop {
        match pc {
            0x82ED6BE8 => {
    //   block [0x82ED6BE8..0x82ED6BEC)
	// 82ED6BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BF0 size=4
    let mut pc: u32 = 0x82ED6BF0;
    'dispatch: loop {
        match pc {
            0x82ED6BF0 => {
    //   block [0x82ED6BF0..0x82ED6BF4)
	// 82ED6BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6BF8 size=4
    let mut pc: u32 = 0x82ED6BF8;
    'dispatch: loop {
        match pc {
            0x82ED6BF8 => {
    //   block [0x82ED6BF8..0x82ED6BFC)
	// 82ED6BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C00 size=4
    let mut pc: u32 = 0x82ED6C00;
    'dispatch: loop {
        match pc {
            0x82ED6C00 => {
    //   block [0x82ED6C00..0x82ED6C04)
	// 82ED6C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C08 size=4
    let mut pc: u32 = 0x82ED6C08;
    'dispatch: loop {
        match pc {
            0x82ED6C08 => {
    //   block [0x82ED6C08..0x82ED6C0C)
	// 82ED6C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C10 size=4
    let mut pc: u32 = 0x82ED6C10;
    'dispatch: loop {
        match pc {
            0x82ED6C10 => {
    //   block [0x82ED6C10..0x82ED6C14)
	// 82ED6C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C18 size=4
    let mut pc: u32 = 0x82ED6C18;
    'dispatch: loop {
        match pc {
            0x82ED6C18 => {
    //   block [0x82ED6C18..0x82ED6C1C)
	// 82ED6C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C20 size=4
    let mut pc: u32 = 0x82ED6C20;
    'dispatch: loop {
        match pc {
            0x82ED6C20 => {
    //   block [0x82ED6C20..0x82ED6C24)
	// 82ED6C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C28 size=4
    let mut pc: u32 = 0x82ED6C28;
    'dispatch: loop {
        match pc {
            0x82ED6C28 => {
    //   block [0x82ED6C28..0x82ED6C2C)
	// 82ED6C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C30 size=4
    let mut pc: u32 = 0x82ED6C30;
    'dispatch: loop {
        match pc {
            0x82ED6C30 => {
    //   block [0x82ED6C30..0x82ED6C34)
	// 82ED6C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C38 size=4
    let mut pc: u32 = 0x82ED6C38;
    'dispatch: loop {
        match pc {
            0x82ED6C38 => {
    //   block [0x82ED6C38..0x82ED6C3C)
	// 82ED6C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C40 size=4
    let mut pc: u32 = 0x82ED6C40;
    'dispatch: loop {
        match pc {
            0x82ED6C40 => {
    //   block [0x82ED6C40..0x82ED6C44)
	// 82ED6C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C48 size=4
    let mut pc: u32 = 0x82ED6C48;
    'dispatch: loop {
        match pc {
            0x82ED6C48 => {
    //   block [0x82ED6C48..0x82ED6C4C)
	// 82ED6C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C50 size=4
    let mut pc: u32 = 0x82ED6C50;
    'dispatch: loop {
        match pc {
            0x82ED6C50 => {
    //   block [0x82ED6C50..0x82ED6C54)
	// 82ED6C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C58 size=4
    let mut pc: u32 = 0x82ED6C58;
    'dispatch: loop {
        match pc {
            0x82ED6C58 => {
    //   block [0x82ED6C58..0x82ED6C5C)
	// 82ED6C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C60 size=4
    let mut pc: u32 = 0x82ED6C60;
    'dispatch: loop {
        match pc {
            0x82ED6C60 => {
    //   block [0x82ED6C60..0x82ED6C64)
	// 82ED6C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C68 size=4
    let mut pc: u32 = 0x82ED6C68;
    'dispatch: loop {
        match pc {
            0x82ED6C68 => {
    //   block [0x82ED6C68..0x82ED6C6C)
	// 82ED6C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C70 size=4
    let mut pc: u32 = 0x82ED6C70;
    'dispatch: loop {
        match pc {
            0x82ED6C70 => {
    //   block [0x82ED6C70..0x82ED6C74)
	// 82ED6C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C78 size=4
    let mut pc: u32 = 0x82ED6C78;
    'dispatch: loop {
        match pc {
            0x82ED6C78 => {
    //   block [0x82ED6C78..0x82ED6C7C)
	// 82ED6C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C80 size=4
    let mut pc: u32 = 0x82ED6C80;
    'dispatch: loop {
        match pc {
            0x82ED6C80 => {
    //   block [0x82ED6C80..0x82ED6C84)
	// 82ED6C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C88 size=4
    let mut pc: u32 = 0x82ED6C88;
    'dispatch: loop {
        match pc {
            0x82ED6C88 => {
    //   block [0x82ED6C88..0x82ED6C8C)
	// 82ED6C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C90 size=4
    let mut pc: u32 = 0x82ED6C90;
    'dispatch: loop {
        match pc {
            0x82ED6C90 => {
    //   block [0x82ED6C90..0x82ED6C94)
	// 82ED6C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6C98 size=4
    let mut pc: u32 = 0x82ED6C98;
    'dispatch: loop {
        match pc {
            0x82ED6C98 => {
    //   block [0x82ED6C98..0x82ED6C9C)
	// 82ED6C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CA0 size=4
    let mut pc: u32 = 0x82ED6CA0;
    'dispatch: loop {
        match pc {
            0x82ED6CA0 => {
    //   block [0x82ED6CA0..0x82ED6CA4)
	// 82ED6CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CA8 size=8
    let mut pc: u32 = 0x82ED6CA8;
    'dispatch: loop {
        match pc {
            0x82ED6CA8 => {
    //   block [0x82ED6CA8..0x82ED6CB0)
	// 82ED6CA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6CAC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CB0 size=8
    let mut pc: u32 = 0x82ED6CB0;
    'dispatch: loop {
        match pc {
            0x82ED6CB0 => {
    //   block [0x82ED6CB0..0x82ED6CB8)
	// 82ED6CB0: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED6CB4: 48016724  b 0x82eed3d8
	sub_82EED3D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CB8 size=4
    let mut pc: u32 = 0x82ED6CB8;
    'dispatch: loop {
        match pc {
            0x82ED6CB8 => {
    //   block [0x82ED6CB8..0x82ED6CBC)
	// 82ED6CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CC0 size=8
    let mut pc: u32 = 0x82ED6CC0;
    'dispatch: loop {
        match pc {
            0x82ED6CC0 => {
    //   block [0x82ED6CC0..0x82ED6CC8)
	// 82ED6CC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6CC4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CC8 size=8
    let mut pc: u32 = 0x82ED6CC8;
    'dispatch: loop {
        match pc {
            0x82ED6CC8 => {
    //   block [0x82ED6CC8..0x82ED6CD0)
	// 82ED6CC8: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED6CCC: 4801670C  b 0x82eed3d8
	sub_82EED3D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CD0 size=4
    let mut pc: u32 = 0x82ED6CD0;
    'dispatch: loop {
        match pc {
            0x82ED6CD0 => {
    //   block [0x82ED6CD0..0x82ED6CD4)
	// 82ED6CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CD8 size=4
    let mut pc: u32 = 0x82ED6CD8;
    'dispatch: loop {
        match pc {
            0x82ED6CD8 => {
    //   block [0x82ED6CD8..0x82ED6CDC)
	// 82ED6CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CE0 size=4
    let mut pc: u32 = 0x82ED6CE0;
    'dispatch: loop {
        match pc {
            0x82ED6CE0 => {
    //   block [0x82ED6CE0..0x82ED6CE4)
	// 82ED6CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CE8 size=4
    let mut pc: u32 = 0x82ED6CE8;
    'dispatch: loop {
        match pc {
            0x82ED6CE8 => {
    //   block [0x82ED6CE8..0x82ED6CEC)
	// 82ED6CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CF0 size=4
    let mut pc: u32 = 0x82ED6CF0;
    'dispatch: loop {
        match pc {
            0x82ED6CF0 => {
    //   block [0x82ED6CF0..0x82ED6CF4)
	// 82ED6CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6CF8 size=4
    let mut pc: u32 = 0x82ED6CF8;
    'dispatch: loop {
        match pc {
            0x82ED6CF8 => {
    //   block [0x82ED6CF8..0x82ED6CFC)
	// 82ED6CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D00 size=4
    let mut pc: u32 = 0x82ED6D00;
    'dispatch: loop {
        match pc {
            0x82ED6D00 => {
    //   block [0x82ED6D00..0x82ED6D04)
	// 82ED6D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D08 size=4
    let mut pc: u32 = 0x82ED6D08;
    'dispatch: loop {
        match pc {
            0x82ED6D08 => {
    //   block [0x82ED6D08..0x82ED6D0C)
	// 82ED6D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D10 size=4
    let mut pc: u32 = 0x82ED6D10;
    'dispatch: loop {
        match pc {
            0x82ED6D10 => {
    //   block [0x82ED6D10..0x82ED6D14)
	// 82ED6D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D18 size=4
    let mut pc: u32 = 0x82ED6D18;
    'dispatch: loop {
        match pc {
            0x82ED6D18 => {
    //   block [0x82ED6D18..0x82ED6D1C)
	// 82ED6D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D20 size=4
    let mut pc: u32 = 0x82ED6D20;
    'dispatch: loop {
        match pc {
            0x82ED6D20 => {
    //   block [0x82ED6D20..0x82ED6D24)
	// 82ED6D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D28 size=4
    let mut pc: u32 = 0x82ED6D28;
    'dispatch: loop {
        match pc {
            0x82ED6D28 => {
    //   block [0x82ED6D28..0x82ED6D2C)
	// 82ED6D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D30 size=4
    let mut pc: u32 = 0x82ED6D30;
    'dispatch: loop {
        match pc {
            0x82ED6D30 => {
    //   block [0x82ED6D30..0x82ED6D34)
	// 82ED6D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D38 size=4
    let mut pc: u32 = 0x82ED6D38;
    'dispatch: loop {
        match pc {
            0x82ED6D38 => {
    //   block [0x82ED6D38..0x82ED6D3C)
	// 82ED6D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D40 size=4
    let mut pc: u32 = 0x82ED6D40;
    'dispatch: loop {
        match pc {
            0x82ED6D40 => {
    //   block [0x82ED6D40..0x82ED6D44)
	// 82ED6D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D48 size=4
    let mut pc: u32 = 0x82ED6D48;
    'dispatch: loop {
        match pc {
            0x82ED6D48 => {
    //   block [0x82ED6D48..0x82ED6D4C)
	// 82ED6D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D50 size=4
    let mut pc: u32 = 0x82ED6D50;
    'dispatch: loop {
        match pc {
            0x82ED6D50 => {
    //   block [0x82ED6D50..0x82ED6D54)
	// 82ED6D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D58 size=4
    let mut pc: u32 = 0x82ED6D58;
    'dispatch: loop {
        match pc {
            0x82ED6D58 => {
    //   block [0x82ED6D58..0x82ED6D5C)
	// 82ED6D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D60 size=4
    let mut pc: u32 = 0x82ED6D60;
    'dispatch: loop {
        match pc {
            0x82ED6D60 => {
    //   block [0x82ED6D60..0x82ED6D64)
	// 82ED6D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D68 size=4
    let mut pc: u32 = 0x82ED6D68;
    'dispatch: loop {
        match pc {
            0x82ED6D68 => {
    //   block [0x82ED6D68..0x82ED6D6C)
	// 82ED6D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D70 size=4
    let mut pc: u32 = 0x82ED6D70;
    'dispatch: loop {
        match pc {
            0x82ED6D70 => {
    //   block [0x82ED6D70..0x82ED6D74)
	// 82ED6D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D78 size=4
    let mut pc: u32 = 0x82ED6D78;
    'dispatch: loop {
        match pc {
            0x82ED6D78 => {
    //   block [0x82ED6D78..0x82ED6D7C)
	// 82ED6D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D80 size=4
    let mut pc: u32 = 0x82ED6D80;
    'dispatch: loop {
        match pc {
            0x82ED6D80 => {
    //   block [0x82ED6D80..0x82ED6D84)
	// 82ED6D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D88 size=4
    let mut pc: u32 = 0x82ED6D88;
    'dispatch: loop {
        match pc {
            0x82ED6D88 => {
    //   block [0x82ED6D88..0x82ED6D8C)
	// 82ED6D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D90 size=4
    let mut pc: u32 = 0x82ED6D90;
    'dispatch: loop {
        match pc {
            0x82ED6D90 => {
    //   block [0x82ED6D90..0x82ED6D94)
	// 82ED6D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6D98 size=4
    let mut pc: u32 = 0x82ED6D98;
    'dispatch: loop {
        match pc {
            0x82ED6D98 => {
    //   block [0x82ED6D98..0x82ED6D9C)
	// 82ED6D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6DA0 size=4
    let mut pc: u32 = 0x82ED6DA0;
    'dispatch: loop {
        match pc {
            0x82ED6DA0 => {
    //   block [0x82ED6DA0..0x82ED6DA4)
	// 82ED6DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6DA8 size=4
    let mut pc: u32 = 0x82ED6DA8;
    'dispatch: loop {
        match pc {
            0x82ED6DA8 => {
    //   block [0x82ED6DA8..0x82ED6DAC)
	// 82ED6DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6DB0 size=4
    let mut pc: u32 = 0x82ED6DB0;
    'dispatch: loop {
        match pc {
            0x82ED6DB0 => {
    //   block [0x82ED6DB0..0x82ED6DB4)
	// 82ED6DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6DB8 size=4
    let mut pc: u32 = 0x82ED6DB8;
    'dispatch: loop {
        match pc {
            0x82ED6DB8 => {
    //   block [0x82ED6DB8..0x82ED6DBC)
	// 82ED6DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6DC0 size=20
    let mut pc: u32 = 0x82ED6DC0;
    'dispatch: loop {
        match pc {
            0x82ED6DC0 => {
    //   block [0x82ED6DC0..0x82ED6DD4)
	// 82ED6DC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6DC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED6DC8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6DCC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED6DD0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6DD8 size=4
    let mut pc: u32 = 0x82ED6DD8;
    'dispatch: loop {
        match pc {
            0x82ED6DD8 => {
    //   block [0x82ED6DD8..0x82ED6DDC)
	// 82ED6DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6DE0 size=8
    let mut pc: u32 = 0x82ED6DE0;
    'dispatch: loop {
        match pc {
            0x82ED6DE0 => {
    //   block [0x82ED6DE0..0x82ED6DE8)
	// 82ED6DE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6DE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6DE8 size=24
    let mut pc: u32 = 0x82ED6DE8;
    'dispatch: loop {
        match pc {
            0x82ED6DE8 => {
    //   block [0x82ED6DE8..0x82ED6E00)
	// 82ED6DE8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED6DEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED6DF0: 392B6A54  addi r9, r11, 0x6a54
	ctx.r[9].s64 = ctx.r[11].s64 + 27220;
	// 82ED6DF4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED6DF8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED6DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E00 size=12
    let mut pc: u32 = 0x82ED6E00;
    'dispatch: loop {
        match pc {
            0x82ED6E00 => {
    //   block [0x82ED6E00..0x82ED6E0C)
	// 82ED6E00: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED6E04: 386B6A54  addi r3, r11, 0x6a54
	ctx.r[3].s64 = ctx.r[11].s64 + 27220;
	// 82ED6E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E10 size=4
    let mut pc: u32 = 0x82ED6E10;
    'dispatch: loop {
        match pc {
            0x82ED6E10 => {
    //   block [0x82ED6E10..0x82ED6E14)
	// 82ED6E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E18 size=20
    let mut pc: u32 = 0x82ED6E18;
    'dispatch: loop {
        match pc {
            0x82ED6E18 => {
    //   block [0x82ED6E18..0x82ED6E2C)
	// 82ED6E18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6E1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED6E20: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6E24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED6E28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E30 size=4
    let mut pc: u32 = 0x82ED6E30;
    'dispatch: loop {
        match pc {
            0x82ED6E30 => {
    //   block [0x82ED6E30..0x82ED6E34)
	// 82ED6E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E38 size=8
    let mut pc: u32 = 0x82ED6E38;
    'dispatch: loop {
        match pc {
            0x82ED6E38 => {
    //   block [0x82ED6E38..0x82ED6E40)
	// 82ED6E38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6E3C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E40 size=24
    let mut pc: u32 = 0x82ED6E40;
    'dispatch: loop {
        match pc {
            0x82ED6E40 => {
    //   block [0x82ED6E40..0x82ED6E58)
	// 82ED6E40: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED6E44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED6E48: 392B6B0C  addi r9, r11, 0x6b0c
	ctx.r[9].s64 = ctx.r[11].s64 + 27404;
	// 82ED6E4C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED6E50: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED6E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E58 size=12
    let mut pc: u32 = 0x82ED6E58;
    'dispatch: loop {
        match pc {
            0x82ED6E58 => {
    //   block [0x82ED6E58..0x82ED6E64)
	// 82ED6E58: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED6E5C: 386B6B0C  addi r3, r11, 0x6b0c
	ctx.r[3].s64 = ctx.r[11].s64 + 27404;
	// 82ED6E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E68 size=4
    let mut pc: u32 = 0x82ED6E68;
    'dispatch: loop {
        match pc {
            0x82ED6E68 => {
    //   block [0x82ED6E68..0x82ED6E6C)
	// 82ED6E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E70 size=12
    let mut pc: u32 = 0x82ED6E70;
    'dispatch: loop {
        match pc {
            0x82ED6E70 => {
    //   block [0x82ED6E70..0x82ED6E7C)
	// 82ED6E70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED6E74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6E78: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E7C size=8
    let mut pc: u32 = 0x82ED6E7C;
    'dispatch: loop {
        match pc {
            0x82ED6E7C => {
    //   block [0x82ED6E7C..0x82ED6E84)
	// 82ED6E7C: 48017A24  b 0x82eee8a0
	sub_82EEE8A0(ctx, base);
	return;
	// 82ED6E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6E88 size=20
    let mut pc: u32 = 0x82ED6E88;
    'dispatch: loop {
        match pc {
            0x82ED6E88 => {
    //   block [0x82ED6E88..0x82ED6E9C)
	// 82ED6E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6E8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED6E90: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6E94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED6E98: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6EA0 size=44
    let mut pc: u32 = 0x82ED6EA0;
    'dispatch: loop {
        match pc {
            0x82ED6EA0 => {
    //   block [0x82ED6EA0..0x82ED6ECC)
	// 82ED6EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED6EA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED6EAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED6EB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED6EB4: 480179ED  bl 0x82eee8a0
	ctx.lr = 0x82ED6EB8;
	sub_82EEE8A0(ctx, base);
	// 82ED6EB8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED6EBC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED6EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED6EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED6EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6ED0 size=4
    let mut pc: u32 = 0x82ED6ED0;
    'dispatch: loop {
        match pc {
            0x82ED6ED0 => {
    //   block [0x82ED6ED0..0x82ED6ED4)
	// 82ED6ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6ED8 size=20
    let mut pc: u32 = 0x82ED6ED8;
    'dispatch: loop {
        match pc {
            0x82ED6ED8 => {
    //   block [0x82ED6ED8..0x82ED6EEC)
	// 82ED6ED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6EDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED6EE0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6EE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED6EE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6EF0 size=8
    let mut pc: u32 = 0x82ED6EF0;
    'dispatch: loop {
        match pc {
            0x82ED6EF0 => {
    //   block [0x82ED6EF0..0x82ED6EF8)
	// 82ED6EF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6EF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6EF8 size=24
    let mut pc: u32 = 0x82ED6EF8;
    'dispatch: loop {
        match pc {
            0x82ED6EF8 => {
    //   block [0x82ED6EF8..0x82ED6F10)
	// 82ED6EF8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED6EFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED6F00: 392B6E4C  addi r9, r11, 0x6e4c
	ctx.r[9].s64 = ctx.r[11].s64 + 28236;
	// 82ED6F04: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED6F08: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED6F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6F10 size=12
    let mut pc: u32 = 0x82ED6F10;
    'dispatch: loop {
        match pc {
            0x82ED6F10 => {
    //   block [0x82ED6F10..0x82ED6F1C)
	// 82ED6F10: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED6F14: 386B6E4C  addi r3, r11, 0x6e4c
	ctx.r[3].s64 = ctx.r[11].s64 + 28236;
	// 82ED6F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6F20 size=4
    let mut pc: u32 = 0x82ED6F20;
    'dispatch: loop {
        match pc {
            0x82ED6F20 => {
    //   block [0x82ED6F20..0x82ED6F24)
	// 82ED6F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6F28 size=20
    let mut pc: u32 = 0x82ED6F28;
    'dispatch: loop {
        match pc {
            0x82ED6F28 => {
    //   block [0x82ED6F28..0x82ED6F3C)
	// 82ED6F28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6F2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED6F30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED6F34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED6F38: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6F40 size=12
    let mut pc: u32 = 0x82ED6F40;
    'dispatch: loop {
        match pc {
            0x82ED6F40 => {
    //   block [0x82ED6F40..0x82ED6F4C)
	// 82ED6F40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED6F44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED6F48: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6F4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED6F4C size=8
    let mut pc: u32 = 0x82ED6F4C;
    'dispatch: loop {
        match pc {
            0x82ED6F4C => {
    //   block [0x82ED6F4C..0x82ED6F54)
	// 82ED6F4C: 4800003C  b 0x82ed6f88
	sub_82ED6F88(ctx, base);
	return;
	// 82ED6F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6F58 size=44
    let mut pc: u32 = 0x82ED6F58;
    'dispatch: loop {
        match pc {
            0x82ED6F58 => {
    //   block [0x82ED6F58..0x82ED6F84)
	// 82ED6F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED6F60: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED6F64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED6F68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED6F6C: 4800001D  bl 0x82ed6f88
	ctx.lr = 0x82ED6F70;
	sub_82ED6F88(ctx, base);
	// 82ED6F70: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED6F74: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82ED6F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED6F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED6F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6F88 size=104
    let mut pc: u32 = 0x82ED6F88;
    'dispatch: loop {
        match pc {
            0x82ED6F88 => {
    //   block [0x82ED6F88..0x82ED6FF0)
	// 82ED6F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED6F90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED6F94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED6F98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED6F9C: 4BFFE595  bl 0x82ed5530
	ctx.lr = 0x82ED6FA0;
	sub_82ED5530(ctx, base);
	// 82ED6FA0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED6FA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED6FA8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82ED6FAC: 39096EF4  addi r8, r9, 0x6ef4
	ctx.r[8].s64 = ctx.r[9].s64 + 28404;
	// 82ED6FB0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ED6FB4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED6FB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED6FBC: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82ED6FC0: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82ED6FC4: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82ED6FC8: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82ED6FCC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED6FD0: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82ED6FD4: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82ED6FD8: 915F0158  stw r10, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 82ED6FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED6FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED6FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED6FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED6FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED6FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED6FF0 size=100
    let mut pc: u32 = 0x82ED6FF0;
    'dispatch: loop {
        match pc {
            0x82ED6FF0 => {
    //   block [0x82ED6FF0..0x82ED7054)
	// 82ED6FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED6FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED6FF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED6FFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7008: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED700C: 480180BD  bl 0x82eef0c8
	ctx.lr = 0x82ED7010;
	sub_82EEF0C8(ctx, base);
	// 82ED7010: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED7014: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7018: 419A0020  beq cr6, 0x82ed7038
	if ctx.cr[6].eq {
	pc = 0x82ED7038; continue 'dispatch;
	}
	// 82ED701C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7020: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED7024: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82ED7028: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED702C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED7030: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED7034: 4BFC977D  bl 0x82ea07b0
	ctx.lr = 0x82ED7038;
	sub_82EA07B0(ctx, base);
	// 82ED7038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED703C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED7040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7048: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED704C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7058 size=4
    let mut pc: u32 = 0x82ED7058;
    'dispatch: loop {
        match pc {
            0x82ED7058 => {
    //   block [0x82ED7058..0x82ED705C)
	// 82ED7058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7060 size=20
    let mut pc: u32 = 0x82ED7060;
    'dispatch: loop {
        match pc {
            0x82ED7060 => {
    //   block [0x82ED7060..0x82ED7074)
	// 82ED7060: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7064: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7068: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED706C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7070: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7078 size=4
    let mut pc: u32 = 0x82ED7078;
    'dispatch: loop {
        match pc {
            0x82ED7078 => {
    //   block [0x82ED7078..0x82ED707C)
	// 82ED7078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7080 size=8
    let mut pc: u32 = 0x82ED7080;
    'dispatch: loop {
        match pc {
            0x82ED7080 => {
    //   block [0x82ED7080..0x82ED7088)
	// 82ED7080: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7084: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7088 size=24
    let mut pc: u32 = 0x82ED7088;
    'dispatch: loop {
        match pc {
            0x82ED7088 => {
    //   block [0x82ED7088..0x82ED70A0)
	// 82ED7088: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED708C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7090: 392B6FD0  addi r9, r11, 0x6fd0
	ctx.r[9].s64 = ctx.r[11].s64 + 28624;
	// 82ED7094: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7098: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED709C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED70A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED70A0 size=12
    let mut pc: u32 = 0x82ED70A0;
    'dispatch: loop {
        match pc {
            0x82ED70A0 => {
    //   block [0x82ED70A0..0x82ED70AC)
	// 82ED70A0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED70A4: 386B6FD0  addi r3, r11, 0x6fd0
	ctx.r[3].s64 = ctx.r[11].s64 + 28624;
	// 82ED70A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED70B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED70B0 size=4
    let mut pc: u32 = 0x82ED70B0;
    'dispatch: loop {
        match pc {
            0x82ED70B0 => {
    //   block [0x82ED70B0..0x82ED70B4)
	// 82ED70B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED70B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED70B8 size=20
    let mut pc: u32 = 0x82ED70B8;
    'dispatch: loop {
        match pc {
            0x82ED70B8 => {
    //   block [0x82ED70B8..0x82ED70CC)
	// 82ED70B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED70BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED70C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED70C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED70C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED70D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED70D0 size=4
    let mut pc: u32 = 0x82ED70D0;
    'dispatch: loop {
        match pc {
            0x82ED70D0 => {
    //   block [0x82ED70D0..0x82ED70D4)
	// 82ED70D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED70D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED70D8 size=12
    let mut pc: u32 = 0x82ED70D8;
    'dispatch: loop {
        match pc {
            0x82ED70D8 => {
    //   block [0x82ED70D8..0x82ED70E4)
	// 82ED70D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED70DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED70E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED70E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED70E4 size=8
    let mut pc: u32 = 0x82ED70E4;
    'dispatch: loop {
        match pc {
            0x82ED70E4 => {
    //   block [0x82ED70E4..0x82ED70EC)
	// 82ED70E4: 4800003C  b 0x82ed7120
	sub_82ED7120(ctx, base);
	return;
	// 82ED70E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED70F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED70F0 size=44
    let mut pc: u32 = 0x82ED70F0;
    'dispatch: loop {
        match pc {
            0x82ED70F0 => {
    //   block [0x82ED70F0..0x82ED711C)
	// 82ED70F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED70F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED70F8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED70FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7100: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED7104: 4800001D  bl 0x82ed7120
	ctx.lr = 0x82ED7108;
	sub_82ED7120(ctx, base);
	// 82ED7108: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED710C: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82ED7110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7120 size=100
    let mut pc: u32 = 0x82ED7120;
    'dispatch: loop {
        match pc {
            0x82ED7120 => {
    //   block [0x82ED7120..0x82ED7184)
	// 82ED7120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED712C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7134: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED7138: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ED713C: 392A713C  addi r9, r10, 0x713c
	ctx.r[9].s64 = ctx.r[10].s64 + 28988;
	// 82ED7140: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82ED7144: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82ED7148: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED714C: 419A0024  beq cr6, 0x82ed7170
	if ctx.cr[6].eq {
	pc = 0x82ED7170; continue 'dispatch;
	}
	// 82ED7150: 895F009A  lbz r10, 0x9a(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 82ED7154: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED7158: 419A0018  beq cr6, 0x82ed7170
	if ctx.cr[6].eq {
	pc = 0x82ED7170; continue 'dispatch;
	}
	// 82ED715C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82ED7160: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED7164: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7168: 48018BD1  bl 0x82eefd38
	ctx.lr = 0x82ED716C;
	sub_82EEFD38(ctx, base);
	// 82ED716C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED7170: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED7174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED717C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7188 size=4
    let mut pc: u32 = 0x82ED7188;
    'dispatch: loop {
        match pc {
            0x82ED7188 => {
    //   block [0x82ED7188..0x82ED718C)
	// 82ED7188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7190 size=20
    let mut pc: u32 = 0x82ED7190;
    'dispatch: loop {
        match pc {
            0x82ED7190 => {
    //   block [0x82ED7190..0x82ED71A4)
	// 82ED7190: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7194: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7198: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED719C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED71A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED71A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED71A8 size=4
    let mut pc: u32 = 0x82ED71A8;
    'dispatch: loop {
        match pc {
            0x82ED71A8 => {
    //   block [0x82ED71A8..0x82ED71AC)
	// 82ED71A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED71B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED71B0 size=12
    let mut pc: u32 = 0x82ED71B0;
    'dispatch: loop {
        match pc {
            0x82ED71B0 => {
    //   block [0x82ED71B0..0x82ED71BC)
	// 82ED71B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED71B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED71B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED71BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED71BC size=8
    let mut pc: u32 = 0x82ED71BC;
    'dispatch: loop {
        match pc {
            0x82ED71BC => {
    //   block [0x82ED71BC..0x82ED71C4)
	// 82ED71BC: 4800003C  b 0x82ed71f8
	sub_82ED71F8(ctx, base);
	return;
	// 82ED71C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED71C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED71C8 size=44
    let mut pc: u32 = 0x82ED71C8;
    'dispatch: loop {
        match pc {
            0x82ED71C8 => {
    //   block [0x82ED71C8..0x82ED71F4)
	// 82ED71C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED71CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED71D0: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED71D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED71D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED71DC: 4800001D  bl 0x82ed71f8
	ctx.lr = 0x82ED71E0;
	sub_82ED71F8(ctx, base);
	// 82ED71E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED71E4: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82ED71E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED71EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED71F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED71F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED71F8 size=100
    let mut pc: u32 = 0x82ED71F8;
    'dispatch: loop {
        match pc {
            0x82ED71F8 => {
    //   block [0x82ED71F8..0x82ED725C)
	// 82ED71F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED71FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7204: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7208: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED720C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED7210: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ED7214: 392A7324  addi r9, r10, 0x7324
	ctx.r[9].s64 = ctx.r[10].s64 + 29476;
	// 82ED7218: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82ED721C: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82ED7220: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7224: 419A0024  beq cr6, 0x82ed7248
	if ctx.cr[6].eq {
	pc = 0x82ED7248; continue 'dispatch;
	}
	// 82ED7228: 895F0116  lbz r10, 0x116(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(278 as u32) ) } as u64;
	// 82ED722C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED7230: 419A0018  beq cr6, 0x82ed7248
	if ctx.cr[6].eq {
	pc = 0x82ED7248; continue 'dispatch;
	}
	// 82ED7234: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82ED7238: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ED723C: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7240: 48004691  bl 0x82edb8d0
	ctx.lr = 0x82ED7244;
	sub_82EDB8D0(ctx, base);
	// 82ED7244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED7248: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED724C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7254: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7260 size=100
    let mut pc: u32 = 0x82ED7260;
    'dispatch: loop {
        match pc {
            0x82ED7260 => {
    //   block [0x82ED7260..0x82ED72C4)
	// 82ED7260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED726C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7278: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED727C: 48004C25  bl 0x82edbea0
	ctx.lr = 0x82ED7280;
	sub_82EDBEA0(ctx, base);
	// 82ED7280: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED7284: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7288: 419A0020  beq cr6, 0x82ed72a8
	if ctx.cr[6].eq {
	pc = 0x82ED72A8; continue 'dispatch;
	}
	// 82ED728C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7290: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED7294: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82ED7298: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED729C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED72A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED72A4: 4BFC950D  bl 0x82ea07b0
	ctx.lr = 0x82ED72A8;
	sub_82EA07B0(ctx, base);
	// 82ED72A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED72AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED72B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED72B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED72B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED72BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED72C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED72C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED72C8 size=4
    let mut pc: u32 = 0x82ED72C8;
    'dispatch: loop {
        match pc {
            0x82ED72C8 => {
    //   block [0x82ED72C8..0x82ED72CC)
	// 82ED72C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED72D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED72D0 size=20
    let mut pc: u32 = 0x82ED72D0;
    'dispatch: loop {
        match pc {
            0x82ED72D0 => {
    //   block [0x82ED72D0..0x82ED72E4)
	// 82ED72D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED72D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED72D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED72DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED72E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED72E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED72E8 size=4
    let mut pc: u32 = 0x82ED72E8;
    'dispatch: loop {
        match pc {
            0x82ED72E8 => {
    //   block [0x82ED72E8..0x82ED72EC)
	// 82ED72E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED72F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED72F0 size=8
    let mut pc: u32 = 0x82ED72F0;
    'dispatch: loop {
        match pc {
            0x82ED72F0 => {
    //   block [0x82ED72F0..0x82ED72F8)
	// 82ED72F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED72F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED72F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED72F8 size=24
    let mut pc: u32 = 0x82ED72F8;
    'dispatch: loop {
        match pc {
            0x82ED72F8 => {
    //   block [0x82ED72F8..0x82ED7310)
	// 82ED72F8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED72FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7300: 392B73F4  addi r9, r11, 0x73f4
	ctx.r[9].s64 = ctx.r[11].s64 + 29684;
	// 82ED7304: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7308: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7310 size=12
    let mut pc: u32 = 0x82ED7310;
    'dispatch: loop {
        match pc {
            0x82ED7310 => {
    //   block [0x82ED7310..0x82ED731C)
	// 82ED7310: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7314: 386B73F4  addi r3, r11, 0x73f4
	ctx.r[3].s64 = ctx.r[11].s64 + 29684;
	// 82ED7318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7320 size=96
    let mut pc: u32 = 0x82ED7320;
    'dispatch: loop {
        match pc {
            0x82ED7320 => {
    //   block [0x82ED7320..0x82ED7380)
	// 82ED7320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED732C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7330: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7334: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED7338: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82ED733C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82ED7340: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED7344: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7348: 419A0020  beq cr6, 0x82ed7368
	if ctx.cr[6].eq {
	pc = 0x82ED7368; continue 'dispatch;
	}
	// 82ED734C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7350: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED7354: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82ED7358: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED735C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED7360: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED7364: 4BFC944D  bl 0x82ea07b0
	ctx.lr = 0x82ED7368;
	sub_82EA07B0(ctx, base);
	// 82ED7368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED736C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED7370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7378: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED737C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7380 size=4
    let mut pc: u32 = 0x82ED7380;
    'dispatch: loop {
        match pc {
            0x82ED7380 => {
    //   block [0x82ED7380..0x82ED7384)
	// 82ED7380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7388 size=4
    let mut pc: u32 = 0x82ED7388;
    'dispatch: loop {
        match pc {
            0x82ED7388 => {
    //   block [0x82ED7388..0x82ED738C)
	// 82ED7388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7390 size=4
    let mut pc: u32 = 0x82ED7390;
    'dispatch: loop {
        match pc {
            0x82ED7390 => {
    //   block [0x82ED7390..0x82ED7394)
	// 82ED7390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7398 size=20
    let mut pc: u32 = 0x82ED7398;
    'dispatch: loop {
        match pc {
            0x82ED7398 => {
    //   block [0x82ED7398..0x82ED73AC)
	// 82ED7398: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED739C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED73A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED73A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED73A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED73B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED73B0 size=4
    let mut pc: u32 = 0x82ED73B0;
    'dispatch: loop {
        match pc {
            0x82ED73B0 => {
    //   block [0x82ED73B0..0x82ED73B4)
	// 82ED73B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED73B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED73B8 size=8
    let mut pc: u32 = 0x82ED73B8;
    'dispatch: loop {
        match pc {
            0x82ED73B8 => {
    //   block [0x82ED73B8..0x82ED73C0)
	// 82ED73B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED73BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED73C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED73C0 size=24
    let mut pc: u32 = 0x82ED73C0;
    'dispatch: loop {
        match pc {
            0x82ED73C0 => {
    //   block [0x82ED73C0..0x82ED73D8)
	// 82ED73C0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED73C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED73C8: 392B7614  addi r9, r11, 0x7614
	ctx.r[9].s64 = ctx.r[11].s64 + 30228;
	// 82ED73CC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED73D0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED73D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED73D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED73D8 size=12
    let mut pc: u32 = 0x82ED73D8;
    'dispatch: loop {
        match pc {
            0x82ED73D8 => {
    //   block [0x82ED73D8..0x82ED73E4)
	// 82ED73D8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED73DC: 386B7614  addi r3, r11, 0x7614
	ctx.r[3].s64 = ctx.r[11].s64 + 30228;
	// 82ED73E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED73E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED73E8 size=100
    let mut pc: u32 = 0x82ED73E8;
    'dispatch: loop {
        match pc {
            0x82ED73E8 => {
    //   block [0x82ED73E8..0x82ED744C)
	// 82ED73E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED73EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED73F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED73F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED73F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED73FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7400: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED7404: 4801900D  bl 0x82ef0410
	ctx.lr = 0x82ED7408;
	sub_82EF0410(ctx, base);
	// 82ED7408: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED740C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7410: 419A0020  beq cr6, 0x82ed7430
	if ctx.cr[6].eq {
	pc = 0x82ED7430; continue 'dispatch;
	}
	// 82ED7414: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7418: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED741C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82ED7420: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED7424: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED7428: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED742C: 4BFC9385  bl 0x82ea07b0
	ctx.lr = 0x82ED7430;
	sub_82EA07B0(ctx, base);
	// 82ED7430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED7434: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED7438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED743C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7440: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED7444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7450 size=4
    let mut pc: u32 = 0x82ED7450;
    'dispatch: loop {
        match pc {
            0x82ED7450 => {
    //   block [0x82ED7450..0x82ED7454)
	// 82ED7450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7458 size=12
    let mut pc: u32 = 0x82ED7458;
    'dispatch: loop {
        match pc {
            0x82ED7458 => {
    //   block [0x82ED7458..0x82ED7464)
	// 82ED7458: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED745C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7460: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7464(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7464 size=8
    let mut pc: u32 = 0x82ED7464;
    'dispatch: loop {
        match pc {
            0x82ED7464 => {
    //   block [0x82ED7464..0x82ED746C)
	// 82ED7464: 4801965C  b 0x82ef0ac0
	sub_82EF0AC0(ctx, base);
	return;
	// 82ED7468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7470 size=20
    let mut pc: u32 = 0x82ED7470;
    'dispatch: loop {
        match pc {
            0x82ED7470 => {
    //   block [0x82ED7470..0x82ED7484)
	// 82ED7470: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7474: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7478: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED747C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7480: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7488 size=44
    let mut pc: u32 = 0x82ED7488;
    'dispatch: loop {
        match pc {
            0x82ED7488 => {
    //   block [0x82ED7488..0x82ED74B4)
	// 82ED7488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED748C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7490: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7494: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED749C: 48019625  bl 0x82ef0ac0
	ctx.lr = 0x82ED74A0;
	sub_82EF0AC0(ctx, base);
	// 82ED74A0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED74A4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82ED74A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED74AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED74B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED74B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED74B8 size=4
    let mut pc: u32 = 0x82ED74B8;
    'dispatch: loop {
        match pc {
            0x82ED74B8 => {
    //   block [0x82ED74B8..0x82ED74BC)
	// 82ED74B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED74C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED74C0 size=20
    let mut pc: u32 = 0x82ED74C0;
    'dispatch: loop {
        match pc {
            0x82ED74C0 => {
    //   block [0x82ED74C0..0x82ED74D4)
	// 82ED74C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED74C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED74C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED74CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED74D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED74D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED74D8 size=4
    let mut pc: u32 = 0x82ED74D8;
    'dispatch: loop {
        match pc {
            0x82ED74D8 => {
    //   block [0x82ED74D8..0x82ED74DC)
	// 82ED74D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED74E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED74E0 size=8
    let mut pc: u32 = 0x82ED74E0;
    'dispatch: loop {
        match pc {
            0x82ED74E0 => {
    //   block [0x82ED74E0..0x82ED74E8)
	// 82ED74E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED74E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED74E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED74E8 size=24
    let mut pc: u32 = 0x82ED74E8;
    'dispatch: loop {
        match pc {
            0x82ED74E8 => {
    //   block [0x82ED74E8..0x82ED7500)
	// 82ED74E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED74EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED74F0: 392B7904  addi r9, r11, 0x7904
	ctx.r[9].s64 = ctx.r[11].s64 + 30980;
	// 82ED74F4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED74F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED74FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


