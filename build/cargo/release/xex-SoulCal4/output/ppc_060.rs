pub fn sub_8246D960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D960 size=88
    let mut pc: u32 = 0x8246D960;
    'dispatch: loop {
        match pc {
            0x8246D960 => {
    //   block [0x8246D960..0x8246D9B8)
	// 8246D960: 394079B1  li r10, 0x79b1
	ctx.r[10].s64 = 31153;
	// 8246D964: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D968: 788BE102  rldicl r11, r4, 0x3c, 4
	ctx.r[11].u64 = ctx.r[4].u64 & 0x000000000000000Fu64;
	// 8246D96C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D970: 654A9E37  oris r10, r10, 0x9e37
	ctx.r[10].u64 = ctx.r[10].u64 | 2654404608;
	// 8246D974: 7CE807B4  extsw r8, r7
	ctx.r[8].s64 = ctx.r[7].s32 as i64;
	// 8246D978: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8246D97C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8246D980: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D984: 7D4A482A  ldx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 8246D988: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 8246D98C: 419A0024  beq cr6, 0x8246d9b0
	if ctx.cr[6].eq {
	pc = 0x8246D9B0; continue 'dispatch;
	}
	// 8246D990: 7F2A2040  cmpld cr6, r10, r4
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[4].u64, &mut ctx.xer);
	// 8246D994: 419A0024  beq cr6, 0x8246d9b8
	if ctx.cr[6].eq {
		sub_8246D9B8(ctx, base);
		return;
	}
	// 8246D998: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246D99C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8246D9A0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246D9A4: 7D4A482A  ldx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 8246D9A8: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 8246D9AC: 409AFFE4  bne cr6, 0x8246d990
	if !ctx.cr[6].eq {
	pc = 0x8246D990; continue 'dispatch;
	}
	// 8246D9B0: 38670001  addi r3, r7, 1
	ctx.r[3].s64 = ctx.r[7].s64 + 1;
	// 8246D9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D9B8 size=8
    let mut pc: u32 = 0x8246D9B8;
    'dispatch: loop {
        match pc {
            0x8246D9B8 => {
    //   block [0x8246D9B8..0x8246D9C0)
	// 8246D9B8: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246D9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246D9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246D9C0 size=332
    let mut pc: u32 = 0x8246D9C0;
    'dispatch: loop {
        match pc {
            0x8246D9C0 => {
    //   block [0x8246D9C0..0x8246DB0C)
	// 8246D9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246D9C4: 480C76F9  bl 0x825350bc
	ctx.lr = 0x8246D9C8;
	sub_82535080(ctx, base);
	// 8246D9C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246D9CC: 7C8507B4  extsw r5, r4
	ctx.r[5].s64 = ctx.r[4].s32 as i64;
	// 8246D9D0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D9D4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8246D9D8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246D9DC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246D9E0: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246D9E4: 7FCB512A  stdx r30, r11, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u64) };
	// 8246D9E8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246D9EC: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246D9F0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246D9F4: 7D4B2A14  add r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8246D9F8: 7D495838  and r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8246D9FC: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246DA00: 7D4A402A  ldx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) };
	// 8246DA04: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 8246DA08: 419A0020  beq cr6, 0x8246da28
	if ctx.cr[6].eq {
	pc = 0x8246DA28; continue 'dispatch;
	}
	// 8246DA0C: 550A003E  slwi r10, r8, 0
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246DA10: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8246DA14: 7D295838  and r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 8246DA18: 55271838  slwi r7, r9, 3
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246DA1C: 7CE7502A  ldx r7, r7, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	// 8246DA20: 2F27FFFF  cmpdi cr6, r7, -1
	ctx.cr[6].compare_i64(ctx.r[7].s64, -1, &mut ctx.xer);
	// 8246DA24: 409AFFEC  bne cr6, 0x8246da10
	if !ctx.cr[6].eq {
	pc = 0x8246DA10; continue 'dispatch;
	}
	// 8246DA28: 39450001  addi r10, r5, 1
	ctx.r[10].s64 = ctx.r[5].s64 + 1;
	// 8246DA2C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8246DA30: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8246DA34: 7D3F5838  and r31, r9, r11
	ctx.r[31].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 8246DA38: 5546003E  slwi r6, r10, 0
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8246DA3C: 54C71838  slwi r7, r6, 3
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246DA40: 7D28382A  ldx r9, r8, r7
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	// 8246DA44: 2F29FFFF  cmpdi cr6, r9, -1
	ctx.cr[6].compare_i64(ctx.r[9].s64, -1, &mut ctx.xer);
	// 8246DA48: 419A00C0  beq cr6, 0x8246db08
	if ctx.cr[6].eq {
	pc = 0x8246DB08; continue 'dispatch;
	}
	// 8246DA4C: 392079B1  li r9, 0x79b1
	ctx.r[9].s64 = 31153;
	// 8246DA50: 65249E37  oris r4, r9, 0x9e37
	ctx.r[4].u64 = ctx.r[9].u64 | 2654404608;
	// 8246DA54: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DA58: 7F2AF840  cmpld cr6, r10, r31
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[31].u64, &mut ctx.xer);
	// 8246DA5C: 7D09382A  ldx r8, r9, r7
	ctx.r[8].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) };
	// 8246DA60: 791DE102  rldicl r29, r8, 0x3c, 4
	ctx.r[29].u64 = ctx.r[8].u64 & 0x000000000000000Fu64;
	// 8246DA64: 7FBD21D2  mulld r29, r29, r4
	ctx.r[29].s64 = ctx.r[29].s64 * ctx.r[4].s64;
	// 8246DA68: 7FAB5838  and r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 & ctx.r[11].u64;
	// 8246DA6C: 4198000C  blt cr6, 0x8246da78
	if ctx.cr[6].lt {
	pc = 0x8246DA78; continue 'dispatch;
	}
	// 8246DA70: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 8246DA74: 4199006C  bgt cr6, 0x8246dae0
	if ctx.cr[6].gt {
	pc = 0x8246DAE0; continue 'dispatch;
	}
	// 8246DA78: 7F2A2840  cmpld cr6, r10, r5
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[5].u64, &mut ctx.xer);
	// 8246DA7C: 40980014  bge cr6, 0x8246da90
	if !ctx.cr[6].lt {
	pc = 0x8246DA90; continue 'dispatch;
	}
	// 8246DA80: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 8246DA84: 4199005C  bgt cr6, 0x8246dae0
	if ctx.cr[6].gt {
	pc = 0x8246DAE0; continue 'dispatch;
	}
	// 8246DA88: 7F2B5040  cmpld cr6, r11, r10
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[10].u64, &mut ctx.xer);
	// 8246DA8C: 40990054  ble cr6, 0x8246dae0
	if !ctx.cr[6].gt {
	pc = 0x8246DAE0; continue 'dispatch;
	}
	// 8246DA90: 7F2B2840  cmpld cr6, r11, r5
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[5].u64, &mut ctx.xer);
	// 8246DA94: 4099000C  ble cr6, 0x8246daa0
	if !ctx.cr[6].gt {
	pc = 0x8246DAA0; continue 'dispatch;
	}
	// 8246DA98: 7F2BF840  cmpld cr6, r11, r31
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[31].u64, &mut ctx.xer);
	// 8246DA9C: 41980044  blt cr6, 0x8246dae0
	if ctx.cr[6].lt {
	pc = 0x8246DAE0; continue 'dispatch;
	}
	// 8246DAA0: 54AB003E  slwi r11, r5, 0
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DAA4: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 8246DAA8: 557D1838  slwi r29, r11, 3
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8246DAAC: 7D1D492A  stdx r8, r29, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u64) };
	// 8246DAB0: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DAB4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DAB8: 7CC93214  add r6, r9, r6
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 8246DABC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8246DAC0: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 8246DAC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246DAC8: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8246DACC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DAD0: 7D29402A  ldx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	// 8246DAD4: 7D2B412A  stdx r9, r11, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 8246DAD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DADC: 7FC7592A  stdx r30, r7, r11
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u64) };
	// 8246DAE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DAE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246DAE8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DAEC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8246DAF0: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8246DAF4: 5546003E  slwi r6, r10, 0
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8246DAF8: 54C71838  slwi r7, r6, 3
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8246DAFC: 7D27482A  ldx r9, r7, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) };
	// 8246DB00: 2F29FFFF  cmpdi cr6, r9, -1
	ctx.cr[6].compare_i64(ctx.r[9].s64, -1, &mut ctx.xer);
	// 8246DB04: 409AFF50  bne cr6, 0x8246da54
	if !ctx.cr[6].eq {
	pc = 0x8246DA54; continue 'dispatch;
	}
	// 8246DB08: 480C7604  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246DB10 size=232
    let mut pc: u32 = 0x8246DB10;
    'dispatch: loop {
        match pc {
            0x8246DB10 => {
    //   block [0x8246DB10..0x8246DBF8)
	// 8246DB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246DB14: 480C7591  bl 0x825350a4
	ctx.lr = 0x8246DB18;
	sub_82535080(ctx, base);
	// 8246DB18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246DB1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246DB20: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DB24: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 8246DB28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246DB2C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246DB30: 57C42036  slwi r4, r30, 4
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246DB34: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246DB38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DB3C: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DB40: 55570000  rlwinm r23, r10, 0, 0, 0
	ctx.r[23].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8246DB44: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246DB48: 3B6B0001  addi r27, r11, 1
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	// 8246DB4C: 4BFF64ED  bl 0x82464038
	ctx.lr = 0x8246DB50;
	sub_82464038(ctx, base);
	// 8246DB50: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246DB54: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8246DB58: 40990028  ble cr6, 0x8246db80
	if !ctx.cr[6].gt {
	pc = 0x8246DB80; continue 'dispatch;
	}
	// 8246DB5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246DB60: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8246DB64: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8246DB68: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DB6C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246DB70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246DB74: 7D2A412A  stdx r9, r10, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u64) };
	// 8246DB78: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8246DB7C: 409AFFEC  bne cr6, 0x8246db68
	if !ctx.cr[6].eq {
	pc = 0x8246DB68; continue 'dispatch;
	}
	// 8246DB80: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 8246DB84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246DB88: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8246DB8C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246DB90: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8246DB94: 40990040  ble cr6, 0x8246dbd4
	if !ctx.cr[6].gt {
	pc = 0x8246DBD4; continue 'dispatch;
	}
	// 8246DB98: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DB9C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8246DBA0: 7FCBD214  add r30, r11, r26
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8246DBA4: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 8246DBA8: E89D0000  ld r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 8246DBAC: 2F24FFFF  cmpdi cr6, r4, -1
	ctx.cr[6].compare_i64(ctx.r[4].s64, -1, &mut ctx.xer);
	// 8246DBB0: 419A0010  beq cr6, 0x8246dbc0
	if ctx.cr[6].eq {
	pc = 0x8246DBC0; continue 'dispatch;
	}
	// 8246DBB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246DBB8: E8BE0000  ld r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8246DBBC: 4BFFFCC5  bl 0x8246d880
	ctx.lr = 0x8246DBC0;
	sub_8246D880(ctx, base);
	// 8246DBC0: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 8246DBC4: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8246DBC8: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 8246DBCC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8246DBD0: 409AFFD8  bne cr6, 0x8246dba8
	if !ctx.cr[6].eq {
	pc = 0x8246DBA8; continue 'dispatch;
	}
	// 8246DBD4: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8246DBD8: 409A0018  bne cr6, 0x8246dbf0
	if !ctx.cr[6].eq {
	pc = 0x8246DBF0; continue 'dispatch;
	}
	// 8246DBDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246DBE0: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246DBE4: 57652036  slwi r5, r27, 4
	ctx.r[5].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8246DBE8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8246DBEC: 4BFF64CD  bl 0x824640b8
	ctx.lr = 0x8246DBF0;
	sub_824640B8(ctx, base);
	// 8246DBF0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246DBF4: 480C7500  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246DBF8 size=116
    let mut pc: u32 = 0x8246DBF8;
    'dispatch: loop {
        match pc {
            0x8246DBF8 => {
    //   block [0x8246DBF8..0x8246DC6C)
	// 8246DBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246DBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246DC00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246DC04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246DC08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246DC0C: 4BFFF8CD  bl 0x8246d4d8
	ctx.lr = 0x8246DC10;
	sub_8246D4D8(ctx, base);
	// 8246DC10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DC14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8246DC18: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DC1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246DC20: 40990008  ble cr6, 0x8246dc28
	if !ctx.cr[6].gt {
	pc = 0x8246DC28; continue 'dispatch;
	}
	// 8246DC24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246DC28: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246DC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246DC30: 419A0024  beq cr6, 0x8246dc54
	if ctx.cr[6].eq {
	pc = 0x8246DC54; continue 'dispatch;
	}
	// 8246DC34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246DC38: 4BFFF9E9  bl 0x8246d620
	ctx.lr = 0x8246DC3C;
	sub_8246D620(ctx, base);
	// 8246DC3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246DC40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246DC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246DC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246DC4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246DC50: 4E800020  blr
	return;
	// 8246DC54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246DC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246DC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246DC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246DC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246DC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246DC70 size=40
    let mut pc: u32 = 0x8246DC70;
    'dispatch: loop {
        match pc {
            0x8246DC70 => {
    //   block [0x8246DC70..0x8246DC98)
	// 8246DC70: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246DC74: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8246DC78: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 8246DC7C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8246DC80: 40990010  ble cr6, 0x8246dc90
	if !ctx.cr[6].gt {
	pc = 0x8246DC90; continue 'dispatch;
	}
	// 8246DC84: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DC88: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246DC8C: 4198FFF8  blt cr6, 0x8246dc84
	if ctx.cr[6].lt {
	pc = 0x8246DC84; continue 'dispatch;
	}
	// 8246DC90: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DC94: 4BFFFACC  b 0x8246d760
	sub_8246D760(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246DC98 size=412
    let mut pc: u32 = 0x8246DC98;
    'dispatch: loop {
        match pc {
            0x8246DC98 => {
    //   block [0x8246DC98..0x8246DE34)
	// 8246DC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246DC9C: 480C7419  bl 0x825350b4
	ctx.lr = 0x8246DCA0;
	sub_82535080(ctx, base);
	// 8246DCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246DCA4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246DCA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246DCAC: 3B6B8728  addi r27, r11, -0x78d8
	ctx.r[27].s64 = ctx.r[11].s64 + -30936;
	// 8246DCB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246DCB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246DCB8: 4BFFC569  bl 0x8246a220
	ctx.lr = 0x8246DCBC;
	sub_8246A220(ctx, base);
	// 8246DCBC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8246DCC0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8246DCC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246DCC8: 4BFFC339  bl 0x8246a000
	ctx.lr = 0x8246DCCC;
	sub_8246A000(ctx, base);
	// 8246DCCC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246DCD0: 419A0020  beq cr6, 0x8246dcf0
	if ctx.cr[6].eq {
	pc = 0x8246DCF0; continue 'dispatch;
	}
	// 8246DCD4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246DCD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246DCDC: 388B8724  addi r4, r11, -0x78dc
	ctx.r[4].s64 = ctx.r[11].s64 + -30940;
	// 8246DCE0: 4BFFC5A1  bl 0x8246a280
	ctx.lr = 0x8246DCE4;
	sub_8246A280(ctx, base);
	// 8246DCE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246DCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246DCEC: 419A0008  beq cr6, 0x8246dcf4
	if ctx.cr[6].eq {
	pc = 0x8246DCF4; continue 'dispatch;
	}
	// 8246DCF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246DCF4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246DCF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246DCFC: 409A0094  bne cr6, 0x8246dd90
	if !ctx.cr[6].eq {
	pc = 0x8246DD90; continue 'dispatch;
	}
	// 8246DD00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8246DD04: 7D7EE8AE  lbzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8246DD08: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246DD0C: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 8246DD10: 419A000C  beq cr6, 0x8246dd1c
	if ctx.cr[6].eq {
	pc = 0x8246DD1C; continue 'dispatch;
	}
	// 8246DD14: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 8246DD18: 409A000C  bne cr6, 0x8246dd24
	if !ctx.cr[6].eq {
	pc = 0x8246DD24; continue 'dispatch;
	}
	// 8246DD1C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8246DD20: 4BFFFFE4  b 0x8246dd04
	pc = 0x8246DD04; continue 'dispatch;
	// 8246DD24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246DD28: 4BFFC4F9  bl 0x8246a220
	ctx.lr = 0x8246DD2C;
	sub_8246A220(ctx, base);
	// 8246DD2C: 7FBEEA14  add r29, r30, r29
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 8246DD30: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246DD34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246DD38: 4BFFC4E9  bl 0x8246a220
	ctx.lr = 0x8246DD3C;
	sub_8246A220(ctx, base);
	// 8246DD3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DD40: 7D43E214  add r10, r3, r28
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 8246DD44: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DD48: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 8246DD4C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246DD50: 40980024  bge cr6, 0x8246dd74
	if !ctx.cr[6].lt {
	pc = 0x8246DD74; continue 'dispatch;
	}
	// 8246DD54: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DD58: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DD5C: 41980008  blt cr6, 0x8246dd64
	if ctx.cr[6].lt {
	pc = 0x8246DD64; continue 'dispatch;
	}
	// 8246DD60: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8246DD64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DD68: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DD6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246DD70: 48000559  bl 0x8246e2c8
	ctx.lr = 0x8246DD74;
	sub_8246E2C8(ctx, base);
	// 8246DD74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8246DD78: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DD7C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8246DD80: 4BFFC471  bl 0x8246a1f0
	ctx.lr = 0x8246DD84;
	sub_8246A1F0(ctx, base);
	// 8246DD84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DD88: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8246DD8C: 48000048  b 0x8246ddd4
	pc = 0x8246DDD4; continue 'dispatch;
	// 8246DD90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246DD94: 4BFFC48D  bl 0x8246a220
	ctx.lr = 0x8246DD98;
	sub_8246A220(ctx, base);
	// 8246DD98: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246DD9C: 3BC30001  addi r30, r3, 1
	ctx.r[30].s64 = ctx.r[3].s64 + 1;
	// 8246DDA0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DDA4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246DDA8: 40980024  bge cr6, 0x8246ddcc
	if !ctx.cr[6].lt {
	pc = 0x8246DDCC; continue 'dispatch;
	}
	// 8246DDAC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DDB0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DDB4: 41980008  blt cr6, 0x8246ddbc
	if ctx.cr[6].lt {
	pc = 0x8246DDBC; continue 'dispatch;
	}
	// 8246DDB8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8246DDBC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DDC0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DDC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246DDC8: 48000501  bl 0x8246e2c8
	ctx.lr = 0x8246DDCC;
	sub_8246E2C8(ctx, base);
	// 8246DDCC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DDD0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8246DDD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246DDD8: 4BFFC419  bl 0x8246a1f0
	ctx.lr = 0x8246DDDC;
	sub_8246A1F0(ctx, base);
	// 8246DDDC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246DDE0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DDE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246DDE8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8246DDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246DDF0: 992AFFFF  stb r9, -1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	// 8246DDF4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246DDF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246DDFC: 4099002C  ble cr6, 0x8246de28
	if !ctx.cr[6].gt {
	pc = 0x8246DE28; continue 'dispatch;
	}
	// 8246DE00: 3920005C  li r9, 0x5c
	ctx.r[9].s64 = 92;
	// 8246DE04: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DE08: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246DE0C: 2B08002F  cmplwi cr6, r8, 0x2f
	ctx.cr[6].compare_u32(ctx.r[8].u32, 47 as u32, &mut ctx.xer);
	// 8246DE10: 409A0008  bne cr6, 0x8246de18
	if !ctx.cr[6].eq {
	pc = 0x8246DE18; continue 'dispatch;
	}
	// 8246DE14: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8246DE18: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246DE1C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246DE20: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246DE24: 4198FFE0  blt cr6, 0x8246de04
	if ctx.cr[6].lt {
	pc = 0x8246DE04; continue 'dispatch;
	}
	// 8246DE28: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246DE30: 480C72D4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246DE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246DE38 size=1168
    let mut pc: u32 = 0x8246DE38;
    'dispatch: loop {
        match pc {
            0x8246DE38 => {
    //   block [0x8246DE38..0x8246E2C8)
	// 8246DE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246DE3C: 480C726D  bl 0x825350a8
	ctx.lr = 0x8246DE40;
	sub_82535080(ctx, base);
	// 8246DE40: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246DE44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8246DE48: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 8246DE4C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246DE50: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8246DE54: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 8246DE58: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8246DE5C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8246DE60: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8246DE64: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8246DE68: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 8246DE6C: 419A0054  beq cr6, 0x8246dec0
	if ctx.cr[6].eq {
	pc = 0x8246DEC0; continue 'dispatch;
	}
	// 8246DE70: 4BFFC3B1  bl 0x8246a220
	ctx.lr = 0x8246DE74;
	sub_8246A220(ctx, base);
	// 8246DE74: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246DE78: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246DE7C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DE80: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246DE84: 40980024  bge cr6, 0x8246dea8
	if !ctx.cr[6].lt {
	pc = 0x8246DEA8; continue 'dispatch;
	}
	// 8246DE88: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DE8C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DE90: 41980008  blt cr6, 0x8246de98
	if ctx.cr[6].lt {
	pc = 0x8246DE98; continue 'dispatch;
	}
	// 8246DE94: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246DE98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DE9C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DEA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8246DEA4: 48000425  bl 0x8246e2c8
	ctx.lr = 0x8246DEA8;
	sub_8246E2C8(ctx, base);
	// 8246DEA8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246DEAC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246DEB0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8246DEB4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8246DEB8: 4BFFC471  bl 0x8246a328
	ctx.lr = 0x8246DEBC;
	sub_8246A328(ctx, base);
	// 8246DEBC: 48000020  b 0x8246dedc
	pc = 0x8246DEDC; continue 'dispatch;
	// 8246DEC0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DEC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246DEC8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8246DECC: 480003FD  bl 0x8246e2c8
	ctx.lr = 0x8246DED0;
	sub_8246E2C8(ctx, base);
	// 8246DED0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246DED4: 9301005C  stw r24, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 8246DED8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8246DEDC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8246DEE0: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 8246DEE4: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 8246DEE8: 3B8BDEB0  addi r28, r11, -0x2150
	ctx.r[28].s64 = ctx.r[11].s64 + -8528;
	// 8246DEEC: 93610080  stw r27, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u32 ) };
	// 8246DEF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246DEF4: 4BFFC32D  bl 0x8246a220
	ctx.lr = 0x8246DEF8;
	sub_8246A220(ctx, base);
	// 8246DEF8: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8246DEFC: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246DF00: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DF04: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246DF08: 40980024  bge cr6, 0x8246df2c
	if !ctx.cr[6].lt {
	pc = 0x8246DF2C; continue 'dispatch;
	}
	// 8246DF0C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DF10: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DF14: 41980008  blt cr6, 0x8246df1c
	if ctx.cr[6].lt {
	pc = 0x8246DF1C; continue 'dispatch;
	}
	// 8246DF18: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246DF1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DF20: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DF24: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8246DF28: 480003A1  bl 0x8246e2c8
	ctx.lr = 0x8246DF2C;
	sub_8246E2C8(ctx, base);
	// 8246DF2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246DF30: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8246DF34: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246DF38: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 8246DF3C: 4BFFC3ED  bl 0x8246a328
	ctx.lr = 0x8246DF40;
	sub_8246A328(ctx, base);
	// 8246DF40: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246DF44: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 8246DF48: 3BCB8734  addi r30, r11, -0x78cc
	ctx.r[30].s64 = ctx.r[11].s64 + -30924;
	// 8246DF4C: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 8246DF50: 93610070  stw r27, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 8246DF54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246DF58: 4BFFC2C9  bl 0x8246a220
	ctx.lr = 0x8246DF5C;
	sub_8246A220(ctx, base);
	// 8246DF5C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8246DF60: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246DF64: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DF68: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246DF6C: 40980024  bge cr6, 0x8246df90
	if !ctx.cr[6].lt {
	pc = 0x8246DF90; continue 'dispatch;
	}
	// 8246DF70: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246DF74: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246DF78: 41980008  blt cr6, 0x8246df80
	if ctx.cr[6].lt {
	pc = 0x8246DF80; continue 'dispatch;
	}
	// 8246DF7C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246DF80: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246DF84: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246DF88: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8246DF8C: 4800033D  bl 0x8246e2c8
	ctx.lr = 0x8246DF90;
	sub_8246E2C8(ctx, base);
	// 8246DF90: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246DF94: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246DF98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246DF9C: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 8246DFA0: 4BFFC389  bl 0x8246a328
	ctx.lr = 0x8246DFA4;
	sub_8246A328(ctx, base);
	// 8246DFA4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8246DFA8: 38C10078  addi r6, r1, 0x78
	ctx.r[6].s64 = ctx.r[1].s64 + 120;
	// 8246DFAC: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8246DFB0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8246DFB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246DFB8: 4BFFD071  bl 0x8246b028
	ctx.lr = 0x8246DFBC;
	sub_8246B028(ctx, base);
	// 8246DFBC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8246DFC0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246DFC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246DFC8: 409A0020  bne cr6, 0x8246dfe8
	if !ctx.cr[6].eq {
	pc = 0x8246DFE8; continue 'dispatch;
	}
	// 8246DFCC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DFD0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246DFD4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246DFD8: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8246DFDC: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246DFE0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246DFE4: 4BFF60D5  bl 0x824640b8
	ctx.lr = 0x8246DFE8;
	sub_824640B8(ctx, base);
	// 8246DFE8: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8246DFEC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246DFF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246DFF4: 409A0020  bne cr6, 0x8246e014
	if !ctx.cr[6].eq {
	pc = 0x8246E014; continue 'dispatch;
	}
	// 8246DFF8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246DFFC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E000: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E004: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8246E008: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E00C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E010: 4BFF60A9  bl 0x824640b8
	ctx.lr = 0x8246E014;
	sub_824640B8(ctx, base);
	// 8246E014: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246E018: 93A10098  stw r29, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[29].u32 ) };
	// 8246E01C: 93A1009C  stw r29, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 8246E020: 936100A0  stw r27, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[27].u32 ) };
	// 8246E024: 4BFFC1FD  bl 0x8246a220
	ctx.lr = 0x8246E028;
	sub_8246A220(ctx, base);
	// 8246E028: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 8246E02C: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246E030: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E034: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246E038: 40980024  bge cr6, 0x8246e05c
	if !ctx.cr[6].lt {
	pc = 0x8246E05C; continue 'dispatch;
	}
	// 8246E03C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E040: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E044: 41980008  blt cr6, 0x8246e04c
	if ctx.cr[6].lt {
	pc = 0x8246E04C; continue 'dispatch;
	}
	// 8246E048: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246E04C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246E050: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246E054: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8246E058: 48000271  bl 0x8246e2c8
	ctx.lr = 0x8246E05C;
	sub_8246E2C8(ctx, base);
	// 8246E05C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246E060: 80610098  lwz r3, 0x98(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 8246E064: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246E068: 93E1009C  stw r31, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[31].u32 ) };
	// 8246E06C: 4BFFC2BD  bl 0x8246a328
	ctx.lr = 0x8246E070;
	sub_8246A328(ctx, base);
	// 8246E070: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246E074: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 8246E078: 3BCB8730  addi r30, r11, -0x78d0
	ctx.r[30].s64 = ctx.r[11].s64 + -30928;
	// 8246E07C: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 8246E080: 93610090  stw r27, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[27].u32 ) };
	// 8246E084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246E088: 4BFFC199  bl 0x8246a220
	ctx.lr = 0x8246E08C;
	sub_8246A220(ctx, base);
	// 8246E08C: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 8246E090: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246E094: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E098: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246E09C: 40980024  bge cr6, 0x8246e0c0
	if !ctx.cr[6].lt {
	pc = 0x8246E0C0; continue 'dispatch;
	}
	// 8246E0A0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E0A4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E0A8: 41980008  blt cr6, 0x8246e0b0
	if ctx.cr[6].lt {
	pc = 0x8246E0B0; continue 'dispatch;
	}
	// 8246E0AC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246E0B0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246E0B4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246E0B8: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8246E0BC: 4800020D  bl 0x8246e2c8
	ctx.lr = 0x8246E0C0;
	sub_8246E2C8(ctx, base);
	// 8246E0C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246E0C4: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8246E0C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246E0CC: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 8246E0D0: 4BFFC259  bl 0x8246a328
	ctx.lr = 0x8246E0D4;
	sub_8246A328(ctx, base);
	// 8246E0D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8246E0D8: 38C10098  addi r6, r1, 0x98
	ctx.r[6].s64 = ctx.r[1].s64 + 152;
	// 8246E0DC: 38A10088  addi r5, r1, 0x88
	ctx.r[5].s64 = ctx.r[1].s64 + 136;
	// 8246E0E0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8246E0E4: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 8246E0E8: 4BFFCF41  bl 0x8246b028
	ctx.lr = 0x8246E0EC;
	sub_8246B028(ctx, base);
	// 8246E0EC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E0F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E0F4: 409A0014  bne cr6, 0x8246e108
	if !ctx.cr[6].eq {
	pc = 0x8246E108; continue 'dispatch;
	}
	// 8246E0F8: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246E0FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E100: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8246E104: 419A0008  beq cr6, 0x8246e10c
	if ctx.cr[6].eq {
	pc = 0x8246E10C; continue 'dispatch;
	}
	// 8246E108: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8246E10C: 81410090  lwz r10, 0x90(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 8246E110: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8246E114: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E118: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8246E11C: 409A0020  bne cr6, 0x8246e13c
	if !ctx.cr[6].eq {
	pc = 0x8246E13C; continue 'dispatch;
	}
	// 8246E120: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E124: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E128: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E12C: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8246E130: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E134: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246E138: 4BFF5F81  bl 0x824640b8
	ctx.lr = 0x8246E13C;
	sub_824640B8(ctx, base);
	// 8246E13C: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 8246E140: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E144: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E148: 409A0020  bne cr6, 0x8246e168
	if !ctx.cr[6].eq {
	pc = 0x8246E168; continue 'dispatch;
	}
	// 8246E14C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E150: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E154: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E158: 80810098  lwz r4, 0x98(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 8246E15C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E160: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E164: 4BFF5F55  bl 0x824640b8
	ctx.lr = 0x8246E168;
	sub_824640B8(ctx, base);
	// 8246E168: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246E16C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8246E170: 38AB872C  addi r5, r11, -0x78d4
	ctx.r[5].s64 = ctx.r[11].s64 + -30932;
	// 8246E174: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 8246E178: 4BFFC7B9  bl 0x8246a930
	ctx.lr = 0x8246E17C;
	sub_8246A930(ctx, base);
	// 8246E17C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E184: 419A007C  beq cr6, 0x8246e200
	if ctx.cr[6].eq {
	pc = 0x8246E200; continue 'dispatch;
	}
	// 8246E188: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8246E18C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8246E190: 615EFFFF  ori r30, r10, 0xffff
	ctx.r[30].u64 = ctx.r[10].u64 | 65535;
	// 8246E194: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 8246E198: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8246E19C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246E1A0: 40980008  bge cr6, 0x8246e1a8
	if !ctx.cr[6].lt {
	pc = 0x8246E1A8; continue 'dispatch;
	}
	// 8246E1A4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8246E1A8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E1AC: 3BFE0001  addi r31, r30, 1
	ctx.r[31].s64 = ctx.r[30].s64 + 1;
	// 8246E1B0: 38830002  addi r4, r3, 2
	ctx.r[4].s64 = ctx.r[3].s64 + 2;
	// 8246E1B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246E1B8: 4BFFC179  bl 0x8246a330
	ctx.lr = 0x8246E1BC;
	sub_8246A330(ctx, base);
	// 8246E1BC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E1C0: 7FBE59AE  stbx r29, r30, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u8) };
	// 8246E1C4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246E1C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E1CC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246E1D0: 40980024  bge cr6, 0x8246e1f4
	if !ctx.cr[6].lt {
	pc = 0x8246E1F4; continue 'dispatch;
	}
	// 8246E1D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E1D8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E1DC: 41980008  blt cr6, 0x8246e1e4
	if ctx.cr[6].lt {
	pc = 0x8246E1E4; continue 'dispatch;
	}
	// 8246E1E0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246E1E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246E1E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246E1EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8246E1F0: 480000D9  bl 0x8246e2c8
	ctx.lr = 0x8246E1F4;
	sub_8246E2C8(ctx, base);
	// 8246E1F4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8246E1F8: 9B010050  stb r24, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u8 ) };
	// 8246E1FC: 48000008  b 0x8246e204
	pc = 0x8246E204; continue 'dispatch;
	// 8246E200: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8246E204: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246E208: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E20C: 419A0084  beq cr6, 0x8246e290
	if ctx.cr[6].eq {
	pc = 0x8246E290; continue 'dispatch;
	}
	// 8246E210: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E214: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E218: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246E21C: 40980024  bge cr6, 0x8246e240
	if !ctx.cr[6].lt {
	pc = 0x8246E240; continue 'dispatch;
	}
	// 8246E220: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E224: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E228: 41980008  blt cr6, 0x8246e230
	if ctx.cr[6].lt {
	pc = 0x8246E230; continue 'dispatch;
	}
	// 8246E22C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246E230: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246E234: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246E238: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8246E23C: 4800008D  bl 0x8246e2c8
	ctx.lr = 0x8246E240;
	sub_8246E2C8(ctx, base);
	// 8246E240: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246E244: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E248: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E24C: 93F90004  stw r31, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8246E250: 4BFFC0D9  bl 0x8246a328
	ctx.lr = 0x8246E254;
	sub_8246A328(ctx, base);
	// 8246E254: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246E258: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E25C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E260: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E264: 409A0020  bne cr6, 0x8246e284
	if !ctx.cr[6].eq {
	pc = 0x8246E284; continue 'dispatch;
	}
	// 8246E268: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E26C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E270: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E274: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E278: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E27C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E280: 4BFF5E39  bl 0x824640b8
	ctx.lr = 0x8246E284;
	sub_824640B8(ctx, base);
	// 8246E284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E288: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8246E28C: 480C6E6C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 8246E290: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246E294: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E298: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E29C: 409A0020  bne cr6, 0x8246e2bc
	if !ctx.cr[6].eq {
	pc = 0x8246E2BC; continue 'dispatch;
	}
	// 8246E2A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E2A4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E2A8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E2AC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8246E2B0: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E2B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E2B8: 4BFF5E01  bl 0x824640b8
	ctx.lr = 0x8246E2BC;
	sub_824640B8(ctx, base);
	// 8246E2BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8246E2C0: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8246E2C4: 480C6E34  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E2C8 size=136
    let mut pc: u32 = 0x8246E2C8;
    'dispatch: loop {
        match pc {
            0x8246E2C8 => {
    //   block [0x8246E2C8..0x8246E350)
	// 8246E2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E2CC: 480C6DE5  bl 0x825350b0
	ctx.lr = 0x8246E2D0;
	sub_82535080(ctx, base);
	// 8246E2D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E2D4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E2D8: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 8246E2DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246E2E0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8246E2E4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246E2E8: 7C9BF1D6  mullw r4, r27, r30
	ctx.r[4].s64 = (ctx.r[27].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8246E2EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E2F0: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246E2F4: 4BFF5D45  bl 0x82464038
	ctx.lr = 0x8246E2F8;
	sub_82464038(ctx, base);
	// 8246E2F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E2FC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246E300: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E304: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8246E308: 480C6849  bl 0x82534b50
	ctx.lr = 0x8246E30C;
	sub_82534B50(ctx, base);
	// 8246E30C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E310: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E314: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E318: 409A001C  bne cr6, 0x8246e334
	if !ctx.cr[6].eq {
	pc = 0x8246E334; continue 'dispatch;
	}
	// 8246E31C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E320: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E324: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E328: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246E32C: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8246E330: 4BFF5D89  bl 0x824640b8
	ctx.lr = 0x8246E334;
	sub_824640B8(ctx, base);
	// 8246E334: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E338: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8246E33C: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E340: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 8246E344: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246E348: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246E34C: 480C6DB4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E350 size=152
    let mut pc: u32 = 0x8246E350;
    'dispatch: loop {
        match pc {
            0x8246E350 => {
    //   block [0x8246E350..0x8246E3E8)
	// 8246E350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E354: 480C6D5D  bl 0x825350b0
	ctx.lr = 0x8246E358;
	sub_82535080(ctx, base);
	// 8246E358: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E35C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E360: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246E364: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E368: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246E36C: 557A083C  slwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 8246E370: 409A0008  bne cr6, 0x8246e378
	if !ctx.cr[6].eq {
	pc = 0x8246E378; continue 'dispatch;
	}
	// 8246E374: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8246E378: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E37C: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 8246E380: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246E384: 7C9AF1D6  mullw r4, r26, r30
	ctx.r[4].s64 = (ctx.r[26].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8246E388: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246E38C: 4BFF5CAD  bl 0x82464038
	ctx.lr = 0x8246E390;
	sub_82464038(ctx, base);
	// 8246E390: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E394: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E398: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8246E39C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8246E3A0: 480C67B1  bl 0x82534b50
	ctx.lr = 0x8246E3A4;
	sub_82534B50(ctx, base);
	// 8246E3A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E3A8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E3AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E3B0: 409A001C  bne cr6, 0x8246e3cc
	if !ctx.cr[6].eq {
	pc = 0x8246E3CC; continue 'dispatch;
	}
	// 8246E3B4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E3B8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E3BC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E3C0: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8246E3C4: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8246E3C8: 4BFF5CF1  bl 0x824640b8
	ctx.lr = 0x8246E3CC;
	sub_824640B8(ctx, base);
	// 8246E3CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E3D0: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8246E3D4: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E3D8: 7D6BD378  or r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[26].u64;
	// 8246E3DC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246E3E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246E3E4: 480C6D1C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E3E8 size=176
    let mut pc: u32 = 0x8246E3E8;
    'dispatch: loop {
        match pc {
            0x8246E3E8 => {
    //   block [0x8246E3E8..0x8246E498)
	// 8246E3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E3EC: 480C6CC9  bl 0x825350b4
	ctx.lr = 0x8246E3F0;
	sub_82535080(ctx, base);
	// 8246E3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E3F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E3F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246E3FC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8246E400: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8246E404: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8246E408: 419A001C  beq cr6, 0x8246e424
	if ctx.cr[6].eq {
	pc = 0x8246E424; continue 'dispatch;
	}
	// 8246E40C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E410: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8246E414: 40980010  bge cr6, 0x8246e424
	if !ctx.cr[6].lt {
	pc = 0x8246E424; continue 'dispatch;
	}
	// 8246E418: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246E41C: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 8246E420: 48000020  b 0x8246e440
	pc = 0x8246E440; continue 'dispatch;
	// 8246E424: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E428: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246E42C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8246E430: 7C9DE1D6  mullw r4, r29, r28
	ctx.r[4].s64 = (ctx.r[29].s32 as i64) * (ctx.r[28].s32 as i64);
	// 8246E434: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246E438: 4BFF5C01  bl 0x82464038
	ctx.lr = 0x8246E43C;
	sub_82464038(ctx, base);
	// 8246E43C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246E440: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246E448: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E44C: 7CABE9D6  mullw r5, r11, r29
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8246E450: 4BFFBED9  bl 0x8246a328
	ctx.lr = 0x8246E454;
	sub_8246A328(ctx, base);
	// 8246E454: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E458: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E45C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E460: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246E464: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246E468: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246E46C: 7CABE9D6  mullw r5, r11, r29
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8246E470: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246E474: 4BFF5C45  bl 0x824640b8
	ctx.lr = 0x8246E478;
	sub_824640B8(ctx, base);
	// 8246E478: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E47C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8246E480: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246E484: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 8246E488: 7D6BE378  or r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 8246E48C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246E490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E494: 480C6C70  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8246E498 size=208
    let mut pc: u32 = 0x8246E498;
    'dispatch: loop {
        match pc {
            0x8246E498 => {
    //   block [0x8246E498..0x8246E568)
	// 8246E498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E49C: 480C6C1D  bl 0x825350b8
	ctx.lr = 0x8246E4A0;
	sub_82535080(ctx, base);
	// 8246E4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E4A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E4A8: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8246E4AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E4B0: 40980018  bge cr6, 0x8246e4c8
	if !ctx.cr[6].lt {
	pc = 0x8246E4C8; continue 'dispatch;
	}
	// 8246E4B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246E4B8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E4BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E4C4: 480C6C44  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8246E4C8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E4CC: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8246E4D0: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E4D4: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246E4D8: 40990024  ble cr6, 0x8246e4fc
	if !ctx.cr[6].gt {
	pc = 0x8246E4FC; continue 'dispatch;
	}
	// 8246E4DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246E4E0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8246E4E4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E4E8: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8246E4EC: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8246E4F0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E4F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E4F8: 480C6C10  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8246E4FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E500: 40990060  ble cr6, 0x8246e560
	if !ctx.cr[6].gt {
	pc = 0x8246E560; continue 'dispatch;
	}
	// 8246E504: 7FCB4E70  srawi r11, r30, 9
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 9) as i64;
	// 8246E508: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8246E50C: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E510: 7F8BF050  subf r28, r11, r30
	ctx.r[28].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8246E514: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8246E518: 23BC0200  subfic r29, r28, 0x200
	ctx.xer.ca = ctx.r[28].u32 <= 512 as u32;
	ctx.r[29].s64 = (512 as i64) - ctx.r[28].s64;
	// 8246E51C: 409A0008  bne cr6, 0x8246e524
	if !ctx.cr[6].eq {
	pc = 0x8246E524; continue 'dispatch;
	}
	// 8246E520: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8246E524: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E528: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246E52C: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E530: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8246E534: 4BFFBDFD  bl 0x8246a330
	ctx.lr = 0x8246E538;
	sub_8246A330(ctx, base);
	// 8246E538: 7F8B0034  cntlzw r11, r28
	ctx.r[11].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 8246E53C: 7FCA4E70  srawi r10, r30, 9
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 9) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 9) as i64;
	// 8246E540: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 8246E544: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246E548: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8246E54C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8246E550: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E554: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246E558: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E55C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E564: 480C6BA4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E568 size=192
    let mut pc: u32 = 0x8246E568;
    'dispatch: loop {
        match pc {
            0x8246E568 => {
    //   block [0x8246E568..0x8246E628)
	// 8246E568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E56C: 480C6B51  bl 0x825350bc
	ctx.lr = 0x8246E570;
	sub_82535080(ctx, base);
	// 8246E570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E578: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246E57C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E580: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E584: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E58C: 4E800421  bctrl
	ctx.lr = 0x8246E590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E590: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E594: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E598: 409A0010  bne cr6, 0x8246e5a8
	if !ctx.cr[6].eq {
	pc = 0x8246E5A8; continue 'dispatch;
	}
	// 8246E59C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246E5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E5A4: 480C6B68  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8246E5A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E5AC: 4BFFFEED  bl 0x8246e498
	ctx.lr = 0x8246E5B0;
	sub_8246E498(ctx, base);
	// 8246E5B0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246E5B4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E5B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8246E5BC: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E5C0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246E5C4: 40990048  ble cr6, 0x8246e60c
	if !ctx.cr[6].gt {
	pc = 0x8246E60C; continue 'dispatch;
	}
	// 8246E5C8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E5CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246E5D0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E5D4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E5D8: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E5DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E5E0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E5E8: 4E800421  bctrl
	ctx.lr = 0x8246E5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E5EC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E5F0: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 8246E5F4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246E5F8: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246E5FC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E600: 409A0018  bne cr6, 0x8246e618
	if !ctx.cr[6].eq {
	pc = 0x8246E618; continue 'dispatch;
	}
	// 8246E604: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246E608: 4198FFC0  blt cr6, 0x8246e5c8
	if ctx.cr[6].lt {
	pc = 0x8246E5C8; continue 'dispatch;
	}
	// 8246E60C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246E610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E614: 480C6AF8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8246E618: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 8246E61C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246E620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E624: 480C6AE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E628 size=196
    let mut pc: u32 = 0x8246E628;
    'dispatch: loop {
        match pc {
            0x8246E628 => {
    //   block [0x8246E628..0x8246E6EC)
	// 8246E628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E62C: 480C6A89  bl 0x825350b4
	ctx.lr = 0x8246E630;
	sub_82535080(ctx, base);
	// 8246E630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E638: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8246E63C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246E640: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8246E644: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E648: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E64C: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246E650: 7F1BF000  cmpw cr6, r27, r30
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246E654: 4099005C  ble cr6, 0x8246e6b0
	if !ctx.cr[6].gt {
	pc = 0x8246E6B0; continue 'dispatch;
	}
	// 8246E658: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E65C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246E660: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246E664: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E668: 4BFFBCC1  bl 0x8246a328
	ctx.lr = 0x8246E66C;
	sub_8246A328(ctx, base);
	// 8246E66C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E670: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E674: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 8246E678: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246E67C: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8246E680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E684: 814A0030  lwz r10, 0x30(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8246E688: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E68C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8246E690: 4E800421  bctrl
	ctx.lr = 0x8246E694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E694: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246E698: 409A0048  bne cr6, 0x8246e6e0
	if !ctx.cr[6].eq {
	pc = 0x8246E6E0; continue 'dispatch;
	}
	// 8246E69C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E6A0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E6A4: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246E6A8: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246E6AC: 4199FFAC  bgt cr6, 0x8246e658
	if ctx.cr[6].gt {
	pc = 0x8246E658; continue 'dispatch;
	}
	// 8246E6B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E6B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8246E6B8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E6BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246E6C0: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246E6C4: 4BFFBC65  bl 0x8246a328
	ctx.lr = 0x8246E6C8;
	sub_8246A328(ctx, base);
	// 8246E6C8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E6CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246E6D0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8246E6D4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E6DC: 480C6A28  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8246E6E0: 7C7DD850  subf r3, r29, r27
	ctx.r[3].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 8246E6E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246E6E8: 480C6A1C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E6F0 size=132
    let mut pc: u32 = 0x8246E6F0;
    'dispatch: loop {
        match pc {
            0x8246E6F0 => {
    //   block [0x8246E6F0..0x8246E774)
	// 8246E6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E6F4: 480C69C9  bl 0x825350bc
	ctx.lr = 0x8246E6F8;
	sub_82535080(ctx, base);
	// 8246E6F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E6FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E700: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246E704: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8246E708: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E70C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E710: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E714: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E718: 40990038  ble cr6, 0x8246e750
	if !ctx.cr[6].gt {
	pc = 0x8246E750; continue 'dispatch;
	}
	// 8246E71C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E720: 7FCBF050  subf r30, r11, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8246E724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E728: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8246E72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E730: 4E800421  bctrl
	ctx.lr = 0x8246E734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E734: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246E738: 409A0030  bne cr6, 0x8246e768
	if !ctx.cr[6].eq {
	pc = 0x8246E768; continue 'dispatch;
	}
	// 8246E73C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E740: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E744: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E748: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246E74C: 4199FFD0  bgt cr6, 0x8246e71c
	if ctx.cr[6].gt {
	pc = 0x8246E71C; continue 'dispatch;
	}
	// 8246E750: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E754: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246E758: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246E75C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E760: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246E764: 480C69A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8246E768: 7C7EE850  subf r3, r30, r29
	ctx.r[3].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8246E76C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246E770: 480C699C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E778 size=108
    let mut pc: u32 = 0x8246E778;
    'dispatch: loop {
        match pc {
            0x8246E778 => {
    //   block [0x8246E778..0x8246E7E4)
	// 8246E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E784: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E788: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E790: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E794: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246E798: 409A002C  bne cr6, 0x8246e7c4
	if !ctx.cr[6].eq {
	pc = 0x8246E7C4; continue 'dispatch;
	}
	// 8246E79C: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E7A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246E7A4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E7A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246E7AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E7B0: 4E800421  bctrl
	ctx.lr = 0x8246E7B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E7B4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E7B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246E7BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246E7C0: 419A0008  beq cr6, 0x8246e7c8
	if ctx.cr[6].eq {
	pc = 0x8246E7C8; continue 'dispatch;
	}
	// 8246E7C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246E7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E7CC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246E7D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246E7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E7DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E7E8 size=24
    let mut pc: u32 = 0x8246E7E8;
    'dispatch: loop {
        match pc {
            0x8246E7E8 => {
    //   block [0x8246E7E8..0x8246E800)
	// 8246E7E8: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246E7EC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246E7F0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246E7F4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8246E7F8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246E7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E800 size=28
    let mut pc: u32 = 0x8246E800;
    'dispatch: loop {
        match pc {
            0x8246E800 => {
    //   block [0x8246E800..0x8246E81C)
	// 8246E800: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E804: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246E808: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 8246E80C: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246E810: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8246E814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246E818: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E81C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E81C size=8
    let mut pc: u32 = 0x8246E81C;
    'dispatch: loop {
        match pc {
            0x8246E81C => {
    //   block [0x8246E81C..0x8246E824)
	// 8246E81C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246E820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E828 size=28
    let mut pc: u32 = 0x8246E828;
    'dispatch: loop {
        match pc {
            0x8246E828 => {
    //   block [0x8246E828..0x8246E844)
	// 8246E828: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246E82C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8246E830: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246E834: 41980010  blt cr6, 0x8246e844
	if ctx.cr[6].lt {
		sub_8246E844(ctx, base);
		return;
	}
	// 8246E838: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246E83C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8246E840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E844(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E844 size=8
    let mut pc: u32 = 0x8246E844;
    'dispatch: loop {
        match pc {
            0x8246E844 => {
    //   block [0x8246E844..0x8246E84C)
	// 8246E844: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246E848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E850 size=64
    let mut pc: u32 = 0x8246E850;
    'dispatch: loop {
        match pc {
            0x8246E850 => {
    //   block [0x8246E850..0x8246E890)
	// 8246E850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E858: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E85C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E860: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E868: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E86C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8246E870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E874: 4E800421  bctrl
	ctx.lr = 0x8246E878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E87C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246E880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E890 size=48
    let mut pc: u32 = 0x8246E890;
    'dispatch: loop {
        match pc {
            0x8246E890 => {
    //   block [0x8246E890..0x8246E8C0)
	// 8246E890: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246E894: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8246E898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246E89C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E8A0: 912B001C  stw r9, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 8246E8A4: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 8246E8A8: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8246E8AC: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8246E8B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E8B4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8246E8B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E8BC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E8C0 size=108
    let mut pc: u32 = 0x8246E8C0;
    'dispatch: loop {
        match pc {
            0x8246E8C0 => {
    //   block [0x8246E8C0..0x8246E92C)
	// 8246E8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E8C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E8CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E8D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E8D4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246E8D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E8DC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8246E8E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E8E4: 4E800421  bctrl
	ctx.lr = 0x8246E8E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E8E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246E8EC: 41980028  blt cr6, 0x8246e914
	if ctx.cr[6].lt {
	pc = 0x8246E914; continue 'dispatch;
	}
	// 8246E8F0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246E8F4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246E8F8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246E8FC: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E90C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E910: 4E800020  blr
	return;
	// 8246E914: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246E918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246E91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E930 size=128
    let mut pc: u32 = 0x8246E930;
    'dispatch: loop {
        match pc {
            0x8246E930 => {
    //   block [0x8246E930..0x8246E9B0)
	// 8246E930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246E93C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E944: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E948: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246E94C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E950: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246E954: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246E958: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246E95C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246E960: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8246E964: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E968: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E970: 4E800421  bctrl
	ctx.lr = 0x8246E974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246E974: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246E978: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8246E97C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8246E980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246E984: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8246E988: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8246E98C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246E990: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246E994: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246E998: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246E99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246E9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246E9A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246E9A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246E9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246E9B0 size=32
    let mut pc: u32 = 0x8246E9B0;
    'dispatch: loop {
        match pc {
            0x8246E9B0 => {
    //   block [0x8246E9B0..0x8246E9D0)
	// 8246E9B0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E9B4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246E9B8: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E9BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246E9C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246E9C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246E9C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246E9CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246E9D0 size=112
    let mut pc: u32 = 0x8246E9D0;
    'dispatch: loop {
        match pc {
            0x8246E9D0 => {
    //   block [0x8246E9D0..0x8246EA40)
	// 8246E9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246E9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246E9D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246E9DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246E9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246E9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246E9E8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8246E9EC: 394A873C  addi r10, r10, -0x78c4
	ctx.r[10].s64 = ctx.r[10].s64 + -30916;
	// 8246E9F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246E9F4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8246E9F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8246E9FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246EA00: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8246EA04: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246EA08: 4BFFFF29  bl 0x8246e930
	ctx.lr = 0x8246EA0C;
	sub_8246E930(ctx, base);
	// 8246EA0C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EA14: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246EA18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246EA1C: 419A0010  beq cr6, 0x8246ea2c
	if ctx.cr[6].eq {
	pc = 0x8246EA2C; continue 'dispatch;
	}
	// 8246EA20: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246EA24: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246EA28: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246EA2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EA38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EA40 size=156
    let mut pc: u32 = 0x8246EA40;
    'dispatch: loop {
        match pc {
            0x8246EA40 => {
    //   block [0x8246EA40..0x8246EADC)
	// 8246EA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EA48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EA4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EA54: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246EA58: 396B873C  addi r11, r11, -0x78c4
	ctx.r[11].s64 = ctx.r[11].s64 + -30916;
	// 8246EA5C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EA60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246EA64: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246EA68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EA6C: 419A0030  beq cr6, 0x8246ea9c
	if ctx.cr[6].eq {
	pc = 0x8246EA9C; continue 'dispatch;
	}
	// 8246EA70: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246EA74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246EA78: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8246EA7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246EA80: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8246EA84: 409A0018  bne cr6, 0x8246ea9c
	if !ctx.cr[6].eq {
	pc = 0x8246EA9C; continue 'dispatch;
	}
	// 8246EA88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EA8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246EA90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EA94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EA98: 4E800421  bctrl
	ctx.lr = 0x8246EA9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246EA9C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EAA0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246EAA4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EAA8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246EAAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EAB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246EAB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EAB8: 4E800421  bctrl
	ctx.lr = 0x8246EABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246EABC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246EAC0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246EAC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246EAC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EAD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EAE0 size=100
    let mut pc: u32 = 0x8246EAE0;
    'dispatch: loop {
        match pc {
            0x8246EAE0 => {
    //   block [0x8246EAE0..0x8246EB44)
	// 8246EAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EAE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246EAEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EAF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EAF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EAF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246EAFC: 4BFFFF45  bl 0x8246ea40
	ctx.lr = 0x8246EB00;
	sub_8246EA40(ctx, base);
	// 8246EB00: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246EB04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EB08: 419A0020  beq cr6, 0x8246eb28
	if ctx.cr[6].eq {
	pc = 0x8246EB28; continue 'dispatch;
	}
	// 8246EB0C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EB10: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246EB14: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246EB18: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246EB1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246EB20: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246EB24: 4BFF5595  bl 0x824640b8
	ctx.lr = 0x8246EB28;
	sub_824640B8(ctx, base);
	// 8246EB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EB2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EB30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EB34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EB38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246EB3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EB48 size=72
    let mut pc: u32 = 0x8246EB48;
    'dispatch: loop {
        match pc {
            0x8246EB48 => {
    //   block [0x8246EB48..0x8246EB90)
	// 8246EB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EB50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EB54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EB58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EB5C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246EB60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246EB64: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EB68: 480C9989  bl 0x825384f0
	ctx.lr = 0x8246EB6C;
	sub_825384F0(ctx, base);
	// 8246EB6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246EB70: 4199000C  bgt cr6, 0x8246eb7c
	if ctx.cr[6].gt {
	pc = 0x8246EB7C; continue 'dispatch;
	}
	// 8246EB74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246EB78: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 8246EB7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EB88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246EB90 size=12
    let mut pc: u32 = 0x8246EB90;
    'dispatch: loop {
        match pc {
            0x8246EB90 => {
    //   block [0x8246EB90..0x8246EB9C)
	// 8246EB90: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EB94: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246EB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246EBA0 size=12
    let mut pc: u32 = 0x8246EBA0;
    'dispatch: loop {
        match pc {
            0x8246EBA0 => {
    //   block [0x8246EBA0..0x8246EBAC)
	// 8246EBA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246EBA4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246EBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246EBB0 size=12
    let mut pc: u32 = 0x8246EBB0;
    'dispatch: loop {
        match pc {
            0x8246EBB0 => {
    //   block [0x8246EBB0..0x8246EBBC)
	// 8246EBB0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246EBB4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246EBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EBC0 size=48
    let mut pc: u32 = 0x8246EBC0;
    'dispatch: loop {
        match pc {
            0x8246EBC0 => {
    //   block [0x8246EBC0..0x8246EBF0)
	// 8246EBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EBCC: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EBD0: 480C9A11  bl 0x825385e0
	ctx.lr = 0x8246EBD4;
	sub_825385E0(ctx, base);
	// 8246EBD4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8246EBD8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246EBDC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8246EBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246EBF0 size=8
    let mut pc: u32 = 0x8246EBF0;
    'dispatch: loop {
        match pc {
            0x8246EBF0 => {
    //   block [0x8246EBF0..0x8246EBF8)
	// 8246EBF0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EBF4: 480C9D2C  b 0x82538920
	sub_82538920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EBF8 size=116
    let mut pc: u32 = 0x8246EBF8;
    'dispatch: loop {
        match pc {
            0x8246EBF8 => {
    //   block [0x8246EBF8..0x8246EC6C)
	// 8246EBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EC00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EC04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EC08: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246EC0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EC10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246EC14: 396B8778  addi r11, r11, -0x7888
	ctx.r[11].s64 = ctx.r[11].s64 + -30856;
	// 8246EC18: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8246EC1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246EC20: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246EC24: 388A8770  addi r4, r10, -0x7890
	ctx.r[4].s64 = ctx.r[10].s64 + -30864;
	// 8246EC28: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246EC2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246EC30: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8246EC34: 993F0010  stb r9, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 8246EC38: 480C9F81  bl 0x82538bb8
	ctx.lr = 0x8246EC3C;
	sub_82538BB8(ctx, base);
	// 8246EC3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246EC40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EC44: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246EC48: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8246EC4C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8246EC50: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8246EC54: 995F0010  stb r10, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 8246EC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EC70 size=80
    let mut pc: u32 = 0x8246EC70;
    'dispatch: loop {
        match pc {
            0x8246EC70 => {
    //   block [0x8246EC70..0x8246ECC0)
	// 8246EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EC78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EC7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EC80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EC84: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246EC88: 396B8778  addi r11, r11, -0x7888
	ctx.r[11].s64 = ctx.r[11].s64 + -30856;
	// 8246EC8C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EC90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246EC94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246EC98: 419A0008  beq cr6, 0x8246eca0
	if ctx.cr[6].eq {
	pc = 0x8246ECA0; continue 'dispatch;
	}
	// 8246EC9C: 480C9FE5  bl 0x82538c80
	ctx.lr = 0x8246ECA0;
	sub_82538C80(ctx, base);
	// 8246ECA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246ECA4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246ECA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246ECAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246ECB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246ECB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246ECB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246ECBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246ECC0 size=72
    let mut pc: u32 = 0x8246ECC0;
    'dispatch: loop {
        match pc {
            0x8246ECC0 => {
    //   block [0x8246ECC0..0x8246ED08)
	// 8246ECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246ECC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246ECC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246ECCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246ECD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246ECD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ECD8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8246ECDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246ECE0: 4E800421  bctrl
	ctx.lr = 0x8246ECE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246ECE4: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8246ECE8: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8246ECEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246ECF0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246ECF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246ECF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246ECFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246ED00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246ED04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ED08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246ED08 size=24
    let mut pc: u32 = 0x8246ED08;
    'dispatch: loop {
        match pc {
            0x8246ED08 => {
    //   block [0x8246ED08..0x8246ED20)
	// 8246ED08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ED0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8246ED10: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246ED14: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8246ED18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246ED1C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ED20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246ED20 size=136
    let mut pc: u32 = 0x8246ED20;
    'dispatch: loop {
        match pc {
            0x8246ED20 => {
    //   block [0x8246ED20..0x8246EDA8)
	// 8246ED20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246ED24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246ED28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246ED2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246ED30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246ED34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246ED38: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246ED3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246ED40: 396B8778  addi r11, r11, -0x7888
	ctx.r[11].s64 = ctx.r[11].s64 + -30856;
	// 8246ED44: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246ED48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246ED4C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246ED50: 419A0008  beq cr6, 0x8246ed58
	if ctx.cr[6].eq {
	pc = 0x8246ED58; continue 'dispatch;
	}
	// 8246ED54: 480C9F2D  bl 0x82538c80
	ctx.lr = 0x8246ED58;
	sub_82538C80(ctx, base);
	// 8246ED58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246ED5C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246ED60: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246ED64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246ED68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246ED6C: 419A0020  beq cr6, 0x8246ed8c
	if ctx.cr[6].eq {
	pc = 0x8246ED8C; continue 'dispatch;
	}
	// 8246ED70: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ED74: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246ED78: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246ED7C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246ED80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246ED84: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246ED88: 4BFF5331  bl 0x824640b8
	ctx.lr = 0x8246ED8C;
	sub_824640B8(ctx, base);
	// 8246ED8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246ED90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246ED94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246ED98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246ED9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246EDA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EDA8 size=140
    let mut pc: u32 = 0x8246EDA8;
    'dispatch: loop {
        match pc {
            0x8246EDA8 => {
    //   block [0x8246EDA8..0x8246EE34)
	// 8246EDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EDAC: 480C6311  bl 0x825350bc
	ctx.lr = 0x8246EDB0;
	sub_82535080(ctx, base);
	// 8246EDB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EDB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246EDB8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EDBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EDC0: 419A0068  beq cr6, 0x8246ee28
	if ctx.cr[6].eq {
	pc = 0x8246EE28; continue 'dispatch;
	}
	// 8246EDC4: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EDC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246EDCC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246EDD0: 40990038  ble cr6, 0x8246ee08
	if !ctx.cr[6].gt {
	pc = 0x8246EE08; continue 'dispatch;
	}
	// 8246EDD4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EDD8: 7CBFE850  subf r5, r31, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 8246EDDC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EDE0: 7C9F5A14  add r4, r31, r11
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 8246EDE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EDE8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EDEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EDF0: 4E800421  bctrl
	ctx.lr = 0x8246EDF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246EDF4: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8246EDF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246EDFC: 419A0020  beq cr6, 0x8246ee1c
	if ctx.cr[6].eq {
	pc = 0x8246EE1C; continue 'dispatch;
	}
	// 8246EE00: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8246EE04: 4198FFD0  blt cr6, 0x8246edd4
	if ctx.cr[6].lt {
	pc = 0x8246EDD4; continue 'dispatch;
	}
	// 8246EE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246EE0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EE10: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246EE14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EE18: 480C62F4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8246EE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EE20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EE24: 480C62E8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8246EE28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246EE2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EE30: 480C62DC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EE38 size=184
    let mut pc: u32 = 0x8246EE38;
    'dispatch: loop {
        match pc {
            0x8246EE38 => {
    //   block [0x8246EE38..0x8246EEF0)
	// 8246EE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EE3C: 480C6275  bl 0x825350b0
	ctx.lr = 0x8246EE40;
	sub_82535080(ctx, base);
	// 8246EE40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EE44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EE48: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8246EE4C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246EE50: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8246EE54: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EE58: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246EE5C: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246EE60: 7F1AF000  cmpw cr6, r26, r30
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246EE64: 40990050  ble cr6, 0x8246eeb4
	if !ctx.cr[6].gt {
	pc = 0x8246EEB4; continue 'dispatch;
	}
	// 8246EE68: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EE6C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246EE70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246EE74: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246EE78: 4BFFB4B1  bl 0x8246a328
	ctx.lr = 0x8246EE7C;
	sub_8246A328(ctx, base);
	// 8246EE7C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EE80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246EE84: 7F6BF214  add r27, r11, r30
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246EE88: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 8246EE8C: 7FBEE850  subf r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8246EE90: 937F0010  stw r27, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8246EE94: 4BFFFF15  bl 0x8246eda8
	ctx.lr = 0x8246EE98;
	sub_8246EDA8(ctx, base);
	// 8246EE98: 7F03D800  cmpw cr6, r3, r27
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8246EE9C: 409A0048  bne cr6, 0x8246eee4
	if !ctx.cr[6].eq {
	pc = 0x8246EEE4; continue 'dispatch;
	}
	// 8246EEA0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EEA4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246EEA8: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246EEAC: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246EEB0: 4199FFB8  bgt cr6, 0x8246ee68
	if ctx.cr[6].gt {
	pc = 0x8246EE68; continue 'dispatch;
	}
	// 8246EEB4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246EEB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8246EEBC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EEC0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246EEC4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246EEC8: 4BFFB461  bl 0x8246a328
	ctx.lr = 0x8246EECC;
	sub_8246A328(ctx, base);
	// 8246EECC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EED0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8246EED4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8246EED8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246EEDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246EEE0: 480C6220  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 8246EEE4: 7C7DD050  subf r3, r29, r26
	ctx.r[3].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	// 8246EEE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246EEEC: 480C6214  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EEF0 size=76
    let mut pc: u32 = 0x8246EEF0;
    'dispatch: loop {
        match pc {
            0x8246EEF0 => {
    //   block [0x8246EEF0..0x8246EF3C)
	// 8246EEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246EEF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246EEFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EF00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EF04: 4BFFFEA5  bl 0x8246eda8
	ctx.lr = 0x8246EF08;
	sub_8246EDA8(ctx, base);
	// 8246EF08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EF0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EF10: 419A0018  beq cr6, 0x8246ef28
	if ctx.cr[6].eq {
	pc = 0x8246EF28; continue 'dispatch;
	}
	// 8246EF14: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246EF18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EF1C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246EF20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EF24: 4E800421  bctrl
	ctx.lr = 0x8246EF28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246EF28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246EF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246EF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246EF34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246EF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246EF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246EF40 size=192
    let mut pc: u32 = 0x8246EF40;
    'dispatch: loop {
        match pc {
            0x8246EF40 => {
    //   block [0x8246EF40..0x8246F000)
	// 8246EF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246EF44: 480C6179  bl 0x825350bc
	ctx.lr = 0x8246EF48;
	sub_82535080(ctx, base);
	// 8246EF48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246EF4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246EF50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246EF54: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8246EF58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EF5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246EF60: 419A002C  beq cr6, 0x8246ef8c
	if ctx.cr[6].eq {
	pc = 0x8246EF8C; continue 'dispatch;
	}
	// 8246EF64: 4BFFFE45  bl 0x8246eda8
	ctx.lr = 0x8246EF68;
	sub_8246EDA8(ctx, base);
	// 8246EF68: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246EF6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8246EF70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246EF74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246EF78: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8246EF7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246EF80: 4E800421  bctrl
	ctx.lr = 0x8246EF84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246EF84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EF88: 480C6184  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8246EF8C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8246EF90: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 8246EF94: 41980028  blt cr6, 0x8246efbc
	if ctx.cr[6].lt {
	pc = 0x8246EFBC; continue 'dispatch;
	}
	// 8246EF98: 419A0018  beq cr6, 0x8246efb0
	if ctx.cr[6].eq {
	pc = 0x8246EFB0; continue 'dispatch;
	}
	// 8246EF9C: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 8246EFA0: 40980020  bge cr6, 0x8246efc0
	if !ctx.cr[6].lt {
	pc = 0x8246EFC0; continue 'dispatch;
	}
	// 8246EFA4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EFA8: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8246EFAC: 48000014  b 0x8246efc0
	pc = 0x8246EFC0; continue 'dispatch;
	// 8246EFB0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246EFB4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8246EFB8: 48000008  b 0x8246efc0
	pc = 0x8246EFC0; continue 'dispatch;
	// 8246EFBC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8246EFC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246EFC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246EFC8: 40980018  bge cr6, 0x8246efe0
	if !ctx.cr[6].lt {
	pc = 0x8246EFE0; continue 'dispatch;
	}
	// 8246EFCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246EFD0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246EFD4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246EFD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EFDC: 480C6130  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8246EFE0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246EFE4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246EFE8: 4099000C  ble cr6, 0x8246eff4
	if !ctx.cr[6].gt {
	pc = 0x8246EFF4; continue 'dispatch;
	}
	// 8246EFEC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8246EFF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246EFF4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8246EFF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246EFFC: 480C6110  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F000 size=120
    let mut pc: u32 = 0x8246F000;
    'dispatch: loop {
        match pc {
            0x8246F000 => {
    //   block [0x8246F000..0x8246F078)
	// 8246F000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F00C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F014: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F01C: 419A003C  beq cr6, 0x8246f058
	if ctx.cr[6].eq {
	pc = 0x8246F058; continue 'dispatch;
	}
	// 8246F020: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246F024: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F028: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8246F02C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F030: 4E800421  bctrl
	ctx.lr = 0x8246F034;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F034: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246F038: 41980028  blt cr6, 0x8246f060
	if ctx.cr[6].lt {
	pc = 0x8246F060; continue 'dispatch;
	}
	// 8246F03C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246F040: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246F044: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F054: 4E800020  blr
	return;
	// 8246F058: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246F05C: 4BFFFFE0  b 0x8246f03c
	pc = 0x8246F03C; continue 'dispatch;
	// 8246F060: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246F064: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F070: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F078 size=128
    let mut pc: u32 = 0x8246F078;
    'dispatch: loop {
        match pc {
            0x8246F078 => {
    //   block [0x8246F078..0x8246F0F8)
	// 8246F078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246F08C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F090: 394A87AC  addi r10, r10, -0x7854
	ctx.r[10].s64 = ctx.r[10].s64 + -30804;
	// 8246F094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246F098: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246F09C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8246F0A0: 7CCB0774  extsb r11, r6
	ctx.r[11].s64 = ctx.r[6].s8 as i64;
	// 8246F0A4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8246F0A8: 3945FFFF  addi r10, r5, -1
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	// 8246F0AC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8246F0B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246F0B4: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8246F0B8: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8246F0BC: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8246F0C0: 409A0008  bne cr6, 0x8246f0c8
	if !ctx.cr[6].eq {
	pc = 0x8246F0C8; continue 'dispatch;
	}
	// 8246F0C4: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8246F0C8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8246F0CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246F0D0: 993F0018  stb r9, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 8246F0D4: 419A000C  beq cr6, 0x8246f0e0
	if ctx.cr[6].eq {
	pc = 0x8246F0E0; continue 'dispatch;
	}
	// 8246F0D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8246F0DC: 4BFFB25D  bl 0x8246a338
	ctx.lr = 0x8246F0E0;
	sub_8246A338(ctx, base);
	// 8246F0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F0E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F0F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F0F8 size=120
    let mut pc: u32 = 0x8246F0F8;
    'dispatch: loop {
        match pc {
            0x8246F0F8 => {
    //   block [0x8246F0F8..0x8246F170)
	// 8246F0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F104: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F108: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F10C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246F114: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F118: 419A001C  beq cr6, 0x8246f134
	if ctx.cr[6].eq {
	pc = 0x8246F134; continue 'dispatch;
	}
	// 8246F11C: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246F120: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F124: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F12C: 4E800421  bctrl
	ctx.lr = 0x8246F130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F130: 48000020  b 0x8246f150
	pc = 0x8246F150; continue 'dispatch;
	// 8246F134: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246F138: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246F13C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246F140: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246F144: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246F148: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8246F14C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8246F150: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F158: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F15C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F170 size=100
    let mut pc: u32 = 0x8246F170;
    'dispatch: loop {
        match pc {
            0x8246F170 => {
    //   block [0x8246F170..0x8246F1D4)
	// 8246F170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F17C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F180: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246F18C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F190: 419A001C  beq cr6, 0x8246f1ac
	if ctx.cr[6].eq {
	pc = 0x8246F1AC; continue 'dispatch;
	}
	// 8246F194: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8246F198: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F19C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246F1A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F1A4: 4E800421  bctrl
	ctx.lr = 0x8246F1A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F1A8: 4800000C  b 0x8246f1b4
	pc = 0x8246F1B4; continue 'dispatch;
	// 8246F1AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246F1B0: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8246F1B4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F1BC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F1C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F1CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F1D8 size=180
    let mut pc: u32 = 0x8246F1D8;
    'dispatch: loop {
        match pc {
            0x8246F1D8 => {
    //   block [0x8246F1D8..0x8246F28C)
	// 8246F1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246F1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F1EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F1F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F1F4: 396B87AC  addi r11, r11, -0x7854
	ctx.r[11].s64 = ctx.r[11].s64 + -30804;
	// 8246F1F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246F1FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246F200: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8246F204: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F208: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F20C: 995F0018  stb r10, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 8246F210: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F214: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F218: 419A001C  beq cr6, 0x8246f234
	if ctx.cr[6].eq {
	pc = 0x8246F234; continue 'dispatch;
	}
	// 8246F21C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F220: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246F224: 419A0010  beq cr6, 0x8246f234
	if ctx.cr[6].eq {
	pc = 0x8246F234; continue 'dispatch;
	}
	// 8246F228: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246F22C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246F230: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F234: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F238: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F23C: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246F240: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246F244: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8246F248: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F24C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F250: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F258: 4E800421  bctrl
	ctx.lr = 0x8246F25C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F25C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246F260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246F264: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8246F268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F26C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8246F270: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8246F274: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246F284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F290 size=208
    let mut pc: u32 = 0x8246F290;
    'dispatch: loop {
        match pc {
            0x8246F290 => {
    //   block [0x8246F290..0x8246F360)
	// 8246F290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F298: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F29C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F2A0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F2A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F2A8: 396B87AC  addi r11, r11, -0x7854
	ctx.r[11].s64 = ctx.r[11].s64 + -30804;
	// 8246F2AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F2B0: 4BFFFAF9  bl 0x8246eda8
	ctx.lr = 0x8246F2B4;
	sub_8246EDA8(ctx, base);
	// 8246F2B4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F2B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F2BC: 419A0014  beq cr6, 0x8246f2d0
	if ctx.cr[6].eq {
	pc = 0x8246F2D0; continue 'dispatch;
	}
	// 8246F2C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F2C4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246F2C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F2CC: 4E800421  bctrl
	ctx.lr = 0x8246F2D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F2D0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F2D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F2D8: 419A003C  beq cr6, 0x8246f314
	if ctx.cr[6].eq {
	pc = 0x8246F314; continue 'dispatch;
	}
	// 8246F2DC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F2E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F2E4: 419A0030  beq cr6, 0x8246f314
	if ctx.cr[6].eq {
	pc = 0x8246F314; continue 'dispatch;
	}
	// 8246F2E8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246F2EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246F2F0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8246F2F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246F2F8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8246F2FC: 409A0018  bne cr6, 0x8246f314
	if !ctx.cr[6].eq {
	pc = 0x8246F314; continue 'dispatch;
	}
	// 8246F300: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F304: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246F308: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F30C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F310: 4E800421  bctrl
	ctx.lr = 0x8246F314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F314: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246F318: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F31C: 419A0024  beq cr6, 0x8246f340
	if ctx.cr[6].eq {
	pc = 0x8246F340; continue 'dispatch;
	}
	// 8246F320: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F324: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F328: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F32C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F330: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F334: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F338: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F33C: 4E800421  bctrl
	ctx.lr = 0x8246F340;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F340: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F344: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F348: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F34C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F358: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F360 size=100
    let mut pc: u32 = 0x8246F360;
    'dispatch: loop {
        match pc {
            0x8246F360 => {
    //   block [0x8246F360..0x8246F3C4)
	// 8246F360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246F36C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F378: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246F37C: 4BFFFF15  bl 0x8246f290
	ctx.lr = 0x8246F380;
	sub_8246F290(ctx, base);
	// 8246F380: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246F384: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F388: 419A0020  beq cr6, 0x8246f3a8
	if ctx.cr[6].eq {
	pc = 0x8246F3A8; continue 'dispatch;
	}
	// 8246F38C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F390: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F394: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246F398: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F39C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246F3A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F3A4: 4BFF4D15  bl 0x824640b8
	ctx.lr = 0x8246F3A8;
	sub_824640B8(ctx, base);
	// 8246F3A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F3AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F3B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246F3BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F3C8 size=56
    let mut pc: u32 = 0x8246F3C8;
    'dispatch: loop {
        match pc {
            0x8246F3C8 => {
    //   block [0x8246F3C8..0x8246F400)
	// 8246F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F3D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F3D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F3D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F3DC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246F3E0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8246F3E4: 480C97D5  bl 0x82538bb8
	ctx.lr = 0x8246F3E8;
	sub_82538BB8(ctx, base);
	// 8246F3E8: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8246F3EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F3F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F400 size=76
    let mut pc: u32 = 0x8246F400;
    'dispatch: loop {
        match pc {
            0x8246F400 => {
    //   block [0x8246F400..0x8246F44C)
	// 8246F400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F40C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F414: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F41C: 419A0014  beq cr6, 0x8246f430
	if ctx.cr[6].eq {
	pc = 0x8246F430; continue 'dispatch;
	}
	// 8246F420: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F424: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F428: 419A0008  beq cr6, 0x8246f430
	if ctx.cr[6].eq {
	pc = 0x8246F430; continue 'dispatch;
	}
	// 8246F42C: 480C9855  bl 0x82538c80
	ctx.lr = 0x8246F430;
	sub_82538C80(ctx, base);
	// 8246F430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246F434: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246F438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F450 size=132
    let mut pc: u32 = 0x8246F450;
    'dispatch: loop {
        match pc {
            0x8246F450 => {
    //   block [0x8246F450..0x8246F4D4)
	// 8246F450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246F45C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F468: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246F46C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F470: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8246F474: 419A0044  beq cr6, 0x8246f4b8
	if ctx.cr[6].eq {
	pc = 0x8246F4B8; continue 'dispatch;
	}
	// 8246F478: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246F47C: 480C9B9D  bl 0x82539018
	ctx.lr = 0x8246F480;
	sub_82539018(ctx, base);
	// 8246F480: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246F484: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246F488: 41990028  bgt cr6, 0x8246f4b0
	if ctx.cr[6].gt {
	pc = 0x8246F4B0; continue 'dispatch;
	}
	// 8246F48C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F490: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F494: 419A0014  beq cr6, 0x8246f4a8
	if ctx.cr[6].eq {
	pc = 0x8246F4A8; continue 'dispatch;
	}
	// 8246F498: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F49C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F4A0: 419A0008  beq cr6, 0x8246f4a8
	if ctx.cr[6].eq {
	pc = 0x8246F4A8; continue 'dispatch;
	}
	// 8246F4A4: 480C97DD  bl 0x82538c80
	ctx.lr = 0x8246F4A8;
	sub_82538C80(ctx, base);
	// 8246F4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246F4AC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246F4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246F4B4: 48000008  b 0x8246f4bc
	pc = 0x8246F4BC; continue 'dispatch;
	// 8246F4B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246F4BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F4C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246F4CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F4D8 size=12
    let mut pc: u32 = 0x8246F4D8;
    'dispatch: loop {
        match pc {
            0x8246F4D8 => {
    //   block [0x8246F4D8..0x8246F4E4)
	// 8246F4D8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F4DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F4E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F4E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F4E4 size=8
    let mut pc: u32 = 0x8246F4E4;
    'dispatch: loop {
        match pc {
            0x8246F4E4 => {
    //   block [0x8246F4E4..0x8246F4EC)
	// 8246F4E4: 480C7BE4  b 0x825370c8
	sub_825370C8(ctx, base);
	return;
	// 8246F4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F4F0 size=24
    let mut pc: u32 = 0x8246F4F0;
    'dispatch: loop {
        match pc {
            0x8246F4F0 => {
    //   block [0x8246F4F0..0x8246F508)
	// 8246F4F0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F4F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246F4F8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246F4FC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8246F500: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F508 size=12
    let mut pc: u32 = 0x8246F508;
    'dispatch: loop {
        match pc {
            0x8246F508 => {
    //   block [0x8246F508..0x8246F514)
	// 8246F508: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246F50C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F518 size=12
    let mut pc: u32 = 0x8246F518;
    'dispatch: loop {
        match pc {
            0x8246F518 => {
    //   block [0x8246F518..0x8246F524)
	// 8246F518: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246F51C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246F520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F528 size=48
    let mut pc: u32 = 0x8246F528;
    'dispatch: loop {
        match pc {
            0x8246F528 => {
    //   block [0x8246F528..0x8246F558)
	// 8246F528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F534: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F538: 480C90A9  bl 0x825385e0
	ctx.lr = 0x8246F53C;
	sub_825385E0(ctx, base);
	// 8246F53C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8246F540: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246F544: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8246F548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F558 size=8
    let mut pc: u32 = 0x8246F558;
    'dispatch: loop {
        match pc {
            0x8246F558 => {
    //   block [0x8246F558..0x8246F560)
	// 8246F558: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F55C: 480C93C4  b 0x82538920
	sub_82538920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F560 size=104
    let mut pc: u32 = 0x8246F560;
    'dispatch: loop {
        match pc {
            0x8246F560 => {
    //   block [0x8246F560..0x8246F5C8)
	// 8246F560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F56C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F570: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F578: 396B87D4  addi r11, r11, -0x782c
	ctx.r[11].s64 = ctx.r[11].s64 + -30764;
	// 8246F57C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246F580: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246F584: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8246F588: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F58C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F590: 993F000C  stb r9, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u8 ) };
	// 8246F594: 409A0010  bne cr6, 0x8246f5a4
	if !ctx.cr[6].eq {
	pc = 0x8246F5A4; continue 'dispatch;
	}
	// 8246F598: 480C7591  bl 0x82536b28
	ctx.lr = 0x8246F59C;
	sub_82536B28(ctx, base);
	// 8246F59C: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 8246F5A0: 4800000C  b 0x8246f5ac
	pc = 0x8246F5AC; continue 'dispatch;
	// 8246F5A4: 480C7585  bl 0x82536b28
	ctx.lr = 0x8246F5A8;
	sub_82536B28(ctx, base);
	// 8246F5A8: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 8246F5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F5B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246F5B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F5C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F5C8 size=108
    let mut pc: u32 = 0x8246F5C8;
    'dispatch: loop {
        match pc {
            0x8246F5C8 => {
    //   block [0x8246F5C8..0x8246F634)
	// 8246F5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F5D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F5D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F5D8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F5E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246F5E4: 396B87D4  addi r11, r11, -0x782c
	ctx.r[11].s64 = ctx.r[11].s64 + -30764;
	// 8246F5E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8246F5EC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8246F5F0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F5F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F5F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F5FC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8246F600: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 8246F604: 419A0018  beq cr6, 0x8246f61c
	if ctx.cr[6].eq {
	pc = 0x8246F61C; continue 'dispatch;
	}
	// 8246F608: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F60C: 388B87FC  addi r4, r11, -0x7804
	ctx.r[4].s64 = ctx.r[11].s64 + -30724;
	// 8246F610: 480C95A9  bl 0x82538bb8
	ctx.lr = 0x8246F614;
	sub_82538BB8(ctx, base);
	// 8246F614: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246F618: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246F61C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F62C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F638 size=100
    let mut pc: u32 = 0x8246F638;
    'dispatch: loop {
        match pc {
            0x8246F638 => {
    //   block [0x8246F638..0x8246F69C)
	// 8246F638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F640: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F644: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F64C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F650: 396B87D4  addi r11, r11, -0x782c
	ctx.r[11].s64 = ctx.r[11].s64 + -30764;
	// 8246F654: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F658: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F65C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F660: 419A0014  beq cr6, 0x8246f674
	if ctx.cr[6].eq {
	pc = 0x8246F674; continue 'dispatch;
	}
	// 8246F664: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F66C: 419A0008  beq cr6, 0x8246f674
	if ctx.cr[6].eq {
	pc = 0x8246F674; continue 'dispatch;
	}
	// 8246F670: 480C9611  bl 0x82538c80
	ctx.lr = 0x8246F674;
	sub_82538C80(ctx, base);
	// 8246F674: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246F67C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F680: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246F684: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F6A0 size=156
    let mut pc: u32 = 0x8246F6A0;
    'dispatch: loop {
        match pc {
            0x8246F6A0 => {
    //   block [0x8246F6A0..0x8246F73C)
	// 8246F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F6A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246F6AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F6B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F6B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F6B8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F6BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246F6C0: 396B87D4  addi r11, r11, -0x782c
	ctx.r[11].s64 = ctx.r[11].s64 + -30764;
	// 8246F6C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F6C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246F6CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F6D0: 419A0014  beq cr6, 0x8246f6e4
	if ctx.cr[6].eq {
	pc = 0x8246F6E4; continue 'dispatch;
	}
	// 8246F6D4: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F6D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F6DC: 419A0008  beq cr6, 0x8246f6e4
	if ctx.cr[6].eq {
	pc = 0x8246F6E4; continue 'dispatch;
	}
	// 8246F6E0: 480C95A1  bl 0x82538c80
	ctx.lr = 0x8246F6E4;
	sub_82538C80(ctx, base);
	// 8246F6E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246F6EC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F6F0: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246F6F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246F6F8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246F6FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F700: 419A0020  beq cr6, 0x8246f720
	if ctx.cr[6].eq {
	pc = 0x8246F720; continue 'dispatch;
	}
	// 8246F704: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F708: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F70C: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246F710: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246F714: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246F718: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F71C: 4BFF499D  bl 0x824640b8
	ctx.lr = 0x8246F720;
	sub_824640B8(ctx, base);
	// 8246F720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F724: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F730: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246F734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F740 size=20
    let mut pc: u32 = 0x8246F740;
    'dispatch: loop {
        match pc {
            0x8246F740 => {
    //   block [0x8246F740..0x8246F754)
	// 8246F740: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F744: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8246F748: 386B8800  addi r3, r11, -0x7800
	ctx.r[3].s64 = ctx.r[11].s64 + -30720;
	// 8246F74C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8246F750: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F758 size=8
    let mut pc: u32 = 0x8246F758;
    'dispatch: loop {
        match pc {
            0x8246F758 => {
    //   block [0x8246F758..0x8246F760)
	// 8246F758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246F75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F760 size=24
    let mut pc: u32 = 0x8246F760;
    'dispatch: loop {
        match pc {
            0x8246F760 => {
    //   block [0x8246F760..0x8246F778)
	// 8246F760: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F764: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246F768: 396B8820  addi r11, r11, -0x77e0
	ctx.r[11].s64 = ctx.r[11].s64 + -30688;
	// 8246F76C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246F770: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F778 size=16
    let mut pc: u32 = 0x8246F778;
    'dispatch: loop {
        match pc {
            0x8246F778 => {
    //   block [0x8246F778..0x8246F788)
	// 8246F778: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F77C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F780: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F788 size=108
    let mut pc: u32 = 0x8246F788;
    'dispatch: loop {
        match pc {
            0x8246F788 => {
    //   block [0x8246F788..0x8246F7F4)
	// 8246F788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F78C: 480C592D  bl 0x825350b8
	ctx.lr = 0x8246F790;
	sub_82535080(ctx, base);
	// 8246F790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F794: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246F798: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246F79C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246F7A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246F7A4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246F7A8: 40990034  ble cr6, 0x8246f7dc
	if !ctx.cr[6].gt {
	pc = 0x8246F7DC; continue 'dispatch;
	}
	// 8246F7AC: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F7B0: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8246F7B4: 7C9FE214  add r4, r31, r28
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 8246F7B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F7BC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246F7C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F7C4: 4E800421  bctrl
	ctx.lr = 0x8246F7C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F7C8: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8246F7CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246F7D0: 419A0018  beq cr6, 0x8246f7e8
	if ctx.cr[6].eq {
	pc = 0x8246F7E8; continue 'dispatch;
	}
	// 8246F7D4: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246F7D8: 4198FFD4  blt cr6, 0x8246f7ac
	if ctx.cr[6].lt {
	pc = 0x8246F7AC; continue 'dispatch;
	}
	// 8246F7DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246F7E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246F7E4: 480C5924  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8246F7E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246F7F0: 480C5918  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F7F8 size=64
    let mut pc: u32 = 0x8246F7F8;
    'dispatch: loop {
        match pc {
            0x8246F7F8 => {
    //   block [0x8246F7F8..0x8246F838)
	// 8246F7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F800: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F804: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F808: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F80C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F810: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F814: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F81C: 4E800421  bctrl
	ctx.lr = 0x8246F820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F82C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F838 size=108
    let mut pc: u32 = 0x8246F838;
    'dispatch: loop {
        match pc {
            0x8246F838 => {
    //   block [0x8246F838..0x8246F8A4)
	// 8246F838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F83C: 480C587D  bl 0x825350b8
	ctx.lr = 0x8246F840;
	sub_82535080(ctx, base);
	// 8246F840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F844: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8246F848: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246F84C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246F850: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246F854: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8246F858: 40990034  ble cr6, 0x8246f88c
	if !ctx.cr[6].gt {
	pc = 0x8246F88C; continue 'dispatch;
	}
	// 8246F85C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F860: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8246F864: 7C9FE214  add r4, r31, r28
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 8246F868: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F86C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246F870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F874: 4E800421  bctrl
	ctx.lr = 0x8246F878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F878: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8246F87C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246F880: 419A0018  beq cr6, 0x8246f898
	if ctx.cr[6].eq {
	pc = 0x8246F898; continue 'dispatch;
	}
	// 8246F884: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246F888: 4198FFD4  blt cr6, 0x8246f85c
	if ctx.cr[6].lt {
	pc = 0x8246F85C; continue 'dispatch;
	}
	// 8246F88C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246F890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246F894: 480C5874  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8246F898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246F8A0: 480C5868  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F8A8 size=64
    let mut pc: u32 = 0x8246F8A8;
    'dispatch: loop {
        match pc {
            0x8246F8A8 => {
    //   block [0x8246F8A8..0x8246F8E8)
	// 8246F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F8B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F8B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F8B8: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246F8BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F8C0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F8C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246F8C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F8CC: 4E800421  bctrl
	ctx.lr = 0x8246F8D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F8D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F8E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246F8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F8E8 size=68
    let mut pc: u32 = 0x8246F8E8;
    'dispatch: loop {
        match pc {
            0x8246F8E8 => {
    //   block [0x8246F8E8..0x8246F92C)
	// 8246F8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F8F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F8F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246F8F8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246F8FC: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8246F900: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8246F904: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246F908: 4BFF4731  bl 0x82464038
	ctx.lr = 0x8246F90C;
	sub_82464038(ctx, base);
	// 8246F90C: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 8246F910: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8246F914: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8246F918: 480013D1  bl 0x82470ce8
	ctx.lr = 0x8246F91C;
	sub_82470CE8(ctx, base);
	// 8246F91C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246F920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246F924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246F928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F930 size=148
    let mut pc: u32 = 0x8246F930;
    'dispatch: loop {
        match pc {
            0x8246F930 => {
    //   block [0x8246F930..0x8246F9C4)
	// 8246F930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F934: 480C5789  bl 0x825350bc
	ctx.lr = 0x8246F938;
	sub_82535080(ctx, base);
	// 8246F938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F93C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246F940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8246F944: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 8246F948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F94C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8246F950: 396B888C  addi r11, r11, -0x7774
	ctx.r[11].s64 = ctx.r[11].s64 + -30580;
	// 8246F954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8246F958: 394A8830  addi r10, r10, -0x77d0
	ctx.r[10].s64 = ctx.r[10].s64 + -30672;
	// 8246F95C: 39298864  addi r9, r9, -0x779c
	ctx.r[9].s64 = ctx.r[9].s64 + -30620;
	// 8246F960: 3FA08293  lis r29, -0x7d6d
	ctx.r[29].s64 = -2104295424;
	// 8246F964: B3DF0006  sth r30, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[30].u16 ) };
	// 8246F968: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F96C: B3DF000E  sth r30, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[30].u16 ) };
	// 8246F970: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8246F974: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8246F978: B3DF001A  sth r30, 0x1a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[30].u16 ) };
	// 8246F97C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8246F980: 911F001C  stw r8, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8246F984: 93FF0010  stw r31, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 8246F988: 93FF001C  stw r31, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 8246F98C: 897D91D8  lbz r11, -0x6e28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-28200 as u32) ) } as u64;
	// 8246F990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246F994: 409A0024  bne cr6, 0x8246f9b8
	if !ctx.cr[6].eq {
	pc = 0x8246F9B8; continue 'dispatch;
	}
	// 8246F998: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246F99C: 814B4EA4  lwz r10, 0x4ea4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20132 as u32) ) } as u64;
	// 8246F9A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246F9A4: 419A0014  beq cr6, 0x8246f9b8
	if ctx.cr[6].eq {
	pc = 0x8246F9B8; continue 'dispatch;
	}
	// 8246F9A8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246F9AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246F9B0: 4E800421  bctrl
	ctx.lr = 0x8246F9B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246F9B4: 9BDD91D8  stb r30, -0x6e28(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(-28200 as u32), ctx.r[30].u8 ) };
	// 8246F9B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246F9BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246F9C0: 480C574C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246F9C8 size=8
    let mut pc: u32 = 0x8246F9C8;
    'dispatch: loop {
        match pc {
            0x8246F9C8 => {
    //   block [0x8246F9C8..0x8246F9D0)
	// 8246F9C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246F9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246F9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246F9D0 size=96
    let mut pc: u32 = 0x8246F9D0;
    'dispatch: loop {
        match pc {
            0x8246F9D0 => {
    //   block [0x8246F9D0..0x8246FA30)
	// 8246F9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246F9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246F9D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246F9DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246F9E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246F9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246F9E8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246F9EC: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8246F9F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246F9F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246F9F8: 419A0020  beq cr6, 0x8246fa18
	if ctx.cr[6].eq {
	pc = 0x8246FA18; continue 'dispatch;
	}
	// 8246F9FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FA00: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FA04: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8246FA08: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246FA0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246FA10: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FA14: 4BFF46A5  bl 0x824640b8
	ctx.lr = 0x8246FA18;
	sub_824640B8(ctx, base);
	// 8246FA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FA1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FA28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FA30 size=104
    let mut pc: u32 = 0x8246FA30;
    'dispatch: loop {
        match pc {
            0x8246FA30 => {
    //   block [0x8246FA30..0x8246FA98)
	// 8246FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FA38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FA3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FA40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246FA44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FA48: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246FA4C: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8246FA50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246FA54: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8246FA58: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246FA5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FA60: 419A0020  beq cr6, 0x8246fa80
	if ctx.cr[6].eq {
	pc = 0x8246FA80; continue 'dispatch;
	}
	// 8246FA64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FA68: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FA6C: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8246FA70: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246FA74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246FA78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FA7C: 4BFF463D  bl 0x824640b8
	ctx.lr = 0x8246FA80;
	sub_824640B8(ctx, base);
	// 8246FA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FA84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FA98 size=8
    let mut pc: u32 = 0x8246FA98;
    'dispatch: loop {
        match pc {
            0x8246FA98 => {
    //   block [0x8246FA98..0x8246FAA0)
	// 8246FA98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246FA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FAA0 size=8
    let mut pc: u32 = 0x8246FAA0;
    'dispatch: loop {
        match pc {
            0x8246FAA0 => {
    //   block [0x8246FAA0..0x8246FAA8)
	// 8246FAA0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246FAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FAA8 size=12
    let mut pc: u32 = 0x8246FAA8;
    'dispatch: loop {
        match pc {
            0x8246FAA8 => {
    //   block [0x8246FAA8..0x8246FAB4)
	// 8246FAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246FAAC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246FAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FAB8 size=108
    let mut pc: u32 = 0x8246FAB8;
    'dispatch: loop {
        match pc {
            0x8246FAB8 => {
    //   block [0x8246FAB8..0x8246FB24)
	// 8246FAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FABC: 480C5601  bl 0x825350bc
	ctx.lr = 0x8246FAC0;
	sub_82535080(ctx, base);
	// 8246FAC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FAC4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FAC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246FACC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246FAD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246FAD4: 419A0048  beq cr6, 0x8246fb1c
	if ctx.cr[6].eq {
	pc = 0x8246FB1C; continue 'dispatch;
	}
	// 8246FAD8: 7FA42850  subf r29, r4, r5
	ctx.r[29].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 8246FADC: 7CBDF82E  lwzx r5, r29, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8246FAE0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8246FAE4: 419A0038  beq cr6, 0x8246fb1c
	if ctx.cr[6].eq {
	pc = 0x8246FB1C; continue 'dispatch;
	}
	// 8246FAE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FAEC: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246FAF0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8246FAF4: 419A0018  beq cr6, 0x8246fb0c
	if ctx.cr[6].eq {
	pc = 0x8246FB0C; continue 'dispatch;
	}
	// 8246FAF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FAFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246FB00: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246FB04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246FB08: 4E800421  bctrl
	ctx.lr = 0x8246FB0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246FB0C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8246FB10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FB14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246FB18: 409AFFC4  bne cr6, 0x8246fadc
	if !ctx.cr[6].eq {
	pc = 0x8246FADC; continue 'dispatch;
	}
	// 8246FB1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246FB20: 480C55EC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FB28 size=104
    let mut pc: u32 = 0x8246FB28;
    'dispatch: loop {
        match pc {
            0x8246FB28 => {
    //   block [0x8246FB28..0x8246FB90)
	// 8246FB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FB30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FB34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FB38: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FB3C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FB40: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8246FB44: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8246FB48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FB4C: 4BFF44ED  bl 0x82464038
	ctx.lr = 0x8246FB50;
	sub_82464038(ctx, base);
	// 8246FB50: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246FB54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FB58: 396B88BC  addi r11, r11, -0x7744
	ctx.r[11].s64 = ctx.r[11].s64 + -30532;
	// 8246FB5C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8246FB60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246FB64: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8246FB68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FB6C: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8246FB70: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246FB74: 4BFFD7CD  bl 0x8246d340
	ctx.lr = 0x8246FB78;
	sub_8246D340(ctx, base);
	// 8246FB78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FB7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FB88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FB90 size=216
    let mut pc: u32 = 0x8246FB90;
    'dispatch: loop {
        match pc {
            0x8246FB90 => {
    //   block [0x8246FB90..0x8246FC68)
	// 8246FB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FB94: 480C5529  bl 0x825350bc
	ctx.lr = 0x8246FB98;
	sub_82535080(ctx, base);
	// 8246FB98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FB9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246FBA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246FBA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246FBA8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FBAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246FBB0: 41980024  blt cr6, 0x8246fbd4
	if ctx.cr[6].lt {
	pc = 0x8246FBD4; continue 'dispatch;
	}
	// 8246FBB4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FBB8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FBBC: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 8246FBC0: 409A0014  bne cr6, 0x8246fbd4
	if !ctx.cr[6].eq {
	pc = 0x8246FBD4; continue 'dispatch;
	}
	// 8246FBC4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8246FBC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8246FBCC: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246FBD0: 4099FFE8  ble cr6, 0x8246fbb8
	if !ctx.cr[6].gt {
	pc = 0x8246FBB8; continue 'dispatch;
	}
	// 8246FBD4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FBD8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246FBDC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246FBE0: 40990008  ble cr6, 0x8246fbe8
	if !ctx.cr[6].gt {
	pc = 0x8246FBE8; continue 'dispatch;
	}
	// 8246FBE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246FBE8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246FBEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246FBF0: 419A0070  beq cr6, 0x8246fc60
	if ctx.cr[6].eq {
	pc = 0x8246FC60; continue 'dispatch;
	}
	// 8246FBF4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FBF8: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246FBFC: 387D0008  addi r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 8;
	// 8246FC00: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FC04: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FC08: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FC0C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8246FC10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246FC14: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246FC18: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246FC1C: 4BFFD7E5  bl 0x8246d400
	ctx.lr = 0x8246FC20;
	sub_8246D400(ctx, base);
	// 8246FC20: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FC24: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8246FC28: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246FC2C: 4199002C  bgt cr6, 0x8246fc58
	if ctx.cr[6].gt {
	pc = 0x8246FC58; continue 'dispatch;
	}
	// 8246FC30: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FC34: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246FC38: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8246FC3C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FC40: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 8246FC44: 409A0014  bne cr6, 0x8246fc58
	if !ctx.cr[6].eq {
	pc = 0x8246FC58; continue 'dispatch;
	}
	// 8246FC48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246FC4C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8246FC50: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246FC54: 4099FFE8  ble cr6, 0x8246fc3c
	if !ctx.cr[6].gt {
	pc = 0x8246FC3C; continue 'dispatch;
	}
	// 8246FC58: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8246FC5C: 4BFFFF78  b 0x8246fbd4
	pc = 0x8246FBD4; continue 'dispatch;
	// 8246FC60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246FC64: 480C54A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FC68 size=8
    let mut pc: u32 = 0x8246FC68;
    'dispatch: loop {
        match pc {
            0x8246FC68 => {
    //   block [0x8246FC68..0x8246FC70)
	// 8246FC68: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8246FC6C: 4BFFD794  b 0x8246d400
	sub_8246D400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246FC70 size=16
    let mut pc: u32 = 0x8246FC70;
    'dispatch: loop {
        match pc {
            0x8246FC70 => {
    //   block [0x8246FC70..0x8246FC80)
	// 8246FC70: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FC74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8246FC78: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8246FC7C: 4BFFD8B4  b 0x8246d530
	sub_8246D530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FC80 size=116
    let mut pc: u32 = 0x8246FC80;
    'dispatch: loop {
        match pc {
            0x8246FC80 => {
    //   block [0x8246FC80..0x8246FCF4)
	// 8246FC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FC88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246FC8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FC90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FC94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FC98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246FC9C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8246FCA0: 4BFFD729  bl 0x8246d3c8
	ctx.lr = 0x8246FCA4;
	sub_8246D3C8(ctx, base);
	// 8246FCA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246FCA8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8246FCAC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246FCB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246FCB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FCB8: 419A0020  beq cr6, 0x8246fcd8
	if ctx.cr[6].eq {
	pc = 0x8246FCD8; continue 'dispatch;
	}
	// 8246FCBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FCC0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FCC4: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8246FCC8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246FCCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246FCD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FCD4: 4BFF43E5  bl 0x824640b8
	ctx.lr = 0x8246FCD8;
	sub_824640B8(ctx, base);
	// 8246FCD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FCDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246FCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FCE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246FCEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FCF8 size=144
    let mut pc: u32 = 0x8246FCF8;
    'dispatch: loop {
        match pc {
            0x8246FCF8 => {
    //   block [0x8246FCF8..0x8246FD88)
	// 8246FCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FCFC: 480C53B5  bl 0x825350b0
	ctx.lr = 0x8246FD00;
	sub_82535080(ctx, base);
	// 8246FD00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FD04: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246FD08: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246FD0C: 3BCB4F80  addi r30, r11, 0x4f80
	ctx.r[30].s64 = ctx.r[11].s64 + 20352;
	// 8246FD10: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8246FD14: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8246FD18: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8246FD1C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8246FD20: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246FD24: 4BFF6B35  bl 0x82466858
	ctx.lr = 0x8246FD28;
	sub_82466858(ctx, base);
	// 8246FD28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8246FD2C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FD30: 4BFFA2A1  bl 0x82469fd0
	ctx.lr = 0x8246FD34;
	sub_82469FD0(ctx, base);
	// 8246FD34: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246FD38: 419A002C  beq cr6, 0x8246fd64
	if ctx.cr[6].eq {
	pc = 0x8246FD64; continue 'dispatch;
	}
	// 8246FD3C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8246FD40: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 8246FD44: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8246FD48: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246FD4C: 4198FFD4  blt cr6, 0x8246fd20
	if ctx.cr[6].lt {
	pc = 0x8246FD20; continue 'dispatch;
	}
	// 8246FD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246FD54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246FD58: 997C0000  stb r11, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246FD5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246FD60: 480C53A0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 8246FD64: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246FD68: 393E0004  addi r9, r30, 4
	ctx.r[9].s64 = ctx.r[30].s64 + 4;
	// 8246FD6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246FD70: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246FD74: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246FD78: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8246FD7C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FD80: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246FD84: 480C537C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FD88 size=72
    let mut pc: u32 = 0x8246FD88;
    'dispatch: loop {
        match pc {
            0x8246FD88 => {
    //   block [0x8246FD88..0x8246FDD0)
	// 8246FD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FD90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FD94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FD98: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246FD9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FDA0: 396B88E8  addi r11, r11, -0x7718
	ctx.r[11].s64 = ctx.r[11].s64 + -30488;
	// 8246FDA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246FDA8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8246FDAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FDB0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8246FDB4: 4BFFD58D  bl 0x8246d340
	ctx.lr = 0x8246FDB8;
	sub_8246D340(ctx, base);
	// 8246FDB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FDBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FDC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FDD0 size=128
    let mut pc: u32 = 0x8246FDD0;
    'dispatch: loop {
        match pc {
            0x8246FDD0 => {
    //   block [0x8246FDD0..0x8246FE50)
	// 8246FDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FDD4: 480C52E9  bl 0x825350bc
	ctx.lr = 0x8246FDD8;
	sub_82535080(ctx, base);
	// 8246FDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FDDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246FDE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246FDE4: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 8246FDE8: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8246FDEC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246FDF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246FDF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246FDF8: 4BFFFF01  bl 0x8246fcf8
	ctx.lr = 0x8246FDFC;
	sub_8246FCF8(ctx, base);
	// 8246FDFC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FE00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246FE04: 409A0040  bne cr6, 0x8246fe44
	if !ctx.cr[6].eq {
	pc = 0x8246FE44; continue 'dispatch;
	}
	// 8246FE08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246FE0C: 4BFF6A55  bl 0x82466860
	ctx.lr = 0x8246FE10;
	sub_82466860(ctx, base);
	// 8246FE10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246FE14: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8246FE18: 409AFFD0  bne cr6, 0x8246fde8
	if !ctx.cr[6].eq {
	pc = 0x8246FDE8; continue 'dispatch;
	}
	// 8246FE1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246FE20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246FE24: 4BFF6EA5  bl 0x82466cc8
	ctx.lr = 0x8246FE28;
	sub_82466CC8(ctx, base);
	// 8246FE28: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FE2C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246FE30: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8246FE34: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8246FE38: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8246FE3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246FE40: 480C52CC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8246FE44: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8246FE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246FE4C: 480C52C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FE50 size=104
    let mut pc: u32 = 0x8246FE50;
    'dispatch: loop {
        match pc {
            0x8246FE50 => {
    //   block [0x8246FE50..0x8246FEB8)
	// 8246FE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FE58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FE5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FE60: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FE64: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246FE68: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8246FE6C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8246FE70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246FE74: 4BFF41C5  bl 0x82464038
	ctx.lr = 0x8246FE78;
	sub_82464038(ctx, base);
	// 8246FE78: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246FE7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FE80: 396B88E8  addi r11, r11, -0x7718
	ctx.r[11].s64 = ctx.r[11].s64 + -30488;
	// 8246FE84: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8246FE88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8246FE8C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8246FE90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246FE94: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8246FE98: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8246FE9C: 4BFFD4A5  bl 0x8246d340
	ctx.lr = 0x8246FEA0;
	sub_8246D340(ctx, base);
	// 8246FEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246FEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FEB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FEB8 size=104
    let mut pc: u32 = 0x8246FEB8;
    'dispatch: loop {
        match pc {
            0x8246FEB8 => {
    //   block [0x8246FEB8..0x8246FF20)
	// 8246FEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246FEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246FEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246FEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246FED0: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 8246FED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246FED8: 4BFFD601  bl 0x8246d4d8
	ctx.lr = 0x8246FEDC;
	sub_8246D4D8(ctx, base);
	// 8246FEDC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FEE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8246FEE4: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246FEE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246FEEC: 40990008  ble cr6, 0x8246fef4
	if !ctx.cr[6].gt {
	pc = 0x8246FEF4; continue 'dispatch;
	}
	// 8246FEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246FEF4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246FEF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246FEFC: 419A000C  beq cr6, 0x8246ff08
	if ctx.cr[6].eq {
	pc = 0x8246FF08; continue 'dispatch;
	}
	// 8246FF00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246FF04: 4BFFD71D  bl 0x8246d620
	ctx.lr = 0x8246FF08;
	sub_8246D620(ctx, base);
	// 8246FF08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246FF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246FF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246FF14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246FF18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246FF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FF20 size=124
    let mut pc: u32 = 0x8246FF20;
    'dispatch: loop {
        match pc {
            0x8246FF20 => {
    //   block [0x8246FF20..0x8246FF9C)
	// 8246FF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FF24: 480C5195  bl 0x825350b8
	ctx.lr = 0x8246FF28;
	sub_82535080(ctx, base);
	// 8246FF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FF2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246FF30: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246FF34: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 8246FF38: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8246FF3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FF40: 4BFFD599  bl 0x8246d4d8
	ctx.lr = 0x8246FF44;
	sub_8246D4D8(ctx, base);
	// 8246FF44: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FF48: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246FF4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246FF50: 40990008  ble cr6, 0x8246ff58
	if !ctx.cr[6].gt {
	pc = 0x8246FF58; continue 'dispatch;
	}
	// 8246FF54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246FF58: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246FF5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246FF60: 419A0024  beq cr6, 0x8246ff84
	if ctx.cr[6].eq {
	pc = 0x8246FF84; continue 'dispatch;
	}
	// 8246FF64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246FF68: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FF6C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8246FF70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246FF74: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246FF78: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 8246FF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246FF80: 480C5188  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8246FF84: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8246FF88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246FF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FF90: 4BFFD471  bl 0x8246d400
	ctx.lr = 0x8246FF94;
	sub_8246D400(ctx, base);
	// 8246FF94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246FF98: 480C5170  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246FFA0 size=140
    let mut pc: u32 = 0x8246FFA0;
    'dispatch: loop {
        match pc {
            0x8246FFA0 => {
    //   block [0x8246FFA0..0x8247002C)
	// 8246FFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246FFA4: 480C5115  bl 0x825350b8
	ctx.lr = 0x8246FFA8;
	sub_82535080(ctx, base);
	// 8246FFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246FFAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246FFB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246FFB4: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 8246FFB8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8246FFBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246FFC0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8246FFC4: 4BFFD515  bl 0x8246d4d8
	ctx.lr = 0x8246FFC8;
	sub_8246D4D8(ctx, base);
	// 8246FFC8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246FFCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246FFD0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246FFD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246FFD8: 40990008  ble cr6, 0x8246ffe0
	if !ctx.cr[6].gt {
	pc = 0x8246FFE0; continue 'dispatch;
	}
	// 8246FFDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246FFE0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246FFE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246FFE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246FFEC: 419A0030  beq cr6, 0x8247001c
	if ctx.cr[6].eq {
	pc = 0x8247001C; continue 'dispatch;
	}
	// 8246FFF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246FFF4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246FFF8: 995D0000  stb r10, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8246FFFC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82470000: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82470004: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82470008: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247000C: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82470010: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82470018: 480C50F0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8247001C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82470020: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82470024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82470028: 480C50E0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470030 size=144
    let mut pc: u32 = 0x82470030;
    'dispatch: loop {
        match pc {
            0x82470030 => {
    //   block [0x82470030..0x824700C0)
	// 82470030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470034: 480C5085  bl 0x825350b8
	ctx.lr = 0x82470038;
	sub_82535080(ctx, base);
	// 82470038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247003C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82470040: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82470044: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	// 82470048: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 8247004C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82470050: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82470054: 4BFFD485  bl 0x8246d4d8
	ctx.lr = 0x82470058;
	sub_8246D4D8(ctx, base);
	// 82470058: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8247005C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82470060: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82470064: 40990008  ble cr6, 0x8247006c
	if !ctx.cr[6].gt {
	pc = 0x8247006C; continue 'dispatch;
	}
	// 82470068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247006C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82470070: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82470074: 409A002C  bne cr6, 0x824700a0
	if !ctx.cr[6].eq {
	pc = 0x824700A0; continue 'dispatch;
	}
	// 82470078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247007C: 4BFF67E5  bl 0x82466860
	ctx.lr = 0x82470080;
	sub_82466860(ctx, base);
	// 82470080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470084: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82470088: 409AFFC4  bne cr6, 0x8247004c
	if !ctx.cr[6].eq {
	pc = 0x8247004C; continue 'dispatch;
	}
	// 8247008C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82470090: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470094: 4BFFFD3D  bl 0x8246fdd0
	ctx.lr = 0x82470098;
	sub_8246FDD0(ctx, base);
	// 82470098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8247009C: 480C506C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824700A0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824700A4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824700A8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824700AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824700B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824700B4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824700B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824700BC: 480C504C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824700C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824700C0 size=116
    let mut pc: u32 = 0x824700C0;
    'dispatch: loop {
        match pc {
            0x824700C0 => {
    //   block [0x824700C0..0x82470134)
	// 824700C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824700C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824700C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824700CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824700D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824700D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824700D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824700DC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824700E0: 4BFFD2E9  bl 0x8246d3c8
	ctx.lr = 0x824700E4;
	sub_8246D3C8(ctx, base);
	// 824700E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824700E8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824700EC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824700F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824700F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824700F8: 419A0020  beq cr6, 0x82470118
	if ctx.cr[6].eq {
	pc = 0x82470118; continue 'dispatch;
	}
	// 824700FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470100: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82470104: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82470108: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247010C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82470110: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82470114: 4BFF3FA5  bl 0x824640b8
	ctx.lr = 0x82470118;
	sub_824640B8(ctx, base);
	// 82470118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247011C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470128: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8247012C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470138 size=20
    let mut pc: u32 = 0x82470138;
    'dispatch: loop {
        match pc {
            0x82470138 => {
    //   block [0x82470138..0x8247014C)
	// 82470138: A1650012  lhz r11, 0x12(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(18 as u32) ) } as u64;
	// 8247013C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82470140: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82470144: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470150 size=96
    let mut pc: u32 = 0x82470150;
    'dispatch: loop {
        match pc {
            0x82470150 => {
    //   block [0x82470150..0x824701B0)
	// 82470150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8247015C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247016C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470170: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82470174: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82470178: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8247017C: 4BFF69AD  bl 0x82466b28
	ctx.lr = 0x82470180;
	sub_82466B28(ctx, base);
	// 82470180: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470184: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82470188: 419A0010  beq cr6, 0x82470198
	if ctx.cr[6].eq {
	pc = 0x82470198; continue 'dispatch;
	}
	// 8247018C: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82470190: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82470194: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8247019C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824701A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824701A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824701A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824701AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824701B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824701B0 size=20
    let mut pc: u32 = 0x824701B0;
    'dispatch: loop {
        match pc {
            0x824701B0 => {
    //   block [0x824701B0..0x824701C4)
	// 824701B0: A1650012  lhz r11, 0x12(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(18 as u32) ) } as u64;
	// 824701B4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824701B8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824701BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824701C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824701C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824701C8 size=104
    let mut pc: u32 = 0x824701C8;
    'dispatch: loop {
        match pc {
            0x824701C8 => {
    //   block [0x824701C8..0x82470230)
	// 824701C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824701CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824701D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824701D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824701D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824701DC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824701E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824701E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824701E8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824701EC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824701F0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824701F4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824701F8: 4BFF6931  bl 0x82466b28
	ctx.lr = 0x824701FC;
	sub_82466B28(ctx, base);
	// 824701FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470200: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82470204: 419A0010  beq cr6, 0x82470214
	if ctx.cr[6].eq {
	pc = 0x82470214; continue 'dispatch;
	}
	// 82470208: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 8247020C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82470210: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470218: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8247021C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470224: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82470228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8247022C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470230 size=100
    let mut pc: u32 = 0x82470230;
    'dispatch: loop {
        match pc {
            0x82470230 => {
    //   block [0x82470230..0x82470294)
	// 82470230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8247023C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470244: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247024C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470250: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82470254: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82470258: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8247025C: 4BFF68CD  bl 0x82466b28
	ctx.lr = 0x82470260;
	sub_82466B28(ctx, base);
	// 82470260: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470264: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82470268: 419A0010  beq cr6, 0x82470278
	if ctx.cr[6].eq {
	pc = 0x82470278; continue 'dispatch;
	}
	// 8247026C: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82470270: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82470274: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247027C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470288: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8247028C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470298 size=24
    let mut pc: u32 = 0x82470298;
    'dispatch: loop {
        match pc {
            0x82470298 => {
    //   block [0x82470298..0x824702B0)
	// 82470298: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247029C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824702A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824702A4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824702A8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824702AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702B0 size=8
    let mut pc: u32 = 0x824702B0;
    'dispatch: loop {
        match pc {
            0x824702B0 => {
    //   block [0x824702B0..0x824702B8)
	// 824702B0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702B8 size=8
    let mut pc: u32 = 0x824702B8;
    'dispatch: loop {
        match pc {
            0x824702B8 => {
    //   block [0x824702B8..0x824702C0)
	// 824702B8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702C0 size=16
    let mut pc: u32 = 0x824702C0;
    'dispatch: loop {
        match pc {
            0x824702C0 => {
    //   block [0x824702C0..0x824702D0)
	// 824702C0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702C4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824702C8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824702CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702D0 size=16
    let mut pc: u32 = 0x824702D0;
    'dispatch: loop {
        match pc {
            0x824702D0 => {
    //   block [0x824702D0..0x824702E0)
	// 824702D0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702D4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824702D8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824702DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702E0 size=12
    let mut pc: u32 = 0x824702E0;
    'dispatch: loop {
        match pc {
            0x824702E0 => {
    //   block [0x824702E0..0x824702EC)
	// 824702E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702E4: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824702E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824702F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824702F0 size=16
    let mut pc: u32 = 0x824702F0;
    'dispatch: loop {
        match pc {
            0x824702F0 => {
    //   block [0x824702F0..0x82470300)
	// 824702F0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824702F4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824702F8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824702FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470300 size=16
    let mut pc: u32 = 0x82470300;
    'dispatch: loop {
        match pc {
            0x82470300 => {
    //   block [0x82470300..0x82470310)
	// 82470300: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470304: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470308: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247030C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470310 size=16
    let mut pc: u32 = 0x82470310;
    'dispatch: loop {
        match pc {
            0x82470310 => {
    //   block [0x82470310..0x82470320)
	// 82470310: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470314: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470318: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247031C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470320 size=16
    let mut pc: u32 = 0x82470320;
    'dispatch: loop {
        match pc {
            0x82470320 => {
    //   block [0x82470320..0x82470330)
	// 82470320: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470324: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470328: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247032C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470330 size=16
    let mut pc: u32 = 0x82470330;
    'dispatch: loop {
        match pc {
            0x82470330 => {
    //   block [0x82470330..0x82470340)
	// 82470330: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470334: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470338: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247033C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470340 size=16
    let mut pc: u32 = 0x82470340;
    'dispatch: loop {
        match pc {
            0x82470340 => {
    //   block [0x82470340..0x82470350)
	// 82470340: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470344: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470348: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247034C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470350 size=12
    let mut pc: u32 = 0x82470350;
    'dispatch: loop {
        match pc {
            0x82470350 => {
    //   block [0x82470350..0x8247035C)
	// 82470350: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470354: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82470358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470360 size=12
    let mut pc: u32 = 0x82470360;
    'dispatch: loop {
        match pc {
            0x82470360 => {
    //   block [0x82470360..0x8247036C)
	// 82470360: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470364: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82470368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470370 size=16
    let mut pc: u32 = 0x82470370;
    'dispatch: loop {
        match pc {
            0x82470370 => {
    //   block [0x82470370..0x82470380)
	// 82470370: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470374: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470378: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247037C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470380 size=24
    let mut pc: u32 = 0x82470380;
    'dispatch: loop {
        match pc {
            0x82470380 => {
    //   block [0x82470380..0x82470398)
	// 82470380: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470384: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470388: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8247038C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470390: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82470394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470398 size=16
    let mut pc: u32 = 0x82470398;
    'dispatch: loop {
        match pc {
            0x82470398 => {
    //   block [0x82470398..0x824703A8)
	// 82470398: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247039C: 548B3032  slwi r11, r4, 6
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703A0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703A8 size=24
    let mut pc: u32 = 0x824703A8;
    'dispatch: loop {
        match pc {
            0x824703A8 => {
    //   block [0x824703A8..0x824703C0)
	// 824703A8: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703AC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824703B0: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 824703B4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703B8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703C0 size=16
    let mut pc: u32 = 0x824703C0;
    'dispatch: loop {
        match pc {
            0x824703C0 => {
    //   block [0x824703C0..0x824703D0)
	// 824703C0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824703C4: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703C8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703D0 size=24
    let mut pc: u32 = 0x824703D0;
    'dispatch: loop {
        match pc {
            0x824703D0 => {
    //   block [0x824703D0..0x824703E8)
	// 824703D0: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703D4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824703D8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 824703DC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703E0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703E8 size=16
    let mut pc: u32 = 0x824703E8;
    'dispatch: loop {
        match pc {
            0x824703E8 => {
    //   block [0x824703E8..0x824703F8)
	// 824703E8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824703EC: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824703F0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824703F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824703F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824703F8 size=8
    let mut pc: u32 = 0x824703F8;
    'dispatch: loop {
        match pc {
            0x824703F8 => {
    //   block [0x824703F8..0x82470400)
	// 824703F8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824703FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470400 size=88
    let mut pc: u32 = 0x82470400;
    'dispatch: loop {
        match pc {
            0x82470400 => {
    //   block [0x82470400..0x82470458)
	// 82470400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470404: 480C4CB9  bl 0x825350bc
	ctx.lr = 0x82470408;
	sub_82535080(ctx, base);
	// 82470408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247040C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470414: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82470418: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247041C: 4BFFADBD  bl 0x8246b1d8
	ctx.lr = 0x82470420;
	sub_8246B1D8(ctx, base);
	// 82470420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82470424: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470428: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8247042C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470430: 4BFF66F9  bl 0x82466b28
	ctx.lr = 0x82470434;
	sub_82466B28(ctx, base);
	// 82470434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470438: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8247043C: 419A0010  beq cr6, 0x8247044c
	if ctx.cr[6].eq {
	pc = 0x8247044C; continue 'dispatch;
	}
	// 82470440: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82470444: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82470448: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8247044C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470454: 480C4CB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470458 size=12
    let mut pc: u32 = 0x82470458;
    'dispatch: loop {
        match pc {
            0x82470458 => {
    //   block [0x82470458..0x82470464)
	// 82470458: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8247045C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82470460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470468 size=20
    let mut pc: u32 = 0x82470468;
    'dispatch: loop {
        match pc {
            0x82470468 => {
    //   block [0x82470468..0x8247047C)
	// 82470468: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247046C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470470: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470474: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82470478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470480 size=80
    let mut pc: u32 = 0x82470480;
    'dispatch: loop {
        match pc {
            0x82470480 => {
    //   block [0x82470480..0x824704D0)
	// 82470480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8247048C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470494: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470498: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8247049C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824704A0: 4BFFAD39  bl 0x8246b1d8
	ctx.lr = 0x824704A4;
	sub_8246B1D8(ctx, base);
	// 824704A4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824704A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824704AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824704B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824704B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824704B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824704BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824704C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824704C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824704C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824704CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824704D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824704D0 size=120
    let mut pc: u32 = 0x824704D0;
    'dispatch: loop {
        match pc {
            0x824704D0 => {
    //   block [0x824704D0..0x82470548)
	// 824704D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824704D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824704D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824704DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824704E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824704E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824704E8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824704EC: 4BFF9E95  bl 0x8246a380
	ctx.lr = 0x824704F0;
	sub_8246A380(ctx, base);
	// 824704F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824704F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824704F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824704FC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82470500: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82470504: 409A0010  bne cr6, 0x82470514
	if !ctx.cr[6].eq {
	pc = 0x82470514; continue 'dispatch;
	}
	// 82470508: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8247050C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470510: 4BFFDE41  bl 0x8246e350
	ctx.lr = 0x82470514;
	sub_8246E350(ctx, base);
	// 82470514: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470518: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247051C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470520: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82470524: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470528: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8247052C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82470530: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8247053C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82470540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470548 size=124
    let mut pc: u32 = 0x82470548;
    'dispatch: loop {
        match pc {
            0x82470548 => {
    //   block [0x82470548..0x824705C4)
	// 82470548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247054C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470550: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82470554: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247055C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470560: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82470564: 4BFF9E1D  bl 0x8246a380
	ctx.lr = 0x82470568;
	sub_8246A380(ctx, base);
	// 82470568: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 8247056C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82470570: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82470574: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470578: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8247057C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82470580: 409A0010  bne cr6, 0x82470590
	if !ctx.cr[6].eq {
	pc = 0x82470590; continue 'dispatch;
	}
	// 82470584: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82470588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247058C: 4BFFDDC5  bl 0x8246e350
	ctx.lr = 0x82470590;
	sub_8246E350(ctx, base);
	// 82470590: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82470594: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470598: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247059C: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824705A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824705A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824705A8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824705AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824705B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824705B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824705B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824705BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824705C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824705C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824705C8 size=488
    let mut pc: u32 = 0x824705C8;
    'dispatch: loop {
        match pc {
            0x824705C8 => {
    //   block [0x824705C8..0x824707B0)
	// 824705C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824705CC: 480C4AE9  bl 0x825350b4
	ctx.lr = 0x824705D0;
	sub_82535080(ctx, base);
	// 824705D0: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824705D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824705D8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824705DC: 4BFF9C45  bl 0x8246a220
	ctx.lr = 0x824705E0;
	sub_8246A220(ctx, base);
	// 824705E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824705E4: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 824705E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824705EC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824705F0: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 824705F4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 824705F8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824705FC: 40990094  ble cr6, 0x82470690
	if !ctx.cr[6].gt {
	pc = 0x82470690; continue 'dispatch;
	}
	// 82470600: 3BDF0003  addi r30, r31, 3
	ctx.r[30].s64 = ctx.r[31].s64 + 3;
	// 82470604: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82470608: 4099001C  ble cr6, 0x82470624
	if !ctx.cr[6].gt {
	pc = 0x82470624; continue 'dispatch;
	}
	// 8247060C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82470610: 41980008  blt cr6, 0x82470618
	if ctx.cr[6].lt {
	pc = 0x82470618; continue 'dispatch;
	}
	// 82470614: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82470618: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8247061C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470620: 4BFFDCA9  bl 0x8246e2c8
	ctx.lr = 0x82470624;
	sub_8246E2C8(ctx, base);
	// 82470624: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82470628: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8247062C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82470630: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82470634: 4BFF9CF5  bl 0x8246a328
	ctx.lr = 0x82470638;
	sub_8246A328(ctx, base);
	// 82470638: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8247063C: 7FABF9AE  stbx r29, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u8) };
	// 82470640: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470644: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82470648: 894BFFFF  lbz r10, -1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8247064C: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 82470650: 419A0010  beq cr6, 0x82470660
	if ctx.cr[6].eq {
	pc = 0x82470660; continue 'dispatch;
	}
	// 82470654: 3940002F  li r10, 0x2f
	ctx.r[10].s64 = 47;
	// 82470658: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8247065C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82470660: 3920002A  li r9, 0x2a
	ctx.r[9].s64 = 42;
	// 82470664: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 82470668: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8247066C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82470670: 9BAB0001  stb r29, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[29].u8 ) };
	// 82470674: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470678: 816A4DA0  lwz r11, 0x4da0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19872 as u32) ) } as u64;
	// 8247067C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470680: 4E800421  bctrl
	ctx.lr = 0x82470684;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470684: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82470688: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247068C: 409A000C  bne cr6, 0x82470698
	if !ctx.cr[6].eq {
	pc = 0x82470698; continue 'dispatch;
	}
	// 82470690: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82470694: 48000008  b 0x8247069c
	pc = 0x8247069C; continue 'dispatch;
	// 82470698: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8247069C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824706A0: 480D67F9  bl 0x82546e98
	ctx.lr = 0x824706A4;
	sub_82546E98(ctx, base);
	// 824706A4: 546BE7FE  rlwinm r11, r3, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 824706A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824706AC: 409A003C  bne cr6, 0x824706e8
	if !ctx.cr[6].eq {
	pc = 0x824706E8; continue 'dispatch;
	}
	// 824706B0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824706B4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824706B8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824706BC: 409A0020  bne cr6, 0x824706dc
	if !ctx.cr[6].eq {
	pc = 0x824706DC; continue 'dispatch;
	}
	// 824706C0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824706C4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824706C8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824706CC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824706D0: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824706D4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824706D8: 4BFF39E1  bl 0x824640b8
	ctx.lr = 0x824706DC;
	sub_824640B8(ctx, base);
	// 824706DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824706E0: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 824706E4: 480C4A20  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824706E8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824706EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824706F0: 4BF5AE21  bl 0x823cb510
	ctx.lr = 0x824706F4;
	sub_823CB510(ctx, base);
	// 824706F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824706F8: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 824706FC: 419A0074  beq cr6, 0x82470770
	if ctx.cr[6].eq {
	pc = 0x82470770; continue 'dispatch;
	}
	// 82470700: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470704: 3BEB88FC  addi r31, r11, -0x7704
	ctx.r[31].s64 = ctx.r[11].s64 + -30468;
	// 82470708: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8247070C: 3BCB88F8  addi r30, r11, -0x7708
	ctx.r[30].s64 = ctx.r[11].s64 + -30472;
	// 82470710: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 82470714: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82470718: 4BFF98B9  bl 0x82469fd0
	ctx.lr = 0x8247071C;
	sub_82469FD0(ctx, base);
	// 8247071C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82470720: 419A003C  beq cr6, 0x8247075c
	if ctx.cr[6].eq {
	pc = 0x8247075C; continue 'dispatch;
	}
	// 82470724: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 82470728: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8247072C: 4BFF98A5  bl 0x82469fd0
	ctx.lr = 0x82470730;
	sub_82469FD0(ctx, base);
	// 82470730: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82470734: 419A0028  beq cr6, 0x8247075c
	if ctx.cr[6].eq {
	pc = 0x8247075C; continue 'dispatch;
	}
	// 82470738: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8247073C: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82470740: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82470744: 556BE7FE  rlwinm r11, r11, 0x1c, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82470748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247074C: 419A000C  beq cr6, 0x82470758
	if ctx.cr[6].eq {
	pc = 0x82470758; continue 'dispatch;
	}
	// 82470750: 4BFFFD81  bl 0x824704d0
	ctx.lr = 0x82470754;
	sub_824704D0(ctx, base);
	// 82470754: 48000008  b 0x8247075c
	pc = 0x8247075C; continue 'dispatch;
	// 82470758: 4BFFFDF1  bl 0x82470548
	ctx.lr = 0x8247075C;
	sub_82470548(ctx, base);
	// 8247075C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82470760: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470764: 4BF5AD55  bl 0x823cb4b8
	ctx.lr = 0x82470768;
	sub_823CB4B8(ctx, base);
	// 82470768: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8247076C: 409AFFA4  bne cr6, 0x82470710
	if !ctx.cr[6].eq {
	pc = 0x82470710; continue 'dispatch;
	}
	// 82470770: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470774: 4BF501E5  bl 0x823c0958
	ctx.lr = 0x82470778;
	sub_823C0958(ctx, base);
	// 82470778: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8247077C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82470780: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82470784: 409A0020  bne cr6, 0x824707a4
	if !ctx.cr[6].eq {
	pc = 0x824707A4; continue 'dispatch;
	}
	// 82470788: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247078C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82470790: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82470794: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470798: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8247079C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824707A0: 4BFF3919  bl 0x824640b8
	ctx.lr = 0x824707A4;
	sub_824640B8(ctx, base);
	// 824707A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824707A8: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 824707AC: 480C4958  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824707B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824707B0 size=104
    let mut pc: u32 = 0x824707B0;
    'dispatch: loop {
        match pc {
            0x824707B0 => {
    //   block [0x824707B0..0x82470818)
	// 824707B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824707B4: 480C4909  bl 0x825350bc
	ctx.lr = 0x824707B8;
	sub_82535080(ctx, base);
	// 824707B8: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824707BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824707C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824707C4: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 824707C8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824707CC: 419A0040  beq cr6, 0x8247080c
	if ctx.cr[6].eq {
	pc = 0x8247080C; continue 'dispatch;
	}
	// 824707D0: 2F1F0200  cmpwi cr6, r31, 0x200
	ctx.cr[6].compare_i32(ctx.r[31].s32, 512, &mut ctx.xer);
	// 824707D4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824707D8: 41990008  bgt cr6, 0x824707e0
	if ctx.cr[6].gt {
	pc = 0x824707E0; continue 'dispatch;
	}
	// 824707DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824707E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824707E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824707E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824707EC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824707F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824707F4: 4E800421  bctrl
	ctx.lr = 0x824707F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824707F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824707FC: 419A0010  beq cr6, 0x8247080c
	if ctx.cr[6].eq {
	pc = 0x8247080C; continue 'dispatch;
	}
	// 82470800: 7FE3F850  subf r31, r3, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82470804: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82470808: 409AFFC8  bne cr6, 0x824707d0
	if !ctx.cr[6].eq {
	pc = 0x824707D0; continue 'dispatch;
	}
	// 8247080C: 7C7FE850  subf r3, r31, r29
	ctx.r[3].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82470810: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82470814: 480C48F8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470818 size=88
    let mut pc: u32 = 0x82470818;
    'dispatch: loop {
        match pc {
            0x82470818 => {
    //   block [0x82470818..0x82470870)
	// 82470818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247081C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470824: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470828: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8247082C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82470830: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470838: 4E800421  bctrl
	ctx.lr = 0x8247083C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247083C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82470840: 419A001C  beq cr6, 0x8247085c
	if ctx.cr[6].eq {
	pc = 0x8247085C; continue 'dispatch;
	}
	// 82470844: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470848: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8247084C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470858: 4E800020  blr
	return;
	// 8247085C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82470860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8247086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470870 size=8
    let mut pc: u32 = 0x82470870;
    'dispatch: loop {
        match pc {
            0x82470870 => {
    //   block [0x82470870..0x82470878)
	// 82470870: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470878 size=8
    let mut pc: u32 = 0x82470878;
    'dispatch: loop {
        match pc {
            0x82470878 => {
    //   block [0x82470878..0x82470880)
	// 82470878: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8247087C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470880 size=8
    let mut pc: u32 = 0x82470880;
    'dispatch: loop {
        match pc {
            0x82470880 => {
    //   block [0x82470880..0x82470888)
	// 82470880: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470888 size=8
    let mut pc: u32 = 0x82470888;
    'dispatch: loop {
        match pc {
            0x82470888 => {
    //   block [0x82470888..0x82470890)
	// 82470888: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8247088C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470890 size=12
    let mut pc: u32 = 0x82470890;
    'dispatch: loop {
        match pc {
            0x82470890 => {
    //   block [0x82470890..0x8247089C)
	// 82470890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82470894: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82470898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824708A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824708A0 size=12
    let mut pc: u32 = 0x824708A0;
    'dispatch: loop {
        match pc {
            0x824708A0 => {
    //   block [0x824708A0..0x824708AC)
	// 824708A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824708A4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824708A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824708B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824708B0 size=4
    let mut pc: u32 = 0x824708B0;
    'dispatch: loop {
        match pc {
            0x824708B0 => {
    //   block [0x824708B0..0x824708B4)
	// 824708B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824708B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824708B8 size=32
    let mut pc: u32 = 0x824708B8;
    'dispatch: loop {
        match pc {
            0x824708B8 => {
    //   block [0x824708B8..0x824708D8)
	// 824708B8: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 824708BC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 824708C0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824708C4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824708C8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824708CC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824708D0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824708D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824708D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824708D8 size=64
    let mut pc: u32 = 0x824708D8;
    'dispatch: loop {
        match pc {
            0x824708D8 => {
    //   block [0x824708D8..0x82470918)
	// 824708D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824708DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824708E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824708E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824708E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824708EC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824708F0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824708F4: 419A0010  beq cr6, 0x82470904
	if ctx.cr[6].eq {
	pc = 0x82470904; continue 'dispatch;
	}
	// 824708F8: 4BF80A79  bl 0x823f1370
	ctx.lr = 0x824708FC;
	sub_823F1370(ctx, base);
	// 824708FC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82470900: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82470904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8247090C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470918 size=92
    let mut pc: u32 = 0x82470918;
    'dispatch: loop {
        match pc {
            0x82470918 => {
    //   block [0x82470918..0x82470974)
	// 82470918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247091C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8247092C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470930: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470938: 4E800421  bctrl
	ctx.lr = 0x8247093C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247093C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470940: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470944: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82470948: 4BF80A11  bl 0x823f1358
	ctx.lr = 0x8247094C;
	sub_823F1358(ctx, base);
	// 8247094C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82470950: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82470954: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82470958: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8247095C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82470960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8247096C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470978 size=100
    let mut pc: u32 = 0x82470978;
    'dispatch: loop {
        match pc {
            0x82470978 => {
    //   block [0x82470978..0x824709DC)
	// 82470978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470980: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470984: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470988: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8247098C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470990: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470994: 419A0030  beq cr6, 0x824709c4
	if ctx.cr[6].eq {
	pc = 0x824709C4; continue 'dispatch;
	}
	// 82470998: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8247099C: 4BF80AA5  bl 0x823f1440
	ctx.lr = 0x824709A0;
	sub_823F1440(ctx, base);
	// 824709A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824709A4: 4099000C  ble cr6, 0x824709b0
	if !ctx.cr[6].gt {
	pc = 0x824709B0; continue 'dispatch;
	}
	// 824709A8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824709AC: 409A001C  bne cr6, 0x824709c8
	if !ctx.cr[6].eq {
	pc = 0x824709C8; continue 'dispatch;
	}
	// 824709B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824709B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824709B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824709BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824709C0: 4E800421  bctrl
	ctx.lr = 0x824709C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824709C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824709C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824709CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824709D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824709D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824709D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824709E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824709E0 size=100
    let mut pc: u32 = 0x824709E0;
    'dispatch: loop {
        match pc {
            0x824709E0 => {
    //   block [0x824709E0..0x82470A44)
	// 824709E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824709E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824709E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824709EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824709F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824709F4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824709F8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824709FC: 419A0030  beq cr6, 0x82470a2c
	if ctx.cr[6].eq {
	pc = 0x82470A2C; continue 'dispatch;
	}
	// 82470A00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82470A04: 4BF80A75  bl 0x823f1478
	ctx.lr = 0x82470A08;
	sub_823F1478(ctx, base);
	// 82470A08: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82470A0C: 4099000C  ble cr6, 0x82470a18
	if !ctx.cr[6].gt {
	pc = 0x82470A18; continue 'dispatch;
	}
	// 82470A10: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470A14: 409A001C  bne cr6, 0x82470a30
	if !ctx.cr[6].eq {
	pc = 0x82470A30; continue 'dispatch;
	}
	// 82470A18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470A20: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470A24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470A28: 4E800421  bctrl
	ctx.lr = 0x82470A2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470A2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82470A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82470A48 size=24
    let mut pc: u32 = 0x82470A48;
    'dispatch: loop {
        match pc {
            0x82470A48 => {
    //   block [0x82470A48..0x82470A60)
	// 82470A48: 3964FFD0  addi r11, r4, -0x30
	ctx.r[11].s64 = ctx.r[4].s64 + -48;
	// 82470A4C: 216B0009  subfic r11, r11, 9
	ctx.xer.ca = ctx.r[11].u32 <= 9 as u32;
	ctx.r[11].s64 = (9 as i64) - ctx.r[11].s64;
	// 82470A50: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82470A54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82470A58: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82470A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82470A60 size=8
    let mut pc: u32 = 0x82470A60;
    'dispatch: loop {
        match pc {
            0x82470A60 => {
    //   block [0x82470A60..0x82470A68)
	// 82470A60: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470A68 size=220
    let mut pc: u32 = 0x82470A68;
    'dispatch: loop {
        match pc {
            0x82470A68 => {
    //   block [0x82470A68..0x82470B44)
	// 82470A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470A74: 9421FBF0  stwu r1, -0x410(r1)
	ea = ctx.r[1].u32.wrapping_add(-1040 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470A78: 3FE08293  lis r31, -0x7d6d
	ctx.r[31].s64 = -2104295424;
	// 82470A7C: 897F91E4  lbz r11, -0x6e1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-28188 as u32) ) } as u64;
	// 82470A80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82470A84: 409A00AC  bne cr6, 0x82470b30
	if !ctx.cr[6].eq {
	pc = 0x82470B30; continue 'dispatch;
	}
	// 82470A88: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82470A8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82470A90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470A94: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82470A98: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82470A9C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82470AA0: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82470AA4: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82470AA8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82470AAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82470AB0: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 82470AB4: 4BF80765  bl 0x823f1218
	ctx.lr = 0x82470AB8;
	sub_823F1218(ctx, base);
	// 82470AB8: 38810270  addi r4, r1, 0x270
	ctx.r[4].s64 = ctx.r[1].s64 + 624;
	// 82470ABC: 38600202  li r3, 0x202
	ctx.r[3].s64 = 514;
	// 82470AC0: 4BF80889  bl 0x823f1348
	ctx.lr = 0x82470AC4;
	sub_823F1348(ctx, base);
	// 82470AC4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470AC8: 409A0060  bne cr6, 0x82470b28
	if !ctx.cr[6].eq {
	pc = 0x82470B28; continue 'dispatch;
	}
	// 82470ACC: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82470AD0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82470AD4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470AD8: 4BFF8321  bl 0x82468df8
	ctx.lr = 0x82470ADC;
	sub_82468DF8(ctx, base);
	// 82470ADC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470AE0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470AE4: 388B8928  addi r4, r11, -0x76d8
	ctx.r[4].s64 = ctx.r[11].s64 + -30424;
	// 82470AE8: 4BFF77B9  bl 0x824682a0
	ctx.lr = 0x82470AEC;
	sub_824682A0(ctx, base);
	// 82470AEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82470AF0: 3CA03218  lis r5, 0x3218
	ctx.r[5].s64 = 840433664;
	// 82470AF4: 39000048  li r8, 0x48
	ctx.r[8].s64 = 72;
	// 82470AF8: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82470AFC: 60A525F8  ori r5, r5, 0x25f8
	ctx.r[5].u64 = ctx.r[5].u64 | 9720;
	// 82470B00: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82470B04: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470B08: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82470B0C: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 82470B10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470B14: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82470B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470B1C: 4E800421  bctrl
	ctx.lr = 0x82470B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470B20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470B24: 4BFF7DBD  bl 0x824688e0
	ctx.lr = 0x82470B28;
	sub_824688E0(ctx, base);
	// 82470B28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82470B2C: 997F91E4  stb r11, -0x6e1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-28188 as u32), ctx.r[11].u8 ) };
	// 82470B30: 38210410  addi r1, r1, 0x410
	ctx.r[1].s64 = ctx.r[1].s64 + 1040;
	// 82470B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470B3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470B48 size=96
    let mut pc: u32 = 0x82470B48;
    'dispatch: loop {
        match pc {
            0x82470B48 => {
    //   block [0x82470B48..0x82470BA8)
	// 82470B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470B5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470B60: 396B8954  addi r11, r11, -0x76ac
	ctx.r[11].s64 = ctx.r[11].s64 + -30380;
	// 82470B64: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470B68: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470B6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470B70: 419A0010  beq cr6, 0x82470b80
	if ctx.cr[6].eq {
	pc = 0x82470B80; continue 'dispatch;
	}
	// 82470B74: 4BF807FD  bl 0x823f1370
	ctx.lr = 0x82470B78;
	sub_823F1370(ctx, base);
	// 82470B78: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82470B7C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82470B80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82470B84: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82470B88: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82470B8C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82470B90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82470B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82470BA8 size=316
    let mut pc: u32 = 0x82470BA8;
    'dispatch: loop {
        match pc {
            0x82470BA8 => {
    //   block [0x82470BA8..0x82470CE4)
	// 82470BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470BAC: 480C4511  bl 0x825350bc
	ctx.lr = 0x82470BB0;
	sub_82535080(ctx, base);
	// 82470BB0: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470BB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82470BB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82470BBC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82470BC0: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82470BC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82470BC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470BCC: 4BFF976D  bl 0x8246a338
	ctx.lr = 0x82470BD0;
	sub_8246A338(ctx, base);
	// 82470BD0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470BD4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82470BD8: B3C10052  sth r30, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[30].u16 ) };
	// 82470BDC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82470BE0: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 82470BE4: 216B0009  subfic r11, r11, 9
	ctx.xer.ca = ctx.r[11].u32 <= 9 as u32;
	ctx.r[11].s64 = (9 as i64) - ctx.r[11].s64;
	// 82470BE8: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82470BEC: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82470BF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82470BF4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82470BF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82470BFC: 419A0014  beq cr6, 0x82470c10
	if ctx.cr[6].eq {
	pc = 0x82470C10; continue 'dispatch;
	}
	// 82470C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470C04: 4BF808AD  bl 0x823f14b0
	ctx.lr = 0x82470C08;
	sub_823F14B0(ctx, base);
	// 82470C08: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82470C0C: 4800005C  b 0x82470c68
	pc = 0x82470C68; continue 'dispatch;
	// 82470C10: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82470C14: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82470C18: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470C1C: 4BFF81DD  bl 0x82468df8
	ctx.lr = 0x82470C20;
	sub_82468DF8(ctx, base);
	// 82470C20: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470C24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470C28: 388B8980  addi r4, r11, -0x7680
	ctx.r[4].s64 = ctx.r[11].s64 + -30336;
	// 82470C2C: 4BFF7675  bl 0x824682a0
	ctx.lr = 0x82470C30;
	sub_824682A0(ctx, base);
	// 82470C30: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82470C34: 390000C3  li r8, 0xc3
	ctx.r[8].s64 = 195;
	// 82470C38: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82470C3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470C40: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82470C44: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82470C48: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470C4C: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 82470C50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470C54: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82470C58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470C5C: 4E800421  bctrl
	ctx.lr = 0x82470C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470C60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470C64: 4BFF7C7D  bl 0x824688e0
	ctx.lr = 0x82470C68;
	sub_824688E0(ctx, base);
	// 82470C68: 807D0020  lwz r3, 0x20(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470C6C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470C70: 409A0034  bne cr6, 0x82470ca4
	if !ctx.cr[6].eq {
	pc = 0x82470CA4; continue 'dispatch;
	}
	// 82470C74: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470C78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470C7C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470C84: 4E800421  bctrl
	ctx.lr = 0x82470C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470C88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470C8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470C90: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82470C94: 4BF806C5  bl 0x823f1358
	ctx.lr = 0x82470C98;
	sub_823F1358(ctx, base);
	// 82470C98: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470C9C: 907D0020  stw r3, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82470CA0: 419A002C  beq cr6, 0x82470ccc
	if ctx.cr[6].eq {
	pc = 0x82470CCC; continue 'dispatch;
	}
	// 82470CA4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82470CA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82470CAC: 4BF80735  bl 0x823f13e0
	ctx.lr = 0x82470CB0;
	sub_823F13E0(ctx, base);
	// 82470CB0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470CB4: 409A0024  bne cr6, 0x82470cd8
	if !ctx.cr[6].eq {
	pc = 0x82470CD8; continue 'dispatch;
	}
	// 82470CB8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470CBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82470CC0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470CC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470CC8: 4E800421  bctrl
	ctx.lr = 0x82470CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470CCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470CD0: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82470CD4: 480C4438  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82470CD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82470CDC: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82470CE0: 480C442C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470CE8 size=124
    let mut pc: u32 = 0x82470CE8;
    'dispatch: loop {
        match pc {
            0x82470CE8 => {
    //   block [0x82470CE8..0x82470D64)
	// 82470CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82470CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82470CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82470CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470D00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82470D04: 4BFFEC2D  bl 0x8246f930
	ctx.lr = 0x82470D08;
	sub_8246F930(ctx, base);
	// 82470D08: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470D0C: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82470D10: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82470D14: 396B8954  addi r11, r11, -0x76ac
	ctx.r[11].s64 = ctx.r[11].s64 + -30380;
	// 82470D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470D1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82470D20: 409A002C  bne cr6, 0x82470d4c
	if !ctx.cr[6].eq {
	pc = 0x82470D4C; continue 'dispatch;
	}
	// 82470D24: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470D28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470D2C: 4E800421  bctrl
	ctx.lr = 0x82470D30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470D30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470D34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470D38: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82470D3C: 4BF8061D  bl 0x823f1358
	ctx.lr = 0x82470D40;
	sub_823F1358(ctx, base);
	// 82470D40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82470D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470D48: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82470D4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82470D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82470D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82470D58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82470D5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82470D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470D68 size=616
    let mut pc: u32 = 0x82470D68;
    'dispatch: loop {
        match pc {
            0x82470D68 => {
    //   block [0x82470D68..0x82470FD0)
	// 82470D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470D6C: 480C4345  bl 0x825350b0
	ctx.lr = 0x82470D70;
	sub_82535080(ctx, base);
	// 82470D70: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470D78: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82470D7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470D80: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470D84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470D88: 4E800421  bctrl
	ctx.lr = 0x82470D8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82470D90: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470D94: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82470D98: 4BF805C1  bl 0x823f1358
	ctx.lr = 0x82470D9C;
	sub_823F1358(ctx, base);
	// 82470D9C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470DA0: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82470DA4: 419A0064  beq cr6, 0x82470e08
	if ctx.cr[6].eq {
	pc = 0x82470E08; continue 'dispatch;
	}
	// 82470DA8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82470DAC: B38100A2  sth r28, 0xa2(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(162 as u32), ctx.r[28].u16 ) };
	// 82470DB0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82470DB4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82470DB8: 3C800000  lis r4, 0
	ctx.r[4].s64 = 0;
	// 82470DBC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82470DC0: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82470DC4: B16100A0  sth r11, 0xa0(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u16 ) };
	// 82470DC8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82470DCC: 93A100A4  stw r29, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 82470DD0: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 82470DD4: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82470DD8: 4BF805D1  bl 0x823f13a8
	ctx.lr = 0x82470DDC;
	sub_823F13A8(ctx, base);
	// 82470DDC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82470DE0: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82470DE4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470DE8: 4BF805E1  bl 0x823f13c8
	ctx.lr = 0x82470DEC;
	sub_823F13C8(ctx, base);
	// 82470DEC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470DF0: 409A0024  bne cr6, 0x82470e14
	if !ctx.cr[6].eq {
	pc = 0x82470E14; continue 'dispatch;
	}
	// 82470DF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470DF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82470DFC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82470E00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470E04: 4E800421  bctrl
	ctx.lr = 0x82470E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470E08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82470E0C: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82470E10: 480C42F0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82470E14: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82470E18: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470E1C: 4BF805DD  bl 0x823f13f8
	ctx.lr = 0x82470E20;
	sub_823F13F8(ctx, base);
	// 82470E20: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82470E24: 419AFFD0  beq cr6, 0x82470df4
	if ctx.cr[6].eq {
	pc = 0x82470DF4; continue 'dispatch;
	}
	// 82470E28: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470E2C: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82470E30: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82470E34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82470E38: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82470E3C: 4BFF31FD  bl 0x82464038
	ctx.lr = 0x82470E40;
	sub_82464038(ctx, base);
	// 82470E40: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82470E44: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82470E48: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82470E4C: 9BA30000  stb r29, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82470E50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82470E54: 4BF8043D  bl 0x823f1290
	ctx.lr = 0x82470E58;
	sub_823F1290(ctx, base);
	// 82470E58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82470E5C: 419AFFF4  beq cr6, 0x82470e50
	if ctx.cr[6].eq {
	pc = 0x82470E50; continue 'dispatch;
	}
	// 82470E60: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82470E64: 409A0038  bne cr6, 0x82470e9c
	if !ctx.cr[6].eq {
	pc = 0x82470E9C; continue 'dispatch;
	}
	// 82470E68: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82470E6C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82470E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82470E74: 419A0058  beq cr6, 0x82470ecc
	if ctx.cr[6].eq {
	pc = 0x82470ECC; continue 'dispatch;
	}
	// 82470E78: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470E7C: 89010067  lbz r8, 0x67(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(103 as u32) ) } as u64;
	// 82470E80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470E84: 88E10066  lbz r7, 0x66(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 82470E88: 388B89C4  addi r4, r11, -0x763c
	ctx.r[4].s64 = ctx.r[11].s64 + -30268;
	// 82470E8C: 88C10065  lbz r6, 0x65(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(101 as u32) ) } as u64;
	// 82470E90: 88A10064  lbz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82470E94: 4BFF96CD  bl 0x8246a560
	ctx.lr = 0x82470E98;
	sub_8246A560(ctx, base);
	// 82470E98: 4800008C  b 0x82470f24
	pc = 0x82470F24; continue 'dispatch;
	// 82470E9C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82470EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82470EA4: 419AFFC8  beq cr6, 0x82470e6c
	if ctx.cr[6].eq {
	pc = 0x82470E6C; continue 'dispatch;
	}
	// 82470EA8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470EAC: 89010063  lbz r8, 0x63(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(99 as u32) ) } as u64;
	// 82470EB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470EB4: 88E10062  lbz r7, 0x62(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(98 as u32) ) } as u64;
	// 82470EB8: 388B89C4  addi r4, r11, -0x763c
	ctx.r[4].s64 = ctx.r[11].s64 + -30268;
	// 82470EBC: 88C10061  lbz r6, 0x61(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(97 as u32) ) } as u64;
	// 82470EC0: 88A10060  lbz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82470EC4: 4BFF969D  bl 0x8246a560
	ctx.lr = 0x82470EC8;
	sub_8246A560(ctx, base);
	// 82470EC8: 4800005C  b 0x82470f24
	pc = 0x82470F24; continue 'dispatch;
	// 82470ECC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82470ED0: 3BCBCD5C  addi r30, r11, -0x32a4
	ctx.r[30].s64 = ctx.r[11].s64 + -12964;
	// 82470ED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82470ED8: 4BFF9349  bl 0x8246a220
	ctx.lr = 0x82470EDC;
	sub_8246A220(ctx, base);
	// 82470EDC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82470EE0: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82470EE4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82470EE8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82470EEC: 40980024  bge cr6, 0x82470f10
	if !ctx.cr[6].lt {
	pc = 0x82470F10; continue 'dispatch;
	}
	// 82470EF0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82470EF4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82470EF8: 41980008  blt cr6, 0x82470f00
	if ctx.cr[6].lt {
	pc = 0x82470F00; continue 'dispatch;
	}
	// 82470EFC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82470F00: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82470F04: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82470F08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82470F0C: 4BFFD3BD  bl 0x8246e2c8
	ctx.lr = 0x82470F10;
	sub_8246E2C8(ctx, base);
	// 82470F10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82470F14: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470F18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82470F1C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82470F20: 4BFF9409  bl 0x8246a328
	ctx.lr = 0x82470F24;
	sub_8246A328(ctx, base);
	// 82470F24: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82470F28: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82470F2C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82470F30: 4BFF7EC9  bl 0x82468df8
	ctx.lr = 0x82470F34;
	sub_82468DF8(ctx, base);
	// 82470F34: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470F38: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82470F3C: 388B89B0  addi r4, r11, -0x7650
	ctx.r[4].s64 = ctx.r[11].s64 + -30288;
	// 82470F40: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470F44: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82470F48: 3BEB89A8  addi r31, r11, -0x7658
	ctx.r[31].s64 = ctx.r[11].s64 + -30296;
	// 82470F4C: 4BFF7355  bl 0x824682a0
	ctx.lr = 0x82470F50;
	sub_824682A0(ctx, base);
	// 82470F50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82470F54: 4BFF7AA5  bl 0x824689f8
	ctx.lr = 0x82470F58;
	sub_824689F8(ctx, base);
	// 82470F58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82470F5C: 4BFF7345  bl 0x824682a0
	ctx.lr = 0x82470F60;
	sub_824682A0(ctx, base);
	// 82470F60: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82470F64: 4BFF7475  bl 0x824683d8
	ctx.lr = 0x82470F68;
	sub_824683D8(ctx, base);
	// 82470F68: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82470F6C: 3900016C  li r8, 0x16c
	ctx.r[8].s64 = 364;
	// 82470F70: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82470F74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82470F78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82470F7C: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82470F80: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82470F84: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 82470F88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82470F8C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82470F90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82470F94: 4E800421  bctrl
	ctx.lr = 0x82470F98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82470F98: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82470F9C: 4BFF7945  bl 0x824688e0
	ctx.lr = 0x82470FA0;
	sub_824688E0(ctx, base);
	// 82470FA0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82470FA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82470FA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82470FAC: 409A0018  bne cr6, 0x82470fc4
	if !ctx.cr[6].eq {
	pc = 0x82470FC4; continue 'dispatch;
	}
	// 82470FB0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82470FB4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82470FB8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82470FBC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82470FC0: 4BFF30F9  bl 0x824640b8
	ctx.lr = 0x82470FC4;
	sub_824640B8(ctx, base);
	// 82470FC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82470FC8: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82470FCC: 480C4134  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82470FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82470FD0 size=468
    let mut pc: u32 = 0x82470FD0;
    'dispatch: loop {
        match pc {
            0x82470FD0 => {
    //   block [0x82470FD0..0x824711A4)
	// 82470FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82470FD4: 480C40E1  bl 0x825350b4
	ctx.lr = 0x82470FD8;
	sub_82535080(ctx, base);
	// 82470FD8: 9421FB20  stwu r1, -0x4e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1248 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82470FDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82470FE0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82470FE4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82470FE8: 419A0134  beq cr6, 0x8247111c
	if ctx.cr[6].eq {
	pc = 0x8247111C; continue 'dispatch;
	}
	// 82470FEC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82470FF0: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82470FF4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82470FF8: 916101A4  stw r11, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[11].u32 ) };
	// 82470FFC: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82471000: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 82471004: 38C101A0  addi r6, r1, 0x1a0
	ctx.r[6].s64 = ctx.r[1].s64 + 416;
	// 82471008: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8247100C: 93C10090  stw r30, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82471010: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82471014: 93C101A0  stw r30, 0x1a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(416 as u32), ctx.r[30].u32 ) };
	// 82471018: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8247101C: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82471020: 4BF80401  bl 0x823f1420
	ctx.lr = 0x82471024;
	sub_823F1420(ctx, base);
	// 82471024: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471028: 409900F4  ble cr6, 0x8247111c
	if !ctx.cr[6].gt {
	pc = 0x8247111C; continue 'dispatch;
	}
	// 8247102C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82471030: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471034: 4BF8048D  bl 0x823f14c0
	ctx.lr = 0x82471038;
	sub_823F14C0(ctx, base);
	// 82471038: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8247103C: 419A00E0  beq cr6, 0x8247111c
	if ctx.cr[6].eq {
	pc = 0x8247111C; continue 'dispatch;
	}
	// 82471040: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82471044: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471048: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8247104C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82471050: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82471054: 4BF803B5  bl 0x823f1408
	ctx.lr = 0x82471058;
	sub_823F1408(ctx, base);
	// 82471058: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247105C: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82471060: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82471064: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82471068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8247106C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82471070: 4BFF2FC9  bl 0x82464038
	ctx.lr = 0x82471074;
	sub_82464038(ctx, base);
	// 82471074: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471078: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8247107C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82471080: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82471084: 388B89D0  addi r4, r11, -0x7630
	ctx.r[4].s64 = ctx.r[11].s64 + -30256;
	// 82471088: 9B630000  stb r27, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8247108C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82471090: A0C10082  lhz r6, 0x82(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(130 as u32) ) } as u64;
	// 82471094: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82471098: 4BFF94C9  bl 0x8246a560
	ctx.lr = 0x8247109C;
	sub_8246A560(ctx, base);
	// 8247109C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824710A0: 388102B0  addi r4, r1, 0x2b0
	ctx.r[4].s64 = ctx.r[1].s64 + 688;
	// 824710A4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824710A8: 4BFF7D51  bl 0x82468df8
	ctx.lr = 0x824710AC;
	sub_82468DF8(ctx, base);
	// 824710AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824710B0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824710B4: 4BFF7945  bl 0x824689f8
	ctx.lr = 0x824710B8;
	sub_824689F8(ctx, base);
	// 824710B8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824710BC: 39000194  li r8, 0x194
	ctx.r[8].s64 = 404;
	// 824710C0: 38C102B0  addi r6, r1, 0x2b0
	ctx.r[6].s64 = ctx.r[1].s64 + 688;
	// 824710C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 824710C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824710CC: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824710D0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824710D4: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 824710D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824710DC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824710E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824710E4: 4E800421  bctrl
	ctx.lr = 0x824710E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824710E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824710EC: 4BFF77F5  bl 0x824688e0
	ctx.lr = 0x824710F0;
	sub_824688E0(ctx, base);
	// 824710F0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 824710F4: 409A0034  bne cr6, 0x82471128
	if !ctx.cr[6].eq {
	pc = 0x82471128; continue 'dispatch;
	}
	// 824710F8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824710FC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82471100: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471104: 409A0018  bne cr6, 0x8247111c
	if !ctx.cr[6].eq {
	pc = 0x8247111C; continue 'dispatch;
	}
	// 82471108: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8247110C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82471110: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82471114: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82471118: 4BFF2FA1  bl 0x824640b8
	ctx.lr = 0x8247111C;
	sub_824640B8(ctx, base);
	// 8247111C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471120: 382104E0  addi r1, r1, 0x4e0
	ctx.r[1].s64 = ctx.r[1].s64 + 1248;
	// 82471124: 480C3FE0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82471128: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8247112C: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82471130: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82471134: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82471138: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8247113C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471140: 4BF80269  bl 0x823f13a8
	ctx.lr = 0x82471144;
	sub_823F13A8(ctx, base);
	// 82471144: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82471148: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8247114C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82471150: 4BFF2EE9  bl 0x82464038
	ctx.lr = 0x82471154;
	sub_82464038(ctx, base);
	// 82471154: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82471158: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 8247115C: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82471160: 4BFFE7D1  bl 0x8246f930
	ctx.lr = 0x82471164;
	sub_8246F930(ctx, base);
	// 82471164: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471168: 93FE0020  stw r31, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8247116C: 396B8954  addi r11, r11, -0x76ac
	ctx.r[11].s64 = ctx.r[11].s64 + -30380;
	// 82471170: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82471174: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82471178: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8247117C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471180: 409A0018  bne cr6, 0x82471198
	if !ctx.cr[6].eq {
	pc = 0x82471198; continue 'dispatch;
	}
	// 82471184: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82471188: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8247118C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82471190: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82471194: 4BFF2F25  bl 0x824640b8
	ctx.lr = 0x82471198;
	sub_824640B8(ctx, base);
	// 82471198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247119C: 382104E0  addi r1, r1, 0x4e0
	ctx.r[1].s64 = ctx.r[1].s64 + 1248;
	// 824711A0: 480C3F64  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824711A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824711A8 size=152
    let mut pc: u32 = 0x824711A8;
    'dispatch: loop {
        match pc {
            0x824711A8 => {
    //   block [0x824711A8..0x82471240)
	// 824711A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824711AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824711B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824711B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824711B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824711BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824711C0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824711C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824711C8: 396B8954  addi r11, r11, -0x76ac
	ctx.r[11].s64 = ctx.r[11].s64 + -30380;
	// 824711CC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824711D0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824711D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824711D8: 419A0010  beq cr6, 0x824711e8
	if ctx.cr[6].eq {
	pc = 0x824711E8; continue 'dispatch;
	}
	// 824711DC: 4BF80195  bl 0x823f1370
	ctx.lr = 0x824711E0;
	sub_823F1370(ctx, base);
	// 824711E0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824711E4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824711E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824711EC: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824711F0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824711F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824711F8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824711FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82471200: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82471204: 419A0020  beq cr6, 0x82471224
	if ctx.cr[6].eq {
	pc = 0x82471224; continue 'dispatch;
	}
	// 82471208: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247120C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82471210: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82471214: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471218: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8247121C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471220: 4BFF2E99  bl 0x824640b8
	ctx.lr = 0x82471224;
	sub_824640B8(ctx, base);
	// 82471224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8247122C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82471230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82471234: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82471238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8247123C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471240 size=12
    let mut pc: u32 = 0x82471240;
    'dispatch: loop {
        match pc {
            0x82471240 => {
    //   block [0x82471240..0x8247124C)
	// 82471240: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82471244: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471248: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8247124C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8247124C size=36
    let mut pc: u32 = 0x8247124C;
    'dispatch: loop {
        match pc {
            0x8247124C => {
    //   block [0x8247124C..0x82471270)
	// 8247124C: 3963002C  addi r11, r3, 0x2c
	ctx.r[11].s64 = ctx.r[3].s64 + 44;
	// 82471250: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82471254: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82471258: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8247125C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82471260: 912BFFF8  stw r9, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 82471264: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82471268: 409AFFE8  bne cr6, 0x82471250
	if !ctx.cr[6].eq {
	pc = 0x82471250; continue 'dispatch;
	}
	// 8247126C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471270 size=16
    let mut pc: u32 = 0x82471270;
    'dispatch: loop {
        match pc {
            0x82471270 => {
    //   block [0x82471270..0x82471280)
	// 82471270: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82471274: 409A000C  bne cr6, 0x82471280
	if !ctx.cr[6].eq {
		sub_82471280(ctx, base);
		return;
	}
	// 82471278: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8247127C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82471280 size=96
    let mut pc: u32 = 0x82471280;
    'dispatch: loop {
        match pc {
            0x82471280 => {
    //   block [0x82471280..0x824712E0)
	// 82471280: 7C8B1670  srawi r11, r4, 2
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 2) as i64;
	// 82471284: 0CC50000  twi 6, r5, 0
	// 82471288: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8247128C: 7D2B2BD6  divw r9, r11, r5
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[5].s32;
	// 82471290: 556A083E  rotlwi r10, r11, 1
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82471294: 7D2929D6  mullw r9, r9, r5
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82471298: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8247129C: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 824712A0: 7CAA5078  andc r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 & !ctx.r[10].u64;
	// 824712A4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 824712A8: 0CAAFFFF  twi 5, r10, -1
	// 824712AC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824712B0: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 824712B4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824712B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824712BC: 4098001C  bge cr6, 0x824712d8
	if !ctx.cr[6].lt {
	pc = 0x824712D8; continue 'dispatch;
	}
	// 824712C0: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824712C4: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 824712C8: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 824712CC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824712D0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824712D4: 4198FFF0  blt cr6, 0x824712c4
	if ctx.cr[6].lt {
	pc = 0x824712C4; continue 'dispatch;
	}
	// 824712D8: 7C6B2A14  add r3, r11, r5
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 824712DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824712E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824712E0 size=20
    let mut pc: u32 = 0x824712E0;
    'dispatch: loop {
        match pc {
            0x824712E0 => {
    //   block [0x824712E0..0x824712F4)
	// 824712E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824712E4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 824712E8: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824712EC: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 824712F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824712F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824712F4 size=24
    let mut pc: u32 = 0x824712F4;
    'dispatch: loop {
        match pc {
            0x824712F4 => {
    //   block [0x824712F4..0x8247130C)
	// 824712F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824712F8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824712FC: 409A0010  bne cr6, 0x8247130c
	if !ctx.cr[6].eq {
		sub_8247130C(ctx, base);
		return;
	}
	// 82471300: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471304: 386B8A38  addi r3, r11, -0x75c8
	ctx.r[3].s64 = ctx.r[11].s64 + -30152;
	// 82471308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8247130C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8247130C size=20
    let mut pc: u32 = 0x8247130C;
    'dispatch: loop {
        match pc {
            0x8247130C => {
    //   block [0x8247130C..0x82471320)
	// 8247130C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82471310: 409A0010  bne cr6, 0x82471320
	if !ctx.cr[6].eq {
		sub_82471320(ctx, base);
		return;
	}
	// 82471314: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471318: 386B8A2C  addi r3, r11, -0x75d4
	ctx.r[3].s64 = ctx.r[11].s64 + -30164;
	// 8247131C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471320 size=8
    let mut pc: u32 = 0x82471320;
    'dispatch: loop {
        match pc {
            0x82471320 => {
    //   block [0x82471320..0x82471328)
	// 82471320: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471328 size=128
    let mut pc: u32 = 0x82471328;
    'dispatch: loop {
        match pc {
            0x82471328 => {
    //   block [0x82471328..0x824713A8)
	// 82471328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247132C: 480C3D8D  bl 0x825350b8
	ctx.lr = 0x82471330;
	sub_82535080(ctx, base);
	// 82471330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471334: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82471338: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8247133C: 3BAB5128  addi r29, r11, 0x5128
	ctx.r[29].s64 = ctx.r[11].s64 + 20776;
	// 82471340: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82471344: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247134C: 419A0044  beq cr6, 0x82471390
	if ctx.cr[6].eq {
	pc = 0x82471390; continue 'dispatch;
	}
	// 82471350: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82471354: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247135C: 4BFF8EC5  bl 0x8246a220
	ctx.lr = 0x82471360;
	sub_8246A220(ctx, base);
	// 82471360: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82471364: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82471368: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8247136C: 4BFF8C95  bl 0x8246a000
	ctx.lr = 0x82471370;
	sub_8246A000(ctx, base);
	// 82471370: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471374: 419A0028  beq cr6, 0x8247139c
	if ctx.cr[6].eq {
	pc = 0x8247139C; continue 'dispatch;
	}
	// 82471378: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8247137C: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471380: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82471384: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471388: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8247138C: 409AFFC8  bne cr6, 0x82471354
	if !ctx.cr[6].eq {
	pc = 0x82471354; continue 'dispatch;
	}
	// 82471390: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82471398: 480C3D70  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8247139C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824713A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824713A4: 480C3D64  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824713A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824713A8 size=148
    let mut pc: u32 = 0x824713A8;
    'dispatch: loop {
        match pc {
            0x824713A8 => {
    //   block [0x824713A8..0x8247143C)
	// 824713A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824713AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824713B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824713B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824713B8: 4BFF54A1  bl 0x82466858
	ctx.lr = 0x824713BC;
	sub_82466858(ctx, base);
	// 824713BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824713C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824713C4: 388B7468  addi r4, r11, 0x7468
	ctx.r[4].s64 = ctx.r[11].s64 + 29800;
	// 824713C8: 4BFF8C09  bl 0x82469fd0
	ctx.lr = 0x824713CC;
	sub_82469FD0(ctx, base);
	// 824713CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824713D0: 419A0050  beq cr6, 0x82471420
	if ctx.cr[6].eq {
	pc = 0x82471420; continue 'dispatch;
	}
	// 824713D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824713D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824713DC: 388B72F4  addi r4, r11, 0x72f4
	ctx.r[4].s64 = ctx.r[11].s64 + 29428;
	// 824713E0: 4BFF8BF1  bl 0x82469fd0
	ctx.lr = 0x824713E4;
	sub_82469FD0(ctx, base);
	// 824713E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824713E8: 419A0038  beq cr6, 0x82471420
	if ctx.cr[6].eq {
	pc = 0x82471420; continue 'dispatch;
	}
	// 824713EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824713F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824713F4: 388B7898  addi r4, r11, 0x7898
	ctx.r[4].s64 = ctx.r[11].s64 + 30872;
	// 824713F8: 4BFF8BD9  bl 0x82469fd0
	ctx.lr = 0x824713FC;
	sub_82469FD0(ctx, base);
	// 824713FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471400: 419A0020  beq cr6, 0x82471420
	if ctx.cr[6].eq {
	pc = 0x82471420; continue 'dispatch;
	}
	// 82471404: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82471408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247140C: 388B7888  addi r4, r11, 0x7888
	ctx.r[4].s64 = ctx.r[11].s64 + 30856;
	// 82471410: 4BFF8BC1  bl 0x82469fd0
	ctx.lr = 0x82471414;
	sub_82469FD0(ctx, base);
	// 82471414: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471418: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8247141C: 409A0008  bne cr6, 0x82471424
	if !ctx.cr[6].eq {
	pc = 0x82471424; continue 'dispatch;
	}
	// 82471420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82471424: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82471428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8247142C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82471430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82471434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82471438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471440 size=8
    let mut pc: u32 = 0x82471440;
    'dispatch: loop {
        match pc {
            0x82471440 => {
    //   block [0x82471440..0x82471448)
	// 82471440: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471448 size=8
    let mut pc: u32 = 0x82471448;
    'dispatch: loop {
        match pc {
            0x82471448 => {
    //   block [0x82471448..0x82471450)
	// 82471448: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247144C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471450 size=12
    let mut pc: u32 = 0x82471450;
    'dispatch: loop {
        match pc {
            0x82471450 => {
    //   block [0x82471450..0x8247145C)
	// 82471450: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471454: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471460 size=24
    let mut pc: u32 = 0x82471460;
    'dispatch: loop {
        match pc {
            0x82471460 => {
    //   block [0x82471460..0x82471478)
	// 82471460: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471464: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471468: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8247146C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471470: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471478 size=116
    let mut pc: u32 = 0x82471478;
    'dispatch: loop {
        match pc {
            0x82471478 => {
    //   block [0x82471478..0x824714EC)
	// 82471478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247147C: 480C3C3D  bl 0x825350b8
	ctx.lr = 0x82471480;
	sub_82535080(ctx, base);
	// 82471480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471484: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82471488: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8247148C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82471490: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471494: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471498: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247149C: 40990038  ble cr6, 0x824714d4
	if !ctx.cr[6].gt {
	pc = 0x824714D4; continue 'dispatch;
	}
	// 824714A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824714A4: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 824714A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824714AC: 7C7F5A14  add r3, r31, r11
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 824714B0: 4BFF8B21  bl 0x82469fd0
	ctx.lr = 0x824714B4;
	sub_82469FD0(ctx, base);
	// 824714B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824714B8: 419A0028  beq cr6, 0x824714e0
	if ctx.cr[6].eq {
	pc = 0x824714E0; continue 'dispatch;
	}
	// 824714BC: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 824714C0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824714C4: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 824714C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824714CC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824714D0: 4198FFD4  blt cr6, 0x824714a4
	if ctx.cr[6].lt {
	pc = 0x824714A4; continue 'dispatch;
	}
	// 824714D4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824714D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824714DC: 480C3C2C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824714E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824714E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824714E8: 480C3C20  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824714F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824714F0 size=140
    let mut pc: u32 = 0x824714F0;
    'dispatch: loop {
        match pc {
            0x824714F0 => {
    //   block [0x824714F0..0x8247157C)
	// 824714F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824714F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824714F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824714FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82471500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82471508: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247150C: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471510: 80AB001C  lwz r5, 0x1c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471514: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82471518: 41980018  blt cr6, 0x82471530
	if ctx.cr[6].lt {
	pc = 0x82471530; continue 'dispatch;
	}
	// 8247151C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82471520: 41980010  blt cr6, 0x82471530
	if ctx.cr[6].lt {
	pc = 0x82471530; continue 'dispatch;
	}
	// 82471524: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471528: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8247152C: 4800002C  b 0x82471558
	pc = 0x82471558; continue 'dispatch;
	// 82471530: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471534: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247153C: 388B8A74  addi r4, r11, -0x758c
	ctx.r[4].s64 = ctx.r[11].s64 + -30092;
	// 82471540: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82471544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471548: 4E800421  bctrl
	ctx.lr = 0x8247154C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247154C: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82471550: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82471554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82471558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247155C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471560: 4E800421  bctrl
	ctx.lr = 0x82471564;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82471568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8247156C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82471570: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82471574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82471578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471580 size=44
    let mut pc: u32 = 0x82471580;
    'dispatch: loop {
        match pc {
            0x82471580 => {
    //   block [0x82471580..0x824715AC)
	// 82471580: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471584: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471588: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8247158C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82471590: 4198001C  blt cr6, 0x824715ac
	if ctx.cr[6].lt {
		sub_824715AC(ctx, base);
		return;
	}
	// 82471594: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82471598: 41980014  blt cr6, 0x824715ac
	if ctx.cr[6].lt {
		sub_824715AC(ctx, base);
		return;
	}
	// 8247159C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715A0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824715A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824715A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824715AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824715AC size=8
    let mut pc: u32 = 0x824715AC;
    'dispatch: loop {
        match pc {
            0x824715AC => {
    //   block [0x824715AC..0x824715B4)
	// 824715AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824715B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824715B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824715B8 size=312
    let mut pc: u32 = 0x824715B8;
    'dispatch: loop {
        match pc {
            0x824715B8 => {
    //   block [0x824715B8..0x824716F0)
	// 824715B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824715BC: 480C3AF5  bl 0x825350b0
	ctx.lr = 0x824715C0;
	sub_82535080(ctx, base);
	// 824715C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824715C4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824715C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824715CC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824715D0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824715D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824715D8: 4099010C  ble cr6, 0x824716e4
	if !ctx.cr[6].gt {
	pc = 0x824716E4; continue 'dispatch;
	}
	// 824715DC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824715E0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715E4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824715E8: 7FDB5A14  add r30, r27, r11
	ctx.r[30].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 824715EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715F0: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715F4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824715F8: 83A50000  lwz r29, 0(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824715FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82471600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471604: 4E800421  bctrl
	ctx.lr = 0x82471608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471608: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8247160C: 419A0080  beq cr6, 0x8247168c
	if ctx.cr[6].eq {
	pc = 0x8247168C; continue 'dispatch;
	}
	// 82471610: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471614: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471618: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8247161C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471620: 48003171  bl 0x82474790
	ctx.lr = 0x82471624;
	sub_82474790(ctx, base);
	// 82471624: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82471628: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247162C: 4BFF569D  bl 0x82466cc8
	ctx.lr = 0x82471630;
	sub_82466CC8(ctx, base);
	// 82471630: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471634: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471638: 419A0048  beq cr6, 0x82471680
	if ctx.cr[6].eq {
	pc = 0x82471680; continue 'dispatch;
	}
	// 8247163C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471640: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82471644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82471648: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8247164C: 4BFFBEE5  bl 0x8246d530
	ctx.lr = 0x82471650;
	sub_8246D530(ctx, base);
	// 82471650: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471654: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82471658: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8247165C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471660: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471668: 4E800421  bctrl
	ctx.lr = 0x8247166C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247166C: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82471670: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471674: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471678: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8247167C: 48003115  bl 0x82474790
	ctx.lr = 0x82471680;
	sub_82474790(ctx, base);
	// 82471680: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82471684: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82471688: 48000050  b 0x824716d8
	pc = 0x824716D8; continue 'dispatch;
	// 8247168C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471690: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82471694: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82471698: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8247169C: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824716A0: 40980038  bge cr6, 0x824716d8
	if !ctx.cr[6].lt {
	pc = 0x824716D8; continue 'dispatch;
	}
	// 824716A4: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 824716A8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824716AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824716B0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824716B4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824716B8: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 824716BC: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824716C0: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824716C4: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824716C8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824716CC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824716D0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824716D4: 4198FFD4  blt cr6, 0x824716a8
	if ctx.cr[6].lt {
	pc = 0x824716A8; continue 'dispatch;
	}
	// 824716D8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824716DC: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824716E0: 4198FF00  blt cr6, 0x824715e0
	if ctx.cr[6].lt {
	pc = 0x824715E0; continue 'dispatch;
	}
	// 824716E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824716E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824716EC: 480C3A14  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824716F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824716F0 size=16
    let mut pc: u32 = 0x824716F0;
    'dispatch: loop {
        match pc {
            0x824716F0 => {
    //   block [0x824716F0..0x82471700)
	// 824716F0: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824716F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824716F8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824716FC: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471700 size=36
    let mut pc: u32 = 0x82471700;
    'dispatch: loop {
        match pc {
            0x82471700 => {
    //   block [0x82471700..0x82471724)
	// 82471700: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471704: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471708: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8247170C: 419A0018  beq cr6, 0x82471724
	if ctx.cr[6].eq {
		sub_82471724(ctx, base);
		return;
	}
	// 82471710: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82471714: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82471718: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8247171C: 4198FFE8  blt cr6, 0x82471704
	if ctx.cr[6].lt {
	pc = 0x82471704; continue 'dispatch;
	}
	// 82471720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471724 size=48
    let mut pc: u32 = 0x82471724;
    'dispatch: loop {
        match pc {
            0x82471724 => {
    //   block [0x82471724..0x82471754)
	// 82471724: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471728: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8247172C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471730: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82471734: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82471738: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247173C: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82471740: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82471744: 7D46582E  lwzx r10, r6, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471748: 7D48592E  stwx r10, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8247174C: 7C67482E  lwzx r3, r7, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82471750: 4BFF2B70  b 0x824642c0
	sub_824642C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471758 size=32
    let mut pc: u32 = 0x82471758;
    'dispatch: loop {
        match pc {
            0x82471758 => {
    //   block [0x82471758..0x82471778)
	// 82471758: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8247175C: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471760: 7D2B502E  lwzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82471764: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82471768: 419A0010  beq cr6, 0x82471778
	if ctx.cr[6].eq {
		sub_82471778(ctx, base);
		return;
	}
	// 8247176C: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471770: 7C655A14  add r3, r5, r11
	ctx.r[3].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82471774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471778 size=8
    let mut pc: u32 = 0x82471778;
    'dispatch: loop {
        match pc {
            0x82471778 => {
    //   block [0x82471778..0x82471780)
	// 82471778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8247177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82471780 size=28
    let mut pc: u32 = 0x82471780;
    'dispatch: loop {
        match pc {
            0x82471780 => {
    //   block [0x82471780..0x8247179C)
	// 82471780: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82471784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471788: 419A0014  beq cr6, 0x8247179c
	if ctx.cr[6].eq {
		sub_8247179C(ctx, base);
		return;
	}
	// 8247178C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82471790: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82471794: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82471798: 4BFFBD98  b 0x8246d530
	sub_8246D530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8247179C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8247179C size=44
    let mut pc: u32 = 0x8247179C;
    'dispatch: loop {
        match pc {
            0x8247179C => {
    //   block [0x8247179C..0x824717C8)
	// 8247179C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824717A0: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824717A4: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824717A8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824717AC: 4198001C  blt cr6, 0x824717c8
	if ctx.cr[6].lt {
		sub_824717C8(ctx, base);
		return;
	}
	// 824717B0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824717B4: 41980014  blt cr6, 0x824717c8
	if ctx.cr[6].lt {
		sub_824717C8(ctx, base);
		return;
	}
	// 824717B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824717BC: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824717C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824717C4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824717C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824717C8 size=8
    let mut pc: u32 = 0x824717C8;
    'dispatch: loop {
        match pc {
            0x824717C8 => {
    //   block [0x824717C8..0x824717D0)
	// 824717C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824717CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824717D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824717D0 size=664
    let mut pc: u32 = 0x824717D0;
    'dispatch: loop {
        match pc {
            0x824717D0 => {
    //   block [0x824717D0..0x82471A68)
	// 824717D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824717D4: 480C38C1  bl 0x82535094
	ctx.lr = 0x824717D8;
	sub_82535080(ctx, base);
	// 824717D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824717DC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824717E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824717E4: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 824717E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824717EC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 824717F0: 4098013C  bge cr6, 0x8247192c
	if !ctx.cr[6].lt {
	pc = 0x8247192C; continue 'dispatch;
	}
	// 824717F4: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824717F8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824717FC: 3BCB8A9C  addi r30, r11, -0x7564
	ctx.r[30].s64 = ctx.r[11].s64 + -30052;
	// 82471800: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471808: 4E800421  bctrl
	ctx.lr = 0x8247180C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247180C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82471810: 4BFF87C1  bl 0x82469fd0
	ctx.lr = 0x82471814;
	sub_82469FD0(ctx, base);
	// 82471814: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471818: 419A0114  beq cr6, 0x8247192c
	if ctx.cr[6].eq {
	pc = 0x8247192C; continue 'dispatch;
	}
	// 8247181C: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471820: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82471824: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82471828: 388B8A8C  addi r4, r11, -0x7574
	ctx.r[4].s64 = ctx.r[11].s64 + -30068;
	// 8247182C: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82471830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471834: 4E800421  bctrl
	ctx.lr = 0x82471838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471838: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8247183C: 41980224  blt cr6, 0x82471a60
	if ctx.cr[6].lt {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 82471840: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471844: 815A0020  lwz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471848: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 8247184C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471850: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471854: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471858: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247185C: 419A0204  beq cr6, 0x82471a60
	if ctx.cr[6].eq {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 82471860: 813A0024  lwz r9, 0x24(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471864: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471868: 7D0A482E  lwzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8247186C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82471870: 419A01F0  beq cr6, 0x82471a60
	if ctx.cr[6].eq {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 82471874: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82471878: 7D6A482E  lwzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8247187C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82471880: 7F080194  addze r24, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[24].s64 = tmp.s64;
	// 82471884: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82471888: 409901D8  ble cr6, 0x82471a60
	if !ctx.cr[6].gt {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 8247188C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82471890: 839B0000  lwz r28, 0(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471894: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82471898: 419A01C8  beq cr6, 0x82471a60
	if ctx.cr[6].eq {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 8247189C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824718A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824718A4: 83DB0004  lwz r30, 4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824718A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824718AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824718B0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824718B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824718B8: 4E800421  bctrl
	ctx.lr = 0x824718BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824718BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824718C0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824718C4: 419A0050  beq cr6, 0x82471914
	if ctx.cr[6].eq {
	pc = 0x82471914; continue 'dispatch;
	}
	// 824718C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824718CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824718D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824718D4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824718D8: 409A0010  bne cr6, 0x824718e8
	if !ctx.cr[6].eq {
	pc = 0x824718E8; continue 'dispatch;
	}
	// 824718DC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 824718E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824718E4: 4BFFCA6D  bl 0x8246e350
	ctx.lr = 0x824718E8;
	sub_8246E350(ctx, base);
	// 824718E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824718EC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824718F0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824718F4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 824718F8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824718FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471900: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471904: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82471908: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8247190C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82471910: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82471914: 3B390002  addi r25, r25, 2
	ctx.r[25].s64 = ctx.r[25].s64 + 2;
	// 82471918: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 8247191C: 7F19C000  cmpw cr6, r25, r24
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82471920: 4198FF70  blt cr6, 0x82471890
	if ctx.cr[6].lt {
	pc = 0x82471890; continue 'dispatch;
	}
	// 82471924: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82471928: 480C37BC  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
	// 8247192C: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471930: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82471934: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471938: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247193C: 40990124  ble cr6, 0x82471a60
	if !ctx.cr[6].gt {
	pc = 0x82471A60; continue 'dispatch;
	}
	// 82471940: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82471944: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82471948: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8247194C: 3A8B7468  addi r20, r11, 0x7468
	ctx.r[20].s64 = ctx.r[11].s64 + 29800;
	// 82471950: 815A0024  lwz r10, 0x24(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471954: 7D6AB02E  lwzx r11, r10, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82471958: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247195C: 419A00E8  beq cr6, 0x82471a44
	if ctx.cr[6].eq {
	pc = 0x82471A44; continue 'dispatch;
	}
	// 82471960: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471964: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82471968: 7F0AB02E  lwzx r24, r10, r22
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 8247196C: 7F2BAA14  add r25, r11, r21
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 82471970: 81790020  lwz r11, 0x20(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471974: 81590024  lwz r10, 0x24(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471978: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8247197C: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82471980: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82471984: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471988: 408100BC  ble 0x82471a44
	if !ctx.cr[0].gt {
	pc = 0x82471A44; continue 'dispatch;
	}
	// 8247198C: 3B6B0008  addi r27, r11, 8
	ctx.r[27].s64 = ctx.r[11].s64 + 8;
	// 82471990: 839BFFF8  lwz r28, -8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82471994: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82471998: 419A0088  beq cr6, 0x82471a20
	if ctx.cr[6].eq {
	pc = 0x82471A20; continue 'dispatch;
	}
	// 8247199C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824719A0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824719A4: 83DBFFFC  lwz r30, -4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824719A8: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824719AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824719B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824719B4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824719B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824719BC: 4E800421  bctrl
	ctx.lr = 0x824719C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824719C0: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 824719C4: 4BFF860D  bl 0x82469fd0
	ctx.lr = 0x824719C8;
	sub_82469FD0(ctx, base);
	// 824719C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824719CC: 409A0054  bne cr6, 0x82471a20
	if !ctx.cr[6].eq {
	pc = 0x82471A20; continue 'dispatch;
	}
	// 824719D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824719D4: 7F9CC214  add r28, r28, r24
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[24].u64;
	// 824719D8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824719DC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824719E0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824719E4: 409A0010  bne cr6, 0x824719f4
	if !ctx.cr[6].eq {
	pc = 0x824719F4; continue 'dispatch;
	}
	// 824719E8: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 824719EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824719F0: 4BFFC961  bl 0x8246e350
	ctx.lr = 0x824719F4;
	sub_8246E350(ctx, base);
	// 824719F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824719F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824719FC: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82471A00: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82471A04: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82471A08: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471A0C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471A10: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82471A14: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82471A18: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82471A1C: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82471A20: 81790024  lwz r11, 0x24(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471A24: 3AF70003  addi r23, r23, 3
	ctx.r[23].s64 = ctx.r[23].s64 + 3;
	// 82471A28: 81590020  lwz r10, 0x20(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471A2C: 3B7B000C  addi r27, r27, 0xc
	ctx.r[27].s64 = ctx.r[27].s64 + 12;
	// 82471A30: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82471A34: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82471A38: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82471A3C: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471A40: 4198FF50  blt cr6, 0x82471990
	if ctx.cr[6].lt {
	pc = 0x82471990; continue 'dispatch;
	}
	// 82471A44: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471A48: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 82471A4C: 3AB50030  addi r21, r21, 0x30
	ctx.r[21].s64 = ctx.r[21].s64 + 48;
	// 82471A50: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 82471A54: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471A58: 7F135800  cmpw cr6, r19, r11
	ctx.cr[6].compare_i32(ctx.r[19].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471A5C: 4198FEF4  blt cr6, 0x82471950
	if ctx.cr[6].lt {
	pc = 0x82471950; continue 'dispatch;
	}
	// 82471A60: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82471A64: 480C3680  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471A68 size=308
    let mut pc: u32 = 0x82471A68;
    'dispatch: loop {
        match pc {
            0x82471A68 => {
    //   block [0x82471A68..0x82471B9C)
	// 82471A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471A6C: 480C364D  bl 0x825350b8
	ctx.lr = 0x82471A70;
	sub_82535080(ctx, base);
	// 82471A70: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471A74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82471A78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82471A7C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82471A80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82471A84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471A88: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471A8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471A90: 4E800421  bctrl
	ctx.lr = 0x82471A94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471A94: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471A98: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82471A9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471AA0: 419A001C  beq cr6, 0x82471abc
	if ctx.cr[6].eq {
	pc = 0x82471ABC; continue 'dispatch;
	}
	// 82471AA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471AA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471AAC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82471AB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471AB4: 4E800421  bctrl
	ctx.lr = 0x82471AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471AB8: 48000008  b 0x82471ac0
	pc = 0x82471AC0; continue 'dispatch;
	// 82471ABC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82471AC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82471AC4: 907D0070  stw r3, 0x70(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82471AC8: 409A002C  bne cr6, 0x82471af4
	if !ctx.cr[6].eq {
	pc = 0x82471AF4; continue 'dispatch;
	}
	// 82471ACC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471AD0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82471AD4: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82471AD8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82471ADC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471AE0: 4BFF2789  bl 0x82464268
	ctx.lr = 0x82471AE4;
	sub_82464268(ctx, base);
	// 82471AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82471AE8: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471AEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471AF0: 480023E1  bl 0x82473ed0
	ctx.lr = 0x82471AF4;
	sub_82473ED0(ctx, base);
	// 82471AF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471AF8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82471AFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471B00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471B04: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471B08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471B0C: 4E800421  bctrl
	ctx.lr = 0x82471B10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471B10: 2B030040  cmplwi cr6, r3, 0x40
	ctx.cr[6].compare_u32(ctx.r[3].u32, 64 as u32, &mut ctx.xer);
	// 82471B14: 409A0078  bne cr6, 0x82471b8c
	if !ctx.cr[6].eq {
	pc = 0x82471B8C; continue 'dispatch;
	}
	// 82471B18: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82471B1C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82471B20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82471B24: 4BFF8815  bl 0x8246a338
	ctx.lr = 0x82471B28;
	sub_8246A338(ctx, base);
	// 82471B28: 3D6057E0  lis r11, 0x57e0
	ctx.r[11].s64 = 1474297856;
	// 82471B2C: 3D4010C0  lis r10, 0x10c0
	ctx.r[10].s64 = 281018368;
	// 82471B30: 9B810088  stb r28, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[28].u8 ) };
	// 82471B34: 616BE057  ori r11, r11, 0xe057
	ctx.r[11].u64 = ctx.r[11].u64 | 57431;
	// 82471B38: 614AC010  ori r10, r10, 0xc010
	ctx.r[10].u64 = ctx.r[10].u64 | 49168;
	// 82471B3C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82471B40: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82471B44: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471B48: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471B4C: 409A0040  bne cr6, 0x82471b8c
	if !ctx.cr[6].eq {
	pc = 0x82471B8C; continue 'dispatch;
	}
	// 82471B50: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471B54: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82471B58: 409A0034  bne cr6, 0x82471b8c
	if !ctx.cr[6].eq {
	pc = 0x82471B8C; continue 'dispatch;
	}
	// 82471B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471B60: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471B64: 93FD001C  stw r31, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 82471B68: 4BFFF779  bl 0x824712e0
	ctx.lr = 0x82471B6C;
	sub_824712E0(ctx, base);
	// 82471B6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82471B70: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 82471B74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82471B78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471B7C: 4E800421  bctrl
	ctx.lr = 0x82471B80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471B80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471B84: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82471B88: 480C3580  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82471B8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82471B90: 939D001C  stw r28, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82471B94: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82471B98: 480C3570  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471BA0 size=356
    let mut pc: u32 = 0x82471BA0;
    'dispatch: loop {
        match pc {
            0x82471BA0 => {
    //   block [0x82471BA0..0x82471D04)
	// 82471BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471BA4: 480C3515  bl 0x825350b8
	ctx.lr = 0x82471BA8;
	sub_82535080(ctx, base);
	// 82471BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471BAC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82471BB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82471BB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82471BB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82471BBC: 409A003C  bne cr6, 0x82471bf8
	if !ctx.cr[6].eq {
	pc = 0x82471BF8; continue 'dispatch;
	}
	// 82471BC0: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471BC4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82471BC8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471BCC: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82471BD0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471BD4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82471BD8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471BDC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471BE0: 55642036  slwi r4, r11, 4
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82471BE4: 4BFF2685  bl 0x82464268
	ctx.lr = 0x82471BE8;
	sub_82464268(ctx, base);
	// 82471BE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82471BEC: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471BF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471BF4: 480022DD  bl 0x82473ed0
	ctx.lr = 0x82471BF8;
	sub_82473ED0(ctx, base);
	// 82471BF8: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471BFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471C00: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471C04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471C08: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471C0C: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471C10: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471C14: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82471C18: 557E2036  slwi r30, r11, 4
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82471C1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82471C20: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82471C24: 4E800421  bctrl
	ctx.lr = 0x82471C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471C28: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82471C2C: 409A00CC  bne cr6, 0x82471cf8
	if !ctx.cr[6].eq {
	pc = 0x82471CF8; continue 'dispatch;
	}
	// 82471C30: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471C34: 3BDD0024  addi r30, r29, 0x24
	ctx.r[30].s64 = ctx.r[29].s64 + 36;
	// 82471C38: 93FD0020  stw r31, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82471C3C: 83EB0014  lwz r31, 0x14(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471C40: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471C44: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82471C48: 40990060  ble cr6, 0x82471ca8
	if !ctx.cr[6].gt {
	pc = 0x82471CA8; continue 'dispatch;
	}
	// 82471C4C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471C50: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82471C54: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82471C58: 40980024  bge cr6, 0x82471c7c
	if !ctx.cr[6].lt {
	pc = 0x82471C7C; continue 'dispatch;
	}
	// 82471C5C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471C60: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471C64: 41980008  blt cr6, 0x82471c6c
	if ctx.cr[6].lt {
	pc = 0x82471C6C; continue 'dispatch;
	}
	// 82471C68: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82471C6C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82471C70: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82471C74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471C78: 4BFFC651  bl 0x8246e2c8
	ctx.lr = 0x82471C7C;
	sub_8246E2C8(ctx, base);
	// 82471C7C: 7F1CF800  cmpw cr6, r28, r31
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82471C80: 40980028  bge cr6, 0x82471ca8
	if !ctx.cr[6].lt {
	pc = 0x82471CA8; continue 'dispatch;
	}
	// 82471C84: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471C88: 7D7CF850  subf r11, r28, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 82471C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82471C90: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471C94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82471C98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471C9C: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82471CA0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82471CA4: 409AFFEC  bne cr6, 0x82471c90
	if !ctx.cr[6].eq {
	pc = 0x82471C90; continue 'dispatch;
	}
	// 82471CA8: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82471CAC: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471CB0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82471CB4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82471CB8: 40980034  bge cr6, 0x82471cec
	if !ctx.cr[6].lt {
	pc = 0x82471CEC; continue 'dispatch;
	}
	// 82471CBC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471CC0: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471CC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471CC8: 40990024  ble cr6, 0x82471cec
	if !ctx.cr[6].gt {
	pc = 0x82471CEC; continue 'dispatch;
	}
	// 82471CCC: 396B002C  addi r11, r11, 0x2c
	ctx.r[11].s64 = ctx.r[11].s64 + 44;
	// 82471CD0: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82471CD4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82471CD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82471CDC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82471CE0: 912BFFF8  stw r9, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 82471CE4: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82471CE8: 409AFFE8  bne cr6, 0x82471cd0
	if !ctx.cr[6].eq {
	pc = 0x82471CD0; continue 'dispatch;
	}
	// 82471CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471CF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82471CF4: 480C3414  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82471CF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82471CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82471D00: 480C3408  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471D08 size=100
    let mut pc: u32 = 0x82471D08;
    'dispatch: loop {
        match pc {
            0x82471D08 => {
    //   block [0x82471D08..0x82471D6C)
	// 82471D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471D0C: 480C33B1  bl 0x825350bc
	ctx.lr = 0x82471D10;
	sub_82535080(ctx, base);
	// 82471D10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471D14: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 82471D18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82471D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471D20: 4BFFB7B9  bl 0x8246d4d8
	ctx.lr = 0x82471D24;
	sub_8246D4D8(ctx, base);
	// 82471D24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471D28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82471D2C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471D30: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82471D34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82471D38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82471D3C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82471D40: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471D44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471D48: 419A0010  beq cr6, 0x82471d58
	if ctx.cr[6].eq {
	pc = 0x82471D58; continue 'dispatch;
	}
	// 82471D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82471D50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471D54: 4E800421  bctrl
	ctx.lr = 0x82471D58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471D58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82471D5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471D60: 4BFFB8C1  bl 0x8246d620
	ctx.lr = 0x82471D64;
	sub_8246D620(ctx, base);
	// 82471D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82471D68: 480C33A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82471D70 size=556
    let mut pc: u32 = 0x82471D70;
    'dispatch: loop {
        match pc {
            0x82471D70 => {
    //   block [0x82471D70..0x82471F9C)
	// 82471D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471D74: 480C3329  bl 0x8253509c
	ctx.lr = 0x82471D78;
	sub_82535080(ctx, base);
	// 82471D78: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471D7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82471D80: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82471D84: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471D88: 48008B71  bl 0x8247a8f8
	ctx.lr = 0x82471D8C;
	sub_8247A8F8(ctx, base);
	// 82471D8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82471D90: 409A0118  bne cr6, 0x82471ea8
	if !ctx.cr[6].eq {
	pc = 0x82471EA8; continue 'dispatch;
	}
	// 82471D94: 817C0078  lwz r11, 0x78(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(120 as u32) ) } as u64;
	// 82471D98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471D9C: 409A0118  bne cr6, 0x82471eb4
	if !ctx.cr[6].eq {
	pc = 0x82471EB4; continue 'dispatch;
	}
	// 82471DA0: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471DA4: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82471DA8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471DAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82471DB0: 409900F8  ble cr6, 0x82471ea8
	if !ctx.cr[6].gt {
	pc = 0x82471EA8; continue 'dispatch;
	}
	// 82471DB4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82471DB8: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82471DBC: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 82471DC0: 815C0024  lwz r10, 0x24(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471DC4: 7D6AC82E  lwzx r11, r10, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82471DC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82471DCC: 419A00C0  beq cr6, 0x82471e8c
	if ctx.cr[6].eq {
	pc = 0x82471E8C; continue 'dispatch;
	}
	// 82471DD0: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471DD4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82471DD8: 7FAAC82E  lwzx r29, r10, r25
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82471DDC: 7F6BC214  add r27, r11, r24
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82471DE0: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471DE4: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471DE8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82471DEC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82471DF0: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82471DF4: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82471DF8: 40810094  ble 0x82471e8c
	if !ctx.cr[0].gt {
	pc = 0x82471E8C; continue 'dispatch;
	}
	// 82471DFC: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82471E00: 83FEFFF8  lwz r31, -8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82471E04: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82471E08: 419A0060  beq cr6, 0x82471e68
	if ctx.cr[6].eq {
	pc = 0x82471E68; continue 'dispatch;
	}
	// 82471E0C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471E10: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82471E14: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82471E18: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471E1C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82471E20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471E24: 4E800421  bctrl
	ctx.lr = 0x82471E28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471E28: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471E2C: 7FFFEA14  add r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82471E30: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82471E34: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82471E38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471E3C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471E40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82471E44: 4E800421  bctrl
	ctx.lr = 0x82471E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471E48: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82471E4C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82471E50: 419A0018  beq cr6, 0x82471e68
	if ctx.cr[6].eq {
	pc = 0x82471E68; continue 'dispatch;
	}
	// 82471E54: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471E58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471E5C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82471E60: 92AB000C  stw r21, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[21].u32 ) };
	// 82471E64: 4BFFB59D  bl 0x8246d400
	ctx.lr = 0x82471E68;
	sub_8246D400(ctx, base);
	// 82471E68: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82471E6C: 3B5A0003  addi r26, r26, 3
	ctx.r[26].s64 = ctx.r[26].s64 + 3;
	// 82471E70: 815B0020  lwz r10, 0x20(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82471E74: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82471E78: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82471E7C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82471E80: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82471E84: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471E88: 4198FF78  blt cr6, 0x82471e00
	if ctx.cr[6].lt {
	pc = 0x82471E00; continue 'dispatch;
	}
	// 82471E8C: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82471E90: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82471E94: 3B180030  addi r24, r24, 0x30
	ctx.r[24].s64 = ctx.r[24].s64 + 48;
	// 82471E98: 3B390004  addi r25, r25, 4
	ctx.r[25].s64 = ctx.r[25].s64 + 4;
	// 82471E9C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82471EA0: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471EA4: 4198FF1C  blt cr6, 0x82471dc0
	if ctx.cr[6].lt {
	pc = 0x82471DC0; continue 'dispatch;
	}
	// 82471EA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82471EAC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82471EB0: 480C323C  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
	// 82471EB4: 3BAB0028  addi r29, r11, 0x28
	ctx.r[29].s64 = ctx.r[11].s64 + 40;
	// 82471EB8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82471EBC: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471EC0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82471EC4: 41980024  blt cr6, 0x82471ee8
	if ctx.cr[6].lt {
	pc = 0x82471EE8; continue 'dispatch;
	}
	// 82471EC8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471ECC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471ED0: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82471ED4: 409A0014  bne cr6, 0x82471ee8
	if !ctx.cr[6].eq {
	pc = 0x82471EE8; continue 'dispatch;
	}
	// 82471ED8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82471EDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82471EE0: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82471EE4: 4099FFE8  ble cr6, 0x82471ecc
	if !ctx.cr[6].gt {
	pc = 0x82471ECC; continue 'dispatch;
	}
	// 82471EE8: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 82471EEC: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82471EF0: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 82471EF4: 40990008  ble cr6, 0x82471efc
	if !ctx.cr[6].gt {
	pc = 0x82471EFC; continue 'dispatch;
	}
	// 82471EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82471EFC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82471F00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82471F04: 419AFFA4  beq cr6, 0x82471ea8
	if ctx.cr[6].eq {
	pc = 0x82471EA8; continue 'dispatch;
	}
	// 82471F08: 7D49F214  add r10, r9, r30
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82471F0C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471F10: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82471F14: 81370000  lwz r9, 0(r23)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471F18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82471F1C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82471F20: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471F24: 7FE8582E  lwzx r31, r8, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471F28: 81290010  lwz r9, 0x10(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82471F2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471F30: 7CAA582E  lwzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82471F34: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82471F38: 4E800421  bctrl
	ctx.lr = 0x82471F3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82471F3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82471F40: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82471F44: 419A0018  beq cr6, 0x82471f5c
	if ctx.cr[6].eq {
	pc = 0x82471F5C; continue 'dispatch;
	}
	// 82471F48: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82471F4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82471F50: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82471F54: 92AB000C  stw r21, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[21].u32 ) };
	// 82471F58: 4BFFB4A9  bl 0x8246d400
	ctx.lr = 0x82471F5C;
	sub_8246D400(ctx, base);
	// 82471F5C: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471F60: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82471F64: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82471F68: 4199002C  bgt cr6, 0x82471f94
	if ctx.cr[6].gt {
	pc = 0x82471F94; continue 'dispatch;
	}
	// 82471F6C: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471F70: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471F74: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82471F78: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471F7C: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 82471F80: 409A0014  bne cr6, 0x82471f94
	if !ctx.cr[6].eq {
	pc = 0x82471F94; continue 'dispatch;
	}
	// 82471F84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82471F88: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82471F8C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82471F90: 4099FFE8  ble cr6, 0x82471f78
	if !ctx.cr[6].gt {
	pc = 0x82471F78; continue 'dispatch;
	}
	// 82471F94: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82471F98: 4BFFFF54  b 0x82471eec
	pc = 0x82471EEC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82471FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82471FA0 size=376
    let mut pc: u32 = 0x82471FA0;
    'dispatch: loop {
        match pc {
            0x82471FA0 => {
    //   block [0x82471FA0..0x82472118)
	// 82471FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82471FA4: 480C3109  bl 0x825350ac
	ctx.lr = 0x82471FA8;
	sub_82535080(ctx, base);
	// 82471FA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82471FAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82471FB0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82471FB4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82471FB8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82471FBC: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471FC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82471FC4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471FC8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82471FCC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82471FD0: 409A0010  bne cr6, 0x82471fe0
	if !ctx.cr[6].eq {
	pc = 0x82471FE0; continue 'dispatch;
	}
	// 82471FD4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82471FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82471FDC: 4BFFC375  bl 0x8246e350
	ctx.lr = 0x82471FE0;
	sub_8246E350(ctx, base);
	// 82471FE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82471FE4: 3B3C000C  addi r25, r28, 0xc
	ctx.r[25].s64 = ctx.r[28].s64 + 12;
	// 82471FE8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82471FEC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82471FF0: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82471FF4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82471FF8: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82471FFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472000: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82472004: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82472008: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8247200C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82472010: 4BFFB3F1  bl 0x8246d400
	ctx.lr = 0x82472014;
	sub_8246D400(ctx, base);
	// 82472014: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472018: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8247201C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472020: 409900EC  ble cr6, 0x8247210c
	if !ctx.cr[6].gt {
	pc = 0x8247210C; continue 'dispatch;
	}
	// 82472024: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82472028: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247202C: 7FFA5A14  add r31, r26, r11
	ctx.r[31].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82472030: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472034: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472038: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8247203C: 419A0078  beq cr6, 0x824720b4
	if ctx.cr[6].eq {
	pc = 0x824720B4; continue 'dispatch;
	}
	// 82472040: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472044: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82472048: 4BFFB491  bl 0x8246d4d8
	ctx.lr = 0x8247204C;
	sub_8246D4D8(ctx, base);
	// 8247204C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82472050: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472054: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82472058: 40990008  ble cr6, 0x82472060
	if !ctx.cr[6].gt {
	pc = 0x82472060; continue 'dispatch;
	}
	// 8247205C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82472060: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82472064: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472068: 409A004C  bne cr6, 0x824720b4
	if !ctx.cr[6].eq {
	pc = 0x824720B4; continue 'dispatch;
	}
	// 8247206C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472070: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472074: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82472078: 4BFFB4B9  bl 0x8246d530
	ctx.lr = 0x8247207C;
	sub_8246D530(ctx, base);
	// 8247207C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82472080: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82472084: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472088: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8247208C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82472090: 4BFFB371  bl 0x8246d400
	ctx.lr = 0x82472094;
	sub_8246D400(ctx, base);
	// 82472094: 80DC0020  lwz r6, 0x20(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472098: 80BC001C  lwz r5, 0x1c(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247209C: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 824720A0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824720A4: 480026ED  bl 0x82474790
	ctx.lr = 0x824720A8;
	sub_82474790(ctx, base);
	// 824720A8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824720AC: 3B5A0008  addi r26, r26, 8
	ctx.r[26].s64 = ctx.r[26].s64 + 8;
	// 824720B0: 48000050  b 0x82472100
	pc = 0x82472100; continue 'dispatch;
	// 824720B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824720B8: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 824720BC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824720C0: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824720C4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824720C8: 40980038  bge cr6, 0x82472100
	if !ctx.cr[6].lt {
	pc = 0x82472100; continue 'dispatch;
	}
	// 824720CC: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 824720D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824720D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824720D8: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824720DC: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824720E0: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 824720E4: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824720E8: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824720EC: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824720F0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824720F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824720F8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824720FC: 4198FFD4  blt cr6, 0x824720d0
	if ctx.cr[6].lt {
	pc = 0x824720D0; continue 'dispatch;
	}
	// 82472100: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472104: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472108: 4198FF20  blt cr6, 0x82472028
	if ctx.cr[6].lt {
	pc = 0x82472028; continue 'dispatch;
	}
	// 8247210C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82472110: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82472114: 480C2FE8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82472118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82472118 size=192
    let mut pc: u32 = 0x82472118;
    'dispatch: loop {
        match pc {
            0x82472118 => {
    //   block [0x82472118..0x824721D8)
	// 82472118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82472120: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82472124: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82472128: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247212C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82472130: 480026B9  bl 0x824747e8
	ctx.lr = 0x82472134;
	sub_824747E8(ctx, base);
	// 82472134: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82472138: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247213C: 395F0024  addi r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 + 36;
	// 82472140: 392B8AB4  addi r9, r11, -0x754c
	ctx.r[9].s64 = ctx.r[11].s64 + -30028;
	// 82472144: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82472148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247214C: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	// 82472150: 60E70010  ori r7, r7, 0x10
	ctx.r[7].u64 = ctx.r[7].u64 | 16;
	// 82472154: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82472158: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 8247215C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82472160: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82472164: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82472168: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8247216C: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 82472170: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82472174: 90EA0008  stw r7, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82472178: 906A0000  stw r3, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8247217C: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82472180: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82472184: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82472188: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8247218C: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82472190: 913F0084  stw r9, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82472194: 7C66402E  lwzx r3, r6, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82472198: 4BFF1EA1  bl 0x82464038
	ctx.lr = 0x8247219C;
	sub_82464038(ctx, base);
	// 8247219C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824721A0: 3960004C  li r11, 0x4c
	ctx.r[11].s64 = 76;
	// 824721A4: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824721A8: 48008891  bl 0x8247aa38
	ctx.lr = 0x824721AC;
	sub_8247AA38(ctx, base);
	// 824721AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824721B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824721B4: 396B89FC  addi r11, r11, -0x7604
	ctx.r[11].s64 = ctx.r[11].s64 + -30212;
	// 824721B8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824721BC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 824721C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824721C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824721C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824721CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824721D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824721D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824721D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824721D8 size=260
    let mut pc: u32 = 0x824721D8;
    'dispatch: loop {
        match pc {
            0x824721D8 => {
    //   block [0x824721D8..0x824722DC)
	// 824721D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824721DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824721E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824721E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824721E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824721EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824721F0: 396B8AB4  addi r11, r11, -0x754c
	ctx.r[11].s64 = ctx.r[11].s64 + -30028;
	// 824721F4: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 824721F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824721FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82472200: 419A0018  beq cr6, 0x82472218
	if ctx.cr[6].eq {
	pc = 0x82472218; continue 'dispatch;
	}
	// 82472204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472208: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8247220C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472214: 4E800421  bctrl
	ctx.lr = 0x82472218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472218: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8247221C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82472220: 419A000C  beq cr6, 0x8247222c
	if ctx.cr[6].eq {
	pc = 0x8247222C; continue 'dispatch;
	}
	// 82472224: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82472228: 4800E2A9  bl 0x824804d0
	ctx.lr = 0x8247222C;
	sub_824804D0(ctx, base);
	// 8247222C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472230: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472234: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472238: 419A0030  beq cr6, 0x82472268
	if ctx.cr[6].eq {
	pc = 0x82472268; continue 'dispatch;
	}
	// 8247223C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82472240: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82472244: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82472248: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247224C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82472250: 409A0018  bne cr6, 0x82472268
	if !ctx.cr[6].eq {
	pc = 0x82472268; continue 'dispatch;
	}
	// 82472254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472258: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8247225C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472264: 4E800421  bctrl
	ctx.lr = 0x82472268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472268: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8247226C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82472270: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472274: 409A0020  bne cr6, 0x82472294
	if !ctx.cr[6].eq {
	pc = 0x82472294; continue 'dispatch;
	}
	// 82472278: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247227C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82472280: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82472284: 809F007C  lwz r4, 0x7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82472288: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8247228C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472290: 4BFF1E29  bl 0x824640b8
	ctx.lr = 0x82472294;
	sub_824640B8(ctx, base);
	// 82472294: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82472298: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8247229C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824722A0: 409A0020  bne cr6, 0x824722c0
	if !ctx.cr[6].eq {
	pc = 0x824722C0; continue 'dispatch;
	}
	// 824722A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824722A8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824722AC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824722B0: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824722B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824722B8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824722BC: 4BFF1DFD  bl 0x824640b8
	ctx.lr = 0x824722C0;
	sub_824640B8(ctx, base);
	// 824722C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824722C4: 48002575  bl 0x82474838
	ctx.lr = 0x824722C8;
	sub_82474838(ctx, base);
	// 824722C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824722CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824722D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824722D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824722D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824722E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824722E0 size=696
    let mut pc: u32 = 0x824722E0;
    'dispatch: loop {
        match pc {
            0x824722E0 => {
    //   block [0x824722E0..0x82472598)
	// 824722E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824722E4: 480C2DC5  bl 0x825350a8
	ctx.lr = 0x824722E8;
	sub_82535080(ctx, base);
	// 824722E8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824722EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824722F0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 824722F4: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824722F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824722FC: 409A00DC  bne cr6, 0x824723d8
	if !ctx.cr[6].eq {
	pc = 0x824723D8; continue 'dispatch;
	}
	// 82472300: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82472304: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82472308: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8247230C: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 82472310: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82472314: 4BFFF4BD  bl 0x824717d0
	ctx.lr = 0x82472318;
	sub_824717D0(ctx, base);
	// 82472318: 3BDF007C  addi r30, r31, 0x7c
	ctx.r[30].s64 = ctx.r[31].s64 + 124;
	// 8247231C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472320: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472324: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82472328: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 8247232C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82472330: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82472334: 40980024  bge cr6, 0x82472358
	if !ctx.cr[6].lt {
	pc = 0x82472358; continue 'dispatch;
	}
	// 82472338: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247233C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472340: 41980008  blt cr6, 0x82472348
	if ctx.cr[6].lt {
	pc = 0x82472348; continue 'dispatch;
	}
	// 82472344: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82472348: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8247234C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82472350: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82472354: 4BFFBF75  bl 0x8246e2c8
	ctx.lr = 0x82472358;
	sub_8246E2C8(ctx, base);
	// 82472358: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247235C: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82472360: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82472364: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82472368: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8247236C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472370: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472374: 4099002C  ble cr6, 0x824723a0
	if !ctx.cr[6].gt {
	pc = 0x824723A0; continue 'dispatch;
	}
	// 82472378: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 8247237C: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82472380: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82472384: 7D0A402E  lwzx r8, r10, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82472388: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 8247238C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82472390: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82472394: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472398: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8247239C: 4198FFE0  blt cr6, 0x8247237c
	if ctx.cr[6].lt {
	pc = 0x8247237C; continue 'dispatch;
	}
	// 824723A0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824723A4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824723A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824723AC: 409A002C  bne cr6, 0x824723d8
	if !ctx.cr[6].eq {
	pc = 0x824723D8; continue 'dispatch;
	}
	// 824723B0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824723B4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824723B8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824723BC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824723C0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824723C4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824723C8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824723CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824723D0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824723D4: 4BFF1CE5  bl 0x824640b8
	ctx.lr = 0x824723D8;
	sub_824640B8(ctx, base);
	// 824723D8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824723DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824723E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824723E4: 83CB92F0  lwz r30, -0x6d10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27920 as u32) ) } as u64;
	// 824723E8: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 824723EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824723F0: 4E800421  bctrl
	ctx.lr = 0x824723F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824723F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824723F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824723FC: 48007E7D  bl 0x8247a278
	ctx.lr = 0x82472400;
	sub_8247A278(ctx, base);
	// 82472400: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82472404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472408: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247240C: 419A0168  beq cr6, 0x82472574
	if ctx.cr[6].eq {
	pc = 0x82472574; continue 'dispatch;
	}
	// 82472410: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472414: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82472418: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8247241C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82472420: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472424: 4BFF1C15  bl 0x82464038
	ctx.lr = 0x82472428;
	sub_82464038(ctx, base);
	// 82472428: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8247242C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472430: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82472434: 48008CE5  bl 0x8247b118
	ctx.lr = 0x82472438;
	sub_8247B118(ctx, base);
	// 82472438: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8247243C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82472440: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82472444: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472448: 40990078  ble cr6, 0x824724c0
	if !ctx.cr[6].gt {
	pc = 0x824724C0; continue 'dispatch;
	}
	// 8247244C: 3B3F0008  addi r25, r31, 8
	ctx.r[25].s64 = ctx.r[31].s64 + 8;
	// 82472450: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82472454: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82472458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247245C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472460: 7FDD582E  lwzx r30, r29, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472464: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247246C: 4E800421  bctrl
	ctx.lr = 0x82472470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472470: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472474: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82472478: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247247C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82472480: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82472484: 4800230D  bl 0x82474790
	ctx.lr = 0x82472488;
	sub_82474790(ctx, base);
	// 82472488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247248C: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472490: 4BFF43C9  bl 0x82466858
	ctx.lr = 0x82472494;
	sub_82466858(ctx, base);
	// 82472494: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82472498: 817B001C  lwz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247249C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824724A0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824724A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824724A8: 4E800421  bctrl
	ctx.lr = 0x824724AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824724AC: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824724B0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824724B4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824724B8: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824724BC: 4198FF98  blt cr6, 0x82472454
	if ctx.cr[6].lt {
	pc = 0x82472454; continue 'dispatch;
	}
	// 824724C0: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824724C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824724C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 824724CC: 93010070  stw r24, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[24].u32 ) };
	// 824724D0: 3D208293  lis r9, -0x7d6d
	ctx.r[9].s64 = -2104295424;
	// 824724D4: 3D008293  lis r8, -0x7d6d
	ctx.r[8].s64 = -2104295424;
	// 824724D8: 396B8E24  addi r11, r11, -0x71dc
	ctx.r[11].s64 = ctx.r[11].s64 + -29148;
	// 824724DC: 394A8DC4  addi r10, r10, -0x723c
	ctx.r[10].s64 = ctx.r[10].s64 + -29244;
	// 824724E0: 80E7000C  lwz r7, 0xc(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 824724E4: 39298F14  addi r9, r9, -0x70ec
	ctx.r[9].s64 = ctx.r[9].s64 + -28908;
	// 824724E8: 39088EE4  addi r8, r8, -0x711c
	ctx.r[8].s64 = ctx.r[8].s64 + -28956;
	// 824724EC: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 824724F0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824724F4: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 824724F8: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 824724FC: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82472500: 409A001C  bne cr6, 0x8247251c
	if !ctx.cr[6].eq {
	pc = 0x8247251C; continue 'dispatch;
	}
	// 82472504: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82472508: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8247250C: 396B3A78  addi r11, r11, 0x3a78
	ctx.r[11].s64 = ctx.r[11].s64 + 14968;
	// 82472510: 394A3AD8  addi r10, r10, 0x3ad8
	ctx.r[10].s64 = ctx.r[10].s64 + 15064;
	// 82472514: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82472518: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8247251C: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82472520: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472524: 419A0044  beq cr6, 0x82472568
	if ctx.cr[6].eq {
	pc = 0x82472568; continue 'dispatch;
	}
	// 82472528: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8247252C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472530: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472534: 4BFF4325  bl 0x82466858
	ctx.lr = 0x82472538;
	sub_82466858(ctx, base);
	// 82472538: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8247253C: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472540: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82472544: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82472548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247254C: 4E800421  bctrl
	ctx.lr = 0x82472550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472550: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82472554: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82472558: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8247255C: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472560: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82472564: 409AFFC8  bne cr6, 0x8247252c
	if !ctx.cr[6].eq {
	pc = 0x8247252C; continue 'dispatch;
	}
	// 82472568: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8247256C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82472570: 480C2B88  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82472574: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472578: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247257C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472580: 419A0010  beq cr6, 0x82472590
	if ctx.cr[6].eq {
	pc = 0x82472590; continue 'dispatch;
	}
	// 82472584: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82472588: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8247258C: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82472590: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82472594: 480C2B64  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82472598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82472598 size=828
    let mut pc: u32 = 0x82472598;
    'dispatch: loop {
        match pc {
            0x82472598 => {
    //   block [0x82472598..0x824728D4)
	// 82472598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247259C: 480C2AF9  bl 0x82535094
	ctx.lr = 0x824725A0;
	sub_82535080(ctx, base);
	// 824725A0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824725A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824725A8: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824725AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824725B0: 409A0318  bne cr6, 0x824728c8
	if !ctx.cr[6].eq {
	pc = 0x824728C8; continue 'dispatch;
	}
	// 824725B4: 4BFFFD2D  bl 0x824722e0
	ctx.lr = 0x824725B8;
	sub_824722E0(ctx, base);
	// 824725B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824725BC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 824725C0: 3AC0FFFF  li r22, -1
	ctx.r[22].s64 = -1;
	// 824725C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824725C8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824725CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824725D0: 4E800421  bctrl
	ctx.lr = 0x824725D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824725D4: 4BFFED55  bl 0x82471328
	ctx.lr = 0x824725D8;
	sub_82471328(ctx, base);
	// 824725D8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 824725DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824725E0: 419A0024  beq cr6, 0x82472604
	if ctx.cr[6].eq {
	pc = 0x82472604; continue 'dispatch;
	}
	// 824725E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824725E8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824725EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824725F0: 388B8AF0  addi r4, r11, -0x7510
	ctx.r[4].s64 = ctx.r[11].s64 + -29968;
	// 824725F4: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 824725F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824725FC: 4E800421  bctrl
	ctx.lr = 0x82472600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472600: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82472604: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82472608: 4BFFAD39  bl 0x8246d340
	ctx.lr = 0x8247260C;
	sub_8246D340(ctx, base);
	// 8247260C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472610: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82472614: 7E749B78  mr r20, r19
	ctx.r[20].u64 = ctx.r[19].u64;
	// 82472618: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8247261C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472620: 409900F8  ble cr6, 0x82472718
	if !ctx.cr[6].gt {
	pc = 0x82472718; continue 'dispatch;
	}
	// 82472624: 7E789B78  mr r24, r19
	ctx.r[24].u64 = ctx.r[19].u64;
	// 82472628: 7E759B78  mr r21, r19
	ctx.r[21].u64 = ctx.r[19].u64;
	// 8247262C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472630: 7D78502E  lwzx r11, r24, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472634: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472638: 419A00C4  beq cr6, 0x824726fc
	if ctx.cr[6].eq {
	pc = 0x824726FC; continue 'dispatch;
	}
	// 8247263C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472640: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 82472644: 7F58502E  lwzx r26, r24, r10
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472648: 7F6BAA14  add r27, r11, r21
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 8247264C: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472650: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472654: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82472658: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8247265C: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82472660: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472664: 40810098  ble 0x824726fc
	if !ctx.cr[0].gt {
	pc = 0x824726FC; continue 'dispatch;
	}
	// 82472668: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 8247266C: 833EFFF8  lwz r25, -8(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82472670: 2F19FFFF  cmpwi cr6, r25, -1
	ctx.cr[6].compare_i32(ctx.r[25].s32, -1, &mut ctx.xer);
	// 82472674: 419A0064  beq cr6, 0x824726d8
	if ctx.cr[6].eq {
	pc = 0x824726D8; continue 'dispatch;
	}
	// 82472678: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247267C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472680: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82472684: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472688: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8247268C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472690: 4E800421  bctrl
	ctx.lr = 0x82472694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472694: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472698: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8247269C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 824726A0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824726A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824726A8: 4E800421  bctrl
	ctx.lr = 0x824726AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824726AC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824726B0: 7F16E800  cmpw cr6, r22, r29
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824726B4: 419A0024  beq cr6, 0x824726d8
	if ctx.cr[6].eq {
	pc = 0x824726D8; continue 'dispatch;
	}
	// 824726B8: 4BFFECF1  bl 0x824713a8
	ctx.lr = 0x824726BC;
	sub_824713A8(ctx, base);
	// 824726BC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 824726C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824726C4: 419A0014  beq cr6, 0x824726d8
	if ctx.cr[6].eq {
	pc = 0x824726D8; continue 'dispatch;
	}
	// 824726C8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824726CC: 7C99D214  add r4, r25, r26
	ctx.r[4].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 824726D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824726D4: 4BFFAD2D  bl 0x8246d400
	ctx.lr = 0x824726D8;
	sub_8246D400(ctx, base);
	// 824726D8: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 824726DC: 3BBD0003  addi r29, r29, 3
	ctx.r[29].s64 = ctx.r[29].s64 + 3;
	// 824726E0: 815B0020  lwz r10, 0x20(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 824726E4: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 824726E8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824726EC: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824726F0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824726F4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824726F8: 4198FF74  blt cr6, 0x8247266c
	if ctx.cr[6].lt {
	pc = 0x8247266C; continue 'dispatch;
	}
	// 824726FC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472700: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 82472704: 3AB50030  addi r21, r21, 0x30
	ctx.r[21].s64 = ctx.r[21].s64 + 48;
	// 82472708: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 8247270C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82472710: 7F145800  cmpw cr6, r20, r11
	ctx.cr[6].compare_i32(ctx.r[20].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472714: 4198FF18  blt cr6, 0x8247262c
	if ctx.cr[6].lt {
	pc = 0x8247262C; continue 'dispatch;
	}
	// 82472718: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247271C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82472720: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472724: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82472728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247272C: 419A001C  beq cr6, 0x82472748
	if ctx.cr[6].eq {
	pc = 0x82472748; continue 'dispatch;
	}
	// 82472730: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82472734: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82472738: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8247273C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472740: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82472744: 48000010  b 0x82472754
	pc = 0x82472754; continue 'dispatch;
	// 82472748: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8247274C: 4BFF16F5  bl 0x82463e40
	ctx.lr = 0x82472750;
	sub_82463E40(ctx, base);
	// 82472750: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82472754: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472758: 419A0018  beq cr6, 0x82472770
	if ctx.cr[6].eq {
	pc = 0x82472770; continue 'dispatch;
	}
	// 8247275C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82472760: 926B0000  stw r19, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 82472764: 926B0004  stw r19, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[19].u32 ) };
	// 82472768: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8247276C: 48000008  b 0x82472774
	pc = 0x82472774; continue 'dispatch;
	// 82472770: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 82472774: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247277C: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82472780: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472784: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82472788: 4E800421  bctrl
	ctx.lr = 0x8247278C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247278C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82472790: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472794: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472798: 396B8A84  addi r11, r11, -0x757c
	ctx.r[11].s64 = ctx.r[11].s64 + -30076;
	// 8247279C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 824727A0: 83AA000C  lwz r29, 0xc(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824727A4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824727A8: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824727AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824727B0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824727B4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824727B8: 4BFFAB89  bl 0x8246d340
	ctx.lr = 0x824727BC;
	sub_8246D340(ctx, base);
	// 824727BC: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 824727C0: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824727C4: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 824727C8: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 824727CC: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824727D0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824727D4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824727D8: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 824727DC: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824727E0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824727E4: 40980020  bge cr6, 0x82472804
	if !ctx.cr[6].lt {
	pc = 0x82472804; continue 'dispatch;
	}
	// 824727E8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824727EC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824727F0: 40980008  bge cr6, 0x824727f8
	if !ctx.cr[6].lt {
	pc = 0x824727F8; continue 'dispatch;
	}
	// 824727F4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 824727F8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824727FC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82472800: 4BFFBAC9  bl 0x8246e2c8
	ctx.lr = 0x82472804;
	sub_8246E2C8(ctx, base);
	// 82472804: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472808: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8247280C: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472810: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82472814: 41980028  blt cr6, 0x8247283c
	if ctx.cr[6].lt {
	pc = 0x8247283C; continue 'dispatch;
	}
	// 82472818: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8247281C: 41980020  blt cr6, 0x8247283c
	if ctx.cr[6].lt {
	pc = 0x8247283C; continue 'dispatch;
	}
	// 82472820: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472828: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8247282C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472830: 4E800421  bctrl
	ctx.lr = 0x82472834;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472834: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82472838: 48000008  b 0x82472840
	pc = 0x82472840; continue 'dispatch;
	// 8247283C: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82472840: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472844: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82472848: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8247284C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472850: 4E800421  bctrl
	ctx.lr = 0x82472854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472854: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247285C: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82472860: 4BFFEC91  bl 0x824714f0
	ctx.lr = 0x82472864;
	sub_824714F0(ctx, base);
	// 82472864: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472868: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8247286C: 48008E65  bl 0x8247b6d0
	ctx.lr = 0x82472870;
	sub_8247B6D0(ctx, base);
	// 82472870: A1770004  lhz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472874: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472878: 419A0034  beq cr6, 0x824728ac
	if ctx.cr[6].eq {
	pc = 0x824728AC; continue 'dispatch;
	}
	// 8247287C: A1770006  lhz r11, 6(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(6 as u32) ) } as u64;
	// 82472880: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82472884: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82472888: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247288C: B1770006  sth r11, 6(r23)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[23].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82472890: 409A001C  bne cr6, 0x824728ac
	if !ctx.cr[6].eq {
	pc = 0x824728AC; continue 'dispatch;
	}
	// 82472894: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472898: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8247289C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 824728A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824728A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824728A8: 4E800421  bctrl
	ctx.lr = 0x824728AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824728AC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 824728B0: 4BFFAB19  bl 0x8246d3c8
	ctx.lr = 0x824728B4;
	sub_8246D3C8(ctx, base);
	// 824728B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824728B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824728BC: 396B8A18  addi r11, r11, -0x75e8
	ctx.r[11].s64 = ctx.r[11].s64 + -30184;
	// 824728C0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824728C4: 4BFFAB05  bl 0x8246d3c8
	ctx.lr = 0x824728C8;
	sub_8246D3C8(ctx, base);
	// 824728C8: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 824728CC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 824728D0: 480C2814  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824728D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824728D8 size=636
    let mut pc: u32 = 0x824728D8;
    'dispatch: loop {
        match pc {
            0x824728D8 => {
    //   block [0x824728D8..0x82472B54)
	// 824728D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824728DC: 480C27C5  bl 0x825350a0
	ctx.lr = 0x824728E0;
	sub_82535080(ctx, base);
	// 824728E0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824728E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824728E8: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 824728EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824728F0: 48002209  bl 0x82474af8
	ctx.lr = 0x824728F4;
	sub_82474AF8(ctx, base);
	// 824728F4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824728F8: 419A0010  beq cr6, 0x82472908
	if ctx.cr[6].eq {
	pc = 0x82472908; continue 'dispatch;
	}
	// 824728FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472904: 4BFFF46D  bl 0x82471d70
	ctx.lr = 0x82472908;
	sub_82471D70(ctx, base);
	// 82472908: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8247290C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472910: 419A0020  beq cr6, 0x82472930
	if ctx.cr[6].eq {
	pc = 0x82472930; continue 'dispatch;
	}
	// 82472914: 83AB0034  lwz r29, 0x34(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82472918: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8247291C: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82472920: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472924: 4BFFAC0D  bl 0x8246d530
	ctx.lr = 0x82472928;
	sub_8246D530(ctx, base);
	// 82472928: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8247292C: 480001F4  b 0x82472b20
	pc = 0x82472B20; continue 'dispatch;
	// 82472930: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472938: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8247293C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472940: 4E800421  bctrl
	ctx.lr = 0x82472944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472944: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472948: 48002601  bl 0x82474f48
	ctx.lr = 0x8247294C;
	sub_82474F48(ctx, base);
	// 8247294C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472950: 4BFF7681  bl 0x82469fd0
	ctx.lr = 0x82472954;
	sub_82469FD0(ctx, base);
	// 82472954: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82472958: 409A0180  bne cr6, 0x82472ad8
	if !ctx.cr[6].eq {
	pc = 0x82472AD8; continue 'dispatch;
	}
	// 8247295C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82472960: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472968: 83CB92F0  lwz r30, -0x6d10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27920 as u32) ) } as u64;
	// 8247296C: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472970: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472974: 4E800421  bctrl
	ctx.lr = 0x82472978;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472978: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8247297C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82472980: 480078F9  bl 0x8247a278
	ctx.lr = 0x82472984;
	sub_8247A278(ctx, base);
	// 82472984: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82472988: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8247298C: 419A014C  beq cr6, 0x82472ad8
	if ctx.cr[6].eq {
	pc = 0x82472AD8; continue 'dispatch;
	}
	// 82472990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472994: 4BFFF94D  bl 0x824722e0
	ctx.lr = 0x82472998;
	sub_824722E0(ctx, base);
	// 82472998: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247299C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 824729A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824729A4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824729A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824729AC: 4E800421  bctrl
	ctx.lr = 0x824729B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824729B0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824729B4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824729B8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824729BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824729C0: 40990090  ble cr6, 0x82472a50
	if !ctx.cr[6].gt {
	pc = 0x82472A50; continue 'dispatch;
	}
	// 824729C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824729C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824729CC: 3B0B8E24  addi r24, r11, -0x71dc
	ctx.r[24].s64 = ctx.r[11].s64 + -29148;
	// 824729D0: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 824729D4: 83DA0000  lwz r30, 0(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824729D8: 7FABE02E  lwzx r29, r11, r28
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824729DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824729E0: 4BFF3E79  bl 0x82466858
	ctx.lr = 0x824729E4;
	sub_82466858(ctx, base);
	// 824729E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824729E8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824729EC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824729F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824729F4: 4E800421  bctrl
	ctx.lr = 0x824729F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824729F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824729FC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82472A00: 419A003C  beq cr6, 0x82472a3c
	if ctx.cr[6].eq {
	pc = 0x82472A3C; continue 'dispatch;
	}
	// 82472A04: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472A08: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82472A0C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82472A10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472A14: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82472A18: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472A1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472A20: 4E800421  bctrl
	ctx.lr = 0x82472A24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472A24: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472A28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472A2C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82472A30: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472A34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472A38: 4E800421  bctrl
	ctx.lr = 0x82472A3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472A3C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82472A40: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82472A44: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82472A48: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472A4C: 4198FF84  blt cr6, 0x824729d0
	if ctx.cr[6].lt {
	pc = 0x824729D0; continue 'dispatch;
	}
	// 82472A50: A1770004  lhz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472A54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472A58: 419A0034  beq cr6, 0x82472a8c
	if ctx.cr[6].eq {
	pc = 0x82472A8C; continue 'dispatch;
	}
	// 82472A5C: A1770006  lhz r11, 6(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(6 as u32) ) } as u64;
	// 82472A60: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82472A64: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82472A68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472A6C: B1770006  sth r11, 6(r23)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[23].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82472A70: 409A001C  bne cr6, 0x82472a8c
	if !ctx.cr[6].eq {
	pc = 0x82472A8C; continue 'dispatch;
	}
	// 82472A74: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472A78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82472A7C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82472A80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472A84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472A88: 4E800421  bctrl
	ctx.lr = 0x82472A8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472A8C: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82472A90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82472A94: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82472A98: 83AB0034  lwz r29, 0x34(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82472A9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82472AA0: 4BFFAA91  bl 0x8246d530
	ctx.lr = 0x82472AA4;
	sub_8246D530(ctx, base);
	// 82472AA4: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82472AA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472AAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82472AB0: 419A001C  beq cr6, 0x82472acc
	if ctx.cr[6].eq {
	pc = 0x82472ACC; continue 'dispatch;
	}
	// 82472AB4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472AB8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82472ABC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82472AC0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472AC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472AC8: 4E800421  bctrl
	ctx.lr = 0x82472ACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472ACC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82472AD0: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82472AD4: 4800004C  b 0x82472b20
	pc = 0x82472B20; continue 'dispatch;
	// 82472AD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472ADC: 4BFFEA15  bl 0x824714f0
	ctx.lr = 0x82472AE0;
	sub_824714F0(ctx, base);
	// 82472AE0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472AE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82472AE8: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472AEC: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472AF0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82472AF4: 41980028  blt cr6, 0x82472b1c
	if ctx.cr[6].lt {
	pc = 0x82472B1C; continue 'dispatch;
	}
	// 82472AF8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82472AFC: 41980020  blt cr6, 0x82472b1c
	if ctx.cr[6].lt {
	pc = 0x82472B1C; continue 'dispatch;
	}
	// 82472B00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472B04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472B08: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82472B0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472B10: 4E800421  bctrl
	ctx.lr = 0x82472B14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472B14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82472B18: 48000008  b 0x82472b20
	pc = 0x82472B20; continue 'dispatch;
	// 82472B1C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82472B20: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82472B24: 419A0024  beq cr6, 0x82472b48
	if ctx.cr[6].eq {
	pc = 0x82472B48; continue 'dispatch;
	}
	// 82472B28: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82472B2C: 419A001C  beq cr6, 0x82472b48
	if ctx.cr[6].eq {
	pc = 0x82472B48; continue 'dispatch;
	}
	// 82472B30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472B34: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82472B38: 4BFF7499  bl 0x82469fd0
	ctx.lr = 0x82472B3C;
	sub_82469FD0(ctx, base);
	// 82472B3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82472B40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82472B44: 409A0008  bne cr6, 0x82472b4c
	if !ctx.cr[6].eq {
	pc = 0x82472B4C; continue 'dispatch;
	}
	// 82472B48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82472B4C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82472B50: 480C25A0  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82472B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82472B58 size=620
    let mut pc: u32 = 0x82472B58;
    'dispatch: loop {
        match pc {
            0x82472B58 => {
    //   block [0x82472B58..0x82472DC4)
	// 82472B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82472B5C: 480C254D  bl 0x825350a8
	ctx.lr = 0x82472B60;
	sub_82535080(ctx, base);
	// 82472B60: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82472B64: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82472B68: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82472B6C: 570B083C  slwi r11, r24, 1
	ctx.r[11].u32 = ctx.r[24].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82472B70: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82472B74: 7D785A14  add r11, r24, r11
	ctx.r[11].u64 = ctx.r[24].u64 + ctx.r[11].u64;
	// 82472B78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82472B7C: 815B0020  lwz r10, 0x20(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472B80: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82472B84: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82472B88: 7F4B5214  add r26, r11, r10
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82472B8C: 83FA002C  lwz r31, 0x2c(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(44 as u32) ) } as u64;
	// 82472B90: 409A002C  bne cr6, 0x82472bbc
	if !ctx.cr[6].eq {
	pc = 0x82472BBC; continue 'dispatch;
	}
	// 82472B94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472B98: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82472B9C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82472BA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82472BA4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472BA8: 4BFF16C1  bl 0x82464268
	ctx.lr = 0x82472BAC;
	sub_82464268(ctx, base);
	// 82472BAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82472BB0: 807B0018  lwz r3, 0x18(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472BB4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82472BB8: 48001319  bl 0x82473ed0
	ctx.lr = 0x82472BBC;
	sub_82473ED0(ctx, base);
	// 82472BBC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472BC0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82472BC4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82472BC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82472BCC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82472BD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472BD4: 4E800421  bctrl
	ctx.lr = 0x82472BD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472BD8: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82472BDC: 409A01DC  bne cr6, 0x82472db8
	if !ctx.cr[6].eq {
	pc = 0x82472DB8; continue 'dispatch;
	}
	// 82472BE0: 817A0018  lwz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472BE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82472BE8: 815A001C  lwz r10, 0x1c(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472BEC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82472BF0: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82472BF4: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82472BF8: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82472BFC: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472C00: 40810040  ble 0x82472c40
	if !ctx.cr[0].gt {
	pc = 0x82472C40; continue 'dispatch;
	}
	// 82472C04: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472C08: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82472C0C: 419A0010  beq cr6, 0x82472c1c
	if ctx.cr[6].eq {
	pc = 0x82472C1C; continue 'dispatch;
	}
	// 82472C10: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472C14: 7D29E214  add r9, r9, r28
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[28].u64;
	// 82472C18: 7D2AE12E  stwx r9, r10, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), ctx.r[9].u32) };
	// 82472C1C: 815A001C  lwz r10, 0x1c(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472C20: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82472C24: 813A0018  lwz r9, 0x18(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472C28: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82472C2C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82472C30: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82472C34: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82472C38: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82472C3C: 4198FFC8  blt cr6, 0x82472c04
	if ctx.cr[6].lt {
	pc = 0x82472C04; continue 'dispatch;
	}
	// 82472C40: 3F208000  lis r25, -0x8000
	ctx.r[25].s64 = -2147483648;
	// 82472C44: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82472C48: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82472C4C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82472C50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82472C54: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82472C58: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 82472C5C: 48008BFD  bl 0x8247b858
	ctx.lr = 0x82472C60;
	sub_8247B858(ctx, base);
	// 82472C60: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472C64: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82472C68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472C6C: 40990034  ble cr6, 0x82472ca0
	if !ctx.cr[6].gt {
	pc = 0x82472CA0; continue 'dispatch;
	}
	// 82472C70: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82472C74: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82472C78: 807B0018  lwz r3, 0x18(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472C7C: 7D5E5A14  add r10, r30, r11
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82472C80: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472C84: 80AA0004  lwz r5, 4(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472C88: 48008229  bl 0x8247aeb0
	ctx.lr = 0x82472C8C;
	sub_8247AEB0(ctx, base);
	// 82472C8C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472C90: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82472C94: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82472C98: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472C9C: 4198FFD8  blt cr6, 0x82472c74
	if ctx.cr[6].lt {
	pc = 0x82472C74; continue 'dispatch;
	}
	// 82472CA0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82472CA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82472CA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472CAC: 409A0020  bne cr6, 0x82472ccc
	if !ctx.cr[6].eq {
	pc = 0x82472CCC; continue 'dispatch;
	}
	// 82472CB0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472CB4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82472CB8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82472CBC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82472CC0: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82472CC4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472CC8: 4BFF13F1  bl 0x824640b8
	ctx.lr = 0x82472CCC;
	sub_824640B8(ctx, base);
	// 82472CCC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82472CD0: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82472CD4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82472CD8: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82472CDC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82472CE0: 93210068  stw r25, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[25].u32 ) };
	// 82472CE4: 48008C65  bl 0x8247b948
	ctx.lr = 0x82472CE8;
	sub_8247B948(ctx, base);
	// 82472CE8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82472CEC: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82472CF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472CF4: 40990030  ble cr6, 0x82472d24
	if !ctx.cr[6].gt {
	pc = 0x82472D24; continue 'dispatch;
	}
	// 82472CF8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82472CFC: 807B0018  lwz r3, 0x18(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472D00: 7D5F5A14  add r10, r31, r11
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82472D04: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82472D08: 80AA0004  lwz r5, 4(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472D0C: 48008205  bl 0x8247af10
	ctx.lr = 0x82472D10;
	sub_8247AF10(ctx, base);
	// 82472D10: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82472D14: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82472D18: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82472D1C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472D20: 4198FFD8  blt cr6, 0x82472cf8
	if ctx.cr[6].lt {
	pc = 0x82472CF8; continue 'dispatch;
	}
	// 82472D24: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82472D28: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82472D2C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472D30: 409A0020  bne cr6, 0x82472d50
	if !ctx.cr[6].eq {
	pc = 0x82472D50; continue 'dispatch;
	}
	// 82472D34: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472D38: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82472D3C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82472D40: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82472D44: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82472D48: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472D4C: 4BFF136D  bl 0x824640b8
	ctx.lr = 0x82472D50;
	sub_824640B8(ctx, base);
	// 82472D50: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472D54: 570A103A  slwi r10, r24, 2
	ctx.r[10].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82472D58: 7F8A592E  stwx r28, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 82472D5C: 817B001C  lwz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472D60: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472D64: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472D68: 7F182000  cmpw cr6, r24, r4
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82472D6C: 409A0040  bne cr6, 0x82472dac
	if !ctx.cr[6].eq {
	pc = 0x82472DAC; continue 'dispatch;
	}
	// 82472D70: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82472D74: 41980038  blt cr6, 0x82472dac
	if ctx.cr[6].lt {
	pc = 0x82472DAC; continue 'dispatch;
	}
	// 82472D78: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82472D7C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82472D80: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82472D84: 40980028  bge cr6, 0x82472dac
	if !ctx.cr[6].lt {
	pc = 0x82472DAC; continue 'dispatch;
	}
	// 82472D88: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472D8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82472D90: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82472D94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82472D98: 4E800421  bctrl
	ctx.lr = 0x82472D9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472D9C: 4BFF3ABD  bl 0x82466858
	ctx.lr = 0x82472DA0;
	sub_82466858(ctx, base);
	// 82472DA0: 815B001C  lwz r10, 0x1c(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82472DA4: 7D7C1850  subf r11, r28, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[28].s64;
	// 82472DA8: 916A0024  stw r11, 0x24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82472DAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82472DB0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82472DB4: 480C2344  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82472DB8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82472DBC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82472DC0: 480C2338  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82472DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82472DC8 size=132
    let mut pc: u32 = 0x82472DC8;
    'dispatch: loop {
        match pc {
            0x82472DC8 => {
    //   block [0x82472DC8..0x82472E4C)
	// 82472DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82472DCC: 480C22ED  bl 0x825350b8
	ctx.lr = 0x82472DD0;
	sub_82535080(ctx, base);
	// 82472DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82472DD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82472DD8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82472DDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82472DE0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82472DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82472DE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472DEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82472DF0: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472DF4: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82472DF8: 810B0028  lwz r8, 0x28(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82472DFC: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82472E00: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82472E04: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82472E08: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82472E0C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82472E10: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82472E14: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82472E18: 4E800421  bctrl
	ctx.lr = 0x82472E1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82472E1C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82472E20: 409A0020  bne cr6, 0x82472e40
	if !ctx.cr[6].eq {
	pc = 0x82472E40; continue 'dispatch;
	}
	// 82472E24: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82472E28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82472E2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82472E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82472E34: 4BFFFD25  bl 0x82472b58
	ctx.lr = 0x82472E38;
	sub_82472B58(ctx, base);
	// 82472E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82472E3C: 480C22CC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82472E40: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82472E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82472E48: 480C22C0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82472E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82472E50 size=720
    let mut pc: u32 = 0x82472E50;
    'dispatch: loop {
        match pc {
            0x82472E50 => {
    //   block [0x82472E50..0x82473120)
	// 82472E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82472E54: 480C2251  bl 0x825350a4
	ctx.lr = 0x82472E58;
	sub_82535080(ctx, base);
	// 82472E58: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82472E5C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82472E60: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82472E64: 5497103A  slwi r23, r4, 2
	ctx.r[23].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82472E68: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82472E6C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82472E70: 813B0024  lwz r9, 0x24(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472E74: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82472E78: 815B0020  lwz r10, 0x20(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472E7C: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 82472E80: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82472E84: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82472E88: 7F37482E  lwzx r25, r23, r9
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82472E8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82472E90: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82472E94: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82472E98: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 82472E9C: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82472EA0: 48008AA9  bl 0x8247b948
	ctx.lr = 0x82472EA4;
	sub_8247B948(ctx, base);
	// 82472EA4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472EA8: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82472EAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472EB0: 40990030  ble cr6, 0x82472ee0
	if !ctx.cr[6].gt {
	pc = 0x82472EE0; continue 'dispatch;
	}
	// 82472EB4: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82472EB8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82472EBC: 807B0018  lwz r3, 0x18(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472EC0: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82472EC4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472EC8: 48007AF9  bl 0x8247a9c0
	ctx.lr = 0x82472ECC;
	sub_8247A9C0(ctx, base);
	// 82472ECC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82472ED0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82472ED4: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82472ED8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472EDC: 4198FFDC  blt cr6, 0x82472eb8
	if ctx.cr[6].lt {
	pc = 0x82472EB8; continue 'dispatch;
	}
	// 82472EE0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82472EE4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82472EE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472EEC: 409A0020  bne cr6, 0x82472f0c
	if !ctx.cr[6].eq {
	pc = 0x82472F0C; continue 'dispatch;
	}
	// 82472EF0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472EF4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82472EF8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82472EFC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82472F00: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82472F04: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472F08: 4BFF11B1  bl 0x824640b8
	ctx.lr = 0x82472F0C;
	sub_824640B8(ctx, base);
	// 82472F0C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82472F10: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82472F14: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82472F18: 93010064  stw r24, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[24].u32 ) };
	// 82472F1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82472F20: 93810068  stw r28, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 82472F24: 48008935  bl 0x8247b858
	ctx.lr = 0x82472F28;
	sub_8247B858(ctx, base);
	// 82472F28: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82472F2C: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82472F30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472F34: 40990030  ble cr6, 0x82472f64
	if !ctx.cr[6].gt {
	pc = 0x82472F64; continue 'dispatch;
	}
	// 82472F38: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82472F3C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82472F40: 807B0018  lwz r3, 0x18(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82472F44: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82472F48: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82472F4C: 48007A0D  bl 0x8247a958
	ctx.lr = 0x82472F50;
	sub_8247A958(ctx, base);
	// 82472F50: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82472F54: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82472F58: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82472F5C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82472F60: 4198FFDC  blt cr6, 0x82472f3c
	if ctx.cr[6].lt {
	pc = 0x82472F3C; continue 'dispatch;
	}
	// 82472F64: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82472F68: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82472F6C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82472F70: 409A0020  bne cr6, 0x82472f90
	if !ctx.cr[6].eq {
	pc = 0x82472F90; continue 'dispatch;
	}
	// 82472F74: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472F78: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82472F7C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82472F80: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82472F84: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82472F88: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82472F8C: 4BFF112D  bl 0x824640b8
	ctx.lr = 0x82472F90;
	sub_824640B8(ctx, base);
	// 82472F90: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82472F94: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82472F98: 7F8ACA14  add r28, r10, r25
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[25].u64;
	// 82472F9C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82472FA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82472FA4: 409A000C  bne cr6, 0x82472fb0
	if !ctx.cr[6].eq {
	pc = 0x82472FB0; continue 'dispatch;
	}
	// 82472FA8: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82472FAC: 4800005C  b 0x82473008
	pc = 0x82473008; continue 'dispatch;
	// 82472FB0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82472FB4: 3D405555  lis r10, 0x5555
	ctx.r[10].s64 = 1431633920;
	// 82472FB8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82472FBC: 614A5556  ori r10, r10, 0x5556
	ctx.r[10].u64 = ctx.r[10].u64 | 21846;
	// 82472FC0: 7D4B5096  mulhw r10, r11, r10
	ctx.r[10].s64 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 82472FC4: 55490FFE  srwi r9, r10, 0x1f
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82472FC8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82472FCC: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82472FD0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82472FD4: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82472FD8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82472FDC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82472FE0: 7D4AE214  add r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82472FE4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472FE8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82472FEC: 40980018  bge cr6, 0x82473004
	if !ctx.cr[6].lt {
	pc = 0x82473004; continue 'dispatch;
	}
	// 82472FF0: 394AFFF4  addi r10, r10, -0xc
	ctx.r[10].s64 = ctx.r[10].s64 + -12;
	// 82472FF4: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 82472FF8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82472FFC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82473000: 4198FFF0  blt cr6, 0x82472ff0
	if ctx.cr[6].lt {
	pc = 0x82472FF0; continue 'dispatch;
	}
	// 82473004: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82473008: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8247300C: 40990080  ble cr6, 0x8247308c
	if !ctx.cr[6].gt {
	pc = 0x8247308C; continue 'dispatch;
	}
	// 82473010: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82473014: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82473018: 7D6B5396  divwu r11, r11, r10
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 8247301C: 3B4B0001  addi r26, r11, 1
	ctx.r[26].s64 = ctx.r[11].s64 + 1;
	// 82473020: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82473024: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473028: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 8247302C: 7FCACA14  add r30, r10, r25
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[25].u64;
	// 82473030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473034: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82473038: 4BFFA4A1  bl 0x8246d4d8
	ctx.lr = 0x8247303C;
	sub_8246D4D8(ctx, base);
	// 8247303C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82473040: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82473044: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473048: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8247304C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82473050: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82473054: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82473058: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8247305C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82473060: 419A0010  beq cr6, 0x82473070
	if ctx.cr[6].eq {
	pc = 0x82473070; continue 'dispatch;
	}
	// 82473064: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473068: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247306C: 4E800421  bctrl
	ctx.lr = 0x82473070;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473070: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82473074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473078: 4BFFA5A9  bl 0x8246d620
	ctx.lr = 0x8247307C;
	sub_8246D620(ctx, base);
	// 8247307C: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 82473080: 3B9C000C  addi r28, r28, 0xc
	ctx.r[28].s64 = ctx.r[28].s64 + 12;
	// 82473084: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82473088: 409AFF98  bne cr6, 0x82473020
	if !ctx.cr[6].eq {
	pc = 0x82473020; continue 'dispatch;
	}
	// 8247308C: 815B0018  lwz r10, 0x18(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82473090: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82473094: 813B0024  lwz r9, 0x24(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82473098: 810A0020  lwz r8, 0x20(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8247309C: 7C97482E  lwzx r4, r23, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824730A0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824730A4: 40990068  ble cr6, 0x8247310c
	if !ctx.cr[6].gt {
	pc = 0x8247310C; continue 'dispatch;
	}
	// 824730A8: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 824730AC: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824730B0: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824730B4: 419A0028  beq cr6, 0x824730dc
	if ctx.cr[6].eq {
	pc = 0x824730DC; continue 'dispatch;
	}
	// 824730B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824730BC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824730C0: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824730C4: 4198FFE8  blt cr6, 0x824730ac
	if ctx.cr[6].lt {
	pc = 0x824730AC; continue 'dispatch;
	}
	// 824730C8: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 824730CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824730D0: 7F17592E  stwx r24, r23, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[11].u32), ctx.r[24].u32) };
	// 824730D4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824730D8: 480C201C  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 824730DC: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 824730E0: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824730E4: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 824730E8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 824730EC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824730F0: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824730F4: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824730F8: 912A0020  stw r9, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 824730FC: 7D45582E  lwzx r10, r5, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82473100: 7D47592E  stwx r10, r7, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82473104: 7C66402E  lwzx r3, r6, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82473108: 4BFF11B9  bl 0x824642c0
	ctx.lr = 0x8247310C;
	sub_824642C0(ctx, base);
	// 8247310C: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82473110: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82473114: 7F17592E  stwx r24, r23, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[11].u32), ctx.r[24].u32) };
	// 82473118: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8247311C: 480C1FD8  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82473120 size=892
    let mut pc: u32 = 0x82473120;
    'dispatch: loop {
        match pc {
            0x82473120 => {
    //   block [0x82473120..0x8247349C)
	// 82473120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473124: 480C1F79  bl 0x8253509c
	ctx.lr = 0x82473128;
	sub_82535080(ctx, base);
	// 82473128: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247312C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82473130: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82473134: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82473138: 409A0358  bne cr6, 0x82473490
	if !ctx.cr[6].eq {
	pc = 0x82473490; continue 'dispatch;
	}
	// 8247313C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473140: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82473144: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82473148: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 8247314C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82473150: 4BFF0EE9  bl 0x82464038
	ctx.lr = 0x82473154;
	sub_82464038(ctx, base);
	// 82473154: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82473158: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8247315C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82473160: 48000E59  bl 0x82473fb8
	ctx.lr = 0x82473164;
	sub_82473FB8(ctx, base);
	// 82473164: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473168: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8247316C: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 82473170: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82473174: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473178: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8247317C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82473180: 7D4B0034  cntlzw r11, r10
	ctx.r[11].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82473184: 5575DFFE  rlwinm r21, r11, 0x1b, 0x1f, 0x1f
	ctx.r[21].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82473188: 40990194  ble cr6, 0x8247331c
	if !ctx.cr[6].gt {
	pc = 0x8247331C; continue 'dispatch;
	}
	// 8247318C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82473190: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82473194: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82473198: 7D77502E  lwzx r11, r23, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8247319C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824731A0: 419A0160  beq cr6, 0x82473300
	if ctx.cr[6].eq {
	pc = 0x82473300; continue 'dispatch;
	}
	// 824731A4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824731A8: 7EA90774  extsb r9, r21
	ctx.r[9].s64 = ctx.r[21].s8 as i64;
	// 824731AC: 7F97502E  lwzx r28, r23, r10
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824731B0: 7F4BB214  add r26, r11, r22
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 824731B4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824731B8: 409A0094  bne cr6, 0x8247324c
	if !ctx.cr[6].eq {
	pc = 0x8247324C; continue 'dispatch;
	}
	// 824731BC: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 824731C0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824731C4: 815A0020  lwz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 824731C8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824731CC: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824731D0: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824731D4: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824731D8: 40810074  ble 0x8247324c
	if !ctx.cr[0].gt {
	pc = 0x8247324C; continue 'dispatch;
	}
	// 824731DC: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 824731E0: 83BEFFF8  lwz r29, -8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824731E4: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 824731E8: 419A0040  beq cr6, 0x82473228
	if ctx.cr[6].eq {
	pc = 0x82473228; continue 'dispatch;
	}
	// 824731EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824731F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824731F4: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824731F8: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824731FC: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82473200: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82473204: 4E800421  bctrl
	ctx.lr = 0x82473208;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473208: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8247320C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82473210: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82473214: 7CBDE214  add r5, r29, r28
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82473218: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247321C: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82473224: 4E800421  bctrl
	ctx.lr = 0x82473228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473228: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 8247322C: 3B7B0003  addi r27, r27, 3
	ctx.r[27].s64 = ctx.r[27].s64 + 3;
	// 82473230: 815A001C  lwz r10, 0x1c(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473234: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82473238: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8247323C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82473240: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82473244: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473248: 4198FF98  blt cr6, 0x824731e0
	if ctx.cr[6].lt {
	pc = 0x824731E0; continue 'dispatch;
	}
	// 8247324C: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82473250: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82473254: 815A0024  lwz r10, 0x24(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82473258: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8247325C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82473260: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82473264: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82473268: 40810098  ble 0x82473300
	if !ctx.cr[0].gt {
	pc = 0x82473300; continue 'dispatch;
	}
	// 8247326C: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82473270: 80BEFFF8  lwz r5, -8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82473274: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82473278: 419A0064  beq cr6, 0x824732dc
	if ctx.cr[6].eq {
	pc = 0x824732DC; continue 'dispatch;
	}
	// 8247327C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473280: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82473284: 83BEFFFC  lwz r29, -4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82473288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247328C: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473290: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82473294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82473298: 4E800421  bctrl
	ctx.lr = 0x8247329C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247329C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824732A0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824732A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824732A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824732AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824732B0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 824732B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824732B8: 4E800421  bctrl
	ctx.lr = 0x824732BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824732BC: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 824732C0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824732C4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824732C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824732CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824732D0: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 824732D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824732D8: 4E800421  bctrl
	ctx.lr = 0x824732DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824732DC: 817A0024  lwz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 824732E0: 3B390003  addi r25, r25, 3
	ctx.r[25].s64 = ctx.r[25].s64 + 3;
	// 824732E4: 815A0020  lwz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 824732E8: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 824732EC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824732F0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824732F4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824732F8: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824732FC: 4198FF74  blt cr6, 0x82473270
	if ctx.cr[6].lt {
	pc = 0x82473270; continue 'dispatch;
	}
	// 82473300: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473304: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82473308: 3AD60030  addi r22, r22, 0x30
	ctx.r[22].s64 = ctx.r[22].s64 + 48;
	// 8247330C: 3AF70004  addi r23, r23, 4
	ctx.r[23].s64 = ctx.r[23].s64 + 4;
	// 82473310: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473314: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473318: 4198FE7C  blt cr6, 0x82473194
	if ctx.cr[6].lt {
	pc = 0x82473194; continue 'dispatch;
	}
	// 8247331C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473320: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82473324: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82473328: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8247332C: 41980028  blt cr6, 0x82473354
	if ctx.cr[6].lt {
	pc = 0x82473354; continue 'dispatch;
	}
	// 82473330: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82473334: 41980020  blt cr6, 0x82473354
	if ctx.cr[6].lt {
	pc = 0x82473354; continue 'dispatch;
	}
	// 82473338: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247333C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473340: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82473344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82473348: 4E800421  bctrl
	ctx.lr = 0x8247334C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247334C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82473350: 48000008  b 0x82473358
	pc = 0x82473358; continue 'dispatch;
	// 82473354: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82473358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247335C: 4BFFE195  bl 0x824714f0
	ctx.lr = 0x82473360;
	sub_824714F0(ctx, base);
	// 82473360: 83DF0078  lwz r30, 0x78(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82473364: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82473368: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8247336C: 387E0028  addi r3, r30, 0x28
	ctx.r[3].s64 = ctx.r[30].s64 + 40;
	// 82473370: 3BBE0034  addi r29, r30, 0x34
	ctx.r[29].s64 = ctx.r[30].s64 + 52;
	// 82473374: 909E0034  stw r4, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 82473378: 4BFFA089  bl 0x8246d400
	ctx.lr = 0x8247337C;
	sub_8246D400(ctx, base);
	// 8247337C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473380: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82473384: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82473388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247338C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473390: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82473394: 4E800421  bctrl
	ctx.lr = 0x82473398;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473398: 7EAB0774  extsb r11, r21
	ctx.r[11].s64 = ctx.r[21].s8 as i64;
	// 8247339C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824733A0: 419A00F0  beq cr6, 0x82473490
	if ctx.cr[6].eq {
	pc = 0x82473490; continue 'dispatch;
	}
	// 824733A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824733A8: 4BFFEF39  bl 0x824722e0
	ctx.lr = 0x824733AC;
	sub_824722E0(ctx, base);
	// 824733AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824733B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824733B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824733B8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824733BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824733C0: 4E800421  bctrl
	ctx.lr = 0x824733C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824733C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 824733C8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824733CC: 394A8A24  addi r10, r10, -0x75dc
	ctx.r[10].s64 = ctx.r[10].s64 + -30172;
	// 824733D0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824733D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824733D8: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 824733DC: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 824733E0: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 824733E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 824733E8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 824733EC: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 824733F0: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 824733F4: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824733F8: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824733FC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82473400: 41980028  blt cr6, 0x82473428
	if ctx.cr[6].lt {
	pc = 0x82473428; continue 'dispatch;
	}
	// 82473404: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82473408: 41980020  blt cr6, 0x82473428
	if ctx.cr[6].lt {
	pc = 0x82473428; continue 'dispatch;
	}
	// 8247340C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473414: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82473418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247341C: 4E800421  bctrl
	ctx.lr = 0x82473420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473420: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82473424: 48000008  b 0x8247342c
	pc = 0x8247342C; continue 'dispatch;
	// 82473428: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8247342C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473430: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473434: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82473438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247343C: 4E800421  bctrl
	ctx.lr = 0x82473440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473440: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82473444: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82473448: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8247344C: 806B0034  lwz r3, 0x34(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82473450: 48008281  bl 0x8247b6d0
	ctx.lr = 0x82473454;
	sub_8247B6D0(ctx, base);
	// 82473454: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473458: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8247345C: 419A0034  beq cr6, 0x82473490
	if ctx.cr[6].eq {
	pc = 0x82473490; continue 'dispatch;
	}
	// 82473460: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82473464: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82473468: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8247346C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82473470: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82473474: 409A001C  bne cr6, 0x82473490
	if !ctx.cr[6].eq {
	pc = 0x82473490; continue 'dispatch;
	}
	// 82473478: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247347C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82473480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473484: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473488: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247348C: 4E800421  bctrl
	ctx.lr = 0x82473490;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473490: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82473494: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82473498: 480C1C54  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824734A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824734A0 size=1544
    let mut pc: u32 = 0x824734A0;
    'dispatch: loop {
        match pc {
            0x824734A0 => {
    //   block [0x824734A0..0x82473AA8)
	// 824734A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824734A4: 480C1BE9  bl 0x8253508c
	ctx.lr = 0x824734A8;
	sub_82535080(ctx, base);
	// 824734A8: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824734AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824734B0: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 824734B4: 7E378B78  mr r23, r17
	ctx.r[23].u64 = ctx.r[17].u64;
	// 824734B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824734BC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824734C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824734C4: 409900B8  ble cr6, 0x8247357c
	if !ctx.cr[6].gt {
	pc = 0x8247357C; continue 'dispatch;
	}
	// 824734C8: 7E398B78  mr r25, r17
	ctx.r[25].u64 = ctx.r[17].u64;
	// 824734CC: 7E388B78  mr r24, r17
	ctx.r[24].u64 = ctx.r[17].u64;
	// 824734D0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824734D4: 7D6AC82E  lwzx r11, r10, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824734D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824734DC: 419A0084  beq cr6, 0x82473560
	if ctx.cr[6].eq {
	pc = 0x82473560; continue 'dispatch;
	}
	// 824734E0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824734E4: 7E3A8B78  mr r26, r17
	ctx.r[26].u64 = ctx.r[17].u64;
	// 824734E8: 7F8AC82E  lwzx r28, r10, r25
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824734EC: 7FABC214  add r29, r11, r24
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 824734F0: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 824734F4: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 824734F8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824734FC: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82473500: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82473504: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82473508: 40810058  ble 0x82473560
	if !ctx.cr[0].gt {
	pc = 0x82473560; continue 'dispatch;
	}
	// 8247350C: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82473510: 837EFFF8  lwz r27, -8(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82473514: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 82473518: 419A0024  beq cr6, 0x8247353c
	if ctx.cr[6].eq {
	pc = 0x8247353C; continue 'dispatch;
	}
	// 8247351C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473524: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82473528: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247352C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82473530: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82473534: 4E800421  bctrl
	ctx.lr = 0x82473538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473538: 7C7BE12E  stwx r3, r27, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32), ctx.r[3].u32) };
	// 8247353C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82473540: 3B5A0003  addi r26, r26, 3
	ctx.r[26].s64 = ctx.r[26].s64 + 3;
	// 82473544: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473548: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 8247354C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82473550: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82473554: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82473558: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8247355C: 4198FFB4  blt cr6, 0x82473510
	if ctx.cr[6].lt {
	pc = 0x82473510; continue 'dispatch;
	}
	// 82473560: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473564: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 82473568: 3B180030  addi r24, r24, 0x30
	ctx.r[24].s64 = ctx.r[24].s64 + 48;
	// 8247356C: 3B390004  addi r25, r25, 4
	ctx.r[25].s64 = ctx.r[25].s64 + 4;
	// 82473570: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473574: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473578: 4198FF58  blt cr6, 0x824734d0
	if ctx.cr[6].lt {
	pc = 0x824734D0; continue 'dispatch;
	}
	// 8247357C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473580: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82473584: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82473588: 40980514  bge cr6, 0x82473a9c
	if !ctx.cr[6].lt {
	pc = 0x82473A9C; continue 'dispatch;
	}
	// 8247358C: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 82473590: 92210060  stw r17, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[17].u32 ) };
	// 82473594: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82473598: 92210064  stw r17, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[17].u32 ) };
	// 8247359C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824735A0: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 824735A4: 4BFFE22D  bl 0x824717d0
	ctx.lr = 0x824735A8;
	sub_824717D0(ctx, base);
	// 824735A8: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824735AC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824735B0: 409904B4  ble cr6, 0x82473a64
	if !ctx.cr[6].gt {
	pc = 0x82473A64; continue 'dispatch;
	}
	// 824735B4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824735B8: 92210050  stw r17, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[17].u32 ) };
	// 824735BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824735C0: 92210054  stw r17, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[17].u32 ) };
	// 824735C4: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 824735C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824735CC: 4BFFACFD  bl 0x8246e2c8
	ctx.lr = 0x824735D0;
	sub_8246E2C8(ctx, base);
	// 824735D0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 824735D4: 7E298B78  mr r9, r17
	ctx.r[9].u64 = ctx.r[17].u64;
	// 824735D8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824735DC: 4099003C  ble cr6, 0x82473618
	if !ctx.cr[6].gt {
	pc = 0x82473618; continue 'dispatch;
	}
	// 824735E0: 3D00829A  lis r8, -0x7d66
	ctx.r[8].s64 = -2103836672;
	// 824735E4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824735E8: 7E2A8B78  mr r10, r17
	ctx.r[10].u64 = ctx.r[17].u64;
	// 824735EC: 39083A78  addi r8, r8, 0x3a78
	ctx.r[8].s64 = ctx.r[8].s64 + 14968;
	// 824735F0: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824735F4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824735F8: 7CE7502E  lwzx r7, r7, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824735FC: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82473600: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82473604: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82473608: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8247360C: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82473610: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82473614: 4198FFDC  blt cr6, 0x824735f0
	if ctx.cr[6].lt {
	pc = 0x824735F0; continue 'dispatch;
	}
	// 82473618: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8247361C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82473620: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82473624: 922100AC  stw r17, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[17].u32 ) };
	// 82473628: 396B8B28  addi r11, r11, -0x74d8
	ctx.r[11].s64 = ctx.r[11].s64 + -29912;
	// 8247362C: 922100B4  stw r17, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[17].u32 ) };
	// 82473630: 922100B8  stw r17, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[17].u32 ) };
	// 82473634: 93A100BC  stw r29, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[29].u32 ) };
	// 82473638: 914100A8  stw r10, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 8247363C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82473640: 916100A0  stw r11, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82473644: B14100A6  sth r10, 0xa6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(166 as u32), ctx.r[10].u16 ) };
	// 82473648: 4BFF9CF9  bl 0x8246d340
	ctx.lr = 0x8247364C;
	sub_8246D340(ctx, base);
	// 8247364C: 3FC08293  lis r30, -0x7d6d
	ctx.r[30].s64 = -2104295424;
	// 82473650: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82473654: 817E9210  lwz r11, -0x6df0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28144 as u32) ) } as u64;
	// 82473658: 936100CC  stw r27, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[27].u32 ) };
	// 8247365C: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82473660: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82473664: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82473668: 3BAA91F8  addi r29, r10, -0x6e08
	ctx.r[29].s64 = ctx.r[10].s64 + -28168;
	// 8247366C: 409A0038  bne cr6, 0x824736a4
	if !ctx.cr[6].eq {
	pc = 0x824736A4; continue 'dispatch;
	}
	// 82473670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82473674: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82473678: 38CA8B4C  addi r6, r10, -0x74b4
	ctx.r[6].s64 = ctx.r[10].s64 + -29876;
	// 8247367C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 82473680: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82473684: 388A5238  addi r4, r10, 0x5238
	ctx.r[4].s64 = ctx.r[10].s64 + 21048;
	// 82473688: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8247368C: 917E9210  stw r11, -0x6df0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-28144 as u32), ctx.r[11].u32 ) };
	// 82473690: 48008429  bl 0x8247bab8
	ctx.lr = 0x82473694;
	sub_8247BAB8(ctx, base);
	// 82473694: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82473698: 386BCCC0  addi r3, r11, -0x3340
	ctx.r[3].s64 = ctx.r[11].s64 + -13120;
	// 8247369C: 480BF49D  bl 0x82532b38
	ctx.lr = 0x824736A0;
	sub_82532B38(ctx, base);
	// 824736A0: 817E9210  lwz r11, -0x6df0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28144 as u32) ) } as u64;
	// 824736A4: 3D400EF9  lis r10, 0xef9
	ctx.r[10].s64 = 251199488;
	// 824736A8: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824736AC: 92210080  stw r17, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[17].u32 ) };
	// 824736B0: 614A0576  ori r10, r10, 0x576
	ctx.r[10].u64 = ctx.r[10].u64 | 1398;
	// 824736B4: 92210084  stw r17, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[17].u32 ) };
	// 824736B8: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 824736BC: 9141008C  stw r10, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 824736C0: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 824736C4: 8149000C  lwz r10, 0xc(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 824736C8: 3D208273  lis r9, -0x7d8d
	ctx.r[9].s64 = -2106392576;
	// 824736CC: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 824736D0: 39295210  addi r9, r9, 0x5210
	ctx.r[9].s64 = ctx.r[9].s64 + 21008;
	// 824736D4: 40980014  bge cr6, 0x824736e8
	if !ctx.cr[6].lt {
	pc = 0x824736E8; continue 'dispatch;
	}
	// 824736D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824736DC: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 824736E0: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824736E4: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824736E8: 556A07BC  rlwinm r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824736EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824736F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 824736F4: 38AA91E8  addi r5, r10, -0x6e18
	ctx.r[5].s64 = ctx.r[10].s64 + -28184;
	// 824736F8: 409A0020  bne cr6, 0x82473718
	if !ctx.cr[6].eq {
	pc = 0x82473718; continue 'dispatch;
	}
	// 824736FC: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82473700: 917E9210  stw r11, -0x6df0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-28144 as u32), ctx.r[11].u32 ) };
	// 82473704: 7E2B8B78  mr r11, r17
	ctx.r[11].u64 = ctx.r[17].u64;
	// 82473708: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8247370C: 91250004  stw r9, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82473710: 93A50008  stw r29, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82473714: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82473718: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8247371C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82473720: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82473724: 48003705  bl 0x82476e28
	ctx.lr = 0x82473728;
	sub_82476E28(ctx, base);
	// 82473728: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247372C: 824D0000  lwz r18, 0(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473730: 3A600010  li r19, 0x10
	ctx.r[19].s64 = 16;
	// 82473734: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82473738: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8247373C: 7C73902E  lwzx r3, r19, r18
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[19].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82473740: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82473744: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82473748: 557C2036  slwi r28, r11, 4
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8247374C: 389C0030  addi r4, r28, 0x30
	ctx.r[4].s64 = ctx.r[28].s64 + 48;
	// 82473750: 4BFF0B19  bl 0x82464268
	ctx.lr = 0x82473754;
	sub_82464268(ctx, base);
	// 82473754: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82473758: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8247375C: 3BCB001C  addi r30, r11, 0x1c
	ctx.r[30].s64 = ctx.r[11].s64 + 28;
	// 82473760: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82473764: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473768: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8247376C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473770: 409A0010  bne cr6, 0x82473780
	if !ctx.cr[6].eq {
	pc = 0x82473780; continue 'dispatch;
	}
	// 82473774: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82473778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247377C: 4BFFABD5  bl 0x8246e350
	ctx.lr = 0x82473780;
	sub_8246E350(ctx, base);
	// 82473780: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473784: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82473788: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247378C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82473790: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82473794: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82473798: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247379C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824737A0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824737A4: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824737A8: 4BFF6B81  bl 0x8246a328
	ctx.lr = 0x824737AC;
	sub_8246A328(ctx, base);
	// 824737AC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824737B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 824737B4: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 824737B8: 388A8AF0  addi r4, r10, -0x7510
	ctx.r[4].s64 = ctx.r[10].s64 + -29968;
	// 824737BC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824737C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824737C4: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824737C8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824737CC: 7FDC5A14  add r30, r28, r11
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 824737D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824737D4: 4BFF6A1D  bl 0x8246a1f0
	ctx.lr = 0x824737D8;
	sub_8246A1F0(ctx, base);
	// 824737D8: 9A3E0013  stb r17, 0x13(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(19 as u32), ctx.r[17].u8 ) };
	// 824737DC: 937E0014  stw r27, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 824737E0: 3B3F0024  addi r25, r31, 0x24
	ctx.r[25].s64 = ctx.r[31].s64 + 36;
	// 824737E4: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 824737E8: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824737EC: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 824737F0: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824737F4: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 824737F8: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824737FC: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82473800: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82473804: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82473808: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8247380C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82473810: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82473814: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82473818: 81590004  lwz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247381C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82473820: 829F0028  lwz r20, 0x28(r31)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82473824: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473828: 409A0010  bne cr6, 0x82473838
	if !ctx.cr[6].eq {
	pc = 0x82473838; continue 'dispatch;
	}
	// 8247382C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82473830: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82473834: 4BFFAB1D  bl 0x8246e350
	ctx.lr = 0x82473838;
	sub_8246E350(ctx, base);
	// 82473838: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247383C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82473840: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82473844: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473848: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247384C: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82473850: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473854: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82473858: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8247385C: 4BFF9AE5  bl 0x8246d340
	ctx.lr = 0x82473860;
	sub_8246D340(ctx, base);
	// 82473860: 3BBF007C  addi r29, r31, 0x7c
	ctx.r[29].s64 = ctx.r[31].s64 + 124;
	// 82473864: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82473868: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247386C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82473870: 7F8AF214  add r28, r10, r30
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82473874: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82473878: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8247387C: 40980024  bge cr6, 0x824738a0
	if !ctx.cr[6].lt {
	pc = 0x824738A0; continue 'dispatch;
	}
	// 82473880: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82473884: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473888: 41980008  blt cr6, 0x82473890
	if ctx.cr[6].lt {
	pc = 0x82473890; continue 'dispatch;
	}
	// 8247388C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82473890: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82473894: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82473898: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8247389C: 4BFFAA2D  bl 0x8246e2c8
	ctx.lr = 0x824738A0;
	sub_8246E2C8(ctx, base);
	// 824738A0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824738A4: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824738A8: 7E3E8B78  mr r30, r17
	ctx.r[30].u64 = ctx.r[17].u64;
	// 824738AC: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 824738B0: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824738B4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824738B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824738BC: 40990048  ble cr6, 0x82473904
	if !ctx.cr[6].gt {
	pc = 0x82473904; continue 'dispatch;
	}
	// 824738C0: 7E3C8B78  mr r28, r17
	ctx.r[28].u64 = ctx.r[17].u64;
	// 824738C4: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 824738C8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824738CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824738D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824738D4: 7C8BE82E  lwzx r4, r11, r29
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824738D8: 4BFF9B29  bl 0x8246d400
	ctx.lr = 0x824738DC;
	sub_8246D400(ctx, base);
	// 824738DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824738E0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824738E4: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 824738E8: 7D6BE02E  lwzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824738EC: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 824738F0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824738F4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 824738F8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824738FC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473900: 4198FFC8  blt cr6, 0x824738c8
	if ctx.cr[6].lt {
	pc = 0x824738C8; continue 'dispatch;
	}
	// 82473904: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473908: 7E358B78  mr r21, r17
	ctx.r[21].u64 = ctx.r[17].u64;
	// 8247390C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473910: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82473914: 409900EC  ble cr6, 0x82473a00
	if !ctx.cr[6].gt {
	pc = 0x82473A00; continue 'dispatch;
	}
	// 82473918: 7E378B78  mr r23, r17
	ctx.r[23].u64 = ctx.r[17].u64;
	// 8247391C: 7E388B78  mr r24, r17
	ctx.r[24].u64 = ctx.r[17].u64;
	// 82473920: 3AC0FFFF  li r22, -1
	ctx.r[22].s64 = -1;
	// 82473924: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82473928: 7E3A8B78  mr r26, r17
	ctx.r[26].u64 = ctx.r[17].u64;
	// 8247392C: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473930: 7FB85A14  add r29, r24, r11
	ctx.r[29].u64 = ctx.r[24].u64 + ctx.r[11].u64;
	// 82473934: 7F8AB82E  lwzx r28, r10, r23
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82473938: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247393C: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82473940: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82473944: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82473948: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 8247394C: 7D4A0195  addze. r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82473950: 40810094  ble 0x824739e4
	if !ctx.cr[0].gt {
	pc = 0x824739E4; continue 'dispatch;
	}
	// 82473954: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82473958: 837EFFF8  lwz r27, -8(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8247395C: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 82473960: 419A0060  beq cr6, 0x824739c0
	if ctx.cr[6].eq {
	pc = 0x824739C0; continue 'dispatch;
	}
	// 82473964: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247396C: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82473970: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473974: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82473978: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247397C: 4E800421  bctrl
	ctx.lr = 0x82473980;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473980: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82473984: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82473988: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8247398C: 4BFF9BA5  bl 0x8246d530
	ctx.lr = 0x82473990;
	sub_8246D530(ctx, base);
	// 82473990: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82473994: 419A002C  beq cr6, 0x824739c0
	if ctx.cr[6].eq {
	pc = 0x824739C0; continue 'dispatch;
	}
	// 82473998: 929EFFFC  stw r20, -4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), ctx.r[20].u32 ) };
	// 8247399C: 546B1838  slwi r11, r3, 3
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824739A0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824739A4: 812100AC  lwz r9, 0xac(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 824739A8: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824739AC: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 824739B0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824739B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824739B8: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824739BC: 7D7BE12E  stwx r11, r27, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u32) };
	// 824739C0: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 824739C4: 3B5A0003  addi r26, r26, 3
	ctx.r[26].s64 = ctx.r[26].s64 + 3;
	// 824739C8: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 824739CC: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 824739D0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824739D4: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824739D8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824739DC: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824739E0: 4198FF78  blt cr6, 0x82473958
	if ctx.cr[6].lt {
	pc = 0x82473958; continue 'dispatch;
	}
	// 824739E4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824739E8: 3AB50001  addi r21, r21, 1
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	// 824739EC: 3B180030  addi r24, r24, 0x30
	ctx.r[24].s64 = ctx.r[24].s64 + 48;
	// 824739F0: 3AF70004  addi r23, r23, 4
	ctx.r[23].s64 = ctx.r[23].s64 + 4;
	// 824739F4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824739F8: 7F155800  cmpw cr6, r21, r11
	ctx.cr[6].compare_i32(ctx.r[21].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824739FC: 4198FF28  blt cr6, 0x82473924
	if ctx.cr[6].lt {
	pc = 0x82473924; continue 'dispatch;
	}
	// 82473A00: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82473A04: 4BFF99C5  bl 0x8246d3c8
	ctx.lr = 0x82473A08;
	sub_8246D3C8(ctx, base);
	// 82473A08: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82473A0C: 4BFF99BD  bl 0x8246d3c8
	ctx.lr = 0x82473A10;
	sub_8246D3C8(ctx, base);
	// 82473A10: 816100BC  lwz r11, 0xbc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 82473A14: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82473A18: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82473A1C: 409A0018  bne cr6, 0x82473a34
	if !ctx.cr[6].eq {
	pc = 0x82473A34; continue 'dispatch;
	}
	// 82473A20: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82473A24: 7C73902E  lwzx r3, r19, r18
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[19].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82473A28: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82473A2C: 808100B4  lwz r4, 0xb4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82473A30: 4BFF0689  bl 0x824640b8
	ctx.lr = 0x82473A34;
	sub_824640B8(ctx, base);
	// 82473A34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82473A38: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82473A3C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82473A40: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82473A44: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82473A48: 916100A0  stw r11, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82473A4C: 409A0018  bne cr6, 0x82473a64
	if !ctx.cr[6].eq {
	pc = 0x82473A64; continue 'dispatch;
	}
	// 82473A50: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82473A54: 7C73902E  lwzx r3, r19, r18
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[19].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82473A58: 55451838  slwi r5, r10, 3
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82473A5C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82473A60: 4BFF0659  bl 0x824640b8
	ctx.lr = 0x82473A64;
	sub_824640B8(ctx, base);
	// 82473A64: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82473A68: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82473A6C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82473A70: 409A002C  bne cr6, 0x82473a9c
	if !ctx.cr[6].eq {
	pc = 0x82473A9C; continue 'dispatch;
	}
	// 82473A74: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473A78: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82473A7C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82473A80: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82473A84: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82473A88: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82473A8C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82473A90: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82473A94: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82473A98: 4BFF0621  bl 0x824640b8
	ctx.lr = 0x82473A9C;
	sub_824640B8(ctx, base);
	// 82473A9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82473AA0: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82473AA4: 480C1638  b 0x825350dc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82473AA8 size=168
    let mut pc: u32 = 0x82473AA8;
    'dispatch: loop {
        match pc {
            0x82473AA8 => {
    //   block [0x82473AA8..0x82473B50)
	// 82473AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473AAC: 480C1611  bl 0x825350bc
	ctx.lr = 0x82473AB0;
	sub_82535080(ctx, base);
	// 82473AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82473AB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82473AB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82473ABC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82473AC0: 4BFFDFA9  bl 0x82471a68
	ctx.lr = 0x82473AC4;
	sub_82471A68(ctx, base);
	// 82473AC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82473AC8: 409A007C  bne cr6, 0x82473b44
	if !ctx.cr[6].eq {
	pc = 0x82473B44; continue 'dispatch;
	}
	// 82473ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82473AD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82473AD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473AD8: 4BFFE0C9  bl 0x82471ba0
	ctx.lr = 0x82473ADC;
	sub_82471BA0(ctx, base);
	// 82473ADC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82473AE0: 409A0064  bne cr6, 0x82473b44
	if !ctx.cr[6].eq {
	pc = 0x82473B44; continue 'dispatch;
	}
	// 82473AE4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473AE8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82473AEC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473AF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82473AF4: 40990034  ble cr6, 0x82473b28
	if !ctx.cr[6].gt {
	pc = 0x82473B28; continue 'dispatch;
	}
	// 82473AF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82473AFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82473B00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82473B04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473B08: 4BFFF051  bl 0x82472b58
	ctx.lr = 0x82473B0C;
	sub_82472B58(ctx, base);
	// 82473B0C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82473B10: 419A0034  beq cr6, 0x82473b44
	if ctx.cr[6].eq {
	pc = 0x82473B44; continue 'dispatch;
	}
	// 82473B14: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473B18: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82473B1C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473B20: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473B24: 4198FFD4  blt cr6, 0x82473af8
	if ctx.cr[6].lt {
	pc = 0x82473AF8; continue 'dispatch;
	}
	// 82473B28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473B2C: 4BFFF975  bl 0x824734a0
	ctx.lr = 0x82473B30;
	sub_824734A0(ctx, base);
	// 82473B30: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82473B34: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82473B38: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82473B3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82473B40: 480C15CC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82473B44: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82473B48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82473B4C: 480C15C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82473B50 size=624
    let mut pc: u32 = 0x82473B50;
    'dispatch: loop {
        match pc {
            0x82473B50 => {
    //   block [0x82473B50..0x82473DC0)
	// 82473B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473B54: 480C1565  bl 0x825350b8
	ctx.lr = 0x82473B58;
	sub_82535080(ctx, base);
	// 82473B58: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82473B5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82473B60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82473B64: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82473B68: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82473B6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82473B70: 4BFF67C9  bl 0x8246a338
	ctx.lr = 0x82473B74;
	sub_8246A338(ctx, base);
	// 82473B74: 3D6057E0  lis r11, 0x57e0
	ctx.r[11].s64 = 1474297856;
	// 82473B78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82473B7C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473B80: 616BE057  ori r11, r11, 0xe057
	ctx.r[11].u64 = ctx.r[11].u64 | 57431;
	// 82473B84: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82473B88: 99010078  stb r8, 0x78(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[8].u8 ) };
	// 82473B8C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82473B90: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82473B94: 3D6010C0  lis r11, 0x10c0
	ctx.r[11].s64 = 281018368;
	// 82473B98: 616BC010  ori r11, r11, 0xc010
	ctx.r[11].u64 = ctx.r[11].u64 | 49168;
	// 82473B9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82473BA0: 409A0214  bne cr6, 0x82473db4
	if !ctx.cr[6].eq {
	pc = 0x82473DB4; continue 'dispatch;
	}
	// 82473BA4: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473BA8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473BAC: 409A0208  bne cr6, 0x82473db4
	if !ctx.cr[6].eq {
	pc = 0x82473DB4; continue 'dispatch;
	}
	// 82473BB0: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82473BB4: 389C0028  addi r4, r28, 0x28
	ctx.r[4].s64 = ctx.r[28].s64 + 40;
	// 82473BB8: 897C0028  lbz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82473BBC: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82473BC0: 409A0034  bne cr6, 0x82473bf4
	if !ctx.cr[6].eq {
	pc = 0x82473BF4; continue 'dispatch;
	}
	// 82473BC4: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82473BC8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82473BCC: 409A0010  bne cr6, 0x82473bdc
	if !ctx.cr[6].eq {
	pc = 0x82473BDC; continue 'dispatch;
	}
	// 82473BD0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82473BD4: 388B8A38  addi r4, r11, -0x75c8
	ctx.r[4].s64 = ctx.r[11].s64 + -30152;
	// 82473BD8: 4800001C  b 0x82473bf4
	pc = 0x82473BF4; continue 'dispatch;
	// 82473BDC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82473BE0: 409A0010  bne cr6, 0x82473bf0
	if !ctx.cr[6].eq {
	pc = 0x82473BF0; continue 'dispatch;
	}
	// 82473BE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82473BE8: 388B8A2C  addi r4, r11, -0x75d4
	ctx.r[4].s64 = ctx.r[11].s64 + -30164;
	// 82473BEC: 48000008  b 0x82473bf4
	pc = 0x82473BF4; continue 'dispatch;
	// 82473BF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82473BF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473BF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473BFC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82473C00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82473C04: 4E800421  bctrl
	ctx.lr = 0x82473C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473C08: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473C0C: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473C10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82473C14: 397C0040  addi r11, r28, 0x40
	ctx.r[11].s64 = ctx.r[28].s64 + 64;
	// 82473C18: 41990008  bgt cr6, 0x82473c20
	if ctx.cr[6].gt {
	pc = 0x82473C20; continue 'dispatch;
	}
	// 82473C1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82473C20: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82473C24: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82473C28: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82473C2C: 40980030  bge cr6, 0x82473c5c
	if !ctx.cr[6].lt {
	pc = 0x82473C5C; continue 'dispatch;
	}
	// 82473C30: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473C34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82473C38: 40990024  ble cr6, 0x82473c5c
	if !ctx.cr[6].gt {
	pc = 0x82473C5C; continue 'dispatch;
	}
	// 82473C3C: 396B002C  addi r11, r11, 0x2c
	ctx.r[11].s64 = ctx.r[11].s64 + 44;
	// 82473C40: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82473C44: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82473C48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82473C4C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82473C50: 912BFFF8  stw r9, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 82473C54: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82473C58: 409AFFE8  bne cr6, 0x82473c40
	if !ctx.cr[6].eq {
	pc = 0x82473C40; continue 'dispatch;
	}
	// 82473C5C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473C60: 3BBE0024  addi r29, r30, 0x24
	ctx.r[29].s64 = ctx.r[30].s64 + 36;
	// 82473C64: 83EB0014  lwz r31, 0x14(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473C68: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82473C6C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82473C70: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82473C74: 40980024  bge cr6, 0x82473c98
	if !ctx.cr[6].lt {
	pc = 0x82473C98; continue 'dispatch;
	}
	// 82473C78: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82473C7C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473C80: 41980008  blt cr6, 0x82473c88
	if ctx.cr[6].lt {
	pc = 0x82473C88; continue 'dispatch;
	}
	// 82473C84: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82473C88: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82473C8C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82473C90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82473C94: 4BFFA635  bl 0x8246e2c8
	ctx.lr = 0x82473C98;
	sub_8246E2C8(ctx, base);
	// 82473C98: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82473C9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82473CA0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473CA4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473CA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82473CAC: 4099009C  ble cr6, 0x82473d48
	if !ctx.cr[6].gt {
	pc = 0x82473D48; continue 'dispatch;
	}
	// 82473CB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82473CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82473CB8: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82473CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82473CC0: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82473CC4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82473CC8: 810B001C  lwz r8, 0x1c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473CCC: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473CD0: 7D0A4050  subf r8, r10, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82473CD4: 7D29E214  add r9, r9, r28
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[28].u64;
	// 82473CD8: 7D081670  srawi r8, r8, 2
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 2) as i64;
	// 82473CDC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82473CE0: 7D080195  addze. r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82473CE4: 40810040  ble 0x82473d24
	if !ctx.cr[0].gt {
	pc = 0x82473D24; continue 'dispatch;
	}
	// 82473CE8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473CEC: 2F08FFFF  cmpwi cr6, r8, -1
	ctx.cr[6].compare_i32(ctx.r[8].s32, -1, &mut ctx.xer);
	// 82473CF0: 419A0010  beq cr6, 0x82473d00
	if ctx.cr[6].eq {
	pc = 0x82473D00; continue 'dispatch;
	}
	// 82473CF4: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473CF8: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82473CFC: 7CE8492E  stwx r7, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[7].u32) };
	// 82473D00: 810B001C  lwz r8, 0x1c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473D04: 38C60002  addi r6, r6, 2
	ctx.r[6].s64 = ctx.r[6].s64 + 2;
	// 82473D08: 80EB0018  lwz r7, 0x18(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82473D0C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82473D10: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82473D14: 7D081670  srawi r8, r8, 2
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 2) as i64;
	// 82473D18: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82473D1C: 7F064000  cmpw cr6, r6, r8
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82473D20: 4198FFC8  blt cr6, 0x82473ce8
	if ctx.cr[6].lt {
	pc = 0x82473CE8; continue 'dispatch;
	}
	// 82473D24: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473D28: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82473D2C: 38A50030  addi r5, r5, 0x30
	ctx.r[5].s64 = ctx.r[5].s64 + 48;
	// 82473D30: 7D24592E  stwx r9, r4, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82473D34: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82473D38: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473D3C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82473D40: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473D44: 4198FF74  blt cr6, 0x82473cb8
	if ctx.cr[6].lt {
	pc = 0x82473CB8; continue 'dispatch;
	}
	// 82473D48: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473D4C: 83EB0020  lwz r31, 0x20(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82473D50: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82473D54: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82473D58: 4198004C  blt cr6, 0x82473da4
	if ctx.cr[6].lt {
	pc = 0x82473DA4; continue 'dispatch;
	}
	// 82473D5C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82473D60: 41980044  blt cr6, 0x82473da4
	if ctx.cr[6].lt {
	pc = 0x82473DA4; continue 'dispatch;
	}
	// 82473D64: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82473D68: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82473D6C: 40980038  bge cr6, 0x82473da4
	if !ctx.cr[6].lt {
	pc = 0x82473DA4; continue 'dispatch;
	}
	// 82473D70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473D74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82473D78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473D7C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82473D80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82473D84: 4E800421  bctrl
	ctx.lr = 0x82473D88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82473D88: 4BFF2AD1  bl 0x82466858
	ctx.lr = 0x82473D8C;
	sub_82466858(ctx, base);
	// 82473D8C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473D90: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82473D94: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82473D98: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82473D9C: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82473DA0: 91690024  stw r11, 0x24(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82473DA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82473DA8: 4BFFF6F9  bl 0x824734a0
	ctx.lr = 0x82473DAC;
	sub_824734A0(ctx, base);
	// 82473DAC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82473DB0: 480C1358  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82473DB4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82473DB8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82473DBC: 480C134C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82473DC0 size=100
    let mut pc: u32 = 0x82473DC0;
    'dispatch: loop {
        match pc {
            0x82473DC0 => {
    //   block [0x82473DC0..0x82473E24)
	// 82473DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82473DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82473DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82473DD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82473DD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82473DD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82473DDC: 48006DDD  bl 0x8247abb8
	ctx.lr = 0x82473DE0;
	sub_8247ABB8(ctx, base);
	// 82473DE0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82473DE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82473DE8: 419A0020  beq cr6, 0x82473e08
	if ctx.cr[6].eq {
	pc = 0x82473E08; continue 'dispatch;
	}
	// 82473DEC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473DF0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82473DF4: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82473DF8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473DFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82473E00: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82473E04: 4BFF02B5  bl 0x824640b8
	ctx.lr = 0x82473E08;
	sub_824640B8(ctx, base);
	// 82473E08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473E0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82473E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82473E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82473E18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82473E1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82473E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82473E28 size=96
    let mut pc: u32 = 0x82473E28;
    'dispatch: loop {
        match pc {
            0x82473E28 => {
    //   block [0x82473E28..0x82473E88)
	// 82473E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82473E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82473E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82473E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82473E3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82473E40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82473E44: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82473E48: 4BFF9581  bl 0x8246d3c8
	ctx.lr = 0x82473E4C;
	sub_8246D3C8(ctx, base);
	// 82473E4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82473E50: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82473E54: 396B8A18  addi r11, r11, -0x75e8
	ctx.r[11].s64 = ctx.r[11].s64 + -30184;
	// 82473E58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82473E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473E60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82473E64: 419A000C  beq cr6, 0x82473e70
	if ctx.cr[6].eq {
	pc = 0x82473E70; continue 'dispatch;
	}
	// 82473E68: 480BED51  bl 0x82532bb8
	ctx.lr = 0x82473E6C;
	sub_82532BB8(ctx, base);
	// 82473E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473E70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82473E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82473E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82473E7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82473E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82473E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82473E88 size=72
    let mut pc: u32 = 0x82473E88;
    'dispatch: loop {
        match pc {
            0x82473E88 => {
    //   block [0x82473E88..0x82473ED0)
	// 82473E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82473E90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82473E94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82473E98: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82473E9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82473EA0: 396B8A18  addi r11, r11, -0x75e8
	ctx.r[11].s64 = ctx.r[11].s64 + -30184;
	// 82473EA4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82473EA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82473EAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82473EB0: 419A000C  beq cr6, 0x82473ebc
	if ctx.cr[6].eq {
	pc = 0x82473EBC; continue 'dispatch;
	}
	// 82473EB4: 480BED05  bl 0x82532bb8
	ctx.lr = 0x82473EB8;
	sub_82532BB8(ctx, base);
	// 82473EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473EBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82473EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82473EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82473EC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82473ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82473ED0 size=112
    let mut pc: u32 = 0x82473ED0;
    'dispatch: loop {
        match pc {
            0x82473ED0 => {
    //   block [0x82473ED0..0x82473F40)
	// 82473ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82473ED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82473EDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82473EE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82473EE4: 3BE3001C  addi r31, r3, 0x1c
	ctx.r[31].s64 = ctx.r[3].s64 + 28;
	// 82473EE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82473EEC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82473EF0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473EF4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82473EF8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82473EFC: 409A0010  bne cr6, 0x82473f0c
	if !ctx.cr[6].eq {
	pc = 0x82473F0C; continue 'dispatch;
	}
	// 82473F00: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82473F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82473F08: 4BFFA449  bl 0x8246e350
	ctx.lr = 0x82473F0C;
	sub_8246E350(ctx, base);
	// 82473F0C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473F10: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473F14: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82473F18: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82473F1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82473F20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82473F24: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82473F28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82473F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82473F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82473F34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82473F38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82473F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82473F40 size=116
    let mut pc: u32 = 0x82473F40;
    'dispatch: loop {
        match pc {
            0x82473F40 => {
    //   block [0x82473F40..0x82473FB4)
	// 82473F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82473F48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82473F4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82473F50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82473F54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82473F58: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 82473F5C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82473F60: 4BFF9469  bl 0x8246d3c8
	ctx.lr = 0x82473F64;
	sub_8246D3C8(ctx, base);
	// 82473F64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82473F68: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82473F6C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82473F70: 409A0020  bne cr6, 0x82473f90
	if !ctx.cr[6].eq {
	pc = 0x82473F90; continue 'dispatch;
	}
	// 82473F74: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473F78: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82473F7C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82473F80: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82473F84: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82473F88: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82473F8C: 4BFF012D  bl 0x824640b8
	ctx.lr = 0x82473F90;
	sub_824640B8(ctx, base);
	// 82473F90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82473F94: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82473F98: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82473F9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82473FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82473FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82473FA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82473FAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82473FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82473FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82473FB8 size=132
    let mut pc: u32 = 0x82473FB8;
    'dispatch: loop {
        match pc {
            0x82473FB8 => {
    //   block [0x82473FB8..0x8247403C)
	// 82473FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82473FBC: 480C1101  bl 0x825350bc
	ctx.lr = 0x82473FC0;
	sub_82535080(ctx, base);
	// 82473FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82473FC4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82473FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82473FCC: 396B8B00  addi r11, r11, -0x7500
	ctx.r[11].s64 = ctx.r[11].s64 + -29952;
	// 82473FD0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82473FD4: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82473FD8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82473FDC: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82473FE0: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82473FE4: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82473FE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82473FEC: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82473FF0: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82473FF4: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82473FF8: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82473FFC: 4BFF9345  bl 0x8246d340
	ctx.lr = 0x82474000;
	sub_8246D340(ctx, base);
	// 82474000: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82474004: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82474008: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8247400C: 4BFF9335  bl 0x8246d340
	ctx.lr = 0x82474010;
	sub_8246D340(ctx, base);
	// 82474010: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474018: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 8247401C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474020: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82474024: 419A0010  beq cr6, 0x82474034
	if ctx.cr[6].eq {
	pc = 0x82474034; continue 'dispatch;
	}
	// 82474028: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8247402C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82474030: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82474034: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474038: 480C10D4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474040 size=8
    let mut pc: u32 = 0x82474040;
    'dispatch: loop {
        match pc {
            0x82474040 => {
    //   block [0x82474040..0x82474048)
	// 82474040: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82474044: 4BFF93BC  b 0x8246d400
	sub_8246D400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474048 size=8
    let mut pc: u32 = 0x82474048;
    'dispatch: loop {
        match pc {
            0x82474048 => {
    //   block [0x82474048..0x82474050)
	// 82474048: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 8247404C: 4BFF9BAC  b 0x8246dbf8
	sub_8246DBF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474050 size=132
    let mut pc: u32 = 0x82474050;
    'dispatch: loop {
        match pc {
            0x82474050 => {
    //   block [0x82474050..0x824740D4)
	// 82474050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8247405C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474064: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474068: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8247406C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82474070: 419A001C  beq cr6, 0x8247408c
	if ctx.cr[6].eq {
	pc = 0x8247408C; continue 'dispatch;
	}
	// 82474074: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474078: 546B1838  slwi r11, r3, 3
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247407C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82474080: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474084: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82474088: 48000034  b 0x824740bc
	pc = 0x824740BC; continue 'dispatch;
	// 8247408C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474090: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474094: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82474098: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8247409C: 409A0010  bne cr6, 0x824740ac
	if !ctx.cr[6].eq {
	pc = 0x824740AC; continue 'dispatch;
	}
	// 824740A0: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824740A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824740A8: 4BFFA2A9  bl 0x8246e350
	ctx.lr = 0x824740AC;
	sub_8246E350(ctx, base);
	// 824740AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824740B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824740B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824740B8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824740BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824740C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824740C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824740C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824740CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824740D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824740D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824740D8 size=116
    let mut pc: u32 = 0x824740D8;
    'dispatch: loop {
        match pc {
            0x824740D8 => {
    //   block [0x824740D8..0x8247414C)
	// 824740D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824740DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824740E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824740E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824740E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824740EC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824740F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824740F4: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 824740F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824740FC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474100: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82474104: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82474108: 409A0010  bne cr6, 0x82474118
	if !ctx.cr[6].eq {
	pc = 0x82474118; continue 'dispatch;
	}
	// 8247410C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82474110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474114: 4BFFA23D  bl 0x8246e350
	ctx.lr = 0x82474118;
	sub_8246E350(ctx, base);
	// 82474118: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247411C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474120: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82474124: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82474128: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247412C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82474130: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82474134: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8247413C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474140: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82474144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474150 size=120
    let mut pc: u32 = 0x82474150;
    'dispatch: loop {
        match pc {
            0x82474150 => {
    //   block [0x82474150..0x824741C8)
	// 82474150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474154: 480C0F65  bl 0x825350b8
	ctx.lr = 0x82474158;
	sub_82535080(ctx, base);
	// 82474158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247415C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474160: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82474164: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82474168: 3BEB0028  addi r31, r11, 0x28
	ctx.r[31].s64 = ctx.r[11].s64 + 40;
	// 8247416C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82474170: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474174: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474178: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8247417C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82474180: 409A0010  bne cr6, 0x82474190
	if !ctx.cr[6].eq {
	pc = 0x82474190; continue 'dispatch;
	}
	// 82474184: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82474188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247418C: 4BFFA1C5  bl 0x8246e350
	ctx.lr = 0x82474190;
	sub_8246E350(ctx, base);
	// 82474190: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474194: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474198: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8247419C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824741A0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824741A4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824741A8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824741AC: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824741B0: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 824741B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824741B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824741BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824741C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824741C4: 480C0F44  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824741C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824741C8 size=264
    let mut pc: u32 = 0x824741C8;
    'dispatch: loop {
        match pc {
            0x824741C8 => {
    //   block [0x824741C8..0x824742D0)
	// 824741C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824741CC: 480C0EE5  bl 0x825350b0
	ctx.lr = 0x824741D0;
	sub_82535080(ctx, base);
	// 824741D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824741D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824741D8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824741DC: 3BBE0018  addi r29, r30, 0x18
	ctx.r[29].s64 = ctx.r[30].s64 + 24;
	// 824741E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 824741E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824741E8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824741EC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 824741F0: 4BFF9341  bl 0x8246d530
	ctx.lr = 0x824741F4;
	sub_8246D530(ctx, base);
	// 824741F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824741F8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824741FC: 419A0014  beq cr6, 0x82474210
	if ctx.cr[6].eq {
	pc = 0x82474210; continue 'dispatch;
	}
	// 82474200: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82474204: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82474208: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8247420C: 4BFF91F5  bl 0x8246d400
	ctx.lr = 0x82474210;
	sub_8246D400(ctx, base);
	// 82474210: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82474214: 419A0028  beq cr6, 0x8247423c
	if ctx.cr[6].eq {
	pc = 0x8247423C; continue 'dispatch;
	}
	// 82474218: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8247421C: 57EA1838  slwi r10, r31, 3
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82474220: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82474224: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82474228: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8247422C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82474230: 83EA0004  lwz r31, 4(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474234: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82474238: 409AFFE4  bne cr6, 0x8247421c
	if !ctx.cr[6].eq {
	pc = 0x8247421C; continue 'dispatch;
	}
	// 8247423C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474240: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82474244: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82474248: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8247424C: 40990038  ble cr6, 0x82474284
	if !ctx.cr[6].gt {
	pc = 0x82474284; continue 'dispatch;
	}
	// 82474250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82474254: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82474258: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247425C: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474260: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82474264: 409A0008  bne cr6, 0x8247426c
	if !ctx.cr[6].eq {
	pc = 0x8247426C; continue 'dispatch;
	}
	// 82474268: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8247426C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474270: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82474274: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82474278: 810B0038  lwz r8, 0x38(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8247427C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82474280: 4198FFD4  blt cr6, 0x82474254
	if ctx.cr[6].lt {
	pc = 0x82474254; continue 'dispatch;
	}
	// 82474284: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474288: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8247428C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82474290: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82474294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474298: 4E800421  bctrl
	ctx.lr = 0x8247429C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8247429C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824742A0: 419A0028  beq cr6, 0x824742c8
	if ctx.cr[6].eq {
	pc = 0x824742C8; continue 'dispatch;
	}
	// 824742A4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824742A8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824742AC: 4BFF25AD  bl 0x82466858
	ctx.lr = 0x824742B0;
	sub_82466858(ctx, base);
	// 824742B0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824742B4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824742B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824742BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824742C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824742C4: 4E800421  bctrl
	ctx.lr = 0x824742C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824742C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824742CC: 480C0E34  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824742D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824742D0 size=220
    let mut pc: u32 = 0x824742D0;
    'dispatch: loop {
        match pc {
            0x824742D0 => {
    //   block [0x824742D0..0x824743AC)
	// 824742D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824742D4: 480C0DE1  bl 0x825350b4
	ctx.lr = 0x824742D8;
	sub_82535080(ctx, base);
	// 824742D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824742DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824742E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824742E4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824742E8: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 824742EC: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824742F0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824742F4: 419A005C  beq cr6, 0x82474350
	if ctx.cr[6].eq {
	pc = 0x82474350; continue 'dispatch;
	}
	// 824742F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824742FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82474300: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82474304: 4BFF922D  bl 0x8246d530
	ctx.lr = 0x82474308;
	sub_8246D530(ctx, base);
	// 82474308: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8247430C: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82474310: 419A0040  beq cr6, 0x82474350
	if ctx.cr[6].eq {
	pc = 0x82474350; continue 'dispatch;
	}
	// 82474314: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82474318: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8247431C: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82474320: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82474324: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474328: 7F09E040  cmplw cr6, r9, r28
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8247432C: 419A0014  beq cr6, 0x82474340
	if ctx.cr[6].eq {
	pc = 0x82474340; continue 'dispatch;
	}
	// 82474330: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474334: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82474338: 409AFFE4  bne cr6, 0x8247431c
	if !ctx.cr[6].eq {
	pc = 0x8247431C; continue 'dispatch;
	}
	// 8247433C: 48000014  b 0x82474350
	pc = 0x82474350; continue 'dispatch;
	// 82474340: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82474344: 419A0060  beq cr6, 0x824743a4
	if ctx.cr[6].eq {
	pc = 0x824743A4; continue 'dispatch;
	}
	// 82474348: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8247434C: 4800B4BD  bl 0x8247f808
	ctx.lr = 0x82474350;
	sub_8247F808(ctx, base);
	// 82474350: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82474354: 419A004C  beq cr6, 0x824743a0
	if ctx.cr[6].eq {
	pc = 0x824743A0; continue 'dispatch;
	}
	// 82474358: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 8247435C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82474360: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82474364: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82474368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247436C: 4BFF91C5  bl 0x8246d530
	ctx.lr = 0x82474370;
	sub_8246D530(ctx, base);
	// 82474370: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82474374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474378: 4BFFFCD9  bl 0x82474050
	ctx.lr = 0x8247437C;
	sub_82474050(ctx, base);
	// 8247437C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82474380: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474384: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82474388: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247438C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82474390: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82474394: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82474398: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8247439C: 4BFF9065  bl 0x8246d400
	ctx.lr = 0x824743A0;
	sub_8246D400(ctx, base);
	// 824743A0: 937C0000  stw r27, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 824743A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824743A8: 480C0D5C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824743B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824743B0 size=196
    let mut pc: u32 = 0x824743B0;
    'dispatch: loop {
        match pc {
            0x824743B0 => {
    //   block [0x824743B0..0x82474474)
	// 824743B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824743B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824743B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824743BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824743C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824743C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824743C8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824743CC: 396B8B00  addi r11, r11, -0x7500
	ctx.r[11].s64 = ctx.r[11].s64 + -29952;
	// 824743D0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824743D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824743D8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824743DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824743E0: 419A0030  beq cr6, 0x82474410
	if ctx.cr[6].eq {
	pc = 0x82474410; continue 'dispatch;
	}
	// 824743E4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824743E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824743EC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824743F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824743F4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824743F8: 409A0018  bne cr6, 0x82474410
	if !ctx.cr[6].eq {
	pc = 0x82474410; continue 'dispatch;
	}
	// 824743FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474400: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82474404: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247440C: 4E800421  bctrl
	ctx.lr = 0x82474410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474410: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82474414: 4BFF8FB5  bl 0x8246d3c8
	ctx.lr = 0x82474418;
	sub_8246D3C8(ctx, base);
	// 82474418: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 8247441C: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82474420: 4BFF8FA9  bl 0x8246d3c8
	ctx.lr = 0x82474424;
	sub_8246D3C8(ctx, base);
	// 82474424: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474428: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8247442C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82474430: 409A0020  bne cr6, 0x82474450
	if !ctx.cr[6].eq {
	pc = 0x82474450; continue 'dispatch;
	}
	// 82474434: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474438: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8247443C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82474440: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474444: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82474448: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8247444C: 4BFEFC6D  bl 0x824640b8
	ctx.lr = 0x82474450;
	sub_824640B8(ctx, base);
	// 82474450: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82474454: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82474458: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8247445C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474468: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8247446C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474478 size=100
    let mut pc: u32 = 0x82474478;
    'dispatch: loop {
        match pc {
            0x82474478 => {
    //   block [0x82474478..0x824744DC)
	// 82474478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247447C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82474484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247448C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474490: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82474494: 4BFFFF1D  bl 0x824743b0
	ctx.lr = 0x82474498;
	sub_824743B0(ctx, base);
	// 82474498: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8247449C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824744A0: 419A0020  beq cr6, 0x824744c0
	if ctx.cr[6].eq {
	pc = 0x824744C0; continue 'dispatch;
	}
	// 824744A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824744A8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824744AC: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 824744B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824744B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824744B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824744BC: 4BFEFBFD  bl 0x824640b8
	ctx.lr = 0x824744C0;
	sub_824640B8(ctx, base);
	// 824744C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824744C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824744C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824744CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824744D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824744D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824744D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824744E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824744E0 size=132
    let mut pc: u32 = 0x824744E0;
    'dispatch: loop {
        match pc {
            0x824744E0 => {
    //   block [0x824744E0..0x82474564)
	// 824744E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824744E4: 480C0BD1  bl 0x825350b4
	ctx.lr = 0x824744E8;
	sub_82535080(ctx, base);
	// 824744E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824744EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824744F0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824744F4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824744F8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 824744FC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474500: 3BEB0028  addi r31, r11, 0x28
	ctx.r[31].s64 = ctx.r[11].s64 + 40;
	// 82474504: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82474508: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247450C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82474510: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82474514: 409A0010  bne cr6, 0x82474524
	if !ctx.cr[6].eq {
	pc = 0x82474524; continue 'dispatch;
	}
	// 82474518: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8247451C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474520: 4BFF9E31  bl 0x8246e350
	ctx.lr = 0x82474524;
	sub_8246E350(ctx, base);
	// 82474524: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474528: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247452C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82474530: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82474534: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82474538: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8247453C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82474540: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82474544: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82474548: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8247454C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82474550: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82474554: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82474558: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8247455C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82474560: 480C0BA4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474568 size=116
    let mut pc: u32 = 0x82474568;
    'dispatch: loop {
        match pc {
            0x82474568 => {
    //   block [0x82474568..0x824745DC)
	// 82474568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247456C: 480C0B4D  bl 0x825350b8
	ctx.lr = 0x82474570;
	sub_82535080(ctx, base);
	// 82474570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474574: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82474578: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8247457C: 3B9D0020  addi r28, r29, 0x20
	ctx.r[28].s64 = ctx.r[29].s64 + 32;
	// 82474580: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82474584: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82474588: 4BFF8FA9  bl 0x8246d530
	ctx.lr = 0x8247458C;
	sub_8246D530(ctx, base);
	// 8247458C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474590: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82474594: 419A0014  beq cr6, 0x824745a8
	if ctx.cr[6].eq {
	pc = 0x824745A8; continue 'dispatch;
	}
	// 82474598: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8247459C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824745A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824745A4: 4BFF8E5D  bl 0x8246d400
	ctx.lr = 0x824745A8;
	sub_8246D400(ctx, base);
	// 824745A8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 824745AC: 419A0028  beq cr6, 0x824745d4
	if ctx.cr[6].eq {
	pc = 0x824745D4; continue 'dispatch;
	}
	// 824745B0: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 824745B4: 57EA1838  slwi r10, r31, 3
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824745B8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824745BC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824745C0: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 824745C4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824745C8: 83EA0004  lwz r31, 4(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824745CC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 824745D0: 409AFFE4  bne cr6, 0x824745b4
	if !ctx.cr[6].eq {
	pc = 0x824745B4; continue 'dispatch;
	}
	// 824745D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824745D8: 480C0B30  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824745E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824745E0 size=220
    let mut pc: u32 = 0x824745E0;
    'dispatch: loop {
        match pc {
            0x824745E0 => {
    //   block [0x824745E0..0x824746BC)
	// 824745E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824745E4: 480C0AD1  bl 0x825350b4
	ctx.lr = 0x824745E8;
	sub_82535080(ctx, base);
	// 824745E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824745EC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824745F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824745F4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824745F8: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 824745FC: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474600: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82474604: 419A005C  beq cr6, 0x82474660
	if ctx.cr[6].eq {
	pc = 0x82474660; continue 'dispatch;
	}
	// 82474608: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8247460C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82474610: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82474614: 4BFF8F1D  bl 0x8246d530
	ctx.lr = 0x82474618;
	sub_8246D530(ctx, base);
	// 82474618: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8247461C: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82474620: 419A0040  beq cr6, 0x82474660
	if ctx.cr[6].eq {
	pc = 0x82474660; continue 'dispatch;
	}
	// 82474624: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82474628: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8247462C: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82474630: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82474634: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474638: 7F09E040  cmplw cr6, r9, r28
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8247463C: 419A0014  beq cr6, 0x82474650
	if ctx.cr[6].eq {
	pc = 0x82474650; continue 'dispatch;
	}
	// 82474640: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474644: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82474648: 409AFFE4  bne cr6, 0x8247462c
	if !ctx.cr[6].eq {
	pc = 0x8247462C; continue 'dispatch;
	}
	// 8247464C: 48000014  b 0x82474660
	pc = 0x82474660; continue 'dispatch;
	// 82474650: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82474654: 419A0060  beq cr6, 0x824746b4
	if ctx.cr[6].eq {
	pc = 0x824746B4; continue 'dispatch;
	}
	// 82474658: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8247465C: 4800B1AD  bl 0x8247f808
	ctx.lr = 0x82474660;
	sub_8247F808(ctx, base);
	// 82474660: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82474664: 419A004C  beq cr6, 0x824746b0
	if ctx.cr[6].eq {
	pc = 0x824746B0; continue 'dispatch;
	}
	// 82474668: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 8247466C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82474670: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82474674: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82474678: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8247467C: 4BFF8EB5  bl 0x8246d530
	ctx.lr = 0x82474680;
	sub_8246D530(ctx, base);
	// 82474680: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82474684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474688: 4BFFF9C9  bl 0x82474050
	ctx.lr = 0x8247468C;
	sub_82474050(ctx, base);
	// 8247468C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82474690: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474694: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82474698: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8247469C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824746A0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824746A4: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824746A8: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824746AC: 4BFF8D55  bl 0x8246d400
	ctx.lr = 0x824746B0;
	sub_8246D400(ctx, base);
	// 824746B0: 937C0000  stw r27, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 824746B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824746B8: 480C0A4C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824746C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824746C0 size=100
    let mut pc: u32 = 0x824746C0;
    'dispatch: loop {
        match pc {
            0x824746C0 => {
    //   block [0x824746C0..0x82474724)
	// 824746C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824746C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824746C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824746CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824746D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824746D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824746D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824746DC: 4BFFF865  bl 0x82473f40
	ctx.lr = 0x824746E0;
	sub_82473F40(ctx, base);
	// 824746E0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824746E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824746E8: 419A0020  beq cr6, 0x82474708
	if ctx.cr[6].eq {
	pc = 0x82474708; continue 'dispatch;
	}
	// 824746EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824746F0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824746F4: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 824746F8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824746FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82474700: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82474704: 4BFEF9B5  bl 0x824640b8
	ctx.lr = 0x82474708;
	sub_824640B8(ctx, base);
	// 82474708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247470C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474718: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8247471C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474728 size=100
    let mut pc: u32 = 0x82474728;
    'dispatch: loop {
        match pc {
            0x82474728 => {
    //   block [0x82474728..0x8247478C)
	// 82474728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247472C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82474734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247473C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474740: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82474744: 4BFFDA95  bl 0x824721d8
	ctx.lr = 0x82474748;
	sub_824721D8(ctx, base);
	// 82474748: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8247474C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82474750: 419A0020  beq cr6, 0x82474770
	if ctx.cr[6].eq {
	pc = 0x82474770; continue 'dispatch;
	}
	// 82474754: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474758: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8247475C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82474760: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474764: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82474768: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8247476C: 4BFEF94D  bl 0x824640b8
	ctx.lr = 0x82474770;
	sub_824640B8(ctx, base);
	// 82474770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8247477C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474780: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82474784: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474790 size=12
    let mut pc: u32 = 0x82474790;
    'dispatch: loop {
        match pc {
            0x82474790 => {
    //   block [0x82474790..0x8247479C)
	// 82474790: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82474794: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82474798: 48131A98  b 0x825a6230
	sub_825A6230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824747A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824747A0 size=8
    let mut pc: u32 = 0x824747A0;
    'dispatch: loop {
        match pc {
            0x824747A0 => {
    //   block [0x824747A0..0x824747A8)
	// 824747A0: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824747A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824747A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824747A8 size=64
    let mut pc: u32 = 0x824747A8;
    'dispatch: loop {
        match pc {
            0x824747A8 => {
    //   block [0x824747A8..0x824747E8)
	// 824747A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824747AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824747B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824747B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824747B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824747BC: 4800078D  bl 0x82474f48
	ctx.lr = 0x824747C0;
	sub_82474F48(ctx, base);
	// 824747C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824747C4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824747C8: 4BFF5809  bl 0x82469fd0
	ctx.lr = 0x824747CC;
	sub_82469FD0(ctx, base);
	// 824747CC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824747D0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824747D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824747D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824747DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824747E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824747E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824747E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824747E8 size=80
    let mut pc: u32 = 0x824747E8;
    'dispatch: loop {
        match pc {
            0x824747E8 => {
    //   block [0x824747E8..0x82474838)
	// 824747E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824747EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824747F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824747F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824747F8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824747FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474800: 396B8B70  addi r11, r11, -0x7490
	ctx.r[11].s64 = ctx.r[11].s64 + -29840;
	// 82474804: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82474808: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8247480C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474810: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82474814: 4BFF8B2D  bl 0x8246d340
	ctx.lr = 0x82474818;
	sub_8246D340(ctx, base);
	// 82474818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247481C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474820: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82474824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82474828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8247482C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474838 size=92
    let mut pc: u32 = 0x82474838;
    'dispatch: loop {
        match pc {
            0x82474838 => {
    //   block [0x82474838..0x82474894)
	// 82474838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247483C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474848: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8247484C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474850: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474854: 396B8B70  addi r11, r11, -0x7490
	ctx.r[11].s64 = ctx.r[11].s64 + -29840;
	// 82474858: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8247485C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82474860: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474864: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82474868: 4BFEFA59  bl 0x824642c0
	ctx.lr = 0x8247486C;
	sub_824642C0(ctx, base);
	// 8247486C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82474870: 4BFF8B59  bl 0x8246d3c8
	ctx.lr = 0x82474874;
	sub_8246D3C8(ctx, base);
	// 82474874: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82474878: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8247487C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82474884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8247488C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474898 size=80
    let mut pc: u32 = 0x82474898;
    'dispatch: loop {
        match pc {
            0x82474898 => {
    //   block [0x82474898..0x824748E8)
	// 82474898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247489C: 480C0821  bl 0x825350bc
	ctx.lr = 0x824748A0;
	sub_82535080(ctx, base);
	// 824748A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824748A4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824748A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824748AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824748B0: 806B92D4  lwz r3, -0x6d2c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) } as u64;
	// 824748B4: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824748B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824748BC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824748C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824748C4: 4E800421  bctrl
	ctx.lr = 0x824748C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824748C8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824748CC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824748D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824748D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824748D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824748DC: 4E800421  bctrl
	ctx.lr = 0x824748E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824748E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824748E4: 480C0828  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824748E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824748E8 size=440
    let mut pc: u32 = 0x824748E8;
    'dispatch: loop {
        match pc {
            0x824748E8 => {
    //   block [0x824748E8..0x82474AA0)
	// 824748E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824748EC: 480C07C9  bl 0x825350b4
	ctx.lr = 0x824748F0;
	sub_82535080(ctx, base);
	// 824748F0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824748F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824748F8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824748FC: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82474900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474904: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474908: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8247490C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474910: 4E800421  bctrl
	ctx.lr = 0x82474914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474914: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474918: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8247491C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82474920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474924: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82474928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247492C: 4E800421  bctrl
	ctx.lr = 0x82474930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474930: 2B030040  cmplwi cr6, r3, 0x40
	ctx.cr[6].compare_u32(ctx.r[3].u32, 64 as u32, &mut ctx.xer);
	// 82474934: 419A0018  beq cr6, 0x8247494c
	if ctx.cr[6].eq {
	pc = 0x8247494C; continue 'dispatch;
	}
	// 82474938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8247493C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82474940: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474944: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82474948: 480C07BC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8247494C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474954: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82474958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8247495C: 4E800421  bctrl
	ctx.lr = 0x82474960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474960: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82474964: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82474968: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8247496C: 4BFF59CD  bl 0x8246a338
	ctx.lr = 0x82474970;
	sub_8246A338(ctx, base);
	// 82474970: 3D6057E0  lis r11, 0x57e0
	ctx.r[11].s64 = 1474297856;
	// 82474974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82474978: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8247497C: 616BE057  ori r11, r11, 0xe057
	ctx.r[11].u64 = ctx.r[11].u64 | 57431;
	// 82474980: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82474984: 990100C8  stb r8, 0xc8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[8].u8 ) };
	// 82474988: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8247498C: 916100A0  stw r11, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82474990: 3D6010C0  lis r11, 0x10c0
	ctx.r[11].s64 = 281018368;
	// 82474994: 616BC010  ori r11, r11, 0xc010
	ctx.r[11].u64 = ctx.r[11].u64 | 49168;
	// 82474998: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 8247499C: 409A0024  bne cr6, 0x824749c0
	if !ctx.cr[6].eq {
	pc = 0x824749C0; continue 'dispatch;
	}
	// 824749A0: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824749A4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824749A8: 409A0018  bne cr6, 0x824749c0
	if !ctx.cr[6].eq {
	pc = 0x824749C0; continue 'dispatch;
	}
	// 824749AC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 824749B0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824749B4: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824749B8: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 824749BC: 480C0748  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824749C0: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 824749C4: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824749C8: 2B0B003C  cmplwi cr6, r11, 0x3c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 60 as u32, &mut ctx.xer);
	// 824749CC: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 824749D0: 419A0024  beq cr6, 0x824749f4
	if ctx.cr[6].eq {
	pc = 0x824749F4; continue 'dispatch;
	}
	// 824749D4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824749D8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824749DC: 409AFFE8  bne cr6, 0x824749c4
	if !ctx.cr[6].eq {
	pc = 0x824749C4; continue 'dispatch;
	}
	// 824749E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824749E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824749E8: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824749EC: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 824749F0: 480C0714  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824749F4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824749F8: 419AFFE8  beq cr6, 0x824749e0
	if ctx.cr[6].eq {
	pc = 0x824749E0; continue 'dispatch;
	}
	// 824749FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82474A00: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82474A04: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82474A08: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82474A0C: 396B8BB8  addi r11, r11, -0x7448
	ctx.r[11].s64 = ctx.r[11].s64 + -29768;
	// 82474A10: 394A8BAC  addi r10, r10, -0x7454
	ctx.r[10].s64 = ctx.r[10].s64 + -29780;
	// 82474A14: 39298BA0  addi r9, r9, -0x7460
	ctx.r[9].s64 = ctx.r[9].s64 + -29792;
	// 82474A18: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82474A1C: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82474A20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82474A24: 7FBE3850  subf r29, r30, r7
	ctx.r[29].s64 = ctx.r[7].s64 - ctx.r[30].s64;
	// 82474A28: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82474A2C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82474A30: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82474A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474A38: 4BFF57E9  bl 0x8246a220
	ctx.lr = 0x82474A3C;
	sub_8246A220(ctx, base);
	// 82474A3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82474A40: 7F05E800  cmpw cr6, r5, r29
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82474A44: 41980008  blt cr6, 0x82474a4c
	if ctx.cr[6].lt {
	pc = 0x82474A4C; continue 'dispatch;
	}
	// 82474A48: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82474A4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82474A50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82474A54: 4BFF55AD  bl 0x8246a000
	ctx.lr = 0x82474A58;
	sub_8246A000(ctx, base);
	// 82474A58: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82474A5C: 419A0030  beq cr6, 0x82474a8c
	if ctx.cr[6].eq {
	pc = 0x82474A8C; continue 'dispatch;
	}
	// 82474A60: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82474A64: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82474A68: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82474A6C: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82474A70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82474A74: 409AFFC0  bne cr6, 0x82474a34
	if !ctx.cr[6].eq {
	pc = 0x82474A34; continue 'dispatch;
	}
	// 82474A78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82474A7C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82474A80: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474A84: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82474A88: 480C067C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82474A8C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82474A90: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82474A94: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474A98: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82474A9C: 480C0668  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474AA0 size=84
    let mut pc: u32 = 0x82474AA0;
    'dispatch: loop {
        match pc {
            0x82474AA0 => {
    //   block [0x82474AA0..0x82474AF4)
	// 82474AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474AA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82474AAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474AB4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474AB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474ABC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82474AC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82474AC4: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82474AC8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82474ACC: 4BFEF7F5  bl 0x824642c0
	ctx.lr = 0x82474AD0;
	sub_824642C0(ctx, base);
	// 82474AD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82474AD4: 4BFF58AD  bl 0x8246a380
	ctx.lr = 0x82474AD8;
	sub_8246A380(ctx, base);
	// 82474AD8: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82474ADC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474AE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82474AEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474AF8 size=200
    let mut pc: u32 = 0x82474AF8;
    'dispatch: loop {
        match pc {
            0x82474AF8 => {
    //   block [0x82474AF8..0x82474BC0)
	// 82474AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474AFC: 480C05BD  bl 0x825350b8
	ctx.lr = 0x82474B00;
	sub_82535080(ctx, base);
	// 82474B00: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474B04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82474B08: 48000441  bl 0x82474f48
	ctx.lr = 0x82474B0C;
	sub_82474F48(ctx, base);
	// 82474B0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82474B10: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82474B14: 4BFF54BD  bl 0x82469fd0
	ctx.lr = 0x82474B18;
	sub_82469FD0(ctx, base);
	// 82474B18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82474B1C: 419A009C  beq cr6, 0x82474bb8
	if ctx.cr[6].eq {
	pc = 0x82474BB8; continue 'dispatch;
	}
	// 82474B20: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82474B24: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82474B28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82474B2C: 4BFF42CD  bl 0x82468df8
	ctx.lr = 0x82474B30;
	sub_82468DF8(ctx, base);
	// 82474B30: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82474B34: 3BEB8C20  addi r31, r11, -0x73e0
	ctx.r[31].s64 = ctx.r[11].s64 + -29664;
	// 82474B38: 48000411  bl 0x82474f48
	ctx.lr = 0x82474B3C;
	sub_82474F48(ctx, base);
	// 82474B3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82474B40: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82474B44: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82474B48: 388B8C00  addi r4, r11, -0x7400
	ctx.r[4].s64 = ctx.r[11].s64 + -29696;
	// 82474B4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82474B50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82474B54: 3BCB8BE0  addi r30, r11, -0x7420
	ctx.r[30].s64 = ctx.r[11].s64 + -29728;
	// 82474B58: 4BFF3749  bl 0x824682a0
	ctx.lr = 0x82474B5C;
	sub_824682A0(ctx, base);
	// 82474B5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82474B60: 4BFF3741  bl 0x824682a0
	ctx.lr = 0x82474B64;
	sub_824682A0(ctx, base);
	// 82474B64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82474B68: 4BFF3739  bl 0x824682a0
	ctx.lr = 0x82474B6C;
	sub_824682A0(ctx, base);
	// 82474B6C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82474B70: 4BFF3731  bl 0x824682a0
	ctx.lr = 0x82474B74;
	sub_824682A0(ctx, base);
	// 82474B74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82474B78: 4BFF3729  bl 0x824682a0
	ctx.lr = 0x82474B7C;
	sub_824682A0(ctx, base);
	// 82474B7C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82474B80: 3CA07AEF  lis r5, 0x7aef
	ctx.r[5].s64 = 2062483456;
	// 82474B84: 39000076  li r8, 0x76
	ctx.r[8].s64 = 118;
	// 82474B88: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82474B8C: 60A56C06  ori r5, r5, 0x6c06
	ctx.r[5].u64 = ctx.r[5].u64 | 27654;
	// 82474B90: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82474B94: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82474B98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82474B9C: 38EB8BC0  addi r7, r11, -0x7440
	ctx.r[7].s64 = ctx.r[11].s64 + -29760;
	// 82474BA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474BA4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82474BA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474BAC: 4E800421  bctrl
	ctx.lr = 0x82474BB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474BB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82474BB4: 4BFF3D2D  bl 0x824688e0
	ctx.lr = 0x82474BB8;
	sub_824688E0(ctx, base);
	// 82474BB8: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82474BBC: 480C054C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474BC0 size=116
    let mut pc: u32 = 0x82474BC0;
    'dispatch: loop {
        match pc {
            0x82474BC0 => {
    //   block [0x82474BC0..0x82474C34)
	// 82474BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474BC4: 480C04F5  bl 0x825350b8
	ctx.lr = 0x82474BC8;
	sub_82535080(ctx, base);
	// 82474BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474BCC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82474BD0: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474BD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474BD8: 396B8B70  addi r11, r11, -0x7490
	ctx.r[11].s64 = ctx.r[11].s64 + -29840;
	// 82474BDC: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82474BE0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82474BE4: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82474BE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474BEC: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82474BF0: 4BFEF6D1  bl 0x824642c0
	ctx.lr = 0x82474BF4;
	sub_824642C0(ctx, base);
	// 82474BF4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82474BF8: 4BFF87D1  bl 0x8246d3c8
	ctx.lr = 0x82474BFC;
	sub_8246D3C8(ctx, base);
	// 82474BFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82474C00: 578A07FE  clrlwi r10, r28, 0x1f
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	// 82474C04: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82474C08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82474C0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474C10: 419A0018  beq cr6, 0x82474c28
	if ctx.cr[6].eq {
	pc = 0x82474C28; continue 'dispatch;
	}
	// 82474C14: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82474C18: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474C1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82474C20: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82474C24: 4BFEF495  bl 0x824640b8
	ctx.lr = 0x82474C28;
	sub_824640B8(ctx, base);
	// 82474C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82474C30: 480C04D8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474C38 size=4
    let mut pc: u32 = 0x82474C38;
    'dispatch: loop {
        match pc {
            0x82474C38 => {
    //   block [0x82474C38..0x82474C3C)
	// 82474C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474C40 size=4
    let mut pc: u32 = 0x82474C40;
    'dispatch: loop {
        match pc {
            0x82474C40 => {
    //   block [0x82474C40..0x82474C44)
	// 82474C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474C48 size=4
    let mut pc: u32 = 0x82474C48;
    'dispatch: loop {
        match pc {
            0x82474C48 => {
    //   block [0x82474C48..0x82474C4C)
	// 82474C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474C50 size=4
    let mut pc: u32 = 0x82474C50;
    'dispatch: loop {
        match pc {
            0x82474C50 => {
    //   block [0x82474C50..0x82474C54)
	// 82474C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474C58 size=100
    let mut pc: u32 = 0x82474C58;
    'dispatch: loop {
        match pc {
            0x82474C58 => {
    //   block [0x82474C58..0x82474CBC)
	// 82474C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474C60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82474C64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474C68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474C6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82474C70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82474C74: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474C78: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82474C7C: 419A0028  beq cr6, 0x82474ca4
	if ctx.cr[6].eq {
	pc = 0x82474CA4; continue 'dispatch;
	}
	// 82474C80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474C84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82474C88: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82474C8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474C90: 4E800421  bctrl
	ctx.lr = 0x82474C94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474C94: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82474C98: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474C9C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82474CA0: 409AFFE0  bne cr6, 0x82474c80
	if !ctx.cr[6].eq {
	pc = 0x82474C80; continue 'dispatch;
	}
	// 82474CA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474CB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82474CB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474CC0 size=132
    let mut pc: u32 = 0x82474CC0;
    'dispatch: loop {
        match pc {
            0x82474CC0 => {
    //   block [0x82474CC0..0x82474D44)
	// 82474CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474CC4: 480C03F5  bl 0x825350b8
	ctx.lr = 0x82474CC8;
	sub_82535080(ctx, base);
	// 82474CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474CCC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82474CD0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82474CD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82474CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474CDC: 83AB3C58  lwz r29, 0x3c58(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15448 as u32) ) } as u64;
	// 82474CE0: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474CE4: 4BFF1B75  bl 0x82466858
	ctx.lr = 0x82474CE8;
	sub_82466858(ctx, base);
	// 82474CE8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82474CEC: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82474CF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82474CF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82474CF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474CFC: 4E800421  bctrl
	ctx.lr = 0x82474D00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474D00: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82474D04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82474D08: 806B3C5C  lwz r3, 0x3c5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15452 as u32) ) } as u64;
	// 82474D0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474D10: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82474D14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474D18: 4E800421  bctrl
	ctx.lr = 0x82474D1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474D1C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82474D20: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82474D24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82474D28: 806B91DC  lwz r3, -0x6e24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 82474D2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474D30: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82474D34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474D38: 4E800421  bctrl
	ctx.lr = 0x82474D3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82474D40: 480C03C8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474D48 size=132
    let mut pc: u32 = 0x82474D48;
    'dispatch: loop {
        match pc {
            0x82474D48 => {
    //   block [0x82474D48..0x82474DCC)
	// 82474D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474D50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474D54: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82474D58: 816B91DC  lwz r11, -0x6e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 82474D5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82474D60: 419A0058  beq cr6, 0x82474db8
	if ctx.cr[6].eq {
	pc = 0x82474DB8; continue 'dispatch;
	}
	// 82474D64: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82474D68: 816B3C58  lwz r11, 0x3c58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15448 as u32) ) } as u64;
	// 82474D6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82474D70: 419A0048  beq cr6, 0x82474db8
	if ctx.cr[6].eq {
	pc = 0x82474DB8; continue 'dispatch;
	}
	// 82474D74: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82474D78: 816B3C5C  lwz r11, 0x3c5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15452 as u32) ) } as u64;
	// 82474D7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82474D80: 419A0038  beq cr6, 0x82474db8
	if ctx.cr[6].eq {
	pc = 0x82474DB8; continue 'dispatch;
	}
	// 82474D84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474D88: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82474D8C: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82474D90: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82474D94: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82474D98: 4BFEF2A1  bl 0x82464038
	ctx.lr = 0x82474D9C;
	sub_82464038(ctx, base);
	// 82474D9C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82474DA0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82474DA4: 4800002D  bl 0x82474dd0
	ctx.lr = 0x82474DA8;
	sub_82474DD0(ctx, base);
	// 82474DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82474DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474DB4: 4E800020  blr
	return;
	// 82474DB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82474DBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82474DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474DD0 size=152
    let mut pc: u32 = 0x82474DD0;
    'dispatch: loop {
        match pc {
            0x82474DD0 => {
    //   block [0x82474DD0..0x82474E68)
	// 82474DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474DD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82474DDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474DE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474DE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82474DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474DEC: 396B9098  addi r11, r11, -0x6f68
	ctx.r[11].s64 = ctx.r[11].s64 + -28520;
	// 82474DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82474DF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82474DF8: 388A92D8  addi r4, r10, -0x6d28
	ctx.r[4].s64 = ctx.r[10].s64 + -27944;
	// 82474DFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82474E00: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82474E04: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82474E08: 806B3C58  lwz r3, 0x3c58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15448 as u32) ) } as u64;
	// 82474E0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474E10: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82474E14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474E18: 4E800421  bctrl
	ctx.lr = 0x82474E1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474E1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82474E20: 3BCBB0A8  addi r30, r11, -0x4f58
	ctx.r[30].s64 = ctx.r[11].s64 + -20312;
	// 82474E24: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82474E28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82474E2C: 806B3C5C  lwz r3, 0x3c5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15452 as u32) ) } as u64;
	// 82474E30: 4BFFFE29  bl 0x82474c58
	ctx.lr = 0x82474E34;
	sub_82474C58(ctx, base);
	// 82474E34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82474E38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82474E3C: 38ABB518  addi r5, r11, -0x4ae8
	ctx.r[5].s64 = ctx.r[11].s64 + -19176;
	// 82474E40: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82474E44: 806B91DC  lwz r3, -0x6e24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 82474E48: 4BFFAC71  bl 0x8246fab8
	ctx.lr = 0x82474E4C;
	sub_8246FAB8(ctx, base);
	// 82474E4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82474E50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474E5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82474E60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474E68 size=12
    let mut pc: u32 = 0x82474E68;
    'dispatch: loop {
        match pc {
            0x82474E68 => {
    //   block [0x82474E68..0x82474E74)
	// 82474E68: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82474E6C: 806B3C5C  lwz r3, 0x3c5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15452 as u32) ) } as u64;
	// 82474E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474E78 size=12
    let mut pc: u32 = 0x82474E78;
    'dispatch: loop {
        match pc {
            0x82474E78 => {
    //   block [0x82474E78..0x82474E84)
	// 82474E78: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82474E7C: 806B3C58  lwz r3, 0x3c58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15448 as u32) ) } as u64;
	// 82474E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474E88 size=12
    let mut pc: u32 = 0x82474E88;
    'dispatch: loop {
        match pc {
            0x82474E88 => {
    //   block [0x82474E88..0x82474E94)
	// 82474E88: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82474E8C: 806B91DC  lwz r3, -0x6e24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 82474E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474E98 size=156
    let mut pc: u32 = 0x82474E98;
    'dispatch: loop {
        match pc {
            0x82474E98 => {
    //   block [0x82474E98..0x82474F34)
	// 82474E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474EA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82474EA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474EA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474EAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82474EB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82474EB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82474EB8: 419A001C  beq cr6, 0x82474ed4
	if ctx.cr[6].eq {
	pc = 0x82474ED4; continue 'dispatch;
	}
	// 82474EBC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474EC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82474EC4: 419A0010  beq cr6, 0x82474ed4
	if ctx.cr[6].eq {
	pc = 0x82474ED4; continue 'dispatch;
	}
	// 82474EC8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82474ECC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82474ED0: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82474ED4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82474ED8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82474EDC: 419A003C  beq cr6, 0x82474f18
	if ctx.cr[6].eq {
	pc = 0x82474F18; continue 'dispatch;
	}
	// 82474EE0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82474EE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82474EE8: 419A0030  beq cr6, 0x82474f18
	if ctx.cr[6].eq {
	pc = 0x82474F18; continue 'dispatch;
	}
	// 82474EEC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82474EF0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82474EF4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82474EF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82474EFC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82474F00: 409A0018  bne cr6, 0x82474f18
	if !ctx.cr[6].eq {
	pc = 0x82474F18; continue 'dispatch;
	}
	// 82474F04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474F08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82474F0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474F10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82474F14: 4E800421  bctrl
	ctx.lr = 0x82474F18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82474F18: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82474F1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82474F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82474F28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82474F2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82474F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474F38 size=8
    let mut pc: u32 = 0x82474F38;
    'dispatch: loop {
        match pc {
            0x82474F38 => {
    //   block [0x82474F38..0x82474F40)
	// 82474F38: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82474F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474F40 size=8
    let mut pc: u32 = 0x82474F40;
    'dispatch: loop {
        match pc {
            0x82474F40 => {
    //   block [0x82474F40..0x82474F48)
	// 82474F40: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82474F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82474F48 size=12
    let mut pc: u32 = 0x82474F48;
    'dispatch: loop {
        match pc {
            0x82474F48 => {
    //   block [0x82474F48..0x82474F54)
	// 82474F48: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82474F4C: 386B9724  addi r3, r11, -0x68dc
	ctx.r[3].s64 = ctx.r[11].s64 + -26844;
	// 82474F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82474F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82474F58 size=180
    let mut pc: u32 = 0x82474F58;
    'dispatch: loop {
        match pc {
            0x82474F58 => {
    //   block [0x82474F58..0x8247500C)
	// 82474F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82474F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82474F60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82474F64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82474F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82474F6C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82474F70: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82474F74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82474F78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82474F7C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82474F80: 4BFFB249  bl 0x824701c8
	ctx.lr = 0x82474F84;
	sub_824701C8(ctx, base);
	// 82474F84: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82474F88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82474F8C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82474F90: 4BFFB239  bl 0x824701c8
	ctx.lr = 0x82474F94;
	sub_824701C8(ctx, base);
	// 82474F94: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82474F98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82474F9C: 4BFFB2FD  bl 0x82470298
	ctx.lr = 0x82474FA0;
	sub_82470298(ctx, base);
	// 82474FA0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474FA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82474FA8: 419A004C  beq cr6, 0x82474ff4
	if ctx.cr[6].eq {
	pc = 0x82474FF4; continue 'dispatch;
	}
	// 82474FAC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82474FB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82474FB4: 4BFFB2E5  bl 0x82470298
	ctx.lr = 0x82474FB8;
	sub_82470298(ctx, base);
	// 82474FB8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82474FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82474FC0: 419A0034  beq cr6, 0x82474ff4
	if ctx.cr[6].eq {
	pc = 0x82474FF4; continue 'dispatch;
	}
	// 82474FC4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82474FC8: 4BFFB431  bl 0x824703f8
	ctx.lr = 0x82474FCC;
	sub_824703F8(ctx, base);
	// 82474FCC: 4BFF6255  bl 0x8246b220
	ctx.lr = 0x82474FD0;
	sub_8246B220(ctx, base);
	// 82474FD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82474FD4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82474FD8: 4BFFB2D9  bl 0x824702b0
	ctx.lr = 0x82474FDC;
	sub_824702B0(ctx, base);
	// 82474FDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82474FE0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82474FE4: 4BFFB2CD  bl 0x824702b0
	ctx.lr = 0x82474FE8;
	sub_824702B0(ctx, base);
	// 82474FE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82474FEC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82474FF0: 4BFF5339  bl 0x8246a328
	ctx.lr = 0x82474FF4;
	sub_8246A328(ctx, base);
	// 82474FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82474FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82474FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82475000: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82475004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82475008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82475010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82475010 size=100
    let mut pc: u32 = 0x82475010;
    'dispatch: loop {
        match pc {
            0x82475010 => {
    //   block [0x82475010..0x82475074)
	// 82475010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82475014: 480C00A5  bl 0x825350b8
	ctx.lr = 0x82475018;
	sub_82535080(ctx, base);
	// 82475018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247501C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82475020: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82475024: 40990048  ble cr6, 0x8247506c
	if !ctx.cr[6].gt {
	pc = 0x8247506C; continue 'dispatch;
	}
	// 82475028: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 8247502C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82475030: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475034: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82475038: 419A0024  beq cr6, 0x8247505c
	if ctx.cr[6].eq {
	pc = 0x8247505C; continue 'dispatch;
	}
	// 8247503C: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475040: 4BFF1819  bl 0x82466858
	ctx.lr = 0x82475044;
	sub_82466858(ctx, base);
	// 82475044: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82475048: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8247504C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82475050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82475054: 4E800421  bctrl
	ctx.lr = 0x82475058;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82475058: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8247505C: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82475060: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82475064: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82475068: 409AFFC8  bne cr6, 0x82475030
	if !ctx.cr[6].eq {
	pc = 0x82475030; continue 'dispatch;
	}
	// 8247506C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82475070: 480C0098  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82475078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82475078 size=20
    let mut pc: u32 = 0x82475078;
    'dispatch: loop {
        match pc {
            0x82475078 => {
    //   block [0x82475078..0x8247508C)
	// 82475078: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8247507C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82475080: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475084: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82475088: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8247508C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8247508C size=24
    let mut pc: u32 = 0x8247508C;
    'dispatch: loop {
        match pc {
            0x8247508C => {
    //   block [0x8247508C..0x824750A4)
	// 8247508C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82475090: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82475094: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475098: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8247509C: 409AFFF0  bne cr6, 0x8247508c
	if !ctx.cr[6].eq {
	pc = 0x8247508C; continue 'dispatch;
	}
	// 824750A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824750A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824750A8 size=188
    let mut pc: u32 = 0x824750A8;
    'dispatch: loop {
        match pc {
            0x824750A8 => {
    //   block [0x824750A8..0x82475164)
	// 824750A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824750AC: 480C000D  bl 0x825350b8
	ctx.lr = 0x824750B0;
	sub_82535080(ctx, base);
	// 824750B0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824750B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824750B8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824750BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824750C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824750C4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824750C8: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 824750CC: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 824750D0: 4BFF1BE9  bl 0x82466cb8
	ctx.lr = 0x824750D4;
	sub_82466CB8(ctx, base);
	// 824750D4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824750D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824750DC: 88DD0000  lbz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824750E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824750E4: 4BFF9F95  bl 0x8246f078
	ctx.lr = 0x824750E8;
	sub_8246F078(ctx, base);
	// 824750E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824750EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824750F0: 4BFF1981  bl 0x82466a70
	ctx.lr = 0x824750F4;
	sub_82466A70(ctx, base);
	// 824750F4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824750F8: 4099005C  ble cr6, 0x82475154
	if !ctx.cr[6].gt {
	pc = 0x82475154; continue 'dispatch;
	}
	// 824750FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82475100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475104: 4BFF1995  bl 0x82466a98
	ctx.lr = 0x82475108;
	sub_82466A98(ctx, base);
	// 82475108: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8247510C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82475110: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475114: 4BFF1A15  bl 0x82466b28
	ctx.lr = 0x82475118;
	sub_82466B28(ctx, base);
	// 82475118: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8247511C: 409A0024  bne cr6, 0x82475140
	if !ctx.cr[6].eq {
	pc = 0x82475140; continue 'dispatch;
	}
	// 82475120: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82475124: A09D0012  lhz r4, 0x12(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(18 as u32) ) } as u64;
	// 82475128: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8247512C: 4BFF9E15  bl 0x8246ef40
	ctx.lr = 0x82475130;
	sub_8246EF40(ctx, base);
	// 82475130: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82475134: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82475138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247513C: 4BFF1C65  bl 0x82466da0
	ctx.lr = 0x82475140;
	sub_82466DA0(ctx, base);
	// 82475140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475144: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82475148: 4BFF1929  bl 0x82466a70
	ctx.lr = 0x8247514C;
	sub_82466A70(ctx, base);
	// 8247514C: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82475150: 4198FFAC  blt cr6, 0x824750fc
	if ctx.cr[6].lt {
	pc = 0x824750FC; continue 'dispatch;
	}
	// 82475154: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82475158: 4BFFA139  bl 0x8246f290
	ctx.lr = 0x8247515C;
	sub_8246F290(ctx, base);
	// 8247515C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82475160: 480BFFA8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82475168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82475168 size=124
    let mut pc: u32 = 0x82475168;
    'dispatch: loop {
        match pc {
            0x82475168 => {
    //   block [0x82475168..0x824751E4)
	// 82475168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8247516C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82475170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82475174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82475178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8247517C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82475180: 48131131  bl 0x825a62b0
	ctx.lr = 0x82475184;
	sub_825A62B0(ctx, base);
	// 82475184: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82475188: 48006AB1  bl 0x8247bc38
	ctx.lr = 0x8247518C;
	sub_8247BC38(ctx, base);
	// 8247518C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82475190: 4BFF81B1  bl 0x8246d340
	ctx.lr = 0x82475194;
	sub_8246D340(ctx, base);
	// 82475194: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475198: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8247519C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824751A0: 419A0024  beq cr6, 0x824751c4
	if ctx.cr[6].eq {
	pc = 0x824751C4; continue 'dispatch;
	}
	// 824751A4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824751A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824751AC: 480070FD  bl 0x8247c2a8
	ctx.lr = 0x824751B0;
	sub_8247C2A8(ctx, base);
	// 824751B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824751B4: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824751B8: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824751BC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824751C0: 409AFFE4  bne cr6, 0x824751a4
	if !ctx.cr[6].eq {
	pc = 0x824751A4; continue 'dispatch;
	}
	// 824751C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824751C8: 4BFF8201  bl 0x8246d3c8
	ctx.lr = 0x824751CC;
	sub_8246D3C8(ctx, base);
	// 824751CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824751D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824751D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824751D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824751DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824751E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824751E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824751E8 size=1240
    let mut pc: u32 = 0x824751E8;
    'dispatch: loop {
        match pc {
            0x824751E8 => {
    //   block [0x824751E8..0x824752A4)
	// 824751E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824751EC: 480BFED1  bl 0x825350bc
	ctx.lr = 0x824751F0;
	sub_82535080(ctx, base);
	// 824751F0: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 824751F4: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 824751F8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824751FC: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82475200: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82475204: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82475208: 2B0A001E  cmplwi cr6, r10, 0x1e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 30 as u32, &mut ctx.xer);
	// 8247520C: 419904A4  bgt cr6, 0x824756b0
	if ctx.cr[6].gt {
	pc = 0x824756B0; continue 'dispatch;
	}
	// 82475210: 3D808247  lis r12, -0x7db9
	ctx.r[12].s64 = -2109276160;
	// 82475214: 398C5228  addi r12, r12, 0x5228
	ctx.r[12].s64 = ctx.r[12].s64 + 21032;
	// 82475218: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8247521C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82475220: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82475224: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x824752C8; continue 'dispatch;
		},
		1 => {
	pc = 0x824753A4; continue 'dispatch;
		},
		2 => {
	pc = 0x824753CC; continue 'dispatch;
		},
		3 => {
	pc = 0x824753F4; continue 'dispatch;
		},
		4 => {
	pc = 0x82475418; continue 'dispatch;
		},
		5 => {
	pc = 0x82475440; continue 'dispatch;
		},
		6 => {
	pc = 0x82475464; continue 'dispatch;
		},
		7 => {
	pc = 0x82475488; continue 'dispatch;
		},
		8 => {
	pc = 0x824754AC; continue 'dispatch;
		},
		9 => {
	pc = 0x824754D0; continue 'dispatch;
		},
		10 => {
	pc = 0x824754F4; continue 'dispatch;
		},
		11 => {
	pc = 0x82475520; continue 'dispatch;
		},
		12 => {
	pc = 0x82475520; continue 'dispatch;
		},
		13 => {
	pc = 0x82475570; continue 'dispatch;
		},
		14 => {
	pc = 0x82475570; continue 'dispatch;
		},
		15 => {
	pc = 0x82475570; continue 'dispatch;
		},
		16 => {
	pc = 0x82475608; continue 'dispatch;
		},
		17 => {
	pc = 0x82475608; continue 'dispatch;
		},
		18 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		19 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		20 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		21 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		22 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		23 => {
	pc = 0x824752A4; continue 'dispatch;
		},
		24 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		25 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		26 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		27 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		28 => {
	pc = 0x824756B0; continue 'dispatch;
		},
		29 => {
	pc = 0x82475488; continue 'dispatch;
		},
		30 => {
	pc = 0x824752A4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82475228: 824752C8  lwz r18, 0x52c8(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21192 as u32) ) } as u64;
	// 8247522C: 824753A4  lwz r18, 0x53a4(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21412 as u32) ) } as u64;
	// 82475230: 824753CC  lwz r18, 0x53cc(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21452 as u32) ) } as u64;
	// 82475234: 824753F4  lwz r18, 0x53f4(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21492 as u32) ) } as u64;
	// 82475238: 82475418  lwz r18, 0x5418(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21528 as u32) ) } as u64;
	// 8247523C: 82475440  lwz r18, 0x5440(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21568 as u32) ) } as u64;
	// 82475240: 82475464  lwz r18, 0x5464(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21604 as u32) ) } as u64;
	// 82475244: 82475488  lwz r18, 0x5488(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21640 as u32) ) } as u64;
	// 82475248: 824754AC  lwz r18, 0x54ac(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21676 as u32) ) } as u64;
	// 8247524C: 824754D0  lwz r18, 0x54d0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21712 as u32) ) } as u64;
	// 82475250: 824754F4  lwz r18, 0x54f4(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21748 as u32) ) } as u64;
	// 82475254: 82475520  lwz r18, 0x5520(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21792 as u32) ) } as u64;
	// 82475258: 82475520  lwz r18, 0x5520(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21792 as u32) ) } as u64;
	// 8247525C: 82475570  lwz r18, 0x5570(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21872 as u32) ) } as u64;
	// 82475260: 82475570  lwz r18, 0x5570(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21872 as u32) ) } as u64;
	// 82475264: 82475570  lwz r18, 0x5570(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21872 as u32) ) } as u64;
	// 82475268: 82475608  lwz r18, 0x5608(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22024 as u32) ) } as u64;
	// 8247526C: 82475608  lwz r18, 0x5608(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22024 as u32) ) } as u64;
	// 82475270: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 82475274: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 82475278: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 8247527C: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 82475280: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 82475284: 824752A4  lwz r18, 0x52a4(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21156 as u32) ) } as u64;
	// 82475288: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 8247528C: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 82475290: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 82475294: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 82475298: 824756B0  lwz r18, 0x56b0(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(22192 as u32) ) } as u64;
	// 8247529C: 82475488  lwz r18, 0x5488(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21640 as u32) ) } as u64;
	// 824752A0: 824752A4  lwz r18, 0x52a4(r7)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(21156 as u32) ) } as u64;
            }
            0x824752A4 => {
    //   block [0x824752A4..0x824752C8)
	// 824752A4: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 824752A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824752AC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824752B0: 2B0A001E  cmplwi cr6, r10, 0x1e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 30 as u32, &mut ctx.xer);
	// 824752B4: 4099FF5C  ble cr6, 0x82475210
	if !ctx.cr[6].gt {
	pc = 0x82475210; continue 'dispatch;
	}
	// 824752B8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824752BC: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824752C0: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824752C4: 480BFE48  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824752C8 => {
    //   block [0x824752C8..0x824753A4)
	// 824752C8: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824752CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824752D0: 419A0010  beq cr6, 0x824752e0
	if ctx.cr[6].eq {
	pc = 0x824752E0; continue 'dispatch;
	}
	// 824752D4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824752D8: 3BAB8314  addi r29, r11, -0x7cec
	ctx.r[29].s64 = ctx.r[11].s64 + -31980;
	// 824752DC: 4800000C  b 0x824752e8
	pc = 0x824752E8; continue 'dispatch;
	// 824752E0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824752E4: 3BAB830C  addi r29, r11, -0x7cf4
	ctx.r[29].s64 = ctx.r[11].s64 + -31988;
	// 824752E8: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824752EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824752F0: 419A0064  beq cr6, 0x82475354
	if ctx.cr[6].eq {
	pc = 0x82475354; continue 'dispatch;
	}
	// 824752F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824752F8: 4BFF4F29  bl 0x8246a220
	ctx.lr = 0x824752FC;
	sub_8246A220(ctx, base);
	// 824752FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82475300: 3BC30001  addi r30, r3, 1
	ctx.r[30].s64 = ctx.r[3].s64 + 1;
	// 82475304: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82475308: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8247530C: 40980024  bge cr6, 0x82475330
	if !ctx.cr[6].lt {
	pc = 0x82475330; continue 'dispatch;
	}
	// 82475310: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82475314: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82475318: 41980008  blt cr6, 0x82475320
	if ctx.cr[6].lt {
	pc = 0x82475320; continue 'dispatch;
	}
	// 8247531C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82475320: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82475324: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82475328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247532C: 4BFF8F9D  bl 0x8246e2c8
	ctx.lr = 0x82475330;
	sub_8246E2C8(ctx, base);
	// 82475330: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82475334: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475338: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8247533C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82475340: 4BFF4FE9  bl 0x8246a328
	ctx.lr = 0x82475344;
	sub_8246A328(ctx, base);
	// 82475344: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82475348: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8247534C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82475350: 480BFDBC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82475354: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82475358: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8247535C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82475360: 40980020  bge cr6, 0x82475380
	if !ctx.cr[6].lt {
	pc = 0x82475380; continue 'dispatch;
	}
	// 82475364: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82475368: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8247536C: 41990008  bgt cr6, 0x82475374
	if ctx.cr[6].gt {
	pc = 0x82475374; continue 'dispatch;
	}
	// 82475370: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82475374: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82475378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247537C: 4BFF8F4D  bl 0x8246e2c8
	ctx.lr = 0x82475380;
	sub_8246E2C8(ctx, base);
	// 82475380: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82475384: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475388: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8247538C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82475390: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82475394: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82475398: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8247539C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824753A0: 480BFD6C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824753A4 => {
    //   block [0x824753A4..0x824753CC)
	// 824753A4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824753A8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824753AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824753B0: 388B9800  addi r4, r11, -0x6800
	ctx.r[4].s64 = ctx.r[11].s64 + -26624;
	// 824753B4: 7D450774  extsb r5, r10
	ctx.r[5].s64 = ctx.r[10].s8 as i64;
	// 824753B8: 4BFF51A9  bl 0x8246a560
	ctx.lr = 0x824753BC;
	sub_8246A560(ctx, base);
	// 824753BC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824753C0: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824753C4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824753C8: 480BFD44  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824753CC => {
    //   block [0x824753CC..0x824753F4)
	// 824753CC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824753D0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 824753D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824753D8: 388B0034  addi r4, r11, 0x34
	ctx.r[4].s64 = ctx.r[11].s64 + 52;
	// 824753DC: 7D450774  extsb r5, r10
	ctx.r[5].s64 = ctx.r[10].s8 as i64;
	// 824753E0: 4BFF5181  bl 0x8246a560
	ctx.lr = 0x824753E4;
	sub_8246A560(ctx, base);
	// 824753E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824753E8: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824753EC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824753F0: 480BFD1C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824753F4 => {
    //   block [0x824753F4..0x82475418)
	// 824753F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 824753F8: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824753FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475400: 388A831C  addi r4, r10, -0x7ce4
	ctx.r[4].s64 = ctx.r[10].s64 + -31972;
	// 82475404: 4BFF515D  bl 0x8246a560
	ctx.lr = 0x82475408;
	sub_8246A560(ctx, base);
	// 82475408: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8247540C: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82475410: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82475414: 480BFCF8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82475418 => {
    //   block [0x82475418..0x82475440)
	// 82475418: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247541C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82475420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475424: 388B0034  addi r4, r11, 0x34
	ctx.r[4].s64 = ctx.r[11].s64 + 52;
	// 82475428: 7D450734  extsh r5, r10
	ctx.r[5].s64 = ctx.r[10].s16 as i64;
	// 8247542C: 4BFF5135  bl 0x8246a560
	ctx.lr = 0x82475430;
	sub_8246A560(ctx, base);
	// 82475430: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82475434: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82475438: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8247543C: 480BFCD0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82475440 => {
    //   block [0x82475440..0x82475464)
	// 82475440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82475444: A0AB0000  lhz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8247544C: 388A831C  addi r4, r10, -0x7ce4
	ctx.r[4].s64 = ctx.r[10].s64 + -31972;
	// 82475450: 4BFF5111  bl 0x8246a560
	ctx.lr = 0x82475454;
	sub_8246A560(ctx, base);
	// 82475454: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82475458: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8247545C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82475460: 480BFCAC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82475464 => {
    //   block [0x82475464..0x82475488)
	// 82475464: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82475468: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8247546C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475470: 388A0034  addi r4, r10, 0x34
	ctx.r[4].s64 = ctx.r[10].s64 + 52;
	// 82475474: 4BFF50ED  bl 0x8246a560
	ctx.lr = 0x82475478;
	sub_8246A560(ctx, base);
	// 82475478: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8247547C: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82475480: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82475484: 480BFC88  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82475488 => {
    //   block [0x82475488..0x824754AC)
	// 82475488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8247548C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82475490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475494: 388A831C  addi r4, r10, -0x7ce4
	ctx.r[4].s64 = ctx.r[10].s64 + -31972;
	// 82475498: 4BFF50C9  bl 0x8246a560
	ctx.lr = 0x8247549C;
	sub_8246A560(ctx, base);
	// 8247549C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824754A0: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824754A4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824754A8: 480BFC64  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824754AC => {
    //   block [0x824754AC..0x824754D0)
	// 824754AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 824754B0: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824754B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824754B8: 388A8324  addi r4, r10, -0x7cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -31964;
	// 824754BC: 4BFF50A5  bl 0x8246a560
	ctx.lr = 0x824754C0;
	sub_8246A560(ctx, base);
	// 824754C0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824754C4: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824754C8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824754CC: 480BFC40  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824754D0 => {
    //   block [0x824754D0..0x824754F4)
	// 824754D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 824754D4: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 824754D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824754DC: 388A832C  addi r4, r10, -0x7cd4
	ctx.r[4].s64 = ctx.r[10].s64 + -31956;
	// 824754E0: 4BFF5081  bl 0x8246a560
	ctx.lr = 0x824754E4;
	sub_8246A560(ctx, base);
	// 824754E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824754E8: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824754EC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824754F0: 480BFC1C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824754F4 => {
    //   block [0x824754F4..0x82475520)
	// 824754F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 824754F8: C02B0000  lfs f1, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824754FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475500: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 82475504: 388A97FC  addi r4, r10, -0x6804
	ctx.r[4].s64 = ctx.r[10].s64 + -26628;
	// 82475508: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 8247550C: 4BFF5055  bl 0x8246a560
	ctx.lr = 0x82475510;
	sub_8246A560(ctx, base);
	// 82475510: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82475514: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82475518: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8247551C: 480BFBF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82475520 => {
    //   block [0x82475520..0x82475570)
	// 82475520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82475524: C08B000C  lfs f4, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82475528: C06B0008  lfs f3, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8247552C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475530: C04B0004  lfs f2, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82475534: 388A97E4  addi r4, r10, -0x681c
	ctx.r[4].s64 = ctx.r[10].s64 + -26652;
	// 82475538: C02B0000  lfs f1, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8247553C: D8810038  stfd f4, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.f[4].u64 ) };
	// 82475540: E9010038  ld r8, 0x38(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(56 as u32) ) };
	// 82475544: D8610030  stfd f3, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.f[3].u64 ) };
	// 82475548: E8E10030  ld r7, 0x30(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(48 as u32) ) };
	// 8247554C: D8410028  stfd f2, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.f[2].u64 ) };
	// 82475550: E8C10028  ld r6, 0x28(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(40 as u32) ) };
	// 82475554: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 82475558: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 8247555C: 4BFF5005  bl 0x8246a560
	ctx.lr = 0x82475560;
	sub_8246A560(ctx, base);
	// 82475560: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82475564: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82475568: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8247556C: 480BFBA0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82475570 => {
    //   block [0x82475570..0x82475608)
	// 82475570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82475574: C0CB0014  lfs f6, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82475578: C0AB0010  lfs f5, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8247557C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475580: C08B000C  lfs f4, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82475584: 388A97A4  addi r4, r10, -0x685c
	ctx.r[4].s64 = ctx.r[10].s64 + -26716;
	// 82475588: C06B0008  lfs f3, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8247558C: C04B0004  lfs f2, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82475590: C02B0000  lfs f1, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82475594: C18B002C  lfs f12, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82475598: C16B0028  lfs f11, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8247559C: C14B0024  lfs f10, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824755A0: C12B0020  lfs f9, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824755A4: C10B001C  lfs f8, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 824755A8: C0EB0018  lfs f7, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 824755AC: D8C10048  stfd f6, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.f[6].u64 ) };
	// 824755B0: E9410048  ld r10, 0x48(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(72 as u32) ) };
	// 824755B4: D8A10040  stfd f5, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.f[5].u64 ) };
	// 824755B8: E9210040  ld r9, 0x40(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(64 as u32) ) };
	// 824755BC: D8810038  stfd f4, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.f[4].u64 ) };
	// 824755C0: E9010038  ld r8, 0x38(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(56 as u32) ) };
	// 824755C4: D8610030  stfd f3, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.f[3].u64 ) };
	// 824755C8: E8E10030  ld r7, 0x30(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(48 as u32) ) };
	// 824755CC: D8410028  stfd f2, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.f[2].u64 ) };
	// 824755D0: E8C10028  ld r6, 0x28(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(40 as u32) ) };
	// 824755D4: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 824755D8: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 824755DC: D9810078  stfd f12, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.f[12].u64 ) };
	// 824755E0: D9610070  stfd f11, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.f[11].u64 ) };
	// 824755E4: D9410068  stfd f10, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.f[10].u64 ) };
	// 824755E8: D9210060  stfd f9, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[9].u64 ) };
	// 824755EC: D9010058  stfd f8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[8].u64 ) };
	// 824755F0: D8E10050  stfd f7, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[7].u64 ) };
	// 824755F4: 4BFF4F6D  bl 0x8246a560
	ctx.lr = 0x824755F8;
	sub_8246A560(ctx, base);
	// 824755F8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824755FC: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82475600: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82475604: 480BFB08  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82475608 => {
    //   block [0x82475608..0x824756B0)
	// 82475608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8247560C: C0CB0014  lfs f6, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82475610: C0AB0010  lfs f5, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82475614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82475618: C08B000C  lfs f4, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8247561C: 388A9750  addi r4, r10, -0x68b0
	ctx.r[4].s64 = ctx.r[10].s64 + -26800;
	// 82475620: C06B0008  lfs f3, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82475624: C04B0004  lfs f2, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82475628: C02B0000  lfs f1, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8247562C: C00B003C  lfs f0, 0x3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82475630: C3EB0038  lfs f31, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82475634: C3CB0034  lfs f30, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82475638: C1AB0030  lfs f13, 0x30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8247563C: C18B002C  lfs f12, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82475640: C16B0028  lfs f11, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82475644: C14B0024  lfs f10, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82475648: C12B0020  lfs f9, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8247564C: C10B001C  lfs f8, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82475650: C0EB0018  lfs f7, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82475654: D8C10048  stfd f6, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.f[6].u64 ) };
	// 82475658: E9410048  ld r10, 0x48(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(72 as u32) ) };
	// 8247565C: D8A10040  stfd f5, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.f[5].u64 ) };
	// 82475660: E9210040  ld r9, 0x40(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(64 as u32) ) };
	// 82475664: D8810038  stfd f4, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.f[4].u64 ) };
	// 82475668: E9010038  ld r8, 0x38(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(56 as u32) ) };
	// 8247566C: D8610030  stfd f3, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.f[3].u64 ) };
	// 82475670: E8E10030  ld r7, 0x30(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(48 as u32) ) };
	// 82475674: D8410028  stfd f2, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.f[2].u64 ) };
	// 82475678: E8C10028  ld r6, 0x28(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(40 as u32) ) };
	// 8247567C: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 82475680: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 82475684: D8010098  stfd f0, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.f[0].u64 ) };
	// 82475688: DBE10090  stfd f31, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.f[31].u64 ) };
	// 8247568C: DBC10088  stfd f30, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.f[30].u64 ) };
	// 82475690: D9A10080  stfd f13, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.f[13].u64 ) };
	// 82475694: D9810078  stfd f12, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.f[12].u64 ) };
	// 82475698: D9610070  stfd f11, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.f[11].u64 ) };
	// 8247569C: D9410068  stfd f10, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.f[10].u64 ) };
	// 824756A0: D9210060  stfd f9, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[9].u64 ) };
	// 824756A4: D9010058  stfd f8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[8].u64 ) };
	// 824756A8: D8E10050  stfd f7, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[7].u64 ) };
	// 824756AC: 4BFF4EB5  bl 0x8246a560
	ctx.lr = 0x824756B0;
	sub_8246A560(ctx, base);
	pc = 0x824756B0; continue 'dispatch;
            }
            0x824756B0 => {
    //   block [0x824756B0..0x824756C0)
	// 824756B0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824756B4: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 824756B8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824756BC: 480BFA50  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


